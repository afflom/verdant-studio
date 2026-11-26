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
    F497(Bytes),
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
            Fn3::F497(k0) => f497(k0.clone(), p0),
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
    F544(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F406(k0) => f406(k0.clone(), p0),
            Fn5::F544(k0, k1) => Ok(f544(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F408,
    F529,
    F740,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F408 => f408(p0, p1),
            Fn6::F529 => f529(p0, p1),
            Fn6::F740 => f740(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F411,
    F530,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F411 => f411(p0, p1),
            Fn7::F530 => f530(p0, p1),
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
    F508,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn13::F508 => f508(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F511,
}

impl Fn14 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn14::F511 => f511(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F548,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F548 => f548(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F550,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F550 => f550(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F553(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F553(k0) => f553(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F570,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F570 => f570(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F589,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F589 => f589(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F593,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F593 => f593(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F656,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F656 => f656(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F667,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F667 => f667(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F698,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F698 => f698(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F726,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F726 => f726(p0),
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
    f486({
        let a1653 = {
            let a1652 = v0.clone();
            length_bytes(a1652)
        };
        let a1654 = 1u64;
        nat_add(a1653, a1654)?
    }, v0)
}

pub fn f401(v0: Adt14, v1: Bytes) -> bool {
    let m1655 = f94(v0, v1);
    match m1655 {
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
            let a1657 = {
                let a1656 = v0;
                length_bytes(a1656)
            };
            let a1658 = 1u64;
            nat_sub(a1657, a1658)
        })
    } else {
        v0
    }
}

pub fn f403(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1659 = v0;
    match m1659 {
        Adt14::C0(v3) => {
            Adt14::C0(v3)
        }
        Adt14::C1(v4, v5) => {
            Adt14::C1(v4, v5)
        }
        Adt14::C2(v2) => {
            Adt14::C2(f487(f60(), v2, v1))
        }
        Adt14::C3(r1660) => {
            let v6 = (*r1660).clone();
            Adt14::C3(std::rc::Rc::new(v6))
        }
        Adt14::C4(r1661) => {
            let v7 = (*r1661).clone();
            Adt14::C4(std::rc::Rc::new(v7))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1662, r1663) => {
            let v8 = (*r1662).clone();
            let v9 = (*r1663).clone();
            Adt14::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt14::C7(v10, r1664, r1665) => {
            let v11 = (*r1664).clone();
            let v12 = (*r1665).clone();
            Adt14::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f404(v0: u64) -> R<Bytes> {
    f488(f489(), 10u64, v0)
}

pub fn f405(v0: Fn5, p1670: u64, p1671: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1669 = (p1670, p1671);
    {
        let mut m1669 = m1669;
        loop {
            let step = {
                let (v1, v2) = m1669;
                let m1666 = v1;
                if m1666 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1666 - 1;
                    let m1668 = {
                        let c1667: Fn5 = v0.clone();
                        c1667.apply(v2.clone())?
                    };
                    match m1668 {
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
                Ok(next) => { m1669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f490(v0, v1)
}

pub fn f407(v0: Fn6, p1675: List<(Bytes, Bytes)>, p1676: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1674 = (p1675, p1676);
    {
        let mut m1674 = m1674;
        loop {
            let step = {
                let (v1, v2) = m1674;
                let m1672 = v2;
                match m1672.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1673: Fn6 = v0.clone();
                            c1673.apply(v1, v3)
                        }, v4))
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

pub fn f408(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f491(v0, v1)
}

pub fn f409(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f492(v0)
}

pub fn f410(v0: Fn7, p1680: List<Bytes>, p1681: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1679 = (p1680, p1681);
    {
        let mut m1679 = m1679;
        loop {
            let step = {
                let (v1, v2) = m1679;
                let m1677 = v2;
                match m1677.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1678: Fn7 = v0.clone();
                            c1678.apply(v1, v3)?
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

pub fn f411(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f493(v1)?, v0))
}

pub fn f412(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f304(v0)?, List::cons(f8(Str::lit(":")), List::cons(f304(v1)?, List::<Bytes>::nil())))))
}

pub fn f413(v0: Adt13) -> Bytes {
    let m1682 = v0;
    match m1682 {
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
    let m1683 = v0;
    match m1683 {
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
    let m1684 = v0;
    match m1684 {
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
    let a1685 = 0u64;
    let a1686 = v1;
    if nat_lt(a1685, a1686) {
        if f494(v0, {
            let a1687 = v1;
            let a1688 = 1u64;
            nat_sub(a1687, a1688)
        }) {
            Some({
                let a1689 = v1;
                let a1690 = 1u64;
                nat_sub(a1689, a1690)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f417(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1691 = v2;
    let a1692 = v0;
    if nat_lt(a1691, a1692) {
        if f494(v1, v2) {
            Ok(Some({
                let a1693 = v2;
                let a1694 = 1u64;
                nat_add(a1693, a1694)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f418(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1697 = f46(v0.clone(), {
        let a1695 = v1;
        let a1696 = 2u64;
        nat_add(a1695, a1696)?
    }, 2u64);
    let a1698 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1697, a1698) {
        let m1701 = f495(f46(v0, {
            let a1699 = v1;
            let a1700 = 4u64;
            nat_add(a1699, a1700)?
        }, 2u64))?;
        match m1701 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f116(v2.clone(), 0u64)?;
                let a1702 = v3;
                let a1703 = 32u64;
                if nat_lt(a1702, a1703) {
                    if f496(v3) {
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
    let a1704 = v0;
    let a1705 = 34u64;
    if nat_eq(a1704, a1705) {
        Bytes::lit(&[0x22])
    } else {
        let a1706 = v0;
        let a1707 = 92u64;
        if nat_eq(a1706, a1707) {
            Bytes::lit(&[0x5c])
        } else {
            let a1708 = v0;
            let a1709 = 98u64;
            if nat_eq(a1708, a1709) {
                Bytes::lit(&[0x08])
            } else {
                let a1710 = v0;
                let a1711 = 102u64;
                if nat_eq(a1710, a1711) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1712 = v0;
                    let a1713 = 110u64;
                    if nat_eq(a1712, a1713) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1714 = v0;
                        let a1715 = 114u64;
                        if nat_eq(a1714, a1715) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1716 = v0;
                            let a1717 = 116u64;
                            if nat_eq(a1716, a1717) {
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
    let a1718 = 47u64;
    let a1719 = v2;
    if if nat_lt(a1718, a1719) {
        let a1720 = v2;
        let a1721 = 58u64;
        nat_lt(a1720, a1721)
    } else {
        false
    } {
        Ok(Some({
            let a1722 = v1;
            let a1723 = 1u64;
            nat_add(a1722, a1723)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f421(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1724 = f116(v0, v1)?;
    let a1725 = 48u64;
    if nat_eq(a1724, a1725) {
        Ok(Some({
            let a1726 = v1;
            let a1727 = 1u64;
            nat_add(a1726, a1727)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f422(v0: Bytes) -> R<u64> {
    let (h1728, _) = f316(Fn3::F497(v0.clone()), {
        let a1729 = v0;
        length_bytes(a1729)
    }, 0u64)?;
    Ok(h1728)
}

pub fn f423(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1733 = {
        let a1731 = v0;
        let a1732 = {
            let (h1730, _) = v1;
            h1730
        };
        index_bytes(a1731, a1732)
    };
    match m1733 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1735 = {
                    let (h1734, _) = v1;
                    h1734
                };
                let a1736 = 1u64;
                nat_add(a1735, a1736)?
            }, {
                let a1740 = {
                    let a1738 = {
                        let (_, h1737) = v1;
                        h1737
                    };
                    let a1739 = 10u64;
                    nat_mul(a1738, a1739)?
                };
                let a1741 = f498(v2)?;
                nat_add(a1740, a1741)?
            })))
        }
    }
}

pub fn f424(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f499(v0, v1)
}

pub fn f425(v0: (Bytes, Bytes), p1744: List<(Bytes, Bytes)>) -> bool {
    let m1743 = p1744;
    {
        let mut m1743 = m1743;
        loop {
            let step = {
                let v1 = m1743;
                let m1742 = v1;
                match m1742.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f500(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1745 = v1;
    match m1745.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1746, _) = v2;
                h1746
            }, v0.clone()) {
                List::cons({
                    let (_, h1747) = v2;
                    h1747
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
    let m1748 = f94(v0, v1);
    match m1748 {
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
        let a1750 = {
            let a1749 = v1;
            length_bytes(a1749)
        };
        let a1751 = v2;
        nat_sub(a1750, a1751)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1752 = f336(v3.clone(), f8(Str::lit(")")))?;
        match m1752 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1753 = v4;
                    let a1754 = 1u64;
                    nat_add(a1753, a1754)?
                }, {
                    let a1758 = {
                        let a1755 = v3;
                        length_bytes(a1755)
                    };
                    let a1759 = {
                        let a1756 = v4;
                        let a1757 = 1u64;
                        nat_add(a1756, a1757)?
                    };
                    nat_sub(a1758, a1759)
                })
            }
        }
    } else {
        v3
    };
    let a1760 = 0u64;
    let a1761 = v2;
    if nat_lt(a1760, a1761) {
        f501(v5)
    } else {
        Ok(false)
    }
}

pub fn f432(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f433(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1764 = {
            let a1762 = v4;
            let a1763 = v1;
            nat_add(a1762, a1763)?
        };
        let a1765 = v3;
        if nat_lt(a1764, a1765) {
            Ok(Some({
                let a1766 = v4;
                let a1767 = 1u64;
                nat_add(a1766, a1767)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f433(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1772 = {
        let a1769 = v0;
        let a1770 = v2;
        let a1771 = {
            let a1768 = v1.clone();
            length_bytes(a1768)
        };
        slice_bytes(a1769, a1770, a1771)
    };
    match m1772 {
        None => {
            false
        }
        Some(v3) => {
            let a1773 = v3;
            let a1774 = v1;
            equal(a1773, a1774)
        }
    }
}

pub fn f434(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1775 = v0;
    match m1775.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f434(v3, {
                let a1776 = v1;
                let a1777 = 1u64;
                nat_add(a1776, a1777)?
            })?))
        }
    }
}

pub fn f435(v0: Adt19, v1: List<u64>, v2: Adt22) -> bool {
    let m1778 = v2;
    match m1778 {
        Adt22::C0(v3, v4) => {
            if f441(v3, match v0.clone() {
                Adt19::C0(h1779, _, _, _) => {
                    h1779
                }
            }) {
                f502(v0, v4)
            } else {
                false
            }
        }
        Adt22::C1(v5) => {
            f441(v5, match v0 {
                Adt19::C0(_, _, h1780, _) => {
                    h1780
                }
            })
        }
        Adt22::C2(_, v7, v8) => {
            if f437(v7, v1) {
                f441(v8, match v0 {
                    Adt19::C0(_, _, _, h1781) => {
                        h1781
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f436(v0: Adt16, v1: Adt14) -> R<Adt22> {
    let m1782 = f94(v1.clone(), f8(Str::lit("std")));
    match m1782 {
        None => {
            let m1787 = f93(f94(v1.clone(), f8(Str::lit("req"))));
            match m1787 {
                None => {
                    let m1789 = f94(v1, f8(Str::lit("der")));
                    match m1789 {
                        None => {
                            Ok(f503(v0))
                        }
                        Some(v5) => {
                            let m1790 = f504(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1790 {
                                None => {
                                    Ok(f503(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt22::C2(v6, f344(f215(f188(f94(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt16::C0(_, _, _, _, h1791) => {
                                            h1791
                                        }
                                    })?, f439(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt16::C0(_, _, _, h1792, _) => {
                                            h1792
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt22::C1(f439(v4, match v0 {
                        Adt16::C0(_, _, h1788, _, _) => {
                            h1788
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1783 = f93(f94(v2.clone(), f8(Str::lit("row"))));
            match m1783 {
                None => {
                    Ok(Adt22::C0(f439(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt16::C0(h1784, _, _, _, _) => {
                            h1784
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt22::C0(f439(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt16::C0(h1785, _, _, _, _) => {
                            h1785
                        }
                    })?, Some(f439(v3, match v0 {
                        Adt16::C0(_, h1786, _, _, _) => {
                            h1786
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f437(p1795: List<u64>, v1: List<u64>) -> bool {
    let m1794 = p1795;
    {
        let mut m1794 = m1794;
        loop {
            let step = {
                let v0 = m1794;
                let m1793 = v0;
                match m1793.uncons() {
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
                Ok(next) => { m1794 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(p1800: List<u64>) -> bool {
    let m1799 = p1800;
    {
        let mut m1799 = m1799;
        loop {
            let step = {
                let v0 = m1799;
                let m1796 = v0;
                match m1796.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1797 = v1;
                        let a1798 = 7u64;
                        if nat_le(a1797, a1798) {
                            Ok::<List<u64>, bool>(v2)
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

pub fn f439(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1801 = f505(v0, v1.clone(), 0u64)?;
    match m1801 {
        None => {
            let a1802 = v1;
            Ok(length_list(a1802))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f440(v0: Adt14) -> u64 {
    let m1803 = f506(f94(v0, f8(Str::lit("stratum"))));
    match m1803 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f441(v0: u64, p1808: List<u64>) -> bool {
    let m1807 = p1808;
    {
        let mut m1807 = m1807;
        loop {
            let step = {
                let v1 = m1807;
                let m1804 = v1;
                match m1804.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1805 = v2;
                        let a1806 = v0;
                        if nat_eq(a1805, a1806) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: Fn8, p1812: List<Bytes>, p1813: List<(Bytes, ())>) -> List<Bytes> {
    let m1811 = (p1812, p1813);
    {
        let mut m1811 = m1811;
        loop {
            let step = {
                let (v1, v2) = m1811;
                let m1809 = v2;
                match m1809.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1810: Fn8 = v0.clone();
                            c1810.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1811 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1814, _) = v1;
        h1814
    }, v0)
}

pub fn f444(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f507(Fn13::F508, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f445(v0: Fn9, p1818: List<List<(Bytes, ())>>, p1819: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1817 = (p1818, p1819);
    {
        let mut m1817 = m1817;
        loop {
            let step = {
                let (v1, v2) = m1817;
                let m1815 = v2;
                match m1815.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1816: Fn9 = v0.clone();
                            c1816.apply(v1, v3)
                        }, v4))
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

pub fn f446(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f447(v0: Fn10, p1824: u64, p1825: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1823 = (p1824, p1825);
    {
        let mut m1823 = m1823;
        loop {
            let step = {
                let (v1, v2) = m1823;
                let m1820 = v1;
                if m1820 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1820 - 1;
                    let m1822 = {
                        let c1821: Fn10 = v0.clone();
                        c1821.apply(v2.clone())?
                    };
                    match m1822 {
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
                Ok(next) => { m1823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f509(v0)
}

pub fn f449(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f510(Fn14::F511, List::<(Bytes, ())>::nil(), v0)
}

pub fn f450(v0: Fn11, p1829: List<(Bytes, ())>, p1830: List<Bytes>) -> List<(Bytes, ())> {
    let m1828 = (p1829, p1830);
    {
        let mut m1828 = m1828;
        loop {
            let step = {
                let (v1, v2) = m1828;
                let m1826 = v2;
                match m1826.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1827: Fn11 = v0.clone();
                            c1827.apply(v1, v3)
                        }, v4))
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

pub fn f451(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f452(p1835: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1834 = p1835;
    {
        let mut m1834 = m1834;
        loop {
            let step = {
                let v0 = m1834;
                let m1831 = v0;
                match m1831.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1832, _) = v2;
                            h1832
                        }) {
                            let (_, h1833) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1833)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1834 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: Bytes, p1861: u64, p1862: u64, p1863: Bytes) -> R<Bytes> {
    let m1860 = (p1861, (p1862, p1863));
    {
        let mut m1860 = m1860;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1860;
                let m1836 = v2;
                if m1836 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1836 - 1;
                    let m1839 = {
                        let a1837 = v0.clone();
                        let a1838 = v1;
                        index_bytes(a1837, a1838)
                    };
                    match m1839 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f203({
                                let a1840 = v6;
                                let a1841 = 34u64;
                                nat_eq(a1840, a1841)
                            }, {
                                let a1842 = v6;
                                let a1843 = 92u64;
                                nat_eq(a1842, a1843)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1844 = v1;
                                    let a1845 = 1u64;
                                    nat_add(a1844, a1845)?
                                }, (v4, {
                                    let a1848 = v3;
                                    let a1849 = {
                                        let a1846 = f8(Str::lit("\\"));
                                        let a1847 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1846, a1847)
                                    };
                                    append_bytes(a1848, a1849)
                                })))
                            } else {
                                let a1850 = v6;
                                let a1851 = 10u64;
                                if nat_eq(a1850, a1851) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1852 = v1;
                                        let a1853 = 1u64;
                                        nat_add(a1852, a1853)?
                                    }, (v4, {
                                        let a1854 = v3;
                                        let a1855 = f8(Str::lit("\\n"));
                                        append_bytes(a1854, a1855)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1856 = v1;
                                        let a1857 = 1u64;
                                        nat_add(a1856, a1857)?
                                    }, (v4, {
                                        let a1858 = v3;
                                        let a1859 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1858, a1859)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1864 = v0;
    match m1864 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f455(v0: List<Adt14>) -> List<Bytes> {
    let m1865 = v0;
    match m1865.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f512(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f282(f58(v1, f8(Str::lit("authority")))), f455(v2)))
        }
    }
}

pub fn f456(v0: List<Bytes>) -> R<List<Adt14>> {
    let m1866 = f295(v0.clone())?;
    match m1866 {
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
    let m1867 = f295(v3.clone())?;
    match m1867 {
        None => {
            let m1868 = f268(v3.clone())?;
            match m1868 {
                None => {
                    f515(v0, v1, v3)
                }
                Some(v5) => {
                    f514(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f513(v0, v1, v4)
        }
    }
}

pub fn f458(v0: List<Bytes>) -> List<Bytes> {
    let m1869 = v0;
    match m1869.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1871 = {
                    let a1870 = v1.clone();
                    length_bytes(a1870)
                };
                let a1872 = 0u64;
                if nat_eq(a1871, a1872) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f458(v2))
        }
    }
}

pub fn f459(v0: Bytes, v1: u64) -> R<u64> {
    let m1875 = {
        let a1873 = v0;
        let a1874 = v1;
        index_bytes(a1873, a1874)
    };
    match m1875 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f460(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1879 = {
        let a1876 = v1;
        let a1877 = v2;
        let a1878 = 1u64;
        slice_bytes(a1876, a1877, a1878)
    };
    match m1879 {
        None => {
            v0
        }
        Some(v3) => {
            let a1880 = v0;
            let a1881 = v3;
            append_bytes(a1880, a1881)
        }
    }
}

pub fn f461(p1884: List<Bytes>, p1885: List<Bytes>) -> List<Bytes> {
    let m1883 = (p1884, p1885);
    {
        let mut m1883 = m1883;
        loop {
            let step = {
                let (v0, v1) = m1883;
                let m1882 = v0;
                match m1882.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: List<Bytes>) -> Bytes {
    let m1886 = v0;
    match m1886.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f463(v0: Bytes) -> R<bool> {
    let m1888 = {
        let a1887 = v0.clone();
        length_bytes(a1887)
    };
    if m1888 == 0 {
        Ok(true)
    } else {
        let v1 = m1888 - 1;
        let m1891 = {
            let a1889 = v0;
            let a1890 = v1;
            index_bytes(a1889, a1890)
        };
        match m1891 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f516({
                    let v3: u8 = v2;
                    let a1936 = {
                        let a1912 = {
                            let a1900 = {
                                let a1894 = {
                                    let a1892 = v3;
                                    let a1893 = 128u8;
                                    fixed_u8::bit_and(a1892, a1893)
                                };
                                let a1895 = 128u8;
                                if equal(a1894, a1895) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1901 = {
                                let a1898 = {
                                    let a1896 = v3;
                                    let a1897 = 64u8;
                                    fixed_u8::bit_and(a1896, a1897)
                                };
                                let a1899 = 64u8;
                                if equal(a1898, a1899) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1900, a1901)?
                        };
                        let a1913 = {
                            let a1910 = {
                                let a1904 = {
                                    let a1902 = v3;
                                    let a1903 = 32u8;
                                    fixed_u8::bit_and(a1902, a1903)
                                };
                                let a1905 = 32u8;
                                if equal(a1904, a1905) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1911 = {
                                let a1908 = {
                                    let a1906 = v3;
                                    let a1907 = 16u8;
                                    fixed_u8::bit_and(a1906, a1907)
                                };
                                let a1909 = 16u8;
                                if equal(a1908, a1909) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1910, a1911)?
                        };
                        nat_add(a1912, a1913)?
                    };
                    let a1937 = {
                        let a1934 = {
                            let a1922 = {
                                let a1916 = {
                                    let a1914 = v3;
                                    let a1915 = 8u8;
                                    fixed_u8::bit_and(a1914, a1915)
                                };
                                let a1917 = 8u8;
                                if equal(a1916, a1917) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1923 = {
                                let a1920 = {
                                    let a1918 = v3;
                                    let a1919 = 4u8;
                                    fixed_u8::bit_and(a1918, a1919)
                                };
                                let a1921 = 4u8;
                                if equal(a1920, a1921) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1922, a1923)?
                        };
                        let a1935 = {
                            let a1932 = {
                                let a1926 = {
                                    let a1924 = v3;
                                    let a1925 = 2u8;
                                    fixed_u8::bit_and(a1924, a1925)
                                };
                                let a1927 = 2u8;
                                if equal(a1926, a1927) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1933 = {
                                let a1930 = {
                                    let a1928 = v3;
                                    let a1929 = 1u8;
                                    fixed_u8::bit_and(a1928, a1929)
                                };
                                let a1931 = 1u8;
                                if equal(a1930, a1931) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1932, a1933)?
                        };
                        nat_add(a1934, a1935)?
                    };
                    nat_add(a1936, a1937)?
                }))
            }
        }
    }
}

pub fn f464(v0: Adt2, v1: Bytes) -> bool {
    let a1939 = {
        let a1938 = f279(v0, v1);
        length_bytes(a1938)
    };
    let a1940 = 0u64;
    !nat_eq(a1939, a1940)
}

pub fn f465(v0: Adt2, v1: Bytes) -> bool {
    let m1941 = f276(v0, v1);
    match m1941 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f466(v0: Bytes, p1962: u64, p1963: u64, p1964: bool, p1965: Bytes) -> R<Bytes> {
    let m1961 = (p1962, (p1963, (p1964, p1965)));
    {
        let mut m1961 = m1961;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1961;
                let m1942 = v2;
                if m1942 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1942 - 1;
                    let m1945 = {
                        let a1943 = v0.clone();
                        let a1944 = v1;
                        index_bytes(a1943, a1944)
                    };
                    match m1945 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f517(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1946 = v1;
                                    let a1947 = 1u64;
                                    nat_add(a1946, a1947)?
                                }, (v5, (true, v4))))
                            } else {
                                if f118(f518(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1948 = v1;
                                        let a1949 = 1u64;
                                        nat_add(a1948, a1949)?
                                    }, (v5, (false, {
                                        let a1953 = v4;
                                        let a1954 = f46({
                                            let a1950 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1950)
                                        }, {
                                            let a1951 = v7;
                                            let a1952 = 97u64;
                                            nat_sub(a1951, a1952)
                                        }, 1u64);
                                        append_bytes(a1953, a1954)
                                    }))))
                                } else {
                                    if f203(f518(v7), f203(f519(v7), f520(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1955 = v1;
                                            let a1956 = 1u64;
                                            nat_add(a1955, a1956)?
                                        }, (v5, (false, {
                                            let a1957 = v4;
                                            let a1958 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1957, a1958)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1959 = v1;
                                            let a1960 = 1u64;
                                            nat_add(a1959, a1960)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1961 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f521(f522(f276(v0.clone(), f8(Str::lit("$defs")))), f522(f276(v0.clone(), f8(Str::lit("definitions")))));
    let m1966 = f523(4096u64, v3.clone(), List::<Bytes>::nil(), f524(v3)?)?;
    match m1966 {
        Ok(v5) => {
            let m1967 = f525(4096u64, f526(f527(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1967 {
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
        let a1968 = v3.clone();
        length_bytes(a1968)
    };
    let v5: Bytes = {
        let a1969 = v2;
        let a1970 = v4;
        if nat_lt(a1969, a1970) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1971 = v4;
                let a1972 = v2;
                nat_sub(a1971, a1972)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1973 = v4;
                let a1974 = v2;
                nat_sub(a1973, a1974)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f323({
                let a1975 = v2;
                let a1976 = v4;
                nat_sub(a1975, a1976)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1977 = f8(Str::lit("-"));
        let a1978 = v5;
        Ok(append_bytes(a1977, a1978))
    } else {
        Ok(v5)
    }
}

pub fn f469(v0: Bytes) -> R<Bytes> {
    let a1983 = f200(v0.clone(), 0u64, {
        let a1980 = {
            let a1979 = v0.clone();
            length_bytes(a1979)
        };
        let a1981 = 1u64;
        nat_add(a1980, a1981)?
    })?;
    let a1984 = {
        let a1982 = v0.clone();
        length_bytes(a1982)
    };
    if nat_eq(a1983, a1984) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1990 = {
            let a1988 = Bytes::lit(&[0x22]);
            let a1989 = f528(v0.clone(), 0u64, {
                let a1986 = {
                    let a1985 = v0;
                    length_bytes(a1985)
                };
                let a1987 = 1u64;
                nat_add(a1986, a1987)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1988, a1989)
        };
        let a1991 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1990, a1991))
    }
}

pub fn f470(v0: Adt2) -> R<List<Bytes>> {
    let m1992 = v0;
    match m1992 {
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
        Adt2::C9(r1993, r1994) => {
            let v12 = (*r1993).clone();
            let v13 = (*r1994).clone();
            Ok(List::cons(f384(v12)?, f470(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f471(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f407(Fn6::F529, List::<(Bytes, Bytes)>::nil(), f409(v0)?);
    let v2: List<Bytes> = f410(Fn7::F530, List::<Bytes>::nil(), v1)?;
    let a1997 = {
        let a1995 = Bytes::lit(&[0x7b]);
        let a1996 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1995, a1996)
    };
    let a1998 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1997, a1998))
}

pub fn f472(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1999 = v0;
    match m1999 {
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
        Adt2::C10(v14, r2000, r2001) => {
            let v15 = (*r2000).clone();
            let v16 = (*r2001).clone();
            Ok(List::cons((v14, f384(v15)?), f472(v16)?))
        }
    }
}

pub fn f473(v0: u32) -> Bytes {
    let a2030 = f531({
        let a2002 = f532(v0, 28u32);
        let a2003 = 15u32;
        fixed_u32::bit_and(a2002, a2003)
    });
    let a2031 = {
        let a2028 = f531({
            let a2004 = f532(v0, 24u32);
            let a2005 = 15u32;
            fixed_u32::bit_and(a2004, a2005)
        });
        let a2029 = {
            let a2026 = f531({
                let a2006 = f532(v0, 20u32);
                let a2007 = 15u32;
                fixed_u32::bit_and(a2006, a2007)
            });
            let a2027 = {
                let a2024 = f531({
                    let a2008 = f532(v0, 16u32);
                    let a2009 = 15u32;
                    fixed_u32::bit_and(a2008, a2009)
                });
                let a2025 = {
                    let a2022 = f531({
                        let a2010 = f532(v0, 12u32);
                        let a2011 = 15u32;
                        fixed_u32::bit_and(a2010, a2011)
                    });
                    let a2023 = {
                        let a2020 = f531({
                            let a2012 = f532(v0, 8u32);
                            let a2013 = 15u32;
                            fixed_u32::bit_and(a2012, a2013)
                        });
                        let a2021 = {
                            let a2018 = f531({
                                let a2014 = f532(v0, 4u32);
                                let a2015 = 15u32;
                                fixed_u32::bit_and(a2014, a2015)
                            });
                            let a2019 = f531({
                                let a2016 = v0;
                                let a2017 = 15u32;
                                fixed_u32::bit_and(a2016, a2017)
                            });
                            append_bytes(a2018, a2019)
                        };
                        append_bytes(a2020, a2021)
                    };
                    append_bytes(a2022, a2023)
                };
                append_bytes(a2024, a2025)
            };
            append_bytes(a2026, a2027)
        };
        append_bytes(a2028, a2029)
    };
    append_bytes(a2030, a2031)
}

pub fn f474(v0: Fn12, p2035: u64, p2036: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m2034 = (p2035, p2036);
    {
        let mut m2034 = m2034;
        loop {
            let step = {
                let (v1, v2) = m2034;
                let m2032 = v1;
                if m2032 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m2032 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c2033: Fn12 = v0.clone();
                        c2033.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a2038 = {
            let (h2037, _) = v3.clone();
            h2037
        };
        let a2039 = 64u64;
        nat_add(a2038, a2039)?
    }, f533({
        let (_, h2040) = v3;
        h2040
    }, f534(v0, v1, v2, {
        let (h2041, _) = v3;
        h2041
    })?)?))
}

pub fn f476() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f477(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Adt2) -> R<Adt12> {
    let m2042 = f535(f536(v1.clone(), v3.clone(), v4, v6))?;
    match m2042 {
        Ok(v8) => {
            let v9: Bytes = f512(v1.clone());
            Ok(Adt12::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f484(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5))))))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f381(v5)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt14::C2(f283(f64(v0, v2))?)), std::rc::Rc::new(Adt14::C5))))))), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f478(p2046: List<Adt14>, v1: Bytes, v2: Bytes) -> bool {
    let m2045 = p2046;
    {
        let mut m2045 = m2045;
        loop {
            let step = {
                let v0 = m2045;
                let m2043 = v0;
                match m2043.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f118(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2044 = f94(v3, f8(Str::lit("std")));
                            match m2044 {
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
                Ok(next) => { m2045 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: Adt2, v1: Adt2) -> R<Option<Bytes>> {
    f537({
        let a2053 = {
            let a2048 = {
                let a2047 = f538(v0.clone())?;
                length_bytes(a2047)
            };
            let a2049 = 2u64;
            nat_add(a2048, a2049)?
        };
        let a2054 = {
            let a2051 = {
                let a2050 = f538(v1.clone())?;
                length_bytes(a2050)
            };
            let a2052 = 2u64;
            nat_add(a2051, a2052)?
        };
        nat_mul(a2053, a2054)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f480(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f395(f539(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f481(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2055 = f44(v1, 34u8)?;
    match m2055.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2056 = v3;
            match m2056.uncons() {
                None => {
                    Ok(f540(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f482(v0: List<Bytes>) -> List<Bytes> {
    let m2057 = v0;
    match m2057.uncons() {
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
    let m2058 = v0;
    match m2058.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f483(v3, v1))
        }
    }
}

pub fn f484(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m2059 = v0;
    match m2059.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f541(v5, v1, v2))
            } else {
                List::cons(v4, f484(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f485(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f395(f542(f96(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f486(p2070: u64, p2071: Bytes) -> R<Bytes> {
    let m2069 = (p2070, p2071);
    {
        let mut m2069 = m2069;
        loop {
            let step = {
                let (v0, v1) = m2069;
                let m2060 = v0;
                if m2060 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m2060 - 1;
                    if f85(v1.clone(), f8(Str::lit("-- "))) {
                        let m2061 = f336(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m2061 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a2062 = v3;
                                    let a2063 = 1u64;
                                    nat_add(a2062, a2063)?
                                }, {
                                    let a2067 = {
                                        let a2064 = v1;
                                        length_bytes(a2064)
                                    };
                                    let a2068 = {
                                        let a2065 = v3;
                                        let a2066 = 1u64;
                                        nat_add(a2065, a2066)?
                                    };
                                    nat_sub(a2067, a2068)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m2069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f487(p2074: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2073 = p2074;
    {
        let mut m2073 = m2073;
        loop {
            let step = {
                let v0 = m2073;
                let m2072 = v0;
                match m2072.uncons() {
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
                Ok(next) => { m2073 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2075 = v2;
    let a2076 = 0u64;
    if nat_eq(a2075, a2076) {
        Ok(f543(v0, 0u64))
    } else {
        let (_, h2077) = {
            let (h2078, _) = f405(Fn5::F544(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2078
        };
        Ok(h2077)
    }
}

pub fn f489() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f490(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2082 = {
        let a2080 = v0.clone();
        let a2081 = {
            let (h2079, _) = v1.clone();
            h2079
        };
        index_bytes(a2080, a2081)
    };
    match m2082 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2084 = {
                    let (h2083, _) = v1.clone();
                    h2083
                };
                let a2085 = 1u64;
                nat_add(a2084, a2085)?
            }, {
                let a2088 = {
                    let (_, h2086) = v1;
                    h2086
                };
                let a2089 = f545(v0, {
                    let (h2087, _) = v1;
                    h2087
                }, v2)?;
                append_bytes(a2088, a2089)
            })))
        }
    }
}

pub fn f491(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2090 = v0.clone();
    match m2090.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2093 = {
                let (h2091, _) = v2;
                h2091
            };
            let a2094 = {
                let (h2092, _) = v1.clone();
                h2092
            };
            if equal(a2093, a2094) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f492(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f546(f547(Fn15::F548, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2099 = {
        let (h2095, _) = f549(Fn16::F550, {
            let a2097 = {
                let a2096 = v0;
                length_list(a2096)
            };
            let a2098 = 1u64;
            nat_add(a2097, a2098)?
        }, v1)?;
        h2095
    };
    match m2099.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f493(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2104 = {
        let a2101 = f304({
            let (h2100, _) = v0;
            h2100
        })?;
        let a2102 = Bytes::lit(&[0x3a]);
        append_bytes(a2101, a2102)
    };
    let a2105 = {
        let (_, h2103) = v0;
        h2103
    };
    Ok(append_bytes(a2104, a2105))
}

pub fn f494(v0: Bytes, v1: u64) -> bool {
    let m2106 = f208(v0, v1);
    match m2106 {
        None => {
            false
        }
        Some(v2) => {
            f551(v2)
        }
    }
}

pub fn f495(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2107 = v0.clone();
        length_bytes(a2107)
    };
    let a2111 = {
        let a2108 = v1;
        let a2109 = 2u64;
        let a2110 = 0u64;
        nat_rem(a2108, a2109, a2110)
    };
    let a2112 = 0u64;
    if nat_eq(a2111, a2112) {
        let (_, h2113) = {
            let (h2114, _) = f552(Fn17::F553(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2114
        };
        Ok(h2113)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f496(v0: u64) -> bool {
    let a2115 = v0;
    let a2116 = 8u64;
    if nat_eq(a2115, a2116) {
        true
    } else {
        let a2117 = v0;
        let a2118 = 9u64;
        if nat_eq(a2117, a2118) {
            true
        } else {
            let a2119 = v0;
            let a2120 = 10u64;
            if nat_eq(a2119, a2120) {
                true
            } else {
                let a2121 = v0;
                let a2122 = 12u64;
                if nat_eq(a2121, a2122) {
                    true
                } else {
                    let a2123 = v0;
                    let a2124 = 13u64;
                    nat_eq(a2123, a2124)
                }
            }
        }
    }
}

pub fn f497(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2127 = {
        let a2125 = v0;
        let a2126 = v1;
        index_bytes(a2125, a2126)
    };
    match m2127 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f554(v2)? {
                Ok(Some({
                    let a2128 = v1;
                    let a2129 = 1u64;
                    nat_add(a2128, a2129)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f498(v0: u8) -> R<u64> {
    let a2176 = {
        let v1: u8 = v0;
        let a2174 = {
            let a2150 = {
                let a2138 = {
                    let a2132 = {
                        let a2130 = v1;
                        let a2131 = 128u8;
                        fixed_u8::bit_and(a2130, a2131)
                    };
                    let a2133 = 128u8;
                    if equal(a2132, a2133) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2139 = {
                    let a2136 = {
                        let a2134 = v1;
                        let a2135 = 64u8;
                        fixed_u8::bit_and(a2134, a2135)
                    };
                    let a2137 = 64u8;
                    if equal(a2136, a2137) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2138, a2139)?
            };
            let a2151 = {
                let a2148 = {
                    let a2142 = {
                        let a2140 = v1;
                        let a2141 = 32u8;
                        fixed_u8::bit_and(a2140, a2141)
                    };
                    let a2143 = 32u8;
                    if equal(a2142, a2143) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2149 = {
                    let a2146 = {
                        let a2144 = v1;
                        let a2145 = 16u8;
                        fixed_u8::bit_and(a2144, a2145)
                    };
                    let a2147 = 16u8;
                    if equal(a2146, a2147) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2148, a2149)?
            };
            nat_add(a2150, a2151)?
        };
        let a2175 = {
            let a2172 = {
                let a2160 = {
                    let a2154 = {
                        let a2152 = v1;
                        let a2153 = 8u8;
                        fixed_u8::bit_and(a2152, a2153)
                    };
                    let a2155 = 8u8;
                    if equal(a2154, a2155) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2161 = {
                    let a2158 = {
                        let a2156 = v1;
                        let a2157 = 4u8;
                        fixed_u8::bit_and(a2156, a2157)
                    };
                    let a2159 = 4u8;
                    if equal(a2158, a2159) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2160, a2161)?
            };
            let a2173 = {
                let a2170 = {
                    let a2164 = {
                        let a2162 = v1;
                        let a2163 = 2u8;
                        fixed_u8::bit_and(a2162, a2163)
                    };
                    let a2165 = 2u8;
                    if equal(a2164, a2165) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2171 = {
                    let a2168 = {
                        let a2166 = v1;
                        let a2167 = 1u8;
                        fixed_u8::bit_and(a2166, a2167)
                    };
                    let a2169 = 1u8;
                    if equal(a2168, a2169) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2170, a2171)?
            };
            nat_add(a2172, a2173)?
        };
        nat_add(a2174, a2175)?
    };
    let a2177 = 48u64;
    Ok(nat_sub(a2176, a2177))
}

pub fn f499(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2178 = v0.clone();
        length_bytes(a2178)
    };
    let a2179 = v1;
    let a2180 = v2;
    if if nat_lt(a2179, a2180) {
        let a2185 = f116(v0, {
            let a2183 = v2;
            let a2184 = {
                let a2181 = v1;
                let a2182 = 1u64;
                nat_add(a2181, a2182)?
            };
            nat_sub(a2183, a2184)
        })?;
        let a2186 = 48u64;
        nat_eq(a2185, a2186)
    } else {
        false
    } {
        Ok(Some({
            let a2187 = v1;
            let a2188 = 1u64;
            nat_add(a2187, a2188)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f500(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f118(f32({
        let (h2189, _) = v0;
        h2189
    }, {
        let (h2190, _) = v1;
        h2190
    }), f32({
        let (_, h2191) = v0;
        h2191
    }, {
        let (_, h2192) = v1;
        h2192
    }))
}

pub fn f501(v0: Bytes) -> R<bool> {
    let m2193 = f336(v0.clone(), f8(Str::lit("&&")))?;
    if match m2193 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2194 = f336(v0, f8(Str::lit("||")))?;
        match m2194 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f502(v0: Adt19, v1: Option<u64>) -> bool {
    let m2195 = v1;
    match m2195 {
        None => {
            true
        }
        Some(v2) => {
            f441(v2, match v0 {
                Adt19::C0(_, h2196, _, _) => {
                    h2196
                }
            })
        }
    }
}

pub fn f503(v0: Adt16) -> Adt22 {
    Adt22::C1({
        let a2198 = match v0 {
            Adt16::C0(_, _, h2197, _, _) => {
                h2197
            }
        };
        length_list(a2198)
    })
}

pub fn f504(v0: Bytes) -> Option<Adt23> {
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

pub fn f505(v0: Bytes, p2203: List<Bytes>, p2204: u64) -> R<Option<u64>> {
    let m2202 = (p2203, p2204);
    {
        let mut m2202 = m2202;
        loop {
            let step = {
                let (v1, v2) = m2202;
                let m2199 = v1;
                match m2199.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2200 = v2;
                                let a2201 = 1u64;
                                nat_add(a2200, a2201)?
                            }))
                        }
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

pub fn f506(v0: Option<Adt14>) -> Option<u64> {
    let m2205 = v0;
    match m2205 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2206 = v1;
            match m2206 {
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

pub fn f507(v0: Fn13, p2210: List<List<(Bytes, ())>>, p2211: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2209 = (p2210, p2211);
    {
        let mut m2209 = m2209;
        loop {
            let step = {
                let (v1, v2) = m2209;
                let m2207 = v2;
                match m2207.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2208: Fn13 = v0.clone();
                            c2208.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f509(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2213 = {
        let a2212 = v0.clone();
        length_list(a2212)
    };
    let a2214 = 1u64;
    if nat_le(a2213, a2214) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f555(v0)?))
    }
}

pub fn f510(v0: Fn14, p2218: List<(Bytes, ())>, p2219: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2217 = (p2218, p2219);
    {
        let mut m2217 = m2217;
        loop {
            let step = {
                let (v1, v2) = m2217;
                let m2215 = v2;
                match m2215.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2216: Fn14 = v0.clone();
                            c2216.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f512(v0: Bytes) -> Bytes {
    f7(List::cons(f140(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f513(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f143(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f143(f97(v0, f154())?, f8(Str::lit("at"))));
    let a2221 = {
        let a2220 = v4.clone();
        length_list(a2220)
    };
    let a2222 = 0u64;
    if nat_eq(a2221, a2222) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2225 = {
            let a2223 = v4;
            length_list(a2223)
        };
        let a2226 = {
            let a2224 = v3;
            length_list(a2224)
        };
        if nat_eq(a2225, a2226) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f556(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f514(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt14>) -> R<(bool, Bytes)> {
    let m2227 = v3;
    match m2227.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f203(f67(v0.clone(), f512(v6.clone())), f52(v6.clone(), v2.clone())) {
                f514(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f282(v6.clone()))) {
                    let v9: (bool, Bytes) = f514(v0, v1.clone(), v2, v5)?;
                    let (h2228, _) = v9.clone();
                    if h2228 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2230 = f556(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2231 = {
                                let (_, h2229) = v9;
                                h2229
                            };
                            append_bytes(a2230, a2231)
                        }))
                    }
                } else {
                    if f118(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f478(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2232 = f8(Str::lit("Catalog."));
                        let a2233 = v8.clone();
                        append_bytes(a2232, a2233)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f514(v0, v1.clone(), v2, v5)?;
                        let (h2234, _) = v10.clone();
                        if h2234 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2236 = f556(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2237 = {
                                    let (_, h2235) = v10;
                                    h2235
                                };
                                append_bytes(a2236, a2237)
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

pub fn f515(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f269(f271(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f269(f271(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f126(f143(v3.clone(), f8(Str::lit("id"))), f143(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f143(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f126(v6.clone(), f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2239 = {
        let a2238 = v7.clone();
        length_list(a2238)
    };
    let a2240 = 0u64;
    if nat_eq(a2239, a2240) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2243 = {
                let a2241 = v7;
                length_list(a2241)
            };
            let a2244 = {
                let a2242 = v5;
                length_list(a2242)
            };
            nat_eq(a2243, a2244)
        }) {
            Ok((false, f556(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f146(f126(f62(f557(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f558(v4), f126(v6, f143(v3, f8(Str::lit("id")))))))?;
            let a2246 = {
                let a2245 = v8.clone();
                length_list(a2245)
            };
            let a2247 = 0u64;
            if nat_eq(a2246, a2247) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f556(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f516(v0: u64) -> bool {
    let a2248 = v0;
    let a2249 = 47u64;
    nat_eq(a2248, a2249)
}

pub fn f517(v0: u64) -> bool {
    f203({
        let a2250 = v0;
        let a2251 = 95u64;
        nat_eq(a2250, a2251)
    }, f203({
        let a2252 = v0;
        let a2253 = 45u64;
        nat_eq(a2252, a2253)
    }, f203({
        let a2254 = v0;
        let a2255 = 32u64;
        nat_eq(a2254, a2255)
    }, f203({
        let a2256 = v0;
        let a2257 = 46u64;
        nat_eq(a2256, a2257)
    }, {
        let a2258 = v0;
        let a2259 = 47u64;
        nat_eq(a2258, a2259)
    }))))
}

pub fn f518(v0: u64) -> bool {
    f118({
        let a2260 = 97u64;
        let a2261 = v0;
        nat_le(a2260, a2261)
    }, {
        let a2262 = v0;
        let a2263 = 122u64;
        nat_le(a2262, a2263)
    })
}

pub fn f519(v0: u64) -> bool {
    f118({
        let a2264 = 65u64;
        let a2265 = v0;
        nat_le(a2264, a2265)
    }, {
        let a2266 = v0;
        let a2267 = 90u64;
        nat_le(a2266, a2267)
    })
}

pub fn f520(v0: u64) -> bool {
    f118({
        let a2268 = 48u64;
        let a2269 = v0;
        nat_le(a2268, a2269)
    }, {
        let a2270 = v0;
        let a2271 = 57u64;
        nat_le(a2270, a2271)
    })
}

pub fn f521(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2272 = v0;
    match m2272.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f521(v3, v1))
        }
    }
}

pub fn f522(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2273 = v0;
    match m2273 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f559(v1)
        }
    }
}

pub fn f523(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2274 = v0;
    if m2274 == 0 {
        let m2275 = v1;
        match m2275.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2274 - 1;
        let m2276 = v1.clone();
        match m2276.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2277 = f560(v1.clone(), v2.clone(), v3.clone())?;
                match m2277 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f395(f524(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2280 = f523(v6, f561(v1, {
                            let (h2278, _) = v9.clone();
                            h2278
                        }), List::cons(f562({
                            let (h2279, _) = v9.clone();
                            h2279
                        })?, v2), v3)?;
                        match m2280 {
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

pub fn f524(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2281 = v0;
    match m2281.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f562({
                let (h2282, _) = v1;
                h2282
            })?, f524(v2)?))
        }
    }
}

pub fn f525(p2288: u64, p2289: List<Adt25>, p2290: List<Bytes>, p2291: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2287 = (p2288, (p2289, (p2290, p2291)));
    {
        let mut m2287 = m2287;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2287;
                let m2283 = v0;
                if m2283 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2283 - 1;
                    let m2284 = v1;
                    match m2284.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2285 = v5;
                            match m2285 {
                                Adt25::C0(v8, v9) => {
                                    if f32(v8.clone(), f563()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2286 = f565(v8.clone(), v9)?;
                                            match m2286 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f526(f566(v11.clone()), List::cons(Adt25::C1(f567(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f52(f563(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f563(), v2), List::cons(f564(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2292 = v0;
    match m2292.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f526(v3, v1))
        }
    }
}

pub fn f527(v0: List<(Bytes, Adt2)>) -> R<List<Adt25>> {
    let m2293 = v0;
    match m2293.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f562({
                let (h2294, _) = v1;
                h2294
            })?, {
                let (_, h2295) = v1;
                h2295
            }), f527(v2)?))
        }
    }
}

pub fn f528(v0: Bytes, p2314: u64, p2315: u64, p2316: Bytes) -> R<Bytes> {
    let m2313 = (p2314, (p2315, p2316));
    {
        let mut m2313 = m2313;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2313;
                let m2296 = v2;
                if m2296 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2296 - 1;
                    let v5: u64 = f200(v0.clone(), v1, {
                        let a2300 = {
                            let a2298 = {
                                let a2297 = v0.clone();
                                length_bytes(a2297)
                            };
                            let a2299 = v1;
                            nat_sub(a2298, a2299)
                        };
                        let a2301 = 1u64;
                        nat_add(a2300, a2301)?
                    })?;
                    let v6: Bytes = {
                        let a2304 = v3;
                        let a2305 = f46(v0.clone(), v1, {
                            let a2302 = v5;
                            let a2303 = v1;
                            nat_sub(a2302, a2303)
                        });
                        append_bytes(a2304, a2305)
                    };
                    let m2308 = {
                        let a2306 = v0.clone();
                        let a2307 = v5;
                        index_bytes(a2306, a2307)
                    };
                    match m2308 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2309 = v5;
                                let a2310 = 1u64;
                                nat_add(a2309, a2310)?
                            }, (v4, {
                                let a2311 = v6;
                                let a2312 = f545(v0.clone(), v5, v7)?;
                                append_bytes(a2311, a2312)
                            })))
                        }
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

pub fn f529(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f491(v0, v1)
}

pub fn f530(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f568(v1)?, v0))
}

pub fn f531(v0: u32) -> Bytes {
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

pub fn f532(v0: u32, v1: u32) -> u32 {
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

pub fn f533(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f569(Fn18::F570, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2380, _, _, _, _, _, _, _) => {
            h2380
        }
    }, match v0.clone() {
        Adt24::C0(_, h2381, _, _, _, _, _, _) => {
            h2381
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2382, _, _, _, _, _) => {
            h2382
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2383, _, _, _, _) => {
            h2383
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2384, _, _, _) => {
            h2384
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2385, _, _) => {
            h2385
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2386, _) => {
            h2386
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2387) => {
            h2387
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f571(match v0.clone() {
        Adt24::C0(h2388, _, _, _, _, _, _, _) => {
            h2388
        }
    }, match v2.clone() {
        Adt27::C0(h2389, _, _, _, _, _, _, _, _, _) => {
            h2389
        }
    }), f571(match v0.clone() {
        Adt24::C0(_, h2390, _, _, _, _, _, _) => {
            h2390
        }
    }, match v2.clone() {
        Adt27::C0(_, h2391, _, _, _, _, _, _, _, _) => {
            h2391
        }
    }), f571(match v0.clone() {
        Adt24::C0(_, _, h2392, _, _, _, _, _) => {
            h2392
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2393, _, _, _, _, _, _, _) => {
            h2393
        }
    }), f571(match v0.clone() {
        Adt24::C0(_, _, _, h2394, _, _, _, _) => {
            h2394
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2395, _, _, _, _, _, _) => {
            h2395
        }
    }), f571(match v0.clone() {
        Adt24::C0(_, _, _, _, h2396, _, _, _) => {
            h2396
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2397, _, _, _, _, _) => {
            h2397
        }
    }), f571(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2398, _, _) => {
            h2398
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2399, _, _, _, _) => {
            h2399
        }
    }), f571(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2400, _) => {
            h2400
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2401, _, _, _) => {
            h2401
        }
    }), f571(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2402) => {
            h2402
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2403, _, _) => {
            h2403
        }
    })))
}

pub fn f534(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f572(v0.clone(), v1, v2, v3)?, f572(v0.clone(), v1, v2, {
        let a2404 = v3;
        let a2405 = 4u64;
        nat_add(a2404, a2405)?
    })?, f572(v0.clone(), v1, v2, {
        let a2406 = v3;
        let a2407 = 8u64;
        nat_add(a2406, a2407)?
    })?, f572(v0.clone(), v1, v2, {
        let a2408 = v3;
        let a2409 = 12u64;
        nat_add(a2408, a2409)?
    })?, f572(v0.clone(), v1, v2, {
        let a2410 = v3;
        let a2411 = 16u64;
        nat_add(a2410, a2411)?
    })?, f572(v0.clone(), v1, v2, {
        let a2412 = v3;
        let a2413 = 20u64;
        nat_add(a2412, a2413)?
    })?, f572(v0.clone(), v1, v2, {
        let a2414 = v3;
        let a2415 = 24u64;
        nat_add(a2414, a2415)?
    })?, f572(v0.clone(), v1, v2, {
        let a2416 = v3;
        let a2417 = 28u64;
        nat_add(a2416, a2417)?
    })?, f572(v0.clone(), v1, v2, {
        let a2418 = v3;
        let a2419 = 32u64;
        nat_add(a2418, a2419)?
    })?, f572(v0.clone(), v1, v2, {
        let a2420 = v3;
        let a2421 = 36u64;
        nat_add(a2420, a2421)?
    })?, f572(v0.clone(), v1, v2, {
        let a2422 = v3;
        let a2423 = 40u64;
        nat_add(a2422, a2423)?
    })?, f572(v0.clone(), v1, v2, {
        let a2424 = v3;
        let a2425 = 44u64;
        nat_add(a2424, a2425)?
    })?, f572(v0.clone(), v1, v2, {
        let a2426 = v3;
        let a2427 = 48u64;
        nat_add(a2426, a2427)?
    })?, f572(v0.clone(), v1, v2, {
        let a2428 = v3;
        let a2429 = 52u64;
        nat_add(a2428, a2429)?
    })?, f572(v0.clone(), v1, v2, {
        let a2430 = v3;
        let a2431 = 56u64;
        nat_add(a2430, a2431)?
    })?, f572(v0, v1, v2, {
        let a2432 = v3;
        let a2433 = 60u64;
        nat_add(a2432, a2433)?
    })?))
}

pub fn f535(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m2434 = f573(v0.clone());
    match m2434 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f384(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f536(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt2) -> Adt2 {
    Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("citation")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f574(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(v2)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("pointer")), std::rc::Rc::new(Adt2::C5(Bytes::lit(&[]))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("value")), std::rc::Rc::new(v3), std::rc::Rc::new(Adt2::C8))))))))))
}

pub fn f537(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2435 = v0;
    if m2435 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m2435 - 1;
        let m2436 = v2.clone();
        match m2436 {
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
                    Ok(f575(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
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
            Adt2::C7(r2450) => {
                let v7 = (*r2450).clone();
                let m2437 = f576(f559(v7));
                match m2437 {
                    None => {
                        Ok(f577(f578(v2.clone(), v3.clone(), v4.clone())?, f577({
                            let m2438 = f385(f276(v2.clone(), f8(Str::lit("$ref"))));
                            match m2438 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m2439 = f579(v1.clone(), v9)?;
                                    match m2439 {
                                        Ok(v11) => {
                                            f537(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f577(f580(v5, v1.clone(), f281(f276(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f577({
                            let m2440 = f276(v2.clone(), f8(Str::lit("anyOf")));
                            match m2440 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a2441 = 0u64;
                                    let a2442 = f581(v5, v1.clone(), f281(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a2441, a2442) {
                                        None::<Bytes>
                                    } else {
                                        f575(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f577({
                            let m2443 = f276(v2.clone(), f8(Str::lit("oneOf")));
                            match m2443 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a2444 = f581(v5, v1.clone(), f281(Some(v13)), v3.clone(), v4.clone())?;
                                    let a2445 = 1u64;
                                    if nat_eq(a2444, a2445) {
                                        None::<Bytes>
                                    } else {
                                        f575(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m2446 = v3.clone();
                            match m2446 {
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
                                Adt2::C6(r2448) => {
                                    let v15 = (*r2448).clone();
                                    let m2447 = f276(v2, f8(Str::lit("items")));
                                    match m2447 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f584(v5, v1, v16, f386(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt2::C7(r2449) => {
                                    let v14 = (*r2449).clone();
                                    f577(f582(v5, v1.clone(), f522(f276(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f583(v5, v1, v2, f559(v14), v4)?)
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

pub fn f538(v0: Adt2) -> R<Bytes> {
    f384(v0)
}

pub fn f539(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2451 = v0;
    match m2451.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f354(v1)?, f539(v2)?))
        }
    }
}

pub fn f540(p2454: List<Bytes>, v1: Bytes) -> bool {
    let m2453 = p2454;
    {
        let mut m2453 = m2453;
        loop {
            let step = {
                let v0 = m2453;
                let m2452 = v0;
                match m2452.uncons() {
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
                Ok(next) => { m2453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f541(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2455 = v0;
    match m2455.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f541(v4, v1, v2)
            } else {
                List::cons(v3, f541(v4, v1, v2))
            }
        }
    }
}

pub fn f542(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2456 = v0;
    match m2456.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f395(f539(f146(List::cons(v1, f585(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f542(v3, v1)?))
            }
        }
    }
}

pub fn f543(v0: Bytes, v1: u64) -> Bytes {
    let m2460 = {
        let a2457 = v0;
        let a2458 = v1;
        let a2459 = 1u64;
        slice_bytes(a2457, a2458, a2459)
    };
    match m2460 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f544(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f586(v1, v0, v2)
}

pub fn f545(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2505 = {
            let a2481 = {
                let a2469 = {
                    let a2463 = {
                        let a2461 = v3;
                        let a2462 = 128u8;
                        fixed_u8::bit_and(a2461, a2462)
                    };
                    let a2464 = 128u8;
                    if equal(a2463, a2464) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2470 = {
                    let a2467 = {
                        let a2465 = v3;
                        let a2466 = 64u8;
                        fixed_u8::bit_and(a2465, a2466)
                    };
                    let a2468 = 64u8;
                    if equal(a2467, a2468) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2469, a2470)?
            };
            let a2482 = {
                let a2479 = {
                    let a2473 = {
                        let a2471 = v3;
                        let a2472 = 32u8;
                        fixed_u8::bit_and(a2471, a2472)
                    };
                    let a2474 = 32u8;
                    if equal(a2473, a2474) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2480 = {
                    let a2477 = {
                        let a2475 = v3;
                        let a2476 = 16u8;
                        fixed_u8::bit_and(a2475, a2476)
                    };
                    let a2478 = 16u8;
                    if equal(a2477, a2478) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2479, a2480)?
            };
            nat_add(a2481, a2482)?
        };
        let a2506 = {
            let a2503 = {
                let a2491 = {
                    let a2485 = {
                        let a2483 = v3;
                        let a2484 = 8u8;
                        fixed_u8::bit_and(a2483, a2484)
                    };
                    let a2486 = 8u8;
                    if equal(a2485, a2486) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2492 = {
                    let a2489 = {
                        let a2487 = v3;
                        let a2488 = 4u8;
                        fixed_u8::bit_and(a2487, a2488)
                    };
                    let a2490 = 4u8;
                    if equal(a2489, a2490) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2491, a2492)?
            };
            let a2504 = {
                let a2501 = {
                    let a2495 = {
                        let a2493 = v3;
                        let a2494 = 2u8;
                        fixed_u8::bit_and(a2493, a2494)
                    };
                    let a2496 = 2u8;
                    if equal(a2495, a2496) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2502 = {
                    let a2499 = {
                        let a2497 = v3;
                        let a2498 = 1u8;
                        fixed_u8::bit_and(a2497, a2498)
                    };
                    let a2500 = 1u8;
                    if equal(a2499, a2500) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2501, a2502)?
            };
            nat_add(a2503, a2504)?
        };
        nat_add(a2505, a2506)?
    };
    let a2507 = v4;
    let a2508 = 34u64;
    if nat_eq(a2507, a2508) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2509 = v4;
        let a2510 = 92u64;
        if nat_eq(a2509, a2510) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2511 = v4;
            let a2512 = 8u64;
            if nat_eq(a2511, a2512) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2513 = v4;
                let a2514 = 12u64;
                if nat_eq(a2513, a2514) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2515 = v4;
                    let a2516 = 10u64;
                    if nat_eq(a2515, a2516) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2517 = v4;
                        let a2518 = 13u64;
                        if nat_eq(a2517, a2518) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2519 = v4;
                            let a2520 = 9u64;
                            if nat_eq(a2519, a2520) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2521 = v4;
                                let a2522 = 32u64;
                                if nat_lt(a2521, a2522) {
                                    let a2523 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2524 = f587(v2)?;
                                    Ok(append_bytes(a2523, a2524))
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

pub fn f546(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f588(Fn19::F589, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f547(v0: Fn15, p2528: List<List<(Bytes, Bytes)>>, p2529: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2527 = (p2528, p2529);
    {
        let mut m2527 = m2527;
        loop {
            let step = {
                let (v1, v2) = m2527;
                let m2525 = v2;
                match m2525.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2526: Fn15 = v0.clone();
                            c2526.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f549(v0: Fn16, p2534: u64, p2535: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2533 = (p2534, p2535);
    {
        let mut m2533 = m2533;
        loop {
            let step = {
                let (v1, v2) = m2533;
                let m2530 = v1;
                if m2530 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2530 - 1;
                    let m2532 = {
                        let c2531: Fn16 = v0.clone();
                        c2531.apply(v2.clone())?
                    };
                    match m2532 {
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
                Ok(next) => { m2533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f590(v0)
}

pub fn f551(v0: u8) -> bool {
    let a2536 = v0;
    let a2537 = 32u8;
    if if if equal(a2536, a2537) {
        true
    } else {
        let a2538 = v0;
        let a2539 = 9u8;
        equal(a2538, a2539)
    } {
        true
    } else {
        let a2540 = v0;
        let a2541 = 10u8;
        equal(a2540, a2541)
    } {
        true
    } else {
        let a2542 = v0;
        let a2543 = 12u8;
        if equal(a2542, a2543) {
            true
        } else {
            let a2544 = v0;
            let a2545 = 13u8;
            equal(a2544, a2545)
        }
    }
}

pub fn f552(v0: Fn17, p2550: u64, p2551: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2549 = (p2550, p2551);
    {
        let mut m2549 = m2549;
        loop {
            let step = {
                let (v1, v2) = m2549;
                let m2546 = v1;
                if m2546 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2546 - 1;
                    let m2548 = {
                        let c2547: Fn17 = v0.clone();
                        c2547.apply(v2.clone())?
                    };
                    match m2548 {
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
                Ok(next) => { m2549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f591(v0, v1)
}

pub fn f554(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2596 = {
            let a2572 = {
                let a2560 = {
                    let a2554 = {
                        let a2552 = v1;
                        let a2553 = 128u8;
                        fixed_u8::bit_and(a2552, a2553)
                    };
                    let a2555 = 128u8;
                    if equal(a2554, a2555) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2561 = {
                    let a2558 = {
                        let a2556 = v1;
                        let a2557 = 64u8;
                        fixed_u8::bit_and(a2556, a2557)
                    };
                    let a2559 = 64u8;
                    if equal(a2558, a2559) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2560, a2561)?
            };
            let a2573 = {
                let a2570 = {
                    let a2564 = {
                        let a2562 = v1;
                        let a2563 = 32u8;
                        fixed_u8::bit_and(a2562, a2563)
                    };
                    let a2565 = 32u8;
                    if equal(a2564, a2565) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2571 = {
                    let a2568 = {
                        let a2566 = v1;
                        let a2567 = 16u8;
                        fixed_u8::bit_and(a2566, a2567)
                    };
                    let a2569 = 16u8;
                    if equal(a2568, a2569) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2570, a2571)?
            };
            nat_add(a2572, a2573)?
        };
        let a2597 = {
            let a2594 = {
                let a2582 = {
                    let a2576 = {
                        let a2574 = v1;
                        let a2575 = 8u8;
                        fixed_u8::bit_and(a2574, a2575)
                    };
                    let a2577 = 8u8;
                    if equal(a2576, a2577) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2583 = {
                    let a2580 = {
                        let a2578 = v1;
                        let a2579 = 4u8;
                        fixed_u8::bit_and(a2578, a2579)
                    };
                    let a2581 = 4u8;
                    if equal(a2580, a2581) {
                        4u64
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
                        let a2585 = 2u8;
                        fixed_u8::bit_and(a2584, a2585)
                    };
                    let a2587 = 2u8;
                    if equal(a2586, a2587) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2593 = {
                    let a2590 = {
                        let a2588 = v1;
                        let a2589 = 1u8;
                        fixed_u8::bit_and(a2588, a2589)
                    };
                    let a2591 = 1u8;
                    if equal(a2590, a2591) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2592, a2593)?
            };
            nat_add(a2594, a2595)?
        };
        nat_add(a2596, a2597)?
    };
    let a2598 = 48u64;
    let a2599 = v2;
    if nat_le(a2598, a2599) {
        let a2600 = v2;
        let a2601 = 57u64;
        Ok(nat_le(a2600, a2601))
    } else {
        Ok(false)
    }
}

pub fn f555(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f444({
        let (_, h2602) = {
            let (h2603, _) = f592(Fn20::F593, {
                let a2605 = {
                    let a2604 = v0.clone();
                    length_list(a2604)
                };
                let a2606 = 1u64;
                nat_add(a2605, a2606)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2603
        };
        h2602
    }))
}

pub fn f556(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f557(v0: List<Adt14>) -> List<Bytes> {
    let m2607 = v0;
    match m2607.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2608 = f94(v1, f8(Str::lit("std")));
            match m2608 {
                None => {
                    f557(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f557(v2))
                }
            }
        }
    }
}

pub fn f558(v0: List<Adt14>) -> List<Bytes> {
    let m2609 = v0;
    match m2609.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2610 = f93(f94(v1, f8(Str::lit("req"))));
            match m2610 {
                None => {
                    f558(v2)
                }
                Some(v3) => {
                    List::cons(v3, f558(v2))
                }
            }
        }
    }
}

pub fn f559(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2611 = v0;
    match m2611 {
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
        Adt2::C7(r2612) => {
            let v4 = (*r2612).clone();
            f559(v4)
        }
        Adt2::C8 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C9(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(v1, r2613, r2614) => {
            let v2 = (*r2613).clone();
            let v3 = (*r2614).clone();
            List::cons((v1, v2), f559(v3))
        }
    }
}

pub fn f560(p2619: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2618 = p2619;
    {
        let mut m2618 = m2618;
        loop {
            let step = {
                let v0 = m2618;
                let m2615 = v0;
                match m2615.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f594(f595({
                            let (_, h2616) = v3.clone();
                            h2616
                        }, List::<Bytes>::nil())?, f562({
                            let (h2617, _) = v3.clone();
                            h2617
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2618 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2620 = v0;
    match m2620.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2621, _) = v2.clone();
                h2621
            }, v1.clone()) {
                f561(v3, v1)
            } else {
                List::cons(v2, f561(v3, v1))
            }
        }
    }
}

pub fn f562(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f379(v0)?;
    if f52(v1.clone(), f596()) {
        f597(v1)
    } else {
        Ok(v1)
    }
}

pub fn f563() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f564() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f565(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    f598(v0, f599(v1))
}

pub fn f566(v0: Adt28) -> List<Adt25> {
    let m2622 = v0;
    match m2622 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f567(v0: Adt28) -> Bytes {
    let m2623 = v0;
    match m2623 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f568(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2628 = {
        let a2625 = f469({
            let (h2624, _) = v0;
            h2624
        })?;
        let a2626 = Bytes::lit(&[0x3a]);
        append_bytes(a2625, a2626)
    };
    let a2629 = {
        let (_, h2627) = v0;
        h2627
    };
    Ok(append_bytes(a2628, a2629))
}

pub fn f569(v0: Fn18, p2633: u64, p2634: Adt27) -> R<Adt27> {
    let m2632 = (p2633, p2634);
    {
        let mut m2632 = m2632;
        loop {
            let step = {
                let (v1, v2) = m2632;
                let m2630 = v1;
                if m2630 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2630 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2631: Fn18 = v0.clone();
                        c2631.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2632 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: Adt27) -> R<Adt27> {
    f600(v0)
}

pub fn f571(v0: u32, v1: u32) -> u32 {
    let m2637 = {
        let a2635 = f601(v0);
        let a2636 = f601(v1);
        fixed_u64::checked_add(a2635, a2636)
    };
    match m2637 {
        None => {
            0u32
        }
        Some(v2) => {
            f602(v2)
        }
    }
}

pub fn f572(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2640 = {
        let a2638 = v3;
        let a2639 = 8u64;
        nat_add(a2638, a2639)?
    };
    let a2641 = v2;
    if nat_eq(a2640, a2641) {
        Ok(f603({
            let a2644 = {
                let a2642 = v1;
                let a2643 = 8u64;
                nat_mul(a2642, a2643)?
            };
            let a2645 = 4294967296u64;
            let a2646 = 0u64;
            nat_quot(a2644, a2645, a2646)
        }))
    } else {
        let a2649 = {
            let a2647 = v3;
            let a2648 = 4u64;
            nat_add(a2647, a2648)?
        };
        let a2650 = v2;
        if nat_eq(a2649, a2650) {
            Ok(f603({
                let a2653 = {
                    let a2651 = v1;
                    let a2652 = 8u64;
                    nat_mul(a2651, a2652)?
                };
                let a2654 = 4294967296u64;
                let a2655 = 0u64;
                nat_rem(a2653, a2654, a2655)
            }))
        } else {
            Ok(f604(f605(v0.clone(), v1, v3), f605(v0.clone(), v1, {
                let a2656 = v3;
                let a2657 = 1u64;
                nat_add(a2656, a2657)?
            }), f605(v0.clone(), v1, {
                let a2658 = v3;
                let a2659 = 2u64;
                nat_add(a2658, a2659)?
            }), f605(v0, v1, {
                let a2660 = v3;
                let a2661 = 3u64;
                nat_add(a2660, a2661)?
            })))
        }
    }
}

pub fn f573(v0: Adt2) -> Option<Bytes> {
    let m2662 = v0;
    match m2662 {
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
        Adt2::C6(r2665) => {
            let v3 = (*r2665).clone();
            f573(v3)
        }
        Adt2::C7(r2666) => {
            let v4 = (*r2666).clone();
            f573(v4)
        }
        Adt2::C8 => {
            None::<Bytes>
        }
        Adt2::C9(r2667, r2668) => {
            let v5 = (*r2667).clone();
            let v6 = (*r2668).clone();
            let m2663 = f573(v5);
            match m2663 {
                None => {
                    f573(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r2669, r2670) => {
            let v9 = (*r2669).clone();
            let v10 = (*r2670).clone();
            let m2664 = f573(v9);
            match m2664 {
                None => {
                    f573(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f574(v0: List<Bytes>) -> Adt2 {
    let m2671 = v0;
    match m2671.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(f574(v2)))
        }
    }
}

pub fn f575(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f576(p2676: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m2675 = p2676;
    {
        let mut m2675 = m2675;
        loop {
            let step = {
                let v0 = m2675;
                let m2672 = v0;
                match m2672.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h2673, _) = v1.clone();
                            h2673
                        }, f606()) {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h2674, _) = v1;
                                h2674
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2675 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f577(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2677 = v0;
    match m2677 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f578(v0: Adt2, v1: Adt2, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f607(v1.clone());
    let v7: Option<Bytes> = {
        let m2678 = f276(v0.clone(), f8(Str::lit("type")));
        match m2678 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2679 = f385(Some(v4.clone()));
                    match m2679 {
                        None => {
                            f280(f281(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f608(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f575(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f395(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2680 = f276(v0.clone(), f8(Str::lit("enum")));
        match m2680 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f609(f281(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f575(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2681 = f276(v0.clone(), f8(Str::lit("const")));
        match m2681 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f610(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f575(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v41: Option<Bytes> = {
        let m2682 = v1.clone();
        match m2682 {
            Adt2::C0 => {
                None::<Bytes>
            }
            Adt2::C1(v28, _) => {
                f575(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
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
                let m2683 = f611(v12.clone())?;
                match m2683 {
                    None => {
                        f575(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f612(v13)?;
                        f577({
                            let m2684 = f613(f276(v0.clone(), f8(Str::lit("minLength"))));
                            match m2684 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2685 = v14;
                                    let a2686 = v15;
                                    if nat_lt(a2685, a2686) {
                                        f575(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f577({
                            let m2687 = f613(f276(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2687 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2688 = v16;
                                    let a2689 = v14;
                                    if nat_lt(a2688, a2689) {
                                        f575(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2690 = f385(f276(v0.clone(), f8(Str::lit("pattern"))));
                            match m2690 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2691 = f614(v17.clone(), v12)?;
                                    match m2691 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f575(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
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
            Adt2::C6(r2699) => {
                let v20 = (*r2699).clone();
                let v21: u64 = f615(v20)?;
                f577({
                    let m2692 = f613(f276(v0.clone(), f8(Str::lit("minItems"))));
                    match m2692 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2693 = v21;
                            let a2694 = v22;
                            if nat_lt(a2693, a2694) {
                                f575(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2695 = f613(f276(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2695 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2696 = v23;
                            let a2697 = v21;
                            if nat_lt(a2696, a2697) {
                                f575(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt2::C7(r2700) => {
                let v24 = (*r2700).clone();
                let v25: List<Bytes> = f616(f280(f281(f276(v0.clone(), f8(Str::lit("required"))))), f617(f559(v24)));
                let m2698 = v25;
                match m2698.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f575(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
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
        let m2701 = f618(v1);
        match m2701 {
            None => {
                None::<Bytes>
            }
            Some(v42) => {
                f577({
                    let m2703 = {
                        let m2702 = f276(v0.clone(), f8(Str::lit("minimum")));
                        match m2702 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v43) => {
                                f618(v43)
                            }
                        }
                    };
                    match m2703 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v44) => {
                            if f619(v42, v44)? {
                                f575(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2705 = {
                        let m2704 = f276(v0, f8(Str::lit("maximum")));
                        match m2704 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v45) => {
                                f618(v45)
                            }
                        }
                    };
                    match m2705 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v46) => {
                            if f619(v46, v42)? {
                                f575(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f577(v7, f577(v9, f577(v11, f577(v41, v47)))))
}

pub fn f579(v0: Adt2, v1: Bytes) -> R<Result<Adt2, Bytes>> {
    let v2: u64 = {
        let a2706 = v1.clone();
        length_bytes(a2706)
    };
    if f32(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt2, Bytes>(v0))
    } else {
        let m2711 = if f85(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f46(v1.clone(), 8u64, {
                let a2707 = v2;
                let a2708 = 8u64;
                nat_sub(a2707, a2708)
            })))
        } else {
            if f85(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f46(v1.clone(), 14u64, {
                    let a2709 = v2;
                    let a2710 = 14u64;
                    nat_sub(a2709, a2710)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2711 {
            None => {
                Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f620({
                    let a2714 = {
                        let a2713 = {
                            let (_, h2712) = v3.clone();
                            h2712
                        };
                        length_bytes(a2713)
                    };
                    let a2715 = 1u64;
                    nat_add(a2714, a2715)?
                }, {
                    let (_, h2716) = v3;
                    h2716
                }, 0u64, Bytes::lit(&[]))?;
                let m2719 = {
                    let m2718 = f276(v0, {
                        let (h2717, _) = v3;
                        h2717
                    });
                    match m2718 {
                        None => {
                            None::<Adt2>
                        }
                        Some(v5) => {
                            f276(v5, v4)
                        }
                    }
                };
                match m2719 {
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

pub fn f580(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2720 = v0;
    if m2720 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2720 - 1;
        let m2721 = v2;
        match m2721.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f577(f537(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f580(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f581(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<u64> {
    let m2722 = v0;
    if m2722 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2722 - 1;
        let m2723 = v2;
        match m2723.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2725 = {
                    let m2724 = f537(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2724 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2726 = f581(v5, v1, v7, v3, v4)?;
                nat_add(a2725, a2726)
            }
        }
    }
}

pub fn f582(v0: u64, v1: Adt2, v2: List<(Bytes, Adt2)>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2727 = v0;
    if m2727 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2727 - 1;
        let m2728 = v2;
        match m2728.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f577({
                    let m2730 = f276(v3.clone(), {
                        let (h2729, _) = v6.clone();
                        h2729
                    });
                    match m2730 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f537(v5, v1.clone(), {
                                let (_, h2731) = v6;
                                h2731
                            }, v8, f621(v4.clone(), {
                                let (h2732, _) = v6;
                                h2732
                            })?)?
                        }
                    }
                }, f582(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f583(v0: u64, v1: Adt2, v2: Adt2, v3: List<(Bytes, Adt2)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2733 = v0;
    if m2733 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2733 - 1;
        let m2734 = v3;
        match m2734.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f621(v4.clone(), {
                    let (h2735, _) = v6.clone();
                    h2735
                })?;
                let m2737 = f622(f522(f276(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2736, _) = v6.clone();
                    h2736
                })?;
                match m2737 {
                    Ok(v10) => {
                        Ok(f577(f580(v5, v1.clone(), v10.clone(), {
                            let (_, h2738) = v6.clone();
                            h2738
                        }, v8.clone())?, f577({
                            let m2739 = v10;
                            match m2739.uncons() {
                                None => {
                                    if f52({
                                        let (h2740, _) = v6;
                                        h2740
                                    }, f617(f559({
                                        let m2741 = f276(v2.clone(), f8(Str::lit("properties")));
                                        match m2741 {
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
                                        let m2742 = f276(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2742 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f537(v5, v1.clone(), v14, {
                                                    let (_, h2743) = v6;
                                                    h2743
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f583(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f584(v0: u64, v1: Adt2, v2: Adt2, v3: List<Adt2>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2744 = v0;
    if m2744 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2744 - 1;
        let m2745 = v3;
        match m2745.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f577(f537(v6, v1.clone(), v2.clone(), v7, f621(v4.clone(), f404(v5)?)?)?, f584(v6, v1, v2, v8, v4, {
                    let a2746 = v5;
                    let a2747 = 1u64;
                    nat_add(a2746, a2747)?
                })?))
            }
        }
    }
}

pub fn f585(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2748 = v0;
    match m2748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f585(v3, false))
            } else {
                f585(v3, true)
            }
        }
    }
}

pub fn f586(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2749, _) = v2;
        h2749
    };
    let a2750 = v3;
    let a2751 = 0u64;
    if nat_eq(a2750, a2751) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2752 = v3;
            let a2753 = v1;
            let a2754 = 0u64;
            nat_quot(a2752, a2753, a2754)
        }, {
            let a2759 = f543(v0, {
                let a2755 = v3;
                let a2756 = v1;
                let a2757 = 0u64;
                nat_rem(a2755, a2756, a2757)
            });
            let a2760 = {
                let (_, h2758) = v2;
                h2758
            };
            append_bytes(a2759, a2760)
        }))
    }
}

pub fn f587(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2805 = {
            let a2781 = {
                let a2769 = {
                    let a2763 = {
                        let a2761 = v1;
                        let a2762 = 128u8;
                        fixed_u8::bit_and(a2761, a2762)
                    };
                    let a2764 = 128u8;
                    if equal(a2763, a2764) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2770 = {
                    let a2767 = {
                        let a2765 = v1;
                        let a2766 = 64u8;
                        fixed_u8::bit_and(a2765, a2766)
                    };
                    let a2768 = 64u8;
                    if equal(a2767, a2768) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2769, a2770)?
            };
            let a2782 = {
                let a2779 = {
                    let a2773 = {
                        let a2771 = v1;
                        let a2772 = 32u8;
                        fixed_u8::bit_and(a2771, a2772)
                    };
                    let a2774 = 32u8;
                    if equal(a2773, a2774) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2780 = {
                    let a2777 = {
                        let a2775 = v1;
                        let a2776 = 16u8;
                        fixed_u8::bit_and(a2775, a2776)
                    };
                    let a2778 = 16u8;
                    if equal(a2777, a2778) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2779, a2780)?
            };
            nat_add(a2781, a2782)?
        };
        let a2806 = {
            let a2803 = {
                let a2791 = {
                    let a2785 = {
                        let a2783 = v1;
                        let a2784 = 8u8;
                        fixed_u8::bit_and(a2783, a2784)
                    };
                    let a2786 = 8u8;
                    if equal(a2785, a2786) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2792 = {
                    let a2789 = {
                        let a2787 = v1;
                        let a2788 = 4u8;
                        fixed_u8::bit_and(a2787, a2788)
                    };
                    let a2790 = 4u8;
                    if equal(a2789, a2790) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2791, a2792)?
            };
            let a2804 = {
                let a2801 = {
                    let a2795 = {
                        let a2793 = v1;
                        let a2794 = 2u8;
                        fixed_u8::bit_and(a2793, a2794)
                    };
                    let a2796 = 2u8;
                    if equal(a2795, a2796) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2802 = {
                    let a2799 = {
                        let a2797 = v1;
                        let a2798 = 1u8;
                        fixed_u8::bit_and(a2797, a2798)
                    };
                    let a2800 = 1u8;
                    if equal(a2799, a2800) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2801, a2802)?
            };
            nat_add(a2803, a2804)?
        };
        nat_add(a2805, a2806)?
    };
    let a2813 = f543(f623(), {
        let a2807 = v2;
        let a2808 = 16u64;
        let a2809 = 0u64;
        nat_quot(a2807, a2808, a2809)
    });
    let a2814 = f543(f623(), {
        let a2810 = v2;
        let a2811 = 16u64;
        let a2812 = 0u64;
        nat_rem(a2810, a2811, a2812)
    });
    Ok(append_bytes(a2813, a2814))
}

pub fn f588(v0: Fn19, p2818: List<List<(Bytes, Bytes)>>, p2819: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2817 = (p2818, p2819);
    {
        let mut m2817 = m2817;
        loop {
            let step = {
                let (v1, v2) = m2817;
                let m2815 = v2;
                match m2815.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2816: Fn19 = v0.clone();
                            c2816.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f590(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2821 = {
        let a2820 = v0.clone();
        length_list(a2820)
    };
    let a2822 = 1u64;
    if nat_le(a2821, a2822) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f624(v0)?))
    }
}

pub fn f591(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2823, _) = v1;
        h2823
    };
    let m2825 = {
        let (_, h2824) = v1;
        h2824
    };
    match m2825 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2828 = {
                let a2826 = v0.clone();
                let a2827 = v2;
                index_bytes(a2826, a2827)
            };
            match m2828 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2833 = {
                        let a2831 = v0;
                        let a2832 = {
                            let a2829 = v2;
                            let a2830 = 1u64;
                            nat_add(a2829, a2830)?
                        };
                        index_bytes(a2831, a2832)
                    };
                    match m2833 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f625(v4)? {
                                f625(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2834 = v2;
                                    let a2835 = 2u64;
                                    nat_add(a2834, a2835)?
                                }, Some({
                                    let a2840 = v3;
                                    let a2841 = f543(f626(), {
                                        let a2838 = {
                                            let a2836 = f627(v4)?;
                                            let a2837 = 16u64;
                                            nat_mul(a2836, a2837)?
                                        };
                                        let a2839 = f627(v5)?;
                                        nat_add(a2838, a2839)?
                                    });
                                    append_bytes(a2840, a2841)
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

pub fn f592(v0: Fn20, p2846: u64, p2847: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2845 = (p2846, p2847);
    {
        let mut m2845 = m2845;
        loop {
            let step = {
                let (v1, v2) = m2845;
                let m2842 = v1;
                if m2842 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2842 - 1;
                    let m2844 = {
                        let c2843: Fn20 = v0.clone();
                        c2843.apply(v2.clone())?
                    };
                    match m2844 {
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
                Ok(next) => { m2845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f628(v0)
}

pub fn f594(p2850: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2849 = p2850;
    {
        let mut m2849 = m2849;
        loop {
            let step = {
                let v0 = m2849;
                let m2848 = v0;
                match m2848.uncons() {
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
                Ok(next) => { m2849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2851 = v0;
    match m2851 {
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
        Adt2::C6(r2854) => {
            let v2 = (*r2854).clone();
            f595(v2, v1)
        }
        Adt2::C7(r2855) => {
            let v3 = (*r2855).clone();
            f595(v3, v1)
        }
        Adt2::C8 => {
            Ok(v1)
        }
        Adt2::C9(r2856, r2857) => {
            let v4 = (*r2856).clone();
            let v5 = (*r2857).clone();
            f595(v4, f595(v5, v1)?)
        }
        Adt2::C10(v6, r2858, r2859) => {
            let v7 = (*r2858).clone();
            let v8 = (*r2859).clone();
            let v9: List<Bytes> = f595(v7.clone(), f595(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2852 = f385(Some(v7));
                match m2852 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2853 = f629(v10)?;
                        match m2853 {
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

pub fn f596() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f597(v0: Bytes) -> R<Bytes> {
    let m2862 = {
        let a2860 = v0.clone();
        let a2861 = 0u64;
        index_bytes(a2860, a2861)
    };
    match m2862 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2868 = {
                let a2863 = f8(Str::lit("u"));
                let a2864 = f630(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2863, a2864)
            };
            let a2869 = f46(v0.clone(), 1u64, {
                let a2866 = {
                    let a2865 = v0;
                    length_bytes(a2865)
                };
                let a2867 = 1u64;
                nat_sub(a2866, a2867)
            });
            Ok(append_bytes(a2868, a2869))
        }
    }
}

pub fn f598(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    let m2870 = f276(v1.clone(), f8(Str::lit("enum")));
    match m2870 {
        None => {
            let m2872 = f276(v1.clone(), f8(Str::lit("oneOf")));
            match m2872 {
                None => {
                    if f635(v1.clone(), v0.clone())? {
                        let m2873 = f636(v0.clone(), f522(f276(v1.clone(), f8(Str::lit("properties")))), f280(f281(f276(v1, f8(Str::lit("required"))))), true)?;
                        match m2873 {
                            Ok(v7) => {
                                Ok(f633(f637(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f395(f638(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f639(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2874 = f522(f276(v1.clone(), f8(Str::lit("properties"))));
                        match m2874.uncons() {
                            None => {
                                if f640(v1.clone()) {
                                    Ok(f633(f641(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2877 = f642(v1, {
                                        let a2875 = v0.clone();
                                        let a2876 = f8(Str::lit("Value"));
                                        append_bytes(a2875, a2876)
                                    })?;
                                    match m2877 {
                                        Ok(v9) => {
                                            Ok(f633(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f567(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f566(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2878 = f636(v0.clone(), f522(f276(v1.clone(), f8(Str::lit("properties")))), f280(f281(f276(v1, f8(Str::lit("required"))))), false)?;
                                match m2878 {
                                    Ok(v13) => {
                                        Ok(f633(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2879 = f638(v13.clone());
                                            match m2879.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f395(f638(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f639(v13)))
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
                    f634(v0, f386(v5))
                }
            }
        }
        Some(v2) => {
            let m2871 = f631(f386(v2), List::<Bytes>::nil())?;
            match m2871 {
                Ok(v4) => {
                    Ok(f633(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f395(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f632(v3, v0))
                }
            }
        }
    }
}

pub fn f599(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2880 = f276(v0.clone(), f8(Str::lit("anyOf")));
        match m2880 {
            None => {
                v0
            }
            Some(v1) => {
                if f643(f386(v1.clone())) {
                    f644(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f645(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f646(f386(v1.clone()))
                    } {
                        v0
                    } else {
                        f647(f644(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2881 = f276(v2.clone(), f8(Str::lit("oneOf")));
        match m2881 {
            None => {
                v2
            }
            Some(v3) => {
                if f643(f386(v3)) {
                    f644(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2882 = f276(v4.clone(), f8(Str::lit("allOf")));
    match m2882 {
        None => {
            v4
        }
        Some(v5) => {
            if f648(f386(v5)) {
                f644(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f600(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f649(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2883) => {
            h2883
        }
    });
    let v2: u32 = f571(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2884, _, _) => {
            h2884
        }
    }, f571(f650(match v0.clone() {
        Adt27::C0(_, _, _, _, h2885, _, _, _, _, _) => {
            h2885
        }
    }), f571(f651(match v0.clone() {
        Adt27::C0(_, _, _, _, h2886, _, _, _, _, _) => {
            h2886
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2887, _, _, _, _) => {
            h2887
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2888, _, _, _) => {
            h2888
        }
    }), f571(v1, {
        let m2891 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2890, _) => {
                h2890
            }
        };
        match m2891 {
            Adt26::C0(h2889, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2889
            }
        }
    }))));
    let v3: u32 = f571(f652(match v0.clone() {
        Adt27::C0(h2892, _, _, _, _, _, _, _, _, _) => {
            h2892
        }
    }), f653(match v0.clone() {
        Adt27::C0(h2893, _, _, _, _, _, _, _, _, _) => {
            h2893
        }
    }, match v0.clone() {
        Adt27::C0(_, h2894, _, _, _, _, _, _, _, _) => {
            h2894
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2895, _, _, _, _, _, _, _) => {
            h2895
        }
    }));
    Ok(Adt27::C0(f571(v2, v3), match v0.clone() {
        Adt27::C0(h2896, _, _, _, _, _, _, _, _, _) => {
            h2896
        }
    }, match v0.clone() {
        Adt27::C0(_, h2897, _, _, _, _, _, _, _, _) => {
            h2897
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2898, _, _, _, _, _, _, _) => {
            h2898
        }
    }, f571(match v0.clone() {
        Adt27::C0(_, _, _, h2899, _, _, _, _, _, _) => {
            h2899
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2900, _, _, _, _, _) => {
            h2900
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2901, _, _, _, _) => {
            h2901
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2902, _, _, _) => {
            h2902
        }
    }, f654(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2903, _) => {
            h2903
        }
    }), {
        let a2905 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2904) => {
                h2904
            }
        };
        let a2906 = 1u64;
        nat_add(a2905, a2906)?
    }))
}

pub fn f601(v0: u32) -> u64 {
    let m2908 = {
        let a2907 = v0;
        fixed_u64::convert(i128::from(a2907))
    };
    match m2908 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f602(v0: u64) -> u32 {
    let m2912 = {
        let a2911 = {
            let a2909 = v0;
            let a2910 = 4294967295u64;
            fixed_u64::bit_and(a2909, a2910)
        };
        fixed_u32::convert(i128::from(a2911))
    };
    match m2912 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f603(v0: u64) -> u32 {
    let (_, h2913) = {
        let (_, h2914) = f655(Fn21::F656, 32u64, (v0, (1u32, 0u32)));
        h2914
    };
    h2913
}

pub fn f604(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2919 = f657(v0, 24u32);
    let a2920 = {
        let a2917 = f657(v1, 16u32);
        let a2918 = {
            let a2915 = f657(v2, 8u32);
            let a2916 = v3;
            fixed_u32::bit_or(a2915, a2916)
        };
        fixed_u32::bit_or(a2917, a2918)
    };
    fixed_u32::bit_or(a2919, a2920)
}

pub fn f605(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2921 = v2;
    let a2922 = v1;
    if nat_lt(a2921, a2922) {
        let m2925 = {
            let a2923 = v0;
            let a2924 = v2;
            index_bytes(a2923, a2924)
        };
        match m2925 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2927 = {
                    let a2926 = v3;
                    fixed_u32::convert(i128::from(a2926))
                };
                match m2927 {
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
        let a2928 = v2;
        let a2929 = v1;
        if nat_eq(a2928, a2929) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f606() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f607(v0: Adt2) -> Bytes {
    let m2930 = v0;
    match m2930 {
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

pub fn f608(p2933: List<Bytes>, v1: Adt2) -> bool {
    let m2932 = p2933;
    {
        let mut m2932 = m2932;
        loop {
            let step = {
                let v0 = m2932;
                let m2931 = v0;
                match m2931.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f658(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2932 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f609(p2936: List<Adt2>, v1: Adt2) -> R<bool> {
    let m2935 = p2936;
    {
        let mut m2935 = m2935;
        loop {
            let step = {
                let v0 = m2935;
                let m2934 = v0;
                match m2934.uncons() {
                    None => {
                        Err::<List<Adt2>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f610(false, v2, v1.clone())? {
                            Err::<List<Adt2>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt2>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2935 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f610(v0: bool, v1: Adt2, v2: Adt2) -> R<bool> {
    let m2937 = v1;
    match m2937 {
        Adt2::C0 => {
            Ok(f32(f538(v2)?, f8(Str::lit("null"))))
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(v77) => {
            Ok(f32(f538(v2)?, f538(Adt2::C2(v77))?))
        }
        Adt2::C3(v78, v79) => {
            Ok(f32(f538(v2)?, f538(Adt2::C3(v78, v79))?))
        }
        Adt2::C4(v80, v81, v82) => {
            Ok(f32(f538(v2)?, f538(Adt2::C4(v80, v81, v82))?))
        }
        Adt2::C5(v83) => {
            let m2949 = v2;
            match m2949 {
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
        Adt2::C6(r2950) => {
            let v58 = (*r2950).clone();
            let m2947 = v2;
            match m2947 {
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
                Adt2::C6(r2948) => {
                    let v59 = (*r2948).clone();
                    f610(false, v58, v59)
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
        Adt2::C7(r2951) => {
            let v41 = (*r2951).clone();
            let m2943 = v2;
            match m2943 {
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
                Adt2::C7(r2946) => {
                    let v42 = (*r2946).clone();
                    if f118({
                        let a2944 = f615(v41.clone())?;
                        let a2945 = f615(v42.clone())?;
                        nat_eq(a2944, a2945)
                    }, true) {
                        f610(true, v41, v42)
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
                let m2939 = v2;
                match m2939 {
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
        Adt2::C9(r2952, r2953) => {
            let v23 = (*r2952).clone();
            let v24 = (*r2953).clone();
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
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(r2941, r2942) => {
                    let v25 = (*r2941).clone();
                    let v26 = (*r2942).clone();
                    if f610(false, v23, v25)? {
                        f610(false, v24, v26)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C10(v3, r2954, r2955) => {
            let v4 = (*r2954).clone();
            let v5 = (*r2955).clone();
            let m2938 = f276(v2.clone(), v3);
            if match m2938 {
                None => {
                    false
                }
                Some(v6) => {
                    f610(false, v4, v6)?
                }
            } {
                f610(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f611(v0: Bytes) -> R<Option<List<u64>>> {
    f659({
        let a2957 = {
            let a2956 = v0.clone();
            length_bytes(a2956)
        };
        let a2958 = 1u64;
        nat_add(a2957, a2958)?
    }, v0, 0u64)
}

pub fn f612(v0: List<u64>) -> R<u64> {
    let m2959 = v0;
    match m2959.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2960 = f612(v2)?;
            let a2961 = 1u64;
            nat_add(a2960, a2961)
        }
    }
}

pub fn f613(v0: Option<Adt2>) -> Option<u64> {
    let m2962 = v0;
    match m2962 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2963 = v1;
            match m2963 {
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

pub fn f614(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m2964 = f660(v0)?;
    match m2964 {
        Ok(v3) => {
            let m2965 = f611(v1)?;
            match m2965 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h2966, _) = {
                            let (h2967, _) = v3.clone();
                            h2967
                        };
                        if h2966 {
                            f661({
                                let a2971 = {
                                    let a2969 = f662({
                                        let (_, h2968) = v3.clone();
                                        h2968
                                    })?;
                                    let a2970 = f612(v4.clone())?;
                                    nat_add(a2969, a2970)?
                                };
                                let a2972 = 2u64;
                                nat_add(a2971, a2972)?
                            }, {
                                let (_, h2973) = v3;
                                h2973
                            }, v4, {
                                let (_, h2974) = {
                                    let (h2975, _) = v3;
                                    h2975
                                };
                                h2974
                            })?
                        } else {
                            f663({
                                let (_, h2976) = v3;
                                h2976
                            }, v4, {
                                let (_, h2977) = {
                                    let (h2978, _) = v3;
                                    h2978
                                };
                                h2977
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

pub fn f615(v0: Adt2) -> R<u64> {
    let m2979 = v0;
    match m2979 {
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
        Adt2::C9(_, r2984) => {
            let v5 = (*r2984).clone();
            let a2982 = f615(v5)?;
            let a2983 = 1u64;
            nat_add(a2982, a2983)
        }
        Adt2::C10(_, _, r2985) => {
            let v3 = (*r2985).clone();
            let a2980 = f615(v3)?;
            let a2981 = 1u64;
            nat_add(a2980, a2981)
        }
    }
}

pub fn f616(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2986 = v0;
    match m2986.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f616(v3, v1)
            } else {
                List::cons(v2, f616(v3, v1))
            }
        }
    }
}

pub fn f617(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m2987 = v0;
    match m2987.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2988, _) = v1;
                h2988
            }, f617(v2))
        }
    }
}

pub fn f618(v0: Adt2) -> Option<(bool, (u64, u64))> {
    let m2989 = v0;
    match m2989 {
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

pub fn f619(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a2994 = {
            let (h2990, _) = {
                let (_, h2991) = v0;
                h2991
            };
            h2990
        };
        let a2995 = f664({
            let (_, h2992) = {
                let (_, h2993) = v1;
                h2993
            };
            h2992
        })?;
        nat_mul(a2994, a2995)?
    };
    let v3: u64 = {
        let a3000 = {
            let (h2996, _) = {
                let (_, h2997) = v1;
                h2997
            };
            h2996
        };
        let a3001 = f664({
            let (_, h2998) = {
                let (_, h2999) = v0;
                h2999
            };
            h2998
        })?;
        nat_mul(a3000, a3001)?
    };
    let (h3002, _) = v0;
    if h3002 {
        let (h3003, _) = v1;
        if h3003 {
            let a3004 = v3;
            let a3005 = v2;
            Ok(nat_lt(a3004, a3005))
        } else {
            Ok(true)
        }
    } else {
        let (h3006, _) = v1;
        if h3006 {
            Ok(false)
        } else {
            let a3007 = v2;
            let a3008 = v3;
            Ok(nat_lt(a3007, a3008))
        }
    }
}

pub fn f620(p3038: u64, v1: Bytes, p3039: u64, p3040: Bytes) -> R<Bytes> {
    let m3037 = (p3038, (p3039, p3040));
    {
        let mut m3037 = m3037;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3037;
                let m3009 = v0;
                if m3009 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3009 - 1;
                    let a3011 = {
                        let a3010 = v1.clone();
                        length_bytes(a3010)
                    };
                    let a3012 = v2;
                    if nat_le(a3011, a3012) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f118({
                            let a3013 = f116(v1.clone(), v2)?;
                            let a3014 = 126u64;
                            nat_eq(a3013, a3014)
                        }, {
                            let a3017 = f116(v1.clone(), {
                                let a3015 = v2;
                                let a3016 = 1u64;
                                nat_add(a3015, a3016)?
                            })?;
                            let a3018 = 48u64;
                            nat_eq(a3017, a3018)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3019 = v2;
                                let a3020 = 2u64;
                                nat_add(a3019, a3020)?
                            }, {
                                let a3021 = v3;
                                let a3022 = f8(Str::lit("~"));
                                append_bytes(a3021, a3022)
                            })))
                        } else {
                            if f118({
                                let a3023 = f116(v1.clone(), v2)?;
                                let a3024 = 126u64;
                                nat_eq(a3023, a3024)
                            }, {
                                let a3027 = f116(v1.clone(), {
                                    let a3025 = v2;
                                    let a3026 = 1u64;
                                    nat_add(a3025, a3026)?
                                })?;
                                let a3028 = 49u64;
                                nat_eq(a3027, a3028)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3029 = v2;
                                    let a3030 = 2u64;
                                    nat_add(a3029, a3030)?
                                }, {
                                    let a3031 = v3;
                                    let a3032 = f8(Str::lit("/"));
                                    append_bytes(a3031, a3032)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3033 = v2;
                                    let a3034 = 1u64;
                                    nat_add(a3033, a3034)?
                                }, {
                                    let a3035 = v3;
                                    let a3036 = f46(v1.clone(), v2, 1u64);
                                    append_bytes(a3035, a3036)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f665({
        let a3042 = {
            let a3041 = v1.clone();
            length_bytes(a3041)
        };
        let a3043 = 1u64;
        nat_add(a3042, a3043)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f622(v0: List<(Bytes, Adt2)>, v1: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let m3044 = v0;
    match m3044.uncons() {
        None => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let m3046 = f614({
                let (h3045, _) = v2;
                h3045
            }, v1.clone())?;
            match m3046 {
                Ok(v5) => {
                    let m3047 = f622(v3, v1)?;
                    match m3047 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt2>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h3048) = v2;
                                    h3048
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

pub fn f623() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f624(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f546({
        let (_, h3049) = {
            let (h3050, _) = f666(Fn22::F667, {
                let a3052 = {
                    let a3051 = v0.clone();
                    length_list(a3051)
                };
                let a3053 = 1u64;
                nat_add(a3052, a3053)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3050
        };
        h3049
    }))
}

pub fn f625(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3098 = {
            let a3074 = {
                let a3062 = {
                    let a3056 = {
                        let a3054 = v1;
                        let a3055 = 128u8;
                        fixed_u8::bit_and(a3054, a3055)
                    };
                    let a3057 = 128u8;
                    if equal(a3056, a3057) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3063 = {
                    let a3060 = {
                        let a3058 = v1;
                        let a3059 = 64u8;
                        fixed_u8::bit_and(a3058, a3059)
                    };
                    let a3061 = 64u8;
                    if equal(a3060, a3061) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3062, a3063)?
            };
            let a3075 = {
                let a3072 = {
                    let a3066 = {
                        let a3064 = v1;
                        let a3065 = 32u8;
                        fixed_u8::bit_and(a3064, a3065)
                    };
                    let a3067 = 32u8;
                    if equal(a3066, a3067) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3073 = {
                    let a3070 = {
                        let a3068 = v1;
                        let a3069 = 16u8;
                        fixed_u8::bit_and(a3068, a3069)
                    };
                    let a3071 = 16u8;
                    if equal(a3070, a3071) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3072, a3073)?
            };
            nat_add(a3074, a3075)?
        };
        let a3099 = {
            let a3096 = {
                let a3084 = {
                    let a3078 = {
                        let a3076 = v1;
                        let a3077 = 8u8;
                        fixed_u8::bit_and(a3076, a3077)
                    };
                    let a3079 = 8u8;
                    if equal(a3078, a3079) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3085 = {
                    let a3082 = {
                        let a3080 = v1;
                        let a3081 = 4u8;
                        fixed_u8::bit_and(a3080, a3081)
                    };
                    let a3083 = 4u8;
                    if equal(a3082, a3083) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3084, a3085)?
            };
            let a3097 = {
                let a3094 = {
                    let a3088 = {
                        let a3086 = v1;
                        let a3087 = 2u8;
                        fixed_u8::bit_and(a3086, a3087)
                    };
                    let a3089 = 2u8;
                    if equal(a3088, a3089) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3095 = {
                    let a3092 = {
                        let a3090 = v1;
                        let a3091 = 1u8;
                        fixed_u8::bit_and(a3090, a3091)
                    };
                    let a3093 = 1u8;
                    if equal(a3092, a3093) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3094, a3095)?
            };
            nat_add(a3096, a3097)?
        };
        nat_add(a3098, a3099)?
    };
    let a3100 = 48u64;
    let a3101 = v2;
    if if nat_le(a3100, a3101) {
        let a3102 = v2;
        let a3103 = 57u64;
        nat_le(a3102, a3103)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3104 = 97u64;
        let a3105 = v2;
        if nat_le(a3104, a3105) {
            let a3106 = v2;
            let a3107 = 102u64;
            Ok(nat_le(a3106, a3107))
        } else {
            Ok(false)
        }
    }
}

pub fn f626() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f627(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3152 = {
            let a3128 = {
                let a3116 = {
                    let a3110 = {
                        let a3108 = v1;
                        let a3109 = 128u8;
                        fixed_u8::bit_and(a3108, a3109)
                    };
                    let a3111 = 128u8;
                    if equal(a3110, a3111) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3117 = {
                    let a3114 = {
                        let a3112 = v1;
                        let a3113 = 64u8;
                        fixed_u8::bit_and(a3112, a3113)
                    };
                    let a3115 = 64u8;
                    if equal(a3114, a3115) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3116, a3117)?
            };
            let a3129 = {
                let a3126 = {
                    let a3120 = {
                        let a3118 = v1;
                        let a3119 = 32u8;
                        fixed_u8::bit_and(a3118, a3119)
                    };
                    let a3121 = 32u8;
                    if equal(a3120, a3121) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3127 = {
                    let a3124 = {
                        let a3122 = v1;
                        let a3123 = 16u8;
                        fixed_u8::bit_and(a3122, a3123)
                    };
                    let a3125 = 16u8;
                    if equal(a3124, a3125) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3126, a3127)?
            };
            nat_add(a3128, a3129)?
        };
        let a3153 = {
            let a3150 = {
                let a3138 = {
                    let a3132 = {
                        let a3130 = v1;
                        let a3131 = 8u8;
                        fixed_u8::bit_and(a3130, a3131)
                    };
                    let a3133 = 8u8;
                    if equal(a3132, a3133) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3139 = {
                    let a3136 = {
                        let a3134 = v1;
                        let a3135 = 4u8;
                        fixed_u8::bit_and(a3134, a3135)
                    };
                    let a3137 = 4u8;
                    if equal(a3136, a3137) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3138, a3139)?
            };
            let a3151 = {
                let a3148 = {
                    let a3142 = {
                        let a3140 = v1;
                        let a3141 = 2u8;
                        fixed_u8::bit_and(a3140, a3141)
                    };
                    let a3143 = 2u8;
                    if equal(a3142, a3143) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3149 = {
                    let a3146 = {
                        let a3144 = v1;
                        let a3145 = 1u8;
                        fixed_u8::bit_and(a3144, a3145)
                    };
                    let a3147 = 1u8;
                    if equal(a3146, a3147) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3148, a3149)?
            };
            nat_add(a3150, a3151)?
        };
        nat_add(a3152, a3153)?
    };
    let a3154 = v2;
    let a3155 = 57u64;
    if nat_le(a3154, a3155) {
        let a3156 = v2;
        let a3157 = 48u64;
        Ok(nat_sub(a3156, a3157))
    } else {
        let a3158 = v2;
        let a3159 = 87u64;
        Ok(nat_sub(a3158, a3159))
    }
}

pub fn f628(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3160, _) = v0;
        h3160
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3161) = v0;
        h3161
    };
    let m3162 = v1;
    match m3162.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3163 = v4;
            match m3163.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f668(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f629(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f562(f46(v0.clone(), 8u64, {
            let a3165 = {
                let a3164 = v0;
                length_bytes(a3164)
            };
            let a3166 = 8u64;
            nat_sub(a3165, a3166)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f562(f46(v0.clone(), 14u64, {
                let a3168 = {
                    let a3167 = v0;
                    length_bytes(a3167)
                };
                let a3169 = 14u64;
                nat_sub(a3168, a3169)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f630(p3186: u64, p3187: u64, p3188: u64, p3189: Bytes) -> Bytes {
    let m3185 = (p3186, (p3187, (p3188, p3189)));
    {
        let mut m3185 = m3185;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3185;
                let m3170 = v0;
                if m3170 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3170 - 1;
                    if f118({
                        let a3171 = v1;
                        let a3172 = 0u64;
                        nat_eq(a3171, a3172)
                    }, {
                        let a3173 = v2;
                        let a3174 = 0u64;
                        nat_eq(a3173, a3174)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3175 = v1;
                            let a3176 = 16u64;
                            let a3177 = 0u64;
                            nat_quot(a3175, a3176, a3177)
                        }, ({
                            let a3178 = v2;
                            let a3179 = 1u64;
                            nat_sub(a3178, a3179)
                        }, {
                            let a3183 = f669({
                                let a3180 = v1;
                                let a3181 = 16u64;
                                let a3182 = 0u64;
                                nat_rem(a3180, a3181, a3182)
                            });
                            let a3184 = v3;
                            append_bytes(a3183, a3184)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f670(f671(f280(v0.clone()))?, v0, v1)
}

pub fn f632(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f633(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f634(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt28, Bytes>> {
    let m3190 = v1.clone();
    match m3190.uncons() {
        None => {
            Ok(f632(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3191 = f672(f522(f276(v2, f8(Str::lit("properties")))), v1.clone());
            match m3191.uncons() {
                None => {
                    let m3192 = f673(f671(f674(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3192 {
                        Ok(v5) => {
                            Ok(f633(f637(v0, f638(v5.clone())), f639(v5)))
                        }
                        Err(v4) => {
                            Ok(f632(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3193 = f675(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3193 {
                        Ok(v9) => {
                            Ok(f633(f637(v0, f638(v9.clone())), f639(v9)))
                        }
                        Err(v8) => {
                            Ok(f632(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f635(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3194 = v0;
    match m3194 {
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
        Adt2::C6(r3195) => {
            let v2 = (*r3195).clone();
            f635(v2, v1)
        }
        Adt2::C7(r3196) => {
            let v3 = (*r3196).clone();
            f635(v3, v1)
        }
        Adt2::C8 => {
            Ok(false)
        }
        Adt2::C9(r3197, r3198) => {
            let v4 = (*r3197).clone();
            let v5 = (*r3198).clone();
            if f635(v4, v1.clone())? {
                Ok(true)
            } else {
                f635(v5, v1)
            }
        }
        Adt2::C10(v6, r3199, r3200) => {
            let v7 = (*r3199).clone();
            let v8 = (*r3200).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f676(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f635(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f635(v8, v1)
                }
            }
        }
    }
}

pub fn f636(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f677(f671(f617(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f637(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f395(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f638(v0: Adt29) -> List<Bytes> {
    let m3201 = v0;
    match m3201 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f639(v0: Adt29) -> List<Adt25> {
    let m3202 = v0;
    match m3202 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f640(v0: Adt2) -> bool {
    let m3203 = f522(f276(v0.clone(), f8(Str::lit("properties"))));
    match m3203.uncons() {
        None => {
            let m3204 = f678(v0);
            match m3204 {
                None => {
                    false
                }
                Some(v3) => {
                    f679(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f641(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f642(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3205 = f680(v0.clone());
    match m3205 {
        None => {
            if f32(f279(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3206 = f276(v0, f8(Str::lit("items")));
                match m3206 {
                    None => {
                        Ok(f681(f8(Str::lit("List")), f683()))
                    }
                    Some(v3) => {
                        Ok(f681(f8(Str::lit("List")), f682(v3, {
                            let a3207 = v1;
                            let a3208 = f8(Str::lit("Item"));
                            append_bytes(a3207, a3208)
                        })?))
                    }
                }
            } else {
                f682(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f681(f8(Str::lit("Option")), f682(v2, v1)?))
        }
    }
}

pub fn f643(p3211: List<Adt2>) -> bool {
    let m3210 = p3211;
    {
        let mut m3210 = m3210;
        loop {
            let step = {
                let v0 = m3210;
                let m3209 = v0;
                match m3209.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f684(f559(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: Adt2, v1: Bytes) -> Adt2 {
    let m3212 = v0;
    match m3212 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r3213) => {
            let v7 = (*r3213).clone();
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
        Adt2::C6(r3214) => {
            let v15 = (*r3214).clone();
            Adt2::C6(std::rc::Rc::new(v15))
        }
        Adt2::C7(r3215) => {
            let v5 = (*r3215).clone();
            Adt2::C7(std::rc::Rc::new(f644(v5, v1)))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r3216, r3217) => {
            let v16 = (*r3216).clone();
            let v17 = (*r3217).clone();
            Adt2::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt2::C10(v2, r3218, r3219) => {
            let v3 = (*r3218).clone();
            let v4 = (*r3219).clone();
            if f32(v2.clone(), v1.clone()) {
                f644(v4, v1)
            } else {
                Adt2::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f644(v4, v1)))
            }
        }
    }
}

pub fn f645(v0: Adt2, v1: Bytes) -> bool {
    let m3220 = f276(v0, v1);
    match m3220 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f646(v0: List<Adt2>) -> bool {
    let m3221 = v0.clone();
    match m3221.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3222 = f672(f522(f276(v1, f8(Str::lit("properties")))), v0.clone());
            match m3222.uncons() {
                None => {
                    f686(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f685(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f647(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m3223 = v0;
    match m3223 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r3224) => {
            let v8 = (*r3224).clone();
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
        Adt2::C6(r3225) => {
            let v16 = (*r3225).clone();
            Adt2::C6(std::rc::Rc::new(v16))
        }
        Adt2::C7(r3226) => {
            let v3 = (*r3226).clone();
            Adt2::C7(std::rc::Rc::new(Adt2::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r3227, r3228) => {
            let v17 = (*r3227).clone();
            let v18 = (*r3228).clone();
            Adt2::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt2::C10(v4, r3229, r3230) => {
            let v5 = (*r3229).clone();
            let v6 = (*r3230).clone();
            Adt2::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f648(p3233: List<Adt2>) -> bool {
    let m3232 = p3233;
    {
        let mut m3232 = m3232;
        loop {
            let step = {
                let v0 = m3232;
                let m3231 = v0;
                match m3231.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f687(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f649(v0: u64) -> u32 {
    let a3234 = v0;
    let a3235 = 32u64;
    if nat_lt(a3234, a3235) {
        let a3236 = v0;
        let a3237 = 16u64;
        if nat_lt(a3236, a3237) {
            let a3238 = v0;
            let a3239 = 8u64;
            if nat_lt(a3238, a3239) {
                let a3240 = v0;
                let a3241 = 4u64;
                if nat_lt(a3240, a3241) {
                    let a3242 = v0;
                    let a3243 = 2u64;
                    if nat_lt(a3242, a3243) {
                        let a3244 = v0;
                        let a3245 = 1u64;
                        if nat_lt(a3244, a3245) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3246 = v0;
                        let a3247 = 3u64;
                        if nat_lt(a3246, a3247) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3248 = v0;
                    let a3249 = 6u64;
                    if nat_lt(a3248, a3249) {
                        let a3250 = v0;
                        let a3251 = 5u64;
                        if nat_lt(a3250, a3251) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3252 = v0;
                        let a3253 = 7u64;
                        if nat_lt(a3252, a3253) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3254 = v0;
                let a3255 = 12u64;
                if nat_lt(a3254, a3255) {
                    let a3256 = v0;
                    let a3257 = 10u64;
                    if nat_lt(a3256, a3257) {
                        let a3258 = v0;
                        let a3259 = 9u64;
                        if nat_lt(a3258, a3259) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3260 = v0;
                        let a3261 = 11u64;
                        if nat_lt(a3260, a3261) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3262 = v0;
                    let a3263 = 14u64;
                    if nat_lt(a3262, a3263) {
                        let a3264 = v0;
                        let a3265 = 13u64;
                        if nat_lt(a3264, a3265) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3266 = v0;
                        let a3267 = 15u64;
                        if nat_lt(a3266, a3267) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3268 = v0;
            let a3269 = 24u64;
            if nat_lt(a3268, a3269) {
                let a3270 = v0;
                let a3271 = 20u64;
                if nat_lt(a3270, a3271) {
                    let a3272 = v0;
                    let a3273 = 18u64;
                    if nat_lt(a3272, a3273) {
                        let a3274 = v0;
                        let a3275 = 17u64;
                        if nat_lt(a3274, a3275) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3276 = v0;
                        let a3277 = 19u64;
                        if nat_lt(a3276, a3277) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3278 = v0;
                    let a3279 = 22u64;
                    if nat_lt(a3278, a3279) {
                        let a3280 = v0;
                        let a3281 = 21u64;
                        if nat_lt(a3280, a3281) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3282 = v0;
                        let a3283 = 23u64;
                        if nat_lt(a3282, a3283) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3284 = v0;
                let a3285 = 28u64;
                if nat_lt(a3284, a3285) {
                    let a3286 = v0;
                    let a3287 = 26u64;
                    if nat_lt(a3286, a3287) {
                        let a3288 = v0;
                        let a3289 = 25u64;
                        if nat_lt(a3288, a3289) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3290 = v0;
                        let a3291 = 27u64;
                        if nat_lt(a3290, a3291) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3292 = v0;
                    let a3293 = 30u64;
                    if nat_lt(a3292, a3293) {
                        let a3294 = v0;
                        let a3295 = 29u64;
                        if nat_lt(a3294, a3295) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3296 = v0;
                        let a3297 = 31u64;
                        if nat_lt(a3296, a3297) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3298 = v0;
        let a3299 = 48u64;
        if nat_lt(a3298, a3299) {
            let a3300 = v0;
            let a3301 = 40u64;
            if nat_lt(a3300, a3301) {
                let a3302 = v0;
                let a3303 = 36u64;
                if nat_lt(a3302, a3303) {
                    let a3304 = v0;
                    let a3305 = 34u64;
                    if nat_lt(a3304, a3305) {
                        let a3306 = v0;
                        let a3307 = 33u64;
                        if nat_lt(a3306, a3307) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3308 = v0;
                        let a3309 = 35u64;
                        if nat_lt(a3308, a3309) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3310 = v0;
                    let a3311 = 38u64;
                    if nat_lt(a3310, a3311) {
                        let a3312 = v0;
                        let a3313 = 37u64;
                        if nat_lt(a3312, a3313) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3314 = v0;
                        let a3315 = 39u64;
                        if nat_lt(a3314, a3315) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3316 = v0;
                let a3317 = 44u64;
                if nat_lt(a3316, a3317) {
                    let a3318 = v0;
                    let a3319 = 42u64;
                    if nat_lt(a3318, a3319) {
                        let a3320 = v0;
                        let a3321 = 41u64;
                        if nat_lt(a3320, a3321) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3322 = v0;
                        let a3323 = 43u64;
                        if nat_lt(a3322, a3323) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3324 = v0;
                    let a3325 = 46u64;
                    if nat_lt(a3324, a3325) {
                        let a3326 = v0;
                        let a3327 = 45u64;
                        if nat_lt(a3326, a3327) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3328 = v0;
                        let a3329 = 47u64;
                        if nat_lt(a3328, a3329) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3330 = v0;
            let a3331 = 56u64;
            if nat_lt(a3330, a3331) {
                let a3332 = v0;
                let a3333 = 52u64;
                if nat_lt(a3332, a3333) {
                    let a3334 = v0;
                    let a3335 = 50u64;
                    if nat_lt(a3334, a3335) {
                        let a3336 = v0;
                        let a3337 = 49u64;
                        if nat_lt(a3336, a3337) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3338 = v0;
                        let a3339 = 51u64;
                        if nat_lt(a3338, a3339) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3340 = v0;
                    let a3341 = 54u64;
                    if nat_lt(a3340, a3341) {
                        let a3342 = v0;
                        let a3343 = 53u64;
                        if nat_lt(a3342, a3343) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3344 = v0;
                        let a3345 = 55u64;
                        if nat_lt(a3344, a3345) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3346 = v0;
                let a3347 = 60u64;
                if nat_lt(a3346, a3347) {
                    let a3348 = v0;
                    let a3349 = 58u64;
                    if nat_lt(a3348, a3349) {
                        let a3350 = v0;
                        let a3351 = 57u64;
                        if nat_lt(a3350, a3351) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3352 = v0;
                        let a3353 = 59u64;
                        if nat_lt(a3352, a3353) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3354 = v0;
                    let a3355 = 62u64;
                    if nat_lt(a3354, a3355) {
                        let a3356 = v0;
                        let a3357 = 61u64;
                        if nat_lt(a3356, a3357) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3358 = v0;
                        let a3359 = 63u64;
                        if nat_lt(a3358, a3359) {
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

pub fn f650(v0: u32) -> u32 {
    let a3362 = {
        let a3360 = f688(v0, 6u32, 26u32);
        let a3361 = f688(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3360, a3361)
    };
    let a3363 = f688(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3362, a3363)
}

pub fn f651(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3369 = {
        let a3364 = v0;
        let a3365 = v1;
        fixed_u32::bit_and(a3364, a3365)
    };
    let a3370 = {
        let a3367 = {
            let a3366 = v0;
            fixed_u32::bit_not(a3366)
        };
        let a3368 = v2;
        fixed_u32::bit_and(a3367, a3368)
    };
    fixed_u32::bit_xor(a3369, a3370)
}

pub fn f652(v0: u32) -> u32 {
    let a3373 = {
        let a3371 = f688(v0, 2u32, 30u32);
        let a3372 = f688(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3371, a3372)
    };
    let a3374 = f688(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3373, a3374)
}

pub fn f653(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3383 = {
        let a3379 = {
            let a3375 = v0;
            let a3376 = v1;
            fixed_u32::bit_and(a3375, a3376)
        };
        let a3380 = {
            let a3377 = v0;
            let a3378 = v2;
            fixed_u32::bit_and(a3377, a3378)
        };
        fixed_u32::bit_xor(a3379, a3380)
    };
    let a3384 = {
        let a3381 = v1;
        let a3382 = v2;
        fixed_u32::bit_and(a3381, a3382)
    };
    fixed_u32::bit_xor(a3383, a3384)
}

pub fn f654(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3385, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3385
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3386, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3386
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3387, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3387
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3388, _, _, _, _, _, _, _, _, _, _, _) => {
            h3388
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3389, _, _, _, _, _, _, _, _, _, _) => {
            h3389
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3390, _, _, _, _, _, _, _, _, _) => {
            h3390
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3391, _, _, _, _, _, _, _, _) => {
            h3391
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3392, _, _, _, _, _, _, _) => {
            h3392
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3393, _, _, _, _, _, _) => {
            h3393
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3394, _, _, _, _, _) => {
            h3394
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3395, _, _, _, _) => {
            h3395
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3396, _, _, _) => {
            h3396
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3397, _, _) => {
            h3397
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3398, _) => {
            h3398
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3399) => {
            h3399
        }
    }, f571(f689(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3400, _) => {
            h3400
        }
    }), f571(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3401, _, _, _, _, _, _) => {
            h3401
        }
    }, f571(f690(match v0.clone() {
        Adt26::C0(_, h3402, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3402
        }
    }), match v0 {
        Adt26::C0(h3403, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3403
        }
    }))))
}

pub fn f655(v0: Fn21, p3407: u64, p3408: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3406 = (p3407, p3408);
    {
        let mut m3406 = m3406;
        loop {
            let step = {
                let (v1, v2) = m3406;
                let m3404 = v1;
                if m3404 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3404 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3405: Fn21 = v0.clone();
                        c3405.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f656(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3409, _) = v0;
        h3409
    };
    let v2: u32 = {
        let (h3410, _) = {
            let (_, h3411) = v0;
            h3411
        };
        h3410
    };
    let v3: u32 = {
        let (_, h3412) = {
            let (_, h3413) = v0;
            h3413
        };
        h3412
    };
    ({
        let a3414 = v1;
        let a3415 = 2u64;
        let a3416 = 0u64;
        nat_quot(a3414, a3415, a3416)
    }, (f657(v2, 1u32), {
        let a3420 = {
            let a3417 = v1;
            let a3418 = 2u64;
            let a3419 = 0u64;
            nat_rem(a3417, a3418, a3419)
        };
        let a3421 = 1u64;
        if nat_eq(a3420, a3421) {
            let a3422 = v3;
            let a3423 = v2;
            fixed_u32::bit_or(a3422, a3423)
        } else {
            v3
        }
    }))
}

pub fn f657(v0: u32, v1: u32) -> u32 {
    let m3426 = {
        let a3424 = v0;
        let a3425 = v1;
        fixed_u32::shift_left(a3424, a3425)
    };
    match m3426 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f658(v0: Bytes, v1: Adt2) -> bool {
    let v2: Bytes = f607(v1);
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

pub fn f659(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m3427 = v0;
    if m3427 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m3427 - 1;
        let a3429 = {
            let a3428 = v1.clone();
            length_bytes(a3428)
        };
        let a3430 = v2;
        if nat_le(a3429, a3430) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m3431 = f691(v1.clone(), v2)?;
            match m3431 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m3435 = f659(v3, v1, {
                        let a3433 = v2;
                        let a3434 = {
                            let (_, h3432) = v4;
                            h3432
                        };
                        nat_add(a3433, a3434)?
                    })?;
                    match m3435 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h3436, _) = v4;
                                h3436
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f660(v0: Bytes) -> R<Result<((bool, bool), List<Adt30>), Bytes>> {
    let v1: u64 = {
        let a3437 = v0.clone();
        length_bytes(a3437)
    };
    let v2: bool = {
        let a3438 = 0u64;
        let a3439 = v1;
        if nat_lt(a3438, a3439) {
            let a3440 = f116(v0.clone(), 0u64)?;
            let a3441 = 94u64;
            nat_eq(a3440, a3441)
        } else {
            false
        }
    };
    let v3: bool = {
        let a3442 = 0u64;
        let a3443 = v1;
        if nat_lt(a3442, a3443) {
            f118({
                let a3446 = f116(v0.clone(), {
                    let a3444 = v1;
                    let a3445 = 1u64;
                    nat_sub(a3444, a3445)
                })?;
                let a3447 = 36u64;
                nat_eq(a3446, a3447)
            }, f692({
                let a3450 = f116(v0.clone(), {
                    let a3448 = v1;
                    let a3449 = 2u64;
                    nat_sub(a3448, a3449)
                })?;
                let a3451 = 92u64;
                nat_eq(a3450, a3451)
            }))
        } else {
            false
        }
    };
    let m3456 = f693({
        let a3452 = v1;
        let a3453 = 1u64;
        nat_add(a3452, a3453)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a3454 = v1;
        let a3455 = 1u64;
        nat_sub(a3454, a3455)
    } else {
        v1
    })?;
    match m3456 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt30>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt30>), Bytes>(v4))
        }
    }
}

pub fn f661(v0: u64, v1: List<Adt30>, v2: List<u64>, v3: bool) -> R<bool> {
    let m3457 = v0;
    if m3457 == 0 {
        Ok(false)
    } else {
        let v4 = m3457 - 1;
        let m3458 = v1.clone();
        match m3458.uncons() {
            None => {
                if v3 {
                    let a3459 = f612(v2)?;
                    let a3460 = 0u64;
                    Ok(nat_eq(a3459, a3460))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m3461 = v5.clone();
                match m3461 {
                    Adt30::C0(v7, v8, v9) => {
                        let a3462 = v9;
                        let a3463 = 1u64;
                        if nat_eq(a3462, a3463) {
                            if f661(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m3464 = v2;
                                match m3464.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f694(v5, v10) {
                                            f661(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a3465 = v9;
                            let a3466 = 2u64;
                            if nat_eq(a3465, a3466) {
                                let m3467 = v2;
                                match m3467.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f694(v5, v12) {
                                            f661(v4, List::cons(Adt30::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a3468 = v9;
                                let a3469 = 3u64;
                                if nat_eq(a3468, a3469) {
                                    if f661(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m3470 = v2;
                                        match m3470.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f694(v5, v14) {
                                                    f661(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m3471 = v2;
                                    match m3471.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f694(v5, v16) {
                                                f661(v4, v6, v17, v3)
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

pub fn f662(v0: List<Adt30>) -> R<u64> {
    let m3472 = v0;
    match m3472.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3473 = f662(v2)?;
            let a3474 = 1u64;
            nat_add(a3473, a3474)
        }
    }
}

pub fn f663(v0: List<Adt30>, v1: List<u64>, v2: bool) -> R<bool> {
    if f661({
        let a3477 = {
            let a3475 = f662(v0.clone())?;
            let a3476 = f612(v1.clone())?;
            nat_add(a3475, a3476)?
        };
        let a3478 = 2u64;
        nat_add(a3477, a3478)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f695(v0, v1, v2)
    }
}

pub fn f664(v0: u64) -> R<u64> {
    let m3479 = v0;
    if m3479 == 0 {
        Ok(1u64)
    } else {
        let v1 = m3479 - 1;
        let a3480 = f664(v1)?;
        let a3481 = 10u64;
        nat_mul(a3480, a3481)
    }
}

pub fn f665(p3495: u64, v1: Bytes, p3496: u64, p3497: Bytes) -> R<Bytes> {
    let m3494 = (p3495, (p3496, p3497));
    {
        let mut m3494 = m3494;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3494;
                let m3482 = v0;
                if m3482 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3482 - 1;
                    let m3485 = {
                        let a3483 = v1.clone();
                        let a3484 = v2;
                        index_bytes(a3483, a3484)
                    };
                    match m3485 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3486 = v2;
                                let a3487 = 1u64;
                                nat_add(a3486, a3487)?
                            }, {
                                let a3492 = v3;
                                let a3493 = {
                                    let a3488 = v6;
                                    let a3489 = 126u64;
                                    if nat_eq(a3488, a3489) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a3490 = v6;
                                        let a3491 = 47u64;
                                        if nat_eq(a3490, a3491) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f46(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a3492, a3493)
                            })))
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

pub fn f666(v0: Fn22, p3502: u64, p3503: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3501 = (p3502, p3503);
    {
        let mut m3501 = m3501;
        loop {
            let step = {
                let (v1, v2) = m3501;
                let m3498 = v1;
                if m3498 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3498 - 1;
                    let m3500 = {
                        let c3499: Fn22 = v0.clone();
                        c3499.apply(v2.clone())?
                    };
                    match m3500 {
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
                Ok(next) => { m3501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f667(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f696(v0)
}

pub fn f668(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3508 = {
            let a3506 = {
                let a3504 = v0.clone();
                length_list(a3504)
            };
            let a3507 = {
                let a3505 = v1.clone();
                length_list(a3505)
            };
            nat_add(a3506, a3507)?
        };
        let a3509 = 1u64;
        nat_add(a3508, a3509)?
    };
    Ok(f449({
        let (_, h3510) = {
            let (_, h3511) = {
                let (h3512, _) = f697(Fn23::F698, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3512
            };
            h3511
        };
        h3510
    }))
}

pub fn f669(v0: u64) -> Bytes {
    f46({
        let a3513 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3513)
    }, v0, 1u64)
}

pub fn f670(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3514 = v1;
    match m3514.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3515 = f699(v0, v3.clone())?;
            match m3515 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f538(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3516 = f670(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3516 {
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

pub fn f671(p3520: List<Bytes>) -> R<bool> {
    let m3519 = p3520;
    {
        let mut m3519 = m3519;
        loop {
            let step = {
                let v0 = m3519;
                let m3517 = v0;
                match m3517.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3518 = f700(v1)?;
                        match m3518 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f701(false, v2.clone())?) {
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
                Ok(next) => { m3519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f672(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3521 = v0;
    match m3521.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f702(v1.clone(), {
                let (h3522, _) = v2.clone();
                h3522
            }) {
                List::cons({
                    let (h3523, _) = v2;
                    h3523
                }, f672(v3, v1))
            } else {
                f672(v3, v1)
            }
        }
    }
}

pub fn f673(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3524 = v2;
    match m3524.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3525 = f703(v4.clone())?;
            match m3525 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3526 = f704(v0, v6.clone())?;
                    match m3526 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3529 = f705(v4, {
                                    let a3527 = v1.clone();
                                    let a3528 = f379(v6)?;
                                    append_bytes(a3527, a3528)
                                })?;
                                match m3529 {
                                    Ok(v9) => {
                                        let m3530 = f673(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3530 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f567(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f638(v11.clone())), f526(f566(v9), f639(v11)))))
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

pub fn f674(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3531 = v0;
    match m3531.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3532 = f703(v1)?;
            match m3532 {
                None => {
                    f674(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f674(v2)?))
                }
            }
        }
    }
}

pub fn f675(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f706(f671(f707(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f676(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3533 = f385(Some(v0));
    match m3533 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3534 = f629(v2)?;
            match m3534 {
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

pub fn f677(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m3535 = v2;
    match m3535.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3537 = f708(v0, {
                let (h3536, _) = v5.clone();
                h3536
            })?;
            match m3537 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3538, _) = v5;
                        h3538
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f709(v0, f617(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3543 = f642({
                            let (_, h3539) = v5.clone();
                            h3539
                        }, {
                            let a3541 = v1.clone();
                            let a3542 = f379({
                                let (h3540, _) = v5.clone();
                                h3540
                            })?;
                            append_bytes(a3541, a3542)
                        })?;
                        match m3543 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3544, _) = v5;
                                    h3544
                                }, v3.clone()) {
                                    f567(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f567(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3545 = f677(v0, v1, v6, v3, v4)?;
                                match m3545 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f638(v12.clone())), f526(f566(v9), f639(v12)))))
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

pub fn f678(v0: Adt2) -> Option<Adt2> {
    let m3546 = f276(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3546 {
        None => {
            f710(v0)
        }
        Some(v1) => {
            if f679(v1.clone()) {
                let m3547 = f710(v0);
                match m3547 {
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

pub fn f679(v0: Adt2) -> bool {
    let m3548 = v0;
    match m3548 {
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

pub fn f680(v0: Adt2) -> Option<Adt2> {
    let m3549 = f281(f276(v0, f8(Str::lit("oneOf"))));
    match m3549.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3550 = v2;
            match m3550.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3551 = v4;
                    match m3551.uncons() {
                        None => {
                            if f711(v3.clone()) {
                                Some(v1)
                            } else {
                                if f711(v1) {
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

pub fn f681(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3552 = v1;
    match m3552 {
        Ok(v3) => {
            f633(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f567(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f566(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f682(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3553 = f680(v0.clone());
    match m3553 {
        None => {
            if f32(f279(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3554 = f276(v0, f8(Str::lit("items")));
                match m3554 {
                    None => {
                        Ok(f681(f8(Str::lit("List")), f683()))
                    }
                    Some(v3) => {
                        Ok(f681(f8(Str::lit("List")), f705(v3, {
                            let a3555 = v1;
                            let a3556 = f8(Str::lit("Item"));
                            append_bytes(a3555, a3556)
                        })?))
                    }
                }
            } else {
                f705(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f681(f8(Str::lit("Option")), f705(v2, v1)?))
        }
    }
}

pub fn f683() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f563(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f684(p3560: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3559 = p3560;
    {
        let mut m3559 = m3559;
        loop {
            let step = {
                let v0 = m3559;
                let m3557 = v0;
                match m3557.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3558, _) = v2;
                            h3558
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f685(p3564: List<Adt2>, v1: Bytes, p3565: List<Bytes>) -> bool {
    let m3563 = (p3564, p3565);
    {
        let mut m3563 = m3563;
        loop {
            let step = {
                let (v0, v2) = m3563;
                let m3561 = v0;
                match m3561.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3562 = f712(v3, v1.clone());
                        match m3562 {
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
                Ok(next) => { m3563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(p3568: List<Adt2>, p3569: List<Bytes>) -> bool {
    let m3567 = (p3568, p3569);
    {
        let mut m3567 = m3567;
        loop {
            let step = {
                let (v0, v1) = m3567;
                let m3566 = v0;
                match m3566.uncons() {
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
                Ok(next) => { m3567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f687(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3570 = f276(v0.clone(), f8(Str::lit("anyOf")));
        match m3570 {
            None => {
                Adt2::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f684(f559(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f643(f386(v2))
    } else {
        false
    }
}

pub fn f688(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3571 = f532(v0, v1);
    let a3572 = f657(v0, v2);
    fixed_u32::bit_or(a3571, a3572)
}

pub fn f689(v0: u32) -> u32 {
    let a3575 = {
        let a3573 = f688(v0, 17u32, 15u32);
        let a3574 = f688(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3573, a3574)
    };
    let a3576 = f532(v0, 10u32);
    fixed_u32::bit_xor(a3575, a3576)
}

pub fn f690(v0: u32) -> u32 {
    let a3579 = {
        let a3577 = f688(v0, 7u32, 25u32);
        let a3578 = f688(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3577, a3578)
    };
    let a3580 = f532(v0, 3u32);
    fixed_u32::bit_xor(a3579, a3580)
}

pub fn f691(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3583 = {
        let a3581 = v0.clone();
        let a3582 = v1;
        index_bytes(a3581, a3582)
    };
    match m3583 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3584 = v3;
            let a3585 = 128u64;
            if nat_lt(a3584, a3585) {
                Ok(Some((v3, 1u64)))
            } else {
                if f118({
                    let a3586 = 194u64;
                    let a3587 = v3;
                    nat_le(a3586, a3587)
                }, {
                    let a3588 = v3;
                    let a3589 = 223u64;
                    nat_le(a3588, a3589)
                }) {
                    let m3592 = f713(v0, {
                        let a3590 = v1;
                        let a3591 = 1u64;
                        nat_add(a3590, a3591)?
                    })?;
                    match m3592 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f714({
                                let a3597 = {
                                    let a3595 = {
                                        let a3593 = v3;
                                        let a3594 = 192u64;
                                        nat_sub(a3593, a3594)
                                    };
                                    let a3596 = 64u64;
                                    nat_mul(a3595, a3596)?
                                };
                                let a3598 = v4;
                                nat_add(a3597, a3598)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f118({
                        let a3599 = 224u64;
                        let a3600 = v3;
                        nat_le(a3599, a3600)
                    }, {
                        let a3601 = v3;
                        let a3602 = 239u64;
                        nat_le(a3601, a3602)
                    }) {
                        let m3605 = f713(v0.clone(), {
                            let a3603 = v1;
                            let a3604 = 1u64;
                            nat_add(a3603, a3604)?
                        })?;
                        match m3605 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3608 = f713(v0, {
                                    let a3606 = v1;
                                    let a3607 = 2u64;
                                    nat_add(a3606, a3607)?
                                })?;
                                match m3608 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f714({
                                            let a3617 = {
                                                let a3611 = {
                                                    let a3609 = v3;
                                                    let a3610 = 224u64;
                                                    nat_sub(a3609, a3610)
                                                };
                                                let a3612 = 4096u64;
                                                nat_mul(a3611, a3612)?
                                            };
                                            let a3618 = {
                                                let a3615 = {
                                                    let a3613 = v5;
                                                    let a3614 = 64u64;
                                                    nat_mul(a3613, a3614)?
                                                };
                                                let a3616 = v6;
                                                nat_add(a3615, a3616)?
                                            };
                                            nat_add(a3617, a3618)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f118({
                            let a3619 = 240u64;
                            let a3620 = v3;
                            nat_le(a3619, a3620)
                        }, {
                            let a3621 = v3;
                            let a3622 = 244u64;
                            nat_le(a3621, a3622)
                        }) {
                            let m3625 = f713(v0.clone(), {
                                let a3623 = v1;
                                let a3624 = 1u64;
                                nat_add(a3623, a3624)?
                            })?;
                            match m3625 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3628 = f713(v0.clone(), {
                                        let a3626 = v1;
                                        let a3627 = 2u64;
                                        nat_add(a3626, a3627)?
                                    })?;
                                    match m3628 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3631 = f713(v0, {
                                                let a3629 = v1;
                                                let a3630 = 3u64;
                                                nat_add(a3629, a3630)?
                                            })?;
                                            match m3631 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f714({
                                                        let a3644 = {
                                                            let a3634 = {
                                                                let a3632 = v3;
                                                                let a3633 = 240u64;
                                                                nat_sub(a3632, a3633)
                                                            };
                                                            let a3635 = 262144u64;
                                                            nat_mul(a3634, a3635)?
                                                        };
                                                        let a3645 = {
                                                            let a3642 = {
                                                                let a3636 = v7;
                                                                let a3637 = 4096u64;
                                                                nat_mul(a3636, a3637)?
                                                            };
                                                            let a3643 = {
                                                                let a3640 = {
                                                                    let a3638 = v8;
                                                                    let a3639 = 64u64;
                                                                    nat_mul(a3638, a3639)?
                                                                };
                                                                let a3641 = v9;
                                                                nat_add(a3640, a3641)?
                                                            };
                                                            nat_add(a3642, a3643)?
                                                        };
                                                        nat_add(a3644, a3645)?
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

pub fn f692(v0: bool) -> bool {
    !v0
}

pub fn f693(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt30>, Bytes>> {
    let m3646 = v0;
    if m3646 == 0 {
        Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
    } else {
        let v4 = m3646 - 1;
        let a3647 = v3;
        let a3648 = v2;
        if nat_le(a3647, a3648) {
            Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
        } else {
            let v5: u64 = f116(v1.clone(), v2)?;
            let m3697 = {
                let a3649 = v5;
                let a3650 = 91u64;
                if nat_eq(a3649, a3650) {
                    let a3653 = f116(v1.clone(), {
                        let a3651 = v2;
                        let a3652 = 1u64;
                        nat_add(a3651, a3652)?
                    })?;
                    let a3654 = 94u64;
                    if nat_eq(a3653, a3654) {
                        let m3658 = f715({
                            let a3655 = v1.clone();
                            length_bytes(a3655)
                        }, v1.clone(), {
                            let a3656 = v2;
                            let a3657 = 2u64;
                            nat_add(a3656, a3657)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3658 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3659, _) = v7;
                                    h3659
                                }), {
                                    let (_, h3660) = v7;
                                    h3660
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3664 = f715({
                            let a3661 = v1.clone();
                            length_bytes(a3661)
                        }, v1.clone(), {
                            let a3662 = v2;
                            let a3663 = 1u64;
                            nat_add(a3662, a3663)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3664 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3665, _) = v9;
                                    h3665
                                }), {
                                    let (_, h3666) = v9;
                                    h3666
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3667 = v5;
                    let a3668 = 46u64;
                    if nat_eq(a3667, a3668) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3669 = v2;
                            let a3670 = 1u64;
                            nat_add(a3669, a3670)?
                        }))
                    } else {
                        let a3671 = v5;
                        let a3672 = 40u64;
                        if if if if if if nat_eq(a3671, a3672) {
                            true
                        } else {
                            let a3673 = v5;
                            let a3674 = 41u64;
                            nat_eq(a3673, a3674)
                        } {
                            true
                        } else {
                            let a3675 = v5;
                            let a3676 = 124u64;
                            if nat_eq(a3675, a3676) {
                                true
                            } else {
                                let a3677 = v5;
                                let a3678 = 123u64;
                                nat_eq(a3677, a3678)
                            }
                        } {
                            true
                        } else {
                            let a3679 = v5;
                            let a3680 = 125u64;
                            if if nat_eq(a3679, a3680) {
                                true
                            } else {
                                let a3681 = v5;
                                let a3682 = 92u64;
                                nat_eq(a3681, a3682)
                            } {
                                true
                            } else {
                                let a3683 = v5;
                                let a3684 = 94u64;
                                if nat_eq(a3683, a3684) {
                                    true
                                } else {
                                    let a3685 = v5;
                                    let a3686 = 36u64;
                                    nat_eq(a3685, a3686)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3687 = v5;
                            let a3688 = 42u64;
                            if if nat_eq(a3687, a3688) {
                                true
                            } else {
                                let a3689 = v5;
                                let a3690 = 43u64;
                                nat_eq(a3689, a3690)
                            } {
                                true
                            } else {
                                let a3691 = v5;
                                let a3692 = 63u64;
                                nat_eq(a3691, a3692)
                            }
                        } {
                            true
                        } else {
                            let a3693 = 127u64;
                            let a3694 = v5;
                            nat_lt(a3693, a3694)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f46(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3695 = v2;
                                let a3696 = 1u64;
                                nat_add(a3695, a3696)?
                            }))
                        }
                    }
                }
            };
            match m3697 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3698) = v11.clone();
                        h3698
                    };
                    let v13: u64 = {
                        let a3699 = v12;
                        let a3700 = v3;
                        if nat_lt(a3699, a3700) {
                            f716(f116(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3701 = v12;
                    let a3702 = v3;
                    if if nat_lt(a3701, a3702) {
                        let a3703 = f116(v1.clone(), v12)?;
                        let a3704 = 123u64;
                        nat_eq(a3703, a3704)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt30>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3709 = f693(v4, v1, {
                            let a3705 = 0u64;
                            let a3706 = v13;
                            if nat_lt(a3705, a3706) {
                                let a3707 = v12;
                                let a3708 = 1u64;
                                nat_add(a3707, a3708)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3709 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt30>, Bytes>(List::cons(Adt30::C0({
                                    let (h3710, _) = {
                                        let (h3711, _) = v11.clone();
                                        h3711
                                    };
                                    h3710
                                }, {
                                    let (_, h3712) = {
                                        let (h3713, _) = v11;
                                        h3713
                                    };
                                    h3712
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

pub fn f694(v0: Adt30, v1: u64) -> bool {
    let m3714 = v0;
    match m3714 {
        Adt30::C0(v2, v3, _) => {
            if v2 {
                f692(f717(v3, v1))
            } else {
                f717(v3, v1)
            }
        }
    }
}

pub fn f695(v0: List<Adt30>, p3721: List<u64>, v2: bool) -> R<bool> {
    let m3720 = p3721;
    {
        let mut m3720 = m3720;
        loop {
            let step = {
                let v1 = m3720;
                let m3715 = v1;
                match m3715.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f661({
                            let a3718 = {
                                let a3716 = f662(v0.clone())?;
                                let a3717 = f612(v4.clone())?;
                                nat_add(a3716, a3717)?
                            };
                            let a3719 = 2u64;
                            nat_add(a3718, a3719)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f696(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3722, _) = v0;
        h3722
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3723) = v0;
        h3723
    };
    let m3724 = v1;
    match m3724.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3725 = v4;
            match m3725.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f718(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f697(v0: Fn23, p3730: u64, p3731: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3729 = (p3730, p3731);
    {
        let mut m3729 = m3729;
        loop {
            let step = {
                let (v1, v2) = m3729;
                let m3726 = v1;
                if m3726 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3726 - 1;
                    let m3728 = {
                        let c3727: Fn23 = v0.clone();
                        c3727.apply(v2.clone())
                    };
                    match m3728 {
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
                Ok(next) => { m3729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f698(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f719(v0)
}

pub fn f699(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3732 = v1;
    match m3732 {
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
                let a3743 = if f118(v7, {
                    let a3741 = 0u64;
                    let a3742 = v8;
                    nat_lt(a3741, a3742)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3744 = f303(false, v8)?;
                append_bytes(a3743, a3744)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3735 = f720({
                let a3733 = f8(Str::lit("n"));
                let a3734 = f468(false, v4, v5)?;
                append_bytes(a3733, a3734)
            })?;
            match m3735 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3739 = f8(Str::lit("nMinus"));
                        let a3740 = f46(v6.clone(), 1u64, {
                            let a3737 = {
                                let a3736 = v6;
                                length_bytes(a3736)
                            };
                            let a3738 = 1u64;
                            nat_sub(a3737, a3738)
                        });
                        append_bytes(a3739, a3740)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f704(v0, v2)
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

pub fn f700(v0: Bytes) -> R<Option<Bytes>> {
    let m3745 = f721(v0)?;
    match m3745 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f722(v1)?))
        }
    }
}

pub fn f701(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3746 = v1;
    match m3746.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3747 = f704(v0, v2)?;
            match m3747 {
                None => {
                    f701(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f701(v0, v3)?))
                }
            }
        }
    }
}

pub fn f702(p3751: List<Adt2>, v1: Bytes) -> bool {
    let m3750 = p3751;
    {
        let mut m3750 = m3750;
        loop {
            let step = {
                let v0 = m3750;
                let m3748 = v0;
                match m3748.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3749 = f712(v2, v1.clone());
                        if match m3749 {
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
                Ok(next) => { m3750 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f703(v0: Adt2) -> R<Option<Bytes>> {
    let m3752 = f385(f276(v0.clone(), f8(Str::lit("$ref"))));
    match m3752 {
        None => {
            Ok(f385(f276(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f629(v1)
        }
    }
}

pub fn f704(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f720(v1)
    } else {
        f700(v1)
    }
}

pub fn f705(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f723(f599(v0), v1)
}

pub fn f706(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3753 = v2;
    match m3753.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3754 = f712(v5.clone(), v3.clone());
            match m3754 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3755 = f704(v0, v7.clone())?;
                    match m3755 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3758 = f636({
                                    let a3756 = v1.clone();
                                    let a3757 = f379(v7)?;
                                    append_bytes(a3756, a3757)
                                }, f561(f522(f276(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f280(f281(f276(v5, f8(Str::lit("required"))))), true)?;
                                match m3758 {
                                    Ok(v10) => {
                                        let m3759 = f706(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3759 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f395(f638(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f638(v12.clone())), f526(f639(v10), f639(v12)))))
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

pub fn f707(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3760 = v0;
    match m3760.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3761 = f712(v2, v1.clone());
            match m3761 {
                None => {
                    f707(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f707(v3, v1))
                }
            }
        }
    }
}

pub fn f708(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3762 = f704(v0, v1)?;
    match m3762 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f597(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f709(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3763 = v1;
    match m3763.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3764 = f708(v0, v2)?;
            match m3764 {
                None => {
                    f709(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f709(v0, v3)?))
                }
            }
        }
    }
}

pub fn f710(v0: Adt2) -> Option<Adt2> {
    let m3765 = f522(f276(v0, f8(Str::lit("patternProperties"))));
    match m3765.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3766 = v2;
            match m3766.uncons() {
                None => {
                    Some({
                        let (_, h3767) = v1;
                        h3767
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f711(v0: Adt2) -> bool {
    f32(f279(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f712(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f280(f281(f276(v0.clone(), f8(Str::lit("required")))))) {
        let m3768 = f276(v0, f8(Str::lit("properties")));
        match m3768 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3769 = f276(v2, v1);
                match m3769 {
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

pub fn f713(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3772 = {
        let a3770 = v0;
        let a3771 = v1;
        index_bytes(a3770, a3771)
    };
    match m3772 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f118({
                let a3773 = 128u64;
                let a3774 = v3;
                nat_le(a3773, a3774)
            }, {
                let a3775 = v3;
                let a3776 = 191u64;
                nat_le(a3775, a3776)
            }) {
                Ok(Some({
                    let a3777 = v3;
                    let a3778 = 128u64;
                    nat_sub(a3777, a3778)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f714(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f118({
        let a3779 = v1;
        let a3780 = v0;
        nat_le(a3779, a3780)
    }, f118({
        let a3781 = v0;
        let a3782 = v2;
        nat_le(a3781, a3782)
    }, f203({
        let a3783 = v0;
        let a3784 = 55296u64;
        nat_lt(a3783, a3784)
    }, {
        let a3785 = 57343u64;
        let a3786 = v0;
        nat_lt(a3785, a3786)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f715(p3823: u64, v1: Bytes, p3824: u64, p3825: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3822 = (p3823, (p3824, p3825));
    {
        let mut m3822 = m3822;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3822;
                let m3787 = v0;
                if m3787 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3787 - 1;
                    let v5: u64 = f116(v1.clone(), v2)?;
                    let a3789 = {
                        let a3788 = v1.clone();
                        length_bytes(a3788)
                    };
                    let a3790 = v2;
                    if nat_le(a3789, a3790) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3791 = v5;
                        let a3792 = 93u64;
                        if nat_eq(a3791, a3792) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3793 = v2;
                                let a3794 = 1u64;
                                nat_add(a3793, a3794)?
                            }))))
                        } else {
                            let a3795 = v5;
                            let a3796 = 92u64;
                            if if nat_eq(a3795, a3796) {
                                true
                            } else {
                                let a3797 = 127u64;
                                let a3798 = v5;
                                nat_lt(a3797, a3798)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3801 = f116(v1.clone(), {
                                    let a3799 = v2;
                                    let a3800 = 1u64;
                                    nat_add(a3799, a3800)?
                                })?;
                                let a3802 = 45u64;
                                if if nat_eq(a3801, a3802) {
                                    f118({
                                        let a3806 = {
                                            let a3803 = v2;
                                            let a3804 = 2u64;
                                            nat_add(a3803, a3804)?
                                        };
                                        let a3807 = {
                                            let a3805 = v1.clone();
                                            length_bytes(a3805)
                                        };
                                        nat_lt(a3806, a3807)
                                    }, f118(f692({
                                        let a3810 = f116(v1.clone(), {
                                            let a3808 = v2;
                                            let a3809 = 2u64;
                                            nat_add(a3808, a3809)?
                                        })?;
                                        let a3811 = 93u64;
                                        nat_eq(a3810, a3811)
                                    }), f692({
                                        let a3814 = f116(v1.clone(), {
                                            let a3812 = v2;
                                            let a3813 = 2u64;
                                            nat_add(a3812, a3813)?
                                        })?;
                                        let a3815 = 92u64;
                                        nat_eq(a3814, a3815)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3816 = v2;
                                        let a3817 = 3u64;
                                        nat_add(a3816, a3817)?
                                    }, List::cons((v5, f116(v1.clone(), {
                                        let a3818 = v2;
                                        let a3819 = 2u64;
                                        nat_add(a3818, a3819)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3820 = v2;
                                        let a3821 = 1u64;
                                        nat_add(a3820, a3821)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716(v0: u64) -> u64 {
    let a3826 = v0;
    let a3827 = 42u64;
    if nat_eq(a3826, a3827) {
        1u64
    } else {
        let a3828 = v0;
        let a3829 = 43u64;
        if nat_eq(a3828, a3829) {
            2u64
        } else {
            let a3830 = v0;
            let a3831 = 63u64;
            if nat_eq(a3830, a3831) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f717(p3840: List<(u64, u64)>, v1: u64) -> bool {
    let m3839 = p3840;
    {
        let mut m3839 = m3839;
        loop {
            let step = {
                let v0 = m3839;
                let m3832 = v0;
                match m3832.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3834 = {
                            let (h3833, _) = v2;
                            h3833
                        };
                        let a3835 = v1;
                        if if nat_le(a3834, a3835) {
                            let a3837 = v1;
                            let a3838 = {
                                let (_, h3836) = v2;
                                h3836
                            };
                            nat_le(a3837, a3838)
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
                Ok(next) => { m3839 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f718(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3845 = {
            let a3843 = {
                let a3841 = v0.clone();
                length_list(a3841)
            };
            let a3844 = {
                let a3842 = v1.clone();
                length_list(a3842)
            };
            nat_add(a3843, a3844)?
        };
        let a3846 = 1u64;
        nat_add(a3845, a3846)?
    };
    Ok(f724({
        let (_, h3847) = {
            let (_, h3848) = {
                let (h3849, _) = f725(Fn24::F726, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3849
            };
            h3848
        };
        h3847
    }))
}

pub fn f719(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3850, _) = v0.clone();
        h3850
    };
    let v2: List<(Bytes, ())> = {
        let (h3851, _) = {
            let (_, h3852) = v0.clone();
            h3852
        };
        h3851
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3853) = {
            let (_, h3854) = v0;
            h3854
        };
        h3853
    };
    let m3855 = v1.clone();
    match m3855.uncons() {
        None => {
            let m3856 = v2;
            match m3856.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3857 = v2.clone();
            match m3857.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f727({
                        let (h3858, _) = v8.clone();
                        h3858
                    }, {
                        let (h3859, _) = v6.clone();
                        h3859
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

pub fn f720(v0: Bytes) -> R<Option<Bytes>> {
    let m3861 = f728({
        let a3860 = v0.clone();
        length_bytes(a3860)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3861 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3863 = {
                let a3862 = v1.clone();
                length_bytes(a3862)
            };
            let a3864 = 0u64;
            if nat_eq(a3863, a3864) {
                let a3866 = {
                    let a3865 = v0;
                    length_bytes(a3865)
                };
                let a3867 = 0u64;
                if nat_eq(a3866, a3867) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f722(v1)?))
            }
        }
    }
}

pub fn f721(v0: Bytes) -> R<Option<Bytes>> {
    if f729({
        let a3868 = v0.clone();
        length_bytes(a3868)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f730(v0)?;
        if f731(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3871 = {
                let a3869 = v1.clone();
                let a3870 = 0u64;
                index_bytes(a3869, a3870)
            };
            match m3871 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f520(f72(v2)?) {
                        Ok(Some({
                            let a3872 = f8(Str::lit("n"));
                            let a3873 = v1;
                            append_bytes(a3872, a3873)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f732({
            let a3874 = v0.clone();
            length_bytes(a3874)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f722(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f733()) {
        f597(v0)
    } else {
        Ok(v0)
    }
}

pub fn f723(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3875 = v0.clone();
    match m3875 {
        Adt2::C0 => {
            Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f683())
            } else {
                Ok(f632(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_) => {
            Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            let m3876 = f734(f559(v0.clone()));
            match m3876 {
                None => {
                    let m3877 = f385(f276(v0.clone(), f8(Str::lit("$ref"))));
                    match m3877 {
                        None => {
                            let m3879 = f276(v0.clone(), f8(Str::lit("allOf")));
                            match m3879 {
                                None => {
                                    let m3884 = f276(v0.clone(), f8(Str::lit("enum")));
                                    match m3884 {
                                        None => {
                                            let m3885 = f276(v0.clone(), f8(Str::lit("const")));
                                            match m3885 {
                                                None => {
                                                    let m3887 = f276(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3887 {
                                                        None => {
                                                            let v32: Bytes = f279(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v32.clone(), Bytes::lit(&[])) {
                                                                !f735(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f683())
                                                            } else {
                                                                if f32(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f633(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f32(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f633(f736(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f32(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f633(f737(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f32(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f633(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f32(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3888 = f522(f276(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3888.uncons() {
                                                                                        None => {
                                                                                            if f640(v0.clone()) {
                                                                                                Ok(f633(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3889 = f678(v0);
                                                                                                match m3889 {
                                                                                                    None => {
                                                                                                        Ok(f681(f8(Str::lit("List")), f738(f683())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f681(f8(Str::lit("List")), f738(f739(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f633(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f632(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f633(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3886 = f385(Some(v29));
                                                    match m3886 {
                                                        None => {
                                                            Ok(f632(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f633(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f633(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3880 = f386(v21);
                                    match m3880.uncons() {
                                        None => {
                                            Ok(f632(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3881 = v23;
                                            match m3881.uncons() {
                                                None => {
                                                    let m3882 = f385(f276(v22, f8(Str::lit("$ref"))));
                                                    match m3882 {
                                                        None => {
                                                            Ok(f632(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3883 = f629(v26)?;
                                                            match m3883 {
                                                                None => {
                                                                    Ok(f632(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f633(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f632(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3878 = f629(v19)?;
                            match m3878 {
                                None => {
                                    Ok(f632(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f633(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f632(if f32(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C8 => {
            Ok(f683())
        }
        Adt2::C9(_, _) => {
            Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C10(_, _, _) => {
            Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f724(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f407(Fn6::F740, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f725(v0: Fn24, p3894: u64, p3895: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3893 = (p3894, p3895);
    {
        let mut m3893 = m3893;
        loop {
            let step = {
                let (v1, v2) = m3893;
                let m3890 = v1;
                if m3890 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3890 - 1;
                    let m3892 = {
                        let c3891: Fn24 = v0.clone();
                        c3891.apply(v2.clone())
                    };
                    match m3892 {
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
                Ok(next) => { m3893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f726(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f741(v0)
}

pub fn f727(v0: Bytes, v1: Bytes) -> bool {
    let a3900 = {
        let a3896 = v0;
        let a3897 = v1;
        compare_bytes(a3896, a3897)
    };
    let a3901 = {
        let a3898 = Bytes::lit(&[]);
        let a3899 = Bytes::lit(&[0x01]);
        compare_bytes(a3898, a3899)
    };
    equal(a3900, a3901)
}

pub fn f728(p3914: u64, v1: Bytes, p3915: u64, p3916: Bytes) -> R<Option<Bytes>> {
    let m3913 = (p3914, (p3915, p3916));
    {
        let mut m3913 = m3913;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3913;
                let m3902 = v0;
                if m3902 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3902 - 1;
                    let a3904 = {
                        let a3903 = v1.clone();
                        length_bytes(a3903)
                    };
                    let a3905 = v2;
                    if nat_le(a3904, a3905) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3906 = f691(v1.clone(), v2)?;
                        match m3906 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3908 = v2;
                                    let a3909 = {
                                        let (_, h3907) = v5;
                                        h3907
                                    };
                                    nat_add(a3908, a3909)?
                                }, {
                                    let a3911 = v3;
                                    let a3912 = f742(v1.clone(), v2, {
                                        let (h3910, _) = v5;
                                        h3910
                                    });
                                    append_bytes(a3911, a3912)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3913 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f729(p3926: u64, v1: Bytes, p3927: u64) -> R<bool> {
    let m3925 = (p3926, p3927);
    {
        let mut m3925 = m3925;
        loop {
            let step = {
                let (v0, v2) = m3925;
                let m3917 = v0;
                if m3917 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3917 - 1;
                    let m3920 = {
                        let a3918 = v1.clone();
                        let a3919 = v2;
                        index_bytes(a3918, a3919)
                    };
                    match m3920 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3921 = f72(v4)?;
                            let a3922 = 128u64;
                            if nat_lt(a3921, a3922) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3923 = v2;
                                    let a3924 = 1u64;
                                    nat_add(a3923, a3924)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f730(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f466(v0.clone(), 0u64, {
        let a3928 = v0;
        length_bytes(a3928)
    }, false, Bytes::lit(&[]))?;
    let m3931 = {
        let a3929 = v1.clone();
        let a3930 = 0u64;
        index_bytes(a3929, a3930)
    };
    match m3931 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f519(v3) {
                let a3938 = f46({
                    let a3932 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3932)
                }, {
                    let a3933 = v3;
                    let a3934 = 65u64;
                    nat_sub(a3933, a3934)
                }, 1u64);
                let a3939 = f46(v1.clone(), 1u64, {
                    let a3936 = {
                        let a3935 = v1;
                        length_bytes(a3935)
                    };
                    let a3937 = 1u64;
                    nat_sub(a3936, a3937)
                });
                Ok(append_bytes(a3938, a3939))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f731(v0: Bytes) -> R<bool> {
    let m3942 = {
        let a3940 = v0;
        let a3941 = 0u64;
        index_bytes(a3940, a3941)
    };
    match m3942 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f203(f518(f72(v1)?), f519(f72(v1)?)))
        }
    }
}

pub fn f732(p3955: u64, v1: Bytes, p3956: u64, p3957: Bytes) -> R<Option<Bytes>> {
    let m3954 = (p3955, (p3956, p3957));
    {
        let mut m3954 = m3954;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3954;
                let m3943 = v0;
                if m3943 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3943 - 1;
                    let a3945 = {
                        let a3944 = v1.clone();
                        length_bytes(a3944)
                    };
                    let a3946 = v2;
                    if nat_le(a3945, a3946) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3947 = f691(v1.clone(), v2)?;
                        match m3947 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3949 = v2;
                                    let a3950 = {
                                        let (_, h3948) = v5;
                                        h3948
                                    };
                                    nat_add(a3949, a3950)?
                                }, {
                                    let a3952 = v3;
                                    let a3953 = f743({
                                        let (h3951, _) = v5;
                                        h3951
                                    });
                                    append_bytes(a3952, a3953)
                                })))
                            }
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

pub fn f733() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f734(p3962: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3961 = p3962;
    {
        let mut m3961 = m3961;
        loop {
            let step = {
                let v0 = m3961;
                let m3958 = v0;
                match m3958.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h3959, _) = v1.clone();
                            h3959
                        }, f744()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3960, _) = v1;
                                h3960
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3961 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f735(v0: Adt2) -> bool {
    if f645(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f645(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f645(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f645(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f645(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f645(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f736(v0: Adt2) -> Bytes {
    let m3963 = f613(f276(v0, f8(Str::lit("minimum"))));
    match m3963 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f737() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f738(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3964 = v0;
    match m3964 {
        Ok(v2) => {
            f633(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f567(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f566(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f739(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f745(16u64, v0, v1)
}

pub fn f740(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f741(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3965, _) = v0.clone();
        h3965
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3966, _) = {
            let (_, h3967) = v0.clone();
            h3967
        };
        h3966
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3968) = {
            let (_, h3969) = v0;
            h3969
        };
        h3968
    };
    let m3970 = v1.clone();
    match m3970.uncons() {
        None => {
            let m3971 = v2;
            match m3971.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3972 = v2.clone();
            match m3972.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f727({
                        let (h3973, _) = v8.clone();
                        h3973
                    }, {
                        let (h3974, _) = v6.clone();
                        h3974
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

pub fn f742(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f203(f118(f203(f518(v2), f519(v2)), f118({
        let a3975 = v2;
        let a3976 = 117u64;
        !nat_eq(a3975, a3976)
    }, {
        let a3977 = v2;
        let a3978 = 85u64;
        !nat_eq(a3977, a3978)
    })), f118(f520(v2), {
        let a3979 = 0u64;
        let a3980 = v1;
        nat_lt(a3979, a3980)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a3981 = v2;
        let a3982 = 65536u64;
        if nat_lt(a3981, a3982) {
            let a3983 = f8(Str::lit("u"));
            let a3984 = f630(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3983, a3984)
        } else {
            let a3985 = f8(Str::lit("U"));
            let a3986 = f630(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3985, a3986)
        }
    }
}

pub fn f743(v0: u64) -> Bytes {
    let a3987 = f8(Str::lit("u"));
    let a3988 = f630(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3987, a3988)
}

pub fn f744() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f745(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3989 = v0;
    if m3989 == 0 {
        Ok(f632(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3989 - 1;
        let m3990 = v1.clone();
        match m3990 {
            Adt2::C0 => {
                Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f683())
                } else {
                    Ok(f632(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_) => {
                Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                let m3991 = f385(f276(v1.clone(), f8(Str::lit("$ref"))));
                match m3991 {
                    None => {
                        if if f645(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f645(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f645(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f633(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f279(v1.clone(), f8(Str::lit("type")));
                            if if f32(v22.clone(), Bytes::lit(&[])) {
                                !f735(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f683())
                            } else {
                                if f32(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f633(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f32(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f633(f736(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f32(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f633(f737(), List::<Adt25>::nil()))
                                        } else {
                                            if f32(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f633(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f32(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3993 = f276(v1, f8(Str::lit("items")));
                                                    match m3993 {
                                                        None => {
                                                            Ok(f681(f8(Str::lit("List")), f683()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f681(f8(Str::lit("List")), f745(v3, v23, {
                                                                let a3994 = v2;
                                                                let a3995 = f8(Str::lit("Item"));
                                                                append_bytes(a3994, a3995)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v22.clone(), f8(Str::lit("object"))) {
                                                        if f640(v1.clone()) {
                                                            Ok(f633(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3996 = f678(v1);
                                                            match m3996 {
                                                                None => {
                                                                    Ok(f681(f8(Str::lit("List")), f738(f683())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f681(f8(Str::lit("List")), f738(f745(v3, v24, {
                                                                        let a3997 = v2;
                                                                        let a3998 = f8(Str::lit("Value"));
                                                                        append_bytes(a3997, a3998)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f632(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3992 = f629(v20)?;
                        match m3992 {
                            None => {
                                Ok(f632(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f633(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C8 => {
                Ok(f683())
            }
            Adt2::C9(_, _) => {
                Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C10(_, _, _) => {
                Ok(f632(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}
