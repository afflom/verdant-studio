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
    C0(Adt15, List<Adt2>, List<Adt2>, List<Bytes>, List<Bytes>),
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
    F182(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F182(k0) => f182(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F121,
    F233,
    F339,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F121 => f121(p0, p1),
            Fn1::F233 => f233(p0, p1),
            Fn1::F339 => f339(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F205(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F205(k0) => f205(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F309(Bytes),
    F310(Bytes),
    F406(Bytes),
    F407(u64, Bytes),
    F414(Bytes),
    F422(Bytes, u64, Bytes, u64),
    F482(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F309(k0) => f309(k0.clone(), p0),
            Fn3::F310(k0) => f310(k0.clone(), p0),
            Fn3::F406(k0) => Ok(f406(k0.clone(), p0)),
            Fn3::F407(k0, k1) => f407(*k0, k1.clone(), p0),
            Fn3::F414(k0) => f414(k0.clone(), p0),
            Fn3::F422(k0, k1, k2, k3) => f422(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F482(k0) => f482(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F313(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F313(k0) => f313(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F398,
    F586,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F398 => f398(p0, p1),
            Fn5::F586 => f586(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F401,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F401 => f401(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F433,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F433 => f433(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F436,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F436 => f436(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F438,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F438 => f438(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F441,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F441 => f441(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F462(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn11::F462(k0, k1, k2) => f462(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F493,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F493 => f493(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F496,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F496 => f496(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F515(u64, Bytes),
    F526(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn14::F515(k0, k1) => Ok(f515(*k0, k1.clone(), p0)),
            Fn14::F526(k0) => f526(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F523,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F523 => f523(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F525,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F525 => f525(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F529(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F529(k0) => f529(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F538,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn18::F538 => f538(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F545,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F545 => f545(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F550,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F550 => f550(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F569,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F569 => f569(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F572,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F572 => f572(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F579,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F579 => f579(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F584,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F584 => f584(p0),
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
        Adt12::C2(f8(Str::lit("npx")), f137(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f51(f52(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h463, _) => {
                h463
            }
        })) {
            Adt12::C2(f8(Str::lit("npx")), f137(Str::lit("build")), f8(Str::lit("prism-code build")))
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
                    Ok(Adt12::C3(f154(f155(v0.clone())?, List::cons((f156(), f157()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f158()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f159()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f160()), List::cons((f8(Str::lit(".github/scripts/readmit.sh")), f161()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f162()), List::cons((f8(Str::lit("docs/catalog.md")), f163(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f164(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f165(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f166()), List::<(Bytes, Bytes)>::nil()))))))))))), f8(Str::lit("published the workflows, the export and readmit scripts, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
            f173(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                f174(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                    f175(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                        f176(v3, v1)
                    } else {
                        if f32(v0, f8(Str::lit("record-error"))) {
                            f177(v3, v1, v2)
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
    f178(v0.clone(), 0u64, {
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
    let m490 = f179(f180(v0.clone(), v1.clone()), v1.clone());
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
            let m500 = f181(v2, v1.clone())?;
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
        let (h502, _) = f76(Fn0::F182(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
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
    Ok(f183(f57(v0, v1)?))
}

pub fn f97(v0: List<Adt2>, v1: Bytes) -> List<Adt2> {
    let m511 = v0;
    match m511.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f184(v2, v1.clone()), f97(v3, v1))
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
    let a520 = f185(v0)?;
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
        let m527 = f186(v0.clone(), f60())?;
        match m527 {
            None => {
                if f51(f52(f189(), match v0.clone() {
                    Adt1::C0(_, _, h528, _) => {
                        h528
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f189()))
                } else {
                    let m529 = f56(v0.clone(), List::cons(f190(), f191(f60())?));
                    match m529.uncons() {
                        None => {
                            Ok(Adt12::C3(f192(v0.clone(), f60(), v1.clone(), v2.clone(), f193(f64(v0, f190()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f187(v3.clone())?, f188(v3, v2)))
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
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f194(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m530 - 1;
                    let m533 = {
                        let a531 = v0.clone();
                        let a532 = v1;
                        index_bytes(a531, a532)
                    };
                    match m533 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f194(v0.clone(), v5, v1, v6)))
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
                                    }, f194(v0.clone(), v5, v1, v6)))))))
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
    let v2: u64 = f195(v0.clone(), v1, {
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
        f196(v0.clone(), v1, {
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
    let v4: u64 = f197(v0.clone(), v3)?;
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
        f197(v0.clone(), v7)?
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
    let v12: bool = f117(v10, f198({
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
        f197(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a648 = v14;
        let a649 = v13;
        nat_sub(a648, a649)
    });
    let v16: u64 = f199(v15.clone())?;
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
                f198(if v10 {
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
                f200(v17)?
            }
        };
        match m669 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m674 = f201(f46(v0.clone(), v3, v5), f46(v0, v7, {
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
                            let m680 = f200({
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
                                    if f202(v21, v20) {
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
    let m692 = f203(v0.clone(), v3);
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
    let m708 = f204(Fn2::F205(v1), None::<Bytes>, v0);
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
    }, f206()) {
        if f138(match v0 {
            Adt1::C0(_, h710, _, _) => {
                h710
            }
        }, f207()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f207(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f206(), List::<Bytes>::nil())
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
    Ok(f56(v0.clone(), List::cons(f208(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f136(v0)?)))))
}

pub fn f127(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f209(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(f96(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
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
            f125(f210(f183(f93(v1, f8(Str::lit("defers"))))), f130(v2))
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
    let v3: List<(Bytes, Bytes)> = f154(f211(v2.clone()), f212(f213(v0, v1.clone(), v2)?));
    f214(f145(f215(v1))?, v3.clone(), {
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
            if if f216(v2.clone(), f8(Str::lit("subject"))) {
                f217(v0.clone(), v2.clone())
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
            let m723 = f218(f95(f64(v0.clone(), v2.clone()))?, f85(v2, f207()))?;
            match m723 {
                None => {
                    f135(v0, v3)
                }
                Some(v4) => {
                    Ok(f219(v4, f135(v0, v3)?))
                }
            }
        }
    }
}

pub fn f136(v0: Adt1) -> R<List<Bytes>> {
    Ok(f125(f145(f220(f206(), f147(match v0.clone() {
        Adt1::C0(_, h724, _, _) => {
            h724
        }
    }, f206())))?, f145(f220(f207(), f147(match v0 {
        Adt1::C0(_, h725, _, _) => {
            h725
        }
    }, f207())))?))
}

pub fn f137(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
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
    f145(f220(f139(), f147(match v0 {
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
                f221(v4.clone(), f95(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f222(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
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
    let m733 = f56(v0.clone(), f223());
    match m733.uncons() {
        None => {
            let m734 = f224(v0.clone(), f223())?;
            match m734.uncons() {
                None => {
                    let m737 = f225(match v0.clone() {
                        Adt1::C0(_, h735, _, _) => {
                            h735
                        }
                    }, f125(f226(), f227(match v0.clone() {
                        Adt1::C0(_, h736, _, _) => {
                            h736
                        }
                    }, false)));
                    match m737.uncons() {
                        None => {
                            let m739 = f56(v0.clone(), f227(match v0.clone() {
                                Adt1::C0(_, h738, _, _) => {
                                    h738
                                }
                            }, true));
                            match m739.uncons() {
                                None => {
                                    let m740 = f56(v0.clone(), f228());
                                    match m740.uncons() {
                                        None => {
                                            let m741 = f229(v0.clone())?;
                                            match m741 {
                                                None => {
                                                    let m743 = f230(v0.clone(), f227(match v0.clone() {
                                                        Adt1::C0(_, h742, _, _) => {
                                                            h742
                                                        }
                                                    }, true))?;
                                                    match m743 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f231(Adt16::C0(Adt15::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f209(v0.clone())?, f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f210(f96(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f227(match v0 {
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
    Ok(f232(f120(Fn1::F233, List::<Bytes>::nil(), f234(v0)?)))
}

pub fn f146(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m748 = v1;
    match m748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f235(v0.clone(), v2), f146(v0, v3))
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
                        if f236(v0.clone(), v1.clone(), v3.clone())? {
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
            Ok(List::cons((f237(v1.clone())?, v1), f149(v2)?))
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
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f238(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f238(v1, List::<Bytes>::nil()), v2))))
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
    }, f239())) {
        Ok(Adt12::C1(List::cons(f239(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f146(f239(), f147(match v0.clone() {
            Adt1::C0(_, h765, _, _) => {
                h765
            }
        }, f239())))?, v1.clone());
        let m766 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m766.uncons() {
            None => {
                let v5: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m767 = f56(v0.clone(), f240(v0.clone(), v2.clone(), v5.clone())?);
                match m767.uncons() {
                    None => {
                        let m768 = f241(v0.clone(), v2, v5)?;
                        match m768 {
                            None => {
                                f242(v0, v1)
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
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f243(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f156() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f157() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}            target/release/verdant_xml\u{a}            target/release/verdant_admit\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      # Restored on every run, not only when certified/ changed: the end-to-end propose-command step below\u{a}      # exports with the attested lexlean binary, and a fresh cargo install does not reproduce its hash.\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3-rust-1.97.0\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          [ \"$(sha256sum target/lexlean/bin/lexlean 2>/dev/null | cut -c1-64)\" = \"$(jq -r .lexlean.executable_sha256 certified/evidence/attestation.json)\" ] || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked --version 0.4.3 lexlean --root target/lexlean; }\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers certified-xml certified-admit)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/, certified-readers/, certified-xml/ or certified-admit/:\"\u{a}            git status --short -- certified certified-readers certified-xml certified-admit | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          # Each encoding admitted through admit-translated recorded the reader command that printed it: run it again and fail if the output differs.\u{a}          target/release/verdant_readers verify-translations model/admissions.json\u{a}          if [ -x target/release/verdant_xml ]; then target/release/verdant_xml verify-translations model/admissions.json; fi\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          # certified-xml/ and certified-admit/ join the comparison once main holds them: the export that first writes it is the one that adds it.\u{a}          bundles=\"certified certified-readers\"\u{a}          if [ -n \"$(git ls-files certified-xml | head -n 1)\" ]; then bundles=\"$bundles certified-xml\"; fi\u{a}          if [ -n \"$(git ls-files certified-admit | head -n 1)\" ]; then bundles=\"$bundles certified-admit\"; fi\u{a}          if [ -n \"$(git status --porcelain -- $bundles)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- $bundles | head -20\u{a}            attested=$(git show HEAD:certified/evidence/attestation.json | jq -r .lexlean.executable_sha256); used=$(sha256sum target/lexlean/bin/lexlean | cut -c1-64)\u{a}            [ \"$attested\" = \"$used\" ] || echo \"::error::the export ran with lexlean $used, but certified/ attests lexlean $attested. lexlean is built with rust 1.97.0 and its cargo home remapped, so a different hash means the toolchain or the runner image changed\"\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f158() -> Bytes {
    f7(List::cons(f244(), List::cons(f245(), List::cons(f246(), List::cons(f247(), List::cons(f248(), List::cons(f249(), List::<Bytes>::nil())))))))
}

pub fn f159() -> Bytes {
    f7(List::cons(f250(), List::cons(f251(), List::cons(f252(), List::cons(f253(), List::<Bytes>::nil())))))
}

pub fn f160() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, readers, readers/xml and admit when they exist.\u{a}# Lean's proof of a project grows with the square of its function count, so the readers are split by family and admission is its own project.\u{a}pids=\"\"\u{a}for project in studio readers readers/xml admit; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.${project//\\//-}\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}if [ -f \"$log.readers-xml\" ]; then cat \"$log.readers-xml\" >> \"$log\"; fi\u{a}if [ -f \"$log.admit\" ]; then cat \"$log.admit\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}if [ -f readers/xml/lexlean.toml ]; then\u{a}  rm -rf certified-xml\u{a}  cp -r readers/xml/.lexlean/production/*/ certified-xml\u{a}  rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-xml\u{a}fi\u{a}if [ -f admit/lexlean.toml ]; then\u{a}  rm -rf certified-admit\u{a}  cp -r admit/.lexlean/production/*/ certified-admit\u{a}  rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-admit\u{a}fi\u{a}"))
}

pub fn f161() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env bash\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# A change to a reader can change what it prints for an encoding already admitted, and the studio check\u{a}")), List::cons(f8(Str::lit("# replays every admitted reader command (verify-translations). So the change re-derives those encodings\u{a}")), List::cons(f8(Str::lit("# itself: each Catalog encoding whose replay now differs is admitted again through read-admit, with the\u{a}")), List::cons(f8(Str::lit("# command and citation its admission row records, and the pull request carries both (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("# It prints each construct it admitted again, one per line.\u{a}")), List::cons(f8(Str::lit("set -euo pipefail\u{a}")), List::cons(f8(Str::lit("readers=$1\u{a}")), List::cons(f8(Str::lit("mkdir -p target\u{a}")), List::cons(f8(Str::lit("rows=$(jq length model/admissions.json)\u{a}")), List::cons(f8(Str::lit("for try in $(seq 1 \"$rows\"); do\u{a}")), List::cons(f8(Str::lit("  if \"$readers\" verify-translations model/admissions.json > target/replay.txt 2>&1; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("  construct=$(sed -n 's/^\\([^:]*\\): its reader command now prints .*/\\1/p' target/replay.txt | head -n 1)\u{a}")), List::cons(f8(Str::lit("  case \"$construct\" in\u{a}")), List::cons(f8(Str::lit("    Catalog.*) ;;\u{a}")), List::cons(f8(Str::lit("    *) cat target/replay.txt >&2; echo \"readmit: only a Catalog encoding whose reader now prints other output is admitted again\" >&2; exit 1;;\u{a}")), List::cons(f8(Str::lit("  esac\u{a}")), List::cons(f8(Str::lit("  row=$(jq -c --arg construct \"$construct\" '[.[] | select(.construct == $construct and .translation != null)][0]' model/admissions.json)\u{a}")), List::cons(f8(Str::lit("  authority=$(jq -r '.std.authority // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  cited=$(jq -r '.std.row // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  if [ -z \"$authority\" ]; then echo \"readmit: $construct cites no std authority, so read-admit cannot cite it again\" >&2; exit 1; fi\u{a}")), List::cons(f8(Str::lit("  mapfile -t command < <(jq -r '.translation.command[]' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  \"$readers\" read-admit \"${construct#Catalog.}\" \"${command[@]}\" std \"$authority\" $cited >/dev/null\u{a}")), List::cons(f8(Str::lit("  echo \"$construct\"\u{a}")), List::cons(f8(Str::lit("done\u{a}")), List::cons(f8(Str::lit("echo \"readmit: the replay did not settle after $rows admissions\" >&2\u{a}")), List::cons(f8(Str::lit("exit 1\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))
}

pub fn f162() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f163(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f127(v0.clone())?;
    let v2: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f134(v1.clone(), f135(v0.clone(), f136(v0.clone())?)?);
    let v4: List<List<Bytes>> = f254(f255(f64(v0, f208()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f256(v4.clone(), f257(v4), f213(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f133(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f164(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f258(f127(v0.clone())?, f135(v0.clone(), f136(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f165(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f259(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f260(f96(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f166() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f167(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m771 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m771.uncons() {
        None => {
            if f261(f95(f64(v0, v1))?)? {
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
    let m772 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m772.uncons() {
        None => {
            let v5: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f262(v5)?
                } else {
                    f263(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f235(f264(), f265(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f235(f264(), f265(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f266()
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
    let m773 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m773.uncons() {
        None => {
            let m774 = f267(f95(f64(v0.clone(), v1.clone()))?)?;
            match m774 {
                None => {
                    Ok(Adt12::C4(f268()))
                }
                Some(v4) => {
                    let m775 = f269(f95(f64(v0, v1))?)?;
                    match m775 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f270(v5)))
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
    let v2: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f117(f271(f272(v2.clone())?), f51({
            let a777 = {
                let a776 = f273(f272(v2)?);
                length_list(a776)
            };
            let a778 = 0u64;
            nat_eq(a777, a778)
        })) {
            Ok(Adt12::C3(List::cons((f235(f264(), f265(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f235(f264(), f265(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f171(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m779 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m779.uncons() {
        None => {
            let m780 = f274(f95(f64(v0, v1))?)?;
            match m780 {
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

pub fn f172(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m781 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m781.uncons() {
        None => {
            let v5: Bytes = f235(if v2 {
                f207()
            } else {
                f206()
            }, f265(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m782 = f218(f95(f64(v0.clone(), v1.clone()))?, v2)?;
                match m782 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a784 = {
                            let a783 = v6;
                            length_list(a783)
                        };
                        let a785 = 0u64;
                        if nat_eq(a784, a785) {
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
    let m786 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m786.uncons() {
        None => {
            let m787 = f57(v0.clone(), v1.clone())?;
            match m787 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f275())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt2 = {
                            let m788 = f93(v4.clone(), f8(Str::lit("source")));
                            match m788 {
                                None => {
                                    Adt2::C9
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f92(f93(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f235(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f235(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m789 = v8;
                        match m789 {
                            None => {
                                f276(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f276(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f235(v9, f265(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h790, _) => {
                                            h790
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m791 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m791.uncons() {
                                            None => {
                                                if f32(f277(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f276(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f174(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m792 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m792.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f255(v4.clone())?;
            if f51(f98(f278(v5.clone()), f279())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f280(Str::lit("id"), v6.clone())?, List::cons(f280(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f280(Str::lit("artifact"), f208())?, List::cons(f280(Str::lit("checksum"), f277(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f208(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f281(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f282(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f122(f257(f254(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f175(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m793 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m793.uncons() {
        None => {
            let m794 = f57(v0, v1)?;
            match m794 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f283(f183(Some(v4))) {
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

pub fn f176(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m795 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f153(), List::<Bytes>::nil())));
    match m795.uncons() {
        None => {
            let m796 = f284(f95(f64(v0.clone(), v1))?)?;
            match m796 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt2> = f96(v0, f153())?;
                    let v6: List<Bytes> = f142(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f145(f142(v5.clone(), f8(Str::lit("at"))))?;
                    let a798 = {
                        let a797 = v6.clone();
                        length_list(a797)
                    };
                    let a799 = 0u64;
                    if if nat_eq(a798, a799) {
                        true
                    } else {
                        f51({
                            let a802 = {
                                let a800 = f145(f125(v6.clone(), v7.clone()))?;
                                length_list(a800)
                            };
                            let a803 = {
                                let a801 = f125(v6.clone(), v7);
                                length_list(a801)
                            };
                            nat_eq(a802, a803)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f285(f153(), f219(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f177(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m804 = v2;
    match m804.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m805 = v4;
            match m805.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m806 = v6.clone();
                    match m806.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m807 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f286(), List::<Bytes>::nil())));
                            match m807.uncons() {
                                None => {
                                    let v11: Bytes = f122(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt2> = f96(v0, f286())?;
                                    if f287(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f285(f286(), f219(v13, List::cons(f288(v11.clone(), v12, v5, v3), List::<Adt2>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f178(v0: Bytes, p861: u64, p862: u64) -> R<bool> {
    let m860 = (p861, p862);
    {
        let mut m860 = m860;
        loop {
            let step = {
                let (v1, v2) = m860;
                let m808 = v2;
                if m808 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m808 - 1;
                    let m811 = {
                        let a809 = v0.clone();
                        let a810 = v1;
                        index_bytes(a809, a810)
                    };
                    match m811 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f289({
                                let v5: u8 = v4;
                                let a856 = {
                                    let a832 = {
                                        let a820 = {
                                            let a814 = {
                                                let a812 = v5;
                                                let a813 = 128u8;
                                                fixed_u8::bit_and(a812, a813)
                                            };
                                            let a815 = 128u8;
                                            if equal(a814, a815) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a821 = {
                                            let a818 = {
                                                let a816 = v5;
                                                let a817 = 64u8;
                                                fixed_u8::bit_and(a816, a817)
                                            };
                                            let a819 = 64u8;
                                            if equal(a818, a819) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a820, a821)?
                                    };
                                    let a833 = {
                                        let a830 = {
                                            let a824 = {
                                                let a822 = v5;
                                                let a823 = 32u8;
                                                fixed_u8::bit_and(a822, a823)
                                            };
                                            let a825 = 32u8;
                                            if equal(a824, a825) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a831 = {
                                            let a828 = {
                                                let a826 = v5;
                                                let a827 = 16u8;
                                                fixed_u8::bit_and(a826, a827)
                                            };
                                            let a829 = 16u8;
                                            if equal(a828, a829) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a830, a831)?
                                    };
                                    nat_add(a832, a833)?
                                };
                                let a857 = {
                                    let a854 = {
                                        let a842 = {
                                            let a836 = {
                                                let a834 = v5;
                                                let a835 = 8u8;
                                                fixed_u8::bit_and(a834, a835)
                                            };
                                            let a837 = 8u8;
                                            if equal(a836, a837) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a843 = {
                                            let a840 = {
                                                let a838 = v5;
                                                let a839 = 4u8;
                                                fixed_u8::bit_and(a838, a839)
                                            };
                                            let a841 = 4u8;
                                            if equal(a840, a841) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a842, a843)?
                                    };
                                    let a855 = {
                                        let a852 = {
                                            let a846 = {
                                                let a844 = v5;
                                                let a845 = 2u8;
                                                fixed_u8::bit_and(a844, a845)
                                            };
                                            let a847 = 2u8;
                                            if equal(a846, a847) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a853 = {
                                            let a850 = {
                                                let a848 = v5;
                                                let a849 = 1u8;
                                                fixed_u8::bit_and(a848, a849)
                                            };
                                            let a851 = 1u8;
                                            if equal(a850, a851) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a852, a853)?
                                    };
                                    nat_add(a854, a855)?
                                };
                                nat_add(a856, a857)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a858 = v1;
                                    let a859 = 1u64;
                                    nat_add(a858, a859)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(p867: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m866 = p867;
    {
        let mut m866 = m866;
        loop {
            let step = {
                let v0 = m866;
                let m863 = v0;
                match m863.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h864, _) = v2;
                            h864
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h865) = v2;
                                h865
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m866 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m871 = {
        let a869 = match v0 {
            Adt1::C0(_, _, _, h868) => {
                h868
            }
        };
        let a870 = f40(v1);
        index_list(a869, a870)
    };
    match m871 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f181(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m872 = f44(v0, 34u8)?;
        match m872.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m873 = v3;
                match m873.uncons() {
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

pub fn f182(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f290(v0, v1)
}

pub fn f183(v0: Option<Adt2>) -> List<Adt2> {
    let m874 = v0;
    match m874 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f291(v1)
        }
    }
}

pub fn f184(v0: Adt2, v1: Bytes) -> Adt2 {
    let m875 = v0;
    match m875 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r876) => {
            let v12 = (*r876).clone();
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
        Adt2::C7(r877) => {
            let v10 = (*r877).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r878) => {
            let v5 = (*r878).clone();
            Adt2::C8(std::rc::Rc::new(f184(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r879, r880) => {
            let v18 = (*r879).clone();
            let v19 = (*r880).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r881, r882) => {
            let v3 = (*r881).clone();
            let v4 = (*r882).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt2::C11(v2, std::rc::Rc::new(f292(v3, v1.clone())), std::rc::Rc::new(f184(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f184(v4, v1)))
            }
        }
    }
}

pub fn f185(v0: Adt2) -> R<Bytes> {
    let m883 = v0;
    match m883 {
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
            f293(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f294(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f295(v9, v10)
        }
        Adt2::C6(v11) => {
            f296(v11)
        }
        Adt2::C7(r884) => {
            let v12 = (*r884).clone();
            Ok(f297(f298(v12)?))
        }
        Adt2::C8(r885) => {
            let v13 = (*r885).clone();
            f299(f300(v13)?)
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

pub fn f186(v0: Adt1, p889: List<Adt13>) -> R<Option<Adt13>> {
    let m888 = p889;
    {
        let mut m888 = m888;
        loop {
            let step = {
                let v1 = m888;
                let m886 = v1;
                match m886.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f187(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h887, _) => {
                                h887
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
                Ok(next) => { m888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Adt13) -> R<Bytes> {
    Ok(f235(f301(), f265(f302(v0))?))
}

pub fn f188(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f302(v0), List::<Bytes>::nil())))))
}

pub fn f189() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f190() -> Bytes {
    f303()
}

pub fn f191(v0: List<Adt13>) -> R<List<Bytes>> {
    let m890 = v0;
    match m890.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f187(v1)?, f191(v2)?))
        }
    }
}

pub fn f192(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m891 = v1;
    match m891.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f304(v5.clone(), v2.clone(), v3.clone(), f277(f64(v0.clone(), f187(v5)?))?, v4.clone())?), f192(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f193(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f305(v0.clone())?;
    let v2: u64 = f306(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a892 = v1;
        let a893 = v2;
        nat_sub(a892, a893)
    }))
}

pub fn f194(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a896 = v3;
    let a897 = f46(v0, v1, {
        let a894 = v2;
        let a895 = v1;
        nat_sub(a894, a895)
    });
    append_bytes(a896, a897)
}

pub fn f195(v0: Bytes, p910: u64, p911: u64) -> R<u64> {
    let m909 = (p910, p911);
    {
        let mut m909 = m909;
        loop {
            let step = {
                let (v1, v2) = m909;
                let m898 = v2;
                if m898 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m898 - 1;
                    let v4: u64 = f115(v0.clone(), v1)?;
                    let a899 = v4;
                    let a900 = 34u64;
                    if nat_eq(a899, a900) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a901 = v4;
                        let a902 = 92u64;
                        if nat_eq(a901, a902) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a903 = v4;
                            let a904 = 32u64;
                            if nat_lt(a903, a904) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a905 = v4;
                                let a906 = 256u64;
                                if nat_eq(a905, a906) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a907 = v1;
                                        let a908 = 1u64;
                                        nat_add(a907, a908)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Bytes, p937: u64, p938: u64, p939: Bytes) -> R<Option<(u64, Bytes)>> {
    let m936 = (p937, (p938, p939));
    {
        let mut m936 = m936;
        loop {
            let step = {
                let (v1, (v2, v3)) = m936;
                let m912 = v2;
                if m912 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m912 - 1;
                    let v5: u64 = f195(v0.clone(), v1, {
                        let a916 = {
                            let a914 = {
                                let a913 = v0.clone();
                                length_bytes(a913)
                            };
                            let a915 = v1;
                            nat_sub(a914, a915)
                        };
                        let a917 = 1u64;
                        nat_add(a916, a917)?
                    })?;
                    let v6: Bytes = {
                        let a920 = v3;
                        let a921 = f46(v0.clone(), v1, {
                            let a918 = v5;
                            let a919 = v1;
                            nat_sub(a918, a919)
                        });
                        append_bytes(a920, a921)
                    };
                    let v7: u64 = f115(v0.clone(), v5)?;
                    let a922 = v7;
                    let a923 = 34u64;
                    if nat_eq(a922, a923) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a924 = v5;
                            let a925 = 1u64;
                            nat_add(a924, a925)?
                        }, v6))))
                    } else {
                        let a926 = v7;
                        let a927 = 92u64;
                        if nat_eq(a926, a927) {
                            let m928 = f307(v0.clone(), v5, v6)?;
                            match m928 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a931 = {
                                        let (_, h929) = {
                                            let (_, h930) = v8.clone();
                                            h930
                                        };
                                        h929
                                    };
                                    let a932 = 0u64;
                                    if nat_eq(a931, a932) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h933, _) = v8;
                                            h933
                                        }, (v4, {
                                            let (h934, _) = {
                                                let (_, h935) = v8;
                                                h935
                                            };
                                            h934
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
                Ok(next) => { m936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Bytes, v1: u64) -> R<u64> {
    let (h940, _) = f308(Fn3::F309(v0.clone()), {
        let a942 = {
            let a941 = v0;
            length_bytes(a941)
        };
        let a943 = 1u64;
        nat_add(a942, a943)?
    }, v1)?;
    Ok(h940)
}

pub fn f198(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f199(v0: Bytes) -> R<u64> {
    let (h944, _) = f308(Fn3::F310(v0.clone()), {
        let a946 = {
            let a945 = v0;
            length_bytes(a945)
        };
        let a947 = 1u64;
        nat_add(a946, a947)?
    }, 0u64)?;
    Ok(h944)
}

pub fn f200(v0: Bytes) -> R<Option<u64>> {
    if f311(v0.clone())? {
        Ok(Some({
            let (_, h948) = {
                let (h949, _) = f312(Fn4::F313(v0.clone()), {
                    let a950 = v0;
                    length_bytes(a950)
                }, (0u64, 0u64))?;
                h949
            };
            h948
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f201(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a952 = {
            let a951 = v1.clone();
            length_bytes(a951)
        };
        let a953 = f314(v1)?;
        nat_sub(a952, a953)
    });
    let v5: Bytes = {
        let a954 = v0;
        let a955 = v4.clone();
        append_bytes(a954, a955)
    };
    let v6: u64 = f199(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a957 = {
            let a956 = v5;
            length_bytes(a956)
        };
        let a958 = v6;
        nat_sub(a957, a958)
    });
    let v8: u64 = {
        let a959 = v4;
        length_bytes(a959)
    };
    let v9: u64 = {
        let a960 = v7.clone();
        length_bytes(a960)
    };
    let a961 = v9;
    let a962 = 0u64;
    if nat_eq(a961, a962) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f198(v2, {
            let a963 = v3;
            let a964 = v8;
            nat_lt(a963, a964)
        }) {
            let v10: u64 = if v2 {
                let a965 = v3;
                let a966 = v8;
                nat_add(a965, a966)?
            } else {
                let a967 = v8;
                let a968 = v3;
                nat_sub(a967, a968)
            };
            let v11: u64 = f314(v7.clone())?;
            let a969 = v10;
            let a970 = v11;
            if nat_le(a969, a970) {
                Ok(Some((f46(v7, 0u64, {
                    let a971 = v9;
                    let a972 = v10;
                    nat_sub(a971, a972)
                }), 0u64)))
            } else {
                let a975 = {
                    let a973 = v10;
                    let a974 = v11;
                    nat_sub(a973, a974)
                };
                let a976 = 19u64;
                if nat_le(a975, a976) {
                    Ok(Some((f46(v7, 0u64, {
                        let a977 = v9;
                        let a978 = v11;
                        nat_sub(a977, a978)
                    }), {
                        let a979 = v10;
                        let a980 = v11;
                        nat_sub(a979, a980)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a981 = v3;
                let a982 = v8;
                nat_sub(a981, a982)
            };
            let a985 = {
                let a983 = v9;
                let a984 = v12;
                nat_add(a983, a984)?
            };
            let a986 = 19u64;
            if nat_le(a985, a986) {
                Ok(Some(({
                    let a987 = v7;
                    let a988 = f315(v12);
                    append_bytes(a987, a988)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f202(v0: bool, v1: u64) -> bool {
    if v0 {
        let a989 = 0u64;
        let a990 = v1;
        if nat_lt(a989, a990) {
            let a991 = v1;
            let a992 = 9223372036854775808u64;
            nat_le(a991, a992)
        } else {
            false
        }
    } else {
        let a993 = v1;
        let a994 = 9223372036854775807u64;
        nat_le(a993, a994)
    }
}

pub fn f203(v0: Bytes, v1: u64) -> Option<u8> {
    let a995 = v0;
    let a996 = v1;
    index_bytes(a995, a996)
}

pub fn f204(v0: Fn2, p1000: Option<Bytes>, p1001: List<Bytes>) -> Option<Bytes> {
    let m999 = (p1000, p1001);
    {
        let mut m999 = m999;
        loop {
            let step = {
                let (v1, v2) = m999;
                let m997 = v2;
                match m997.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c998: Fn2 = v0.clone();
                            c998.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m999 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1002 = v1;
    match m1002 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1005 = {
                    let a1003 = v3;
                    let a1004 = v0;
                    append_bytes(a1003, a1004)
                };
                let a1006 = v2;
                append_bytes(a1005, a1006)
            })
        }
    }
}

pub fn f206() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f207() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f208() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f209(v0: Adt1) -> R<List<Bytes>> {
    let m1007 = f255(f64(v0, f208()))?;
    match m1007.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f257(v2))
        }
    }
}

pub fn f210(v0: List<Adt2>) -> List<Bytes> {
    let m1008 = v0;
    match m1008.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1009 = f92(Some(v1));
            match m1009 {
                None => {
                    f210(v2)
                }
                Some(v3) => {
                    List::cons(v3, f210(v2))
                }
            }
        }
    }
}

pub fn f211(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1010 = v0;
    match m1010.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f211(v2))
        }
    }
}

pub fn f212(v0: List<Adt17>) -> List<(Bytes, Bytes)> {
    let m1011 = v0;
    match m1011.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt17::C0(h1012, _, _) => {
                    h1012
                }
            }, match v1 {
                Adt17::C0(_, _, h1013) => {
                    h1013
                }
            }), f212(v2))
        }
    }
}

pub fn f213(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Adt17>> {
    let v3: List<(Bytes, Bytes)> = f316(v1);
    let v4: List<(Bytes, Bytes)> = f317(v0, v2);
    f318(v3.clone(), f154(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f214(p1020: List<Bytes>, v1: List<(Bytes, Bytes)>, p1021: u64) -> R<List<Bytes>> {
    let m1019 = (p1020, p1021);
    {
        let mut m1019 = m1019;
        loop {
            let step = {
                let (v0, v2) = m1019;
                let m1014 = v2;
                if m1014 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1014 - 1;
                    let v4: List<Bytes> = f145(f125(v0.clone(), f319(v0.clone(), v1.clone())))?;
                    let a1017 = {
                        let a1015 = v4.clone();
                        length_list(a1015)
                    };
                    let a1018 = {
                        let a1016 = v0.clone();
                        length_list(a1016)
                    };
                    if nat_eq(a1017, a1018) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1019 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: List<Adt2>) -> List<Bytes> {
    let m1022 = v0;
    match m1022.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1023 = f93(v1, f8(Str::lit("std")));
            match m1023 {
                None => {
                    f215(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f216(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f215(v2))
                    } else {
                        f215(v2)
                    })
                }
            }
        }
    }
}

pub fn f216(v0: Adt2, v1: Bytes) -> bool {
    f51({
        let a1025 = {
            let a1024 = f58(v0, v1);
            length_bytes(a1024)
        };
        let a1026 = 0u64;
        nat_eq(a1025, a1026)
    })
}

pub fn f217(v0: Adt14, v1: Adt2) -> bool {
    if f320(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f320(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f218(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt2>>> {
    let m1027 = v0;
    match m1027.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f193(v2)?;
            let a1029 = {
                let a1028 = v4.clone();
                length_bytes(a1028)
            };
            let a1030 = 0u64;
            if nat_eq(a1029, a1030) {
                f218(v3, v1)
            } else {
                let m1031 = f43(v4)?;
                match m1031 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f321(v5.clone())
                        } else {
                            f322(v5.clone())
                        } {
                            let m1032 = f218(v3, v1)?;
                            match m1032 {
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

pub fn f219(v0: List<Adt2>, v1: List<Adt2>) -> List<Adt2> {
    let m1033 = v0;
    match m1033.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f219(v3, v1))
        }
    }
}

pub fn f220(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1034 = v1;
    match m1034.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f235(v0.clone(), v2), f220(v0, v3))
            } else {
                f220(v0, v3)
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1035 = v1;
    match m1035.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125({
                let a1037 = {
                    let a1036 = f193(v4.clone())?;
                    length_bytes(a1036)
                };
                let a1038 = 0u64;
                if nat_eq(a1037, a1038) {
                    List::<Bytes>::nil()
                } else {
                    f323(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f221(v0, v5, v2, v3)?))
        }
    }
}

pub fn f222(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f223() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f208(), List::<Bytes>::nil())))))))
}

pub fn f224(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1039 = v1;
    match m1039.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m1040 = f57(v0.clone(), v2.clone())?;
                match m1040 {
                    None => {
                        Ok(List::cons(v2, f224(v0, v3)?))
                    }
                    Some(_) => {
                        f224(v0, v3)
                    }
                }
            } else {
                f224(v0, v3)
            }
        }
    }
}

pub fn f225(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1041 = v1;
    match m1041.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f138(v0.clone(), v2.clone()) {
                f225(v0, v3)
            } else {
                List::cons(v2, f225(v0, v3))
            }
        }
    }
}

pub fn f226() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f227(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1042 = v0;
    match m1042.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f125(f324({
                let (h1043, _) = v2;
                h1043
            }, {
                let (_, h1044) = v2;
                h1044
            }, v1), f227(v3, v1))
        }
    }
}

pub fn f228() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::cons(f8(Str::lit(".github/scripts/readmit.sh")), List::<Bytes>::nil())))))
}

pub fn f229(v0: Adt1) -> R<Option<Bytes>> {
    f325(f326(v0, f228())?)
}

pub fn f230(v0: Adt1, p1048: List<Bytes>) -> R<Option<Bytes>> {
    let m1047 = p1048;
    {
        let mut m1047 = m1047;
        loop {
            let step = {
                let v1 = m1047;
                let m1045 = v1;
                match m1045.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1046 = f327(f64(v0.clone(), v2.clone()), f328())?;
                        match m1046 {
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
                Ok(next) => { m1047 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h1049, _, _, _, _) => {
            h1049
        }
    };
    let v2: Adt18 = f329(v1.clone())?;
    let v3: List<Bytes> = f330(v1.clone());
    let v4: List<Bytes> = f142(match v0.clone() {
        Adt16::C0(_, _, h1050, _, _) => {
            h1050
        }
    }, f8(Str::lit("path")));
    let m1052 = f331(v2, List::<u64>::nil(), f332(v1.clone(), match v0.clone() {
        Adt16::C0(_, h1051, _, _, _) => {
            h1051
        }
    }, 0u64)?);
    match m1052 {
        None => {
            if f334(f335(match v0.clone() {
                Adt16::C0(_, _, _, _, h1054) => {
                    h1054
                }
            }, f125(match v0.clone() {
                Adt16::C0(_, _, _, _, h1055) => {
                    h1055
                }
            }, v4.clone()))?, f335(v4.clone(), f125(match v0.clone() {
                Adt16::C0(_, _, _, _, h1056) => {
                    h1056
                }
            }, v4.clone()))?, f336(match v0.clone() {
                Adt16::C0(_, _, h1057, _, _) => {
                    h1057
                }
            })) {
                let m1061 = f337(f335(match v0.clone() {
                    Adt16::C0(_, _, _, h1058, _) => {
                        h1058
                    }
                }, f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h1059, _) => {
                        h1059
                    }
                }))?, f335(v3.clone(), f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h1060, _) => {
                        h1060
                    }
                }))?);
                match m1061 {
                    None => {
                        let m1064 = f338(f142(match v0 {
                            Adt16::C0(_, h1063, _, _, _) => {
                                h1063
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1064 {
                            None => {
                                let m1065 = f338(v4, List::<Bytes>::nil());
                                match m1065 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f122(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f333(f125(v3, match v0 {
                            Adt16::C0(_, _, _, h1062, _) => {
                                h1062
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f333(match v1 {
                Adt15::C0(_, _, _, _, h1053) => {
                    h1053
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f232(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F339, List::<Bytes>::nil(), v0)
}

pub fn f233(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f340(v0, v1)
}

pub fn f234(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f341(f342(f343(v0))?))
}

pub fn f235(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f236(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1066 = f57(v0, v2)?;
    match m1066 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f344(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1067 = v4.clone();
            match m1067.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1069 = f92(f93({
                        let m1068 = f93(v3.clone(), f8(Str::lit("source")));
                        match m1068 {
                            None => {
                                Adt2::C9
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1069 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f345(f346(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f237(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1070 = v0;
    match m1070.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f347(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1074 = f44(f46(v1.clone(), 6u64, {
                    let a1072 = {
                        let a1071 = v1;
                        length_bytes(a1071)
                    };
                    let a1073 = 6u64;
                    nat_sub(a1072, a1073)
                }), 34u8)?;
                match m1074.uncons() {
                    None => {
                        f237(v2)
                    }
                    Some((v3, v4)) => {
                        let m1075 = v4;
                        match m1075.uncons() {
                            None => {
                                f237(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f237(v2)?))
                            }
                        }
                    }
                }
            } else {
                f237(v2)
            }
        }
    }
}

pub fn f238(p1078: List<Bytes>, p1079: List<Bytes>) -> List<Bytes> {
    let m1077 = (p1078, p1079);
    {
        let mut m1077 = m1077;
        loop {
            let step = {
                let (v0, v1) = m1077;
                let m1076 = v0;
                match m1076.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1077 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f240(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1080 = v1;
    match m1080.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f348(v0.clone(), v3)?;
            if f52(f349(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f350(v5)?, f240(v0, v4, v2)?))
            } else {
                f240(v0, v4, v2)
            }
        }
    }
}

pub fn f241(v0: Adt1, p1084: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1083 = p1084;
    {
        let mut m1083 = m1083;
        loop {
            let step = {
                let v1 = m1083;
                let m1081 = v1;
                match m1081.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f348(v0.clone(), v3.clone())?;
                        if if f52(f349(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f100({
                                let m1082 = f93(v5.clone(), f8(Str::lit("schema")));
                                match m1082 {
                                    None => {
                                        Adt2::C9
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f350(v5)?))
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
                Ok(next) => { m1083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h1085, _, _) => {
            h1085
        }
    }, f264())) {
        Ok(Adt12::C1(List::cons(f264(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f351(f147(match v0.clone() {
            Adt1::C0(_, h1086, _, _) => {
                h1086
            }
        }, f264())))?, v1.clone());
        let m1087 = f56(v0.clone(), List::cons(f153(), List::cons(f208(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1087.uncons() {
            None => {
                let m1088 = f56(v0.clone(), f352(v0.clone(), v2.clone())?);
                match m1088.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f353(v0.clone(), v2, v1)?;
                        let (h1089, _) = v7;
                        if h1089 {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h1090) = v7;
                                h1090
                            }))
                        } else {
                            let m1091 = f124(v0.clone());
                            match m1091.uncons() {
                                None => {
                                    let m1092 = f126(v0.clone())?;
                                    match m1092.uncons() {
                                        None => {
                                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h1093) = v7;
                                                h1093
                                            }, List::cons(f7(f145(f354(f127(v0.clone())?, f135(v0.clone(), f136(v0.clone())?)?))?), List::cons(f355(v0)?, List::<Bytes>::nil()))))))
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

pub fn f243(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f356(f96(v0, f153())?)?)))
}

pub fn f244() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The one way an agent changes this repository: dispatch this workflow with an\u{a}")), List::cons(f8(Str::lit("# operation of the API. It runs the certified command, and opens a pull request\u{a}")), List::cons(f8(Str::lit("# as the Verdant Studio GitHub App, which merges it once the studio check passes.\u{a}")), List::cons(f8(Str::lit("name: operate\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}")), List::cons(f8(Str::lit("# that defers to an open advance pull request always has a successor when that merges.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("    inputs:\u{a}")), List::cons(f8(Str::lit("      args:\u{a}")), List::cons(f8(Str::lit("        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}")), List::cons(f8(Str::lit("        required: true\u{a}")), List::cons(f8(Str::lit("        default: 'advance'\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input:\u{a}")), List::cons(f8(Str::lit("        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input_name:\u{a}")), List::cons(f8(Str::lit("        description: 'File name for input'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("        default: 'input'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Only operations that write the registers share one group. GitHub keeps one\u{a}")), List::cons(f8(Str::lit("# pending run per group, so a staging or propose-command run gets its own.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  operate:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 30\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      # Main moves about every 10 minutes, and refresh's regeneration of an API change plus its check takes\u{a}")), List::cons(f8(Str::lit("      # about 12, so an operation merged in that window pushes the change behind main again (#309, #315).\u{a}")), List::cons(f8(Str::lit("      # refresh keeps a regenerated change ready with its auto-merge set, so this run waits for every such\u{a}")), List::cons(f8(Str::lit("      # change whose studio check has not failed to merge first, then starts from the new main. It waits at\u{a}")), List::cons(f8(Str::lit("      # most 10 minutes, so neither queue stalls. A new propose-command waits too, so it never merges ahead of an\u{a}")), List::cons(f8(Str::lit("      # older change that is being regenerated; revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("      - name: wait for the API changes that are next to merge (every operation but revert-merge)\u{a}")), List::cons(f8(Str::lit("        if: ${{ !startsWith(inputs.args, 'revert-merge ') }}\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          for try in $(seq 1 20); do\u{a}")), List::cons(f8(Str::lit("            next=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio propose-command\")) and .auto_merge != null and (.draft | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then next=\"$next #${pr}\"; fi\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f245() -> Bytes {
    f7(List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$next\" ]; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("            echo \"waiting for${next} to merge (check $try of 20)\"\u{a}")), List::cons(f8(Str::lit("            sleep 30\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          echo \"::warning::${next# } still open after 10 minutes, so this run goes on\"\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}")), List::cons(f8(Str::lit("        if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          mkdir -p target/input\u{a}")), List::cons(f8(Str::lit("          cat > target/input/change.patch <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}")), List::cons(f8(Str::lit("          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^admit/' -e '^lexlean-workspace.toml$' || true)\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers, admit and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          # readers/lexlean.lock, readers/xml/lexlean.lock and admit/lexlean.lock are generated, and the export below writes them again.\u{a}")), List::cons(f8(Str::lit("          git apply --exclude=readers/lexlean.lock --exclude=readers/xml/lexlean.lock --exclude=admit/lexlean.lock target/input/change.patch\u{a}")), List::cons(f8(Str::lit("          # The owner reads this pull request, so its body leads with the proposer's own account (the prose\u{a}")), List::cons(f8(Str::lit("          # before the first diff), then the definitions it changes. Checked before the export, which takes minutes.\u{a}")), List::cons(f8(Str::lit("          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}")), List::cons(f8(Str::lit("          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The export refuses Lean output holding an absolute path (LLV7006) only at its end, so refuse an added\u{a}")), List::cons(f8(Str::lit("          # string literal with a slash at a token boundary before a name character here, in seconds.\u{a}")), List::cons(f8(Str::lit("          if grep '^+[^+]' target/input/change.patch | grep '\"' | grep -E \"(^|[[:space:]\\\"'\\`([{<,;=])/[[:alnum:]_.]\" > target/absolute.txt; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::LLV7006: these added lines put a slash at a token boundary, which the export refuses as an absolute path (write >/dev/null with no space):\"\u{a}")), List::cons(f8(Str::lit("            cat target/absolute.txt\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          [ \"$(sha256sum target/lexlean/bin/lexlean 2>/dev/null | cut -c1-64)\" = \"$(jq -r .lexlean.executable_sha256 certified/evidence/attestation.json)\" ] || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked --version 0.4.3 lexlean --root target/lexlean; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          # The one export, shared with refresh and run end to end by the studio check.\u{a}")), List::cons(f8(Str::lit("          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f357(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("            tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}")), List::cons(f8(Str::lit("          {\u{a}")), List::cons(f8(Str::lit("            cat target/summary.md\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"### Definitions this adds or changes\"\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f246() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            {\u{a}")), List::cons(f8(Str::lit("              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}")), List::cons(f8(Str::lit("              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}")), List::cons(f8(Str::lit("            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            slash=/\u{a}")), List::cons(f8(Str::lit("            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            head -n 8 target/export.log.phases || true\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"<${slash}details>\"\u{a}")), List::cons(f8(Str::lit("          } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("      - id: command\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: |\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_readers\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_xml\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_admit\u{a}")), List::cons(f8(Str::lit("          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}")), List::cons(f8(Str::lit("        if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          [ \"$(sha256sum target/lexlean/bin/lexlean 2>/dev/null | cut -c1-64)\" = \"$(jq -r .lexlean.executable_sha256 certified/evidence/attestation.json)\" ] || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked --version 0.4.3 lexlean --root target/lexlean; }\u{a}")), List::cons(f8(Str::lit("          target/lexlean/bin/lexlean production verify-artifact certified\u{a}")), List::cons(f8(Str::lit("          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}")), List::cons(f8(Str::lit("          if [ -d certified-readers ]; then\u{a}")), List::cons(f8(Str::lit("            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}")), List::cons(f8(Str::lit("            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}")), List::cons(f8(Str::lit("      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("          INPUT_NAME: ${{ inputs.input_name }}\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail -f\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          # read-* operations run in their reader project's command when it is built (the XML ones in verdant_xml), the admission operations in verdant_admit, every other in the studio command.\u{a}")), List::cons(f8(Str::lit("          vs() { case \"$1\" in admit|admit-encoding|admit-registry|admit-translated|propose-encoding) if [ -x target/release/verdant_admit ]; then target/release/verdant_admit \"$@\"; return; fi;; read-xml|read-xsd|read-xsd-components) if [ -x target/release/verdant_xml ]; then target/release/verdant_xml \"$@\"; return; fi;; read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f247() -> Bytes {
    f7(List::cons(f8(Str::lit("          # An App pull request whose files already hold on main has nothing left to merge,\u{a}")), List::cons(f8(Str::lit("          # so it is closed; the propose-command ones change the API and stay open until they merge.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}")), List::cons(f8(Str::lit("              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}")), List::cons(f8(Str::lit("              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}")), List::cons(f8(Str::lit("              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}")), List::cons(f8(Str::lit("                exit 0\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            # A change to a reader re-derives the encodings it printed before the studio check replays them.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then\u{a}")), List::cons(f8(Str::lit("              readmitted=$(bash .github/scripts/readmit.sh target/release/verdant_readers; if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi)\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$readmitted\" ]; then\u{a}")), List::cons(f8(Str::lit("                { echo; echo \"### Encodings admitted again\"; echo; echo \"This change alters what a reader prints for these admitted encodings, so each was admitted again through read-admit with the command and citation its row records:\"; echo; sed 's/^/- /' <<< \"$readmitted\"; } >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            cat target/operation.txt\u{a}")), List::cons(f8(Str::lit("          elif [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            : > target/operation.txt\u{a}")), List::cons(f8(Str::lit("            last=''\u{a}")), List::cons(f8(Str::lit("            skipped=''\u{a}")), List::cons(f8(Str::lit("            # next prints one operation it can dispatch, or else the lines that say why the\u{a}")), List::cons(f8(Str::lit("            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}")), List::cons(f8(Str::lit("            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}")), List::cons(f8(Str::lit("              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("              if vs $op > target/attempt.txt 2>&1; then\u{a}")), List::cons(f8(Str::lit("                tee -a target/operation.txt < target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                last=$op\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                cat target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                binding=${op#* }\u{a}")), List::cons(f8(Str::lit("                echo \"::warning::$op was refused; the other bindings continue\"\u{a}")), List::cons(f8(Str::lit("                echo \"refused: $op\" >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}")), List::cons(f8(Str::lit("                skipped=\"$skipped $binding\"\u{a}")), List::cons(f8(Str::lit("                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            t0=$EPOCHREALTIME\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}")), List::cons(f8(Str::lit("            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}")), List::cons(f8(Str::lit("            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}")), List::cons(f8(Str::lit("              echo \"waiting merge: #${pr} changes the API, and merges once the studio check passes\" >> target/waiting.txt\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}")), List::cons(f8(Str::lit("          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f248() -> Bytes {
    f7(List::cons(f8(Str::lit("            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}")), List::cons(f8(Str::lit("            # good main commit, it reverts the merge on main as the App, and merges once the studio check passes.\u{a}")), List::cons(f8(Str::lit("            merge=${ARGS#revert-merge }\u{a}")), List::cons(f8(Str::lit("            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B revert origin/main\u{a}")), List::cons(f8(Str::lit("            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("            git revert -m 1 --no-commit \"$merge\"\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::cons(f8(Str::lit("            vs $ARGS | tee target/operation.txt\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("      - name: open a pull request as the Verdant Studio app\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          git add -A\u{a}")), List::cons(f8(Str::lit("          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}")), List::cons(f8(Str::lit("          verdant_studio $ARGS\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("          $(cat target/operation.txt)\u{a}")), List::cons(f8(Str::lit("          MSG\u{a}")), List::cons(f8(Str::lit("          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}")), List::cons(f8(Str::lit("          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}")), List::cons(f8(Str::lit("            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          title=\"verdant_studio $ARGS\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}")), List::cons(f8(Str::lit("          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}")), List::cons(f8(Str::lit("          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}")), List::cons(f8(Str::lit("          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}")), List::cons(f8(Str::lit("          pr=\"\"\u{a}")), List::cons(f8(Str::lit("          for try in 1 2 3 4; do\u{a}")), List::cons(f8(Str::lit("            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            pr=\"\"\u{a}")), List::cons(f8(Str::lit("            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}")), List::cons(f8(Str::lit("            sleep $((2 ** try))\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          echo \"opened #${pr}\"\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("          # A change to the API itself (propose-command, revert-merge) merges as every App pull request does:\u{a}")), List::cons(f8(Str::lit("          # the owner automated these approvals (2026-11-26), so the studio check, which runs the API's own\u{a}")), List::cons(f8(Str::lit("          # end-to-end propose run, is its gate. refresh keeps its auto-merge while it regenerates it.\u{a}")), List::cons(f8(Str::lit("          if [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}")), List::cons(f8(Str::lit("            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f249() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::<Bytes>::nil())))
}

pub fn f250() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# When main moves, every open propose-command pull request that no longer merges\u{a}")), List::cons(f8(Str::lit("# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}")), List::cons(f8(Str::lit("# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}")), List::cons(f8(Str::lit("# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}")), List::cons(f8(Str::lit("# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}")), List::cons(f8(Str::lit("# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("# The exception is one whose auto-merge is set, the next change to merge: it stays ready and keeps its\u{a}")), List::cons(f8(Str::lit("# auto-merge, so it merges as soon as its regeneration passes the studio check. It is\u{a}")), List::cons(f8(Str::lit("# regenerated first, and only the oldest such one at a time, since two changes to the\u{a}")), List::cons(f8(Str::lit("# API conflict on their export (#309 and #315 were pushed behind main this way).\u{a}")), List::cons(f8(Str::lit("name: refresh\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}")), List::cons(f8(Str::lit("# operate run completes, which needs no token that may dispatch workflows, so every\u{a}")), List::cons(f8(Str::lit("# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_run:\u{a}")), List::cons(f8(Str::lit("    workflows: [operate]\u{a}")), List::cons(f8(Str::lit("    types: [completed]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A running refresh always finishes, because an export takes minutes and main moves more\u{a}")), List::cons(f8(Str::lit("# often than that; GitHub keeps only the newest pending run behind it.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: refresh\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  find:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 15\u{a}")), List::cons(f8(Str::lit("    outputs:\u{a}")), List::cons(f8(Str::lit("      stale: ${{ steps.stale.outputs.stale }}\u{a}")), List::cons(f8(Str::lit("      matrix: ${{ steps.stale.outputs.matrix }}\u{a}")), List::cons(f8(Str::lit("      auto: ${{ steps.stale.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - id: stale\u{a}")), List::cons(f8(Str::lit("        name: find the propose-command pull requests that conflict with main\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          stale=''\u{a}")), List::cons(f8(Str::lit("          auto=''\u{a}")), List::cons(f8(Str::lit("          # Oldest first, so the oldest one whose auto-merge is set is the one regenerated now.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}")), List::cons(f8(Str::lit("            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}")), List::cons(f8(Str::lit("            # which a cancelled run may have left behind.\u{a}")), List::cons(f8(Str::lit("            mergeable=null\u{a}")), List::cons(f8(Str::lit("            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}")), List::cons(f8(Str::lit("              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f251() -> Bytes {
    f7(List::cons(f8(Str::lit("              if [ \"$mergeable\" != null ]; then break; fi\u{a}")), List::cons(f8(Str::lit("              sleep 10\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = false ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              # Drafting would drop its auto-merge, so it stays ready. A later one waits: it is\u{a}")), List::cons(f8(Str::lit("              # regenerated once this one merges, when it would conflict again anyway.\u{a}")), List::cons(f8(Str::lit("              if [ -z \"$auto\" ]; then auto=$pr; else echo \"#${pr} waits for #${auto} to merge\"; fi\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = false ]; then\u{a}")), List::cons(f8(Str::lit("              stale=\"$stale $pr\"\u{a}")), List::cons(f8(Str::lit("              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}")), List::cons(f8(Str::lit("              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$auto\" ]; then stale=\" $auto$stale\"; fi\u{a}")), List::cons(f8(Str::lit("          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"auto=$auto\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("  regenerate:\u{a}")), List::cons(f8(Str::lit("    needs: find\u{a}")), List::cons(f8(Str::lit("    if: needs.find.outputs.stale != ''\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 60\u{a}")), List::cons(f8(Str::lit("    strategy:\u{a}")), List::cons(f8(Str::lit("      fail-fast: false\u{a}")), List::cons(f8(Str::lit("      matrix:\u{a}")), List::cons(f8(Str::lit("        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          fetch-depth: 0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-0.4.3-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          STALE: ${{ matrix.pr }}\u{a}")), List::cons(f8(Str::lit("          AUTO: ${{ needs.find.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A regeneration of the one whose auto-merge is set that fails leaves it a draft, which drops\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f252() -> Bytes {
    f7(List::cons(f8(Str::lit("          # its auto-merge, so it is no longer the next change to merge and nothing is handed over red.\u{a}")), List::cons(f8(Str::lit("          trap 'rc=$?; if [ \"$rc\" != 0 ] && [ \"$STALE\" = \"$AUTO\" ]; then gh pr ready --undo \"$STALE\" || true; fi' EXIT\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            mkdir -p target\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          [ \"$(sha256sum target/lexlean/bin/lexlean 2>/dev/null | cut -c1-64)\" = \"$(jq -r .lexlean.executable_sha256 certified/evidence/attestation.json)\" ] || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked --version 0.4.3 lexlean --root target/lexlean; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          failed=''\u{a}")), List::cons(f8(Str::lit("          for pr in $STALE; do\u{a}")), List::cons(f8(Str::lit("            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}")), List::cons(f8(Str::lit("            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}")), List::cons(f8(Str::lit("            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("            # The one whose auto-merge is set stays ready, so that it keeps its auto-merge.\u{a}")), List::cons(f8(Str::lit("            if [ \"$pr\" != \"$AUTO\" ]; then gh pr ready --undo \"$pr\" || true; fi\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin \"$branch\" main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B refresh origin/main\u{a}")), List::cons(f8(Str::lit("            git clean -fdq\u{a}")), List::cons(f8(Str::lit("            # readers/lexlean.lock and readers/xml/lexlean.lock are generated: the export below writes them again, so they are left out of the\u{a}")), List::cons(f8(Str::lit("            # change, and two reader pull requests never conflict on it.\u{a}")), List::cons(f8(Str::lit("            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml ':!readers/lexlean.lock' ':!readers/xml/lexlean.lock' ':!admit/lexlean.lock' > target/change.patch\u{a}")), List::cons(f8(Str::lit("            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}")), List::cons(f8(Str::lit("            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}")), List::cons(f8(Str::lit("            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}")), List::cons(f8(Str::lit("            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}")), List::cons(f8(Str::lit("              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}")), List::cons(f8(Str::lit("                base=$(git merge-base origin/main \"origin/$branch\")\u{a}")), List::cons(f8(Str::lit("                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}")), List::cons(f8(Str::lit("                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}")), List::cons(f8(Str::lit("                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}")), List::cons(f8(Str::lit("                git add model/strata.json\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}")), List::cons(f8(Str::lit("                git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("                failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}")), List::cons(f8(Str::lit("                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("                continue\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f357(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("              tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("              git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("              failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("              continue\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            rm -rf studio/.lexlean/production readers/.lexlean/production readers/xml/.lexlean/production admit/.lexlean/production\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            # The regenerated change re-derives the encodings whose reader output it changes, as propose-command does.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then bash .github/scripts/readmit.sh target/release/verdant_readers; fi\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("            git add -A\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f253() -> Bytes {
    f7(List::cons(f8(Str::lit("            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml; } > target/message.txt\u{a}")), List::cons(f8(Str::lit("            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}")), List::cons(f8(Str::lit("            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}")), List::cons(f8(Str::lit("            gh pr ready \"$pr\" || true\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::regenerated #${pr} on the new main\"\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))))))
}

pub fn f254(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1094 = v0;
    match m1094.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f255(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt19 = f358(v0.clone(), 0u64, {
        let a1095 = v0;
        length_bytes(a1095)
    }, Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt19 = {
        let a1098 = {
            let a1097 = match v1.clone() {
                Adt19::C0(_, _, h1096, _, _) => {
                    h1096
                }
            };
            length_bytes(a1097)
        };
        let a1099 = 0u64;
        if if nat_eq(a1098, a1099) {
            let m1101 = match v1.clone() {
                Adt19::C0(_, _, _, h1100, _) => {
                    h1100
                }
            };
            match m1101.uncons() {
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
            f359(v1)
        }
    };
    Ok(f360(match v4 {
        Adt19::C0(_, _, _, _, h1102) => {
            h1102
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f256(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt17>, v3: List<Adt2>, v4: List<Bytes>) -> List<Bytes> {
    let m1103 = v1;
    match m1103.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f125(f361(v5.clone(), v2.clone()), f362(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f363(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1105 = {
                    let a1104 = v7.clone();
                    length_list(a1104)
                };
                let a1106 = 0u64;
                if nat_eq(a1105, a1106) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f122(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f256(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f257(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1107 = v0;
    match m1107.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1108 = v1;
            match m1108.uncons() {
                None => {
                    f257(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f257(v2))
                }
            }
        }
    }
}

pub fn f258(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1109 = v1;
    match m1109.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f216(v2.clone(), f8(Str::lit("domain"))) {
                f364(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1110 = f93(v2.clone(), f8(Str::lit("role")));
                    match m1110 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f365(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1111 = f93(v2.clone(), f8(Str::lit("origin")));
                    match m1111 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1112 = f93(v2, f8(Str::lit("evidence")));
                    match m1112 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f258(v0, v3))
            } else {
                f258(v0, v3)
            }
        }
    }
}

pub fn f259(v0: List<Adt2>) -> List<Bytes> {
    let m1113 = v0;
    match m1113.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f259(v2))
        }
    }
}

pub fn f260(v0: List<Adt2>) -> List<Bytes> {
    let m1114 = v0;
    match m1114.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f366(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f260(v2))
        }
    }
}

pub fn f261(p1120: List<Bytes>) -> R<bool> {
    let m1119 = p1120;
    {
        let mut m1119 = m1119;
        loop {
            let step = {
                let v0 = m1119;
                let m1115 = v0;
                match m1115.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1117 = {
                            let a1116 = f193(v1)?;
                            length_bytes(a1116)
                        };
                        let a1118 = 0u64;
                        if nat_eq(a1117, a1118) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f262(v0: List<Bytes>) -> R<bool> {
    if f271(f284(v0.clone())?) {
        Ok(f51({
            let a1122 = {
                let a1121 = f273(f284(v0)?);
                length_list(a1121)
            };
            let a1123 = 0u64;
            nat_eq(a1122, a1123)
        }))
    } else {
        Ok(false)
    }
}

pub fn f263(v0: List<Bytes>) -> R<bool> {
    if f271(f367(v0.clone(), f8(Str::lit("requirement")))?) {
        if f271(f367(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1125 = {
                    let a1124 = f219(f273(f367(v0.clone(), f8(Str::lit("requirement")))?), f273(f367(v0, f8(Str::lit("admission")))?));
                    length_list(a1124)
                };
                let a1126 = 0u64;
                nat_eq(a1125, a1126)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f264() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f265(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f266() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f267(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1127 = v0;
    match m1127.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f193(v1)?;
            let a1129 = {
                let a1128 = v3.clone();
                length_bytes(a1128)
            };
            let a1130 = 0u64;
            if nat_eq(a1129, a1130) {
                f267(v2)
            } else {
                let m1131 = f43(v3.clone())?;
                match m1131 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f368(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1132 = f267(v2)?;
                            match m1132 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f369(v5), {
                                        let a1133 = v3;
                                        let a1134 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1133, a1134)
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

pub fn f268() -> Bytes {
    let a1135 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1135)
}

pub fn f269(p1140: List<Bytes>) -> R<Option<Bytes>> {
    let m1139 = p1140;
    {
        let mut m1139 = m1139;
        loop {
            let step = {
                let v0 = m1139;
                let m1136 = v0;
                match m1136.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1137 = f43(f193(v1)?)?;
                        match m1137 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1138 = f93(v3.clone(), f8(Str::lit("source")));
                                match m1138 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f370(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes) -> Bytes {
    let a1145 = {
        let a1141 = Str::lit("binding ");
        utf8_encode(a1141)
    };
    let a1146 = {
        let a1143 = v0;
        let a1144 = {
            let a1142 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1142)
        };
        append_bytes(a1143, a1144)
    };
    append_bytes(a1145, a1146)
}

pub fn f271(v0: Option<List<Adt2>>) -> bool {
    let m1147 = v0;
    match m1147 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f272(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1148 = v0;
    match m1148.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f193(v1)?;
            let a1150 = {
                let a1149 = v3.clone();
                length_bytes(a1149)
            };
            let a1151 = 0u64;
            if nat_eq(a1150, a1151) {
                f272(v2)
            } else {
                let m1152 = f43(v3)?;
                match m1152 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if f117(f117(f216(v4.clone(), f8(Str::lit("authority"))), f216(v4.clone(), f8(Str::lit("row")))), f117(f117(f216(v4.clone(), f8(Str::lit("construct"))), f216(v4.clone(), f8(Str::lit("artifact")))), f371(f58(v4.clone(), f8(Str::lit("reader"))), f58(v4.clone(), f8(Str::lit("construct")))))) {
                            let m1153 = f272(v2)?;
                            match m1153 {
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

pub fn f273(v0: Option<List<Adt2>>) -> List<Adt2> {
    let m1154 = v0;
    match m1154 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f274(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1155 = v0;
    match m1155.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f193(v1)?;
            let a1157 = {
                let a1156 = v3.clone();
                length_bytes(a1156)
            };
            let a1158 = 0u64;
            if nat_eq(a1157, a1158) {
                f274(v2)
            } else {
                let m1159 = f43(v3.clone())?;
                match m1159 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f372(v4.clone()) {
                            let m1160 = f274(v2)?;
                            match m1160 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f239(), List::cons(f8(Str::lit("/")), List::cons(f373(f58(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1161 = v3;
                                        let a1162 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1161, a1162)
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

pub fn f275() -> Bytes {
    f374()
}

pub fn f276(v0: Adt1, v1: Adt2, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f281(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f375(v1, v4)?)?), List::cons(f282(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f277(v0: Bytes) -> R<Bytes> {
    Ok(f376(f377(v0)?))
}

pub fn f278(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1163 = v0;
    match m1163.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f279() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f280(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f345(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f281(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f378(f150(f95(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f379(f380(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f282(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f100(Adt2::C7(std::rc::Rc::new(f381(f145(f125(v1, f210(f96(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f283(p1166: List<Adt2>) -> bool {
    let m1165 = p1166;
    {
        let mut m1165 = m1165;
        loop {
            let step = {
                let v0 = m1165;
                let m1164 = v0;
                match m1164.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f142(f382(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1167 = v0;
    match m1167.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f193(v1)?;
            let a1169 = {
                let a1168 = v3.clone();
                length_bytes(a1168)
            };
            let a1170 = 0u64;
            if nat_eq(a1169, a1170) {
                f284(v2)
            } else {
                let m1171 = f43(v3)?;
                match m1171 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if if f216(v4.clone(), f8(Str::lit("agent"))) {
                            if f216(v4.clone(), f8(Str::lit("at"))) {
                                if f216(v4.clone(), f8(Str::lit("quote"))) {
                                    f383(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1172 = f284(v2)?;
                            match m1172 {
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

pub fn f285(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f100(Adt2::C7(std::rc::Rc::new(f101(v1))))?))
}

pub fn f286() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f287(p1175: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m1174 = p1175;
    {
        let mut m1174 = m1174;
        loop {
            let step = {
                let v0 = m1174;
                let m1173 = v0;
                match m1173.uncons() {
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
                Ok(next) => { m1174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt2::C6(v0)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))))))))
}

pub fn f289(v0: u64) -> bool {
    let a1176 = v0;
    let a1177 = 127u64;
    nat_le(a1176, a1177)
}

pub fn f290(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1178, _) = v1.clone();
        h1178
    };
    let v3: u64 = {
        let (h1179, _) = {
            let (_, h1180) = v1.clone();
            h1180
        };
        h1179
    };
    let v4: List<Bytes> = {
        let (_, h1181) = {
            let (_, h1182) = v1;
            h1182
        };
        h1181
    };
    let m1183 = f203(v0.clone(), v2);
    match m1183 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1184 = v5;
            let a1185 = 10u8;
            if equal(a1184, a1185) {
                Ok(Some(({
                    let a1186 = v2;
                    let a1187 = 1u64;
                    nat_add(a1186, a1187)?
                }, ({
                    let a1188 = v2;
                    let a1189 = 1u64;
                    nat_add(a1188, a1189)?
                }, List::cons(f384(f46(v0, v3, {
                    let a1190 = v2;
                    let a1191 = v3;
                    nat_sub(a1190, a1191)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1192 = v2;
                    let a1193 = 1u64;
                    nat_add(a1192, a1193)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f291(v0: Adt2) -> List<Adt2> {
    let m1194 = v0;
    match m1194 {
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
        Adt2::C7(r1195) => {
            let v3 = (*r1195).clone();
            f291(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r1196, r1197) => {
            let v1 = (*r1196).clone();
            let v2 = (*r1197).clone();
            List::cons(v1, f291(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f292(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1198 = v0;
    match m1198 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r1199) => {
            let v12 = (*r1199).clone();
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
        Adt2::C7(r1200) => {
            let v10 = (*r1200).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r1201) => {
            let v5 = (*r1201).clone();
            Adt2::C8(std::rc::Rc::new(f292(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1202, r1203) => {
            let v18 = (*r1202).clone();
            let v19 = (*r1203).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r1204, r1205) => {
            let v3 = (*r1204).clone();
            let v4 = (*r1205).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt2::C11(v2, std::rc::Rc::new(f385(v3, v1.clone())), std::rc::Rc::new(f292(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f292(v4, v1)))
            }
        }
    }
}

pub fn f293(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1206 = 0u64;
        let a1207 = v1;
        nat_lt(a1206, a1207)
    } else {
        false
    } {
        let a1208 = Bytes::lit(&[0x2d]);
        let a1209 = f386(v1)?;
        Ok(append_bytes(a1208, a1209))
    } else {
        f386(v1)
    }
}

pub fn f294(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f386(v1)?;
    let v4: u64 = {
        let a1210 = v3.clone();
        length_bytes(a1210)
    };
    let v5: Bytes = {
        let a1211 = v2;
        let a1212 = v4;
        if nat_lt(a1211, a1212) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1213 = v4;
                let a1214 = v2;
                nat_sub(a1213, a1214)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1215 = v4;
                let a1216 = v2;
                nat_sub(a1215, a1216)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f315({
                let a1217 = v2;
                let a1218 = v4;
                nat_sub(a1217, a1218)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1219 = f8(Str::lit("-"));
        let a1220 = v5;
        Ok(append_bytes(a1219, a1220))
    } else {
        Ok(v5)
    }
}

pub fn f295(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f387(v0)?;
    let v3: bool = {
        let a1221 = f388(v0)?;
        let a1222 = v1;
        nat_le(a1221, a1222)
    };
    let v4: u64 = {
        let a1226 = {
            let a1223 = v1;
            let a1224 = v2;
            let a1225 = 0u64;
            nat_quot(a1223, a1224, a1225)
        };
        let a1227 = f389(f390(v0))?;
        let a1228 = 0u64;
        nat_rem(a1226, a1227, a1228)
    };
    let v5: u64 = {
        let a1229 = v1;
        let a1230 = v2;
        let a1231 = 0u64;
        nat_rem(a1229, a1230, a1231)
    };
    let a1232 = v4;
    let a1233 = f391(v0)?;
    if nat_eq(a1232, a1233) {
        let a1234 = v5;
        let a1235 = 0u64;
        if nat_eq(a1234, a1235) {
            Ok(f392(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f392(v3, {
                let a1239 = {
                    let a1236 = v2;
                    let a1237 = 2u64;
                    let a1238 = 0u64;
                    nat_quot(a1236, a1237, a1238)
                };
                let a1240 = v5;
                if nat_le(a1239, a1240) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f117({
            let a1241 = v4;
            let a1242 = 0u64;
            nat_eq(a1241, a1242)
        }, {
            let a1243 = v5;
            let a1244 = 0u64;
            nat_eq(a1243, a1244)
        }) {
            Ok(f392(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1245 = v4;
                let a1246 = 0u64;
                if nat_eq(a1245, a1246) {
                    v5
                } else {
                    let a1247 = v2;
                    let a1248 = v5;
                    nat_add(a1247, a1248)?
                }
            };
            let v7: u64 = {
                let a1249 = v4;
                let a1250 = 0u64;
                if nat_eq(a1249, a1250) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1257 = {
                    let a1253 = {
                        let a1251 = v7;
                        let a1252 = 2000u64;
                        nat_add(a1251, a1252)?
                    };
                    let a1254 = f393(v0);
                    nat_sub(a1253, a1254)
                };
                let a1258 = {
                    let a1255 = f394(v0);
                    let a1256 = 1u64;
                    nat_sub(a1255, a1256)
                };
                nat_sub(a1257, a1258)
            };
            Ok(f392(v3, {
                let a1259 = 2000u64;
                let a1260 = v8;
                if nat_le(a1259, a1260) {
                    f395(v6, true, {
                        let a1261 = v8;
                        let a1262 = 2000u64;
                        nat_sub(a1261, a1262)
                    })?
                } else {
                    f395(v6, false, {
                        let a1263 = 2000u64;
                        let a1264 = v8;
                        nat_sub(a1263, a1264)
                    })?
                }
            }))
        }
    }
}

pub fn f296(v0: Bytes) -> R<Bytes> {
    let a1269 = f195(v0.clone(), 0u64, {
        let a1266 = {
            let a1265 = v0.clone();
            length_bytes(a1265)
        };
        let a1267 = 1u64;
        nat_add(a1266, a1267)?
    })?;
    let a1270 = {
        let a1268 = v0.clone();
        length_bytes(a1268)
    };
    if nat_eq(a1269, a1270) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1276 = {
            let a1274 = Bytes::lit(&[0x22]);
            let a1275 = f396(v0.clone(), 0u64, {
                let a1272 = {
                    let a1271 = v0;
                    length_bytes(a1271)
                };
                let a1273 = 1u64;
                nat_add(a1272, a1273)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1274, a1275)
        };
        let a1277 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1276, a1277))
    }
}

pub fn f297(v0: List<Bytes>) -> Bytes {
    let a1280 = {
        let a1278 = Bytes::lit(&[0x5b]);
        let a1279 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1278, a1279)
    };
    let a1281 = Bytes::lit(&[0x5d]);
    append_bytes(a1280, a1281)
}

pub fn f298(v0: Adt2) -> R<List<Bytes>> {
    let m1282 = v0;
    match m1282 {
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
        Adt2::C10(r1283, r1284) => {
            let v14 = (*r1283).clone();
            let v15 = (*r1284).clone();
            Ok(List::cons(f185(v14)?, f298(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f299(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f397(Fn5::F398, List::<(Bytes, Bytes)>::nil(), f399(v0)?);
    let v2: List<Bytes> = f400(Fn6::F401, List::<Bytes>::nil(), v1)?;
    let a1287 = {
        let a1285 = Bytes::lit(&[0x7b]);
        let a1286 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1285, a1286)
    };
    let a1288 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1287, a1288))
}

pub fn f300(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1289 = v0;
    match m1289 {
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
        Adt2::C11(v16, r1290, r1291) => {
            let v17 = (*r1290).clone();
            let v18 = (*r1291).clone();
            Ok(List::cons((v16, f185(v17)?), f300(v18)?))
        }
    }
}

pub fn f301() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f302(v0: Adt13) -> Bytes {
    let m1292 = v0;
    match m1292 {
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

pub fn f303() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f304(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f402(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f402(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f402(f8(Str::lit("name")), f403(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f402(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f402(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f402(f8(Str::lit("statement")), f404(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f402(f8(Str::lit("media_type")), f405(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f402(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f402(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f402(f8(Str::lit("url")), f188(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f305(v0: Bytes) -> R<u64> {
    let (h1293, _) = f308(Fn3::F406(v0.clone()), {
        let a1294 = v0.clone();
        length_bytes(a1294)
    }, {
        let a1295 = v0;
        length_bytes(a1295)
    })?;
    Ok(h1293)
}

pub fn f306(v0: Bytes, v1: u64) -> R<u64> {
    let (h1296, _) = f308(Fn3::F407(v1, v0), v1, 0u64)?;
    Ok(h1296)
}

pub fn f307(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f115(v0.clone(), {
        let a1297 = v1;
        let a1298 = 1u64;
        nat_add(a1297, a1298)?
    })?;
    let v4: Bytes = {
        let a1299 = v3;
        let a1300 = 117u64;
        if nat_eq(a1299, a1300) {
            f408(v0, v1)?
        } else {
            f409(v3)
        }
    };
    let a1301 = v4.clone();
    let a1302 = Bytes::lit(&[]);
    if equal(a1301, a1302) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1303 = v3;
            let a1304 = 117u64;
            if nat_eq(a1303, a1304) {
                let a1305 = v1;
                let a1306 = 6u64;
                nat_add(a1305, a1306)?
            } else {
                let a1307 = v1;
                let a1308 = 2u64;
                nat_add(a1307, a1308)?
            }
        }, ({
            let a1309 = v2;
            let a1310 = v4;
            append_bytes(a1309, a1310)
        }, 0u64))))
    }
}

pub fn f308(v0: Fn3, p1315: u64, p1316: u64) -> R<(u64, bool)> {
    let m1314 = (p1315, p1316);
    {
        let mut m1314 = m1314;
        loop {
            let step = {
                let (v1, v2) = m1314;
                let m1311 = v1;
                if m1311 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1311 - 1;
                    let m1313 = {
                        let c1312: Fn3 = v0.clone();
                        c1312.apply(v2)?
                    };
                    match m1313 {
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
                Ok(next) => { m1314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f410(v0, v1)
}

pub fn f310(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f411(v0, v1)
}

pub fn f311(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1317 = v0.clone();
        length_bytes(a1317)
    };
    let a1318 = 0u64;
    let a1319 = v1;
    if nat_lt(a1318, a1319) {
        let a1320 = f412(v0.clone())?;
        let a1321 = v1;
        if nat_eq(a1320, a1321) {
            let a1322 = 1u64;
            let a1323 = v1;
            if nat_lt(a1322, a1323) {
                let m1326 = {
                    let a1324 = v0;
                    let a1325 = 0u64;
                    index_bytes(a1324, a1325)
                };
                match m1326 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1327 = v2;
                        let a1328 = 48u8;
                        Ok(!equal(a1327, a1328))
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

pub fn f312(v0: Fn4, p1333: u64, p1334: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1332 = (p1333, p1334);
    {
        let mut m1332 = m1332;
        loop {
            let step = {
                let (v1, v2) = m1332;
                let m1329 = v1;
                if m1329 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1329 - 1;
                    let m1331 = {
                        let c1330: Fn4 = v0.clone();
                        c1330.apply(v2)?
                    };
                    match m1331 {
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
                Ok(next) => { m1332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f413(v0, v1)
}

pub fn f314(v0: Bytes) -> R<u64> {
    let (h1335, _) = f308(Fn3::F414(v0.clone()), {
        let a1337 = {
            let a1336 = v0;
            length_bytes(a1336)
        };
        let a1338 = 1u64;
        nat_add(a1337, a1338)?
    }, 0u64)?;
    Ok(h1335)
}

pub fn f315(v0: u64) -> Bytes {
    let m1339 = v0;
    if m1339 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1339 - 1;
        let a1340 = f8(Str::lit("0"));
        let a1341 = f315(v1);
        append_bytes(a1340, a1341)
    }
}

pub fn f316(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1342 = v0;
    match m1342.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1343 = f93(v1, f8(Str::lit("std")));
            match m1343 {
                None => {
                    f316(v2)
                }
                Some(v3) => {
                    if f216(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f316(v2))
                    } else {
                        f316(v2)
                    }
                }
            }
        }
    }
}

pub fn f317(v0: Adt14, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1344 = v1;
    match m1344.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt14::C0(h1345, _, _) => {
                    h1345
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1346, _) => {
                        h1346
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f317(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt14::C0(h1347, _, _) => {
                        h1347
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1348, _) => {
                            h1348
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f317(v0, v3))
                } else {
                    f317(v0, v3)
                }
            }
        }
    }
}

pub fn f318(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt17>> {
    let m1349 = v0;
    match m1349.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v4, v5)) => {
            if if f415(v4.clone(), v2.clone()) {
                true
            } else {
                f415(v4.clone(), v3.clone())
            } {
                f318(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt17::C0({
                    let (h1350, _) = v4.clone();
                    h1350
                }, {
                    let a1353 = {
                        let a1352 = f145(f416({
                            let (h1351, _) = v4.clone();
                            h1351
                        }, v1.clone()))?;
                        length_list(a1352)
                    };
                    let a1354 = 1u64;
                    if nat_eq(a1353, a1354) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1355) = v4.clone();
                    h1355
                }), f318(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f319(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1356 = v1;
    match m1356.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1357, _) = v2.clone();
                h1357
            }, v0.clone()) {
                List::cons({
                    let (_, h1358) = v2;
                    h1358
                }, f319(v0, v3))
            } else {
                if f52({
                    let (_, h1359) = v2;
                    h1359
                }, v0.clone()) {
                    List::cons({
                        let (h1360, _) = v2;
                        h1360
                    }, f319(v0, v3))
                } else {
                    f319(v0, v3)
                }
            }
        }
    }
}

pub fn f320(v0: Adt14, v1: Adt2, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt14::C0(h1361, _, _) => {
            h1361
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1362, _) => {
                h1362
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt14::C0(_, _, h1363) => {
                    h1363
                }
            }) {
                true
            } else {
                if f216(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1364, _) => {
                            h1364
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f321(v0: Adt2) -> bool {
    if f216(v0.clone(), f8(Str::lit("domain"))) {
        if f216(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f417()) {
                if f418(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f418(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f418(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f418(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f322(v0: Adt2) -> bool {
    if f216(v0.clone(), f8(Str::lit("subject"))) {
        if f216(v0.clone(), f8(Str::lit("object"))) {
            if f216(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f419())
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

pub fn f323(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1365 = f43(v1)?;
    match m1365 {
        None => {
            Ok(f222(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f210(f183(f93(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1366 = f92(f93(v4.clone(), f8(Str::lit("construct"))));
                match m1366 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1367 = v5;
            match m1367.uncons() {
                None => {
                    Ok(f222(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1368 = v9;
                    match m1368.uncons() {
                        None => {
                            Ok(f222(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f222(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f222(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f117(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f222(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1369 = f92(f93(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1369 {
                                            None => {
                                                Ok(f222(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1370 = f93(v4.clone(), f8(Str::lit("reader")));
                                                match m1370 {
                                                    None => {
                                                        let m1371 = f93(v4, f8(Str::lit("value")));
                                                        match m1371 {
                                                            None => {
                                                                Ok(f222(v0, f8(Str::lit("a row names neither the reader of its artifact nor a value"))))
                                                            }
                                                            Some(_) => {
                                                                Ok(List::<Bytes>::nil())
                                                            }
                                                        }
                                                    }
                                                    Some(_) => {
                                                        if f216(v4, f8(Str::lit("artifact"))) {
                                                            Ok(List::<Bytes>::nil())
                                                        } else {
                                                            Ok(f222(v0, f8(Str::lit("a row names its reader but not its artifact"))))
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

pub fn f324(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1372 = v1;
    match m1372.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f420(v3.clone())
            } else {
                f51(f420(v3.clone()))
            } {
                List::cons(f235(v0.clone(), v3), f324(v0, v4, v2))
            } else {
                f324(v0, v4, v2)
            }
        }
    }
}

pub fn f325(p1375: List<Bytes>) -> R<Option<Bytes>> {
    let m1374 = p1375;
    {
        let mut m1374 = m1374;
        loop {
            let step = {
                let v0 = m1374;
                let m1373 = v0;
                match m1373.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f421(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f193(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1376 = v1;
    match m1376.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f95(f64(v0.clone(), v2))?, f326(v0, v3)?))
        }
    }
}

pub fn f327(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1377 = v0.clone();
        length_bytes(a1377)
    };
    let v3: u64 = {
        let a1378 = v1.clone();
        length_bytes(a1378)
    };
    let v4: u64 = {
        let (h1379, _) = f308(Fn3::F422(v1.clone(), v3, v0.clone(), v2), {
            let a1380 = v2;
            let a1381 = 1u64;
            nat_add(a1380, a1381)?
        }, 0u64)?;
        h1379
    };
    if f423(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f328() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f329(v0: Adt15) -> R<Adt18> {
    Ok(Adt18::C0(f424(match v0.clone() {
        Adt15::C0(h1382, _, _, _, _) => {
            h1382
        }
    }, 0u64)?, f424(match v0.clone() {
        Adt15::C0(_, h1383, _, _, _) => {
            h1383
        }
    }, 0u64)?, f424(match v0.clone() {
        Adt15::C0(_, _, h1384, _, _) => {
            h1384
        }
    }, 0u64)?, f424(match v0 {
        Adt15::C0(_, _, _, h1385, _) => {
            h1385
        }
    }, 0u64)?))
}

pub fn f330(v0: Adt15) -> List<Bytes> {
    f125(match v0.clone() {
        Adt15::C0(h1386, _, _, _, _) => {
            h1386
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, h1387, _, _) => {
            h1387
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, _, h1388, _) => {
            h1388
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1389) => {
            h1389
        }
    })))
}

pub fn f331(v0: Adt18, p1395: List<u64>, p1396: List<Adt20>) -> Option<u64> {
    let m1394 = (p1395, p1396);
    {
        let mut m1394 = m1394;
        loop {
            let step = {
                let (v1, v2) = m1394;
                let m1390 = v2;
                match m1390.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f425(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1391) => {
                                h1391
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1392, _) => {
                                    h1392
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1393, _) => {
                                    h1393
                                }
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

pub fn f332(v0: Adt15, v1: List<Adt2>, v2: u64) -> R<List<Adt20>> {
    let m1397 = v1;
    match m1397.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f426(v0.clone(), v3)?), f332(v0, v4, {
                let a1398 = v2;
                let a1399 = 1u64;
                nat_add(a1398, a1399)?
            })?))
        }
    }
}

pub fn f333(p1403: List<Bytes>, p1404: u64) -> Bytes {
    let m1402 = (p1403, p1404);
    {
        let mut m1402 = m1402;
        loop {
            let step = {
                let (v0, v1) = m1402;
                let m1400 = v0;
                match m1400.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1401 = v1;
                        if m1401 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1401 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
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

pub fn f334(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f427(v0.clone(), v1.clone()) {
        if f427(v1, v0) {
            f428(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f335(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1405 = v0;
    match m1405.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f429(v2, v1.clone())?, f335(v3, v1)?))
        }
    }
}

pub fn f336(v0: List<Adt2>) -> List<u64> {
    let m1406 = v0;
    match m1406.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f430(v1), f336(v2))
        }
    }
}

pub fn f337(p1409: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1408 = p1409;
    {
        let mut m1408 = m1408;
        loop {
            let step = {
                let v0 = m1408;
                let m1407 = v0;
                match m1407.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f431(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f338(p1412: List<Bytes>, p1413: List<Bytes>) -> Option<Bytes> {
    let m1411 = (p1412, p1413);
    {
        let mut m1411 = m1411;
        loop {
            let step = {
                let (v0, v1) = m1411;
                let m1410 = v0;
                match m1410.uncons() {
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
                Ok(next) => { m1411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f340(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1414 = v0.clone();
    match m1414.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1415 = v2;
            let a1416 = v1.clone();
            if equal(a1415, a1416) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f341(v0: List<(Bytes, ())>) -> List<Bytes> {
    f232(f432(Fn7::F433, List::<Bytes>::nil(), v0))
}

pub fn f342(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f434(f435(Fn8::F436, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1421 = {
        let (h1417, _) = f437(Fn9::F438, {
            let a1419 = {
                let a1418 = v0;
                length_list(a1418)
            };
            let a1420 = 1u64;
            nat_add(a1419, a1420)?
        }, v1)?;
        h1417
    };
    match m1421.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f343(v0: List<Bytes>) -> List<(Bytes, ())> {
    f439(f440(Fn10::F441, List::<(Bytes, ())>::nil(), v0))
}

pub fn f344(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1422 = f44(v1, 34u8)?;
    match m1422.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1423 = v3;
            match m1423.uncons() {
                None => {
                    Ok(f442(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f345(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f443(v0.clone(), 0u64, {
        let a1424 = v0;
        length_bytes(a1424)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f346(v0: Adt2, v1: Bytes) -> Bytes {
    if f32(f444(f93(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f347(v0: Bytes, v1: Bytes, p1437: u64, p1438: u64) -> R<bool> {
    let m1436 = (p1437, p1438);
    {
        let mut m1436 = m1436;
        loop {
            let step = {
                let (v2, v3) = m1436;
                let m1425 = v3;
                if m1425 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1425 - 1;
                    let m1428 = {
                        let a1426 = v1.clone();
                        let a1427 = v2;
                        index_bytes(a1426, a1427)
                    };
                    match m1428 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1431 = {
                                let a1429 = v0.clone();
                                let a1430 = v2;
                                index_bytes(a1429, a1430)
                            };
                            match m1431 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1432 = v5;
                                    let a1433 = v6;
                                    if equal(a1432, a1433) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1434 = v2;
                                            let a1435 = 1u64;
                                            nat_add(a1434, a1435)?
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
                Ok(next) => { m1436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f348(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1439 = f91(v0, v1)?;
    match m1439 {
        None => {
            Ok(Adt2::C9)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f349(v0: Bytes) -> R<Bytes> {
    let a1440 = f8(Str::lit("Catalog."));
    let a1441 = f373(v0)?;
    Ok(append_bytes(a1440, a1441))
}

pub fn f350(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f373(f58(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f351(v0: List<Bytes>) -> List<Bytes> {
    let m1442 = v0;
    match m1442.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f235(f264(), v1), f351(v2))
            } else {
                f351(v2)
            }
        }
    }
}

pub fn f352(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1443 = v1;
    match m1443.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f445(f446(f95(f64(v0.clone(), v2))?)?), f352(v0, v3)?))
        }
    }
}

pub fn f353(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1444 = v1;
    match m1444.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f447(v0.clone(), v3, v2.clone())?;
            let (h1445, _) = v5.clone();
            if h1445 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f353(v0, v4, v2)?;
                let (h1446, _) = v6.clone();
                if h1446 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1449 = {
                            let (_, h1447) = v5;
                            h1447
                        };
                        let a1450 = {
                            let (_, h1448) = v6;
                            h1448
                        };
                        append_bytes(a1449, a1450)
                    }))
                }
            }
        }
    }
}

pub fn f354(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1451 = v1;
    match m1451.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f216(v2.clone(), f8(Str::lit("subject"))) {
                if f217(v0.clone(), v2.clone()) {
                    f354(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f354(v0, v3))
                }
            } else {
                if f364(v0.clone(), v2.clone()) {
                    f354(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f365(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f354(v0, v3))
                }
            }
        }
    }
}

pub fn f355(v0: Adt1) -> R<Bytes> {
    let a1453 = {
        let a1452 = f128(f129(f257(f254(f255(f64(v0.clone(), f208()))?)), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(f96(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1452)
    };
    let a1454 = 0u64;
    if nat_eq(a1453, a1454) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f356(v0: List<Adt2>) -> R<List<Bytes>> {
    let m1455 = v0;
    match m1455.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f210(f183(f93(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f448(f95(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f356(v2)?))
        }
    }
}

pub fn f357() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f358(v0: Bytes, p1513: u64, p1514: u64, p1515: Adt19) -> R<Adt19> {
    let m1512 = (p1513, (p1514, p1515));
    {
        let mut m1512 = m1512;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1512;
                let m1456 = v2;
                if m1456 == 0 {
                    Err::<(u64, (u64, Adt19)), R<Adt19>>(Ok(v3))
                } else {
                    let v4 = m1456 - 1;
                    let v5: u64 = f449(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt19::C0(_, h1457, _, _, _) => {
                            h1457
                        }
                    } {
                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                            let a1458 = v1;
                            let a1459 = 1u64;
                            nat_add(a1458, a1459)?
                        }, (v4, Adt19::C0(true, false, match v3.clone() {
                            Adt19::C0(_, _, h1460, _, _) => {
                                h1460
                            }
                        }, match v3.clone() {
                            Adt19::C0(_, _, _, h1461, _) => {
                                h1461
                            }
                        }, match v3 {
                            Adt19::C0(_, _, _, _, h1462) => {
                                h1462
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt19::C0(h1463, _, _, _, _) => {
                                h1463
                            }
                        } {
                            let a1464 = v5;
                            let a1465 = 34u64;
                            if nat_eq(a1464, a1465) {
                                let a1468 = f449(v0.clone(), {
                                    let a1466 = v1;
                                    let a1467 = 1u64;
                                    nat_add(a1466, a1467)?
                                })?;
                                let a1469 = 34u64;
                                if nat_eq(a1468, a1469) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1470 = v1;
                                        let a1471 = 1u64;
                                        nat_add(a1470, a1471)?
                                    }, (v4, Adt19::C0(true, true, f450(match v3.clone() {
                                        Adt19::C0(_, _, h1472, _, _) => {
                                            h1472
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt19::C0(_, _, _, h1473, _) => {
                                            h1473
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1474) => {
                                            h1474
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1475 = v1;
                                        let a1476 = 1u64;
                                        nat_add(a1475, a1476)?
                                    }, (v4, Adt19::C0(false, false, match v3.clone() {
                                        Adt19::C0(_, _, h1477, _, _) => {
                                            h1477
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1478, _) => {
                                            h1478
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1479) => {
                                            h1479
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1480 = v1;
                                    let a1481 = 1u64;
                                    nat_add(a1480, a1481)?
                                }, (v4, Adt19::C0(true, false, f450(match v3.clone() {
                                    Adt19::C0(_, _, h1482, _, _) => {
                                        h1482
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt19::C0(_, _, _, h1483, _) => {
                                        h1483
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1484) => {
                                        h1484
                                    }
                                }))))
                            }
                        } else {
                            let a1485 = v5;
                            let a1486 = 34u64;
                            if nat_eq(a1485, a1486) {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1487 = v1;
                                    let a1488 = 1u64;
                                    nat_add(a1487, a1488)?
                                }, (v4, Adt19::C0(true, false, match v3.clone() {
                                    Adt19::C0(_, _, h1489, _, _) => {
                                        h1489
                                    }
                                }, match v3.clone() {
                                    Adt19::C0(_, _, _, h1490, _) => {
                                        h1490
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1491) => {
                                        h1491
                                    }
                                }))))
                            } else {
                                let a1492 = v5;
                                let a1493 = 44u64;
                                if nat_eq(a1492, a1493) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1494 = v1;
                                        let a1495 = 1u64;
                                        nat_add(a1494, a1495)?
                                    }, (v4, Adt19::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt19::C0(_, _, h1496, _, _) => {
                                            h1496
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1497, _) => {
                                            h1497
                                        }
                                    }), match v3 {
                                        Adt19::C0(_, _, _, _, h1498) => {
                                            h1498
                                        }
                                    }))))
                                } else {
                                    let a1499 = v5;
                                    let a1500 = 13u64;
                                    if nat_eq(a1499, a1500) {
                                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                            let a1501 = v1;
                                            let a1502 = 1u64;
                                            nat_add(a1501, a1502)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1503 = v5;
                                        let a1504 = 10u64;
                                        if nat_eq(a1503, a1504) {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1505 = v1;
                                                let a1506 = 1u64;
                                                nat_add(a1505, a1506)?
                                            }, (v4, f359(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1507 = v1;
                                                let a1508 = 1u64;
                                                nat_add(a1507, a1508)?
                                            }, (v4, Adt19::C0(false, false, f450(match v3.clone() {
                                                Adt19::C0(_, _, h1509, _, _) => {
                                                    h1509
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt19::C0(_, _, _, h1510, _) => {
                                                    h1510
                                                }
                                            }, match v3 {
                                                Adt19::C0(_, _, _, _, h1511) => {
                                                    h1511
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
                Ok(next) => { m1512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Adt19) -> Adt19 {
    Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f451(List::cons(match v0.clone() {
        Adt19::C0(_, _, h1516, _, _) => {
            h1516
        }
    }, match v0.clone() {
        Adt19::C0(_, _, _, h1517, _) => {
            h1517
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt19::C0(_, _, _, _, h1518) => {
            h1518
        }
    }))
}

pub fn f360(p1521: List<List<Bytes>>, p1522: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1520 = (p1521, p1522);
    {
        let mut m1520 = m1520;
        loop {
            let step = {
                let (v0, v1) = m1520;
                let m1519 = v0;
                match m1519.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: Bytes, v1: List<Adt17>) -> List<Bytes> {
    let m1523 = v1;
    match m1523.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt17::C0(h1524, _, _) => {
                    h1524
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt17::C0(_, h1525, _) => {
                        h1525
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt17::C0(_, _, h1526) => {
                        h1526
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f361(v0, v3))
            } else {
                f361(v0, v3)
            }
        }
    }
}

pub fn f362(v0: Bytes, v1: List<Adt2>) -> List<Bytes> {
    let m1527 = v1;
    match m1527.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f362(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f362(v0, v3))
                } else {
                    f362(v0, v3)
                }
            }
        }
    }
}

pub fn f363(p1532: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1531 = p1532;
    {
        let mut m1531 = m1531;
        loop {
            let step = {
                let v0 = m1531;
                let m1528 = v0;
                match m1528.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1529 = v2;
                        match m1529.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1530 = v5;
                                    match m1530.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f452(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: Adt14, v1: Adt2) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1533, _, _) => {
            h1533
        }
    }) {
        f52(f365(v1), match v0 {
            Adt14::C0(_, h1534, _) => {
                h1534
            }
        })
    } else {
        false
    }
}

pub fn f365(v0: Adt2) -> Bytes {
    let m1535 = f93(v0, f8(Str::lit("evidence")));
    match m1535 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f366(v0: Adt2) -> Bytes {
    if f216(v0.clone(), f8(Str::lit("req"))) {
        let a1536 = f8(Str::lit("req "));
        let a1537 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1536, a1537)
    } else {
        let m1538 = f93(v0.clone(), f8(Str::lit("std")));
        match m1538 {
            None => {
                let m1541 = f93(v0, f8(Str::lit("der")));
                match m1541 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f122(f210(f183(f93(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f216(v1.clone(), f8(Str::lit("row"))) {
                    let a1539 = f8(Str::lit(" row "));
                    let a1540 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1539, a1540)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f367(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
    let m1542 = v0;
    match m1542.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f193(v2)?;
            let a1544 = {
                let a1543 = v4.clone();
                length_bytes(a1543)
            };
            let a1545 = 0u64;
            if nat_eq(a1544, a1545) {
                f367(v3, v1)
            } else {
                let m1546 = f43(v4)?;
                match m1546 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f453(v5.clone());
                        let a1548 = {
                            let a1547 = v6.clone();
                            length_bytes(a1547)
                        };
                        let a1549 = 0u64;
                        if nat_eq(a1548, a1549) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m1550 = f367(v3, v1.clone())?;
                            match m1550 {
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

pub fn f368(v0: Bytes, v1: Bytes) -> bool {
    let a1551 = v0;
    let a1552 = f374();
    if equal(a1551, a1552) {
        let a1556 = {
            let a1554 = {
                let a1553 = v1;
                length_bytes(a1553)
            };
            let a1555 = 0u64;
            nat_eq(a1554, a1555)
        };
        bool_not(a1556)
    } else {
        false
    }
}

pub fn f369(v0: Bytes) -> Bytes {
    f7(List::cons(f144(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f370(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1558 = v0;
    let a1559 = {
        let a1557 = Str::lit("redistributable");
        utf8_encode(a1557)
    };
    if equal(a1558, a1559) {
        f454(v1)
    } else {
        Ok(false)
    }
}

pub fn f371(v0: Bytes, v1: Bytes) -> bool {
    f198(f85(v0.clone(), f8(Str::lit("read-"))), f117(f32(v0, f8(Str::lit("check-yaml"))), f32(v1, f8(Str::lit("none")))))
}

pub fn f372(v0: Adt2) -> bool {
    if f455(v0.clone(), f8(Str::lit("name"))) {
        if f456(v0.clone(), f8(Str::lit("schema"))) {
            let m1560 = f457(f210(f183(f93(v0, f8(Str::lit("citation"))))));
            match m1560 {
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

pub fn f373(v0: Bytes) -> R<Bytes> {
    f458(v0.clone(), 0u64, {
        let a1561 = v0;
        length_bytes(a1561)
    }, true, Bytes::lit(&[]))
}

pub fn f374() -> Bytes {
    let a1562 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1562)
}

pub fn f375(v0: Adt2, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt2> = f93(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f280(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f280(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f280(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f280(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f280(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f280(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f280(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f280(Str::lit("url"), f444(v2.clone(), f8(Str::lit("url"))))?, List::cons(f280(Str::lit("media_type"), f444(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f280(Str::lit("license"), f444(v2.clone(), f8(Str::lit("license"))))?, List::cons(f280(Str::lit("redistribution"), f444(v2, f8(Str::lit("redistribution"))))?, List::cons(f280(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1563 = v1;
        match m1563 {
            None => {
                f7(List::cons(f280(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f280(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f280(Str::lit("artifact"), {
                    let (h1564, _) = v3;
                    h1564
                })?, List::cons(f280(Str::lit("checksum"), {
                    let (_, h1565) = v3;
                    h1565
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f459(f210(f183(f93(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f376(v0: Adt23) -> Bytes {
    let a1586 = f460(match v0.clone() {
        Adt23::C0(h1566, _, _, _, _, _, _, _) => {
            h1566
        }
    });
    let a1587 = {
        let a1584 = f460(match v0.clone() {
            Adt23::C0(_, h1567, _, _, _, _, _, _) => {
                h1567
            }
        });
        let a1585 = {
            let a1582 = f460(match v0.clone() {
                Adt23::C0(_, _, h1568, _, _, _, _, _) => {
                    h1568
                }
            });
            let a1583 = {
                let a1580 = f460(match v0.clone() {
                    Adt23::C0(_, _, _, h1569, _, _, _, _) => {
                        h1569
                    }
                });
                let a1581 = {
                    let a1578 = f460(match v0.clone() {
                        Adt23::C0(_, _, _, _, h1570, _, _, _) => {
                            h1570
                        }
                    });
                    let a1579 = {
                        let a1576 = f460(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h1571, _, _) => {
                                h1571
                            }
                        });
                        let a1577 = {
                            let a1574 = f460(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h1572, _) => {
                                    h1572
                                }
                            });
                            let a1575 = f460(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h1573) => {
                                    h1573
                                }
                            });
                            append_bytes(a1574, a1575)
                        };
                        append_bytes(a1576, a1577)
                    };
                    append_bytes(a1578, a1579)
                };
                append_bytes(a1580, a1581)
            };
            append_bytes(a1582, a1583)
        };
        append_bytes(a1584, a1585)
    };
    append_bytes(a1586, a1587)
}

pub fn f377(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a1588 = v0.clone();
        length_bytes(a1588)
    };
    let v2: u64 = {
        let a1594 = {
            let a1591 = {
                let a1589 = v1;
                let a1590 = 72u64;
                nat_add(a1589, a1590)?
            };
            let a1592 = 64u64;
            let a1593 = 0u64;
            nat_quot(a1591, a1592, a1593)
        };
        let a1595 = 64u64;
        nat_mul(a1594, a1595)?
    };
    let (_, h1596) = f461(Fn11::F462(v0, v1, v2), {
        let a1597 = v2;
        let a1598 = 64u64;
        let a1599 = 0u64;
        nat_quot(a1597, a1598, a1599)
    }, (0u64, f463()))?;
    Ok(h1596)
}

pub fn f378(p1602: List<List<Bytes>>, v1: Bytes, p1603: List<Bytes>) -> R<List<Bytes>> {
    let m1601 = (p1602, p1603);
    {
        let mut m1601 = m1601;
        loop {
            let step = {
                let (v0, v2) = m1601;
                let m1600 = v0;
                match m1600.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f464(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f380(f465(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f305(v0)?))
}

pub fn f380(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1604 = v0;
    match m1604.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1605 = v3.clone();
            match m1605.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1608 = v2;
                    let a1609 = {
                        let a1606 = v1.clone();
                        let a1607 = f380(v3, v1);
                        append_bytes(a1606, a1607)
                    };
                    append_bytes(a1608, a1609)
                }
            }
        }
    }
}

pub fn f381(v0: List<Bytes>) -> Adt2 {
    let m1610 = v0;
    match m1610.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f381(v2)))
        }
    }
}

pub fn f382(v0: Adt2, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1611 = f93(v0, v1);
    match m1611 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v3) => {
            f183(f93(v3, v2))
        }
    }
}

pub fn f383(v0: Adt2, v1: Bytes) -> bool {
    let m1612 = f93(v0, v1);
    match m1612 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f384(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1614 = {
                let a1613 = v0;
                length_bytes(a1613)
            };
            let a1615 = 1u64;
            nat_sub(a1614, a1615)
        })
    } else {
        v0
    }
}

pub fn f385(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1616 = v0;
    match m1616 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v8, r1617) => {
            let v9 = (*r1617).clone();
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
            Adt2::C6(f466(f60(), v2, v1))
        }
        Adt2::C7(r1618) => {
            let v6 = (*r1618).clone();
            Adt2::C7(std::rc::Rc::new(v6))
        }
        Adt2::C8(r1619) => {
            let v7 = (*r1619).clone();
            Adt2::C8(std::rc::Rc::new(v7))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1620, r1621) => {
            let v15 = (*r1620).clone();
            let v16 = (*r1621).clone();
            Adt2::C10(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C11(v17, r1622, r1623) => {
            let v18 = (*r1622).clone();
            let v19 = (*r1623).clone();
            Adt2::C11(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
    }
}

pub fn f386(v0: u64) -> R<Bytes> {
    f467(f468(), 10u64, v0)
}

pub fn f387(v0: bool) -> R<u64> {
    f389({
        let a1624 = f394(v0);
        let a1625 = 1u64;
        nat_sub(a1624, a1625)
    })
}

pub fn f388(v0: bool) -> R<u64> {
    f389(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f389(v0: u64) -> R<u64> {
    let m1626 = v0;
    if m1626 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1626 - 1;
        let a1627 = 2u64;
        let a1628 = f389(v1)?;
        nat_mul(a1627, a1628)
    }
}

pub fn f390(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f391(v0: bool) -> R<u64> {
    let a1629 = f389(f390(v0))?;
    let a1630 = 1u64;
    Ok(nat_sub(a1629, a1630))
}

pub fn f392(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1631 = Bytes::lit(&[0x2d]);
        let a1632 = v1;
        append_bytes(a1631, a1632)
    } else {
        v1
    }
}

pub fn f393(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f394(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f395(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f469(f470(v2, f471(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f469(f470(v2, f471(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a1633 = v3.clone();
            length_bytes(a1633)
        };
        let v5: Bytes = {
            let a1634 = v2;
            let a1635 = v4;
            if nat_lt(a1634, a1635) {
                f46(v3.clone(), 0u64, {
                    let a1636 = v4;
                    let a1637 = v2;
                    nat_sub(a1636, a1637)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f472({
            let a1638 = v2;
            let a1639 = v4;
            if nat_lt(a1638, a1639) {
                f46(v3, {
                    let a1640 = v4;
                    let a1641 = v2;
                    nat_sub(a1640, a1641)
                }, v2)
            } else {
                let a1644 = f473({
                    let a1642 = v2;
                    let a1643 = v4;
                    nat_sub(a1642, a1643)
                });
                let a1645 = v3;
                append_bytes(a1644, a1645)
            }
        }, {
            let a1646 = v2;
            let a1647 = 1u64;
            nat_add(a1646, a1647)?
        })?;
        let a1649 = {
            let a1648 = v6.clone();
            length_bytes(a1648)
        };
        let a1650 = 0u64;
        if nat_eq(a1649, a1650) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f396(v0: Bytes, p1669: u64, p1670: u64, p1671: Bytes) -> R<Bytes> {
    let m1668 = (p1669, (p1670, p1671));
    {
        let mut m1668 = m1668;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1668;
                let m1651 = v2;
                if m1651 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1651 - 1;
                    let v5: u64 = f195(v0.clone(), v1, {
                        let a1655 = {
                            let a1653 = {
                                let a1652 = v0.clone();
                                length_bytes(a1652)
                            };
                            let a1654 = v1;
                            nat_sub(a1653, a1654)
                        };
                        let a1656 = 1u64;
                        nat_add(a1655, a1656)?
                    })?;
                    let v6: Bytes = {
                        let a1659 = v3;
                        let a1660 = f46(v0.clone(), v1, {
                            let a1657 = v5;
                            let a1658 = v1;
                            nat_sub(a1657, a1658)
                        });
                        append_bytes(a1659, a1660)
                    };
                    let m1663 = {
                        let a1661 = v0.clone();
                        let a1662 = v5;
                        index_bytes(a1661, a1662)
                    };
                    match m1663 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1664 = v5;
                                let a1665 = 1u64;
                                nat_add(a1664, a1665)?
                            }, (v4, {
                                let a1666 = v6;
                                let a1667 = f474(v0.clone(), v5, v7)?;
                                append_bytes(a1666, a1667)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Fn5, p1675: List<(Bytes, Bytes)>, p1676: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
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
                            let c1673: Fn5 = v0.clone();
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

pub fn f398(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f475(v0, v1)
}

pub fn f399(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f476(v0)
}

pub fn f400(v0: Fn6, p1680: List<Bytes>, p1681: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
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
                            let c1678: Fn6 = v0.clone();
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

pub fn f401(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f477(v1)?, v0))
}

pub fn f402(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f478(v0)?, List::cons(f8(Str::lit(":")), List::cons(f478(v1)?, List::<Bytes>::nil())))))
}

pub fn f403(v0: Adt13) -> Bytes {
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

pub fn f404(v0: Adt13) -> Bytes {
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

pub fn f405(v0: Adt13) -> Bytes {
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

pub fn f406(v0: Bytes, v1: u64) -> Option<u64> {
    let a1685 = 0u64;
    let a1686 = v1;
    if nat_lt(a1685, a1686) {
        if f479(v0, {
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

pub fn f407(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1691 = v2;
    let a1692 = v0;
    if nat_lt(a1691, a1692) {
        if f479(v1, v2) {
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

pub fn f408(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1697 = f46(v0.clone(), {
        let a1695 = v1;
        let a1696 = 2u64;
        nat_add(a1695, a1696)?
    }, 2u64);
    let a1698 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1697, a1698) {
        let m1701 = f480(f46(v0, {
            let a1699 = v1;
            let a1700 = 4u64;
            nat_add(a1699, a1700)?
        }, 2u64))?;
        match m1701 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f115(v2.clone(), 0u64)?;
                let a1702 = v3;
                let a1703 = 32u64;
                if nat_lt(a1702, a1703) {
                    if f481(v3) {
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

pub fn f409(v0: u64) -> Bytes {
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

pub fn f410(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f115(v0, v1)?;
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

pub fn f411(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1724 = f115(v0, v1)?;
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

pub fn f412(v0: Bytes) -> R<u64> {
    let (h1728, _) = f308(Fn3::F482(v0.clone()), {
        let a1729 = v0;
        length_bytes(a1729)
    }, 0u64)?;
    Ok(h1728)
}

pub fn f413(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
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
                let a1741 = f483(v2)?;
                nat_add(a1740, a1741)?
            })))
        }
    }
}

pub fn f414(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f484(v0, v1)
}

pub fn f415(v0: (Bytes, Bytes), p1744: List<(Bytes, Bytes)>) -> bool {
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
                        if f485(v0.clone(), v2) {
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

pub fn f416(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
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
                }, f416(v0, v3))
            } else {
                f416(v0, v3)
            }
        }
    }
}

pub fn f417() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f418(v0: Adt2, v1: Bytes, v2: Bytes) -> bool {
    let m1748 = f93(v0, v1);
    match m1748 {
        None => {
            false
        }
        Some(v3) => {
            f216(v3, v2)
        }
    }
}

pub fn f419() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f420(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f421(v0: Bytes) -> R<bool> {
    let v1: Bytes = f193(v0)?;
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
        let m1752 = f327(v3.clone(), f8(Str::lit(")")))?;
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
        f486(v5)
    } else {
        Ok(false)
    }
}

pub fn f422(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f423(v2, v0, v4) {
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

pub fn f423(v0: Bytes, v1: Bytes, v2: u64) -> bool {
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

pub fn f424(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1775 = v0;
    match m1775.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f424(v3, {
                let a1776 = v1;
                let a1777 = 1u64;
                nat_add(a1776, a1777)?
            })?))
        }
    }
}

pub fn f425(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1778 = v2;
    match m1778 {
        Adt21::C0(v3, v4) => {
            if f431(v3, match v0.clone() {
                Adt18::C0(h1779, _, _, _) => {
                    h1779
                }
            }) {
                f487(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f431(v5, match v0 {
                Adt18::C0(_, _, h1780, _) => {
                    h1780
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f427(v7, v1) {
                f431(v8, match v0 {
                    Adt18::C0(_, _, _, h1781) => {
                        h1781
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f426(v0: Adt15, v1: Adt2) -> R<Adt21> {
    let m1782 = f93(v1.clone(), f8(Str::lit("std")));
    match m1782 {
        None => {
            let m1787 = f92(f93(v1.clone(), f8(Str::lit("req"))));
            match m1787 {
                None => {
                    let m1789 = f93(v1, f8(Str::lit("der")));
                    match m1789 {
                        None => {
                            Ok(f488(v0))
                        }
                        Some(v5) => {
                            let m1790 = f489(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1790 {
                                None => {
                                    Ok(f488(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f335(f210(f183(f93(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1791) => {
                                            h1791
                                        }
                                    })?, f429(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1792, _) => {
                                            h1792
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f429(v4, match v0 {
                        Adt15::C0(_, _, h1788, _, _) => {
                            h1788
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1783 = f92(f93(v2.clone(), f8(Str::lit("row"))));
            match m1783 {
                None => {
                    Ok(Adt21::C0(f429(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1784, _, _, _, _) => {
                            h1784
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f429(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1785, _, _, _, _) => {
                            h1785
                        }
                    })?, Some(f429(v3, match v0 {
                        Adt15::C0(_, h1786, _, _, _) => {
                            h1786
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f427(p1795: List<u64>, v1: List<u64>) -> bool {
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
                        if f431(v2, v1.clone()) {
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

pub fn f428(p1800: List<u64>) -> bool {
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

pub fn f429(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1801 = f490(v0, v1.clone(), 0u64)?;
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

pub fn f430(v0: Adt2) -> u64 {
    let m1803 = f491(f93(v0, f8(Str::lit("stratum"))));
    match m1803 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f431(v0: u64, p1808: List<u64>) -> bool {
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

pub fn f432(v0: Fn7, p1812: List<Bytes>, p1813: List<(Bytes, ())>) -> List<Bytes> {
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
                            let c1810: Fn7 = v0.clone();
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

pub fn f433(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1814, _) = v1;
        h1814
    }, v0)
}

pub fn f434(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f492(Fn12::F493, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f435(v0: Fn8, p1818: List<List<(Bytes, ())>>, p1819: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
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
                            let c1816: Fn8 = v0.clone();
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

pub fn f436(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f437(v0: Fn9, p1824: u64, p1825: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
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
                        let c1821: Fn9 = v0.clone();
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

pub fn f438(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f494(v0)
}

pub fn f439(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f495(Fn13::F496, List::<(Bytes, ())>::nil(), v0)
}

pub fn f440(v0: Fn10, p1829: List<(Bytes, ())>, p1830: List<Bytes>) -> List<(Bytes, ())> {
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
                            let c1827: Fn10 = v0.clone();
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

pub fn f441(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f442(p1835: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
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

pub fn f443(v0: Bytes, p1861: u64, p1862: u64, p1863: Bytes) -> R<Bytes> {
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
                            if f198({
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

pub fn f444(v0: Option<Adt2>, v1: Bytes) -> Bytes {
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

pub fn f445(v0: List<Adt2>) -> List<Bytes> {
    let m1865 = v0;
    match m1865.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f497(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f498(f58(v1, f8(Str::lit("authority")))), f445(v2)))
        }
    }
}

pub fn f446(v0: List<Bytes>) -> R<List<Adt2>> {
    let m1866 = f284(v0.clone())?;
    match m1866 {
        None => {
            Ok(f273(f272(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f447(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    let m1867 = f284(v3.clone())?;
    match m1867 {
        None => {
            let m1868 = f272(v3.clone())?;
            match m1868 {
                None => {
                    f501(v0, v1, v3)
                }
                Some(v5) => {
                    f500(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f499(v0, v1, v4)
        }
    }
}

pub fn f448(v0: List<Bytes>) -> List<Bytes> {
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
            }, f448(v2))
        }
    }
}

pub fn f449(v0: Bytes, v1: u64) -> R<u64> {
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

pub fn f450(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
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

pub fn f451(p1884: List<Bytes>, p1885: List<Bytes>) -> List<Bytes> {
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

pub fn f452(v0: List<Bytes>) -> Bytes {
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

pub fn f453(v0: Adt2) -> Bytes {
    if f216(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f216(v0.clone(), f8(Str::lit("id"))) {
            if f216(v0.clone(), f8(Str::lit("statement"))) {
                if f216(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f383(v0, f8(Str::lit("evidence")))
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

pub fn f454(v0: Bytes) -> R<bool> {
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
                Ok(f502({
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

pub fn f455(v0: Adt2, v1: Bytes) -> bool {
    let a1939 = {
        let a1938 = f58(v0, v1);
        length_bytes(a1938)
    };
    let a1940 = 0u64;
    !nat_eq(a1939, a1940)
}

pub fn f456(v0: Adt2, v1: Bytes) -> bool {
    let m1941 = f93(v0, v1);
    match m1941 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f457(v0: List<Bytes>) -> Option<Adt2> {
    let m1942 = v0;
    match m1942.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1943 = v2;
                match m1943.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1944 = v2;
                    match m1944.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m1945 = v6;
                                match m1945.uncons() {
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
                        let m1946 = v2;
                        match m1946.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m1947 = v10;
                                match m1947.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f381(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
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

pub fn f458(v0: Bytes, p1968: u64, p1969: u64, p1970: bool, p1971: Bytes) -> R<Bytes> {
    let m1967 = (p1968, (p1969, (p1970, p1971)));
    {
        let mut m1967 = m1967;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1967;
                let m1948 = v2;
                if m1948 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1948 - 1;
                    let m1951 = {
                        let a1949 = v0.clone();
                        let a1950 = v1;
                        index_bytes(a1949, a1950)
                    };
                    match m1951 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f503(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1952 = v1;
                                    let a1953 = 1u64;
                                    nat_add(a1952, a1953)?
                                }, (v5, (true, v4))))
                            } else {
                                if f117(f504(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1954 = v1;
                                        let a1955 = 1u64;
                                        nat_add(a1954, a1955)?
                                    }, (v5, (false, {
                                        let a1959 = v4;
                                        let a1960 = f46({
                                            let a1956 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1956)
                                        }, {
                                            let a1957 = v7;
                                            let a1958 = 97u64;
                                            nat_sub(a1957, a1958)
                                        }, 1u64);
                                        append_bytes(a1959, a1960)
                                    }))))
                                } else {
                                    if f198(f504(v7), f198(f505(v7), f506(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1961 = v1;
                                            let a1962 = 1u64;
                                            nat_add(a1961, a1962)?
                                        }, (v5, (false, {
                                            let a1963 = v4;
                                            let a1964 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1963, a1964)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1965 = v1;
                                            let a1966 = 1u64;
                                            nat_add(a1965, a1966)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f380(f507(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f460(v0: u32) -> Bytes {
    let a2000 = f508({
        let a1972 = f509(v0, 28u32);
        let a1973 = 15u32;
        fixed_u32::bit_and(a1972, a1973)
    });
    let a2001 = {
        let a1998 = f508({
            let a1974 = f509(v0, 24u32);
            let a1975 = 15u32;
            fixed_u32::bit_and(a1974, a1975)
        });
        let a1999 = {
            let a1996 = f508({
                let a1976 = f509(v0, 20u32);
                let a1977 = 15u32;
                fixed_u32::bit_and(a1976, a1977)
            });
            let a1997 = {
                let a1994 = f508({
                    let a1978 = f509(v0, 16u32);
                    let a1979 = 15u32;
                    fixed_u32::bit_and(a1978, a1979)
                });
                let a1995 = {
                    let a1992 = f508({
                        let a1980 = f509(v0, 12u32);
                        let a1981 = 15u32;
                        fixed_u32::bit_and(a1980, a1981)
                    });
                    let a1993 = {
                        let a1990 = f508({
                            let a1982 = f509(v0, 8u32);
                            let a1983 = 15u32;
                            fixed_u32::bit_and(a1982, a1983)
                        });
                        let a1991 = {
                            let a1988 = f508({
                                let a1984 = f509(v0, 4u32);
                                let a1985 = 15u32;
                                fixed_u32::bit_and(a1984, a1985)
                            });
                            let a1989 = f508({
                                let a1986 = v0;
                                let a1987 = 15u32;
                                fixed_u32::bit_and(a1986, a1987)
                            });
                            append_bytes(a1988, a1989)
                        };
                        append_bytes(a1990, a1991)
                    };
                    append_bytes(a1992, a1993)
                };
                append_bytes(a1994, a1995)
            };
            append_bytes(a1996, a1997)
        };
        append_bytes(a1998, a1999)
    };
    append_bytes(a2000, a2001)
}

pub fn f461(v0: Fn11, p2005: u64, p2006: (u64, Adt23)) -> R<(u64, Adt23)> {
    let m2004 = (p2005, p2006);
    {
        let mut m2004 = m2004;
        loop {
            let step = {
                let (v1, v2) = m2004;
                let m2002 = v1;
                if m2002 == 0 {
                    Err::<(u64, (u64, Adt23)), R<(u64, Adt23)>>(Ok(v2))
                } else {
                    let v3 = m2002 - 1;
                    Ok::<(u64, (u64, Adt23)), R<(u64, Adt23)>>((v3, {
                        let c2003: Fn11 = v0.clone();
                        c2003.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2004 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a2008 = {
            let (h2007, _) = v3.clone();
            h2007
        };
        let a2009 = 64u64;
        nat_add(a2008, a2009)?
    }, f510({
        let (_, h2010) = v3;
        h2010
    }, f511(v0, v1, v2, {
        let (h2011, _) = v3;
        h2011
    })?)?))
}

pub fn f463() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f464(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2012 = f44(v1, 34u8)?;
    match m2012.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2013 = v3;
            match m2013.uncons() {
                None => {
                    Ok(f512(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f465(v0: List<Bytes>) -> List<Bytes> {
    let m2014 = v0;
    match m2014.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f465(v2)
            } else {
                List::cons(v1, f465(v2))
            }
        }
    }
}

pub fn f466(p2017: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2016 = p2017;
    {
        let mut m2016 = m2016;
        loop {
            let step = {
                let v0 = m2016;
                let m2015 = v0;
                match m2015.uncons() {
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
                Ok(next) => { m2016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2018 = v2;
    let a2019 = 0u64;
    if nat_eq(a2018, a2019) {
        Ok(f513(v0, 0u64))
    } else {
        let (_, h2020) = {
            let (h2021, _) = f514(Fn14::F515(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2021
        };
        Ok(h2020)
    }
}

pub fn f468() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f469(v0: List<u64>) -> R<Bytes> {
    let m2022 = f516(v0, List::<u64>::nil());
    match m2022.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2023 = f386(v1)?;
            let a2024 = f517(v2)?;
            Ok(append_bytes(a2023, a2024))
        }
    }
}

pub fn f470(p2027: u64, p2028: List<u64>, v2: u64) -> R<List<u64>> {
    let m2026 = (p2027, p2028);
    {
        let mut m2026 = m2026;
        loop {
            let step = {
                let (v0, v1) = m2026;
                let m2025 = v0;
                if m2025 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2025 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f518(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: u64, v1: u64) -> List<u64> {
    let m2029 = v0;
    if m2029 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2029 - 1;
        let a2030 = v1;
        let a2031 = 0u64;
        if nat_eq(a2030, a2031) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2032 = v1;
                let a2033 = f519();
                let a2034 = 0u64;
                nat_rem(a2032, a2033, a2034)
            }, f471(v2, {
                let a2035 = v1;
                let a2036 = f519();
                let a2037 = 0u64;
                nat_quot(a2035, a2036, a2037)
            }))
        }
    }
}

pub fn f472(p2048: Bytes, p2049: u64) -> R<Bytes> {
    let m2047 = (p2048, p2049);
    {
        let mut m2047 = m2047;
        loop {
            let step = {
                let (v0, v1) = m2047;
                let m2038 = v1;
                if m2038 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2038 - 1;
                    let a2042 = f115(v0.clone(), {
                        let a2040 = {
                            let a2039 = v0.clone();
                            length_bytes(a2039)
                        };
                        let a2041 = 1u64;
                        nat_sub(a2040, a2041)
                    })?;
                    let a2043 = 48u64;
                    if nat_eq(a2042, a2043) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2045 = {
                                let a2044 = v0;
                                length_bytes(a2044)
                            };
                            let a2046 = 1u64;
                            nat_sub(a2045, a2046)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2047 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: u64) -> Bytes {
    let m2050 = v0;
    if m2050 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2050 - 1;
        let a2051 = Bytes::lit(&[0x30]);
        let a2052 = f473(v1);
        append_bytes(a2051, a2052)
    }
}

pub fn f474(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2097 = {
            let a2073 = {
                let a2061 = {
                    let a2055 = {
                        let a2053 = v3;
                        let a2054 = 128u8;
                        fixed_u8::bit_and(a2053, a2054)
                    };
                    let a2056 = 128u8;
                    if equal(a2055, a2056) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2062 = {
                    let a2059 = {
                        let a2057 = v3;
                        let a2058 = 64u8;
                        fixed_u8::bit_and(a2057, a2058)
                    };
                    let a2060 = 64u8;
                    if equal(a2059, a2060) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2061, a2062)?
            };
            let a2074 = {
                let a2071 = {
                    let a2065 = {
                        let a2063 = v3;
                        let a2064 = 32u8;
                        fixed_u8::bit_and(a2063, a2064)
                    };
                    let a2066 = 32u8;
                    if equal(a2065, a2066) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2072 = {
                    let a2069 = {
                        let a2067 = v3;
                        let a2068 = 16u8;
                        fixed_u8::bit_and(a2067, a2068)
                    };
                    let a2070 = 16u8;
                    if equal(a2069, a2070) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2071, a2072)?
            };
            nat_add(a2073, a2074)?
        };
        let a2098 = {
            let a2095 = {
                let a2083 = {
                    let a2077 = {
                        let a2075 = v3;
                        let a2076 = 8u8;
                        fixed_u8::bit_and(a2075, a2076)
                    };
                    let a2078 = 8u8;
                    if equal(a2077, a2078) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2084 = {
                    let a2081 = {
                        let a2079 = v3;
                        let a2080 = 4u8;
                        fixed_u8::bit_and(a2079, a2080)
                    };
                    let a2082 = 4u8;
                    if equal(a2081, a2082) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2083, a2084)?
            };
            let a2096 = {
                let a2093 = {
                    let a2087 = {
                        let a2085 = v3;
                        let a2086 = 2u8;
                        fixed_u8::bit_and(a2085, a2086)
                    };
                    let a2088 = 2u8;
                    if equal(a2087, a2088) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2094 = {
                    let a2091 = {
                        let a2089 = v3;
                        let a2090 = 1u8;
                        fixed_u8::bit_and(a2089, a2090)
                    };
                    let a2092 = 1u8;
                    if equal(a2091, a2092) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2093, a2094)?
            };
            nat_add(a2095, a2096)?
        };
        nat_add(a2097, a2098)?
    };
    let a2099 = v4;
    let a2100 = 34u64;
    if nat_eq(a2099, a2100) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2101 = v4;
        let a2102 = 92u64;
        if nat_eq(a2101, a2102) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2103 = v4;
            let a2104 = 8u64;
            if nat_eq(a2103, a2104) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2105 = v4;
                let a2106 = 12u64;
                if nat_eq(a2105, a2106) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2107 = v4;
                    let a2108 = 10u64;
                    if nat_eq(a2107, a2108) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2109 = v4;
                        let a2110 = 13u64;
                        if nat_eq(a2109, a2110) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2111 = v4;
                            let a2112 = 9u64;
                            if nat_eq(a2111, a2112) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2113 = v4;
                                let a2114 = 32u64;
                                if nat_lt(a2113, a2114) {
                                    let a2115 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2116 = f520(v2)?;
                                    Ok(append_bytes(a2115, a2116))
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

pub fn f475(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2117 = v0.clone();
    match m2117.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2120 = {
                let (h2118, _) = v2;
                h2118
            };
            let a2121 = {
                let (h2119, _) = v1.clone();
                h2119
            };
            if equal(a2120, a2121) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f476(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f521(f522(Fn15::F523, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2126 = {
        let (h2122, _) = f524(Fn16::F525, {
            let a2124 = {
                let a2123 = v0;
                length_list(a2123)
            };
            let a2125 = 1u64;
            nat_add(a2124, a2125)?
        }, v1)?;
        h2122
    };
    match m2126.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f477(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2131 = {
        let a2128 = f296({
            let (h2127, _) = v0;
            h2127
        })?;
        let a2129 = Bytes::lit(&[0x3a]);
        append_bytes(a2128, a2129)
    };
    let a2132 = {
        let (_, h2130) = v0;
        h2130
    };
    Ok(append_bytes(a2131, a2132))
}

pub fn f478(v0: Bytes) -> R<Bytes> {
    let a2138 = {
        let a2136 = Bytes::lit(&[0x22]);
        let a2137 = {
            let (_, h2133) = {
                let (h2134, _) = f514(Fn14::F526(v0.clone()), {
                    let a2135 = v0;
                    length_bytes(a2135)
                }, (0u64, Bytes::lit(&[])))?;
                h2134
            };
            h2133
        };
        append_bytes(a2136, a2137)
    };
    let a2139 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2138, a2139))
}

pub fn f479(v0: Bytes, v1: u64) -> bool {
    let m2140 = f203(v0, v1);
    match m2140 {
        None => {
            false
        }
        Some(v2) => {
            f527(v2)
        }
    }
}

pub fn f480(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2141 = v0.clone();
        length_bytes(a2141)
    };
    let a2145 = {
        let a2142 = v1;
        let a2143 = 2u64;
        let a2144 = 0u64;
        nat_rem(a2142, a2143, a2144)
    };
    let a2146 = 0u64;
    if nat_eq(a2145, a2146) {
        let (_, h2147) = {
            let (h2148, _) = f528(Fn17::F529(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2148
        };
        Ok(h2147)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f481(v0: u64) -> bool {
    let a2149 = v0;
    let a2150 = 8u64;
    if nat_eq(a2149, a2150) {
        true
    } else {
        let a2151 = v0;
        let a2152 = 9u64;
        if nat_eq(a2151, a2152) {
            true
        } else {
            let a2153 = v0;
            let a2154 = 10u64;
            if nat_eq(a2153, a2154) {
                true
            } else {
                let a2155 = v0;
                let a2156 = 12u64;
                if nat_eq(a2155, a2156) {
                    true
                } else {
                    let a2157 = v0;
                    let a2158 = 13u64;
                    nat_eq(a2157, a2158)
                }
            }
        }
    }
}

pub fn f482(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2161 = {
        let a2159 = v0;
        let a2160 = v1;
        index_bytes(a2159, a2160)
    };
    match m2161 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f530(v2)? {
                Ok(Some({
                    let a2162 = v1;
                    let a2163 = 1u64;
                    nat_add(a2162, a2163)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f483(v0: u8) -> R<u64> {
    let a2210 = {
        let v1: u8 = v0;
        let a2208 = {
            let a2184 = {
                let a2172 = {
                    let a2166 = {
                        let a2164 = v1;
                        let a2165 = 128u8;
                        fixed_u8::bit_and(a2164, a2165)
                    };
                    let a2167 = 128u8;
                    if equal(a2166, a2167) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2173 = {
                    let a2170 = {
                        let a2168 = v1;
                        let a2169 = 64u8;
                        fixed_u8::bit_and(a2168, a2169)
                    };
                    let a2171 = 64u8;
                    if equal(a2170, a2171) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2172, a2173)?
            };
            let a2185 = {
                let a2182 = {
                    let a2176 = {
                        let a2174 = v1;
                        let a2175 = 32u8;
                        fixed_u8::bit_and(a2174, a2175)
                    };
                    let a2177 = 32u8;
                    if equal(a2176, a2177) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2183 = {
                    let a2180 = {
                        let a2178 = v1;
                        let a2179 = 16u8;
                        fixed_u8::bit_and(a2178, a2179)
                    };
                    let a2181 = 16u8;
                    if equal(a2180, a2181) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2182, a2183)?
            };
            nat_add(a2184, a2185)?
        };
        let a2209 = {
            let a2206 = {
                let a2194 = {
                    let a2188 = {
                        let a2186 = v1;
                        let a2187 = 8u8;
                        fixed_u8::bit_and(a2186, a2187)
                    };
                    let a2189 = 8u8;
                    if equal(a2188, a2189) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2195 = {
                    let a2192 = {
                        let a2190 = v1;
                        let a2191 = 4u8;
                        fixed_u8::bit_and(a2190, a2191)
                    };
                    let a2193 = 4u8;
                    if equal(a2192, a2193) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2194, a2195)?
            };
            let a2207 = {
                let a2204 = {
                    let a2198 = {
                        let a2196 = v1;
                        let a2197 = 2u8;
                        fixed_u8::bit_and(a2196, a2197)
                    };
                    let a2199 = 2u8;
                    if equal(a2198, a2199) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2205 = {
                    let a2202 = {
                        let a2200 = v1;
                        let a2201 = 1u8;
                        fixed_u8::bit_and(a2200, a2201)
                    };
                    let a2203 = 1u8;
                    if equal(a2202, a2203) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2204, a2205)?
            };
            nat_add(a2206, a2207)?
        };
        nat_add(a2208, a2209)?
    };
    let a2211 = 48u64;
    Ok(nat_sub(a2210, a2211))
}

pub fn f484(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2212 = v0.clone();
        length_bytes(a2212)
    };
    let a2213 = v1;
    let a2214 = v2;
    if if nat_lt(a2213, a2214) {
        let a2219 = f115(v0, {
            let a2217 = v2;
            let a2218 = {
                let a2215 = v1;
                let a2216 = 1u64;
                nat_add(a2215, a2216)?
            };
            nat_sub(a2217, a2218)
        })?;
        let a2220 = 48u64;
        nat_eq(a2219, a2220)
    } else {
        false
    } {
        Ok(Some({
            let a2221 = v1;
            let a2222 = 1u64;
            nat_add(a2221, a2222)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f485(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f117(f32({
        let (h2223, _) = v0;
        h2223
    }, {
        let (h2224, _) = v1;
        h2224
    }), f32({
        let (_, h2225) = v0;
        h2225
    }, {
        let (_, h2226) = v1;
        h2226
    }))
}

pub fn f486(v0: Bytes) -> R<bool> {
    let m2227 = f327(v0.clone(), f8(Str::lit("&&")))?;
    if match m2227 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2228 = f327(v0, f8(Str::lit("||")))?;
        match m2228 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f487(v0: Adt18, v1: Option<u64>) -> bool {
    let m2229 = v1;
    match m2229 {
        None => {
            true
        }
        Some(v2) => {
            f431(v2, match v0 {
                Adt18::C0(_, h2230, _, _) => {
                    h2230
                }
            })
        }
    }
}

pub fn f488(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a2232 = match v0 {
            Adt15::C0(_, _, h2231, _, _) => {
                h2231
            }
        };
        length_list(a2232)
    })
}

pub fn f489(v0: Bytes) -> Option<Adt22> {
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

pub fn f490(v0: Bytes, p2237: List<Bytes>, p2238: u64) -> R<Option<u64>> {
    let m2236 = (p2237, p2238);
    {
        let mut m2236 = m2236;
        loop {
            let step = {
                let (v1, v2) = m2236;
                let m2233 = v1;
                match m2233.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2234 = v2;
                                let a2235 = 1u64;
                                nat_add(a2234, a2235)?
                            }))
                        }
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

pub fn f491(v0: Option<Adt2>) -> Option<u64> {
    let m2239 = v0;
    match m2239 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2240 = v1;
            match m2240 {
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

pub fn f492(v0: Fn12, p2244: List<List<(Bytes, ())>>, p2245: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2243 = (p2244, p2245);
    {
        let mut m2243 = m2243;
        loop {
            let step = {
                let (v1, v2) = m2243;
                let m2241 = v2;
                match m2241.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2242: Fn12 = v0.clone();
                            c2242.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f494(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2247 = {
        let a2246 = v0.clone();
        length_list(a2246)
    };
    let a2248 = 1u64;
    if nat_le(a2247, a2248) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f531(v0)?))
    }
}

pub fn f495(v0: Fn13, p2252: List<(Bytes, ())>, p2253: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2251 = (p2252, p2253);
    {
        let mut m2251 = m2251;
        loop {
            let step = {
                let (v1, v2) = m2251;
                let m2249 = v2;
                match m2249.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2250: Fn13 = v0.clone();
                            c2250.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f497(v0: Bytes) -> Bytes {
    f7(List::cons(f139(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f498(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f499(v0: Adt1, v1: Bytes, v2: List<Adt2>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f142(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f142(f96(v0, f153())?, f8(Str::lit("at"))));
    let a2255 = {
        let a2254 = v4.clone();
        length_list(a2254)
    };
    let a2256 = 0u64;
    if nat_eq(a2255, a2256) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2259 = {
            let a2257 = v4;
            length_list(a2257)
        };
        let a2260 = {
            let a2258 = v3;
            length_list(a2258)
        };
        if nat_eq(a2259, a2260) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f532(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f500(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt2>) -> R<(bool, Bytes)> {
    let m2261 = v3;
    match m2261.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f198(f533(v0.clone(), v6.clone())?, f52(v6.clone(), v2.clone())) {
                f500(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f498(v6.clone()))) {
                    let v9: (bool, Bytes) = f500(v0, v1.clone(), v2, v5)?;
                    let (h2262, _) = v9.clone();
                    if h2262 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2264 = f532(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2265 = {
                                let (_, h2263) = v9;
                                h2263
                            };
                            append_bytes(a2264, a2265)
                        }))
                    }
                } else {
                    if f117(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f534(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2266 = f8(Str::lit("Catalog."));
                        let a2267 = v8.clone();
                        append_bytes(a2266, a2267)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f500(v0, v1.clone(), v2, v5)?;
                        let (h2268, _) = v10.clone();
                        if h2268 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2270 = f532(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2271 = {
                                    let (_, h2269) = v10;
                                    h2269
                                };
                                append_bytes(a2270, a2271)
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

pub fn f501(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt2> = f273(f367(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt2> = f273(f367(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f125(f142(v3.clone(), f8(Str::lit("id"))), f142(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f125(v6.clone(), f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2273 = {
        let a2272 = v7.clone();
        length_list(a2272)
    };
    let a2274 = 0u64;
    if nat_eq(a2273, a2274) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2277 = {
                let a2275 = v7;
                length_list(a2275)
            };
            let a2278 = {
                let a2276 = v5;
                length_list(a2276)
            };
            nat_eq(a2277, a2278)
        }) {
            Ok((false, f532(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f145(f125(f62(f535(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f536(v4), f125(v6, f142(v3, f8(Str::lit("id")))))))?;
            let a2280 = {
                let a2279 = v8.clone();
                length_list(a2279)
            };
            let a2281 = 0u64;
            if nat_eq(a2280, a2281) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f532(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f502(v0: u64) -> bool {
    let a2282 = v0;
    let a2283 = 47u64;
    nat_eq(a2282, a2283)
}

pub fn f503(v0: u64) -> bool {
    f198({
        let a2284 = v0;
        let a2285 = 95u64;
        nat_eq(a2284, a2285)
    }, f198({
        let a2286 = v0;
        let a2287 = 45u64;
        nat_eq(a2286, a2287)
    }, f198({
        let a2288 = v0;
        let a2289 = 32u64;
        nat_eq(a2288, a2289)
    }, f198({
        let a2290 = v0;
        let a2291 = 46u64;
        nat_eq(a2290, a2291)
    }, {
        let a2292 = v0;
        let a2293 = 47u64;
        nat_eq(a2292, a2293)
    }))))
}

pub fn f504(v0: u64) -> bool {
    f117({
        let a2294 = 97u64;
        let a2295 = v0;
        nat_le(a2294, a2295)
    }, {
        let a2296 = v0;
        let a2297 = 122u64;
        nat_le(a2296, a2297)
    })
}

pub fn f505(v0: u64) -> bool {
    f117({
        let a2298 = 65u64;
        let a2299 = v0;
        nat_le(a2298, a2299)
    }, {
        let a2300 = v0;
        let a2301 = 90u64;
        nat_le(a2300, a2301)
    })
}

pub fn f506(v0: u64) -> bool {
    f117({
        let a2302 = 48u64;
        let a2303 = v0;
        nat_le(a2302, a2303)
    }, {
        let a2304 = v0;
        let a2305 = 57u64;
        nat_le(a2304, a2305)
    })
}

pub fn f507(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2306 = v0;
    match m2306.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f345(v1)?, f507(v2)?))
        }
    }
}

pub fn f508(v0: u32) -> Bytes {
    let a2309 = {
        let a2307 = v0;
        let a2308 = 8u32;
        fixed_u32::bit_and(a2307, a2308)
    };
    let a2310 = 0u32;
    if equal(a2309, a2310) {
        let a2313 = {
            let a2311 = v0;
            let a2312 = 4u32;
            fixed_u32::bit_and(a2311, a2312)
        };
        let a2314 = 0u32;
        if equal(a2313, a2314) {
            let a2317 = {
                let a2315 = v0;
                let a2316 = 2u32;
                fixed_u32::bit_and(a2315, a2316)
            };
            let a2318 = 0u32;
            if equal(a2317, a2318) {
                let a2321 = {
                    let a2319 = v0;
                    let a2320 = 1u32;
                    fixed_u32::bit_and(a2319, a2320)
                };
                let a2322 = 0u32;
                if equal(a2321, a2322) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2325 = {
                    let a2323 = v0;
                    let a2324 = 1u32;
                    fixed_u32::bit_and(a2323, a2324)
                };
                let a2326 = 0u32;
                if equal(a2325, a2326) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2329 = {
                let a2327 = v0;
                let a2328 = 2u32;
                fixed_u32::bit_and(a2327, a2328)
            };
            let a2330 = 0u32;
            if equal(a2329, a2330) {
                let a2333 = {
                    let a2331 = v0;
                    let a2332 = 1u32;
                    fixed_u32::bit_and(a2331, a2332)
                };
                let a2334 = 0u32;
                if equal(a2333, a2334) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2337 = {
                    let a2335 = v0;
                    let a2336 = 1u32;
                    fixed_u32::bit_and(a2335, a2336)
                };
                let a2338 = 0u32;
                if equal(a2337, a2338) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2341 = {
            let a2339 = v0;
            let a2340 = 4u32;
            fixed_u32::bit_and(a2339, a2340)
        };
        let a2342 = 0u32;
        if equal(a2341, a2342) {
            let a2345 = {
                let a2343 = v0;
                let a2344 = 2u32;
                fixed_u32::bit_and(a2343, a2344)
            };
            let a2346 = 0u32;
            if equal(a2345, a2346) {
                let a2349 = {
                    let a2347 = v0;
                    let a2348 = 1u32;
                    fixed_u32::bit_and(a2347, a2348)
                };
                let a2350 = 0u32;
                if equal(a2349, a2350) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2353 = {
                    let a2351 = v0;
                    let a2352 = 1u32;
                    fixed_u32::bit_and(a2351, a2352)
                };
                let a2354 = 0u32;
                if equal(a2353, a2354) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2357 = {
                let a2355 = v0;
                let a2356 = 2u32;
                fixed_u32::bit_and(a2355, a2356)
            };
            let a2358 = 0u32;
            if equal(a2357, a2358) {
                let a2361 = {
                    let a2359 = v0;
                    let a2360 = 1u32;
                    fixed_u32::bit_and(a2359, a2360)
                };
                let a2362 = 0u32;
                if equal(a2361, a2362) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2365 = {
                    let a2363 = v0;
                    let a2364 = 1u32;
                    fixed_u32::bit_and(a2363, a2364)
                };
                let a2366 = 0u32;
                if equal(a2365, a2366) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f509(v0: u32, v1: u32) -> u32 {
    let m2369 = {
        let a2367 = v0;
        let a2368 = v1;
        fixed_u32::shift_right(a2367, a2368)
    };
    match m2369 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f510(v0: Adt23, v1: Adt24) -> R<Adt23> {
    let v2: Adt25 = f537(Fn18::F538, 64u64, Adt25::C0(match v0.clone() {
        Adt23::C0(h2370, _, _, _, _, _, _, _) => {
            h2370
        }
    }, match v0.clone() {
        Adt23::C0(_, h2371, _, _, _, _, _, _) => {
            h2371
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2372, _, _, _, _, _) => {
            h2372
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h2373, _, _, _, _) => {
            h2373
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h2374, _, _, _) => {
            h2374
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2375, _, _) => {
            h2375
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2376, _) => {
            h2376
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2377) => {
            h2377
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f539(match v0.clone() {
        Adt23::C0(h2378, _, _, _, _, _, _, _) => {
            h2378
        }
    }, match v2.clone() {
        Adt25::C0(h2379, _, _, _, _, _, _, _, _, _) => {
            h2379
        }
    }), f539(match v0.clone() {
        Adt23::C0(_, h2380, _, _, _, _, _, _) => {
            h2380
        }
    }, match v2.clone() {
        Adt25::C0(_, h2381, _, _, _, _, _, _, _, _) => {
            h2381
        }
    }), f539(match v0.clone() {
        Adt23::C0(_, _, h2382, _, _, _, _, _) => {
            h2382
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h2383, _, _, _, _, _, _, _) => {
            h2383
        }
    }), f539(match v0.clone() {
        Adt23::C0(_, _, _, h2384, _, _, _, _) => {
            h2384
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h2385, _, _, _, _, _, _) => {
            h2385
        }
    }), f539(match v0.clone() {
        Adt23::C0(_, _, _, _, h2386, _, _, _) => {
            h2386
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h2387, _, _, _, _, _) => {
            h2387
        }
    }), f539(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2388, _, _) => {
            h2388
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h2389, _, _, _, _) => {
            h2389
        }
    }), f539(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2390, _) => {
            h2390
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h2391, _, _, _) => {
            h2391
        }
    }), f539(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h2392) => {
            h2392
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h2393, _, _) => {
            h2393
        }
    })))
}

pub fn f511(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f540(v0.clone(), v1, v2, v3)?, f540(v0.clone(), v1, v2, {
        let a2394 = v3;
        let a2395 = 4u64;
        nat_add(a2394, a2395)?
    })?, f540(v0.clone(), v1, v2, {
        let a2396 = v3;
        let a2397 = 8u64;
        nat_add(a2396, a2397)?
    })?, f540(v0.clone(), v1, v2, {
        let a2398 = v3;
        let a2399 = 12u64;
        nat_add(a2398, a2399)?
    })?, f540(v0.clone(), v1, v2, {
        let a2400 = v3;
        let a2401 = 16u64;
        nat_add(a2400, a2401)?
    })?, f540(v0.clone(), v1, v2, {
        let a2402 = v3;
        let a2403 = 20u64;
        nat_add(a2402, a2403)?
    })?, f540(v0.clone(), v1, v2, {
        let a2404 = v3;
        let a2405 = 24u64;
        nat_add(a2404, a2405)?
    })?, f540(v0.clone(), v1, v2, {
        let a2406 = v3;
        let a2407 = 28u64;
        nat_add(a2406, a2407)?
    })?, f540(v0.clone(), v1, v2, {
        let a2408 = v3;
        let a2409 = 32u64;
        nat_add(a2408, a2409)?
    })?, f540(v0.clone(), v1, v2, {
        let a2410 = v3;
        let a2411 = 36u64;
        nat_add(a2410, a2411)?
    })?, f540(v0.clone(), v1, v2, {
        let a2412 = v3;
        let a2413 = 40u64;
        nat_add(a2412, a2413)?
    })?, f540(v0.clone(), v1, v2, {
        let a2414 = v3;
        let a2415 = 44u64;
        nat_add(a2414, a2415)?
    })?, f540(v0.clone(), v1, v2, {
        let a2416 = v3;
        let a2417 = 48u64;
        nat_add(a2416, a2417)?
    })?, f540(v0.clone(), v1, v2, {
        let a2418 = v3;
        let a2419 = 52u64;
        nat_add(a2418, a2419)?
    })?, f540(v0.clone(), v1, v2, {
        let a2420 = v3;
        let a2421 = 56u64;
        nat_add(a2420, a2421)?
    })?, f540(v0, v1, v2, {
        let a2422 = v3;
        let a2423 = 60u64;
        nat_add(a2422, a2423)?
    })?))
}

pub fn f512(p2426: List<Bytes>, v1: Bytes) -> bool {
    let m2425 = p2426;
    {
        let mut m2425 = m2425;
        loop {
            let step = {
                let v0 = m2425;
                let m2424 = v0;
                match m2424.uncons() {
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
                Ok(next) => { m2425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(v0: Bytes, v1: u64) -> Bytes {
    let m2430 = {
        let a2427 = v0;
        let a2428 = v1;
        let a2429 = 1u64;
        slice_bytes(a2427, a2428, a2429)
    };
    match m2430 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f514(v0: Fn14, p2435: u64, p2436: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m2434 = (p2435, p2436);
    {
        let mut m2434 = m2434;
        loop {
            let step = {
                let (v1, v2) = m2434;
                let m2431 = v1;
                if m2431 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2431 - 1;
                    let m2433 = {
                        let c2432: Fn14 = v0.clone();
                        c2432.apply(v2.clone())?
                    };
                    match m2433 {
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
                Ok(next) => { m2434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f541(v1, v0, v2)
}

pub fn f516(p2439: List<u64>, p2440: List<u64>) -> List<u64> {
    let m2438 = (p2439, p2440);
    {
        let mut m2438 = m2438;
        loop {
            let step = {
                let (v0, v1) = m2438;
                let m2437 = v0;
                match m2437.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: List<u64>) -> R<Bytes> {
    let m2441 = v0;
    match m2441.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a2442 = f542(v1)?;
            let a2443 = f517(v2)?;
            Ok(append_bytes(a2442, a2443))
        }
    }
}

pub fn f518(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m2444 = v0;
    match m2444.uncons() {
        None => {
            let a2445 = v2;
            let a2446 = 0u64;
            if nat_eq(a2445, a2446) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a2449 = {
                    let a2447 = v3;
                    let a2448 = v1;
                    nat_mul(a2447, a2448)?
                };
                let a2450 = v2;
                nat_add(a2449, a2450)?
            };
            Ok(List::cons({
                let a2451 = v5;
                let a2452 = f519();
                let a2453 = 0u64;
                nat_rem(a2451, a2452, a2453)
            }, f518(v4, v1, {
                let a2454 = v5;
                let a2455 = f519();
                let a2456 = 0u64;
                nat_quot(a2454, a2455, a2456)
            })?))
        }
    }
}

pub fn f519() -> u64 {
    1000000000u64
}

pub fn f520(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2501 = {
            let a2477 = {
                let a2465 = {
                    let a2459 = {
                        let a2457 = v1;
                        let a2458 = 128u8;
                        fixed_u8::bit_and(a2457, a2458)
                    };
                    let a2460 = 128u8;
                    if equal(a2459, a2460) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2466 = {
                    let a2463 = {
                        let a2461 = v1;
                        let a2462 = 64u8;
                        fixed_u8::bit_and(a2461, a2462)
                    };
                    let a2464 = 64u8;
                    if equal(a2463, a2464) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2465, a2466)?
            };
            let a2478 = {
                let a2475 = {
                    let a2469 = {
                        let a2467 = v1;
                        let a2468 = 32u8;
                        fixed_u8::bit_and(a2467, a2468)
                    };
                    let a2470 = 32u8;
                    if equal(a2469, a2470) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2476 = {
                    let a2473 = {
                        let a2471 = v1;
                        let a2472 = 16u8;
                        fixed_u8::bit_and(a2471, a2472)
                    };
                    let a2474 = 16u8;
                    if equal(a2473, a2474) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2475, a2476)?
            };
            nat_add(a2477, a2478)?
        };
        let a2502 = {
            let a2499 = {
                let a2487 = {
                    let a2481 = {
                        let a2479 = v1;
                        let a2480 = 8u8;
                        fixed_u8::bit_and(a2479, a2480)
                    };
                    let a2482 = 8u8;
                    if equal(a2481, a2482) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2488 = {
                    let a2485 = {
                        let a2483 = v1;
                        let a2484 = 4u8;
                        fixed_u8::bit_and(a2483, a2484)
                    };
                    let a2486 = 4u8;
                    if equal(a2485, a2486) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2487, a2488)?
            };
            let a2500 = {
                let a2497 = {
                    let a2491 = {
                        let a2489 = v1;
                        let a2490 = 2u8;
                        fixed_u8::bit_and(a2489, a2490)
                    };
                    let a2492 = 2u8;
                    if equal(a2491, a2492) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2498 = {
                    let a2495 = {
                        let a2493 = v1;
                        let a2494 = 1u8;
                        fixed_u8::bit_and(a2493, a2494)
                    };
                    let a2496 = 1u8;
                    if equal(a2495, a2496) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2497, a2498)?
            };
            nat_add(a2499, a2500)?
        };
        nat_add(a2501, a2502)?
    };
    let a2509 = f513(f543(), {
        let a2503 = v2;
        let a2504 = 16u64;
        let a2505 = 0u64;
        nat_quot(a2503, a2504, a2505)
    });
    let a2510 = f513(f543(), {
        let a2506 = v2;
        let a2507 = 16u64;
        let a2508 = 0u64;
        nat_rem(a2506, a2507, a2508)
    });
    Ok(append_bytes(a2509, a2510))
}

pub fn f521(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f544(Fn19::F545, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f522(v0: Fn15, p2514: List<List<(Bytes, Bytes)>>, p2515: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2513 = (p2514, p2515);
    {
        let mut m2513 = m2513;
        loop {
            let step = {
                let (v1, v2) = m2513;
                let m2511 = v2;
                match m2511.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2512: Fn15 = v0.clone();
                            c2512.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f524(v0: Fn16, p2520: u64, p2521: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2519 = (p2520, p2521);
    {
        let mut m2519 = m2519;
        loop {
            let step = {
                let (v1, v2) = m2519;
                let m2516 = v1;
                if m2516 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2516 - 1;
                    let m2518 = {
                        let c2517: Fn16 = v0.clone();
                        c2517.apply(v2.clone())?
                    };
                    match m2518 {
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
                Ok(next) => { m2519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f546(v0)
}

pub fn f526(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f547(v0, v1)
}

pub fn f527(v0: u8) -> bool {
    let a2522 = v0;
    let a2523 = 32u8;
    if if if equal(a2522, a2523) {
        true
    } else {
        let a2524 = v0;
        let a2525 = 9u8;
        equal(a2524, a2525)
    } {
        true
    } else {
        let a2526 = v0;
        let a2527 = 10u8;
        equal(a2526, a2527)
    } {
        true
    } else {
        let a2528 = v0;
        let a2529 = 12u8;
        if equal(a2528, a2529) {
            true
        } else {
            let a2530 = v0;
            let a2531 = 13u8;
            equal(a2530, a2531)
        }
    }
}

pub fn f528(v0: Fn17, p2536: u64, p2537: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2535 = (p2536, p2537);
    {
        let mut m2535 = m2535;
        loop {
            let step = {
                let (v1, v2) = m2535;
                let m2532 = v1;
                if m2532 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2532 - 1;
                    let m2534 = {
                        let c2533: Fn17 = v0.clone();
                        c2533.apply(v2.clone())?
                    };
                    match m2534 {
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
                Ok(next) => { m2535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f548(v0, v1)
}

pub fn f530(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2582 = {
            let a2558 = {
                let a2546 = {
                    let a2540 = {
                        let a2538 = v1;
                        let a2539 = 128u8;
                        fixed_u8::bit_and(a2538, a2539)
                    };
                    let a2541 = 128u8;
                    if equal(a2540, a2541) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2547 = {
                    let a2544 = {
                        let a2542 = v1;
                        let a2543 = 64u8;
                        fixed_u8::bit_and(a2542, a2543)
                    };
                    let a2545 = 64u8;
                    if equal(a2544, a2545) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2546, a2547)?
            };
            let a2559 = {
                let a2556 = {
                    let a2550 = {
                        let a2548 = v1;
                        let a2549 = 32u8;
                        fixed_u8::bit_and(a2548, a2549)
                    };
                    let a2551 = 32u8;
                    if equal(a2550, a2551) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2557 = {
                    let a2554 = {
                        let a2552 = v1;
                        let a2553 = 16u8;
                        fixed_u8::bit_and(a2552, a2553)
                    };
                    let a2555 = 16u8;
                    if equal(a2554, a2555) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2556, a2557)?
            };
            nat_add(a2558, a2559)?
        };
        let a2583 = {
            let a2580 = {
                let a2568 = {
                    let a2562 = {
                        let a2560 = v1;
                        let a2561 = 8u8;
                        fixed_u8::bit_and(a2560, a2561)
                    };
                    let a2563 = 8u8;
                    if equal(a2562, a2563) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2569 = {
                    let a2566 = {
                        let a2564 = v1;
                        let a2565 = 4u8;
                        fixed_u8::bit_and(a2564, a2565)
                    };
                    let a2567 = 4u8;
                    if equal(a2566, a2567) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2568, a2569)?
            };
            let a2581 = {
                let a2578 = {
                    let a2572 = {
                        let a2570 = v1;
                        let a2571 = 2u8;
                        fixed_u8::bit_and(a2570, a2571)
                    };
                    let a2573 = 2u8;
                    if equal(a2572, a2573) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2579 = {
                    let a2576 = {
                        let a2574 = v1;
                        let a2575 = 1u8;
                        fixed_u8::bit_and(a2574, a2575)
                    };
                    let a2577 = 1u8;
                    if equal(a2576, a2577) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2578, a2579)?
            };
            nat_add(a2580, a2581)?
        };
        nat_add(a2582, a2583)?
    };
    let a2584 = 48u64;
    let a2585 = v2;
    if nat_le(a2584, a2585) {
        let a2586 = v2;
        let a2587 = 57u64;
        Ok(nat_le(a2586, a2587))
    } else {
        Ok(false)
    }
}

pub fn f531(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f434({
        let (_, h2588) = {
            let (h2589, _) = f549(Fn20::F550, {
                let a2591 = {
                    let a2590 = v0.clone();
                    length_list(a2590)
                };
                let a2592 = 1u64;
                nat_add(a2591, a2592)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2589
        };
        h2588
    }))
}

pub fn f532(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f533(v0: Adt1, v1: Bytes) -> R<bool> {
    if f67(v0.clone(), f497(v1.clone())) {
        let m2593 = f95(f64(v0, f497(v1)))?;
        match m2593.uncons() {
            None => {
                Ok(false)
            }
            Some((v2, _)) => {
                let m2594 = f43(v2)?;
                match m2594 {
                    None => {
                        Ok(false)
                    }
                    Some(v4) => {
                        let m2595 = f93(v4, f8(Str::lit("reader")));
                        match m2595 {
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

pub fn f534(p2599: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m2598 = p2599;
    {
        let mut m2598 = m2598;
        loop {
            let step = {
                let v0 = m2598;
                let m2596 = v0;
                match m2596.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2597 = f93(v3, f8(Str::lit("std")));
                            match m2597 {
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
                Ok(next) => { m2598 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f535(v0: List<Adt2>) -> List<Bytes> {
    let m2600 = v0;
    match m2600.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2601 = f93(v1, f8(Str::lit("std")));
            match m2601 {
                None => {
                    f535(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f535(v2))
                }
            }
        }
    }
}

pub fn f536(v0: List<Adt2>) -> List<Bytes> {
    let m2602 = v0;
    match m2602.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2603 = f92(f93(v1, f8(Str::lit("req"))));
            match m2603 {
                None => {
                    f536(v2)
                }
                Some(v3) => {
                    List::cons(v3, f536(v2))
                }
            }
        }
    }
}

pub fn f537(v0: Fn18, p2607: u64, p2608: Adt25) -> R<Adt25> {
    let m2606 = (p2607, p2608);
    {
        let mut m2606 = m2606;
        loop {
            let step = {
                let (v1, v2) = m2606;
                let m2604 = v1;
                if m2604 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m2604 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
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

pub fn f538(v0: Adt25) -> R<Adt25> {
    f551(v0)
}

pub fn f539(v0: u32, v1: u32) -> u32 {
    let m2611 = {
        let a2609 = f552(v0);
        let a2610 = f552(v1);
        fixed_u64::checked_add(a2609, a2610)
    };
    match m2611 {
        None => {
            0u32
        }
        Some(v2) => {
            f553(v2)
        }
    }
}

pub fn f540(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2614 = {
        let a2612 = v3;
        let a2613 = 8u64;
        nat_add(a2612, a2613)?
    };
    let a2615 = v2;
    if nat_eq(a2614, a2615) {
        Ok(f554({
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
            Ok(f554({
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
            Ok(f555(f556(v0.clone(), v1, v3), f556(v0.clone(), v1, {
                let a2630 = v3;
                let a2631 = 1u64;
                nat_add(a2630, a2631)?
            }), f556(v0.clone(), v1, {
                let a2632 = v3;
                let a2633 = 2u64;
                nat_add(a2632, a2633)?
            }), f556(v0, v1, {
                let a2634 = v3;
                let a2635 = 3u64;
                nat_add(a2634, a2635)?
            })))
        }
    }
}

pub fn f541(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2636, _) = v2;
        h2636
    };
    let a2637 = v3;
    let a2638 = 0u64;
    if nat_eq(a2637, a2638) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2639 = v3;
            let a2640 = v1;
            let a2641 = 0u64;
            nat_quot(a2639, a2640, a2641)
        }, {
            let a2646 = f513(v0, {
                let a2642 = v3;
                let a2643 = v1;
                let a2644 = 0u64;
                nat_rem(a2642, a2643, a2644)
            });
            let a2647 = {
                let (_, h2645) = v2;
                h2645
            };
            append_bytes(a2646, a2647)
        }))
    }
}

pub fn f542(v0: u64) -> R<Bytes> {
    let v1: Bytes = f386(v0)?;
    let a2651 = f473({
        let a2649 = 9u64;
        let a2650 = {
            let a2648 = v1.clone();
            length_bytes(a2648)
        };
        nat_sub(a2649, a2650)
    });
    let a2652 = v1;
    Ok(append_bytes(a2651, a2652))
}

pub fn f543() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f544(v0: Fn19, p2656: List<List<(Bytes, Bytes)>>, p2657: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2655 = (p2656, p2657);
    {
        let mut m2655 = m2655;
        loop {
            let step = {
                let (v1, v2) = m2655;
                let m2653 = v2;
                match m2653.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2654: Fn19 = v0.clone();
                            c2654.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f545(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f546(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2659 = {
        let a2658 = v0.clone();
        length_list(a2658)
    };
    let a2660 = 1u64;
    if nat_le(a2659, a2660) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f557(v0)?))
    }
}

pub fn f547(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2664 = {
        let a2662 = v0.clone();
        let a2663 = {
            let (h2661, _) = v1.clone();
            h2661
        };
        index_bytes(a2662, a2663)
    };
    match m2664 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2666 = {
                    let (h2665, _) = v1.clone();
                    h2665
                };
                let a2667 = 1u64;
                nat_add(a2666, a2667)?
            }, {
                let a2670 = {
                    let (_, h2668) = v1;
                    h2668
                };
                let a2671 = f474(v0, {
                    let (h2669, _) = v1;
                    h2669
                }, v2)?;
                append_bytes(a2670, a2671)
            })))
        }
    }
}

pub fn f548(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2672, _) = v1;
        h2672
    };
    let m2674 = {
        let (_, h2673) = v1;
        h2673
    };
    match m2674 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2677 = {
                let a2675 = v0.clone();
                let a2676 = v2;
                index_bytes(a2675, a2676)
            };
            match m2677 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2682 = {
                        let a2680 = v0;
                        let a2681 = {
                            let a2678 = v2;
                            let a2679 = 1u64;
                            nat_add(a2678, a2679)?
                        };
                        index_bytes(a2680, a2681)
                    };
                    match m2682 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f558(v4)? {
                                f558(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2683 = v2;
                                    let a2684 = 2u64;
                                    nat_add(a2683, a2684)?
                                }, Some({
                                    let a2689 = v3;
                                    let a2690 = f513(f559(), {
                                        let a2687 = {
                                            let a2685 = f560(v4)?;
                                            let a2686 = 16u64;
                                            nat_mul(a2685, a2686)?
                                        };
                                        let a2688 = f560(v5)?;
                                        nat_add(a2687, a2688)?
                                    });
                                    append_bytes(a2689, a2690)
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

pub fn f549(v0: Fn20, p2695: u64, p2696: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2694 = (p2695, p2696);
    {
        let mut m2694 = m2694;
        loop {
            let step = {
                let (v1, v2) = m2694;
                let m2691 = v1;
                if m2691 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2691 - 1;
                    let m2693 = {
                        let c2692: Fn20 = v0.clone();
                        c2692.apply(v2.clone())?
                    };
                    match m2693 {
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
                Ok(next) => { m2694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f561(v0)
}

pub fn f551(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f562(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h2697) => {
            h2697
        }
    });
    let v2: u32 = f539(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h2698, _, _) => {
            h2698
        }
    }, f539(f563(match v0.clone() {
        Adt25::C0(_, _, _, _, h2699, _, _, _, _, _) => {
            h2699
        }
    }), f539(f564(match v0.clone() {
        Adt25::C0(_, _, _, _, h2700, _, _, _, _, _) => {
            h2700
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2701, _, _, _, _) => {
            h2701
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2702, _, _, _) => {
            h2702
        }
    }), f539(v1, {
        let m2705 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h2704, _) => {
                h2704
            }
        };
        match m2705 {
            Adt24::C0(h2703, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2703
            }
        }
    }))));
    let v3: u32 = f539(f565(match v0.clone() {
        Adt25::C0(h2706, _, _, _, _, _, _, _, _, _) => {
            h2706
        }
    }), f566(match v0.clone() {
        Adt25::C0(h2707, _, _, _, _, _, _, _, _, _) => {
            h2707
        }
    }, match v0.clone() {
        Adt25::C0(_, h2708, _, _, _, _, _, _, _, _) => {
            h2708
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2709, _, _, _, _, _, _, _) => {
            h2709
        }
    }));
    Ok(Adt25::C0(f539(v2, v3), match v0.clone() {
        Adt25::C0(h2710, _, _, _, _, _, _, _, _, _) => {
            h2710
        }
    }, match v0.clone() {
        Adt25::C0(_, h2711, _, _, _, _, _, _, _, _) => {
            h2711
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2712, _, _, _, _, _, _, _) => {
            h2712
        }
    }, f539(match v0.clone() {
        Adt25::C0(_, _, _, h2713, _, _, _, _, _, _) => {
            h2713
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h2714, _, _, _, _, _) => {
            h2714
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2715, _, _, _, _) => {
            h2715
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2716, _, _, _) => {
            h2716
        }
    }, f567(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h2717, _) => {
            h2717
        }
    }), {
        let a2719 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h2718) => {
                h2718
            }
        };
        let a2720 = 1u64;
        nat_add(a2719, a2720)?
    }))
}

pub fn f552(v0: u32) -> u64 {
    let m2722 = {
        let a2721 = v0;
        fixed_u64::convert(i128::from(a2721))
    };
    match m2722 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f553(v0: u64) -> u32 {
    let m2726 = {
        let a2725 = {
            let a2723 = v0;
            let a2724 = 4294967295u64;
            fixed_u64::bit_and(a2723, a2724)
        };
        fixed_u32::convert(i128::from(a2725))
    };
    match m2726 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f554(v0: u64) -> u32 {
    let (_, h2727) = {
        let (_, h2728) = f568(Fn21::F569, 32u64, (v0, (1u32, 0u32)));
        h2728
    };
    h2727
}

pub fn f555(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2733 = f570(v0, 24u32);
    let a2734 = {
        let a2731 = f570(v1, 16u32);
        let a2732 = {
            let a2729 = f570(v2, 8u32);
            let a2730 = v3;
            fixed_u32::bit_or(a2729, a2730)
        };
        fixed_u32::bit_or(a2731, a2732)
    };
    fixed_u32::bit_or(a2733, a2734)
}

pub fn f556(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2735 = v2;
    let a2736 = v1;
    if nat_lt(a2735, a2736) {
        let m2739 = {
            let a2737 = v0;
            let a2738 = v2;
            index_bytes(a2737, a2738)
        };
        match m2739 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2741 = {
                    let a2740 = v3;
                    fixed_u32::convert(i128::from(a2740))
                };
                match m2741 {
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
        let a2742 = v2;
        let a2743 = v1;
        if nat_eq(a2742, a2743) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f557(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f521({
        let (_, h2744) = {
            let (h2745, _) = f571(Fn22::F572, {
                let a2747 = {
                    let a2746 = v0.clone();
                    length_list(a2746)
                };
                let a2748 = 1u64;
                nat_add(a2747, a2748)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2745
        };
        h2744
    }))
}

pub fn f558(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2793 = {
            let a2769 = {
                let a2757 = {
                    let a2751 = {
                        let a2749 = v1;
                        let a2750 = 128u8;
                        fixed_u8::bit_and(a2749, a2750)
                    };
                    let a2752 = 128u8;
                    if equal(a2751, a2752) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2758 = {
                    let a2755 = {
                        let a2753 = v1;
                        let a2754 = 64u8;
                        fixed_u8::bit_and(a2753, a2754)
                    };
                    let a2756 = 64u8;
                    if equal(a2755, a2756) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2757, a2758)?
            };
            let a2770 = {
                let a2767 = {
                    let a2761 = {
                        let a2759 = v1;
                        let a2760 = 32u8;
                        fixed_u8::bit_and(a2759, a2760)
                    };
                    let a2762 = 32u8;
                    if equal(a2761, a2762) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2768 = {
                    let a2765 = {
                        let a2763 = v1;
                        let a2764 = 16u8;
                        fixed_u8::bit_and(a2763, a2764)
                    };
                    let a2766 = 16u8;
                    if equal(a2765, a2766) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2767, a2768)?
            };
            nat_add(a2769, a2770)?
        };
        let a2794 = {
            let a2791 = {
                let a2779 = {
                    let a2773 = {
                        let a2771 = v1;
                        let a2772 = 8u8;
                        fixed_u8::bit_and(a2771, a2772)
                    };
                    let a2774 = 8u8;
                    if equal(a2773, a2774) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2780 = {
                    let a2777 = {
                        let a2775 = v1;
                        let a2776 = 4u8;
                        fixed_u8::bit_and(a2775, a2776)
                    };
                    let a2778 = 4u8;
                    if equal(a2777, a2778) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2779, a2780)?
            };
            let a2792 = {
                let a2789 = {
                    let a2783 = {
                        let a2781 = v1;
                        let a2782 = 2u8;
                        fixed_u8::bit_and(a2781, a2782)
                    };
                    let a2784 = 2u8;
                    if equal(a2783, a2784) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2790 = {
                    let a2787 = {
                        let a2785 = v1;
                        let a2786 = 1u8;
                        fixed_u8::bit_and(a2785, a2786)
                    };
                    let a2788 = 1u8;
                    if equal(a2787, a2788) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2789, a2790)?
            };
            nat_add(a2791, a2792)?
        };
        nat_add(a2793, a2794)?
    };
    let a2795 = 48u64;
    let a2796 = v2;
    if if nat_le(a2795, a2796) {
        let a2797 = v2;
        let a2798 = 57u64;
        nat_le(a2797, a2798)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2799 = 97u64;
        let a2800 = v2;
        if nat_le(a2799, a2800) {
            let a2801 = v2;
            let a2802 = 102u64;
            Ok(nat_le(a2801, a2802))
        } else {
            Ok(false)
        }
    }
}

pub fn f559() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f560(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2847 = {
            let a2823 = {
                let a2811 = {
                    let a2805 = {
                        let a2803 = v1;
                        let a2804 = 128u8;
                        fixed_u8::bit_and(a2803, a2804)
                    };
                    let a2806 = 128u8;
                    if equal(a2805, a2806) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2812 = {
                    let a2809 = {
                        let a2807 = v1;
                        let a2808 = 64u8;
                        fixed_u8::bit_and(a2807, a2808)
                    };
                    let a2810 = 64u8;
                    if equal(a2809, a2810) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2811, a2812)?
            };
            let a2824 = {
                let a2821 = {
                    let a2815 = {
                        let a2813 = v1;
                        let a2814 = 32u8;
                        fixed_u8::bit_and(a2813, a2814)
                    };
                    let a2816 = 32u8;
                    if equal(a2815, a2816) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2822 = {
                    let a2819 = {
                        let a2817 = v1;
                        let a2818 = 16u8;
                        fixed_u8::bit_and(a2817, a2818)
                    };
                    let a2820 = 16u8;
                    if equal(a2819, a2820) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2821, a2822)?
            };
            nat_add(a2823, a2824)?
        };
        let a2848 = {
            let a2845 = {
                let a2833 = {
                    let a2827 = {
                        let a2825 = v1;
                        let a2826 = 8u8;
                        fixed_u8::bit_and(a2825, a2826)
                    };
                    let a2828 = 8u8;
                    if equal(a2827, a2828) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2834 = {
                    let a2831 = {
                        let a2829 = v1;
                        let a2830 = 4u8;
                        fixed_u8::bit_and(a2829, a2830)
                    };
                    let a2832 = 4u8;
                    if equal(a2831, a2832) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2833, a2834)?
            };
            let a2846 = {
                let a2843 = {
                    let a2837 = {
                        let a2835 = v1;
                        let a2836 = 2u8;
                        fixed_u8::bit_and(a2835, a2836)
                    };
                    let a2838 = 2u8;
                    if equal(a2837, a2838) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2844 = {
                    let a2841 = {
                        let a2839 = v1;
                        let a2840 = 1u8;
                        fixed_u8::bit_and(a2839, a2840)
                    };
                    let a2842 = 1u8;
                    if equal(a2841, a2842) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2843, a2844)?
            };
            nat_add(a2845, a2846)?
        };
        nat_add(a2847, a2848)?
    };
    let a2849 = v2;
    let a2850 = 57u64;
    if nat_le(a2849, a2850) {
        let a2851 = v2;
        let a2852 = 48u64;
        Ok(nat_sub(a2851, a2852))
    } else {
        let a2853 = v2;
        let a2854 = 87u64;
        Ok(nat_sub(a2853, a2854))
    }
}

pub fn f561(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2855, _) = v0;
        h2855
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2856) = v0;
        h2856
    };
    let m2857 = v1;
    match m2857.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2858 = v4;
            match m2858.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f573(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f562(v0: u64) -> u32 {
    let a2859 = v0;
    let a2860 = 32u64;
    if nat_lt(a2859, a2860) {
        let a2861 = v0;
        let a2862 = 16u64;
        if nat_lt(a2861, a2862) {
            let a2863 = v0;
            let a2864 = 8u64;
            if nat_lt(a2863, a2864) {
                let a2865 = v0;
                let a2866 = 4u64;
                if nat_lt(a2865, a2866) {
                    let a2867 = v0;
                    let a2868 = 2u64;
                    if nat_lt(a2867, a2868) {
                        let a2869 = v0;
                        let a2870 = 1u64;
                        if nat_lt(a2869, a2870) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2871 = v0;
                        let a2872 = 3u64;
                        if nat_lt(a2871, a2872) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2873 = v0;
                    let a2874 = 6u64;
                    if nat_lt(a2873, a2874) {
                        let a2875 = v0;
                        let a2876 = 5u64;
                        if nat_lt(a2875, a2876) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2877 = v0;
                        let a2878 = 7u64;
                        if nat_lt(a2877, a2878) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2879 = v0;
                let a2880 = 12u64;
                if nat_lt(a2879, a2880) {
                    let a2881 = v0;
                    let a2882 = 10u64;
                    if nat_lt(a2881, a2882) {
                        let a2883 = v0;
                        let a2884 = 9u64;
                        if nat_lt(a2883, a2884) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2885 = v0;
                        let a2886 = 11u64;
                        if nat_lt(a2885, a2886) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2887 = v0;
                    let a2888 = 14u64;
                    if nat_lt(a2887, a2888) {
                        let a2889 = v0;
                        let a2890 = 13u64;
                        if nat_lt(a2889, a2890) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2891 = v0;
                        let a2892 = 15u64;
                        if nat_lt(a2891, a2892) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2893 = v0;
            let a2894 = 24u64;
            if nat_lt(a2893, a2894) {
                let a2895 = v0;
                let a2896 = 20u64;
                if nat_lt(a2895, a2896) {
                    let a2897 = v0;
                    let a2898 = 18u64;
                    if nat_lt(a2897, a2898) {
                        let a2899 = v0;
                        let a2900 = 17u64;
                        if nat_lt(a2899, a2900) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2901 = v0;
                        let a2902 = 19u64;
                        if nat_lt(a2901, a2902) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2903 = v0;
                    let a2904 = 22u64;
                    if nat_lt(a2903, a2904) {
                        let a2905 = v0;
                        let a2906 = 21u64;
                        if nat_lt(a2905, a2906) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2907 = v0;
                        let a2908 = 23u64;
                        if nat_lt(a2907, a2908) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2909 = v0;
                let a2910 = 28u64;
                if nat_lt(a2909, a2910) {
                    let a2911 = v0;
                    let a2912 = 26u64;
                    if nat_lt(a2911, a2912) {
                        let a2913 = v0;
                        let a2914 = 25u64;
                        if nat_lt(a2913, a2914) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2915 = v0;
                        let a2916 = 27u64;
                        if nat_lt(a2915, a2916) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2917 = v0;
                    let a2918 = 30u64;
                    if nat_lt(a2917, a2918) {
                        let a2919 = v0;
                        let a2920 = 29u64;
                        if nat_lt(a2919, a2920) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2921 = v0;
                        let a2922 = 31u64;
                        if nat_lt(a2921, a2922) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2923 = v0;
        let a2924 = 48u64;
        if nat_lt(a2923, a2924) {
            let a2925 = v0;
            let a2926 = 40u64;
            if nat_lt(a2925, a2926) {
                let a2927 = v0;
                let a2928 = 36u64;
                if nat_lt(a2927, a2928) {
                    let a2929 = v0;
                    let a2930 = 34u64;
                    if nat_lt(a2929, a2930) {
                        let a2931 = v0;
                        let a2932 = 33u64;
                        if nat_lt(a2931, a2932) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2933 = v0;
                        let a2934 = 35u64;
                        if nat_lt(a2933, a2934) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2935 = v0;
                    let a2936 = 38u64;
                    if nat_lt(a2935, a2936) {
                        let a2937 = v0;
                        let a2938 = 37u64;
                        if nat_lt(a2937, a2938) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2939 = v0;
                        let a2940 = 39u64;
                        if nat_lt(a2939, a2940) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2941 = v0;
                let a2942 = 44u64;
                if nat_lt(a2941, a2942) {
                    let a2943 = v0;
                    let a2944 = 42u64;
                    if nat_lt(a2943, a2944) {
                        let a2945 = v0;
                        let a2946 = 41u64;
                        if nat_lt(a2945, a2946) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2947 = v0;
                        let a2948 = 43u64;
                        if nat_lt(a2947, a2948) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2949 = v0;
                    let a2950 = 46u64;
                    if nat_lt(a2949, a2950) {
                        let a2951 = v0;
                        let a2952 = 45u64;
                        if nat_lt(a2951, a2952) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2953 = v0;
                        let a2954 = 47u64;
                        if nat_lt(a2953, a2954) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2955 = v0;
            let a2956 = 56u64;
            if nat_lt(a2955, a2956) {
                let a2957 = v0;
                let a2958 = 52u64;
                if nat_lt(a2957, a2958) {
                    let a2959 = v0;
                    let a2960 = 50u64;
                    if nat_lt(a2959, a2960) {
                        let a2961 = v0;
                        let a2962 = 49u64;
                        if nat_lt(a2961, a2962) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2963 = v0;
                        let a2964 = 51u64;
                        if nat_lt(a2963, a2964) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2965 = v0;
                    let a2966 = 54u64;
                    if nat_lt(a2965, a2966) {
                        let a2967 = v0;
                        let a2968 = 53u64;
                        if nat_lt(a2967, a2968) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2969 = v0;
                        let a2970 = 55u64;
                        if nat_lt(a2969, a2970) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2971 = v0;
                let a2972 = 60u64;
                if nat_lt(a2971, a2972) {
                    let a2973 = v0;
                    let a2974 = 58u64;
                    if nat_lt(a2973, a2974) {
                        let a2975 = v0;
                        let a2976 = 57u64;
                        if nat_lt(a2975, a2976) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2977 = v0;
                        let a2978 = 59u64;
                        if nat_lt(a2977, a2978) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2979 = v0;
                    let a2980 = 62u64;
                    if nat_lt(a2979, a2980) {
                        let a2981 = v0;
                        let a2982 = 61u64;
                        if nat_lt(a2981, a2982) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2983 = v0;
                        let a2984 = 63u64;
                        if nat_lt(a2983, a2984) {
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

pub fn f563(v0: u32) -> u32 {
    let a2987 = {
        let a2985 = f574(v0, 6u32, 26u32);
        let a2986 = f574(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2985, a2986)
    };
    let a2988 = f574(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2987, a2988)
}

pub fn f564(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2994 = {
        let a2989 = v0;
        let a2990 = v1;
        fixed_u32::bit_and(a2989, a2990)
    };
    let a2995 = {
        let a2992 = {
            let a2991 = v0;
            fixed_u32::bit_not(a2991)
        };
        let a2993 = v2;
        fixed_u32::bit_and(a2992, a2993)
    };
    fixed_u32::bit_xor(a2994, a2995)
}

pub fn f565(v0: u32) -> u32 {
    let a2998 = {
        let a2996 = f574(v0, 2u32, 30u32);
        let a2997 = f574(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2996, a2997)
    };
    let a2999 = f574(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2998, a2999)
}

pub fn f566(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3008 = {
        let a3004 = {
            let a3000 = v0;
            let a3001 = v1;
            fixed_u32::bit_and(a3000, a3001)
        };
        let a3005 = {
            let a3002 = v0;
            let a3003 = v2;
            fixed_u32::bit_and(a3002, a3003)
        };
        fixed_u32::bit_xor(a3004, a3005)
    };
    let a3009 = {
        let a3006 = v1;
        let a3007 = v2;
        fixed_u32::bit_and(a3006, a3007)
    };
    fixed_u32::bit_xor(a3008, a3009)
}

pub fn f567(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h3010, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3010
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h3011, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3011
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h3012, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3012
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h3013, _, _, _, _, _, _, _, _, _, _, _) => {
            h3013
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h3014, _, _, _, _, _, _, _, _, _, _) => {
            h3014
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h3015, _, _, _, _, _, _, _, _, _) => {
            h3015
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h3016, _, _, _, _, _, _, _, _) => {
            h3016
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h3017, _, _, _, _, _, _, _) => {
            h3017
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3018, _, _, _, _, _, _) => {
            h3018
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h3019, _, _, _, _, _) => {
            h3019
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h3020, _, _, _, _) => {
            h3020
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3021, _, _, _) => {
            h3021
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3022, _, _) => {
            h3022
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3023, _) => {
            h3023
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3024) => {
            h3024
        }
    }, f539(f575(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3025, _) => {
            h3025
        }
    }), f539(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3026, _, _, _, _, _, _) => {
            h3026
        }
    }, f539(f576(match v0.clone() {
        Adt24::C0(_, h3027, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3027
        }
    }), match v0 {
        Adt24::C0(h3028, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3028
        }
    }))))
}

pub fn f568(v0: Fn21, p3032: u64, p3033: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3031 = (p3032, p3033);
    {
        let mut m3031 = m3031;
        loop {
            let step = {
                let (v1, v2) = m3031;
                let m3029 = v1;
                if m3029 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3029 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3030: Fn21 = v0.clone();
                        c3030.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3031 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3034, _) = v0;
        h3034
    };
    let v2: u32 = {
        let (h3035, _) = {
            let (_, h3036) = v0;
            h3036
        };
        h3035
    };
    let v3: u32 = {
        let (_, h3037) = {
            let (_, h3038) = v0;
            h3038
        };
        h3037
    };
    ({
        let a3039 = v1;
        let a3040 = 2u64;
        let a3041 = 0u64;
        nat_quot(a3039, a3040, a3041)
    }, (f570(v2, 1u32), {
        let a3045 = {
            let a3042 = v1;
            let a3043 = 2u64;
            let a3044 = 0u64;
            nat_rem(a3042, a3043, a3044)
        };
        let a3046 = 1u64;
        if nat_eq(a3045, a3046) {
            let a3047 = v3;
            let a3048 = v2;
            fixed_u32::bit_or(a3047, a3048)
        } else {
            v3
        }
    }))
}

pub fn f570(v0: u32, v1: u32) -> u32 {
    let m3051 = {
        let a3049 = v0;
        let a3050 = v1;
        fixed_u32::shift_left(a3049, a3050)
    };
    match m3051 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f571(v0: Fn22, p3056: u64, p3057: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3055 = (p3056, p3057);
    {
        let mut m3055 = m3055;
        loop {
            let step = {
                let (v1, v2) = m3055;
                let m3052 = v1;
                if m3052 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3052 - 1;
                    let m3054 = {
                        let c3053: Fn22 = v0.clone();
                        c3053.apply(v2.clone())?
                    };
                    match m3054 {
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
                Ok(next) => { m3055 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f577(v0)
}

pub fn f573(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3062 = {
            let a3060 = {
                let a3058 = v0.clone();
                length_list(a3058)
            };
            let a3061 = {
                let a3059 = v1.clone();
                length_list(a3059)
            };
            nat_add(a3060, a3061)?
        };
        let a3063 = 1u64;
        nat_add(a3062, a3063)?
    };
    Ok(f439({
        let (_, h3064) = {
            let (_, h3065) = {
                let (h3066, _) = f578(Fn23::F579, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3066
            };
            h3065
        };
        h3064
    }))
}

pub fn f574(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3067 = f509(v0, v1);
    let a3068 = f570(v0, v2);
    fixed_u32::bit_or(a3067, a3068)
}

pub fn f575(v0: u32) -> u32 {
    let a3071 = {
        let a3069 = f574(v0, 17u32, 15u32);
        let a3070 = f574(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3069, a3070)
    };
    let a3072 = f509(v0, 10u32);
    fixed_u32::bit_xor(a3071, a3072)
}

pub fn f576(v0: u32) -> u32 {
    let a3075 = {
        let a3073 = f574(v0, 7u32, 25u32);
        let a3074 = f574(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3073, a3074)
    };
    let a3076 = f509(v0, 3u32);
    fixed_u32::bit_xor(a3075, a3076)
}

pub fn f577(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3077, _) = v0;
        h3077
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3078) = v0;
        h3078
    };
    let m3079 = v1;
    match m3079.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3080 = v4;
            match m3080.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f580(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f578(v0: Fn23, p3085: u64, p3086: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3084 = (p3085, p3086);
    {
        let mut m3084 = m3084;
        loop {
            let step = {
                let (v1, v2) = m3084;
                let m3081 = v1;
                if m3081 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3081 - 1;
                    let m3083 = {
                        let c3082: Fn23 = v0.clone();
                        c3082.apply(v2.clone())
                    };
                    match m3083 {
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
                Ok(next) => { m3084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f581(v0)
}

pub fn f580(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3091 = {
            let a3089 = {
                let a3087 = v0.clone();
                length_list(a3087)
            };
            let a3090 = {
                let a3088 = v1.clone();
                length_list(a3088)
            };
            nat_add(a3089, a3090)?
        };
        let a3092 = 1u64;
        nat_add(a3091, a3092)?
    };
    Ok(f582({
        let (_, h3093) = {
            let (_, h3094) = {
                let (h3095, _) = f583(Fn24::F584, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3095
            };
            h3094
        };
        h3093
    }))
}

pub fn f581(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3096, _) = v0.clone();
        h3096
    };
    let v2: List<(Bytes, ())> = {
        let (h3097, _) = {
            let (_, h3098) = v0.clone();
            h3098
        };
        h3097
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3099) = {
            let (_, h3100) = v0;
            h3100
        };
        h3099
    };
    let m3101 = v1.clone();
    match m3101.uncons() {
        None => {
            let m3102 = v2;
            match m3102.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3103 = v2.clone();
            match m3103.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f585({
                        let (h3104, _) = v8.clone();
                        h3104
                    }, {
                        let (h3105, _) = v6.clone();
                        h3105
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

pub fn f582(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f397(Fn5::F586, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f583(v0: Fn24, p3110: u64, p3111: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3109 = (p3110, p3111);
    {
        let mut m3109 = m3109;
        loop {
            let step = {
                let (v1, v2) = m3109;
                let m3106 = v1;
                if m3106 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3106 - 1;
                    let m3108 = {
                        let c3107: Fn24 = v0.clone();
                        c3107.apply(v2.clone())
                    };
                    match m3108 {
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
                Ok(next) => { m3109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f587(v0)
}

pub fn f585(v0: Bytes, v1: Bytes) -> bool {
    let a3116 = {
        let a3112 = v0;
        let a3113 = v1;
        compare_bytes(a3112, a3113)
    };
    let a3117 = {
        let a3114 = Bytes::lit(&[]);
        let a3115 = Bytes::lit(&[0x01]);
        compare_bytes(a3114, a3115)
    };
    equal(a3116, a3117)
}

pub fn f586(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f587(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3118, _) = v0.clone();
        h3118
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3119, _) = {
            let (_, h3120) = v0.clone();
            h3120
        };
        h3119
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3121) = {
            let (_, h3122) = v0;
            h3122
        };
        h3121
    };
    let m3123 = v1.clone();
    match m3123.uncons() {
        None => {
            let m3124 = v2;
            match m3124.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3125 = v2.clone();
            match m3125.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f585({
                        let (h3126, _) = v8.clone();
                        h3126
                    }, {
                        let (h3127, _) = v6.clone();
                        h3127
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
