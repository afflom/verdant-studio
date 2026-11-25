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
    F59(u8, Bytes),
    F193(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F59(k0, k1) => f59(*k0, k1.clone(), p0),
            Fn0::F193(k0) => f193(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F170,
    F189,
    F285,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F170 => f170(p0, p1),
            Fn1::F189 => f189(p0, p1),
            Fn1::F285 => f285(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F225(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F225(k0) => f225(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F275(Bytes, u64, Bytes, u64),
    F326(Bytes),
    F397(Bytes),
    F402(Bytes, Bytes),
    F403(Bytes),
    F404(Bytes, u64),
    F418(u64, Bytes),
    F465(bool, u64, Bytes),
    F519(Bytes),
    F520(Bytes),
    F556(Bytes),
    F599(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F275(k0, k1, k2, k3) => f275(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F326(k0) => f326(k0.clone(), p0),
            Fn3::F397(k0) => Ok(f397(k0.clone(), p0)),
            Fn3::F402(k0, k1) => f402(k0.clone(), k1.clone(), p0),
            Fn3::F403(k0) => f403(k0.clone(), p0),
            Fn3::F404(k0, k1) => f404(k0.clone(), *k1, p0),
            Fn3::F418(k0, k1) => f418(*k0, k1.clone(), p0),
            Fn3::F465(k0, k1, k2) => f465(*k0, *k1, k2.clone(), p0),
            Fn3::F519(k0) => f519(k0.clone(), p0),
            Fn3::F520(k0) => f520(k0.clone(), p0),
            Fn3::F556(k0) => f556(k0.clone(), p0),
            Fn3::F599(k0) => f599(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F318(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn4::F318(k0, k1, k2) => f318(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F325(Bytes),
    F338(u64, Bytes),
    F438(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F325(k0) => f325(k0.clone(), p0),
            Fn5::F338(k0, k1) => Ok(f338(*k0, k1.clone(), p0)),
            Fn5::F438(k0) => f438(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F377,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F377 => f377(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F380,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F380 => f380(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F382,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F382 => f382(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F385,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F385 => f385(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F440,
    F657,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F440 => f440(p0, p1),
            Fn10::F657 => f657(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F443,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F443 => f443(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F446,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F446 => f446(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F449,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F449 => f449(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F454,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn14::F454 => f454(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F472(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F472(k0) => f472(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F507(bool, Bytes),
    F522(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F507(k0, k1) => f507(*k0, k1.clone(), p0),
            Fn16::F522(k0) => f522(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F531,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F531 => f531(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F533,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F533 => f533(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F535,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F535 => f535(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F543,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F543 => f543(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F573,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F573 => f573(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F631,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F631 => f631(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F633,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F633 => f633(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F635(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F635(k0) => f635(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F656,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F656 => f656(p0),
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
            Ok(Adt11::C4(f35(v9)))
        }
    }
}

pub fn f28(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m177 = v0;
    match m177.uncons() {
        None => {
            Ok(Adt11::C4(f36()))
        }
        Some((v2, v3)) => {
            if f37(v2.clone(), f8(Str::lit("check"))) {
                f38(v1)
            } else {
                if f37(v2.clone(), f8(Str::lit("next"))) {
                    f39(v1, v3)
                } else {
                    if f37(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f40(v1))
                    } else {
                        if f37(v2.clone(), f8(Str::lit("publish"))) {
                            f41(v1)
                        } else {
                            if f37(v2.clone(), f8(Str::lit("agenda"))) {
                                f42(v1)
                            } else {
                                let m178 = v3;
                                match m178.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f36()))
                                    }
                                    Some((v4, v5)) => {
                                        if f37(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f43(v1, v4)
                                        } else {
                                            if f37(v2.clone(), f8(Str::lit("read-xml"))) {
                                                f44(v1, v4)
                                            } else {
                                                if f37(v2.clone(), f8(Str::lit("read-xsd"))) {
                                                    f45(v1, v4)
                                                } else {
                                                    if f37(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                        f46(v1.clone(), v4.clone(), f47(v1, v4)?)
                                                    } else {
                                                        if f37(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                            f46(v1.clone(), v4.clone(), f48(v1, v4, false)?)
                                                        } else {
                                                            if f37(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                                f46(v1.clone(), v4.clone(), f48(v1, v4, true)?)
                                                            } else {
                                                                if f37(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                                    f46(v1.clone(), v4.clone(), f49(v1, v4, false)?)
                                                                } else {
                                                                    if f37(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                        f46(v1.clone(), v4.clone(), f49(v1, v4, true)?)
                                                                    } else {
                                                                        if f37(v2.clone(), f8(Str::lit("admit"))) {
                                                                            f50(v1, v4)
                                                                        } else {
                                                                            if f37(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                                f46(v1.clone(), v4.clone(), f51(v1, v4)?)
                                                                            } else {
                                                                                if f37(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                                    f52(v1, v4)
                                                                                } else {
                                                                                    if f37(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                        f53(v1, v4)
                                                                                    } else {
                                                                                        if f37(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                            f54(v1, v4)
                                                                                        } else {
                                                                                            if f37(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                                f55(v1, v4)
                                                                                            } else {
                                                                                                if f37(v2, f8(Str::lit("propose-encoding"))) {
                                                                                                    let m179 = v5;
                                                                                                    match m179.uncons() {
                                                                                                        None => {
                                                                                                            Ok(Adt11::C4(f36()))
                                                                                                        }
                                                                                                        Some((v6, v7)) => {
                                                                                                            f56(v1, v4, v6, v7)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    Ok(Adt11::C4(f36()))
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
}

pub fn f29(v0: u8) -> R<Str> {
    let m228 = {
        let a226 = f57();
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
        let (h230, _) = f58(Fn0::F59(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f60(List::cons(f32(v0, v4, {
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

pub fn f33(p247: List<Bytes>) -> R<Option<Bytes>> {
    let m246 = p247;
    {
        let mut m246 = m246;
        loop {
            let step = {
                let v0 = m246;
                let m245 = v0;
                match m245.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f61(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m248 = v0;
    match m248.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h249, _) = v1;
                h249
            }, f34(v2))
        }
    }
}

pub fn f35(v0: Bytes) -> Bytes {
    let a254 = {
        let a250 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a250)
    };
    let a255 = {
        let a252 = v0;
        let a253 = {
            let a251 = Str::lit(" is not\u{a}");
            utf8_encode(a251)
        };
        append_bytes(a252, a253)
    };
    append_bytes(a254, a255)
}

pub fn f36() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | read-xml FILE | read-xsd FILE | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f37(v0: Bytes, v1: Bytes) -> bool {
    let a256 = v0;
    let a257 = v1;
    equal(a256, a257)
}

pub fn f38(v0: Adt1) -> R<Adt11> {
    let m258 = f62(v0.clone(), f63());
    match m258.uncons() {
        None => {
            let m259 = f64(v0.clone(), f63())?;
            match m259.uncons() {
                None => {
                    let m262 = f65(match v0.clone() {
                        Adt1::C0(_, h260, _) => {
                            h260
                        }
                    }, f66(f67(), f68(match v0.clone() {
                        Adt1::C0(_, h261, _) => {
                            h261
                        }
                    }, false)));
                    match m262.uncons() {
                        None => {
                            let m264 = f62(v0.clone(), f68(match v0.clone() {
                                Adt1::C0(_, h263, _) => {
                                    h263
                                }
                            }, true));
                            match m264.uncons() {
                                None => {
                                    let m266 = f69(v0.clone(), f68(match v0.clone() {
                                        Adt1::C0(_, h265, _) => {
                                            h265
                                        }
                                    }, true))?;
                                    match m266 {
                                        None => {
                                            let v10: (bool, Bytes) = f70(Adt13::C0(Adt12::C0(f71(f72(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f73(v0.clone())?, f74(f75(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f71(f72(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f74(f75(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f75(v0.clone(), f8(Str::lit("model/admissions.json")))?, f75(v0.clone(), f8(Str::lit("model/strata.json")))?, f76(f75(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f68(match v0 {
                                                Adt1::C0(_, h267, _) => {
                                                    h267
                                                }
                                            }, true)))?;
                                            let (h268, _) = v10;
                                            if h268 {
                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                    let (_, h269) = v10;
                                                    h269
                                                }))
                                            } else {
                                                Ok(Adt11::C4({
                                                    let (_, h270) = v10;
                                                    h270
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

pub fn f39(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f77(f78(match v0.clone() {
        Adt1::C0(_, h271, _) => {
            h271
        }
    }, f79())) {
        Ok(Adt11::C1(List::cons(f79(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f80(f81(f79(), f82(match v0.clone() {
            Adt1::C0(_, h272, _) => {
                h272
            }
        }, f79())))?;
        let m273 = f62(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m273.uncons() {
            None => {
                let m274 = f83(v0.clone(), f84(f85(f72(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f86(v2, v1.clone()))?;
                match m274 {
                    None => {
                        f87(v0, v1)
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

pub fn f40(v0: Adt1) -> Adt11 {
    if f77(f88(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h275) => {
            h275
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f89(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f77(f88(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h276) => {
                h276
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f89(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f41(v0: Adt1) -> R<Adt11> {
    let m277 = f90(v0.clone());
    match m277.uncons() {
        None => {
            let m278 = f66(f62(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f91(), List::<Bytes>::nil())))), f92(v0.clone())?);
            match m278.uncons() {
                None => {
                    Ok(Adt11::C3(f93(f94(v0.clone())?, List::cons((f95(), f96()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f97()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f98()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f99()), List::cons((f8(Str::lit("docs/catalog.md")), f100(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f101(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f102(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f103()), List::<(Bytes, Bytes)>::nil()))))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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

pub fn f42(v0: Adt1) -> R<Adt11> {
    let m279 = f90(v0.clone());
    match m279.uncons() {
        None => {
            let m280 = f66(f62(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f92(v0.clone())?);
            match m280.uncons() {
                None => {
                    let v5: Adt15 = f104(v0.clone())?;
                    let v6: List<Adt14> = f75(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f66(f105(f106(match v5.clone() {
                        Adt15::C0(h281, _, _) => {
                            h281
                        }
                    }, f107(f75(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f108(v6.clone())), f109(f106(match v5.clone() {
                        Adt15::C0(h282, _, _) => {
                            h282
                        }
                    }, f107(f75(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f110(v5.clone(), v6, f111(v5, f112(v0.clone(), f113(v0)?)?))?)))))
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

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m283 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m283.uncons() {
        None => {
            let v4: Bytes = f72(v0.clone(), v1);
            let v5: List<List<Bytes>> = f114(v4.clone())?;
            if f77(f115(f116(v5.clone()), f117())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f118(Str::lit("id"), v6.clone())?, List::cons(f118(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f118(Str::lit("artifact"), f119())?, List::cons(f118(Str::lit("checksum"), f120(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f119(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f121(f72(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f122(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f123(f124(f125(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f44(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m284 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m284.uncons() {
        None => {
            let m285 = f126(f72(v0, v1))?;
            match m285 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f127(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f128(f129(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f45(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m286 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m286.uncons() {
        None => {
            let m287 = f130(f72(v0, v1))?;
            match m287 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f128({
                        let (h288, _) = v5;
                        h288
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h289) = v5;
                        h289
                    }, List::<Bytes>::nil())))))))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v4, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f46(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m290 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m290.uncons() {
        None => {
            if f131(f85(f72(v0, v1))?)? {
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

pub fn f47(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m291 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m291.uncons() {
        None => {
            let m292 = f132(f85(f72(v0.clone(), v1.clone()))?)?;
            match m292 {
                None => {
                    Ok(Adt11::C4(f133()))
                }
                Some(v4) => {
                    let m293 = f134(f85(f72(v0, v1))?)?;
                    match m293 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f135(v5)))
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

pub fn f48(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m294 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m294.uncons() {
        None => {
            let v5: Bytes = f136(if v2 {
                f137()
            } else {
                f138()
            }, f139(v1.clone())?);
            if f77(f140(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m295 = f141(f85(f72(v0.clone(), v1.clone()))?, v2)?;
                match m295 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a297 = {
                            let a296 = v6;
                            length_list(a296)
                        };
                        let a298 = 0u64;
                        if nat_eq(a297, a298) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f72(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f49(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m299 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m299.uncons() {
        None => {
            let v5: List<Bytes> = f85(f72(v0.clone(), v1.clone()))?;
            if f77(f140(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f142(v5)?
                } else {
                    f143(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f136(f144(), f139(v1.clone())?), f72(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f136(f144(), f139(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f145()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m300 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m300.uncons() {
        None => {
            let v4: List<Bytes> = f85(f72(v0.clone(), v1))?;
            let m301 = f146(v4.clone(), f8(Str::lit("requirement")))?;
            match m301 {
                None => {
                    Ok(Adt11::C4(f145()))
                }
                Some(v5) => {
                    let m302 = f146(v4, f8(Str::lit("admission")))?;
                    match m302 {
                        None => {
                            Ok(Adt11::C4(f145()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f75(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f75(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f66(f74(v5.clone(), f8(Str::lit("id"))), f74(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f80(f66(f74(v7.clone(), f8(Str::lit("id"))), f74(v8.clone(), f8(Str::lit("construct")))))?;
                            let m303 = f147(f108(v6.clone()), f107(f75(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m303 {
                                None => {
                                    let a305 = {
                                        let a304 = v9.clone();
                                        length_list(a304)
                                    };
                                    let a306 = 0u64;
                                    if nat_eq(a305, a306) {
                                        Ok(Adt11::C4(f145()))
                                    } else {
                                        if f77({
                                            let a309 = {
                                                let a307 = f80(f66(v9.clone(), v10.clone()))?;
                                                length_list(a307)
                                            };
                                            let a310 = {
                                                let a308 = f66(v9.clone(), v10);
                                                length_list(a308)
                                            };
                                            nat_eq(a309, a310)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f149(f8(Str::lit("model/requirements.json")), f150(v7, v5))?, List::cons(f149(f8(Str::lit("model/admissions.json")), f150(v8, v6))?, List::cons(f122(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f123(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f148(v11))
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

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m311 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m311.uncons() {
        None => {
            let m312 = f151(f85(f72(v0, v1))?)?;
            match m312 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f52(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m313 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m313.uncons() {
        None => {
            let m314 = f152(v0.clone(), v1.clone())?;
            match m314 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m315 = f153(v4.clone(), f8(Str::lit("schema")));
                    match m315 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f154(v0, f155(v5.clone())?, v5, f156(v4.clone(), f8(Str::lit("name"))), f157(f158(f153(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f53(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m316 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f91(), List::<Bytes>::nil())));
    match m316.uncons() {
        None => {
            let m317 = f159(f85(f72(v0.clone(), v1))?)?;
            match m317 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f75(v0, f91())?;
                    let v6: List<Bytes> = f74(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f80(f74(v5.clone(), f8(Str::lit("at"))))?;
                    let a319 = {
                        let a318 = v6.clone();
                        length_list(a318)
                    };
                    let a320 = 0u64;
                    if if nat_eq(a319, a320) {
                        true
                    } else {
                        f77({
                            let a323 = {
                                let a321 = f80(f66(v6.clone(), v7.clone()))?;
                                length_list(a321)
                            };
                            let a324 = {
                                let a322 = f66(v6.clone(), v7);
                                length_list(a322)
                            };
                            nat_eq(a323, a324)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f149(f91(), f150(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f123(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f54(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m325 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m325.uncons() {
        None => {
            let m326 = f160(v0, v1)?;
            match m326 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f161(f162(Some(v4))) {
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

pub fn f55(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m327 = f62(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m327.uncons() {
        None => {
            let m328 = f160(v0.clone(), v1.clone())?;
            match m328 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f77(f37(f163(v4.clone(), f8(Str::lit("schema"))), f164())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f163(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m329 = f165(v4.clone(), f8(Str::lit("source")));
                            match m329 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f166(f165(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f136(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f136(v9.clone(), f8(Str::lit("binding.json"))), f72(v0.clone(), v1));
                        let m330 = v8;
                        match m330 {
                            None => {
                                f167(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f77(f37(f163(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f167(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f136(v9, f139(f163(v7.clone(), f8(Str::lit("url"))))?);
                                    if f77(f88(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h331) => {
                                            h331
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f163(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m332 = f62(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m332.uncons() {
                                            None => {
                                                if f37(f120(f72(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f167(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f56(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m333 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m333.uncons() {
        None => {
            let m334 = f152(v0.clone(), v1.clone())?;
            match m334 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f154(v0.clone(), f72(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f57() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f58(v0: Fn0, p339: u64, p340: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m338 = (p339, p340);
    {
        let mut m338 = m338;
        loop {
            let step = {
                let (v1, v2) = m338;
                let m335 = v1;
                if m335 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m335 - 1;
                    let m337 = {
                        let c336: Fn0 = v0.clone();
                        c336.apply(v2.clone())?
                    };
                    match m337 {
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
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f168(v1, v0, v2)
}

pub fn f60(v0: List<Bytes>) -> List<Bytes> {
    f169(Fn1::F170, List::<Bytes>::nil(), v0)
}

pub fn f61(v0: Bytes) -> R<bool> {
    f171(v0.clone(), 0u64, {
        let a341 = v0;
        length_bytes(a341)
    })
}

pub fn f62(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m342 = v1;
    match m342.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m344 = f172(match v0.clone() {
                Adt1::C0(h343, _, _) => {
                    h343
                }
            }, v2.clone());
            match m344 {
                None => {
                    List::cons(v2, f62(v0, v3))
                }
                Some(_) => {
                    f62(v0, v3)
                }
            }
        }
    }
}

pub fn f63() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f119(), List::<Bytes>::nil())))))))
}

pub fn f64(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m345 = v1;
    match m345.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f173(v0.clone(), v2.clone()) {
                f140(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m346 = f160(v0.clone(), v2.clone())?;
                match m346 {
                    None => {
                        Ok(List::cons(v2, f64(v0, v3)?))
                    }
                    Some(_) => {
                        f64(v0, v3)
                    }
                }
            } else {
                f64(v0, v3)
            }
        }
    }
}

pub fn f65(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m347 = v1;
    match m347.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f78(v0.clone(), v2.clone()) {
                f65(v0, v3)
            } else {
                List::cons(v2, f65(v0, v3))
            }
        }
    }
}

pub fn f66(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m348 = v0;
    match m348.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f66(v3, v1))
        }
    }
}

pub fn f67() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f68(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m349 = v0;
    match m349.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f66(f174({
                let (h350, _) = v2;
                h350
            }, {
                let (_, h351) = v2;
                h351
            }, v1), f68(v3, v1))
        }
    }
}

pub fn f69(v0: Adt1, p355: List<Bytes>) -> R<Option<Bytes>> {
    let m354 = p355;
    {
        let mut m354 = m354;
        loop {
            let step = {
                let v1 = m354;
                let m352 = v1;
                match m352.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m353 = f175(f72(v0.clone(), v2.clone()), f176())?;
                        match m353 {
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
                Ok(next) => { m354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h356, _, _, _, _) => {
            h356
        }
    };
    let v2: Adt16 = f177(v1.clone())?;
    let v3: List<Bytes> = f178(v1.clone());
    let v4: List<Bytes> = f74(match v0.clone() {
        Adt13::C0(_, _, h357, _, _) => {
            h357
        }
    }, f8(Str::lit("path")));
    let m359 = f179(v2, List::<u64>::nil(), f180(v1.clone(), match v0.clone() {
        Adt13::C0(_, h358, _, _, _) => {
            h358
        }
    }, 0u64)?);
    match m359 {
        None => {
            if f182(f183(match v0.clone() {
                Adt13::C0(_, _, _, _, h361) => {
                    h361
                }
            }, f66(match v0.clone() {
                Adt13::C0(_, _, _, _, h362) => {
                    h362
                }
            }, v4.clone()))?, f183(v4.clone(), f66(match v0.clone() {
                Adt13::C0(_, _, _, _, h363) => {
                    h363
                }
            }, v4.clone()))?, f184(match v0.clone() {
                Adt13::C0(_, _, h364, _, _) => {
                    h364
                }
            })) {
                let m368 = f185(f183(match v0.clone() {
                    Adt13::C0(_, _, _, h365, _) => {
                        h365
                    }
                }, f66(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h366, _) => {
                        h366
                    }
                }))?, f183(v3.clone(), f66(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h367, _) => {
                        h367
                    }
                }))?);
                match m368 {
                    None => {
                        let m371 = f186(f74(match v0 {
                            Adt13::C0(_, h370, _, _, _) => {
                                h370
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m371 {
                            None => {
                                let m372 = f186(v4, List::<Bytes>::nil());
                                match m372 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f181(f66(v3, match v0 {
                            Adt13::C0(_, _, _, h369, _) => {
                                h369
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f181(match v1 {
                Adt12::C0(_, _, _, _, h360) => {
                    h360
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f71(v0: Bytes) -> R<List<Bytes>> {
    f187(f85(v0)?, f8(Str::lit("id = \"")))
}

pub fn f72(v0: Adt1, v1: Bytes) -> Bytes {
    let m374 = f172(match v0 {
        Adt1::C0(h373, _, _) => {
            h373
        }
    }, v1);
    match m374 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m375 = v2;
            match m375 {
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

pub fn f73(v0: Adt1) -> R<List<Bytes>> {
    let m376 = f114(f72(v0, f119()))?;
    match m376.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f124(v2))
        }
    }
}

pub fn f74(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m377 = v0;
    match m377.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f163(v2, v1.clone()), f74(v3, v1))
        }
    }
}

pub fn f75(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f162(f160(v0, v1)?))
}

pub fn f76(v0: List<Adt14>) -> List<Bytes> {
    let m378 = v0;
    match m378.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m379 = f166(Some(v1));
            match m379 {
                None => {
                    f76(v2)
                }
                Some(v3) => {
                    List::cons(v3, f76(v2))
                }
            }
        }
    }
}

pub fn f77(v0: bool) -> bool {
    !v0
}

pub fn f78(p383: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m382 = p383;
    {
        let mut m382 = m382;
        loop {
            let step = {
                let v0 = m382;
                let m380 = v0;
                match m380.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h381, _) = v2;
                            h381
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
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

pub fn f79() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f80(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f188(f169(Fn1::F189, List::<Bytes>::nil(), f190(v0)?)))
}

pub fn f81(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m384 = v1;
    match m384.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f140(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f136(v0.clone(), v2), f81(v0, v3))
            } else {
                f81(v0, v3)
            }
        }
    }
}

pub fn f82(p389: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m388 = p389;
    {
        let mut m388 = m388;
        loop {
            let step = {
                let v0 = m388;
                let m385 = v0;
                match m385.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h386, _) = v2;
                            h386
                        }, v1.clone()) {
                            let (_, h387) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h387)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m388 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Adt1, v1: List<List<Bytes>>, p392: List<Bytes>) -> R<Option<Bytes>> {
    let m391 = p392;
    {
        let mut m391 = m391;
        loop {
            let step = {
                let v2 = m391;
                let m390 = v2;
                match m390.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f191(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(p395: List<Bytes>, p396: List<Bytes>, p397: List<List<Bytes>>) -> List<List<Bytes>> {
    let m394 = (p395, (p396, p397));
    {
        let mut m394 = m394;
        loop {
            let step = {
                let (v0, (v1, v2)) = m394;
                let m393 = v0;
                match m393.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f192(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f37(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f192(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a398 = v0.clone();
        length_bytes(a398)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h399, _) = f58(Fn0::F193(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h399
    };
    let v3: u64 = {
        let (h400, _) = {
            let (_, h401) = v2.clone();
            h401
        };
        h400
    };
    let v4: List<Bytes> = {
        let (_, h402) = {
            let (_, h403) = v2;
            h403
        };
        h402
    };
    let a404 = v3;
    let a405 = v1;
    if nat_lt(a404, a405) {
        Ok(f60(List::cons(f32(v0, v3, {
            let a406 = v1;
            let a407 = v3;
            nat_sub(a406, a407)
        }), v4)))
    } else {
        Ok(f60(v4))
    }
}

pub fn f86(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m408 = v0;
    match m408.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f88(v2.clone(), v1.clone()) {
                f86(v3, v1)
            } else {
                List::cons(v2, f86(v3, v1))
            }
        }
    }
}

pub fn f87(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f77(f78(match v0.clone() {
        Adt1::C0(_, h409, _) => {
            h409
        }
    }, f194())) {
        Ok(Adt11::C1(List::cons(f194(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f86(f80(f81(f194(), f82(match v0.clone() {
            Adt1::C0(_, h410, _) => {
                h410
            }
        }, f194())))?, v1.clone());
        let m411 = f62(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m411.uncons() {
            None => {
                let v5: List<Bytes> = f74(f75(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m412 = f62(v0.clone(), f195(v0.clone(), v2.clone(), v5.clone())?);
                match m412.uncons() {
                    None => {
                        let m413 = f196(v0.clone(), v2, v5)?;
                        match m413 {
                            None => {
                                f197(v0, v1)
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

pub fn f88(v0: Bytes, p416: List<Bytes>) -> bool {
    let m415 = p416;
    {
        let mut m415 = m415;
        loop {
            let step = {
                let v1 = m415;
                let m414 = v1;
                match m414.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f90(v0: Adt1) -> List<Bytes> {
    if f78(match v0.clone() {
        Adt1::C0(_, h417, _) => {
            h417
        }
    }, f138()) {
        if f78(match v0 {
            Adt1::C0(_, h418, _) => {
                h418
            }
        }, f137()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f137(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f138(), List::<Bytes>::nil())
    }
}

pub fn f91() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f92(v0: Adt1) -> R<List<Bytes>> {
    Ok(f62(v0.clone(), List::cons(f119(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f113(v0)?)))))
}

pub fn f93(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m419 = v0;
    match m419.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f93(v3, v1))
        }
    }
}

pub fn f94(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m420 = f75(v0.clone(), f91())?;
    match m420.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f198(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f95() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f96() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f97() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f199(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # main may have moved while this ran, so refresh checks the new pull request at once.\u{a}            gh workflow run refresh.yml --repo \"$GITHUB_REPOSITORY\" --ref main || echo \"::warning::refresh could not be started; it runs on the next push to main\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f98() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: operate starts this workflow\u{a}# right after it opens one, so every regeneration takes this one path.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f199(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f99() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f100(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f104(v0.clone())?;
    let v2: List<Adt14> = f75(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f111(v1.clone(), f112(v0.clone(), f113(v0.clone())?)?);
    let v4: List<List<Bytes>> = f125(f114(f72(v0, f119()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f200(v4.clone(), f124(v4), f201(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f110(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f101(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f202(f104(v0.clone())?, f112(v0.clone(), f113(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f102(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f203(f75(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f204(f75(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f103() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f104(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f73(v0.clone())?, f71(f72(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f74(f75(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f105(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m421 = v0;
    match m421.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f88(v2.clone(), v1.clone()) {
                f105(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f105(v3, v1))
            }
        }
    }
}

pub fn f106(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m422 = v0;
    match m422.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f88(v2.clone(), v1.clone()) {
                f106(v3, v1)
            } else {
                List::cons(v2, f106(v3, v1))
            }
        }
    }
}

pub fn f107(v0: List<Adt14>) -> List<Bytes> {
    let m423 = v0;
    match m423.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f66(f76(f162(f165(v1, f8(Str::lit("defers"))))), f107(v2))
        }
    }
}

pub fn f108(v0: List<Adt14>) -> List<Bytes> {
    let m424 = v0;
    match m424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m425 = f165(v1, f8(Str::lit("std")));
            match m425 {
                None => {
                    f108(v2)
                }
                Some(v3) => {
                    List::cons(f163(v3, f8(Str::lit("row"))), f108(v2))
                }
            }
        }
    }
}

pub fn f109(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m426 = v0;
    match m426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f88(v2.clone(), v1.clone()) {
                f109(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f109(v3, v1))
            }
        }
    }
}

pub fn f110(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f93(f205(v2.clone()), f206(f201(v0, v1.clone(), v2)?));
    f207(f80(f208(v1))?, v3.clone(), {
        let a428 = {
            let a427 = v3;
            length_list(a427)
        };
        let a429 = 1u64;
        nat_add(a428, a429)?
    })
}

pub fn f111(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m430 = v1;
    match m430.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f209(v2.clone(), f8(Str::lit("subject"))) {
                f210(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f111(v0, v3))
            } else {
                f111(v0, v3)
            }
        }
    }
}

pub fn f112(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m431 = v1;
    match m431.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m432 = f141(f85(f72(v0.clone(), v2.clone()))?, f211(v2, f137()))?;
            match m432 {
                None => {
                    f112(v0, v3)
                }
                Some(v4) => {
                    Ok(f150(v4, f112(v0, v3)?))
                }
            }
        }
    }
}

pub fn f113(v0: Adt1) -> R<List<Bytes>> {
    Ok(f66(f80(f212(f138(), f82(match v0.clone() {
        Adt1::C0(_, h433, _) => {
            h433
        }
    }, f138())))?, f80(f212(f137(), f82(match v0 {
        Adt1::C0(_, h434, _) => {
            h434
        }
    }, f137())))?))
}

pub fn f114(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f213(v0.clone(), 0u64, {
        let a435 = v0;
        length_bytes(a435)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a438 = {
            let a437 = match v1.clone() {
                Adt17::C0(_, _, h436, _, _) => {
                    h436
                }
            };
            length_bytes(a437)
        };
        let a439 = 0u64;
        if if nat_eq(a438, a439) {
            let m441 = match v1.clone() {
                Adt17::C0(_, _, _, h440, _) => {
                    h440
                }
            };
            match m441.uncons() {
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
            f214(v1)
        }
    };
    Ok(f215(match v4 {
        Adt17::C0(_, _, _, _, h442) => {
            h442
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f115(p447: List<Bytes>, p448: List<Bytes>) -> bool {
    let m446 = (p447, p448);
    {
        let mut m446 = m446;
        loop {
            let step = {
                let (v0, v1) = m446;
                let m443 = v0;
                match m443.uncons() {
                    None => {
                        let m444 = v1;
                        match m444.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m445 = v1;
                        match m445.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f37(v4, v6) {
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
                Ok(next) => { m446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<List<Bytes>>) -> List<Bytes> {
    let m449 = v0;
    match m449.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f117() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f118(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f216(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f119() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f120(v0: Bytes) -> R<Bytes> {
    Ok(f217(f218(v0)?))
}

pub fn f121(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f219(f84(f85(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f220(f221(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f122(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f222(Adt14::C3(std::rc::Rc::new(f223(f80(f66(v1, f76(f75(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m450 = f224(Fn2::F225(v1), None::<Bytes>, v0);
    match m450 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124(v0: List<List<Bytes>>) -> List<Bytes> {
    let m451 = v0;
    match m451.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m452 = v1;
            match m452.uncons() {
                None => {
                    f124(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f124(v2))
                }
            }
        }
    }
}

pub fn f125(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m453 = v0;
    match m453.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f126(v0: Bytes) -> R<Result<Adt18, Bytes>> {
    let v1: Bytes = if f226(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a455 = {
                let a454 = v0;
                length_bytes(a454)
            };
            let a456 = 3u64;
            nat_sub(a455, a456)
        })
    } else {
        v0
    };
    let v2: Bytes = f227(v1)?;
    let v3: u64 = {
        let a457 = v2.clone();
        length_bytes(a457)
    };
    let v4: u64 = f228(v2.clone())?;
    let a458 = v4;
    let a459 = v3;
    if nat_lt(a458, a459) {
        Ok(Err::<Adt18, Bytes>(f229(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m460 = f230(v2.clone())?;
        match m460 {
            Ok(v6) => {
                let m463 = f231(v2.clone(), v6, {
                    let a461 = v3;
                    let a462 = 2u64;
                    nat_add(a461, a462)?
                })?;
                match m463 {
                    Ok(v8) => {
                        if f232(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt18, Bytes>(f229(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a464 = f233(v2.clone(), v8)?;
                            let a465 = 60u64;
                            if !nat_eq(a464, a465) {
                                Ok(Err::<Adt18, Bytes>(f229(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m468 = f234(v2.clone(), v8, {
                                    let a466 = v3;
                                    let a467 = 2u64;
                                    nat_add(a466, a467)?
                                }, 0u64)?;
                                match m468 {
                                    Ok(v10) => {
                                        let m472 = f231(v2, {
                                            let (_, h469) = v10;
                                            h469
                                        }, {
                                            let a470 = v3;
                                            let a471 = 2u64;
                                            nat_add(a470, a471)?
                                        })?;
                                        match m472 {
                                            Ok(v12) => {
                                                let a473 = v12;
                                                let a474 = v3;
                                                if nat_eq(a473, a474) {
                                                    Ok(Ok::<Adt18, Bytes>({
                                                        let (h475, _) = v10;
                                                        h475
                                                    }))
                                                } else {
                                                    Ok(Err::<Adt18, Bytes>(f229(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
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

pub fn f127(v0: Adt18) -> Bytes {
    let m476 = v0;
    match m476 {
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

pub fn f128(v0: u64) -> R<Bytes> {
    f235(f236(), 10u64, v0)
}

pub fn f129(v0: Adt18) -> R<u64> {
    let m477 = v0;
    match m477 {
        Adt18::C0(_, _, r482) => {
            let v3 = (*r482).clone();
            let a478 = 1u64;
            let a479 = f129(v3)?;
            nat_add(a478, a479)
        }
        Adt18::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt18::C2(_) => {
            Ok(0u64)
        }
        Adt18::C3(r483, r484) => {
            let v8 = (*r483).clone();
            let v9 = (*r484).clone();
            let a480 = f129(v8)?;
            let a481 = f129(v9)?;
            nat_add(a480, a481)
        }
        Adt18::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f130(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m485 = f126(v0)?;
    match m485 {
        Ok(v2) => {
            let m486 = v2;
            match m486 {
                Adt18::C0(v3, r498, r499) => {
                    let v4 = (*r498).clone();
                    let v5 = (*r499).clone();
                    let m487 = f237(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m487 {
                        Ok(v7) => {
                            let m488 = f238(v7.clone(), v3)?;
                            match m488 {
                                Ok(v9) => {
                                    if f239(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m489 = f240(v4, f8(Str::lit("targetNamespace")));
                                            match m489 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m490 = f241(v5, v7, v11)?;
                                        match m490 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f242(v13.clone());
                                                let m491 = f243(v14.clone());
                                                match m491 {
                                                    None => {
                                                        let m492 = f244(v13.clone(), v14.clone());
                                                        match m492 {
                                                            None => {
                                                                let m496 = f245({
                                                                    let a494 = {
                                                                        let a493 = v14.clone();
                                                                        length_list(a493)
                                                                    };
                                                                    let a495 = 1u64;
                                                                    nat_add(a494, a495)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m496 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a497 = v14;
                                                                            length_list(a497)
                                                                        }, f221(f246(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<(u64, Bytes), Bytes>(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(Err::<(u64, Bytes), Bytes>(v12))
                                            }
                                        }
                                    } else {
                                        Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(Err::<(u64, Bytes), Bytes>(v6))
                        }
                    }
                }
                Adt18::C1(_, _, _) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C2(_) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C3(_, _) => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt18::C4 => {
                    Ok(Err::<(u64, Bytes), Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f131(p505: List<Bytes>) -> R<bool> {
    let m504 = p505;
    {
        let mut m504 = m504;
        loop {
            let step = {
                let v0 = m504;
                let m500 = v0;
                match m500.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a502 = {
                            let a501 = f247(v1)?;
                            length_bytes(a501)
                        };
                        let a503 = 0u64;
                        if nat_eq(a502, a503) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m504 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m506 = v0;
    match m506.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f247(v1)?;
            let a508 = {
                let a507 = v3.clone();
                length_bytes(a507)
            };
            let a509 = 0u64;
            if nat_eq(a508, a509) {
                f132(v2)
            } else {
                let m510 = f248(v3.clone())?;
                match m510 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f163(v4.clone(), f8(Str::lit("id")));
                        if f249(f163(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m511 = f132(v2)?;
                            match m511 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f250(v5), {
                                        let a512 = v3;
                                        let a513 = Bytes::lit(&[0x0a]);
                                        append_bytes(a512, a513)
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

pub fn f133() -> Bytes {
    let a514 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a514)
}

pub fn f134(p519: List<Bytes>) -> R<Option<Bytes>> {
    let m518 = p519;
    {
        let mut m518 = m518;
        loop {
            let step = {
                let v0 = m518;
                let m515 = v0;
                match m515.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m516 = f248(f247(v1)?)?;
                        match m516 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m517 = f165(v3.clone(), f8(Str::lit("source")));
                                match m517 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f251(f163(v4.clone(), f8(Str::lit("redistribution"))), f163(v4, f8(Str::lit("url"))))? {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f163(v3, f8(Str::lit("id"))))))
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
                Ok(next) => { m518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: Bytes) -> Bytes {
    let a524 = {
        let a520 = Str::lit("binding ");
        utf8_encode(a520)
    };
    let a525 = {
        let a522 = v0;
        let a523 = {
            let a521 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a521)
        };
        append_bytes(a522, a523)
    };
    append_bytes(a524, a525)
}

pub fn f136(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f137() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f138() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f139(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f140(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a526 = v0.clone();
        length_bytes(a526)
    };
    let v3: u64 = {
        let a527 = v1.clone();
        length_bytes(a527)
    };
    let a528 = v3;
    let a529 = v2;
    if nat_le(a528, a529) {
        let m535 = {
            let a532 = v0;
            let a533 = {
                let a530 = v2;
                let a531 = v3;
                nat_sub(a530, a531)
            };
            let a534 = v3;
            slice_bytes(a532, a533, a534)
        };
        match m535 {
            None => {
                false
            }
            Some(v4) => {
                let a536 = v4;
                let a537 = v1;
                equal(a536, a537)
            }
        }
    } else {
        false
    }
}

pub fn f141(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m538 = v0;
    match m538.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f247(v2)?;
            let a540 = {
                let a539 = v4.clone();
                length_bytes(a539)
            };
            let a541 = 0u64;
            if nat_eq(a540, a541) {
                f141(v3, v1)
            } else {
                let m542 = f248(v4)?;
                match m542 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f252(v5.clone())
                        } else {
                            f253(v5.clone())
                        } {
                            let m543 = f141(v3, v1)?;
                            match m543 {
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

pub fn f142(v0: List<Bytes>) -> R<bool> {
    if f254(f159(v0.clone())?) {
        Ok(f77({
            let a545 = {
                let a544 = f255(f159(v0)?);
                length_list(a544)
            };
            let a546 = 0u64;
            nat_eq(a545, a546)
        }))
    } else {
        Ok(false)
    }
}

pub fn f143(v0: List<Bytes>) -> R<bool> {
    if f254(f146(v0.clone(), f8(Str::lit("requirement")))?) {
        if f254(f146(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f77({
                let a548 = {
                    let a547 = f150(f255(f146(v0.clone(), f8(Str::lit("requirement")))?), f255(f146(v0, f8(Str::lit("admission")))?));
                    length_list(a547)
                };
                let a549 = 0u64;
                nat_eq(a548, a549)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f144() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f145() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f146(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m550 = v0;
    match m550.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f247(v2)?;
            let a552 = {
                let a551 = v4.clone();
                length_bytes(a551)
            };
            let a553 = 0u64;
            if nat_eq(a552, a553) {
                f146(v3, v1)
            } else {
                let m554 = f248(v4)?;
                match m554 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f256(v5.clone());
                        let a556 = {
                            let a555 = v6.clone();
                            length_bytes(a555)
                        };
                        let a557 = 0u64;
                        if nat_eq(a556, a557) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m558 = f146(v3, v1.clone())?;
                            match m558 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f37(v6, v1) {
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

pub fn f147(p561: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m560 = p561;
    {
        let mut m560 = m560;
        loop {
            let step = {
                let v0 = m560;
                let m559 = v0;
                match m559.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f88(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f149(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f222(Adt14::C3(std::rc::Rc::new(f257(v1))))?))
}

pub fn f150(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m562 = v0;
    match m562.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f150(v3, v1))
        }
    }
}

pub fn f151(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m563 = v0;
    match m563.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f247(v1)?;
            let a565 = {
                let a564 = v3.clone();
                length_bytes(a564)
            };
            let a566 = 0u64;
            if nat_eq(a565, a566) {
                f151(v2)
            } else {
                let m567 = f258(v3.clone())?;
                match m567 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f259(v4.clone()) {
                            let m568 = f151(v2)?;
                            match m568 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f194(), List::cons(f8(Str::lit("/")), List::cons(f260(f156(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a569 = v3;
                                        let a570 = Bytes::lit(&[0x0a]);
                                        append_bytes(a569, a570)
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

pub fn f152(v0: Adt1, v1: Bytes) -> R<Option<Adt19>> {
    f258(f72(v0, v1))
}

pub fn f153(p576: Adt19, v1: Bytes) -> Option<Adt19> {
    let m575 = p576;
    {
        let mut m575 = m575;
        loop {
            let step = {
                let v0 = m575;
                let m571 = v0;
                match m571 {
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
                    Adt19::C5(r572) => {
                        let v5 = (*r572).clone();
                        Ok::<Adt19, Option<Adt19>>(v5)
                    }
                    Adt19::C6 => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C7(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C8(v2, r573, r574) => {
                        let v3 = (*r573).clone();
                        let v4 = (*r574).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt19, Option<Adt19>>(Some(v3))
                        } else {
                            Ok::<Adt19, Option<Adt19>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m577 = f62(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m577.uncons() {
        None => {
            let m579 = f147(f108(List::cons(Adt14::C4(std::rc::Rc::new({
                let m578 = f261(v4.clone());
                match m578 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f107(f75(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m579 {
                None => {
                    f262(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f148(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f155(v0: Adt19) -> R<Bytes> {
    let a580 = f263(v0)?;
    let a581 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a580, a581))
}

pub fn f156(v0: Adt19, v1: Bytes) -> Bytes {
    let m582 = f264(f153(v0, v1));
    match m582 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f157(v0: List<Adt19>) -> List<Bytes> {
    let m583 = v0;
    match m583.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m584 = f264(Some(v1));
            match m584 {
                None => {
                    f157(v2)
                }
                Some(v3) => {
                    List::cons(v3, f157(v2))
                }
            }
        }
    }
}

pub fn f158(v0: Option<Adt19>) -> List<Adt19> {
    let m585 = v0;
    match m585 {
        None => {
            List::<Adt19>::nil()
        }
        Some(v1) => {
            f265(v1)
        }
    }
}

pub fn f159(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m586 = v0;
    match m586.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f247(v1)?;
            let a588 = {
                let a587 = v3.clone();
                length_bytes(a587)
            };
            let a589 = 0u64;
            if nat_eq(a588, a589) {
                f159(v2)
            } else {
                let m590 = f248(v3)?;
                match m590 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f209(v4.clone(), f8(Str::lit("agent"))) {
                            if f209(v4.clone(), f8(Str::lit("at"))) {
                                if f209(v4.clone(), f8(Str::lit("quote"))) {
                                    f266(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m591 = f159(v2)?;
                            match m591 {
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

pub fn f160(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f248(f72(v0, v1))
}

pub fn f161(p594: List<Adt14>) -> bool {
    let m593 = p594;
    {
        let mut m593 = m593;
        loop {
            let step = {
                let v0 = m593;
                let m592 = v0;
                match m592.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f37(f163(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f88(f8(Str::lit("studio")), f74(f267(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Option<Adt14>) -> List<Adt14> {
    let m595 = v0;
    match m595 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f268(v1)
        }
    }
}

pub fn f163(v0: Adt14, v1: Bytes) -> Bytes {
    let m596 = f166(f165(v0, v1));
    match m596 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f164() -> Bytes {
    f269()
}

pub fn f165(p602: Adt14, v1: Bytes) -> Option<Adt14> {
    let m601 = p602;
    {
        let mut m601 = m601;
        loop {
            let step = {
                let v0 = m601;
                let m597 = v0;
                match m597 {
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
                    Adt14::C4(r598) => {
                        let v5 = (*r598).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r599, r600) => {
                        let v3 = (*r599).clone();
                        let v4 = (*r600).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Option<Adt14>) -> Option<Bytes> {
    let m603 = v0;
    match m603 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m604 = v1;
            match m604 {
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

pub fn f167(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f121(f72(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f270(v1, v4)?)?), List::cons(f122(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f168(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h605, _) = v2.clone();
        h605
    };
    let v4: u64 = {
        let (h606, _) = {
            let (_, h607) = v2.clone();
            h607
        };
        h606
    };
    let v5: List<Bytes> = {
        let (_, h608) = {
            let (_, h609) = v2;
            h609
        };
        h608
    };
    let m610 = f271(v0.clone(), v3);
    match m610 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a611 = v6;
            let a612 = v1;
            if equal(a611, a612) {
                Ok(Some(({
                    let a613 = v3;
                    let a614 = 1u64;
                    nat_add(a613, a614)?
                }, ({
                    let a615 = v3;
                    let a616 = 1u64;
                    nat_add(a615, a616)?
                }, List::cons(f32(v0, v4, {
                    let a617 = v3;
                    let a618 = v4;
                    nat_sub(a617, a618)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a619 = v3;
                    let a620 = 1u64;
                    nat_add(a619, a620)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f169(v0: Fn1, p624: List<Bytes>, p625: List<Bytes>) -> List<Bytes> {
    let m623 = (p624, p625);
    {
        let mut m623 = m623;
        loop {
            let step = {
                let (v1, v2) = m623;
                let m621 = v2;
                match m621.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c622: Fn1 = v0.clone();
                            c622.apply(v1, v3)
                        }, v4))
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

pub fn f170(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f171(v0: Bytes, p679: u64, p680: u64) -> R<bool> {
    let m678 = (p679, p680);
    {
        let mut m678 = m678;
        loop {
            let step = {
                let (v1, v2) = m678;
                let m626 = v2;
                if m626 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m626 - 1;
                    let m629 = {
                        let a627 = v0.clone();
                        let a628 = v1;
                        index_bytes(a627, a628)
                    };
                    match m629 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f272({
                                let v5: u8 = v4;
                                let a674 = {
                                    let a650 = {
                                        let a638 = {
                                            let a632 = {
                                                let a630 = v5;
                                                let a631 = 128u8;
                                                fixed_u8::bit_and(a630, a631)
                                            };
                                            let a633 = 128u8;
                                            if equal(a632, a633) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a639 = {
                                            let a636 = {
                                                let a634 = v5;
                                                let a635 = 64u8;
                                                fixed_u8::bit_and(a634, a635)
                                            };
                                            let a637 = 64u8;
                                            if equal(a636, a637) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a638, a639)?
                                    };
                                    let a651 = {
                                        let a648 = {
                                            let a642 = {
                                                let a640 = v5;
                                                let a641 = 32u8;
                                                fixed_u8::bit_and(a640, a641)
                                            };
                                            let a643 = 32u8;
                                            if equal(a642, a643) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a649 = {
                                            let a646 = {
                                                let a644 = v5;
                                                let a645 = 16u8;
                                                fixed_u8::bit_and(a644, a645)
                                            };
                                            let a647 = 16u8;
                                            if equal(a646, a647) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a648, a649)?
                                    };
                                    nat_add(a650, a651)?
                                };
                                let a675 = {
                                    let a672 = {
                                        let a660 = {
                                            let a654 = {
                                                let a652 = v5;
                                                let a653 = 8u8;
                                                fixed_u8::bit_and(a652, a653)
                                            };
                                            let a655 = 8u8;
                                            if equal(a654, a655) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a661 = {
                                            let a658 = {
                                                let a656 = v5;
                                                let a657 = 4u8;
                                                fixed_u8::bit_and(a656, a657)
                                            };
                                            let a659 = 4u8;
                                            if equal(a658, a659) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a660, a661)?
                                    };
                                    let a673 = {
                                        let a670 = {
                                            let a664 = {
                                                let a662 = v5;
                                                let a663 = 2u8;
                                                fixed_u8::bit_and(a662, a663)
                                            };
                                            let a665 = 2u8;
                                            if equal(a664, a665) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a671 = {
                                            let a668 = {
                                                let a666 = v5;
                                                let a667 = 1u8;
                                                fixed_u8::bit_and(a666, a667)
                                            };
                                            let a669 = 1u8;
                                            if equal(a668, a669) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a670, a671)?
                                    };
                                    nat_add(a672, a673)?
                                };
                                nat_add(a674, a675)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a676 = v1;
                                    let a677 = 1u64;
                                    nat_add(a676, a677)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
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

pub fn f172(p685: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m684 = p685;
    {
        let mut m684 = m684;
        loop {
            let step = {
                let v0 = m684;
                let m681 = v0;
                match m681.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h682, _) = v2;
                            h682
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h683) = v2;
                                h683
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
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

pub fn f173(v0: Adt1, v1: Bytes) -> bool {
    let m687 = f172(match v0 {
        Adt1::C0(h686, _, _) => {
            h686
        }
    }, v1);
    match m687 {
        None => {
            false
        }
        Some(v2) => {
            let m688 = v2;
            match m688 {
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

pub fn f174(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m689 = v1;
    match m689.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f273(v3.clone())
            } else {
                f77(f273(v3.clone()))
            } {
                List::cons(f136(v0.clone(), v3), f174(v0, v4, v2))
            } else {
                f174(v0, v4, v2)
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a690 = v0.clone();
        length_bytes(a690)
    };
    let v3: u64 = {
        let a691 = v1.clone();
        length_bytes(a691)
    };
    let v4: u64 = {
        let (h692, _) = f274(Fn3::F275(v1.clone(), v3, v0.clone(), v2), {
            let a693 = v2;
            let a694 = 1u64;
            nat_add(a693, a694)?
        }, 0u64)?;
        h692
    };
    if f226(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f176() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f177(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f276(match v0.clone() {
        Adt12::C0(h695, _, _, _, _) => {
            h695
        }
    }, 0u64)?, f276(match v0.clone() {
        Adt12::C0(_, h696, _, _, _) => {
            h696
        }
    }, 0u64)?, f276(match v0.clone() {
        Adt12::C0(_, _, h697, _, _) => {
            h697
        }
    }, 0u64)?, f276(match v0 {
        Adt12::C0(_, _, _, h698, _) => {
            h698
        }
    }, 0u64)?))
}

pub fn f178(v0: Adt12) -> List<Bytes> {
    f66(match v0.clone() {
        Adt12::C0(h699, _, _, _, _) => {
            h699
        }
    }, f66(match v0.clone() {
        Adt12::C0(_, _, h700, _, _) => {
            h700
        }
    }, f66(match v0.clone() {
        Adt12::C0(_, _, _, h701, _) => {
            h701
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h702) => {
            h702
        }
    })))
}

pub fn f179(v0: Adt16, p708: List<u64>, p709: List<Adt20>) -> Option<u64> {
    let m707 = (p708, p709);
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v1, v2) = m707;
                let m703 = v2;
                match m703.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f277(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h704) => {
                                h704
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h705, _) => {
                                    h705
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h706, _) => {
                                    h706
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m707 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m710 = v1;
    match m710.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f278(v0.clone(), v3)?), f180(v0, v4, {
                let a711 = v2;
                let a712 = 1u64;
                nat_add(a711, a712)?
            })?))
        }
    }
}

pub fn f181(p716: List<Bytes>, p717: u64) -> Bytes {
    let m715 = (p716, p717);
    {
        let mut m715 = m715;
        loop {
            let step = {
                let (v0, v1) = m715;
                let m713 = v0;
                match m713.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m714 = v1;
                        if m714 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m714 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f279(v0.clone(), v1.clone()) {
        if f279(v1, v0) {
            f280(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f183(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m718 = v0;
    match m718.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f281(v2, v1.clone())?, f183(v3, v1)?))
        }
    }
}

pub fn f184(v0: List<Adt14>) -> List<u64> {
    let m719 = v0;
    match m719.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f282(v1), f184(v2))
        }
    }
}

pub fn f185(p722: List<u64>, v1: List<u64>) -> Option<u64> {
    let m721 = p722;
    {
        let mut m721 = m721;
        loop {
            let step = {
                let v0 = m721;
                let m720 = v0;
                match m720.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f283(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
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

pub fn f186(p725: List<Bytes>, p726: List<Bytes>) -> Option<Bytes> {
    let m724 = (p725, p726);
    {
        let mut m724 = m724;
        loop {
            let step = {
                let (v0, v1) = m724;
                let m723 = v0;
                match m723.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f88(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m727 = v0;
    match m727.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m728 = f284(v2, v1.clone())?;
            match m728 {
                None => {
                    f187(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f187(v3, v1)?))
                }
            }
        }
    }
}

pub fn f188(v0: List<Bytes>) -> List<Bytes> {
    f169(Fn1::F285, List::<Bytes>::nil(), v0)
}

pub fn f189(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f286(v0, v1)
}

pub fn f190(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f287(f288(f289(v0))?))
}

pub fn f191(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m729 = f160(v0, v2)?;
    match m729 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f290(v1, f163(v3.clone(), f8(Str::lit("id"))))?;
            let m730 = v4.clone();
            match m730.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m732 = f166(f165({
                        let m731 = f165(v3.clone(), f8(Str::lit("source")));
                        match m731 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m732 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f77(f88(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f216(f291(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f192(p735: List<Bytes>, p736: List<Bytes>) -> List<Bytes> {
    let m734 = (p735, p736);
    {
        let mut m734 = m734;
        loop {
            let step = {
                let (v0, v1) = m734;
                let m733 = v0;
                match m733.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f292(v0, v1)
}

pub fn f194() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f195(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m737 = v1;
    match m737.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt19 = f293(v0.clone(), v3)?;
            if f88(f294(f156(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f295(v5)?, f195(v0, v4, v2)?))
            } else {
                f195(v0, v4, v2)
            }
        }
    }
}

pub fn f196(v0: Adt1, p741: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m740 = p741;
    {
        let mut m740 = m740;
        loop {
            let step = {
                let v1 = m740;
                let m738 = v1;
                match m738.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt19 = f293(v0.clone(), v3.clone())?;
                        if if f88(f294(f156(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f37(f155({
                                let m739 = f153(v5.clone(), f8(Str::lit("schema")));
                                match m739 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f72(v0.clone(), f295(v5)?))
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
                Ok(next) => { m740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f77(f78(match v0.clone() {
        Adt1::C0(_, h742, _) => {
            h742
        }
    }, f144())) {
        Ok(Adt11::C1(List::cons(f144(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f86(f80(f296(f82(match v0.clone() {
            Adt1::C0(_, h743, _) => {
                h743
            }
        }, f144())))?, v1);
        let m744 = f62(v0.clone(), List::cons(f91(), List::cons(f119(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m744.uncons() {
            None => {
                let v5: (bool, Bytes) = f297(v0.clone(), v2)?;
                let (h745, _) = v5;
                if h745 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h746) = v5;
                        h746
                    }))
                } else {
                    let m747 = f90(v0.clone());
                    match m747.uncons() {
                        None => {
                            let m748 = f92(v0.clone())?;
                            match m748.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h749) = v5;
                                        h749
                                    }, List::cons(f7(f80(f298(f104(v0.clone())?, f112(v0.clone(), f113(v0.clone())?)?))?), List::cons(f299(v0)?, List::<Bytes>::nil()))))))
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

pub fn f198(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f300(f75(v0, f91())?)?)))
}

pub fn f199() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f200(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt23>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m750 = v1;
    match m750.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f66(f301(v5.clone(), v2.clone()), f302(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f303(v0.clone(), v5.clone()), List::cons(if f88(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a752 = {
                    let a751 = v7.clone();
                    length_list(a751)
                };
                let a753 = 0u64;
                if nat_eq(a752, a753) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f123(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f200(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f201(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt23>> {
    let v3: List<(Bytes, Bytes)> = f304(v1);
    let v4: List<(Bytes, Bytes)> = f305(v0, v2);
    f306(v3.clone(), f93(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f202(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m754 = v1;
    match m754.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f209(v2.clone(), f8(Str::lit("domain"))) {
                f307(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f163(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f163(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m755 = f165(v2.clone(), f8(Str::lit("role")));
                    match m755 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f163(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f163(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f163(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f308(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m756 = f165(v2.clone(), f8(Str::lit("origin")));
                    match m756 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f163(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m757 = f165(v2, f8(Str::lit("evidence")));
                    match m757 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f163(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f202(v0, v3))
            } else {
                f202(v0, v3)
            }
        }
    }
}

pub fn f203(v0: List<Adt14>) -> List<Bytes> {
    let m758 = v0;
    match m758.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f163(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f163(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f163(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f203(v2))
        }
    }
}

pub fn f204(v0: List<Adt14>) -> List<Bytes> {
    let m759 = v0;
    match m759.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f163(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f309(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f204(v2))
        }
    }
}

pub fn f205(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m760 = v0;
    match m760.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f163(v1.clone(), f8(Str::lit("subject"))), f163(v1, f8(Str::lit("object")))), f205(v2))
        }
    }
}

pub fn f206(v0: List<Adt23>) -> List<(Bytes, Bytes)> {
    let m761 = v0;
    match m761.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt23::C0(h762, _, _) => {
                    h762
                }
            }, match v1 {
                Adt23::C0(_, _, h763) => {
                    h763
                }
            }), f206(v2))
        }
    }
}

pub fn f207(p770: List<Bytes>, v1: List<(Bytes, Bytes)>, p771: u64) -> R<List<Bytes>> {
    let m769 = (p770, p771);
    {
        let mut m769 = m769;
        loop {
            let step = {
                let (v0, v2) = m769;
                let m764 = v2;
                if m764 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m764 - 1;
                    let v4: List<Bytes> = f80(f66(v0.clone(), f310(v0.clone(), v1.clone())))?;
                    let a767 = {
                        let a765 = v4.clone();
                        length_list(a765)
                    };
                    let a768 = {
                        let a766 = v0.clone();
                        length_list(a766)
                    };
                    if nat_eq(a767, a768) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: List<Adt14>) -> List<Bytes> {
    let m772 = v0;
    match m772.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m773 = f165(v1, f8(Str::lit("std")));
            match m773 {
                None => {
                    f208(v2)
                }
                Some(v3) => {
                    List::cons(f163(v3.clone(), f8(Str::lit("authority"))), if f209(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f163(v3, f8(Str::lit("row"))), f208(v2))
                    } else {
                        f208(v2)
                    })
                }
            }
        }
    }
}

pub fn f209(v0: Adt14, v1: Bytes) -> bool {
    f77({
        let a775 = {
            let a774 = f163(v0, v1);
            length_bytes(a774)
        };
        let a776 = 0u64;
        nat_eq(a775, a776)
    })
}

pub fn f210(v0: Adt15, v1: Adt14) -> bool {
    if f311(v0.clone(), v1.clone(), f163(v1.clone(), f8(Str::lit("subject")))) {
        f311(v0, v1.clone(), f163(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f211(v0: Bytes, v1: Bytes) -> bool {
    let m781 = {
        let a778 = v0;
        let a779 = 0u64;
        let a780 = {
            let a777 = v1.clone();
            length_bytes(a777)
        };
        slice_bytes(a778, a779, a780)
    };
    match m781 {
        None => {
            false
        }
        Some(v2) => {
            let a782 = v2;
            let a783 = v1;
            equal(a782, a783)
        }
    }
}

pub fn f212(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m784 = v1;
    match m784.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f140(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f136(v0.clone(), v2), f212(v0, v3))
            } else {
                f212(v0, v3)
            }
        }
    }
}

pub fn f213(v0: Bytes, p842: u64, p843: u64, p844: Adt17) -> R<Adt17> {
    let m841 = (p842, (p843, p844));
    {
        let mut m841 = m841;
        loop {
            let step = {
                let (v1, (v2, v3)) = m841;
                let m785 = v2;
                if m785 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m785 - 1;
                    let v5: u64 = f312(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h786, _, _, _) => {
                            h786
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a787 = v1;
                            let a788 = 1u64;
                            nat_add(a787, a788)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h789, _, _) => {
                                h789
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h790, _) => {
                                h790
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h791) => {
                                h791
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h792, _, _, _, _) => {
                                h792
                            }
                        } {
                            let a793 = v5;
                            let a794 = 34u64;
                            if nat_eq(a793, a794) {
                                let a797 = f312(v0.clone(), {
                                    let a795 = v1;
                                    let a796 = 1u64;
                                    nat_add(a795, a796)?
                                })?;
                                let a798 = 34u64;
                                if nat_eq(a797, a798) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a799 = v1;
                                        let a800 = 1u64;
                                        nat_add(a799, a800)?
                                    }, (v4, Adt17::C0(true, true, f313(match v3.clone() {
                                        Adt17::C0(_, _, h801, _, _) => {
                                            h801
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h802, _) => {
                                            h802
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h803) => {
                                            h803
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a804 = v1;
                                        let a805 = 1u64;
                                        nat_add(a804, a805)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h806, _, _) => {
                                            h806
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h807, _) => {
                                            h807
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h808) => {
                                            h808
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a809 = v1;
                                    let a810 = 1u64;
                                    nat_add(a809, a810)?
                                }, (v4, Adt17::C0(true, false, f313(match v3.clone() {
                                    Adt17::C0(_, _, h811, _, _) => {
                                        h811
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h812, _) => {
                                        h812
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h813) => {
                                        h813
                                    }
                                }))))
                            }
                        } else {
                            let a814 = v5;
                            let a815 = 34u64;
                            if nat_eq(a814, a815) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a816 = v1;
                                    let a817 = 1u64;
                                    nat_add(a816, a817)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h818, _, _) => {
                                        h818
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h819, _) => {
                                        h819
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h820) => {
                                        h820
                                    }
                                }))))
                            } else {
                                let a821 = v5;
                                let a822 = 44u64;
                                if nat_eq(a821, a822) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a823 = v1;
                                        let a824 = 1u64;
                                        nat_add(a823, a824)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h825, _, _) => {
                                            h825
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h826, _) => {
                                            h826
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h827) => {
                                            h827
                                        }
                                    }))))
                                } else {
                                    let a828 = v5;
                                    let a829 = 13u64;
                                    if nat_eq(a828, a829) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a830 = v1;
                                            let a831 = 1u64;
                                            nat_add(a830, a831)?
                                        }, (v4, v3)))
                                    } else {
                                        let a832 = v5;
                                        let a833 = 10u64;
                                        if nat_eq(a832, a833) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a834 = v1;
                                                let a835 = 1u64;
                                                nat_add(a834, a835)?
                                            }, (v4, f214(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a836 = v1;
                                                let a837 = 1u64;
                                                nat_add(a836, a837)?
                                            }, (v4, Adt17::C0(false, false, f313(match v3.clone() {
                                                Adt17::C0(_, _, h838, _, _) => {
                                                    h838
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h839, _) => {
                                                    h839
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h840) => {
                                                    h840
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
                Ok(next) => { m841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f314(List::cons(match v0.clone() {
        Adt17::C0(_, _, h845, _, _) => {
            h845
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h846, _) => {
            h846
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h847) => {
            h847
        }
    }))
}

pub fn f215(p850: List<List<Bytes>>, p851: List<List<Bytes>>) -> List<List<Bytes>> {
    let m849 = (p850, p851);
    {
        let mut m849 = m849;
        loop {
            let step = {
                let (v0, v1) = m849;
                let m848 = v0;
                match m848.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f315(v0.clone(), 0u64, {
        let a852 = v0;
        length_bytes(a852)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f217(v0: Adt24) -> Bytes {
    let a873 = f316(match v0.clone() {
        Adt24::C0(h853, _, _, _, _, _, _, _) => {
            h853
        }
    });
    let a874 = {
        let a871 = f316(match v0.clone() {
            Adt24::C0(_, h854, _, _, _, _, _, _) => {
                h854
            }
        });
        let a872 = {
            let a869 = f316(match v0.clone() {
                Adt24::C0(_, _, h855, _, _, _, _, _) => {
                    h855
                }
            });
            let a870 = {
                let a867 = f316(match v0.clone() {
                    Adt24::C0(_, _, _, h856, _, _, _, _) => {
                        h856
                    }
                });
                let a868 = {
                    let a865 = f316(match v0.clone() {
                        Adt24::C0(_, _, _, _, h857, _, _, _) => {
                            h857
                        }
                    });
                    let a866 = {
                        let a863 = f316(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h858, _, _) => {
                                h858
                            }
                        });
                        let a864 = {
                            let a861 = f316(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h859, _) => {
                                    h859
                                }
                            });
                            let a862 = f316(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h860) => {
                                    h860
                                }
                            });
                            append_bytes(a861, a862)
                        };
                        append_bytes(a863, a864)
                    };
                    append_bytes(a865, a866)
                };
                append_bytes(a867, a868)
            };
            append_bytes(a869, a870)
        };
        append_bytes(a871, a872)
    };
    append_bytes(a873, a874)
}

pub fn f218(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a875 = v0.clone();
        length_bytes(a875)
    };
    let v2: u64 = {
        let a881 = {
            let a878 = {
                let a876 = v1;
                let a877 = 72u64;
                nat_add(a876, a877)?
            };
            let a879 = 64u64;
            let a880 = 0u64;
            nat_quot(a878, a879, a880)
        };
        let a882 = 64u64;
        nat_mul(a881, a882)?
    };
    let (_, h883) = f317(Fn4::F318(v0, v1, v2), {
        let a884 = v2;
        let a885 = 64u64;
        let a886 = 0u64;
        nat_quot(a884, a885, a886)
    }, (0u64, f319()))?;
    Ok(h883)
}

pub fn f219(p889: List<List<Bytes>>, v1: Bytes, p890: List<Bytes>) -> R<List<Bytes>> {
    let m888 = (p889, p890);
    {
        let mut m888 = m888;
        loop {
            let step = {
                let (v0, v2) = m888;
                let m887 = v0;
                match m887.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f320(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f221(f321(v3), Bytes::lit(&[0x0a])), v2)))
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

pub fn f220(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f322(v0)?))
}

pub fn f221(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m891 = v0;
    match m891.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m892 = v3.clone();
            match m892.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a895 = v2;
                    let a896 = {
                        let a893 = v1.clone();
                        let a894 = f221(v3, v1);
                        append_bytes(a893, a894)
                    };
                    append_bytes(a895, a896)
                }
            }
        }
    }
}

pub fn f222(v0: Adt14) -> R<Bytes> {
    let a897 = f323(v0)?;
    let a898 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a897, a898))
}

pub fn f223(v0: List<Bytes>) -> Adt14 {
    let m899 = v0;
    match m899.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f223(v2)))
        }
    }
}

pub fn f224(v0: Fn2, p903: Option<Bytes>, p904: List<Bytes>) -> Option<Bytes> {
    let m902 = (p903, p904);
    {
        let mut m902 = m902;
        loop {
            let step = {
                let (v1, v2) = m902;
                let m900 = v2;
                match m900.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c901: Fn2 = v0.clone();
                            c901.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m902 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m905 = v1;
    match m905 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a908 = {
                    let a906 = v3;
                    let a907 = v0;
                    append_bytes(a906, a907)
                };
                let a909 = v2;
                append_bytes(a908, a909)
            })
        }
    }
}

pub fn f226(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m914 = {
        let a911 = v0;
        let a912 = v2;
        let a913 = {
            let a910 = v1.clone();
            length_bytes(a910)
        };
        slice_bytes(a911, a912, a913)
    };
    match m914 {
        None => {
            false
        }
        Some(v3) => {
            let a915 = v3;
            let a916 = v1;
            equal(a915, a916)
        }
    }
}

pub fn f227(v0: Bytes) -> R<Bytes> {
    let (_, h917) = {
        let (h918, _) = f324(Fn5::F325(v0.clone()), {
            let a920 = {
                let a919 = v0;
                length_bytes(a919)
            };
            let a921 = 1u64;
            nat_add(a920, a921)?
        }, (0u64, Bytes::lit(&[])))?;
        h918
    };
    Ok(h917)
}

pub fn f228(v0: Bytes) -> R<u64> {
    let (h922, _) = f274(Fn3::F326(v0.clone()), {
        let a924 = {
            let a923 = v0;
            length_bytes(a923)
        };
        let a925 = 1u64;
        nat_add(a924, a925)?
    }, 0u64)?;
    Ok(h922)
}

pub fn f229(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a926 = v0;
        utf8_encode(a926)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f128(v1)?, List::<Bytes>::nil())))))
}

pub fn f230(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f232(v0.clone(), 0u64, Str::lit("<?xml")) {
        f327(f233(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m928 = f328(v0.clone(), {
            let a927 = Str::lit("?>");
            utf8_encode(a927)
        }, 5u64)?;
        match m928 {
            None => {
                Ok(Err::<u64, Bytes>(f229(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m930 = f328(f32(v0.clone(), 0u64, v1), {
                    let a929 = Str::lit("encoding");
                    utf8_encode(a929)
                }, 5u64)?;
                match m930 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a931 = v1;
                            let a932 = 2u64;
                            nat_add(a931, a932)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f329(v0.clone(), {
                            let a933 = v2;
                            let a934 = 8u64;
                            nat_add(a933, a934)?
                        })?;
                        let v4: u64 = f329(v0.clone(), {
                            let a935 = v3;
                            let a936 = 1u64;
                            nat_add(a935, a936)?
                        })?;
                        let _: u64 = f233(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m939 = f328(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a937 = v4;
                                let a938 = 1u64;
                                nat_add(a937, a938)?
                            })?;
                            match m939 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f330(f32(v0, {
                            let a940 = v4;
                            let a941 = 1u64;
                            nat_add(a940, a941)?
                        }, {
                            let a944 = v7;
                            let a945 = {
                                let a942 = v4;
                                let a943 = 1u64;
                                nat_add(a942, a943)?
                            };
                            nat_sub(a944, a945)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a946 = v1;
                                let a947 = 2u64;
                                nat_add(a946, a947)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f229(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f231(v0: Bytes, p952: u64, p953: u64) -> R<Result<u64, Bytes>> {
    let m951 = (p952, p953);
    {
        let mut m951 = m951;
        loop {
            let step = {
                let (v1, v2) = m951;
                let m948 = v2;
                if m948 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f229(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m948 - 1;
                    let v4: u64 = f329(v0.clone(), v1)?;
                    if f232(v0.clone(), v4, Str::lit("<!--")) {
                        let m949 = f331(v0.clone(), v4)?;
                        match m949 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f232(v0.clone(), v4, Str::lit("<?")) {
                            let m950 = f332(v0.clone(), v4)?;
                            match m950 {
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
                Ok(next) => { m951 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: u64, v2: Str) -> bool {
    f226(v0, {
        let a954 = v2;
        utf8_encode(a954)
    }, v1)
}

pub fn f233(v0: Bytes, v1: u64) -> R<u64> {
    f333(v0, v1)
}

pub fn f234(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m955 = v2;
    if m955 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m955 - 1;
        let a956 = v3;
        let a957 = 0u64;
        if nat_eq(a956, a957) {
            let v5: u64 = f334(v0.clone(), {
                let a958 = v1;
                let a959 = 1u64;
                nat_add(a958, a959)?
            })?;
            let a962 = v5;
            let a963 = {
                let a960 = v1;
                let a961 = 1u64;
                nat_add(a960, a961)?
            };
            if nat_eq(a962, a963) {
                Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a964 = v1;
                    let a965 = 1u64;
                    nat_add(a964, a965)?
                }, {
                    let a968 = v5;
                    let a969 = {
                        let a966 = v1;
                        let a967 = 1u64;
                        nat_add(a966, a967)?
                    };
                    nat_sub(a968, a969)
                });
                let m970 = f335(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m970 {
                    Ok(v8) => {
                        if f232(v0.clone(), {
                            let (_, h971) = v8.clone();
                            h971
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h972, _) = v8;
                                h972
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a974 = {
                                    let (_, h973) = v8;
                                    h973
                                };
                                let a975 = 2u64;
                                nat_add(a974, a975)?
                            })))
                        } else {
                            let m979 = f234(v0.clone(), {
                                let a977 = {
                                    let (_, h976) = v8;
                                    h976
                                };
                                let a978 = 1u64;
                                nat_add(a977, a978)?
                            }, v4, 1u64)?;
                            match m979 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a981 = {
                                            let (_, h980) = v10.clone();
                                            h980
                                        };
                                        let a982 = 2u64;
                                        nat_add(a981, a982)?
                                    };
                                    let v12: u64 = f334(v0.clone(), v11)?;
                                    if !f37(f32(v0.clone(), v11, {
                                        let a983 = v12;
                                        let a984 = v11;
                                        nat_sub(a983, a984)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h985) = v10;
                                            h985
                                        })?))
                                    } else {
                                        let v13: u64 = f329(v0.clone(), v12)?;
                                        let a986 = f233(v0, v13)?;
                                        let a987 = 62u64;
                                        if nat_eq(a986, a987) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h988, _) = v8;
                                                h988
                                            }), std::rc::Rc::new({
                                                let (h989, _) = v10;
                                                h989
                                            })), {
                                                let a990 = v13;
                                                let a991 = 1u64;
                                                nat_add(a990, a991)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
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
            let a992 = f233(v0.clone(), v1)?;
            let a993 = 256u64;
            if nat_eq(a992, a993) {
                Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f232(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f232(v0.clone(), v1, Str::lit("<!--")) {
                        let m994 = f331(v0.clone(), v1)?;
                        match m994 {
                            Ok(v15) => {
                                f234(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt18, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f232(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m998 = f328(v0.clone(), {
                                let a995 = Str::lit("]]>");
                                utf8_encode(a995)
                            }, {
                                let a996 = v1;
                                let a997 = 9u64;
                                nat_add(a996, a997)?
                            })?;
                            match m998 {
                                None => {
                                    Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m1001 = f234(v0.clone(), {
                                        let a999 = v16;
                                        let a1000 = 3u64;
                                        nat_add(a999, a1000)?
                                    }, v4, 1u64)?;
                                    match m1001 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f32(v0, {
                                                let a1002 = v1;
                                                let a1003 = 9u64;
                                                nat_add(a1002, a1003)?
                                            }, {
                                                let a1006 = v16;
                                                let a1007 = {
                                                    let a1004 = v1;
                                                    let a1005 = 9u64;
                                                    nat_add(a1004, a1005)?
                                                };
                                                nat_sub(a1006, a1007)
                                            }))), std::rc::Rc::new({
                                                let (h1008, _) = v18;
                                                h1008
                                            })), {
                                                let (_, h1009) = v18;
                                                h1009
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt18, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f232(v0.clone(), v1, Str::lit("<?")) {
                                let m1010 = f332(v0.clone(), v1)?;
                                match m1010 {
                                    Ok(v20) => {
                                        f234(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f232(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a1011 = f233(v0.clone(), v1)?;
                                    let a1012 = 60u64;
                                    if nat_eq(a1011, a1012) {
                                        let m1013 = f234(v0.clone(), v1, v4, 0u64)?;
                                        match m1013 {
                                            Ok(v22) => {
                                                let m1015 = f234(v0, {
                                                    let (_, h1014) = v22;
                                                    h1014
                                                }, v4, 1u64)?;
                                                match m1015 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h1016, _) = v22;
                                                            h1016
                                                        }), std::rc::Rc::new({
                                                            let (h1017, _) = v24;
                                                            h1017
                                                        })), {
                                                            let (_, h1018) = v24;
                                                            h1018
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
                                        let m1019 = f336(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m1019 {
                                            Ok(v26) => {
                                                let m1021 = f234(v0, {
                                                    let (_, h1020) = v26;
                                                    h1020
                                                }, v4, 1u64)?;
                                                match m1021 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h1022, _) = v26;
                                                            h1022
                                                        })), std::rc::Rc::new({
                                                            let (h1023, _) = v28;
                                                            h1023
                                                        })), {
                                                            let (_, h1024) = v28;
                                                            h1024
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

pub fn f235(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1025 = v2;
    let a1026 = 0u64;
    if nat_eq(a1025, a1026) {
        Ok(f337(v0, 0u64))
    } else {
        let (_, h1027) = {
            let (h1028, _) = f324(Fn5::F338(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1028
        };
        Ok(h1027)
    }
}

pub fn f236() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f237(v0: Adt18, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m1029 = v0;
    match m1029 {
        Adt18::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C1(v2, v3, r1034) => {
            let v4 = (*r1034).clone();
            let m1030 = f237(v4, v1);
            match m1030 {
                Ok(v6) => {
                    if f37(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f37(v3.clone(), f339()) {
                            true
                        } else {
                            f37(v3.clone(), f340())
                        } {
                            Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f211(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a1032 = {
                                    let a1031 = v2;
                                    length_bytes(a1031)
                                };
                                let a1033 = 6u64;
                                nat_sub(a1032, a1033)
                            });
                            if f37(v7.clone(), f8(Str::lit("xmlns"))) {
                                Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f37(v7.clone(), f8(Str::lit("xml"))) {
                                    !f37(v3.clone(), f339())
                                } else {
                                    false
                                } {
                                    Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f37(v3.clone(), f339()) {
                                        true
                                    } else {
                                        f37(v3.clone(), f340())
                                    } {
                                        !f37(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f37(v3.clone(), Bytes::lit(&[])) {
                                            Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
                                        } else {
                                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((v7, v3), v6))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(v6)
                        }
                    }
                }
                Err(v5) => {
                    Err::<List<(Bytes, Bytes)>, Bytes>(v5)
                }
            }
        }
        Adt18::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f238(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f341(v1.clone())?;
    if f37(v2.clone(), Bytes::lit(&[])) {
        let m1035 = f342(v0, Bytes::lit(&[]));
        match m1035 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f37(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f339(), f343(v1)?)))
        } else {
            let m1036 = f342(v0, v2.clone());
            match m1036 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f343(v1)?)))
                }
            }
        }
    }
}

pub fn f239(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f37({
        let (h1037, _) = v0;
        h1037
    }, f344()) {
        f37({
            let (_, h1038) = v0;
            h1038
        }, {
            let a1039 = v1;
            utf8_encode(a1039)
        })
    } else {
        false
    }
}

pub fn f240(p1043: Adt18, v1: Bytes) -> Option<Bytes> {
    let m1042 = p1043;
    {
        let mut m1042 = m1042;
        loop {
            let step = {
                let v0 = m1042;
                let m1040 = v0;
                match m1040 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C1(v2, v3, r1041) => {
                        let v4 = (*r1041).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt18, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt18, Option<Bytes>>(v4)
                        }
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C3(_, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C4 => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m1042 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Adt18, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m1044 = v0;
    match m1044 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C3(r1055, r1056) => {
            let v3 = (*r1055).clone();
            let v4 = (*r1056).clone();
            let m1045 = f241(v4, v1.clone(), v2.clone())?;
            match m1045 {
                Ok(v6) => {
                    let m1046 = v3;
                    match m1046 {
                        Adt18::C0(v7, r1053, r1054) => {
                            let v8 = (*r1053).clone();
                            let v9 = (*r1054).clone();
                            let m1047 = f237(v8.clone(), v1);
                            match m1047 {
                                Ok(v11) => {
                                    let m1048 = f238(v11.clone(), v7)?;
                                    match m1048 {
                                        Ok(v13) => {
                                            if f239(v13, Str::lit("simpleType")) {
                                                let m1049 = f240(v8, f8(Str::lit("name")));
                                                match m1049 {
                                                    None => {
                                                        Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m1050 = f345(v14.clone(), v9, v11, v2)?;
                                                        match m1050 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f260(v14)?, ({
                                                                    let (_, h1051) = v16;
                                                                    h1051
                                                                }, {
                                                                    let (h1052, _) = v16;
                                                                    h1052
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v10))
                                }
                            }
                        }
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v5))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f242(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1057 = v0;
    match m1057.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1058, _) = v1;
                h1058
            }, f242(v2))
        }
    }
}

pub fn f243(p1061: List<Bytes>) -> Option<Bytes> {
    let m1060 = p1061;
    {
        let mut m1060 = m1060;
        loop {
            let step = {
                let v0 = m1060;
                let m1059 = v0;
                match m1059.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f88(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1060 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(p1067: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1066 = p1067;
    {
        let mut m1066 = m1066;
        loop {
            let step = {
                let v0 = m1066;
                let m1062 = v0;
                match m1062.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1065 = f346({
                            let (h1063, _) = {
                                let (_, h1064) = v2;
                                h1064
                            };
                            h1063
                        }, v1.clone());
                        match m1065 {
                            None => {
                                Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(Some(v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(p1078: u64, p1079: List<(Bytes, (List<Bytes>, Bytes))>, p1080: List<Bytes>, p1081: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1077 = (p1078, (p1079, (p1080, p1081)));
    {
        let mut m1077 = m1077;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1077;
                let m1068 = v0;
                if m1068 == 0 {
                    let m1069 = v1;
                    match m1069.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1068 - 1;
                    let m1070 = v1.clone();
                    match m1070.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1071 = f347(v1, v2.clone());
                            match m1071 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1072) = v9.clone();
                                        h1072
                                    }, (List::cons({
                                        let (h1073, _) = {
                                            let (h1074, _) = v9.clone();
                                            h1074
                                        };
                                        h1073
                                    }, v2), List::cons({
                                        let (_, h1075) = {
                                            let (h1076, _) = v9;
                                            h1076
                                        };
                                        h1075
                                    }, v3)))))
                                }
                            }
                        }
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

pub fn f246(p1084: List<Bytes>, p1085: List<Bytes>) -> List<Bytes> {
    let m1083 = (p1084, p1085);
    {
        let mut m1083 = m1083;
        loop {
            let step = {
                let (v0, v1) = m1083;
                let m1082 = v0;
                match m1082.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f247(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f322(v0.clone())?;
    let v2: u64 = f348(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a1086 = v1;
        let a1087 = v2;
        nat_sub(a1086, a1087)
    }))
}

pub fn f248(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f349(v0)?;
    let m1091 = f350(v1.clone(), 0u64, {
        let a1089 = {
            let a1088 = v1.clone();
            length_bytes(a1088)
        };
        let a1090 = 2u64;
        nat_add(a1089, a1090)?
    }, 0u64)?;
    match m1091 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a1094 = {
                let (_, h1092) = v2;
                h1092
            };
            let a1095 = {
                let a1093 = v1;
                length_bytes(a1093)
            };
            if nat_eq(a1094, a1095) {
                Ok(Some({
                    let (h1096, _) = v2;
                    h1096
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f249(v0: Bytes, v1: Bytes) -> bool {
    let a1097 = v0;
    let a1098 = f269();
    if equal(a1097, a1098) {
        let a1102 = {
            let a1100 = {
                let a1099 = v1;
                length_bytes(a1099)
            };
            let a1101 = 0u64;
            nat_eq(a1100, a1101)
        };
        bool_not(a1102)
    } else {
        false
    }
}

pub fn f250(v0: Bytes) -> Bytes {
    f7(List::cons(f79(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f251(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1104 = v0;
    let a1105 = {
        let a1103 = Str::lit("redistributable");
        utf8_encode(a1103)
    };
    if equal(a1104, a1105) {
        f351(v1)
    } else {
        Ok(false)
    }
}

pub fn f252(v0: Adt14) -> bool {
    if f209(v0.clone(), f8(Str::lit("domain"))) {
        if f209(v0.clone(), f8(Str::lit("row"))) {
            if f88(f163(v0.clone(), f8(Str::lit("relation"))), f352()) {
                if f353(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f353(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f353(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f353(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f253(v0: Adt14) -> bool {
    if f209(v0.clone(), f8(Str::lit("subject"))) {
        if f209(v0.clone(), f8(Str::lit("object"))) {
            if f209(v0.clone(), f8(Str::lit("evidence"))) {
                f88(f163(v0, f8(Str::lit("relation"))), f354())
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

pub fn f254(v0: Option<List<Adt14>>) -> bool {
    let m1106 = v0;
    match m1106 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f255(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1107 = v0;
    match m1107 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f256(v0: Adt14) -> Bytes {
    if f209(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f209(v0.clone(), f8(Str::lit("id"))) {
            if f209(v0.clone(), f8(Str::lit("statement"))) {
                if f209(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f266(v0, f8(Str::lit("evidence")))
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

pub fn f257(v0: List<Adt14>) -> Adt14 {
    let m1108 = v0;
    match m1108.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f257(v2)))
        }
    }
}

pub fn f258(v0: Bytes) -> R<Option<Adt19>> {
    let v1: Bytes = f349(v0)?;
    let m1112 = f355(v1.clone(), 0u64, {
        let a1110 = {
            let a1109 = v1.clone();
            length_bytes(a1109)
        };
        let a1111 = 2u64;
        nat_add(a1110, a1111)?
    }, 0u64)?;
    match m1112 {
        None => {
            Ok(None::<Adt19>)
        }
        Some(v2) => {
            let a1115 = {
                let (_, h1113) = v2;
                h1113
            };
            let a1116 = {
                let a1114 = v1;
                length_bytes(a1114)
            };
            if nat_eq(a1115, a1116) {
                Ok(Some({
                    let (h1117, _) = v2;
                    h1117
                }))
            } else {
                Ok(None::<Adt19>)
            }
        }
    }
}

pub fn f259(v0: Adt19) -> bool {
    if f356(v0.clone(), f8(Str::lit("name"))) {
        if f357(v0.clone(), f8(Str::lit("schema"))) {
            let m1118 = f261(f157(f158(f153(v0, f8(Str::lit("citation"))))));
            match m1118 {
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

pub fn f260(v0: Bytes) -> R<Bytes> {
    f358(v0.clone(), 0u64, {
        let a1119 = v0;
        length_bytes(a1119)
    }, true, Bytes::lit(&[]))
}

pub fn f261(v0: List<Bytes>) -> Option<Adt14> {
    let m1120 = v0;
    match m1120.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit("req"))) {
                let m1121 = v2;
                match m1121.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f37(v1.clone(), f8(Str::lit("std"))) {
                    let m1122 = v2;
                    match m1122.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1123 = v6;
                                match m1123.uncons() {
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
                    if f37(v1, f8(Str::lit("der"))) {
                        let m1124 = v2;
                        match m1124.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1125 = v10;
                                match m1125.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f223(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f262(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1126 = f62(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1126.uncons() {
        None => {
            let m1127 = f261(v4);
            match m1127 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1128 = f8(Str::lit("Catalog."));
                        let a1129 = f260(v3.clone())?;
                        append_bytes(a1128, a1129)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f260(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1130 = f359(v2, v9.clone(), f260(v3.clone())?)?;
                    match m1130 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f260(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f222(Adt14::C3(std::rc::Rc::new(f257(f360(f75(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f222(Adt14::C3(std::rc::Rc::new(f257(f360(f75(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1134 = f361(f72(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a1132 = {
                                        let a1131 = v10;
                                        length_bytes(a1131)
                                    };
                                    let a1133 = 4u64;
                                    nat_sub(a1132, a1133)
                                }))?;
                                let a1135 = Bytes::lit(&[0x0a]);
                                append_bytes(a1134, a1135)
                            }), List::cons(f122(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f263(v0: Adt19) -> R<Bytes> {
    let m1136 = v0;
    match m1136 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f362(v2, v3)
        }
        Adt19::C2(v4, v5, v6) => {
            f363(v4, v5, v6)
        }
        Adt19::C3(v7) => {
            f364(v7)
        }
        Adt19::C4(r1137) => {
            let v8 = (*r1137).clone();
            Ok(f365(f366(v8)?))
        }
        Adt19::C5(r1138) => {
            let v9 = (*r1138).clone();
            f367(f368(v9)?)
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

pub fn f264(v0: Option<Adt19>) -> Option<Bytes> {
    let m1139 = v0;
    match m1139 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1140 = v1;
            match m1140 {
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

pub fn f265(v0: Adt19) -> List<Adt19> {
    let m1141 = v0;
    match m1141 {
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
        Adt19::C4(r1142) => {
            let v3 = (*r1142).clone();
            f265(v3)
        }
        Adt19::C5(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C6 => {
            List::<Adt19>::nil()
        }
        Adt19::C7(r1143, r1144) => {
            let v1 = (*r1143).clone();
            let v2 = (*r1144).clone();
            List::cons(v1, f265(v2))
        }
        Adt19::C8(_, _, _) => {
            List::<Adt19>::nil()
        }
    }
}

pub fn f266(v0: Adt14, v1: Bytes) -> bool {
    let m1145 = f165(v0, v1);
    match m1145 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f267(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1146 = f165(v0, v1);
    match m1146 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f162(f165(v3, v2))
        }
    }
}

pub fn f268(v0: Adt14) -> List<Adt14> {
    let m1147 = v0;
    match m1147 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1148) => {
            let v3 = (*r1148).clone();
            f268(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1149, r1150) => {
            let v1 = (*r1149).clone();
            let v2 = (*r1150).clone();
            List::cons(v1, f268(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f269() -> Bytes {
    let a1151 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1151)
}

pub fn f270(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f165(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f118(Str::lit("id"), f163(v0.clone(), f8(Str::lit("id"))))?, List::cons(f118(Str::lit("name"), f163(v0.clone(), f8(Str::lit("name"))))?, List::cons(f118(Str::lit("issuer"), f163(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f118(Str::lit("edition"), f163(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f118(Str::lit("canonical_id"), f163(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f118(Str::lit("source_role"), f163(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f118(Str::lit("retrieval_date"), f163(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f118(Str::lit("url"), f369(v2.clone(), f8(Str::lit("url"))))?, List::cons(f118(Str::lit("media_type"), f369(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f118(Str::lit("license"), f369(v2.clone(), f8(Str::lit("license"))))?, List::cons(f118(Str::lit("redistribution"), f369(v2, f8(Str::lit("redistribution"))))?, List::cons(f118(Str::lit("statement"), f163(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1152 = v1;
        match m1152 {
            None => {
                f7(List::cons(f118(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f118(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f118(Str::lit("artifact"), {
                    let (h1153, _) = v3;
                    h1153
                })?, List::cons(f118(Str::lit("checksum"), {
                    let (_, h1154) = v3;
                    h1154
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f370(f76(f162(f165(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f271(v0: Bytes, v1: u64) -> Option<u8> {
    let a1155 = v0;
    let a1156 = v1;
    index_bytes(a1155, a1156)
}

pub fn f272(v0: u64) -> bool {
    let a1157 = v0;
    let a1158 = 127u64;
    nat_le(a1157, a1158)
}

pub fn f273(v0: Bytes) -> bool {
    f140(v0, f8(Str::lit(".lex.tex")))
}

pub fn f274(v0: Fn3, p1163: u64, p1164: u64) -> R<(u64, bool)> {
    let m1162 = (p1163, p1164);
    {
        let mut m1162 = m1162;
        loop {
            let step = {
                let (v1, v2) = m1162;
                let m1159 = v1;
                if m1159 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1159 - 1;
                    let m1161 = {
                        let c1160: Fn3 = v0.clone();
                        c1160.apply(v2)?
                    };
                    match m1161 {
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
                Ok(next) => { m1162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f226(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1167 = {
            let a1165 = v4;
            let a1166 = v1;
            nat_add(a1165, a1166)?
        };
        let a1168 = v3;
        if nat_lt(a1167, a1168) {
            Ok(Some({
                let a1169 = v4;
                let a1170 = 1u64;
                nat_add(a1169, a1170)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f276(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1171 = v0;
    match m1171.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f276(v3, {
                let a1172 = v1;
                let a1173 = 1u64;
                nat_add(a1172, a1173)?
            })?))
        }
    }
}

pub fn f277(v0: Adt16, v1: List<u64>, v2: Adt21) -> bool {
    let m1174 = v2;
    match m1174 {
        Adt21::C0(v3, v4) => {
            if f283(v3, match v0.clone() {
                Adt16::C0(h1175, _, _, _) => {
                    h1175
                }
            }) {
                f371(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f283(v5, match v0 {
                Adt16::C0(_, _, h1176, _) => {
                    h1176
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f279(v7, v1) {
                f283(v8, match v0 {
                    Adt16::C0(_, _, _, h1177) => {
                        h1177
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f278(v0: Adt12, v1: Adt14) -> R<Adt21> {
    let m1178 = f165(v1.clone(), f8(Str::lit("std")));
    match m1178 {
        None => {
            let m1183 = f166(f165(v1.clone(), f8(Str::lit("req"))));
            match m1183 {
                None => {
                    let m1185 = f165(v1, f8(Str::lit("der")));
                    match m1185 {
                        None => {
                            Ok(f372(v0))
                        }
                        Some(v5) => {
                            let m1186 = f373(f163(v5.clone(), f8(Str::lit("rule"))));
                            match m1186 {
                                None => {
                                    Ok(f372(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f183(f76(f162(f165(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h1187) => {
                                            h1187
                                        }
                                    })?, f281(f163(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h1188, _) => {
                                            h1188
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f281(v4, match v0 {
                        Adt12::C0(_, _, h1184, _, _) => {
                            h1184
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1179 = f166(f165(v2.clone(), f8(Str::lit("row"))));
            match m1179 {
                None => {
                    Ok(Adt21::C0(f281(f163(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h1180, _, _, _, _) => {
                            h1180
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f281(f163(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h1181, _, _, _, _) => {
                            h1181
                        }
                    })?, Some(f281(v3, match v0 {
                        Adt12::C0(_, h1182, _, _, _) => {
                            h1182
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f279(p1191: List<u64>, v1: List<u64>) -> bool {
    let m1190 = p1191;
    {
        let mut m1190 = m1190;
        loop {
            let step = {
                let v0 = m1190;
                let m1189 = v0;
                match m1189.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f283(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(p1196: List<u64>) -> bool {
    let m1195 = p1196;
    {
        let mut m1195 = m1195;
        loop {
            let step = {
                let v0 = m1195;
                let m1192 = v0;
                match m1192.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1193 = v1;
                        let a1194 = 7u64;
                        if nat_le(a1193, a1194) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1197 = f374(v0, v1.clone(), 0u64)?;
    match m1197 {
        None => {
            let a1198 = v1;
            Ok(length_list(a1198))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f282(v0: Adt14) -> u64 {
    let m1199 = f375(f165(v0, f8(Str::lit("stratum"))));
    match m1199 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f283(v0: u64, p1204: List<u64>) -> bool {
    let m1203 = p1204;
    {
        let mut m1203 = m1203;
        loop {
            let step = {
                let v1 = m1203;
                let m1200 = v1;
                match m1200.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1201 = v2;
                        let a1202 = v0;
                        if nat_eq(a1201, a1202) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f211(v0.clone(), v1) {
        let m1205 = f30(v0, 34u8)?;
        match m1205.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1206 = v3;
                match m1206.uncons() {
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

pub fn f285(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f286(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1207 = v0.clone();
    match m1207.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1208 = v2;
            let a1209 = v1.clone();
            if equal(a1208, a1209) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f287(v0: List<(Bytes, ())>) -> List<Bytes> {
    f188(f376(Fn6::F377, List::<Bytes>::nil(), v0))
}

pub fn f288(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f378(f379(Fn7::F380, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1214 = {
        let (h1210, _) = f381(Fn8::F382, {
            let a1212 = {
                let a1211 = v0;
                length_list(a1211)
            };
            let a1213 = 1u64;
            nat_add(a1212, a1213)?
        }, v1)?;
        h1210
    };
    match m1214.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f289(v0: List<Bytes>) -> List<(Bytes, ())> {
    f383(f384(Fn9::F385, List::<(Bytes, ())>::nil(), v0))
}

pub fn f290(p1217: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1216 = p1217;
    {
        let mut m1216 = m1216;
        loop {
            let step = {
                let v0 = m1216;
                let m1215 = v0;
                match m1215.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f320(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
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

pub fn f291(v0: Adt14, v1: Bytes) -> Bytes {
    if f37(f369(f165(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f292(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1218, _) = v1.clone();
        h1218
    };
    let v3: u64 = {
        let (h1219, _) = {
            let (_, h1220) = v1.clone();
            h1220
        };
        h1219
    };
    let v4: List<Bytes> = {
        let (_, h1221) = {
            let (_, h1222) = v1;
            h1222
        };
        h1221
    };
    let m1223 = f271(v0.clone(), v2);
    match m1223 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1224 = v5;
            let a1225 = 10u8;
            if equal(a1224, a1225) {
                Ok(Some(({
                    let a1226 = v2;
                    let a1227 = 1u64;
                    nat_add(a1226, a1227)?
                }, ({
                    let a1228 = v2;
                    let a1229 = 1u64;
                    nat_add(a1228, a1229)?
                }, List::cons(f386(f32(v0, v3, {
                    let a1230 = v2;
                    let a1231 = v3;
                    nat_sub(a1230, a1231)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1232 = v2;
                    let a1233 = 1u64;
                    nat_add(a1232, a1233)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f293(v0: Adt1, v1: Bytes) -> R<Adt19> {
    let m1234 = f152(v0, v1)?;
    match m1234 {
        None => {
            Ok(Adt19::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f294(v0: Bytes) -> R<Bytes> {
    let a1235 = f8(Str::lit("Catalog."));
    let a1236 = f260(v0)?;
    Ok(append_bytes(a1235, a1236))
}

pub fn f295(v0: Adt19) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f260(f156(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f296(v0: List<Bytes>) -> List<Bytes> {
    let m1237 = v0;
    match m1237.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f140(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f136(f144(), v1), f296(v2))
            } else {
                f296(v2)
            }
        }
    }
}

pub fn f297(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1238 = v1;
    match m1238.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f387(v0.clone(), v2)?;
            let (h1239, _) = v4.clone();
            if h1239 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f297(v0, v3)?;
                let (h1240, _) = v5.clone();
                if h1240 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1243 = {
                            let (_, h1241) = v4;
                            h1241
                        };
                        let a1244 = {
                            let (_, h1242) = v5;
                            h1242
                        };
                        append_bytes(a1243, a1244)
                    }))
                }
            }
        }
    }
}

pub fn f298(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1245 = v1;
    match m1245.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f209(v2.clone(), f8(Str::lit("subject"))) {
                if f210(v0.clone(), v2.clone()) {
                    f298(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f163(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f163(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f163(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f298(v0, v3))
                }
            } else {
                if f307(v0.clone(), v2.clone()) {
                    f298(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f163(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f308(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f298(v0, v3))
                }
            }
        }
    }
}

pub fn f299(v0: Adt1) -> R<Bytes> {
    let a1247 = {
        let a1246 = f105(f106(f124(f125(f114(f72(v0.clone(), f119()))?)), f107(f75(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f108(f75(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1246)
    };
    let a1248 = 0u64;
    if nat_eq(a1247, a1248) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f300(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1249 = v0;
    match m1249.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f123(f76(f162(f165(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f163(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f163(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f388(f85(f163(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f300(v2)?))
        }
    }
}

pub fn f301(v0: Bytes, v1: List<Adt23>) -> List<Bytes> {
    let m1250 = v1;
    match m1250.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(match v2.clone() {
                Adt23::C0(h1251, _, _) => {
                    h1251
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt23::C0(_, h1252, _) => {
                        h1252
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt23::C0(_, _, h1253) => {
                        h1253
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f301(v0, v3))
            } else {
                f301(v0, v3)
            }
        }
    }
}

pub fn f302(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1254 = v1;
    match m1254.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(f163(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f163(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f163(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f302(v0, v3))
            } else {
                if f37(f163(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f163(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f163(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f302(v0, v3))
                } else {
                    f302(v0, v3)
                }
            }
        }
    }
}

pub fn f303(p1259: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1258 = p1259;
    {
        let mut m1258 = m1258;
        loop {
            let step = {
                let v0 = m1258;
                let m1255 = v0;
                match m1255.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1256 = v2;
                        match m1256.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f37(v4, v1.clone()) {
                                    let m1257 = v5;
                                    match m1257.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f389(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1260 = v0;
    match m1260.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1261 = f165(v1, f8(Str::lit("std")));
            match m1261 {
                None => {
                    f304(v2)
                }
                Some(v3) => {
                    if f209(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f163(v3.clone(), f8(Str::lit("row"))), f163(v3, f8(Str::lit("authority")))), f304(v2))
                    } else {
                        f304(v2)
                    }
                }
            }
        }
    }
}

pub fn f305(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1262 = v1;
    match m1262.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f163(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f163(v2, f8(Str::lit("object")));
            if if f88(v4.clone(), match v0.clone() {
                Adt15::C0(h1263, _, _) => {
                    h1263
                }
            }) {
                f88(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1264, _) => {
                        h1264
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f305(v0, v3))
            } else {
                if if f88(v5.clone(), match v0.clone() {
                    Adt15::C0(h1265, _, _) => {
                        h1265
                    }
                }) {
                    f88(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1266, _) => {
                            h1266
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f305(v0, v3))
                } else {
                    f305(v0, v3)
                }
            }
        }
    }
}

pub fn f306(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt23>> {
    let m1267 = v0;
    match m1267.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v4, v5)) => {
            if if f390(v4.clone(), v2.clone()) {
                true
            } else {
                f390(v4.clone(), v3.clone())
            } {
                f306(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt23::C0({
                    let (h1268, _) = v4.clone();
                    h1268
                }, {
                    let a1271 = {
                        let a1270 = f80(f391({
                            let (h1269, _) = v4.clone();
                            h1269
                        }, v1.clone()))?;
                        length_list(a1270)
                    };
                    let a1272 = 1u64;
                    if nat_eq(a1271, a1272) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1273) = v4.clone();
                    h1273
                }), f306(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f307(v0: Adt15, v1: Adt14) -> bool {
    if f88(f163(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1274, _, _) => {
            h1274
        }
    }) {
        f88(f308(v1), match v0 {
            Adt15::C0(_, h1275, _) => {
                h1275
            }
        })
    } else {
        false
    }
}

pub fn f308(v0: Adt14) -> Bytes {
    let m1276 = f165(v0, f8(Str::lit("evidence")));
    match m1276 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f163(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f309(v0: Adt14) -> Bytes {
    if f209(v0.clone(), f8(Str::lit("req"))) {
        let a1277 = f8(Str::lit("req "));
        let a1278 = f163(v0, f8(Str::lit("req")));
        append_bytes(a1277, a1278)
    } else {
        let m1279 = f165(v0.clone(), f8(Str::lit("std")));
        match m1279 {
            None => {
                let m1282 = f165(v0, f8(Str::lit("der")));
                match m1282 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f163(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f123(f76(f162(f165(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f163(v1.clone(), f8(Str::lit("authority"))), List::cons(if f209(v1.clone(), f8(Str::lit("row"))) {
                    let a1280 = f8(Str::lit(" row "));
                    let a1281 = f163(v1, f8(Str::lit("row")));
                    append_bytes(a1280, a1281)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f310(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1283 = v1;
    match m1283.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f88({
                let (h1284, _) = v2.clone();
                h1284
            }, v0.clone()) {
                List::cons({
                    let (_, h1285) = v2;
                    h1285
                }, f310(v0, v3))
            } else {
                if f88({
                    let (_, h1286) = v2;
                    h1286
                }, v0.clone()) {
                    List::cons({
                        let (h1287, _) = v2;
                        h1287
                    }, f310(v0, v3))
                } else {
                    f310(v0, v3)
                }
            }
        }
    }
}

pub fn f311(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f88(v2.clone(), match v0.clone() {
        Adt15::C0(h1288, _, _) => {
            h1288
        }
    }) {
        true
    } else {
        if f88(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1289, _) => {
                h1289
            }
        }) {
            true
        } else {
            if f88(v2, match v0.clone() {
                Adt15::C0(_, _, h1290) => {
                    h1290
                }
            }) {
                true
            } else {
                if f209(v1.clone(), f8(Str::lit("scheme"))) {
                    f88(f163(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1291, _) => {
                            h1291
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: u64) -> R<u64> {
    let m1294 = {
        let a1292 = v0;
        let a1293 = v1;
        index_bytes(a1292, a1293)
    };
    match m1294 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1339 = {
                let a1315 = {
                    let a1303 = {
                        let a1297 = {
                            let a1295 = v3;
                            let a1296 = 128u8;
                            fixed_u8::bit_and(a1295, a1296)
                        };
                        let a1298 = 128u8;
                        if equal(a1297, a1298) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1304 = {
                        let a1301 = {
                            let a1299 = v3;
                            let a1300 = 64u8;
                            fixed_u8::bit_and(a1299, a1300)
                        };
                        let a1302 = 64u8;
                        if equal(a1301, a1302) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1303, a1304)?
                };
                let a1316 = {
                    let a1313 = {
                        let a1307 = {
                            let a1305 = v3;
                            let a1306 = 32u8;
                            fixed_u8::bit_and(a1305, a1306)
                        };
                        let a1308 = 32u8;
                        if equal(a1307, a1308) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1314 = {
                        let a1311 = {
                            let a1309 = v3;
                            let a1310 = 16u8;
                            fixed_u8::bit_and(a1309, a1310)
                        };
                        let a1312 = 16u8;
                        if equal(a1311, a1312) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1313, a1314)?
                };
                nat_add(a1315, a1316)?
            };
            let a1340 = {
                let a1337 = {
                    let a1325 = {
                        let a1319 = {
                            let a1317 = v3;
                            let a1318 = 8u8;
                            fixed_u8::bit_and(a1317, a1318)
                        };
                        let a1320 = 8u8;
                        if equal(a1319, a1320) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1326 = {
                        let a1323 = {
                            let a1321 = v3;
                            let a1322 = 4u8;
                            fixed_u8::bit_and(a1321, a1322)
                        };
                        let a1324 = 4u8;
                        if equal(a1323, a1324) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1325, a1326)?
                };
                let a1338 = {
                    let a1335 = {
                        let a1329 = {
                            let a1327 = v3;
                            let a1328 = 2u8;
                            fixed_u8::bit_and(a1327, a1328)
                        };
                        let a1330 = 2u8;
                        if equal(a1329, a1330) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1336 = {
                        let a1333 = {
                            let a1331 = v3;
                            let a1332 = 1u8;
                            fixed_u8::bit_and(a1331, a1332)
                        };
                        let a1334 = 1u8;
                        if equal(a1333, a1334) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1335, a1336)?
                };
                nat_add(a1337, a1338)?
            };
            nat_add(a1339, a1340)
        }
    }
}

pub fn f313(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1344 = {
        let a1341 = v1;
        let a1342 = v2;
        let a1343 = 1u64;
        slice_bytes(a1341, a1342, a1343)
    };
    match m1344 {
        None => {
            v0
        }
        Some(v3) => {
            let a1345 = v0;
            let a1346 = v3;
            append_bytes(a1345, a1346)
        }
    }
}

pub fn f314(p1349: List<Bytes>, p1350: List<Bytes>) -> List<Bytes> {
    let m1348 = (p1349, p1350);
    {
        let mut m1348 = m1348;
        loop {
            let step = {
                let (v0, v1) = m1348;
                let m1347 = v0;
                match m1347.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: Bytes, p1422: u64, p1423: u64, p1424: Bytes) -> R<Bytes> {
    let m1421 = (p1422, (p1423, p1424));
    {
        let mut m1421 = m1421;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1421;
                let m1351 = v2;
                if m1351 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1351 - 1;
                    let m1354 = {
                        let a1352 = v0.clone();
                        let a1353 = v1;
                        index_bytes(a1352, a1353)
                    };
                    match m1354 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1399 = {
                                    let a1375 = {
                                        let a1363 = {
                                            let a1357 = {
                                                let a1355 = v6;
                                                let a1356 = 128u8;
                                                fixed_u8::bit_and(a1355, a1356)
                                            };
                                            let a1358 = 128u8;
                                            if equal(a1357, a1358) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1364 = {
                                            let a1361 = {
                                                let a1359 = v6;
                                                let a1360 = 64u8;
                                                fixed_u8::bit_and(a1359, a1360)
                                            };
                                            let a1362 = 64u8;
                                            if equal(a1361, a1362) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1363, a1364)?
                                    };
                                    let a1376 = {
                                        let a1373 = {
                                            let a1367 = {
                                                let a1365 = v6;
                                                let a1366 = 32u8;
                                                fixed_u8::bit_and(a1365, a1366)
                                            };
                                            let a1368 = 32u8;
                                            if equal(a1367, a1368) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1374 = {
                                            let a1371 = {
                                                let a1369 = v6;
                                                let a1370 = 16u8;
                                                fixed_u8::bit_and(a1369, a1370)
                                            };
                                            let a1372 = 16u8;
                                            if equal(a1371, a1372) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1373, a1374)?
                                    };
                                    nat_add(a1375, a1376)?
                                };
                                let a1400 = {
                                    let a1397 = {
                                        let a1385 = {
                                            let a1379 = {
                                                let a1377 = v6;
                                                let a1378 = 8u8;
                                                fixed_u8::bit_and(a1377, a1378)
                                            };
                                            let a1380 = 8u8;
                                            if equal(a1379, a1380) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1386 = {
                                            let a1383 = {
                                                let a1381 = v6;
                                                let a1382 = 4u8;
                                                fixed_u8::bit_and(a1381, a1382)
                                            };
                                            let a1384 = 4u8;
                                            if equal(a1383, a1384) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1385, a1386)?
                                    };
                                    let a1398 = {
                                        let a1395 = {
                                            let a1389 = {
                                                let a1387 = v6;
                                                let a1388 = 2u8;
                                                fixed_u8::bit_and(a1387, a1388)
                                            };
                                            let a1390 = 2u8;
                                            if equal(a1389, a1390) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1396 = {
                                            let a1393 = {
                                                let a1391 = v6;
                                                let a1392 = 1u8;
                                                fixed_u8::bit_and(a1391, a1392)
                                            };
                                            let a1394 = 1u8;
                                            if equal(a1393, a1394) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1395, a1396)?
                                    };
                                    nat_add(a1397, a1398)?
                                };
                                nat_add(a1399, a1400)?
                            };
                            let a1401 = v7;
                            let a1402 = 34u64;
                            if if nat_eq(a1401, a1402) {
                                true
                            } else {
                                let a1403 = v7;
                                let a1404 = 92u64;
                                nat_eq(a1403, a1404)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1405 = v1;
                                    let a1406 = 1u64;
                                    nat_add(a1405, a1406)?
                                }, (v4, {
                                    let a1409 = v3;
                                    let a1410 = {
                                        let a1407 = Bytes::lit(&[0x5c]);
                                        let a1408 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1407, a1408)
                                    };
                                    append_bytes(a1409, a1410)
                                })))
                            } else {
                                let a1411 = v7;
                                let a1412 = 10u64;
                                if nat_eq(a1411, a1412) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1413 = v1;
                                        let a1414 = 1u64;
                                        nat_add(a1413, a1414)?
                                    }, (v4, {
                                        let a1415 = v3;
                                        let a1416 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1415, a1416)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1417 = v1;
                                        let a1418 = 1u64;
                                        nat_add(a1417, a1418)?
                                    }, (v4, {
                                        let a1419 = v3;
                                        let a1420 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1419, a1420)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: u32) -> Bytes {
    let a1453 = f392({
        let a1425 = f393(v0, 28u32);
        let a1426 = 15u32;
        fixed_u32::bit_and(a1425, a1426)
    });
    let a1454 = {
        let a1451 = f392({
            let a1427 = f393(v0, 24u32);
            let a1428 = 15u32;
            fixed_u32::bit_and(a1427, a1428)
        });
        let a1452 = {
            let a1449 = f392({
                let a1429 = f393(v0, 20u32);
                let a1430 = 15u32;
                fixed_u32::bit_and(a1429, a1430)
            });
            let a1450 = {
                let a1447 = f392({
                    let a1431 = f393(v0, 16u32);
                    let a1432 = 15u32;
                    fixed_u32::bit_and(a1431, a1432)
                });
                let a1448 = {
                    let a1445 = f392({
                        let a1433 = f393(v0, 12u32);
                        let a1434 = 15u32;
                        fixed_u32::bit_and(a1433, a1434)
                    });
                    let a1446 = {
                        let a1443 = f392({
                            let a1435 = f393(v0, 8u32);
                            let a1436 = 15u32;
                            fixed_u32::bit_and(a1435, a1436)
                        });
                        let a1444 = {
                            let a1441 = f392({
                                let a1437 = f393(v0, 4u32);
                                let a1438 = 15u32;
                                fixed_u32::bit_and(a1437, a1438)
                            });
                            let a1442 = f392({
                                let a1439 = v0;
                                let a1440 = 15u32;
                                fixed_u32::bit_and(a1439, a1440)
                            });
                            append_bytes(a1441, a1442)
                        };
                        append_bytes(a1443, a1444)
                    };
                    append_bytes(a1445, a1446)
                };
                append_bytes(a1447, a1448)
            };
            append_bytes(a1449, a1450)
        };
        append_bytes(a1451, a1452)
    };
    append_bytes(a1453, a1454)
}

pub fn f317(v0: Fn4, p1458: u64, p1459: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1457 = (p1458, p1459);
    {
        let mut m1457 = m1457;
        loop {
            let step = {
                let (v1, v2) = m1457;
                let m1455 = v1;
                if m1455 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1455 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1456: Fn4 = v0.clone();
                        c1456.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1457 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1461 = {
            let (h1460, _) = v3.clone();
            h1460
        };
        let a1462 = 64u64;
        nat_add(a1461, a1462)?
    }, f394({
        let (_, h1463) = v3;
        h1463
    }, f395(v0, v1, v2, {
        let (h1464, _) = v3;
        h1464
    })?)?))
}

pub fn f319() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f320(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1465 = f30(v1, 34u8)?;
    match m1465.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1466 = v3;
            match m1466.uncons() {
                None => {
                    Ok(f396(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f321(v0: List<Bytes>) -> List<Bytes> {
    let m1467 = v0;
    match m1467.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit("authority = []"))) {
                f321(v2)
            } else {
                List::cons(v1, f321(v2))
            }
        }
    }
}

pub fn f322(v0: Bytes) -> R<u64> {
    let (h1468, _) = f274(Fn3::F397(v0.clone()), {
        let a1469 = v0.clone();
        length_bytes(a1469)
    }, {
        let a1470 = v0;
        length_bytes(a1470)
    })?;
    Ok(h1468)
}

pub fn f323(v0: Adt14) -> R<Bytes> {
    let m1471 = v0;
    match m1471 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f362(v2, v3)
        }
        Adt14::C2(v4) => {
            f364(v4)
        }
        Adt14::C3(r1472) => {
            let v5 = (*r1472).clone();
            Ok(f365(f398(v5)?))
        }
        Adt14::C4(r1473) => {
            let v6 = (*r1473).clone();
            f367(f399(v6)?)
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

pub fn f324(v0: Fn5, p1478: u64, p1479: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1477 = (p1478, p1479);
    {
        let mut m1477 = m1477;
        loop {
            let step = {
                let (v1, v2) = m1477;
                let m1474 = v1;
                if m1474 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1474 - 1;
                    let m1476 = {
                        let c1475: Fn5 = v0.clone();
                        c1475.apply(v2.clone())?
                    };
                    match m1476 {
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
                Ok(next) => { m1477 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f325(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f400(v0, v1)
}

pub fn f326(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f401(v0, v1)
}

pub fn f327(v0: u64) -> bool {
    let a1480 = v0;
    let a1481 = 32u64;
    if if nat_eq(a1480, a1481) {
        true
    } else {
        let a1482 = v0;
        let a1483 = 9u64;
        nat_eq(a1482, a1483)
    } {
        true
    } else {
        let a1484 = v0;
        let a1485 = 10u64;
        if nat_eq(a1484, a1485) {
            true
        } else {
            let a1486 = v0;
            let a1487 = 13u64;
            nat_eq(a1486, a1487)
        }
    }
}

pub fn f328(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1488, _) = f274(Fn3::F402(v1.clone(), v0.clone()), {
            let a1490 = {
                let a1489 = v0.clone();
                length_bytes(a1489)
            };
            let a1491 = 1u64;
            nat_add(a1490, a1491)?
        }, v2)?;
        h1488
    };
    if f226(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f329(v0: Bytes, v1: u64) -> R<u64> {
    let (h1492, _) = f274(Fn3::F403(v0.clone()), {
        let a1494 = {
            let a1493 = v0;
            length_bytes(a1493)
        };
        let a1495 = 1u64;
        nat_add(a1494, a1495)?
    }, v1)?;
    Ok(h1492)
}

pub fn f330(v0: Bytes) -> R<bool> {
    let a1497 = {
        let a1496 = v0.clone();
        length_bytes(a1496)
    };
    let a1498 = 5u64;
    if nat_eq(a1497, a1498) {
        let a1499 = f333(v0.clone(), 0u64)?;
        let a1500 = 85u64;
        if if nat_eq(a1499, a1500) {
            true
        } else {
            let a1501 = f333(v0.clone(), 0u64)?;
            let a1502 = 117u64;
            nat_eq(a1501, a1502)
        } {
            let a1503 = f333(v0.clone(), 1u64)?;
            let a1504 = 84u64;
            if if nat_eq(a1503, a1504) {
                true
            } else {
                let a1505 = f333(v0.clone(), 1u64)?;
                let a1506 = 116u64;
                nat_eq(a1505, a1506)
            } {
                let a1507 = f333(v0.clone(), 2u64)?;
                let a1508 = 70u64;
                if if nat_eq(a1507, a1508) {
                    true
                } else {
                    let a1509 = f333(v0.clone(), 2u64)?;
                    let a1510 = 102u64;
                    nat_eq(a1509, a1510)
                } {
                    let a1511 = f333(v0.clone(), 3u64)?;
                    let a1512 = 45u64;
                    if nat_eq(a1511, a1512) {
                        let a1513 = f333(v0, 4u64)?;
                        let a1514 = 56u64;
                        Ok(nat_eq(a1513, a1514))
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

pub fn f331(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1518 = f328(v0.clone(), {
        let a1515 = Str::lit("--");
        utf8_encode(a1515)
    }, {
        let a1516 = v1;
        let a1517 = 4u64;
        nat_add(a1516, a1517)?
    })?;
    match m1518 {
        None => {
            Ok(Err::<u64, Bytes>(f229(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1521 = f233(v0, {
                let a1519 = v2;
                let a1520 = 2u64;
                nat_add(a1519, a1520)?
            })?;
            let a1522 = 62u64;
            if nat_eq(a1521, a1522) {
                Ok(Ok::<u64, Bytes>({
                    let a1523 = v2;
                    let a1524 = 3u64;
                    nat_add(a1523, a1524)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f229(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f332(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f334(v0.clone(), {
        let a1525 = v1;
        let a1526 = 2u64;
        nat_add(a1525, a1526)?
    })?;
    let a1529 = v2;
    let a1530 = {
        let a1527 = v1;
        let a1528 = 2u64;
        nat_add(a1527, a1528)?
    };
    if nat_eq(a1529, a1530) {
        Ok(Err::<u64, Bytes>(f229(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1533 = v2;
        let a1534 = {
            let a1531 = v1;
            let a1532 = 5u64;
            nat_add(a1531, a1532)?
        };
        if if nat_eq(a1533, a1534) {
            let a1537 = f233(v0.clone(), {
                let a1535 = v1;
                let a1536 = 2u64;
                nat_add(a1535, a1536)?
            })?;
            let a1538 = 120u64;
            if if nat_eq(a1537, a1538) {
                true
            } else {
                let a1541 = f233(v0.clone(), {
                    let a1539 = v1;
                    let a1540 = 2u64;
                    nat_add(a1539, a1540)?
                })?;
                let a1542 = 88u64;
                nat_eq(a1541, a1542)
            } {
                let a1545 = f233(v0.clone(), {
                    let a1543 = v1;
                    let a1544 = 3u64;
                    nat_add(a1543, a1544)?
                })?;
                let a1546 = 109u64;
                if if nat_eq(a1545, a1546) {
                    true
                } else {
                    let a1549 = f233(v0.clone(), {
                        let a1547 = v1;
                        let a1548 = 3u64;
                        nat_add(a1547, a1548)?
                    })?;
                    let a1550 = 77u64;
                    nat_eq(a1549, a1550)
                } {
                    let a1553 = f233(v0.clone(), {
                        let a1551 = v1;
                        let a1552 = 4u64;
                        nat_add(a1551, a1552)?
                    })?;
                    let a1554 = 108u64;
                    if nat_eq(a1553, a1554) {
                        true
                    } else {
                        let a1557 = f233(v0.clone(), {
                            let a1555 = v1;
                            let a1556 = 4u64;
                            nat_add(a1555, a1556)?
                        })?;
                        let a1558 = 76u64;
                        nat_eq(a1557, a1558)
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
            Ok(Err::<u64, Bytes>(f229(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1560 = f328(v0, {
                let a1559 = Str::lit("?>");
                utf8_encode(a1559)
            }, v2)?;
            match m1560 {
                None => {
                    Ok(Err::<u64, Bytes>(f229(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1561 = v3;
                        let a1562 = 2u64;
                        nat_add(a1561, a1562)?
                    }))
                }
            }
        }
    }
}

pub fn f333(v0: Bytes, v1: u64) -> R<u64> {
    let m1565 = {
        let a1563 = v0;
        let a1564 = v1;
        index_bytes(a1563, a1564)
    };
    match m1565 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1610 = {
                let a1586 = {
                    let a1574 = {
                        let a1568 = {
                            let a1566 = v3;
                            let a1567 = 128u8;
                            fixed_u8::bit_and(a1566, a1567)
                        };
                        let a1569 = 128u8;
                        if equal(a1568, a1569) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1575 = {
                        let a1572 = {
                            let a1570 = v3;
                            let a1571 = 64u8;
                            fixed_u8::bit_and(a1570, a1571)
                        };
                        let a1573 = 64u8;
                        if equal(a1572, a1573) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1574, a1575)?
                };
                let a1587 = {
                    let a1584 = {
                        let a1578 = {
                            let a1576 = v3;
                            let a1577 = 32u8;
                            fixed_u8::bit_and(a1576, a1577)
                        };
                        let a1579 = 32u8;
                        if equal(a1578, a1579) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1585 = {
                        let a1582 = {
                            let a1580 = v3;
                            let a1581 = 16u8;
                            fixed_u8::bit_and(a1580, a1581)
                        };
                        let a1583 = 16u8;
                        if equal(a1582, a1583) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1584, a1585)?
                };
                nat_add(a1586, a1587)?
            };
            let a1611 = {
                let a1608 = {
                    let a1596 = {
                        let a1590 = {
                            let a1588 = v3;
                            let a1589 = 8u8;
                            fixed_u8::bit_and(a1588, a1589)
                        };
                        let a1591 = 8u8;
                        if equal(a1590, a1591) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1597 = {
                        let a1594 = {
                            let a1592 = v3;
                            let a1593 = 4u8;
                            fixed_u8::bit_and(a1592, a1593)
                        };
                        let a1595 = 4u8;
                        if equal(a1594, a1595) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1596, a1597)?
                };
                let a1609 = {
                    let a1606 = {
                        let a1600 = {
                            let a1598 = v3;
                            let a1599 = 2u8;
                            fixed_u8::bit_and(a1598, a1599)
                        };
                        let a1601 = 2u8;
                        if equal(a1600, a1601) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1607 = {
                        let a1604 = {
                            let a1602 = v3;
                            let a1603 = 1u8;
                            fixed_u8::bit_and(a1602, a1603)
                        };
                        let a1605 = 1u8;
                        if equal(a1604, a1605) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1606, a1607)?
                };
                nat_add(a1608, a1609)?
            };
            nat_add(a1610, a1611)
        }
    }
}

pub fn f334(v0: Bytes, v1: u64) -> R<u64> {
    let (h1612, _) = f274(Fn3::F404(v0.clone(), v1), {
        let a1614 = {
            let a1613 = v0;
            length_bytes(a1613)
        };
        let a1615 = 1u64;
        nat_add(a1614, a1615)?
    }, v1)?;
    Ok(h1612)
}

pub fn f335(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1616 = v2;
    if m1616 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1616 - 1;
        let v5: u64 = f329(v0.clone(), v1)?;
        let a1617 = f233(v0.clone(), v5)?;
        let a1618 = 62u64;
        if if nat_eq(a1617, a1618) {
            true
        } else {
            f232(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1619 = v5;
            let a1620 = v1;
            if nat_eq(a1619, a1620) {
                Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f334(v0.clone(), v5)?;
                let a1621 = v6;
                let a1622 = v5;
                if nat_eq(a1621, a1622) {
                    Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1623 = v6;
                        let a1624 = v5;
                        nat_sub(a1623, a1624)
                    });
                    if f405(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f329(v0.clone(), v6)?;
                        let a1625 = f233(v0.clone(), v8)?;
                        let a1626 = 61u64;
                        if !nat_eq(a1625, a1626) {
                            Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f329(v0.clone(), {
                                let a1627 = v8;
                                let a1628 = 1u64;
                                nat_add(a1627, a1628)?
                            })?;
                            let v10: u64 = f233(v0.clone(), v9)?;
                            let a1629 = v10;
                            let a1630 = 34u64;
                            if !if nat_eq(a1629, a1630) {
                                true
                            } else {
                                let a1631 = v10;
                                let a1632 = 39u64;
                                nat_eq(a1631, a1632)
                            } {
                                Ok(Err::<(Adt18, u64), Bytes>(f229(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1635 = f336(v0.clone(), {
                                    let a1633 = v9;
                                    let a1634 = 1u64;
                                    nat_add(a1633, a1634)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1635 {
                                    Ok(v12) => {
                                        let m1637 = f335(v0, {
                                            let (_, h1636) = v12;
                                            h1636
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1637 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1638, _) = v12;
                                                    h1638
                                                }, std::rc::Rc::new({
                                                    let (h1639, _) = v14;
                                                    h1639
                                                })), {
                                                    let (_, h1640) = v14;
                                                    h1640
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

pub fn f336(v0: Bytes, p1680: u64, p1681: u64, p1682: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1679 = (p1680, (p1681, p1682));
    {
        let mut m1679 = m1679;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1679;
                let m1641 = v2;
                if m1641 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f229(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1641 - 1;
                    let v7: u64 = f233(v0.clone(), v1)?;
                    if if v4 {
                        let a1642 = v7;
                        let a1643 = v5;
                        nat_eq(a1642, a1643)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1644 = v1;
                            let a1645 = 1u64;
                            nat_add(a1644, a1645)?
                        }))))
                    } else {
                        let a1646 = v7;
                        let a1647 = 256u64;
                        if nat_eq(a1646, a1647) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f229(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1648 = v7;
                            let a1649 = 60u64;
                            if nat_eq(a1648, a1649) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f229(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1650 = v7;
                                let a1651 = 38u64;
                                if nat_eq(a1650, a1651) {
                                    let m1652 = f406(v0.clone(), v1)?;
                                    match m1652 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1653) = v9;
                                                h1653
                                            }, (v6, {
                                                let a1655 = v3;
                                                let a1656 = {
                                                    let (h1654, _) = v9;
                                                    h1654
                                                };
                                                append_bytes(a1655, a1656)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1657 = v7;
                                        let a1658 = 9u64;
                                        if if nat_eq(a1657, a1658) {
                                            true
                                        } else {
                                            let a1659 = v7;
                                            let a1660 = 10u64;
                                            nat_eq(a1659, a1660)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1661 = v1;
                                                let a1662 = 1u64;
                                                nat_add(a1661, a1662)?
                                            }, (v6, {
                                                let a1663 = v3;
                                                let a1664 = Bytes::lit(&[0x20]);
                                                append_bytes(a1663, a1664)
                                            })))
                                        } else {
                                            let v10: u64 = f407(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1667 = v3;
                                                let a1668 = f32(v0.clone(), v1, {
                                                    let a1665 = v10;
                                                    let a1666 = v1;
                                                    nat_sub(a1665, a1666)
                                                });
                                                append_bytes(a1667, a1668)
                                            })))
                                        }
                                    } else {
                                        let a1669 = v7;
                                        let a1670 = 93u64;
                                        if nat_eq(a1669, a1670) {
                                            if f232(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f229(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1671 = v1;
                                                    let a1672 = 1u64;
                                                    nat_add(a1671, a1672)?
                                                }, (v6, {
                                                    let a1673 = v3;
                                                    let a1674 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a1673, a1674)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f407(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1677 = v3;
                                                let a1678 = f32(v0.clone(), v1, {
                                                    let a1675 = v11;
                                                    let a1676 = v1;
                                                    nat_sub(a1675, a1676)
                                                });
                                                append_bytes(a1677, a1678)
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
                Ok(next) => { m1679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: Bytes, v1: u64) -> Bytes {
    let m1686 = {
        let a1683 = v0;
        let a1684 = v1;
        let a1685 = 1u64;
        slice_bytes(a1683, a1684, a1685)
    };
    match m1686 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f338(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f408(v1, v0, v2)
}

pub fn f339() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f340() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f341(v0: Bytes) -> R<Bytes> {
    let m1687 = f328(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1687 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f342(p1692: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1691 = p1692;
    {
        let mut m1691 = m1691;
        loop {
            let step = {
                let v0 = m1691;
                let m1688 = v0;
                match m1688.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h1689, _) = v2;
                            h1689
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1690) = v2;
                                h1690
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: Bytes) -> R<Bytes> {
    let m1693 = f328(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1693 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1694 = v1;
                let a1695 = 1u64;
                nat_add(a1694, a1695)?
            }, {
                let a1699 = {
                    let a1696 = v0;
                    length_bytes(a1696)
                };
                let a1700 = {
                    let a1697 = v1;
                    let a1698 = 1u64;
                    nat_add(a1697, a1698)?
                };
                nat_sub(a1699, a1700)
            }))
        }
    }
}

pub fn f344() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f345(v0: Bytes, v1: Adt18, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v4: Bytes = f260(v0.clone())?;
    let m1701 = f409(v1, v2)?;
    match m1701 {
        Ok(v6) => {
            let m1702 = v6;
            match m1702 {
                None => {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = {
                        let (h1703, _) = v7.clone();
                        h1703
                    };
                    let v9: Adt18 = {
                        let (h1704, _) = {
                            let (_, h1705) = v7.clone();
                            h1705
                        };
                        h1704
                    };
                    let v10: Adt18 = {
                        let (h1706, _) = {
                            let (_, h1707) = {
                                let (_, h1708) = v7.clone();
                                h1708
                            };
                            h1707
                        };
                        h1706
                    };
                    let v11: List<(Bytes, Bytes)> = {
                        let (_, h1709) = {
                            let (_, h1710) = {
                                let (_, h1711) = v7;
                                h1711
                            };
                            h1710
                        };
                        h1709
                    };
                    if f37(v8.clone(), f8(Str::lit("union"))) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f37(v8, f8(Str::lit("list"))) {
                            let m1712 = f240(v9, f8(Str::lit("itemType")));
                            match m1712 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1713 = f411(v11, v3, v12)?;
                                    match m1713 {
                                        Ok(v14) => {
                                            if f211({
                                                let (h1714, _) = v14.clone();
                                                h1714
                                            }, f8(Str::lit("(List "))) {
                                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f412(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons({
                                                    let (h1715, _) = v14;
                                                    h1715
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), {
                                                    let (_, h1716) = v14;
                                                    h1716
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1717 = f240(v9, f8(Str::lit("base")));
                            match m1717 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1718 = f411(v11.clone(), v3, v15)?;
                                    match m1718 {
                                        Ok(v17) => {
                                            let m1719 = f413(v10, v11)?;
                                            match m1719 {
                                                Ok(v19) => {
                                                    let v20: List<Adt19> = f414(v19, List::<Bytes>::nil())?;
                                                    let m1720 = v20.clone();
                                                    match m1720.uncons() {
                                                        None => {
                                                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f412(v4, {
                                                                let (h1721, _) = v17;
                                                                h1721
                                                            }), {
                                                                let (_, h1722) = v17;
                                                                h1722
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1723 = f415(f416(f157(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1723 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f417(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(v16, v0)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(v5) => {
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f410(v5, v0)))
        }
    }
}

pub fn f346(p1726: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1725 = p1726;
    {
        let mut m1725 = m1725;
        loop {
            let step = {
                let v0 = m1725;
                let m1724 = v0;
                match m1724.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f88(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1727 = v0;
    match m1727.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1730 = f346({
                let (h1728, _) = {
                    let (_, h1729) = v2.clone();
                    h1729
                };
                h1728
            }, v1.clone());
            match m1730 {
                None => {
                    Some((({
                        let (h1731, _) = v2;
                        h1731
                    }, {
                        let (_, h1732) = {
                            let (_, h1733) = v2;
                            h1733
                        };
                        h1732
                    }), v3))
                }
                Some(_) => {
                    let m1734 = f347(v3, v1);
                    match m1734 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1735, _) = v5;
                                h1735
                            }, List::cons(v2, {
                                let (_, h1736) = v5;
                                h1736
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f348(v0: Bytes, v1: u64) -> R<u64> {
    let (h1737, _) = f274(Fn3::F418(v1, v0), v1, 0u64)?;
    Ok(h1737)
}

pub fn f349(v0: Bytes) -> R<Bytes> {
    f419(v0.clone(), 0u64, {
        let a1738 = v0;
        length_bytes(a1738)
    }, false, false, Bytes::lit(&[]))
}

pub fn f350(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1739 = v2;
    if m1739 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1739 - 1;
        let v5: u64 = f333(v0.clone(), v1)?;
        let a1740 = v3;
        let a1741 = 1u64;
        if nat_eq(a1740, a1741) {
            let m1742 = f350(v0.clone(), v1, v4, 0u64)?;
            match m1742 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f333(v0.clone(), {
                        let (_, h1743) = v6.clone();
                        h1743
                    })?;
                    let a1744 = v7;
                    let a1745 = 44u64;
                    if nat_eq(a1744, a1745) {
                        let m1749 = f350(v0, {
                            let a1747 = {
                                let (_, h1746) = v6;
                                h1746
                            };
                            let a1748 = 1u64;
                            nat_add(a1747, a1748)?
                        }, v4, 1u64)?;
                        match m1749 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1750, _) = v6;
                                    h1750
                                }), std::rc::Rc::new({
                                    let (h1751, _) = v8;
                                    h1751
                                })), {
                                    let (_, h1752) = v8;
                                    h1752
                                })))
                            }
                        }
                    } else {
                        let a1753 = v7;
                        let a1754 = 93u64;
                        if nat_eq(a1753, a1754) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1755, _) = v6;
                                h1755
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1757 = {
                                    let (_, h1756) = v6;
                                    h1756
                                };
                                let a1758 = 1u64;
                                nat_add(a1757, a1758)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1759 = v3;
            let a1760 = 2u64;
            if nat_eq(a1759, a1760) {
                let m1763 = f420(v0.clone(), {
                    let a1761 = v1;
                    let a1762 = 1u64;
                    nat_add(a1761, a1762)?
                })?;
                match m1763 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1764 = v5;
                        let a1765 = 34u64;
                        if if nat_eq(a1764, a1765) {
                            let a1767 = f333(v0.clone(), {
                                let (h1766, _) = v9.clone();
                                h1766
                            })?;
                            let a1768 = 58u64;
                            nat_eq(a1767, a1768)
                        } else {
                            false
                        } {
                            let m1772 = f350(v0.clone(), {
                                let a1770 = {
                                    let (h1769, _) = v9;
                                    h1769
                                };
                                let a1771 = 1u64;
                                nat_add(a1770, a1771)?
                            }, v4, 0u64)?;
                            match m1772 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f333(v0.clone(), {
                                        let (_, h1773) = v10.clone();
                                        h1773
                                    })?;
                                    let a1774 = v11;
                                    let a1775 = 44u64;
                                    if nat_eq(a1774, a1775) {
                                        let m1779 = f350(v0, {
                                            let a1777 = {
                                                let (_, h1776) = v10;
                                                h1776
                                            };
                                            let a1778 = 1u64;
                                            nat_add(a1777, a1778)?
                                        }, v4, 2u64)?;
                                        match m1779 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1780) = v9;
                                                    h1780
                                                }, std::rc::Rc::new({
                                                    let (h1781, _) = v10;
                                                    h1781
                                                }), std::rc::Rc::new({
                                                    let (h1782, _) = v12;
                                                    h1782
                                                })), {
                                                    let (_, h1783) = v12;
                                                    h1783
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1784 = v11;
                                        let a1785 = 125u64;
                                        if nat_eq(a1784, a1785) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1786) = v9;
                                                h1786
                                            }, std::rc::Rc::new({
                                                let (h1787, _) = v10;
                                                h1787
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1789 = {
                                                    let (_, h1788) = v10;
                                                    h1788
                                                };
                                                let a1790 = 1u64;
                                                nat_add(a1789, a1790)?
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
                let a1791 = v5;
                let a1792 = 34u64;
                if nat_eq(a1791, a1792) {
                    let m1795 = f420(v0, {
                        let a1793 = v1;
                        let a1794 = 1u64;
                        nat_add(a1793, a1794)?
                    })?;
                    match m1795 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1796) = v13;
                                h1796
                            }), {
                                let (h1797, _) = v13;
                                h1797
                            })))
                        }
                    }
                } else {
                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1798 = v1;
                            let a1799 = 4u64;
                            nat_add(a1798, a1799)?
                        })))
                    } else {
                        if f37(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1800 = v1;
                                let a1801 = 5u64;
                                nat_add(a1800, a1801)?
                            })))
                        } else {
                            let a1802 = v5;
                            let a1803 = 91u64;
                            if nat_eq(a1802, a1803) {
                                let a1806 = f333(v0.clone(), {
                                    let a1804 = v1;
                                    let a1805 = 1u64;
                                    nat_add(a1804, a1805)?
                                })?;
                                let a1807 = 93u64;
                                if nat_eq(a1806, a1807) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1808 = v1;
                                        let a1809 = 2u64;
                                        nat_add(a1808, a1809)?
                                    })))
                                } else {
                                    let m1812 = f350(v0, {
                                        let a1810 = v1;
                                        let a1811 = 1u64;
                                        nat_add(a1810, a1811)?
                                    }, v4, 1u64)?;
                                    match m1812 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1813, _) = v14;
                                                h1813
                                            })), {
                                                let (_, h1814) = v14;
                                                h1814
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1815 = v5;
                                let a1816 = 123u64;
                                if nat_eq(a1815, a1816) {
                                    let a1819 = f333(v0.clone(), {
                                        let a1817 = v1;
                                        let a1818 = 1u64;
                                        nat_add(a1817, a1818)?
                                    })?;
                                    let a1820 = 125u64;
                                    if nat_eq(a1819, a1820) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1821 = v1;
                                            let a1822 = 2u64;
                                            nat_add(a1821, a1822)?
                                        })))
                                    } else {
                                        let m1825 = f350(v0, {
                                            let a1823 = v1;
                                            let a1824 = 1u64;
                                            nat_add(a1823, a1824)?
                                        }, v4, 2u64)?;
                                        match m1825 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1826, _) = v15;
                                                    h1826
                                                })), {
                                                    let (_, h1827) = v15;
                                                    h1827
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1828 = f421(v0, v1)?;
                                    match m1828 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1829) = v16;
                                                h1829
                                            }, {
                                                let (h1830, _) = v16;
                                                h1830
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

pub fn f351(v0: Bytes) -> R<bool> {
    let m1832 = {
        let a1831 = v0.clone();
        length_bytes(a1831)
    };
    if m1832 == 0 {
        Ok(true)
    } else {
        let v1 = m1832 - 1;
        let m1835 = {
            let a1833 = v0;
            let a1834 = v1;
            index_bytes(a1833, a1834)
        };
        match m1835 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f422({
                    let v3: u8 = v2;
                    let a1880 = {
                        let a1856 = {
                            let a1844 = {
                                let a1838 = {
                                    let a1836 = v3;
                                    let a1837 = 128u8;
                                    fixed_u8::bit_and(a1836, a1837)
                                };
                                let a1839 = 128u8;
                                if equal(a1838, a1839) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1845 = {
                                let a1842 = {
                                    let a1840 = v3;
                                    let a1841 = 64u8;
                                    fixed_u8::bit_and(a1840, a1841)
                                };
                                let a1843 = 64u8;
                                if equal(a1842, a1843) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1844, a1845)?
                        };
                        let a1857 = {
                            let a1854 = {
                                let a1848 = {
                                    let a1846 = v3;
                                    let a1847 = 32u8;
                                    fixed_u8::bit_and(a1846, a1847)
                                };
                                let a1849 = 32u8;
                                if equal(a1848, a1849) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1855 = {
                                let a1852 = {
                                    let a1850 = v3;
                                    let a1851 = 16u8;
                                    fixed_u8::bit_and(a1850, a1851)
                                };
                                let a1853 = 16u8;
                                if equal(a1852, a1853) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1854, a1855)?
                        };
                        nat_add(a1856, a1857)?
                    };
                    let a1881 = {
                        let a1878 = {
                            let a1866 = {
                                let a1860 = {
                                    let a1858 = v3;
                                    let a1859 = 8u8;
                                    fixed_u8::bit_and(a1858, a1859)
                                };
                                let a1861 = 8u8;
                                if equal(a1860, a1861) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1867 = {
                                let a1864 = {
                                    let a1862 = v3;
                                    let a1863 = 4u8;
                                    fixed_u8::bit_and(a1862, a1863)
                                };
                                let a1865 = 4u8;
                                if equal(a1864, a1865) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1866, a1867)?
                        };
                        let a1879 = {
                            let a1876 = {
                                let a1870 = {
                                    let a1868 = v3;
                                    let a1869 = 2u8;
                                    fixed_u8::bit_and(a1868, a1869)
                                };
                                let a1871 = 2u8;
                                if equal(a1870, a1871) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1877 = {
                                let a1874 = {
                                    let a1872 = v3;
                                    let a1873 = 1u8;
                                    fixed_u8::bit_and(a1872, a1873)
                                };
                                let a1875 = 1u8;
                                if equal(a1874, a1875) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1876, a1877)?
                        };
                        nat_add(a1878, a1879)?
                    };
                    nat_add(a1880, a1881)?
                }))
            }
        }
    }
}

pub fn f352() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f353(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1882 = f165(v0, v1);
    match m1882 {
        None => {
            false
        }
        Some(v3) => {
            f209(v3, v2)
        }
    }
}

pub fn f354() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f355(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt19, u64)>> {
    let m1883 = v2;
    if m1883 == 0 {
        Ok(None::<(Adt19, u64)>)
    } else {
        let v4 = m1883 - 1;
        let v5: u64 = f333(v0.clone(), v1)?;
        let a1884 = v3;
        let a1885 = 1u64;
        if nat_eq(a1884, a1885) {
            let m1886 = f355(v0.clone(), v1, v4, 0u64)?;
            match m1886 {
                None => {
                    Ok(None::<(Adt19, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f333(v0.clone(), {
                        let (_, h1887) = v6.clone();
                        h1887
                    })?;
                    let a1888 = v7;
                    let a1889 = 44u64;
                    if nat_eq(a1888, a1889) {
                        let m1893 = f355(v0, {
                            let a1891 = {
                                let (_, h1890) = v6;
                                h1890
                            };
                            let a1892 = 1u64;
                            nat_add(a1891, a1892)?
                        }, v4, 1u64)?;
                        match m1893 {
                            None => {
                                Ok(None::<(Adt19, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt19::C7(std::rc::Rc::new({
                                    let (h1894, _) = v6;
                                    h1894
                                }), std::rc::Rc::new({
                                    let (h1895, _) = v8;
                                    h1895
                                })), {
                                    let (_, h1896) = v8;
                                    h1896
                                })))
                            }
                        }
                    } else {
                        let a1897 = v7;
                        let a1898 = 93u64;
                        if nat_eq(a1897, a1898) {
                            Ok(Some((Adt19::C7(std::rc::Rc::new({
                                let (h1899, _) = v6;
                                h1899
                            }), std::rc::Rc::new(Adt19::C6)), {
                                let a1901 = {
                                    let (_, h1900) = v6;
                                    h1900
                                };
                                let a1902 = 1u64;
                                nat_add(a1901, a1902)?
                            })))
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1903 = v3;
            let a1904 = 2u64;
            if nat_eq(a1903, a1904) {
                let m1907 = f420(v0.clone(), {
                    let a1905 = v1;
                    let a1906 = 1u64;
                    nat_add(a1905, a1906)?
                })?;
                match m1907 {
                    None => {
                        Ok(None::<(Adt19, u64)>)
                    }
                    Some(v9) => {
                        let a1908 = v5;
                        let a1909 = 34u64;
                        if if nat_eq(a1908, a1909) {
                            let a1911 = f333(v0.clone(), {
                                let (h1910, _) = v9.clone();
                                h1910
                            })?;
                            let a1912 = 58u64;
                            nat_eq(a1911, a1912)
                        } else {
                            false
                        } {
                            let m1916 = f355(v0.clone(), {
                                let a1914 = {
                                    let (h1913, _) = v9;
                                    h1913
                                };
                                let a1915 = 1u64;
                                nat_add(a1914, a1915)?
                            }, v4, 0u64)?;
                            match m1916 {
                                None => {
                                    Ok(None::<(Adt19, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f333(v0.clone(), {
                                        let (_, h1917) = v10.clone();
                                        h1917
                                    })?;
                                    let a1918 = v11;
                                    let a1919 = 44u64;
                                    if nat_eq(a1918, a1919) {
                                        let m1923 = f355(v0, {
                                            let a1921 = {
                                                let (_, h1920) = v10;
                                                h1920
                                            };
                                            let a1922 = 1u64;
                                            nat_add(a1921, a1922)?
                                        }, v4, 2u64)?;
                                        match m1923 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt19::C8({
                                                    let (_, h1924) = v9;
                                                    h1924
                                                }, std::rc::Rc::new({
                                                    let (h1925, _) = v10;
                                                    h1925
                                                }), std::rc::Rc::new({
                                                    let (h1926, _) = v12;
                                                    h1926
                                                })), {
                                                    let (_, h1927) = v12;
                                                    h1927
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1928 = v11;
                                        let a1929 = 125u64;
                                        if nat_eq(a1928, a1929) {
                                            Ok(Some((Adt19::C8({
                                                let (_, h1930) = v9;
                                                h1930
                                            }, std::rc::Rc::new({
                                                let (h1931, _) = v10;
                                                h1931
                                            }), std::rc::Rc::new(Adt19::C6)), {
                                                let a1933 = {
                                                    let (_, h1932) = v10;
                                                    h1932
                                                };
                                                let a1934 = 1u64;
                                                nat_add(a1933, a1934)?
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
                let a1935 = v5;
                let a1936 = 34u64;
                if nat_eq(a1935, a1936) {
                    let m1939 = f420(v0, {
                        let a1937 = v1;
                        let a1938 = 1u64;
                        nat_add(a1937, a1938)?
                    })?;
                    match m1939 {
                        None => {
                            Ok(None::<(Adt19, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt19::C3({
                                let (_, h1940) = v13;
                                h1940
                            }), {
                                let (h1941, _) = v13;
                                h1941
                            })))
                        }
                    }
                } else {
                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt19::C0(true), {
                            let a1942 = v1;
                            let a1943 = 4u64;
                            nat_add(a1942, a1943)?
                        })))
                    } else {
                        if f37(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt19::C0(false), {
                                let a1944 = v1;
                                let a1945 = 5u64;
                                nat_add(a1944, a1945)?
                            })))
                        } else {
                            let a1946 = v5;
                            let a1947 = 91u64;
                            if nat_eq(a1946, a1947) {
                                let a1950 = f333(v0.clone(), {
                                    let a1948 = v1;
                                    let a1949 = 1u64;
                                    nat_add(a1948, a1949)?
                                })?;
                                let a1951 = 93u64;
                                if nat_eq(a1950, a1951) {
                                    Ok(Some((Adt19::C4(std::rc::Rc::new(Adt19::C6)), {
                                        let a1952 = v1;
                                        let a1953 = 2u64;
                                        nat_add(a1952, a1953)?
                                    })))
                                } else {
                                    let m1956 = f355(v0, {
                                        let a1954 = v1;
                                        let a1955 = 1u64;
                                        nat_add(a1954, a1955)?
                                    }, v4, 1u64)?;
                                    match m1956 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt19::C4(std::rc::Rc::new({
                                                let (h1957, _) = v14;
                                                h1957
                                            })), {
                                                let (_, h1958) = v14;
                                                h1958
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1959 = v5;
                                let a1960 = 123u64;
                                if nat_eq(a1959, a1960) {
                                    let a1963 = f333(v0.clone(), {
                                        let a1961 = v1;
                                        let a1962 = 1u64;
                                        nat_add(a1961, a1962)?
                                    })?;
                                    let a1964 = 125u64;
                                    if nat_eq(a1963, a1964) {
                                        Ok(Some((Adt19::C5(std::rc::Rc::new(Adt19::C6)), {
                                            let a1965 = v1;
                                            let a1966 = 2u64;
                                            nat_add(a1965, a1966)?
                                        })))
                                    } else {
                                        let m1969 = f355(v0, {
                                            let a1967 = v1;
                                            let a1968 = 1u64;
                                            nat_add(a1967, a1968)?
                                        }, v4, 2u64)?;
                                        match m1969 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt19::C5(std::rc::Rc::new({
                                                    let (h1970, _) = v15;
                                                    h1970
                                                })), {
                                                    let (_, h1971) = v15;
                                                    h1971
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1972 = f423(v0, v1)?;
                                    match m1972 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1973) = v16;
                                                h1973
                                            }, {
                                                let (h1974, _) = v16;
                                                h1974
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

pub fn f356(v0: Adt19, v1: Bytes) -> bool {
    let a1976 = {
        let a1975 = f156(v0, v1);
        length_bytes(a1975)
    };
    let a1977 = 0u64;
    !nat_eq(a1976, a1977)
}

pub fn f357(v0: Adt19, v1: Bytes) -> bool {
    let m1978 = f153(v0, v1);
    match m1978 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f358(v0: Bytes, p2045: u64, p2046: u64, p2047: bool, p2048: Bytes) -> R<Bytes> {
    let m2044 = (p2045, (p2046, (p2047, p2048)));
    {
        let mut m2044 = m2044;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2044;
                let m1979 = v2;
                if m1979 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1979 - 1;
                    let m1982 = {
                        let a1980 = v0.clone();
                        let a1981 = v1;
                        index_bytes(a1980, a1981)
                    };
                    match m1982 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a2027 = {
                                    let a2003 = {
                                        let a1991 = {
                                            let a1985 = {
                                                let a1983 = v7;
                                                let a1984 = 128u8;
                                                fixed_u8::bit_and(a1983, a1984)
                                            };
                                            let a1986 = 128u8;
                                            if equal(a1985, a1986) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1992 = {
                                            let a1989 = {
                                                let a1987 = v7;
                                                let a1988 = 64u8;
                                                fixed_u8::bit_and(a1987, a1988)
                                            };
                                            let a1990 = 64u8;
                                            if equal(a1989, a1990) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1991, a1992)?
                                    };
                                    let a2004 = {
                                        let a2001 = {
                                            let a1995 = {
                                                let a1993 = v7;
                                                let a1994 = 32u8;
                                                fixed_u8::bit_and(a1993, a1994)
                                            };
                                            let a1996 = 32u8;
                                            if equal(a1995, a1996) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2002 = {
                                            let a1999 = {
                                                let a1997 = v7;
                                                let a1998 = 16u8;
                                                fixed_u8::bit_and(a1997, a1998)
                                            };
                                            let a2000 = 16u8;
                                            if equal(a1999, a2000) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2001, a2002)?
                                    };
                                    nat_add(a2003, a2004)?
                                };
                                let a2028 = {
                                    let a2025 = {
                                        let a2013 = {
                                            let a2007 = {
                                                let a2005 = v7;
                                                let a2006 = 8u8;
                                                fixed_u8::bit_and(a2005, a2006)
                                            };
                                            let a2008 = 8u8;
                                            if equal(a2007, a2008) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2014 = {
                                            let a2011 = {
                                                let a2009 = v7;
                                                let a2010 = 4u8;
                                                fixed_u8::bit_and(a2009, a2010)
                                            };
                                            let a2012 = 4u8;
                                            if equal(a2011, a2012) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2013, a2014)?
                                    };
                                    let a2026 = {
                                        let a2023 = {
                                            let a2017 = {
                                                let a2015 = v7;
                                                let a2016 = 2u8;
                                                fixed_u8::bit_and(a2015, a2016)
                                            };
                                            let a2018 = 2u8;
                                            if equal(a2017, a2018) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2024 = {
                                            let a2021 = {
                                                let a2019 = v7;
                                                let a2020 = 1u8;
                                                fixed_u8::bit_and(a2019, a2020)
                                            };
                                            let a2022 = 1u8;
                                            if equal(a2021, a2022) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2023, a2024)?
                                    };
                                    nat_add(a2025, a2026)?
                                };
                                nat_add(a2027, a2028)?
                            };
                            if f424(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2029 = v1;
                                    let a2030 = 1u64;
                                    nat_add(a2029, a2030)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f425(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2031 = v1;
                                        let a2032 = 1u64;
                                        nat_add(a2031, a2032)?
                                    }, (v5, (false, {
                                        let a2036 = v4;
                                        let a2037 = f32({
                                            let a2033 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2033)
                                        }, {
                                            let a2034 = v8;
                                            let a2035 = 97u64;
                                            nat_sub(a2034, a2035)
                                        }, 1u64);
                                        append_bytes(a2036, a2037)
                                    }))))
                                } else {
                                    if if f425(v8) {
                                        true
                                    } else {
                                        if f426(v8) {
                                            true
                                        } else {
                                            f427(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2038 = v1;
                                            let a2039 = 1u64;
                                            nat_add(a2038, a2039)?
                                        }, (v5, (false, {
                                            let a2040 = v4;
                                            let a2041 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a2040, a2041)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2042 = v1;
                                            let a2043 = 1u64;
                                            nat_add(a2042, a2043)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2044 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Adt19, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt19)> = f428(f429(f153(v0.clone(), f8(Str::lit("$defs")))), f429(f153(v0.clone(), f8(Str::lit("definitions")))));
    let m2049 = f430(4096u64, v3.clone(), List::<Bytes>::nil(), f431(v3)?)?;
    match m2049 {
        Ok(v5) => {
            let m2050 = f432(4096u64, f433(f434(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m2050 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f221(f192(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f360(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m2051 = v0;
    match m2051.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f37(f163(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f435(v5, v1, v2))
            } else {
                List::cons(v4, f360(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f361(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f221(f436(f85(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f362(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a2052 = 0u64;
        let a2053 = v1;
        nat_lt(a2052, a2053)
    } else {
        false
    } {
        let a2054 = Bytes::lit(&[0x2d]);
        let a2055 = f128(v1)?;
        Ok(append_bytes(a2054, a2055))
    } else {
        f128(v1)
    }
}

pub fn f363(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f128(v1)?;
    let v4: u64 = {
        let a2056 = v3.clone();
        length_bytes(a2056)
    };
    let v5: Bytes = {
        let a2057 = v2;
        let a2058 = v4;
        if nat_lt(a2057, a2058) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a2059 = v4;
                let a2060 = v2;
                nat_sub(a2059, a2060)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a2061 = v4;
                let a2062 = v2;
                nat_sub(a2061, a2062)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f437({
                let a2063 = v2;
                let a2064 = v4;
                nat_sub(a2063, a2064)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a2065 = f8(Str::lit("-"));
        let a2066 = v5;
        Ok(append_bytes(a2065, a2066))
    } else {
        Ok(v5)
    }
}

pub fn f364(v0: Bytes) -> R<Bytes> {
    let a2072 = {
        let a2070 = Bytes::lit(&[0x22]);
        let a2071 = {
            let (_, h2067) = {
                let (h2068, _) = f324(Fn5::F438(v0.clone()), {
                    let a2069 = v0;
                    length_bytes(a2069)
                }, (0u64, Bytes::lit(&[])))?;
                h2068
            };
            h2067
        };
        append_bytes(a2070, a2071)
    };
    let a2073 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2072, a2073))
}

pub fn f365(v0: List<Bytes>) -> Bytes {
    let a2076 = {
        let a2074 = Bytes::lit(&[0x5b]);
        let a2075 = f123(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2074, a2075)
    };
    let a2077 = Bytes::lit(&[0x5d]);
    append_bytes(a2076, a2077)
}

pub fn f366(v0: Adt19) -> R<List<Bytes>> {
    let m2078 = v0;
    match m2078 {
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
        Adt19::C7(r2079, r2080) => {
            let v10 = (*r2079).clone();
            let v11 = (*r2080).clone();
            Ok(List::cons(f263(v10)?, f366(v11)?))
        }
        Adt19::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f367(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f439(Fn10::F440, List::<(Bytes, Bytes)>::nil(), f441(v0)?);
    let v2: List<Bytes> = f442(Fn11::F443, List::<Bytes>::nil(), v1)?;
    let a2083 = {
        let a2081 = Bytes::lit(&[0x7b]);
        let a2082 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2081, a2082)
    };
    let a2084 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2083, a2084))
}

pub fn f368(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m2085 = v0;
    match m2085 {
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
        Adt19::C8(v12, r2086, r2087) => {
            let v13 = (*r2086).clone();
            let v14 = (*r2087).clone();
            Ok(List::cons((v12, f263(v13)?), f368(v14)?))
        }
    }
}

pub fn f369(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m2088 = v0;
    match m2088 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f163(v2, v1)
        }
    }
}

pub fn f370(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f221(f444(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f371(v0: Adt16, v1: Option<u64>) -> bool {
    let m2089 = v1;
    match m2089 {
        None => {
            true
        }
        Some(v2) => {
            f283(v2, match v0 {
                Adt16::C0(_, h2090, _, _) => {
                    h2090
                }
            })
        }
    }
}

pub fn f372(v0: Adt12) -> Adt21 {
    Adt21::C1({
        let a2092 = match v0 {
            Adt12::C0(_, _, h2091, _, _) => {
                h2091
            }
        };
        length_list(a2092)
    })
}

pub fn f373(v0: Bytes) -> Option<Adt22> {
    if f37(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f37(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f37(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f374(v0: Bytes, p2097: List<Bytes>, p2098: u64) -> R<Option<u64>> {
    let m2096 = (p2097, p2098);
    {
        let mut m2096 = m2096;
        loop {
            let step = {
                let (v1, v2) = m2096;
                let m2093 = v1;
                match m2093.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f37(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2094 = v2;
                                let a2095 = 1u64;
                                nat_add(a2094, a2095)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2096 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375(v0: Option<Adt14>) -> Option<u64> {
    let m2099 = v0;
    match m2099 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2100 = v1;
            match m2100 {
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

pub fn f376(v0: Fn6, p2104: List<Bytes>, p2105: List<(Bytes, ())>) -> List<Bytes> {
    let m2103 = (p2104, p2105);
    {
        let mut m2103 = m2103;
        loop {
            let step = {
                let (v1, v2) = m2103;
                let m2101 = v2;
                match m2101.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c2102: Fn6 = v0.clone();
                            c2102.apply(v1, v3)
                        }, v4))
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

pub fn f377(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h2106, _) = v1;
        h2106
    }, v0)
}

pub fn f378(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f445(Fn12::F446, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f379(v0: Fn7, p2110: List<List<(Bytes, ())>>, p2111: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m2109 = (p2110, p2111);
    {
        let mut m2109 = m2109;
        loop {
            let step = {
                let (v1, v2) = m2109;
                let m2107 = v2;
                match m2107.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c2108: Fn7 = v0.clone();
                            c2108.apply(v1, v3)
                        }, v4))
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

pub fn f380(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f381(v0: Fn8, p2116: u64, p2117: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m2115 = (p2116, p2117);
    {
        let mut m2115 = m2115;
        loop {
            let step = {
                let (v1, v2) = m2115;
                let m2112 = v1;
                if m2112 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2112 - 1;
                    let m2114 = {
                        let c2113: Fn8 = v0.clone();
                        c2113.apply(v2.clone())?
                    };
                    match m2114 {
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
                Ok(next) => { m2115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f447(v0)
}

pub fn f383(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f448(Fn13::F449, List::<(Bytes, ())>::nil(), v0)
}

pub fn f384(v0: Fn9, p2121: List<(Bytes, ())>, p2122: List<Bytes>) -> List<(Bytes, ())> {
    let m2120 = (p2121, p2122);
    {
        let mut m2120 = m2120;
        loop {
            let step = {
                let (v1, v2) = m2120;
                let m2118 = v2;
                match m2118.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c2119: Fn9 = v0.clone();
                            c2119.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f386(v0: Bytes) -> Bytes {
    if f140(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2124 = {
                let a2123 = v0;
                length_bytes(a2123)
            };
            let a2125 = 1u64;
            nat_sub(a2124, a2125)
        })
    } else {
        v0
    }
}

pub fn f387(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f85(f72(v0.clone(), v1.clone()))?;
    let m2126 = f159(v2.clone())?;
    match m2126 {
        None => {
            f451(v0, v1, v2)
        }
        Some(v3) => {
            f450(v0, v1, v3)
        }
    }
}

pub fn f388(v0: List<Bytes>) -> List<Bytes> {
    let m2127 = v0;
    match m2127.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a2129 = {
                    let a2128 = v1.clone();
                    length_bytes(a2128)
                };
                let a2130 = 0u64;
                if nat_eq(a2129, a2130) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f388(v2))
        }
    }
}

pub fn f389(v0: List<Bytes>) -> Bytes {
    let m2131 = v0;
    match m2131.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f390(v0: (Bytes, Bytes), p2134: List<(Bytes, Bytes)>) -> bool {
    let m2133 = p2134;
    {
        let mut m2133 = m2133;
        loop {
            let step = {
                let v1 = m2133;
                let m2132 = v1;
                match m2132.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f452(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m2135 = v1;
    match m2135.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37({
                let (h2136, _) = v2;
                h2136
            }, v0.clone()) {
                List::cons({
                    let (_, h2137) = v2;
                    h2137
                }, f391(v0, v3))
            } else {
                f391(v0, v3)
            }
        }
    }
}

pub fn f392(v0: u32) -> Bytes {
    let a2140 = {
        let a2138 = v0;
        let a2139 = 8u32;
        fixed_u32::bit_and(a2138, a2139)
    };
    let a2141 = 0u32;
    if equal(a2140, a2141) {
        let a2144 = {
            let a2142 = v0;
            let a2143 = 4u32;
            fixed_u32::bit_and(a2142, a2143)
        };
        let a2145 = 0u32;
        if equal(a2144, a2145) {
            let a2148 = {
                let a2146 = v0;
                let a2147 = 2u32;
                fixed_u32::bit_and(a2146, a2147)
            };
            let a2149 = 0u32;
            if equal(a2148, a2149) {
                let a2152 = {
                    let a2150 = v0;
                    let a2151 = 1u32;
                    fixed_u32::bit_and(a2150, a2151)
                };
                let a2153 = 0u32;
                if equal(a2152, a2153) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2156 = {
                    let a2154 = v0;
                    let a2155 = 1u32;
                    fixed_u32::bit_and(a2154, a2155)
                };
                let a2157 = 0u32;
                if equal(a2156, a2157) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2160 = {
                let a2158 = v0;
                let a2159 = 2u32;
                fixed_u32::bit_and(a2158, a2159)
            };
            let a2161 = 0u32;
            if equal(a2160, a2161) {
                let a2164 = {
                    let a2162 = v0;
                    let a2163 = 1u32;
                    fixed_u32::bit_and(a2162, a2163)
                };
                let a2165 = 0u32;
                if equal(a2164, a2165) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2168 = {
                    let a2166 = v0;
                    let a2167 = 1u32;
                    fixed_u32::bit_and(a2166, a2167)
                };
                let a2169 = 0u32;
                if equal(a2168, a2169) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2172 = {
            let a2170 = v0;
            let a2171 = 4u32;
            fixed_u32::bit_and(a2170, a2171)
        };
        let a2173 = 0u32;
        if equal(a2172, a2173) {
            let a2176 = {
                let a2174 = v0;
                let a2175 = 2u32;
                fixed_u32::bit_and(a2174, a2175)
            };
            let a2177 = 0u32;
            if equal(a2176, a2177) {
                let a2180 = {
                    let a2178 = v0;
                    let a2179 = 1u32;
                    fixed_u32::bit_and(a2178, a2179)
                };
                let a2181 = 0u32;
                if equal(a2180, a2181) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2184 = {
                    let a2182 = v0;
                    let a2183 = 1u32;
                    fixed_u32::bit_and(a2182, a2183)
                };
                let a2185 = 0u32;
                if equal(a2184, a2185) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2188 = {
                let a2186 = v0;
                let a2187 = 2u32;
                fixed_u32::bit_and(a2186, a2187)
            };
            let a2189 = 0u32;
            if equal(a2188, a2189) {
                let a2192 = {
                    let a2190 = v0;
                    let a2191 = 1u32;
                    fixed_u32::bit_and(a2190, a2191)
                };
                let a2193 = 0u32;
                if equal(a2192, a2193) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2196 = {
                    let a2194 = v0;
                    let a2195 = 1u32;
                    fixed_u32::bit_and(a2194, a2195)
                };
                let a2197 = 0u32;
                if equal(a2196, a2197) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f393(v0: u32, v1: u32) -> u32 {
    let m2200 = {
        let a2198 = v0;
        let a2199 = v1;
        fixed_u32::shift_right(a2198, a2199)
    };
    match m2200 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f394(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f453(Fn14::F454, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2201, _, _, _, _, _, _, _) => {
            h2201
        }
    }, match v0.clone() {
        Adt24::C0(_, h2202, _, _, _, _, _, _) => {
            h2202
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2203, _, _, _, _, _) => {
            h2203
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2204, _, _, _, _) => {
            h2204
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2205, _, _, _) => {
            h2205
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2206, _, _) => {
            h2206
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2207, _) => {
            h2207
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2208) => {
            h2208
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f455(match v0.clone() {
        Adt24::C0(h2209, _, _, _, _, _, _, _) => {
            h2209
        }
    }, match v2.clone() {
        Adt27::C0(h2210, _, _, _, _, _, _, _, _, _) => {
            h2210
        }
    }), f455(match v0.clone() {
        Adt24::C0(_, h2211, _, _, _, _, _, _) => {
            h2211
        }
    }, match v2.clone() {
        Adt27::C0(_, h2212, _, _, _, _, _, _, _, _) => {
            h2212
        }
    }), f455(match v0.clone() {
        Adt24::C0(_, _, h2213, _, _, _, _, _) => {
            h2213
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2214, _, _, _, _, _, _, _) => {
            h2214
        }
    }), f455(match v0.clone() {
        Adt24::C0(_, _, _, h2215, _, _, _, _) => {
            h2215
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2216, _, _, _, _, _, _) => {
            h2216
        }
    }), f455(match v0.clone() {
        Adt24::C0(_, _, _, _, h2217, _, _, _) => {
            h2217
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2218, _, _, _, _, _) => {
            h2218
        }
    }), f455(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2219, _, _) => {
            h2219
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2220, _, _, _, _) => {
            h2220
        }
    }), f455(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2221, _) => {
            h2221
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2222, _, _, _) => {
            h2222
        }
    }), f455(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2223) => {
            h2223
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2224, _, _) => {
            h2224
        }
    })))
}

pub fn f395(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f456(v0.clone(), v1, v2, v3)?, f456(v0.clone(), v1, v2, {
        let a2225 = v3;
        let a2226 = 4u64;
        nat_add(a2225, a2226)?
    })?, f456(v0.clone(), v1, v2, {
        let a2227 = v3;
        let a2228 = 8u64;
        nat_add(a2227, a2228)?
    })?, f456(v0.clone(), v1, v2, {
        let a2229 = v3;
        let a2230 = 12u64;
        nat_add(a2229, a2230)?
    })?, f456(v0.clone(), v1, v2, {
        let a2231 = v3;
        let a2232 = 16u64;
        nat_add(a2231, a2232)?
    })?, f456(v0.clone(), v1, v2, {
        let a2233 = v3;
        let a2234 = 20u64;
        nat_add(a2233, a2234)?
    })?, f456(v0.clone(), v1, v2, {
        let a2235 = v3;
        let a2236 = 24u64;
        nat_add(a2235, a2236)?
    })?, f456(v0.clone(), v1, v2, {
        let a2237 = v3;
        let a2238 = 28u64;
        nat_add(a2237, a2238)?
    })?, f456(v0.clone(), v1, v2, {
        let a2239 = v3;
        let a2240 = 32u64;
        nat_add(a2239, a2240)?
    })?, f456(v0.clone(), v1, v2, {
        let a2241 = v3;
        let a2242 = 36u64;
        nat_add(a2241, a2242)?
    })?, f456(v0.clone(), v1, v2, {
        let a2243 = v3;
        let a2244 = 40u64;
        nat_add(a2243, a2244)?
    })?, f456(v0.clone(), v1, v2, {
        let a2245 = v3;
        let a2246 = 44u64;
        nat_add(a2245, a2246)?
    })?, f456(v0.clone(), v1, v2, {
        let a2247 = v3;
        let a2248 = 48u64;
        nat_add(a2247, a2248)?
    })?, f456(v0.clone(), v1, v2, {
        let a2249 = v3;
        let a2250 = 52u64;
        nat_add(a2249, a2250)?
    })?, f456(v0.clone(), v1, v2, {
        let a2251 = v3;
        let a2252 = 56u64;
        nat_add(a2251, a2252)?
    })?, f456(v0, v1, v2, {
        let a2253 = v3;
        let a2254 = 60u64;
        nat_add(a2253, a2254)?
    })?))
}

pub fn f396(p2257: List<Bytes>, v1: Bytes) -> bool {
    let m2256 = p2257;
    {
        let mut m2256 = m2256;
        loop {
            let step = {
                let v0 = m2256;
                let m2255 = v0;
                match m2255.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f211(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Bytes, v1: u64) -> Option<u64> {
    let a2258 = 0u64;
    let a2259 = v1;
    if nat_lt(a2258, a2259) {
        if f457(v0, {
            let a2260 = v1;
            let a2261 = 1u64;
            nat_sub(a2260, a2261)
        }) {
            Some({
                let a2262 = v1;
                let a2263 = 1u64;
                nat_sub(a2262, a2263)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f398(v0: Adt14) -> R<List<Bytes>> {
    let m2264 = v0;
    match m2264 {
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
        Adt14::C6(r2265, r2266) => {
            let v7 = (*r2265).clone();
            let v8 = (*r2266).clone();
            Ok(List::cons(f323(v7)?, f398(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f399(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m2267 = v0;
    match m2267 {
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
        Adt14::C7(v9, r2268, r2269) => {
            let v10 = (*r2268).clone();
            let v11 = (*r2269).clone();
            Ok(List::cons((v9, f323(v10)?), f399(v11)?))
        }
    }
}

pub fn f400(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2270, _) = v1;
        h2270
    };
    let a2272 = {
        let a2271 = v0.clone();
        length_bytes(a2271)
    };
    let a2273 = v2;
    if nat_le(a2272, a2273) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2274 = f233(v0.clone(), v2)?;
        let a2275 = 13u64;
        if nat_eq(a2274, a2275) {
            Ok(Some(({
                let a2278 = f233(v0, {
                    let a2276 = v2;
                    let a2277 = 1u64;
                    nat_add(a2276, a2277)?
                })?;
                let a2279 = 10u64;
                if nat_eq(a2278, a2279) {
                    let a2280 = v2;
                    let a2281 = 2u64;
                    nat_add(a2280, a2281)?
                } else {
                    let a2282 = v2;
                    let a2283 = 1u64;
                    nat_add(a2282, a2283)?
                }
            }, {
                let a2285 = {
                    let (_, h2284) = v1;
                    h2284
                };
                let a2286 = Bytes::lit(&[0x0a]);
                append_bytes(a2285, a2286)
            })))
        } else {
            let v4: u64 = {
                let m2287 = f328(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m2287 {
                    None => {
                        let a2288 = v0.clone();
                        length_bytes(a2288)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2292 = {
                    let (_, h2289) = v1;
                    h2289
                };
                let a2293 = f32(v0, v2, {
                    let a2290 = v4;
                    let a2291 = v2;
                    nat_sub(a2290, a2291)
                });
                append_bytes(a2292, a2293)
            })))
        }
    }
}

pub fn f401(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2294 = f458(v0, v1)?;
    match m2294 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f459({
                let (h2295, _) = v2;
                h2295
            }) {
                Ok(Some({
                    let a2297 = v1;
                    let a2298 = {
                        let (_, h2296) = v2;
                        h2296
                    };
                    nat_add(a2297, a2298)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f402(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f460(v1, v0, v2)
}

pub fn f403(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f461(v0, v1)
}

pub fn f404(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f462(v0, v1, v2)
}

pub fn f405(v0: Bytes, p2301: List<Bytes>) -> bool {
    let m2300 = p2301;
    {
        let mut m2300 = m2300;
        loop {
            let step = {
                let v1 = m2300;
                let m2299 = v1;
                match m2299.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f37(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f232(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2304 = f233(v0.clone(), {
                let a2302 = v1;
                let a2303 = 2u64;
                nat_add(a2302, a2303)?
            })?;
            let a2305 = 120u64;
            nat_eq(a2304, a2305)
        };
        let v3: u64 = if v2 {
            let a2306 = v1;
            let a2307 = 3u64;
            nat_add(a2306, a2307)?
        } else {
            let a2308 = v1;
            let a2309 = 2u64;
            nat_add(a2308, a2309)?
        };
        let v4: (u64, u64) = f463(v0.clone(), v2, v3)?;
        let a2311 = {
            let (h2310, _) = v4;
            h2310
        };
        let a2312 = v3;
        if if nat_eq(a2311, a2312) {
            true
        } else {
            let a2314 = f233(v0, {
                let (h2313, _) = v4;
                h2313
            })?;
            let a2315 = 59u64;
            !nat_eq(a2314, a2315)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f229(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f459({
                let (_, h2316) = v4;
                h2316
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f464({
                    let (_, h2317) = v4;
                    h2317
                })?, {
                    let a2319 = {
                        let (h2318, _) = v4;
                        h2318
                    };
                    let a2320 = 1u64;
                    nat_add(a2319, a2320)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f229(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f232(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a2321 = v1;
                let a2322 = 4u64;
                nat_add(a2321, a2322)?
            })))
        } else {
            if f232(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a2323 = v1;
                    let a2324 = 4u64;
                    nat_add(a2323, a2324)?
                })))
            } else {
                if f232(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a2325 = v1;
                        let a2326 = 5u64;
                        nat_add(a2325, a2326)?
                    })))
                } else {
                    if f232(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a2327 = v1;
                            let a2328 = 6u64;
                            nat_add(a2327, a2328)?
                        })))
                    } else {
                        if f232(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a2329 = v1;
                                let a2330 = 6u64;
                                nat_add(a2329, a2330)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f229(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f407(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2331, _) = f274(Fn3::F465(v1, v2, v0.clone()), {
        let a2333 = {
            let a2332 = v0;
            length_bytes(a2332)
        };
        let a2334 = 1u64;
        nat_add(a2333, a2334)?
    }, v3)?;
    Ok(h2331)
}

pub fn f408(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2335, _) = v2;
        h2335
    };
    let a2336 = v3;
    let a2337 = 0u64;
    if nat_eq(a2336, a2337) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2338 = v3;
            let a2339 = v1;
            let a2340 = 0u64;
            nat_quot(a2338, a2339, a2340)
        }, {
            let a2345 = f337(v0, {
                let a2341 = v3;
                let a2342 = v1;
                let a2343 = 0u64;
                nat_rem(a2341, a2342, a2343)
            });
            let a2346 = {
                let (_, h2344) = v2;
                h2344
            };
            append_bytes(a2345, a2346)
        }))
    }
}

pub fn f409(p2357: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>> {
    let m2356 = p2357;
    {
        let mut m2356 = m2356;
        loop {
            let step = {
                let v0 = m2356;
                let m2347 = v0;
                match m2347 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C1(_, _, _) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C3(r2354, r2355) => {
                        let v2 = (*r2354).clone();
                        let v3 = (*r2355).clone();
                        let m2348 = v2;
                        match m2348 {
                            Adt18::C0(v4, r2352, r2353) => {
                                let v5 = (*r2352).clone();
                                let v6 = (*r2353).clone();
                                let m2349 = f237(v5.clone(), v1.clone());
                                match m2349 {
                                    Ok(v8) => {
                                        let m2350 = f238(v8.clone(), v4)?;
                                        match m2350 {
                                            Ok(v10) => {
                                                if if f239(v10.clone(), Str::lit("restriction")) {
                                                    true
                                                } else {
                                                    if f239(v10.clone(), Str::lit("list")) {
                                                        true
                                                    } else {
                                                        f239(v10.clone(), Str::lit("union"))
                                                    }
                                                } {
                                                    Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(Some(({
                                                        let (_, h2351) = v10;
                                                        h2351
                                                    }, (v5, (v6, v8)))))))
                                                } else {
                                                    Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Err::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Err::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(v7)))
                                    }
                                }
                            }
                            Adt18::C1(_, _, _) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C2(_) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C3(_, _) => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                            Adt18::C4 => {
                                Ok::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt18::C4 => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                }
            };
            match step {
                Ok(next) => { m2356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f411(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m2358 = f238(v0, v2.clone())?;
    match m2358 {
        Ok(v4) => {
            if f37({
                let (h2359, _) = v4.clone();
                h2359
            }, f344()) {
                let m2361 = f466({
                    let (_, h2360) = v4;
                    h2360
                }, 16u64);
                match m2361 {
                    Ok(v6) => {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v5))
                    }
                }
            } else {
                if f37({
                    let (h2362, _) = v4.clone();
                    h2362
                }, v1) {
                    let v7: Bytes = f260({
                        let (_, h2363) = v4;
                        h2363
                    })?;
                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2364, _) = v4;
                        h2364
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(v3))
        }
    }
}

pub fn f412(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f413(v0: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt19>, Bytes>> {
    let m2365 = v0;
    match m2365 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C3(r2372, r2373) => {
            let v2 = (*r2372).clone();
            let v3 = (*r2373).clone();
            let m2366 = f413(v3, v1.clone())?;
            match m2366 {
                Ok(v5) => {
                    let m2367 = v2;
                    match m2367 {
                        Adt18::C0(v6, r2371, _) => {
                            let v7 = (*r2371).clone();
                            let m2368 = f237(v7.clone(), v1);
                            match m2368 {
                                Ok(v10) => {
                                    let m2369 = f238(v10, v6)?;
                                    match m2369 {
                                        Ok(v12) => {
                                            if f239(v12, Str::lit("enumeration")) {
                                                let m2370 = f240(v7, f8(Str::lit("value")));
                                                match m2370 {
                                                    None => {
                                                        Ok(Err::<List<Adt19>, Bytes>(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt19>, Bytes>(List::cons(Adt19::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt19>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt19>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt19>, Bytes>(v9))
                                }
                            }
                        }
                        Adt18::C1(_, _, _) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C2(_) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C3(_, _) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt18::C4 => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt19>, Bytes>(v4))
                }
            }
        }
        Adt18::C4 => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
    }
}

pub fn f414(v0: List<Adt19>, v1: List<Bytes>) -> R<List<Adt19>> {
    let m2374 = v0;
    match m2374.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f467(v2.clone())?;
            if f88(v4.clone(), v1.clone()) {
                f414(v3, v1)
            } else {
                Ok(List::cons(v2, f414(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f415(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2375 = v1;
    match m2375.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2376 = f468(v0, v3.clone())?;
            match m2376 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f467(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f88(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2377 = f415(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2377 {
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

pub fn f416(p2381: List<Bytes>) -> R<bool> {
    let m2380 = p2381;
    {
        let mut m2380 = m2380;
        loop {
            let step = {
                let v0 = m2380;
                let m2378 = v0;
                match m2378.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2379 = f469(v1)?;
                        match m2379 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f88(v3, f470(false, v2.clone())?) {
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
                Ok(next) => { m2380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f221(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f418(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2382 = v2;
    let a2383 = v0;
    if nat_lt(a2382, a2383) {
        if f457(v1, v2) {
            Ok(Some({
                let a2384 = v2;
                let a2385 = 1u64;
                nat_add(a2384, a2385)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f419(v0: Bytes, p2465: u64, p2466: u64, p2467: bool, p2468: bool, p2469: Bytes) -> R<Bytes> {
    let m2464 = (p2465, (p2466, (p2467, (p2468, p2469))));
    {
        let mut m2464 = m2464;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m2464;
                let m2386 = v2;
                if m2386 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m2386 - 1;
                    let m2389 = {
                        let a2387 = v0.clone();
                        let a2388 = v1;
                        index_bytes(a2387, a2388)
                    };
                    match m2389 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a2434 = {
                                    let a2410 = {
                                        let a2398 = {
                                            let a2392 = {
                                                let a2390 = v8;
                                                let a2391 = 128u8;
                                                fixed_u8::bit_and(a2390, a2391)
                                            };
                                            let a2393 = 128u8;
                                            if equal(a2392, a2393) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2399 = {
                                            let a2396 = {
                                                let a2394 = v8;
                                                let a2395 = 64u8;
                                                fixed_u8::bit_and(a2394, a2395)
                                            };
                                            let a2397 = 64u8;
                                            if equal(a2396, a2397) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2398, a2399)?
                                    };
                                    let a2411 = {
                                        let a2408 = {
                                            let a2402 = {
                                                let a2400 = v8;
                                                let a2401 = 32u8;
                                                fixed_u8::bit_and(a2400, a2401)
                                            };
                                            let a2403 = 32u8;
                                            if equal(a2402, a2403) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2409 = {
                                            let a2406 = {
                                                let a2404 = v8;
                                                let a2405 = 16u8;
                                                fixed_u8::bit_and(a2404, a2405)
                                            };
                                            let a2407 = 16u8;
                                            if equal(a2406, a2407) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2408, a2409)?
                                    };
                                    nat_add(a2410, a2411)?
                                };
                                let a2435 = {
                                    let a2432 = {
                                        let a2420 = {
                                            let a2414 = {
                                                let a2412 = v8;
                                                let a2413 = 8u8;
                                                fixed_u8::bit_and(a2412, a2413)
                                            };
                                            let a2415 = 8u8;
                                            if equal(a2414, a2415) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2421 = {
                                            let a2418 = {
                                                let a2416 = v8;
                                                let a2417 = 4u8;
                                                fixed_u8::bit_and(a2416, a2417)
                                            };
                                            let a2419 = 4u8;
                                            if equal(a2418, a2419) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2420, a2421)?
                                    };
                                    let a2433 = {
                                        let a2430 = {
                                            let a2424 = {
                                                let a2422 = v8;
                                                let a2423 = 2u8;
                                                fixed_u8::bit_and(a2422, a2423)
                                            };
                                            let a2425 = 2u8;
                                            if equal(a2424, a2425) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2431 = {
                                            let a2428 = {
                                                let a2426 = v8;
                                                let a2427 = 1u8;
                                                fixed_u8::bit_and(a2426, a2427)
                                            };
                                            let a2429 = 1u8;
                                            if equal(a2428, a2429) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2430, a2431)?
                                    };
                                    nat_add(a2432, a2433)?
                                };
                                nat_add(a2434, a2435)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a2436 = v1;
                                    let a2437 = 1u64;
                                    nat_add(a2436, a2437)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a2438 = v9;
                                    let a2439 = 34u64;
                                    !nat_eq(a2438, a2439)
                                }, (if v4 {
                                    false
                                } else {
                                    let a2440 = v9;
                                    let a2441 = 92u64;
                                    nat_eq(a2440, a2441)
                                }, {
                                    let a2442 = v5;
                                    let a2443 = v10;
                                    append_bytes(a2442, a2443)
                                })))))
                            } else {
                                let a2444 = v9;
                                let a2445 = 32u64;
                                if if if if nat_eq(a2444, a2445) {
                                    true
                                } else {
                                    let a2446 = v9;
                                    let a2447 = 9u64;
                                    nat_eq(a2446, a2447)
                                } {
                                    true
                                } else {
                                    let a2448 = v9;
                                    let a2449 = 10u64;
                                    nat_eq(a2448, a2449)
                                } {
                                    true
                                } else {
                                    let a2450 = v9;
                                    let a2451 = 13u64;
                                    nat_eq(a2450, a2451)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a2452 = v1;
                                        let a2453 = 1u64;
                                        nat_add(a2452, a2453)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2454 = v1;
                                            let a2455 = 4u64;
                                            nat_add(a2454, a2455)?
                                        }, (v6, (false, (false, {
                                            let a2456 = v5;
                                            let a2457 = f8(Str::lit("[]"));
                                            append_bytes(a2456, a2457)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2458 = v1;
                                            let a2459 = 1u64;
                                            nat_add(a2458, a2459)?
                                        }, (v6, ({
                                            let a2460 = v9;
                                            let a2461 = 34u64;
                                            nat_eq(a2460, a2461)
                                        }, (false, {
                                            let a2462 = v5;
                                            let a2463 = v10;
                                            append_bytes(a2462, a2463)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f471(Fn15::F472(v0.clone()), {
        let a2471 = {
            let a2470 = v0;
            length_bytes(a2470)
        };
        let a2472 = 1u64;
        nat_add(a2471, a2472)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a2476 = {
        let (_, h2473) = {
            let (_, h2474) = {
                let (h2475, _) = v2.clone();
                h2475
            };
            h2474
        };
        h2473
    };
    let a2477 = 1u64;
    if nat_eq(a2476, a2477) {
        Ok(Some(({
            let (h2478, _) = {
                let (h2479, _) = v2.clone();
                h2479
            };
            h2478
        }, {
            let (h2480, _) = {
                let (_, h2481) = {
                    let (h2482, _) = v2;
                    h2482
                };
                h2481
            };
            h2480
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f421(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2483 = f333(v0.clone(), v1)?;
        let a2484 = 45u64;
        nat_eq(a2483, a2484)
    };
    let v3: u64 = if v2 {
        let a2485 = v1;
        let a2486 = 1u64;
        nat_add(a2485, a2486)?
    } else {
        v1
    };
    let v4: u64 = f473(v0.clone(), v3)?;
    let v5: u64 = {
        let a2487 = v4;
        let a2488 = v3;
        nat_sub(a2487, a2488)
    };
    let v6: bool = {
        let a2489 = f333(v0.clone(), v4)?;
        let a2490 = 46u64;
        nat_eq(a2489, a2490)
    };
    let v7: u64 = if v6 {
        let a2491 = v4;
        let a2492 = 1u64;
        nat_add(a2491, a2492)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f473(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f333(v0.clone(), v8)?;
    let v10: bool = {
        let a2493 = v9;
        let a2494 = 101u64;
        if nat_eq(a2493, a2494) {
            true
        } else {
            let a2495 = v9;
            let a2496 = 69u64;
            nat_eq(a2495, a2496)
        }
    };
    let v11: u64 = f333(v0.clone(), {
        let a2497 = v8;
        let a2498 = 1u64;
        nat_add(a2497, a2498)?
    })?;
    let v12: bool = if v10 {
        let a2499 = v11;
        let a2500 = 43u64;
        if nat_eq(a2499, a2500) {
            true
        } else {
            let a2501 = v11;
            let a2502 = 45u64;
            nat_eq(a2501, a2502)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2503 = v8;
            let a2504 = 2u64;
            nat_add(a2503, a2504)?
        } else {
            let a2505 = v8;
            let a2506 = 1u64;
            nat_add(a2505, a2506)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f473(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2507 = v14;
        let a2508 = v13;
        nat_sub(a2507, a2508)
    });
    let v16: u64 = f474(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2510 = {
            let a2509 = v15;
            length_bytes(a2509)
        };
        let a2511 = v16;
        nat_sub(a2510, a2511)
    });
    let a2512 = v5;
    let a2513 = 0u64;
    if if nat_eq(a2512, a2513) {
        true
    } else {
        let a2514 = 1u64;
        let a2515 = v5;
        if if nat_lt(a2514, a2515) {
            let a2516 = f333(v0.clone(), v3)?;
            let a2517 = 48u64;
            nat_eq(a2516, a2517)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2518 = v8;
                let a2519 = v7;
                nat_eq(a2518, a2519)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2520 = v14;
                    let a2521 = v13;
                    nat_eq(a2520, a2521)
                } else {
                    false
                } {
                    true
                } else {
                    let a2523 = 18u64;
                    let a2524 = {
                        let a2522 = v17.clone();
                        length_bytes(a2522)
                    };
                    nat_lt(a2523, a2524)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2528 = {
            let a2526 = {
                let a2525 = v17.clone();
                length_bytes(a2525)
            };
            let a2527 = 0u64;
            if nat_eq(a2526, a2527) {
                Some(0u64)
            } else {
                f475(v17)?
            }
        };
        match m2528 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2533 = f476(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2529 = v8;
                    let a2530 = v7;
                    nat_sub(a2529, a2530)
                }), if v12 {
                    let a2531 = v11;
                    let a2532 = 45u64;
                    nat_eq(a2531, a2532)
                } else {
                    false
                }, v18)?;
                match m2533 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2535 = 19u64;
                        let a2536 = {
                            let a2534 = v19.clone();
                            length_bytes(a2534)
                        };
                        if nat_lt(a2535, a2536) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2537 = f475(v19)?;
                            match m2537 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2538 = 0u64;
                                        let a2539 = v20;
                                        nat_lt(a2538, a2539)
                                    } else {
                                        false
                                    };
                                    if f477(v21, v20) {
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

pub fn f422(v0: u64) -> bool {
    let a2540 = v0;
    let a2541 = 47u64;
    nat_eq(a2540, a2541)
}

pub fn f423(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2542 = f333(v0.clone(), v1)?;
        let a2543 = 45u64;
        nat_eq(a2542, a2543)
    };
    let v3: u64 = if v2 {
        let a2544 = v1;
        let a2545 = 1u64;
        nat_add(a2544, a2545)?
    } else {
        v1
    };
    let v4: u64 = f473(v0.clone(), v3)?;
    let v5: u64 = {
        let a2546 = v4;
        let a2547 = v3;
        nat_sub(a2546, a2547)
    };
    let v6: bool = {
        let a2548 = f333(v0.clone(), v4)?;
        let a2549 = 46u64;
        nat_eq(a2548, a2549)
    };
    let v7: u64 = if v6 {
        let a2550 = v4;
        let a2551 = 1u64;
        nat_add(a2550, a2551)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f473(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f333(v0.clone(), v8)?;
    let v10: bool = {
        let a2552 = v9;
        let a2553 = 101u64;
        if nat_eq(a2552, a2553) {
            true
        } else {
            let a2554 = v9;
            let a2555 = 69u64;
            nat_eq(a2554, a2555)
        }
    };
    let v11: u64 = f333(v0.clone(), {
        let a2556 = v8;
        let a2557 = 1u64;
        nat_add(a2556, a2557)?
    })?;
    let v12: bool = if v10 {
        let a2558 = v11;
        let a2559 = 43u64;
        if nat_eq(a2558, a2559) {
            true
        } else {
            let a2560 = v11;
            let a2561 = 45u64;
            nat_eq(a2560, a2561)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2562 = v8;
            let a2563 = 2u64;
            nat_add(a2562, a2563)?
        } else {
            let a2564 = v8;
            let a2565 = 1u64;
            nat_add(a2564, a2565)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f473(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2566 = v14;
        let a2567 = v13;
        nat_sub(a2566, a2567)
    });
    let v16: u64 = f474(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2569 = {
            let a2568 = v15;
            length_bytes(a2568)
        };
        let a2570 = v16;
        nat_sub(a2569, a2570)
    });
    let a2571 = v5;
    let a2572 = 0u64;
    if if nat_eq(a2571, a2572) {
        true
    } else {
        let a2573 = 1u64;
        let a2574 = v5;
        if if nat_lt(a2573, a2574) {
            let a2575 = f333(v0.clone(), v3)?;
            let a2576 = 48u64;
            nat_eq(a2575, a2576)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2577 = v8;
                let a2578 = v7;
                nat_eq(a2577, a2578)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2579 = v14;
                    let a2580 = v13;
                    nat_eq(a2579, a2580)
                } else {
                    false
                } {
                    true
                } else {
                    let a2582 = 18u64;
                    let a2583 = {
                        let a2581 = v17.clone();
                        length_bytes(a2581)
                    };
                    nat_lt(a2582, a2583)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2587 = {
            let a2585 = {
                let a2584 = v17.clone();
                length_bytes(a2584)
            };
            let a2586 = 0u64;
            if nat_eq(a2585, a2586) {
                Some(0u64)
            } else {
                f475(v17)?
            }
        };
        match m2587 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2592 = f478(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2588 = v8;
                    let a2589 = v7;
                    nat_sub(a2588, a2589)
                }), if v12 {
                    let a2590 = v11;
                    let a2591 = 45u64;
                    nat_eq(a2590, a2591)
                } else {
                    false
                }, v18)?;
                match m2592 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2595 = 19u64;
                        let a2596 = {
                            let a2594 = {
                                let (h2593, _) = v19.clone();
                                h2593
                            };
                            length_bytes(a2594)
                        };
                        if nat_lt(a2595, a2596) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2598 = f475({
                                let (h2597, _) = v19.clone();
                                h2597
                            })?;
                            match m2598 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2599 = 0u64;
                                        let a2600 = v20;
                                        nat_lt(a2599, a2600)
                                    } else {
                                        false
                                    };
                                    if f477(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2602 = {
                                                let (_, h2601) = v19.clone();
                                                h2601
                                            };
                                            let a2603 = 0u64;
                                            if nat_eq(a2602, a2603) {
                                                Adt19::C1(v21, v20)
                                            } else {
                                                Adt19::C2(v21, v20, {
                                                    let (_, h2604) = v19;
                                                    h2604
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

pub fn f424(v0: u64) -> bool {
    let a2605 = v0;
    let a2606 = 95u64;
    if nat_eq(a2605, a2606) {
        true
    } else {
        let a2607 = v0;
        let a2608 = 45u64;
        if nat_eq(a2607, a2608) {
            true
        } else {
            let a2609 = v0;
            let a2610 = 32u64;
            if nat_eq(a2609, a2610) {
                true
            } else {
                let a2611 = v0;
                let a2612 = 46u64;
                if nat_eq(a2611, a2612) {
                    true
                } else {
                    let a2613 = v0;
                    let a2614 = 47u64;
                    nat_eq(a2613, a2614)
                }
            }
        }
    }
}

pub fn f425(v0: u64) -> bool {
    let a2615 = 97u64;
    let a2616 = v0;
    if nat_le(a2615, a2616) {
        let a2617 = v0;
        let a2618 = 122u64;
        nat_le(a2617, a2618)
    } else {
        false
    }
}

pub fn f426(v0: u64) -> bool {
    let a2619 = 65u64;
    let a2620 = v0;
    if nat_le(a2619, a2620) {
        let a2621 = v0;
        let a2622 = 90u64;
        nat_le(a2621, a2622)
    } else {
        false
    }
}

pub fn f427(v0: u64) -> bool {
    let a2623 = 48u64;
    let a2624 = v0;
    if nat_le(a2623, a2624) {
        let a2625 = v0;
        let a2626 = 57u64;
        nat_le(a2625, a2626)
    } else {
        false
    }
}

pub fn f428(v0: List<(Bytes, Adt19)>, v1: List<(Bytes, Adt19)>) -> List<(Bytes, Adt19)> {
    let m2627 = v0;
    match m2627.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f428(v3, v1))
        }
    }
}

pub fn f429(v0: Option<Adt19>) -> List<(Bytes, Adt19)> {
    let m2628 = v0;
    match m2628 {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some(v1) => {
            f479(v1)
        }
    }
}

pub fn f430(v0: u64, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt19)>, Bytes>> {
    let m2629 = v0;
    if m2629 == 0 {
        let m2630 = v1;
        match m2630.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2629 - 1;
        let m2631 = v1.clone();
        match m2631.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                let m2632 = f480(v1.clone(), v2.clone(), v3.clone())?;
                match m2632 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f221(f431(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2635 = f430(v6, f481(v1, {
                            let (h2633, _) = v9.clone();
                            h2633
                        }), List::cons(f260({
                            let (h2634, _) = v9.clone();
                            h2634
                        })?, v2), v3)?;
                        match m2635 {
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

pub fn f431(v0: List<(Bytes, Adt19)>) -> R<List<Bytes>> {
    let m2636 = v0;
    match m2636.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f260({
                let (h2637, _) = v1;
                h2637
            })?, f431(v2)?))
        }
    }
}

pub fn f432(p2645: u64, p2646: List<Adt25>, p2647: List<Bytes>, p2648: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2644 = (p2645, (p2646, (p2647, p2648)));
    {
        let mut m2644 = m2644;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2644;
                let m2638 = v0;
                if m2638 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2638 - 1;
                    let m2639 = v1;
                    match m2639.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2640 = v5;
                            match m2640 {
                                Adt25::C0(v8, v9) => {
                                    if f37(v8.clone(), f482()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f88(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2641 = f484(v8.clone(), v9)?;
                                            match m2641 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f433({
                                                        let (_, h2642) = v11;
                                                        h2642
                                                    }, List::cons(Adt25::C1({
                                                        let (h2643, _) = v11;
                                                        h2643
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
                                    if f88(f482(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f482(), v2), List::cons(f483(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2644 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2649 = v0;
    match m2649.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f433(v3, v1))
        }
    }
}

pub fn f434(v0: List<(Bytes, Adt19)>) -> R<List<Adt25>> {
    let m2650 = v0;
    match m2650.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f260({
                let (h2651, _) = v1;
                h2651
            })?, {
                let (_, h2652) = v1;
                h2652
            }), f434(v2)?))
        }
    }
}

pub fn f435(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2653 = v0;
    match m2653.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f37(f163(v3.clone(), v1.clone()), v2.clone()) {
                f435(v4, v1, v2)
            } else {
                List::cons(v3, f435(v4, v1, v2))
            }
        }
    }
}

pub fn f436(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2654 = v0;
    match m2654.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f211(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f221(f444(f80(List::cons(v1, f485(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f436(v3, v1)?))
            }
        }
    }
}

pub fn f437(v0: u64) -> Bytes {
    let m2655 = v0;
    if m2655 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2655 - 1;
        let a2656 = f8(Str::lit("0"));
        let a2657 = f437(v1);
        append_bytes(a2656, a2657)
    }
}

pub fn f438(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f486(v0, v1)
}

pub fn f439(v0: Fn10, p2661: List<(Bytes, Bytes)>, p2662: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2660 = (p2661, p2662);
    {
        let mut m2660 = m2660;
        loop {
            let step = {
                let (v1, v2) = m2660;
                let m2658 = v2;
                match m2658.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2659: Fn10 = v0.clone();
                            c2659.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f487(v0, v1)
}

pub fn f441(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f488(v0)
}

pub fn f442(v0: Fn11, p2666: List<Bytes>, p2667: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2665 = (p2666, p2667);
    {
        let mut m2665 = m2665;
        loop {
            let step = {
                let (v1, v2) = m2665;
                let m2663 = v2;
                match m2663.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2664: Fn11 = v0.clone();
                            c2664.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f489(v1)?, v0))
}

pub fn f444(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2668 = v0;
    match m2668.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f216(v1)?, f444(v2)?))
        }
    }
}

pub fn f445(v0: Fn12, p2672: List<List<(Bytes, ())>>, p2673: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2671 = (p2672, p2673);
    {
        let mut m2671 = m2671;
        loop {
            let step = {
                let (v1, v2) = m2671;
                let m2669 = v2;
                match m2669.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2670: Fn12 = v0.clone();
                            c2670.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f447(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2675 = {
        let a2674 = v0.clone();
        length_list(a2674)
    };
    let a2676 = 1u64;
    if nat_le(a2675, a2676) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f490(v0)?))
    }
}

pub fn f448(v0: Fn13, p2680: List<(Bytes, ())>, p2681: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2679 = (p2680, p2681);
    {
        let mut m2679 = m2679;
        loop {
            let step = {
                let (v1, v2) = m2679;
                let m2677 = v2;
                match m2677.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2678: Fn13 = v0.clone();
                            c2678.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f450(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f74(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f491(v3.clone(), f74(f75(v0, f91())?, f8(Str::lit("at"))));
    let a2683 = {
        let a2682 = v4.clone();
        length_list(a2682)
    };
    let a2684 = 0u64;
    if nat_eq(a2683, a2684) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2687 = {
            let a2685 = v4;
            length_list(a2685)
        };
        let a2688 = {
            let a2686 = v3;
            length_list(a2686)
        };
        if nat_eq(a2687, a2688) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f492(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f451(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f255(f146(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f255(f146(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f66(f74(v3.clone(), f8(Str::lit("id"))), f74(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f74(f75(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f491(v5.clone(), f66(v6.clone(), f74(f75(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2690 = {
        let a2689 = v7.clone();
        length_list(a2689)
    };
    let a2691 = 0u64;
    if nat_eq(a2690, a2691) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f77({
            let a2694 = {
                let a2692 = v7;
                length_list(a2692)
            };
            let a2695 = {
                let a2693 = v5;
                length_list(a2693)
            };
            nat_eq(a2694, a2695)
        }) {
            Ok((false, f492(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f80(f66(f491(f493(v4.clone()), f71(f72(v0, f8(Str::lit("model/authorities.toml"))))?), f491(f494(v4), f66(v6, f74(v3, f8(Str::lit("id")))))))?;
            let a2697 = {
                let a2696 = v8.clone();
                length_list(a2696)
            };
            let a2698 = 0u64;
            if nat_eq(a2697, a2698) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f492(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f452(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f37({
        let (h2699, _) = v0;
        h2699
    }, {
        let (h2700, _) = v1;
        h2700
    }) {
        f37({
            let (_, h2701) = v0;
            h2701
        }, {
            let (_, h2702) = v1;
            h2702
        })
    } else {
        false
    }
}

pub fn f453(v0: Fn14, p2706: u64, p2707: Adt27) -> R<Adt27> {
    let m2705 = (p2706, p2707);
    {
        let mut m2705 = m2705;
        loop {
            let step = {
                let (v1, v2) = m2705;
                let m2703 = v1;
                if m2703 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2703 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2704: Fn14 = v0.clone();
                        c2704.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: Adt27) -> R<Adt27> {
    f495(v0)
}

pub fn f455(v0: u32, v1: u32) -> u32 {
    let m2710 = {
        let a2708 = f496(v0);
        let a2709 = f496(v1);
        fixed_u64::checked_add(a2708, a2709)
    };
    match m2710 {
        None => {
            0u32
        }
        Some(v2) => {
            f497(v2)
        }
    }
}

pub fn f456(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2713 = {
        let a2711 = v3;
        let a2712 = 8u64;
        nat_add(a2711, a2712)?
    };
    let a2714 = v2;
    if nat_eq(a2713, a2714) {
        Ok(f498({
            let a2717 = {
                let a2715 = v1;
                let a2716 = 8u64;
                nat_mul(a2715, a2716)?
            };
            let a2718 = 4294967296u64;
            let a2719 = 0u64;
            nat_quot(a2717, a2718, a2719)
        }))
    } else {
        let a2722 = {
            let a2720 = v3;
            let a2721 = 4u64;
            nat_add(a2720, a2721)?
        };
        let a2723 = v2;
        if nat_eq(a2722, a2723) {
            Ok(f498({
                let a2726 = {
                    let a2724 = v1;
                    let a2725 = 8u64;
                    nat_mul(a2724, a2725)?
                };
                let a2727 = 4294967296u64;
                let a2728 = 0u64;
                nat_rem(a2726, a2727, a2728)
            }))
        } else {
            Ok(f499(f500(v0.clone(), v1, v3), f500(v0.clone(), v1, {
                let a2729 = v3;
                let a2730 = 1u64;
                nat_add(a2729, a2730)?
            }), f500(v0.clone(), v1, {
                let a2731 = v3;
                let a2732 = 2u64;
                nat_add(a2731, a2732)?
            }), f500(v0, v1, {
                let a2733 = v3;
                let a2734 = 3u64;
                nat_add(a2733, a2734)?
            })))
        }
    }
}

pub fn f457(v0: Bytes, v1: u64) -> bool {
    let m2735 = f271(v0, v1);
    match m2735 {
        None => {
            false
        }
        Some(v2) => {
            f501(v2)
        }
    }
}

pub fn f458(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2738 = {
        let a2736 = v0.clone();
        let a2737 = v1;
        index_bytes(a2736, a2737)
    };
    match m2738 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2783 = {
                    let a2759 = {
                        let a2747 = {
                            let a2741 = {
                                let a2739 = v3;
                                let a2740 = 128u8;
                                fixed_u8::bit_and(a2739, a2740)
                            };
                            let a2742 = 128u8;
                            if equal(a2741, a2742) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2748 = {
                            let a2745 = {
                                let a2743 = v3;
                                let a2744 = 64u8;
                                fixed_u8::bit_and(a2743, a2744)
                            };
                            let a2746 = 64u8;
                            if equal(a2745, a2746) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2747, a2748)?
                    };
                    let a2760 = {
                        let a2757 = {
                            let a2751 = {
                                let a2749 = v3;
                                let a2750 = 32u8;
                                fixed_u8::bit_and(a2749, a2750)
                            };
                            let a2752 = 32u8;
                            if equal(a2751, a2752) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2758 = {
                            let a2755 = {
                                let a2753 = v3;
                                let a2754 = 16u8;
                                fixed_u8::bit_and(a2753, a2754)
                            };
                            let a2756 = 16u8;
                            if equal(a2755, a2756) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2757, a2758)?
                    };
                    nat_add(a2759, a2760)?
                };
                let a2784 = {
                    let a2781 = {
                        let a2769 = {
                            let a2763 = {
                                let a2761 = v3;
                                let a2762 = 8u8;
                                fixed_u8::bit_and(a2761, a2762)
                            };
                            let a2764 = 8u8;
                            if equal(a2763, a2764) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2770 = {
                            let a2767 = {
                                let a2765 = v3;
                                let a2766 = 4u8;
                                fixed_u8::bit_and(a2765, a2766)
                            };
                            let a2768 = 4u8;
                            if equal(a2767, a2768) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2769, a2770)?
                    };
                    let a2782 = {
                        let a2779 = {
                            let a2773 = {
                                let a2771 = v3;
                                let a2772 = 2u8;
                                fixed_u8::bit_and(a2771, a2772)
                            };
                            let a2774 = 2u8;
                            if equal(a2773, a2774) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2780 = {
                            let a2777 = {
                                let a2775 = v3;
                                let a2776 = 1u8;
                                fixed_u8::bit_and(a2775, a2776)
                            };
                            let a2778 = 1u8;
                            if equal(a2777, a2778) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2779, a2780)?
                    };
                    nat_add(a2781, a2782)?
                };
                nat_add(a2783, a2784)?
            };
            let a2785 = v4;
            let a2786 = 128u64;
            if nat_lt(a2785, a2786) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2787 = 194u64;
                let a2788 = v4;
                if if nat_le(a2787, a2788) {
                    let a2789 = v4;
                    let a2790 = 223u64;
                    nat_le(a2789, a2790)
                } else {
                    false
                } {
                    let m2793 = f502(v0, {
                        let a2791 = v1;
                        let a2792 = 1u64;
                        nat_add(a2791, a2792)?
                    })?;
                    match m2793 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f503({
                                let a2798 = {
                                    let a2796 = {
                                        let a2794 = v4;
                                        let a2795 = 192u64;
                                        nat_sub(a2794, a2795)
                                    };
                                    let a2797 = 64u64;
                                    nat_mul(a2796, a2797)?
                                };
                                let a2799 = v5;
                                nat_add(a2798, a2799)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2800 = 224u64;
                    let a2801 = v4;
                    if if nat_le(a2800, a2801) {
                        let a2802 = v4;
                        let a2803 = 239u64;
                        nat_le(a2802, a2803)
                    } else {
                        false
                    } {
                        let m2806 = f502(v0.clone(), {
                            let a2804 = v1;
                            let a2805 = 1u64;
                            nat_add(a2804, a2805)?
                        })?;
                        match m2806 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2809 = f502(v0, {
                                    let a2807 = v1;
                                    let a2808 = 2u64;
                                    nat_add(a2807, a2808)?
                                })?;
                                match m2809 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f503({
                                            let a2818 = {
                                                let a2812 = {
                                                    let a2810 = v4;
                                                    let a2811 = 224u64;
                                                    nat_sub(a2810, a2811)
                                                };
                                                let a2813 = 4096u64;
                                                nat_mul(a2812, a2813)?
                                            };
                                            let a2819 = {
                                                let a2816 = {
                                                    let a2814 = v6;
                                                    let a2815 = 64u64;
                                                    nat_mul(a2814, a2815)?
                                                };
                                                let a2817 = v7;
                                                nat_add(a2816, a2817)?
                                            };
                                            nat_add(a2818, a2819)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2820 = 240u64;
                        let a2821 = v4;
                        if if nat_le(a2820, a2821) {
                            let a2822 = v4;
                            let a2823 = 244u64;
                            nat_le(a2822, a2823)
                        } else {
                            false
                        } {
                            let m2826 = f502(v0.clone(), {
                                let a2824 = v1;
                                let a2825 = 1u64;
                                nat_add(a2824, a2825)?
                            })?;
                            match m2826 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2829 = f502(v0.clone(), {
                                        let a2827 = v1;
                                        let a2828 = 2u64;
                                        nat_add(a2827, a2828)?
                                    })?;
                                    match m2829 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2832 = f502(v0, {
                                                let a2830 = v1;
                                                let a2831 = 3u64;
                                                nat_add(a2830, a2831)?
                                            })?;
                                            match m2832 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f503({
                                                        let a2845 = {
                                                            let a2835 = {
                                                                let a2833 = v4;
                                                                let a2834 = 240u64;
                                                                nat_sub(a2833, a2834)
                                                            };
                                                            let a2836 = 262144u64;
                                                            nat_mul(a2835, a2836)?
                                                        };
                                                        let a2846 = {
                                                            let a2843 = {
                                                                let a2837 = v8;
                                                                let a2838 = 4096u64;
                                                                nat_mul(a2837, a2838)?
                                                            };
                                                            let a2844 = {
                                                                let a2841 = {
                                                                    let a2839 = v9;
                                                                    let a2840 = 64u64;
                                                                    nat_mul(a2839, a2840)?
                                                                };
                                                                let a2842 = v10;
                                                                nat_add(a2841, a2842)?
                                                            };
                                                            nat_add(a2843, a2844)?
                                                        };
                                                        nat_add(a2845, a2846)?
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

pub fn f459(v0: u64) -> bool {
    let a2847 = v0;
    let a2848 = 9u64;
    if if if nat_eq(a2847, a2848) {
        true
    } else {
        let a2849 = v0;
        let a2850 = 10u64;
        nat_eq(a2849, a2850)
    } {
        true
    } else {
        let a2851 = v0;
        let a2852 = 13u64;
        nat_eq(a2851, a2852)
    } {
        true
    } else {
        let a2853 = 32u64;
        let a2854 = v0;
        if if if nat_le(a2853, a2854) {
            let a2855 = v0;
            let a2856 = 55295u64;
            nat_le(a2855, a2856)
        } else {
            false
        } {
            true
        } else {
            let a2857 = 57344u64;
            let a2858 = v0;
            if nat_le(a2857, a2858) {
                let a2859 = v0;
                let a2860 = 65533u64;
                nat_le(a2859, a2860)
            } else {
                false
            }
        } {
            true
        } else {
            let a2861 = 65536u64;
            let a2862 = v0;
            if nat_le(a2861, a2862) {
                let a2863 = v0;
                let a2864 = 1114111u64;
                nat_le(a2863, a2864)
            } else {
                false
            }
        }
    }
}

pub fn f460(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f226(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2866 = v2;
        let a2867 = {
            let a2865 = v0;
            length_bytes(a2865)
        };
        if nat_lt(a2866, a2867) {
            Ok(Some({
                let a2868 = v2;
                let a2869 = 1u64;
                nat_add(a2868, a2869)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f461(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f327(f233(v0, v1)?) {
        Ok(Some({
            let a2870 = v1;
            let a2871 = 1u64;
            nat_add(a2870, a2871)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f462(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2872 = f458(v0, v2)?;
    match m2872 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2873 = v2;
            let a2874 = v1;
            if if nat_eq(a2873, a2874) {
                f504({
                    let (h2875, _) = v3;
                    h2875
                })
            } else {
                f505({
                    let (h2876, _) = v3;
                    h2876
                })
            } {
                Ok(Some({
                    let a2878 = v2;
                    let a2879 = {
                        let (_, h2877) = v3;
                        h2877
                    };
                    nat_add(a2878, a2879)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f463(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2880, _) = f506(Fn16::F507(v1, v0.clone()), {
        let a2882 = {
            let a2881 = v0;
            length_bytes(a2881)
        };
        let a2883 = 1u64;
        nat_add(a2882, a2883)?
    }, (v2, 0u64))?;
    Ok(h2880)
}

pub fn f464(v0: u64) -> R<Bytes> {
    let a2884 = v0;
    let a2885 = 128u64;
    if nat_lt(a2884, a2885) {
        Ok(f508(v0))
    } else {
        let a2886 = v0;
        let a2887 = 2048u64;
        if nat_lt(a2886, a2887) {
            let a2898 = f508({
                let a2891 = 192u64;
                let a2892 = {
                    let a2888 = v0;
                    let a2889 = 64u64;
                    let a2890 = 0u64;
                    nat_quot(a2888, a2889, a2890)
                };
                nat_add(a2891, a2892)?
            });
            let a2899 = f508({
                let a2896 = 128u64;
                let a2897 = {
                    let a2893 = v0;
                    let a2894 = 64u64;
                    let a2895 = 0u64;
                    nat_rem(a2893, a2894, a2895)
                };
                nat_add(a2896, a2897)?
            });
            Ok(append_bytes(a2898, a2899))
        } else {
            let a2900 = v0;
            let a2901 = 65536u64;
            if nat_lt(a2900, a2901) {
                let a2922 = f508({
                    let a2905 = 224u64;
                    let a2906 = {
                        let a2902 = v0;
                        let a2903 = 4096u64;
                        let a2904 = 0u64;
                        nat_quot(a2902, a2903, a2904)
                    };
                    nat_add(a2905, a2906)?
                });
                let a2923 = {
                    let a2920 = f508({
                        let a2913 = 128u64;
                        let a2914 = {
                            let a2910 = {
                                let a2907 = v0;
                                let a2908 = 64u64;
                                let a2909 = 0u64;
                                nat_quot(a2907, a2908, a2909)
                            };
                            let a2911 = 64u64;
                            let a2912 = 0u64;
                            nat_rem(a2910, a2911, a2912)
                        };
                        nat_add(a2913, a2914)?
                    });
                    let a2921 = f508({
                        let a2918 = 128u64;
                        let a2919 = {
                            let a2915 = v0;
                            let a2916 = 64u64;
                            let a2917 = 0u64;
                            nat_rem(a2915, a2916, a2917)
                        };
                        nat_add(a2918, a2919)?
                    });
                    append_bytes(a2920, a2921)
                };
                Ok(append_bytes(a2922, a2923))
            } else {
                let a2954 = f508({
                    let a2927 = 240u64;
                    let a2928 = {
                        let a2924 = v0;
                        let a2925 = 262144u64;
                        let a2926 = 0u64;
                        nat_quot(a2924, a2925, a2926)
                    };
                    nat_add(a2927, a2928)?
                });
                let a2955 = {
                    let a2952 = f508({
                        let a2935 = 128u64;
                        let a2936 = {
                            let a2932 = {
                                let a2929 = v0;
                                let a2930 = 4096u64;
                                let a2931 = 0u64;
                                nat_quot(a2929, a2930, a2931)
                            };
                            let a2933 = 64u64;
                            let a2934 = 0u64;
                            nat_rem(a2932, a2933, a2934)
                        };
                        nat_add(a2935, a2936)?
                    });
                    let a2953 = {
                        let a2950 = f508({
                            let a2943 = 128u64;
                            let a2944 = {
                                let a2940 = {
                                    let a2937 = v0;
                                    let a2938 = 64u64;
                                    let a2939 = 0u64;
                                    nat_quot(a2937, a2938, a2939)
                                };
                                let a2941 = 64u64;
                                let a2942 = 0u64;
                                nat_rem(a2940, a2941, a2942)
                            };
                            nat_add(a2943, a2944)?
                        });
                        let a2951 = f508({
                            let a2948 = 128u64;
                            let a2949 = {
                                let a2945 = v0;
                                let a2946 = 64u64;
                                let a2947 = 0u64;
                                nat_rem(a2945, a2946, a2947)
                            };
                            nat_add(a2948, a2949)?
                        });
                        append_bytes(a2950, a2951)
                    };
                    append_bytes(a2952, a2953)
                };
                Ok(append_bytes(a2954, a2955))
            }
        }
    }
}

pub fn f465(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f509(v2, v0, v1, v3)
}

pub fn f466(p2959: Bytes, p2960: u64) -> Result<Bytes, Bytes> {
    let m2958 = (p2959, p2960);
    {
        let mut m2958 = m2958;
        loop {
            let step = {
                let (v0, v1) = m2958;
                let m2956 = v1;
                if m2956 == 0 {
                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines"))))
                } else {
                    let v2 = m2956 - 1;
                    if f405(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)"))))
                    } else {
                        if f37(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Nat"))))
                        } else {
                            if f37(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Int"))))
                            } else {
                                if f37(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f510()))
                                } else {
                                    if f37(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Bool"))))
                                    } else {
                                        if if f37(v0.clone(), f8(Str::lit("float"))) {
                                            true
                                        } else {
                                            f37(v0.clone(), f8(Str::lit("double")))
                                        } {
                                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil()))))))
                                        } else {
                                            if if f37(v0.clone(), f8(Str::lit("anySimpleType"))) {
                                                true
                                            } else {
                                                f37(v0.clone(), f8(Str::lit("anyAtomicType")))
                                            } {
                                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil()))))))
                                            } else {
                                                let m2957 = f511(v0.clone());
                                                match m2957 {
                                                    None => {
                                                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil()))))))
                                                    }
                                                    Some(v3) => {
                                                        if f37(v3.clone(), f8(Str::lit("anyAtomicType"))) {
                                                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Bytes"))))
                                                        } else {
                                                            Ok::<(Bytes, u64), Result<Bytes, Bytes>>((v3, v2))
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
            };
            match step {
                Ok(next) => { m2958 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Adt19) -> R<Bytes> {
    f263(v0)
}

pub fn f468(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m2961 = v1;
    match m2961 {
        Adt19::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(v7, v8) => {
            Ok(Some({
                let a2972 = if if v7 {
                    let a2970 = 0u64;
                    let a2971 = v8;
                    nat_lt(a2970, a2971)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2973 = f362(false, v8)?;
                append_bytes(a2972, a2973)
            }))
        }
        Adt19::C2(v3, v4, v5) => {
            let m2964 = f513({
                let a2962 = f8(Str::lit("n"));
                let a2963 = f363(false, v4, v5)?;
                append_bytes(a2962, a2963)
            })?;
            match m2964 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2968 = f8(Str::lit("nMinus"));
                        let a2969 = f32(v6.clone(), 1u64, {
                            let a2966 = {
                                let a2965 = v6;
                                length_bytes(a2965)
                            };
                            let a2967 = 1u64;
                            nat_sub(a2966, a2967)
                        });
                        append_bytes(a2968, a2969)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C3(v2) => {
            f512(v0, v2)
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

pub fn f469(v0: Bytes) -> R<Option<Bytes>> {
    if f514({
        let a2974 = v0.clone();
        length_bytes(a2974)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f515(v0)?;
        if f516(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2977 = {
                let a2975 = v1.clone();
                let a2976 = 0u64;
                index_bytes(a2975, a2976)
            };
            match m2977 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f427({
                        let v3: u8 = v2;
                        let a3022 = {
                            let a2998 = {
                                let a2986 = {
                                    let a2980 = {
                                        let a2978 = v3;
                                        let a2979 = 128u8;
                                        fixed_u8::bit_and(a2978, a2979)
                                    };
                                    let a2981 = 128u8;
                                    if equal(a2980, a2981) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2987 = {
                                    let a2984 = {
                                        let a2982 = v3;
                                        let a2983 = 64u8;
                                        fixed_u8::bit_and(a2982, a2983)
                                    };
                                    let a2985 = 64u8;
                                    if equal(a2984, a2985) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2986, a2987)?
                            };
                            let a2999 = {
                                let a2996 = {
                                    let a2990 = {
                                        let a2988 = v3;
                                        let a2989 = 32u8;
                                        fixed_u8::bit_and(a2988, a2989)
                                    };
                                    let a2991 = 32u8;
                                    if equal(a2990, a2991) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2997 = {
                                    let a2994 = {
                                        let a2992 = v3;
                                        let a2993 = 16u8;
                                        fixed_u8::bit_and(a2992, a2993)
                                    };
                                    let a2995 = 16u8;
                                    if equal(a2994, a2995) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2996, a2997)?
                            };
                            nat_add(a2998, a2999)?
                        };
                        let a3023 = {
                            let a3020 = {
                                let a3008 = {
                                    let a3002 = {
                                        let a3000 = v3;
                                        let a3001 = 8u8;
                                        fixed_u8::bit_and(a3000, a3001)
                                    };
                                    let a3003 = 8u8;
                                    if equal(a3002, a3003) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3009 = {
                                    let a3006 = {
                                        let a3004 = v3;
                                        let a3005 = 4u8;
                                        fixed_u8::bit_and(a3004, a3005)
                                    };
                                    let a3007 = 4u8;
                                    if equal(a3006, a3007) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3008, a3009)?
                            };
                            let a3021 = {
                                let a3018 = {
                                    let a3012 = {
                                        let a3010 = v3;
                                        let a3011 = 2u8;
                                        fixed_u8::bit_and(a3010, a3011)
                                    };
                                    let a3013 = 2u8;
                                    if equal(a3012, a3013) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3019 = {
                                    let a3016 = {
                                        let a3014 = v3;
                                        let a3015 = 1u8;
                                        fixed_u8::bit_and(a3014, a3015)
                                    };
                                    let a3017 = 1u8;
                                    if equal(a3016, a3017) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3018, a3019)?
                            };
                            nat_add(a3020, a3021)?
                        };
                        nat_add(a3022, a3023)?
                    }) {
                        Ok(Some({
                            let a3024 = f8(Str::lit("n"));
                            let a3025 = v1;
                            append_bytes(a3024, a3025)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f517({
            let a3026 = v0.clone();
            length_bytes(a3026)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f470(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3027 = v1;
    match m3027.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3028 = f512(v0, v2)?;
            match m3028 {
                None => {
                    f470(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f470(v0, v3)?))
                }
            }
        }
    }
}

pub fn f471(v0: Fn15, p3033: u64, p3034: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m3032 = (p3033, p3034);
    {
        let mut m3032 = m3032;
        loop {
            let step = {
                let (v1, v2) = m3032;
                let m3029 = v1;
                if m3029 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3029 - 1;
                    let m3031 = {
                        let c3030: Fn15 = v0.clone();
                        c3030.apply(v2.clone())?
                    };
                    match m3031 {
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
                Ok(next) => { m3032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f518(v0, v1)
}

pub fn f473(v0: Bytes, v1: u64) -> R<u64> {
    let (h3035, _) = f274(Fn3::F519(v0.clone()), {
        let a3037 = {
            let a3036 = v0;
            length_bytes(a3036)
        };
        let a3038 = 1u64;
        nat_add(a3037, a3038)?
    }, v1)?;
    Ok(h3035)
}

pub fn f474(v0: Bytes) -> R<u64> {
    let (h3039, _) = f274(Fn3::F520(v0.clone()), {
        let a3041 = {
            let a3040 = v0;
            length_bytes(a3040)
        };
        let a3042 = 1u64;
        nat_add(a3041, a3042)?
    }, 0u64)?;
    Ok(h3039)
}

pub fn f475(v0: Bytes) -> R<Option<u64>> {
    if f521(v0.clone())? {
        Ok(Some({
            let (_, h3043) = {
                let (h3044, _) = f506(Fn16::F522(v0.clone()), {
                    let a3045 = v0;
                    length_bytes(a3045)
                }, (0u64, 0u64))?;
                h3044
            };
            h3043
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f476(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3047 = {
            let a3046 = v1.clone();
            length_bytes(a3046)
        };
        let a3048 = f523(v1)?;
        nat_sub(a3047, a3048)
    });
    let v5: Bytes = {
        let a3049 = v0;
        let a3050 = v4.clone();
        append_bytes(a3049, a3050)
    };
    let v6: u64 = f474(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3052 = {
            let a3051 = v5;
            length_bytes(a3051)
        };
        let a3053 = v6;
        nat_sub(a3052, a3053)
    });
    let v8: u64 = {
        let a3054 = v4;
        length_bytes(a3054)
    };
    let v9: u64 = {
        let a3055 = v7.clone();
        length_bytes(a3055)
    };
    let a3056 = v9;
    let a3057 = 0u64;
    if nat_eq(a3056, a3057) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a3058 = v3;
            let a3059 = v8;
            nat_lt(a3058, a3059)
        } {
            let v10: u64 = if v2 {
                let a3060 = v3;
                let a3061 = v8;
                nat_add(a3060, a3061)?
            } else {
                let a3062 = v8;
                let a3063 = v3;
                nat_sub(a3062, a3063)
            };
            let a3064 = v10;
            let a3065 = f523(v7.clone())?;
            if nat_le(a3064, a3065) {
                Ok(Some(f32(v7, 0u64, {
                    let a3066 = v9;
                    let a3067 = v10;
                    nat_sub(a3066, a3067)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a3068 = v3;
                let a3069 = v8;
                nat_sub(a3068, a3069)
            };
            let a3072 = {
                let a3070 = v9;
                let a3071 = v11;
                nat_add(a3070, a3071)?
            };
            let a3073 = 19u64;
            if nat_le(a3072, a3073) {
                Ok(Some({
                    let a3074 = v7;
                    let a3075 = f437(v11);
                    append_bytes(a3074, a3075)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f477(v0: bool, v1: u64) -> bool {
    if v0 {
        let a3076 = 0u64;
        let a3077 = v1;
        if nat_lt(a3076, a3077) {
            let a3078 = v1;
            let a3079 = 9223372036854775808u64;
            nat_le(a3078, a3079)
        } else {
            false
        }
    } else {
        let a3080 = v1;
        let a3081 = 9223372036854775807u64;
        nat_le(a3080, a3081)
    }
}

pub fn f478(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3083 = {
            let a3082 = v1.clone();
            length_bytes(a3082)
        };
        let a3084 = f523(v1)?;
        nat_sub(a3083, a3084)
    });
    let v5: Bytes = {
        let a3085 = v0;
        let a3086 = v4.clone();
        append_bytes(a3085, a3086)
    };
    let v6: u64 = f474(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3088 = {
            let a3087 = v5;
            length_bytes(a3087)
        };
        let a3089 = v6;
        nat_sub(a3088, a3089)
    });
    let v8: u64 = {
        let a3090 = v4;
        length_bytes(a3090)
    };
    let v9: u64 = {
        let a3091 = v7.clone();
        length_bytes(a3091)
    };
    let a3092 = v9;
    let a3093 = 0u64;
    if nat_eq(a3092, a3093) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if if v2 {
            true
        } else {
            let a3094 = v3;
            let a3095 = v8;
            nat_lt(a3094, a3095)
        } {
            let v10: u64 = if v2 {
                let a3096 = v3;
                let a3097 = v8;
                nat_add(a3096, a3097)?
            } else {
                let a3098 = v8;
                let a3099 = v3;
                nat_sub(a3098, a3099)
            };
            let v11: u64 = f523(v7.clone())?;
            let a3100 = v10;
            let a3101 = v11;
            if nat_le(a3100, a3101) {
                Ok(Some((f32(v7, 0u64, {
                    let a3102 = v9;
                    let a3103 = v10;
                    nat_sub(a3102, a3103)
                }), 0u64)))
            } else {
                let a3106 = {
                    let a3104 = v10;
                    let a3105 = v11;
                    nat_sub(a3104, a3105)
                };
                let a3107 = 19u64;
                if nat_le(a3106, a3107) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3108 = v9;
                        let a3109 = v11;
                        nat_sub(a3108, a3109)
                    }), {
                        let a3110 = v10;
                        let a3111 = v11;
                        nat_sub(a3110, a3111)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3112 = v3;
                let a3113 = v8;
                nat_sub(a3112, a3113)
            };
            let a3116 = {
                let a3114 = v9;
                let a3115 = v12;
                nat_add(a3114, a3115)?
            };
            let a3117 = 19u64;
            if nat_le(a3116, a3117) {
                Ok(Some(({
                    let a3118 = v7;
                    let a3119 = f437(v12);
                    append_bytes(a3118, a3119)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f479(v0: Adt19) -> List<(Bytes, Adt19)> {
    let m3120 = v0;
    match m3120 {
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
        Adt19::C5(r3121) => {
            let v4 = (*r3121).clone();
            f479(v4)
        }
        Adt19::C6 => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C7(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C8(v1, r3122, r3123) => {
            let v2 = (*r3122).clone();
            let v3 = (*r3123).clone();
            List::cons((v1, v2), f479(v3))
        }
    }
}

pub fn f480(p3128: List<(Bytes, Adt19)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt19)>> {
    let m3127 = p3128;
    {
        let mut m3127 = m3127;
        loop {
            let step = {
                let v0 = m3127;
                let m3124 = v0;
                match m3124.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(None::<(Bytes, Adt19)>))
                    }
                    Some((v3, v4)) => {
                        if f524(f525({
                            let (_, h3125) = v3.clone();
                            h3125
                        }, List::<Bytes>::nil())?, f260({
                            let (h3126, _) = v3.clone();
                            h3126
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: List<(Bytes, Adt19)>, v1: Bytes) -> List<(Bytes, Adt19)> {
    let m3129 = v0;
    match m3129.uncons() {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some((v2, v3)) => {
            if f37({
                let (h3130, _) = v2.clone();
                h3130
            }, v1.clone()) {
                f481(v3, v1)
            } else {
                List::cons(v2, f481(v3, v1))
            }
        }
    }
}

pub fn f482() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f483() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f484(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f526(v0, f527(v1))
}

pub fn f485(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m3131 = v0;
    match m3131.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f485(v3, false))
            } else {
                f485(v3, true)
            }
        }
    }
}

pub fn f486(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3135 = {
        let a3133 = v0.clone();
        let a3134 = {
            let (h3132, _) = v1.clone();
            h3132
        };
        index_bytes(a3133, a3134)
    };
    match m3135 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3137 = {
                    let (h3136, _) = v1.clone();
                    h3136
                };
                let a3138 = 1u64;
                nat_add(a3137, a3138)?
            }, {
                let a3141 = {
                    let (_, h3139) = v1;
                    h3139
                };
                let a3142 = f528(v0, {
                    let (h3140, _) = v1;
                    h3140
                }, v2)?;
                append_bytes(a3141, a3142)
            })))
        }
    }
}

pub fn f487(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3143 = v0.clone();
    match m3143.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3146 = {
                let (h3144, _) = v2;
                h3144
            };
            let a3147 = {
                let (h3145, _) = v1.clone();
                h3145
            };
            if equal(a3146, a3147) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f488(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f529(f530(Fn17::F531, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3152 = {
        let (h3148, _) = f532(Fn18::F533, {
            let a3150 = {
                let a3149 = v0;
                length_list(a3149)
            };
            let a3151 = 1u64;
            nat_add(a3150, a3151)?
        }, v1)?;
        h3148
    };
    match m3152.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f489(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3157 = {
        let a3154 = f364({
            let (h3153, _) = v0;
            h3153
        })?;
        let a3155 = Bytes::lit(&[0x3a]);
        append_bytes(a3154, a3155)
    };
    let a3158 = {
        let (_, h3156) = v0;
        h3156
    };
    Ok(append_bytes(a3157, a3158))
}

pub fn f490(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f378({
        let (_, h3159) = {
            let (h3160, _) = f534(Fn19::F535, {
                let a3162 = {
                    let a3161 = v0.clone();
                    length_list(a3161)
                };
                let a3163 = 1u64;
                nat_add(a3162, a3163)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h3160
        };
        h3159
    }))
}

pub fn f491(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3164 = v0;
    match m3164.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f88(v2.clone(), v1.clone()) {
                f491(v3, v1)
            } else {
                List::cons(v2, f491(v3, v1))
            }
        }
    }
}

pub fn f492(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f493(v0: List<Adt14>) -> List<Bytes> {
    let m3165 = v0;
    match m3165.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3166 = f165(v1, f8(Str::lit("std")));
            match m3166 {
                None => {
                    f493(v2)
                }
                Some(v3) => {
                    List::cons(f163(v3, f8(Str::lit("authority"))), f493(v2))
                }
            }
        }
    }
}

pub fn f494(v0: List<Adt14>) -> List<Bytes> {
    let m3167 = v0;
    match m3167.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3168 = f166(f165(v1, f8(Str::lit("req"))));
            match m3168 {
                None => {
                    f494(v2)
                }
                Some(v3) => {
                    List::cons(v3, f494(v2))
                }
            }
        }
    }
}

pub fn f495(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f536(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h3169) => {
            h3169
        }
    });
    let v2: u32 = f455(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h3170, _, _) => {
            h3170
        }
    }, f455(f537(match v0.clone() {
        Adt27::C0(_, _, _, _, h3171, _, _, _, _, _) => {
            h3171
        }
    }), f455(f538(match v0.clone() {
        Adt27::C0(_, _, _, _, h3172, _, _, _, _, _) => {
            h3172
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h3173, _, _, _, _) => {
            h3173
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h3174, _, _, _) => {
            h3174
        }
    }), f455(v1, {
        let m3177 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h3176, _) => {
                h3176
            }
        };
        match m3177 {
            Adt26::C0(h3175, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3175
            }
        }
    }))));
    let v3: u32 = f455(f539(match v0.clone() {
        Adt27::C0(h3178, _, _, _, _, _, _, _, _, _) => {
            h3178
        }
    }), f540(match v0.clone() {
        Adt27::C0(h3179, _, _, _, _, _, _, _, _, _) => {
            h3179
        }
    }, match v0.clone() {
        Adt27::C0(_, h3180, _, _, _, _, _, _, _, _) => {
            h3180
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h3181, _, _, _, _, _, _, _) => {
            h3181
        }
    }));
    Ok(Adt27::C0(f455(v2, v3), match v0.clone() {
        Adt27::C0(h3182, _, _, _, _, _, _, _, _, _) => {
            h3182
        }
    }, match v0.clone() {
        Adt27::C0(_, h3183, _, _, _, _, _, _, _, _) => {
            h3183
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h3184, _, _, _, _, _, _, _) => {
            h3184
        }
    }, f455(match v0.clone() {
        Adt27::C0(_, _, _, h3185, _, _, _, _, _, _) => {
            h3185
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h3186, _, _, _, _, _) => {
            h3186
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h3187, _, _, _, _) => {
            h3187
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h3188, _, _, _) => {
            h3188
        }
    }, f541(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h3189, _) => {
            h3189
        }
    }), {
        let a3191 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h3190) => {
                h3190
            }
        };
        let a3192 = 1u64;
        nat_add(a3191, a3192)?
    }))
}

pub fn f496(v0: u32) -> u64 {
    let m3194 = {
        let a3193 = v0;
        fixed_u64::convert(i128::from(a3193))
    };
    match m3194 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f497(v0: u64) -> u32 {
    let m3198 = {
        let a3197 = {
            let a3195 = v0;
            let a3196 = 4294967295u64;
            fixed_u64::bit_and(a3195, a3196)
        };
        fixed_u32::convert(i128::from(a3197))
    };
    match m3198 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f498(v0: u64) -> u32 {
    let (_, h3199) = {
        let (_, h3200) = f542(Fn20::F543, 32u64, (v0, (1u32, 0u32)));
        h3200
    };
    h3199
}

pub fn f499(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3205 = f544(v0, 24u32);
    let a3206 = {
        let a3203 = f544(v1, 16u32);
        let a3204 = {
            let a3201 = f544(v2, 8u32);
            let a3202 = v3;
            fixed_u32::bit_or(a3201, a3202)
        };
        fixed_u32::bit_or(a3203, a3204)
    };
    fixed_u32::bit_or(a3205, a3206)
}

pub fn f500(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3207 = v2;
    let a3208 = v1;
    if nat_lt(a3207, a3208) {
        let m3211 = {
            let a3209 = v0;
            let a3210 = v2;
            index_bytes(a3209, a3210)
        };
        match m3211 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3213 = {
                    let a3212 = v3;
                    fixed_u32::convert(i128::from(a3212))
                };
                match m3213 {
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
        let a3214 = v2;
        let a3215 = v1;
        if nat_eq(a3214, a3215) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f501(v0: u8) -> bool {
    let a3216 = v0;
    let a3217 = 32u8;
    if if if equal(a3216, a3217) {
        true
    } else {
        let a3218 = v0;
        let a3219 = 9u8;
        equal(a3218, a3219)
    } {
        true
    } else {
        let a3220 = v0;
        let a3221 = 10u8;
        equal(a3220, a3221)
    } {
        true
    } else {
        let a3222 = v0;
        let a3223 = 12u8;
        if equal(a3222, a3223) {
            true
        } else {
            let a3224 = v0;
            let a3225 = 13u8;
            equal(a3224, a3225)
        }
    }
}

pub fn f502(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3228 = {
        let a3226 = v0;
        let a3227 = v1;
        index_bytes(a3226, a3227)
    };
    match m3228 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3273 = {
                    let a3249 = {
                        let a3237 = {
                            let a3231 = {
                                let a3229 = v3;
                                let a3230 = 128u8;
                                fixed_u8::bit_and(a3229, a3230)
                            };
                            let a3232 = 128u8;
                            if equal(a3231, a3232) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3238 = {
                            let a3235 = {
                                let a3233 = v3;
                                let a3234 = 64u8;
                                fixed_u8::bit_and(a3233, a3234)
                            };
                            let a3236 = 64u8;
                            if equal(a3235, a3236) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3237, a3238)?
                    };
                    let a3250 = {
                        let a3247 = {
                            let a3241 = {
                                let a3239 = v3;
                                let a3240 = 32u8;
                                fixed_u8::bit_and(a3239, a3240)
                            };
                            let a3242 = 32u8;
                            if equal(a3241, a3242) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3248 = {
                            let a3245 = {
                                let a3243 = v3;
                                let a3244 = 16u8;
                                fixed_u8::bit_and(a3243, a3244)
                            };
                            let a3246 = 16u8;
                            if equal(a3245, a3246) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3247, a3248)?
                    };
                    nat_add(a3249, a3250)?
                };
                let a3274 = {
                    let a3271 = {
                        let a3259 = {
                            let a3253 = {
                                let a3251 = v3;
                                let a3252 = 8u8;
                                fixed_u8::bit_and(a3251, a3252)
                            };
                            let a3254 = 8u8;
                            if equal(a3253, a3254) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3260 = {
                            let a3257 = {
                                let a3255 = v3;
                                let a3256 = 4u8;
                                fixed_u8::bit_and(a3255, a3256)
                            };
                            let a3258 = 4u8;
                            if equal(a3257, a3258) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3259, a3260)?
                    };
                    let a3272 = {
                        let a3269 = {
                            let a3263 = {
                                let a3261 = v3;
                                let a3262 = 2u8;
                                fixed_u8::bit_and(a3261, a3262)
                            };
                            let a3264 = 2u8;
                            if equal(a3263, a3264) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3270 = {
                            let a3267 = {
                                let a3265 = v3;
                                let a3266 = 1u8;
                                fixed_u8::bit_and(a3265, a3266)
                            };
                            let a3268 = 1u8;
                            if equal(a3267, a3268) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3269, a3270)?
                    };
                    nat_add(a3271, a3272)?
                };
                nat_add(a3273, a3274)?
            };
            let a3275 = 128u64;
            let a3276 = v4;
            if if nat_le(a3275, a3276) {
                let a3277 = v4;
                let a3278 = 191u64;
                nat_le(a3277, a3278)
            } else {
                false
            } {
                Ok(Some({
                    let a3279 = v4;
                    let a3280 = 128u64;
                    nat_sub(a3279, a3280)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f503(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3281 = v1;
    let a3282 = v0;
    if if nat_le(a3281, a3282) {
        let a3283 = v0;
        let a3284 = v2;
        if nat_le(a3283, a3284) {
            let a3285 = v0;
            let a3286 = 55296u64;
            if nat_lt(a3285, a3286) {
                true
            } else {
                let a3287 = 57343u64;
                let a3288 = v0;
                nat_lt(a3287, a3288)
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

pub fn f504(v0: u64) -> bool {
    let a3289 = v0;
    let a3290 = 58u64;
    if if if nat_eq(a3289, a3290) {
        true
    } else {
        let a3291 = v0;
        let a3292 = 95u64;
        nat_eq(a3291, a3292)
    } {
        true
    } else {
        let a3293 = 65u64;
        let a3294 = v0;
        if if nat_le(a3293, a3294) {
            let a3295 = v0;
            let a3296 = 90u64;
            nat_le(a3295, a3296)
        } else {
            false
        } {
            true
        } else {
            let a3297 = 97u64;
            let a3298 = v0;
            if nat_le(a3297, a3298) {
                let a3299 = v0;
                let a3300 = 122u64;
                nat_le(a3299, a3300)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a3301 = 192u64;
        let a3302 = v0;
        if if if if nat_le(a3301, a3302) {
            let a3303 = v0;
            let a3304 = 214u64;
            nat_le(a3303, a3304)
        } else {
            false
        } {
            true
        } else {
            let a3305 = 216u64;
            let a3306 = v0;
            if nat_le(a3305, a3306) {
                let a3307 = v0;
                let a3308 = 246u64;
                nat_le(a3307, a3308)
            } else {
                false
            }
        } {
            true
        } else {
            let a3309 = 248u64;
            let a3310 = v0;
            if if nat_le(a3309, a3310) {
                let a3311 = v0;
                let a3312 = 767u64;
                nat_le(a3311, a3312)
            } else {
                false
            } {
                true
            } else {
                let a3313 = 880u64;
                let a3314 = v0;
                if nat_le(a3313, a3314) {
                    let a3315 = v0;
                    let a3316 = 893u64;
                    nat_le(a3315, a3316)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a3317 = 895u64;
            let a3318 = v0;
            if if if if nat_le(a3317, a3318) {
                let a3319 = v0;
                let a3320 = 8191u64;
                nat_le(a3319, a3320)
            } else {
                false
            } {
                true
            } else {
                let a3321 = 8204u64;
                let a3322 = v0;
                if nat_le(a3321, a3322) {
                    let a3323 = v0;
                    let a3324 = 8205u64;
                    nat_le(a3323, a3324)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3325 = 8304u64;
                let a3326 = v0;
                if if nat_le(a3325, a3326) {
                    let a3327 = v0;
                    let a3328 = 8591u64;
                    nat_le(a3327, a3328)
                } else {
                    false
                } {
                    true
                } else {
                    let a3329 = 11264u64;
                    let a3330 = v0;
                    if nat_le(a3329, a3330) {
                        let a3331 = v0;
                        let a3332 = 12271u64;
                        nat_le(a3331, a3332)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a3333 = 12289u64;
                let a3334 = v0;
                if if if nat_le(a3333, a3334) {
                    let a3335 = v0;
                    let a3336 = 55295u64;
                    nat_le(a3335, a3336)
                } else {
                    false
                } {
                    true
                } else {
                    let a3337 = 63744u64;
                    let a3338 = v0;
                    if nat_le(a3337, a3338) {
                        let a3339 = v0;
                        let a3340 = 64975u64;
                        nat_le(a3339, a3340)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a3341 = 65008u64;
                    let a3342 = v0;
                    if if nat_le(a3341, a3342) {
                        let a3343 = v0;
                        let a3344 = 65533u64;
                        nat_le(a3343, a3344)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a3345 = 65536u64;
                        let a3346 = v0;
                        if nat_le(a3345, a3346) {
                            let a3347 = v0;
                            let a3348 = 983039u64;
                            nat_le(a3347, a3348)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f505(v0: u64) -> bool {
    if f504(v0) {
        true
    } else {
        let a3349 = v0;
        let a3350 = 45u64;
        if if if nat_eq(a3349, a3350) {
            true
        } else {
            let a3351 = v0;
            let a3352 = 46u64;
            nat_eq(a3351, a3352)
        } {
            true
        } else {
            let a3353 = 48u64;
            let a3354 = v0;
            if nat_le(a3353, a3354) {
                let a3355 = v0;
                let a3356 = 57u64;
                nat_le(a3355, a3356)
            } else {
                false
            }
        } {
            true
        } else {
            let a3357 = v0;
            let a3358 = 183u64;
            if if nat_eq(a3357, a3358) {
                true
            } else {
                let a3359 = 768u64;
                let a3360 = v0;
                if nat_le(a3359, a3360) {
                    let a3361 = v0;
                    let a3362 = 879u64;
                    nat_le(a3361, a3362)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3363 = 8255u64;
                let a3364 = v0;
                if nat_le(a3363, a3364) {
                    let a3365 = v0;
                    let a3366 = 8256u64;
                    nat_le(a3365, a3366)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f506(v0: Fn16, p3371: u64, p3372: (u64, u64)) -> R<((u64, u64), bool)> {
    let m3370 = (p3371, p3372);
    {
        let mut m3370 = m3370;
        loop {
            let step = {
                let (v1, v2) = m3370;
                let m3367 = v1;
                if m3367 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3367 - 1;
                    let m3369 = {
                        let c3368: Fn16 = v0.clone();
                        c3368.apply(v2)?
                    };
                    match m3369 {
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
                Ok(next) => { m3370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f545(v1, v0, v2)
}

pub fn f508(v0: u64) -> Bytes {
    f546(f547({
        let a3373 = v0;
        let a3374 = 16u64;
        let a3375 = 0u64;
        nat_quot(a3373, a3374, a3375)
    }), {
        let a3376 = v0;
        let a3377 = 16u64;
        let a3378 = 0u64;
        nat_rem(a3376, a3377, a3378)
    })
}

pub fn f509(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f548(f233(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3379 = v3;
            let a3380 = 1u64;
            nat_add(a3379, a3380)?
        }))
    }
}

pub fn f510() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f511(v0: Bytes) -> Option<Bytes> {
    if f405(v0.clone(), List::cons(f8(Str::lit("string")), List::cons(f8(Str::lit("boolean")), List::cons(f8(Str::lit("decimal")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::cons(f8(Str::lit("duration")), List::cons(f8(Str::lit("dateTime")), List::cons(f8(Str::lit("time")), List::cons(f8(Str::lit("date")), List::cons(f8(Str::lit("gYearMonth")), List::cons(f8(Str::lit("gYear")), List::cons(f8(Str::lit("gMonthDay")), List::cons(f8(Str::lit("gDay")), List::cons(f8(Str::lit("gMonth")), List::cons(f8(Str::lit("hexBinary")), List::cons(f8(Str::lit("base64Binary")), List::cons(f8(Str::lit("anyURI")), List::cons(f8(Str::lit("QName")), List::cons(f8(Str::lit("NOTATION")), List::<Bytes>::nil())))))))))))))))))))) {
        Some(f8(Str::lit("anyAtomicType")))
    } else {
        if f37(v0.clone(), f8(Str::lit("normalizedString"))) {
            Some(f8(Str::lit("string")))
        } else {
            if f37(v0.clone(), f8(Str::lit("token"))) {
                Some(f8(Str::lit("normalizedString")))
            } else {
                if f405(v0.clone(), List::cons(f8(Str::lit("language")), List::cons(f8(Str::lit("NMTOKEN")), List::cons(f8(Str::lit("Name")), List::<Bytes>::nil())))) {
                    Some(f8(Str::lit("token")))
                } else {
                    if f37(v0.clone(), f8(Str::lit("NCName"))) {
                        Some(f8(Str::lit("Name")))
                    } else {
                        if f405(v0.clone(), List::cons(f8(Str::lit("ID")), List::cons(f8(Str::lit("IDREF")), List::cons(f8(Str::lit("ENTITY")), List::<Bytes>::nil())))) {
                            Some(f8(Str::lit("NCName")))
                        } else {
                            if f37(v0.clone(), f8(Str::lit("integer"))) {
                                Some(f8(Str::lit("decimal")))
                            } else {
                                if f405(v0.clone(), List::cons(f8(Str::lit("nonPositiveInteger")), List::cons(f8(Str::lit("long")), List::cons(f8(Str::lit("nonNegativeInteger")), List::<Bytes>::nil())))) {
                                    Some(f8(Str::lit("integer")))
                                } else {
                                    if f37(v0.clone(), f8(Str::lit("negativeInteger"))) {
                                        Some(f8(Str::lit("nonPositiveInteger")))
                                    } else {
                                        if f37(v0.clone(), f8(Str::lit("int"))) {
                                            Some(f8(Str::lit("long")))
                                        } else {
                                            if f37(v0.clone(), f8(Str::lit("short"))) {
                                                Some(f8(Str::lit("int")))
                                            } else {
                                                if f37(v0.clone(), f8(Str::lit("byte"))) {
                                                    Some(f8(Str::lit("short")))
                                                } else {
                                                    if f405(v0.clone(), List::cons(f8(Str::lit("unsignedLong")), List::cons(f8(Str::lit("positiveInteger")), List::<Bytes>::nil()))) {
                                                        Some(f8(Str::lit("nonNegativeInteger")))
                                                    } else {
                                                        if f37(v0.clone(), f8(Str::lit("unsignedInt"))) {
                                                            Some(f8(Str::lit("unsignedLong")))
                                                        } else {
                                                            if f37(v0.clone(), f8(Str::lit("unsignedShort"))) {
                                                                Some(f8(Str::lit("unsignedInt")))
                                                            } else {
                                                                if f37(v0.clone(), f8(Str::lit("unsignedByte"))) {
                                                                    Some(f8(Str::lit("unsignedShort")))
                                                                } else {
                                                                    if f405(v0.clone(), List::cons(f8(Str::lit("yearMonthDuration")), List::cons(f8(Str::lit("dayTimeDuration")), List::<Bytes>::nil()))) {
                                                                        Some(f8(Str::lit("duration")))
                                                                    } else {
                                                                        if f37(v0.clone(), f8(Str::lit("dateTimeStamp"))) {
                                                                            Some(f8(Str::lit("dateTime")))
                                                                        } else {
                                                                            if f405(v0, List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))))) {
                                                                                Some(f8(Str::lit("anySimpleType")))
                                                                            } else {
                                                                                None::<Bytes>
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

pub fn f512(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f513(v1)
    } else {
        f469(v1)
    }
}

pub fn f513(v0: Bytes) -> R<Option<Bytes>> {
    let m3382 = f549({
        let a3381 = v0.clone();
        length_bytes(a3381)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3382 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3384 = {
                let a3383 = v1.clone();
                length_bytes(a3383)
            };
            let a3385 = 0u64;
            if nat_eq(a3384, a3385) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f514(p3441: u64, v1: Bytes, p3442: u64) -> R<bool> {
    let m3440 = (p3441, p3442);
    {
        let mut m3440 = m3440;
        loop {
            let step = {
                let (v0, v2) = m3440;
                let m3386 = v0;
                if m3386 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3386 - 1;
                    let m3389 = {
                        let a3387 = v1.clone();
                        let a3388 = v2;
                        index_bytes(a3387, a3388)
                    };
                    match m3389 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3436 = {
                                let v5: u8 = v4;
                                let a3434 = {
                                    let a3410 = {
                                        let a3398 = {
                                            let a3392 = {
                                                let a3390 = v5;
                                                let a3391 = 128u8;
                                                fixed_u8::bit_and(a3390, a3391)
                                            };
                                            let a3393 = 128u8;
                                            if equal(a3392, a3393) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3399 = {
                                            let a3396 = {
                                                let a3394 = v5;
                                                let a3395 = 64u8;
                                                fixed_u8::bit_and(a3394, a3395)
                                            };
                                            let a3397 = 64u8;
                                            if equal(a3396, a3397) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3398, a3399)?
                                    };
                                    let a3411 = {
                                        let a3408 = {
                                            let a3402 = {
                                                let a3400 = v5;
                                                let a3401 = 32u8;
                                                fixed_u8::bit_and(a3400, a3401)
                                            };
                                            let a3403 = 32u8;
                                            if equal(a3402, a3403) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3409 = {
                                            let a3406 = {
                                                let a3404 = v5;
                                                let a3405 = 16u8;
                                                fixed_u8::bit_and(a3404, a3405)
                                            };
                                            let a3407 = 16u8;
                                            if equal(a3406, a3407) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3408, a3409)?
                                    };
                                    nat_add(a3410, a3411)?
                                };
                                let a3435 = {
                                    let a3432 = {
                                        let a3420 = {
                                            let a3414 = {
                                                let a3412 = v5;
                                                let a3413 = 8u8;
                                                fixed_u8::bit_and(a3412, a3413)
                                            };
                                            let a3415 = 8u8;
                                            if equal(a3414, a3415) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3421 = {
                                            let a3418 = {
                                                let a3416 = v5;
                                                let a3417 = 4u8;
                                                fixed_u8::bit_and(a3416, a3417)
                                            };
                                            let a3419 = 4u8;
                                            if equal(a3418, a3419) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3420, a3421)?
                                    };
                                    let a3433 = {
                                        let a3430 = {
                                            let a3424 = {
                                                let a3422 = v5;
                                                let a3423 = 2u8;
                                                fixed_u8::bit_and(a3422, a3423)
                                            };
                                            let a3425 = 2u8;
                                            if equal(a3424, a3425) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3431 = {
                                            let a3428 = {
                                                let a3426 = v5;
                                                let a3427 = 1u8;
                                                fixed_u8::bit_and(a3426, a3427)
                                            };
                                            let a3429 = 1u8;
                                            if equal(a3428, a3429) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3430, a3431)?
                                    };
                                    nat_add(a3432, a3433)?
                                };
                                nat_add(a3434, a3435)?
                            };
                            let a3437 = 128u64;
                            if nat_lt(a3436, a3437) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3438 = v2;
                                    let a3439 = 1u64;
                                    nat_add(a3438, a3439)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f358(v0.clone(), 0u64, {
        let a3443 = v0;
        length_bytes(a3443)
    }, false, Bytes::lit(&[]))?;
    let m3446 = {
        let a3444 = v1.clone();
        let a3445 = 0u64;
        index_bytes(a3444, a3445)
    };
    match m3446 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3491 = {
                    let a3467 = {
                        let a3455 = {
                            let a3449 = {
                                let a3447 = v3;
                                let a3448 = 128u8;
                                fixed_u8::bit_and(a3447, a3448)
                            };
                            let a3450 = 128u8;
                            if equal(a3449, a3450) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3456 = {
                            let a3453 = {
                                let a3451 = v3;
                                let a3452 = 64u8;
                                fixed_u8::bit_and(a3451, a3452)
                            };
                            let a3454 = 64u8;
                            if equal(a3453, a3454) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3455, a3456)?
                    };
                    let a3468 = {
                        let a3465 = {
                            let a3459 = {
                                let a3457 = v3;
                                let a3458 = 32u8;
                                fixed_u8::bit_and(a3457, a3458)
                            };
                            let a3460 = 32u8;
                            if equal(a3459, a3460) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3466 = {
                            let a3463 = {
                                let a3461 = v3;
                                let a3462 = 16u8;
                                fixed_u8::bit_and(a3461, a3462)
                            };
                            let a3464 = 16u8;
                            if equal(a3463, a3464) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3465, a3466)?
                    };
                    nat_add(a3467, a3468)?
                };
                let a3492 = {
                    let a3489 = {
                        let a3477 = {
                            let a3471 = {
                                let a3469 = v3;
                                let a3470 = 8u8;
                                fixed_u8::bit_and(a3469, a3470)
                            };
                            let a3472 = 8u8;
                            if equal(a3471, a3472) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3478 = {
                            let a3475 = {
                                let a3473 = v3;
                                let a3474 = 4u8;
                                fixed_u8::bit_and(a3473, a3474)
                            };
                            let a3476 = 4u8;
                            if equal(a3475, a3476) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3477, a3478)?
                    };
                    let a3490 = {
                        let a3487 = {
                            let a3481 = {
                                let a3479 = v3;
                                let a3480 = 2u8;
                                fixed_u8::bit_and(a3479, a3480)
                            };
                            let a3482 = 2u8;
                            if equal(a3481, a3482) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3488 = {
                            let a3485 = {
                                let a3483 = v3;
                                let a3484 = 1u8;
                                fixed_u8::bit_and(a3483, a3484)
                            };
                            let a3486 = 1u8;
                            if equal(a3485, a3486) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3487, a3488)?
                    };
                    nat_add(a3489, a3490)?
                };
                nat_add(a3491, a3492)?
            };
            if f426(v4) {
                let a3499 = f32({
                    let a3493 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3493)
                }, {
                    let a3494 = v4;
                    let a3495 = 65u64;
                    nat_sub(a3494, a3495)
                }, 1u64);
                let a3500 = f32(v1.clone(), 1u64, {
                    let a3497 = {
                        let a3496 = v1;
                        length_bytes(a3496)
                    };
                    let a3498 = 1u64;
                    nat_sub(a3497, a3498)
                });
                Ok(append_bytes(a3499, a3500))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f516(v0: Bytes) -> R<bool> {
    let m3503 = {
        let a3501 = v0;
        let a3502 = 0u64;
        index_bytes(a3501, a3502)
    };
    match m3503 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f425({
                let v2: u8 = v1;
                let a3548 = {
                    let a3524 = {
                        let a3512 = {
                            let a3506 = {
                                let a3504 = v2;
                                let a3505 = 128u8;
                                fixed_u8::bit_and(a3504, a3505)
                            };
                            let a3507 = 128u8;
                            if equal(a3506, a3507) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3513 = {
                            let a3510 = {
                                let a3508 = v2;
                                let a3509 = 64u8;
                                fixed_u8::bit_and(a3508, a3509)
                            };
                            let a3511 = 64u8;
                            if equal(a3510, a3511) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3512, a3513)?
                    };
                    let a3525 = {
                        let a3522 = {
                            let a3516 = {
                                let a3514 = v2;
                                let a3515 = 32u8;
                                fixed_u8::bit_and(a3514, a3515)
                            };
                            let a3517 = 32u8;
                            if equal(a3516, a3517) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3523 = {
                            let a3520 = {
                                let a3518 = v2;
                                let a3519 = 16u8;
                                fixed_u8::bit_and(a3518, a3519)
                            };
                            let a3521 = 16u8;
                            if equal(a3520, a3521) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3522, a3523)?
                    };
                    nat_add(a3524, a3525)?
                };
                let a3549 = {
                    let a3546 = {
                        let a3534 = {
                            let a3528 = {
                                let a3526 = v2;
                                let a3527 = 8u8;
                                fixed_u8::bit_and(a3526, a3527)
                            };
                            let a3529 = 8u8;
                            if equal(a3528, a3529) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3535 = {
                            let a3532 = {
                                let a3530 = v2;
                                let a3531 = 4u8;
                                fixed_u8::bit_and(a3530, a3531)
                            };
                            let a3533 = 4u8;
                            if equal(a3532, a3533) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3534, a3535)?
                    };
                    let a3547 = {
                        let a3544 = {
                            let a3538 = {
                                let a3536 = v2;
                                let a3537 = 2u8;
                                fixed_u8::bit_and(a3536, a3537)
                            };
                            let a3539 = 2u8;
                            if equal(a3538, a3539) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3545 = {
                            let a3542 = {
                                let a3540 = v2;
                                let a3541 = 1u8;
                                fixed_u8::bit_and(a3540, a3541)
                            };
                            let a3543 = 1u8;
                            if equal(a3542, a3543) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3544, a3545)?
                    };
                    nat_add(a3546, a3547)?
                };
                nat_add(a3548, a3549)?
            }) {
                Ok(true)
            } else {
                Ok(f426({
                    let v3: u8 = v1;
                    let a3594 = {
                        let a3570 = {
                            let a3558 = {
                                let a3552 = {
                                    let a3550 = v3;
                                    let a3551 = 128u8;
                                    fixed_u8::bit_and(a3550, a3551)
                                };
                                let a3553 = 128u8;
                                if equal(a3552, a3553) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a3559 = {
                                let a3556 = {
                                    let a3554 = v3;
                                    let a3555 = 64u8;
                                    fixed_u8::bit_and(a3554, a3555)
                                };
                                let a3557 = 64u8;
                                if equal(a3556, a3557) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3558, a3559)?
                        };
                        let a3571 = {
                            let a3568 = {
                                let a3562 = {
                                    let a3560 = v3;
                                    let a3561 = 32u8;
                                    fixed_u8::bit_and(a3560, a3561)
                                };
                                let a3563 = 32u8;
                                if equal(a3562, a3563) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a3569 = {
                                let a3566 = {
                                    let a3564 = v3;
                                    let a3565 = 16u8;
                                    fixed_u8::bit_and(a3564, a3565)
                                };
                                let a3567 = 16u8;
                                if equal(a3566, a3567) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3568, a3569)?
                        };
                        nat_add(a3570, a3571)?
                    };
                    let a3595 = {
                        let a3592 = {
                            let a3580 = {
                                let a3574 = {
                                    let a3572 = v3;
                                    let a3573 = 8u8;
                                    fixed_u8::bit_and(a3572, a3573)
                                };
                                let a3575 = 8u8;
                                if equal(a3574, a3575) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a3581 = {
                                let a3578 = {
                                    let a3576 = v3;
                                    let a3577 = 4u8;
                                    fixed_u8::bit_and(a3576, a3577)
                                };
                                let a3579 = 4u8;
                                if equal(a3578, a3579) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3580, a3581)?
                        };
                        let a3593 = {
                            let a3590 = {
                                let a3584 = {
                                    let a3582 = v3;
                                    let a3583 = 2u8;
                                    fixed_u8::bit_and(a3582, a3583)
                                };
                                let a3585 = 2u8;
                                if equal(a3584, a3585) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a3591 = {
                                let a3588 = {
                                    let a3586 = v3;
                                    let a3587 = 1u8;
                                    fixed_u8::bit_and(a3586, a3587)
                                };
                                let a3589 = 1u8;
                                if equal(a3588, a3589) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3590, a3591)?
                        };
                        nat_add(a3592, a3593)?
                    };
                    nat_add(a3594, a3595)?
                }))
            }
        }
    }
}

pub fn f517(p3608: u64, v1: Bytes, p3609: u64, p3610: Bytes) -> R<Option<Bytes>> {
    let m3607 = (p3608, (p3609, p3610));
    {
        let mut m3607 = m3607;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3607;
                let m3596 = v0;
                if m3596 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3596 - 1;
                    let a3598 = {
                        let a3597 = v1.clone();
                        length_bytes(a3597)
                    };
                    let a3599 = v2;
                    if nat_le(a3598, a3599) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3600 = f458(v1.clone(), v2)?;
                        match m3600 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3602 = v2;
                                    let a3603 = {
                                        let (_, h3601) = v5;
                                        h3601
                                    };
                                    nat_add(a3602, a3603)?
                                }, {
                                    let a3605 = v3;
                                    let a3606 = f550({
                                        let (h3604, _) = v5;
                                        h3604
                                    });
                                    append_bytes(a3605, a3606)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f518(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h3611, _) = v1.clone();
        h3611
    };
    let v3: Bytes = {
        let (h3612, _) = {
            let (_, h3613) = v1.clone();
            h3613
        };
        h3612
    };
    let a3616 = {
        let (_, h3614) = {
            let (_, h3615) = v1;
            h3615
        };
        h3614
    };
    let a3617 = 0u64;
    if nat_eq(a3616, a3617) {
        let v4: u64 = f333(v0.clone(), v2)?;
        let a3618 = v4;
        let a3619 = 34u64;
        if nat_eq(a3618, a3619) {
            Ok(Some(({
                let a3620 = v2;
                let a3621 = 1u64;
                nat_add(a3620, a3621)?
            }, (v3, 1u64))))
        } else {
            let a3622 = v4;
            let a3623 = 92u64;
            if nat_eq(a3622, a3623) {
                f551(v0, v2, v3)
            } else {
                let a3624 = v4;
                let a3625 = 32u64;
                if if nat_lt(a3624, a3625) {
                    true
                } else {
                    let a3626 = v4;
                    let a3627 = 256u64;
                    nat_eq(a3626, a3627)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a3628 = v2;
                        let a3629 = 1u64;
                        nat_add(a3628, a3629)?
                    }, ({
                        let a3630 = v3;
                        let a3631 = f32(v0, v2, 1u64);
                        append_bytes(a3630, a3631)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f519(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f552(v0, v1)
}

pub fn f520(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f553(v0, v1)
}

pub fn f521(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3632 = v0.clone();
        length_bytes(a3632)
    };
    let a3633 = 0u64;
    let a3634 = v1;
    if nat_lt(a3633, a3634) {
        let a3635 = f554(v0.clone())?;
        let a3636 = v1;
        if nat_eq(a3635, a3636) {
            let a3637 = 1u64;
            let a3638 = v1;
            if nat_lt(a3637, a3638) {
                let m3641 = {
                    let a3639 = v0;
                    let a3640 = 0u64;
                    index_bytes(a3639, a3640)
                };
                match m3641 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3642 = v2;
                        let a3643 = 48u8;
                        Ok(!equal(a3642, a3643))
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

pub fn f522(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f555(v0, v1)
}

pub fn f523(v0: Bytes) -> R<u64> {
    let (h3644, _) = f274(Fn3::F556(v0.clone()), {
        let a3646 = {
            let a3645 = v0;
            length_bytes(a3645)
        };
        let a3647 = 1u64;
        nat_add(a3646, a3647)?
    }, 0u64)?;
    Ok(h3644)
}

pub fn f524(p3650: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m3649 = p3650;
    {
        let mut m3649 = m3649;
        loop {
            let step = {
                let v0 = m3649;
                let m3648 = v0;
                match m3648.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f37(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f88(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f88(v4, v3.clone())
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
                Ok(next) => { m3649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: Adt19, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3651 = v0;
    match m3651 {
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
        Adt19::C4(r3654) => {
            let v2 = (*r3654).clone();
            f525(v2, v1)
        }
        Adt19::C5(r3655) => {
            let v3 = (*r3655).clone();
            f525(v3, v1)
        }
        Adt19::C6 => {
            Ok(v1)
        }
        Adt19::C7(r3656, r3657) => {
            let v4 = (*r3656).clone();
            let v5 = (*r3657).clone();
            f525(v4, f525(v5, v1)?)
        }
        Adt19::C8(v6, r3658, r3659) => {
            let v7 = (*r3658).clone();
            let v8 = (*r3659).clone();
            let v9: List<Bytes> = f525(v7.clone(), f525(v8, v1)?)?;
            if f37(v6, f8(Str::lit("$ref"))) {
                let m3652 = f264(Some(v7));
                match m3652 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m3653 = f557(v10)?;
                        match m3653 {
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

pub fn f526(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3660 = f153(v1.clone(), f8(Str::lit("enum")));
    match m3660 {
        None => {
            let m3662 = f153(v1.clone(), f8(Str::lit("oneOf")));
            match m3662 {
                None => {
                    if f562(v1.clone(), v0.clone())? {
                        let m3663 = f563(v0.clone(), f429(f153(v1.clone(), f8(Str::lit("properties")))), f157(f158(f153(v1, f8(Str::lit("required"))))), true)?;
                        match m3663 {
                            Ok(v7) => {
                                Ok(f560(f417(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f221({
                                    let (h3664, _) = v7;
                                    h3664
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h3665) = v7;
                                    h3665
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt25>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m3666 = f429(f153(v1.clone(), f8(Str::lit("properties"))));
                        match m3666.uncons() {
                            None => {
                                let m3669 = f564(v1, {
                                    let a3667 = v0.clone();
                                    let a3668 = f8(Str::lit("Value"));
                                    append_bytes(a3667, a3668)
                                })?;
                                match m3669 {
                                    Ok(v9) => {
                                        Ok(f560(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h3670, _) = v9;
                                            h3670
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3671) = v9;
                                            h3671
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m3672 = f563(v0.clone(), f429(f153(v1.clone(), f8(Str::lit("properties")))), f157(f158(f153(v1, f8(Str::lit("required"))))), false)?;
                                match m3672 {
                                    Ok(v13) => {
                                        Ok(f560(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m3674 = {
                                                let (h3673, _) = v13.clone();
                                                h3673
                                            };
                                            match m3674.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f221({
                                                        let (h3675, _) = v13;
                                                        h3675
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3676) = v13;
                                            h3676
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
                    f561(v0, f265(v5))
                }
            }
        }
        Some(v2) => {
            let m3661 = f558(f265(v2), List::<Bytes>::nil())?;
            match m3661 {
                Ok(v4) => {
                    Ok(f560(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f221(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f559(v3, v0))
                }
            }
        }
    }
}

pub fn f527(v0: Adt19) -> Adt19 {
    let v2: Adt19 = {
        let m3677 = f153(v0.clone(), f8(Str::lit("anyOf")));
        match m3677 {
            None => {
                v0
            }
            Some(v1) => {
                if f565(f265(v1.clone())) {
                    f566(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f567(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f568(f265(v1.clone()))
                    } {
                        v0
                    } else {
                        f569(f566(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt19 = {
        let m3678 = f153(v2.clone(), f8(Str::lit("oneOf")));
        match m3678 {
            None => {
                v2
            }
            Some(v3) => {
                if f565(f265(v3)) {
                    f566(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m3679 = f153(v4.clone(), f8(Str::lit("allOf")));
    match m3679 {
        None => {
            v4
        }
        Some(v5) => {
            if f570(f265(v5)) {
                f566(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f528(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3724 = {
            let a3700 = {
                let a3688 = {
                    let a3682 = {
                        let a3680 = v3;
                        let a3681 = 128u8;
                        fixed_u8::bit_and(a3680, a3681)
                    };
                    let a3683 = 128u8;
                    if equal(a3682, a3683) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3689 = {
                    let a3686 = {
                        let a3684 = v3;
                        let a3685 = 64u8;
                        fixed_u8::bit_and(a3684, a3685)
                    };
                    let a3687 = 64u8;
                    if equal(a3686, a3687) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3688, a3689)?
            };
            let a3701 = {
                let a3698 = {
                    let a3692 = {
                        let a3690 = v3;
                        let a3691 = 32u8;
                        fixed_u8::bit_and(a3690, a3691)
                    };
                    let a3693 = 32u8;
                    if equal(a3692, a3693) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3699 = {
                    let a3696 = {
                        let a3694 = v3;
                        let a3695 = 16u8;
                        fixed_u8::bit_and(a3694, a3695)
                    };
                    let a3697 = 16u8;
                    if equal(a3696, a3697) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3698, a3699)?
            };
            nat_add(a3700, a3701)?
        };
        let a3725 = {
            let a3722 = {
                let a3710 = {
                    let a3704 = {
                        let a3702 = v3;
                        let a3703 = 8u8;
                        fixed_u8::bit_and(a3702, a3703)
                    };
                    let a3705 = 8u8;
                    if equal(a3704, a3705) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3711 = {
                    let a3708 = {
                        let a3706 = v3;
                        let a3707 = 4u8;
                        fixed_u8::bit_and(a3706, a3707)
                    };
                    let a3709 = 4u8;
                    if equal(a3708, a3709) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3710, a3711)?
            };
            let a3723 = {
                let a3720 = {
                    let a3714 = {
                        let a3712 = v3;
                        let a3713 = 2u8;
                        fixed_u8::bit_and(a3712, a3713)
                    };
                    let a3715 = 2u8;
                    if equal(a3714, a3715) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3721 = {
                    let a3718 = {
                        let a3716 = v3;
                        let a3717 = 1u8;
                        fixed_u8::bit_and(a3716, a3717)
                    };
                    let a3719 = 1u8;
                    if equal(a3718, a3719) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3720, a3721)?
            };
            nat_add(a3722, a3723)?
        };
        nat_add(a3724, a3725)?
    };
    let a3726 = v4;
    let a3727 = 34u64;
    if nat_eq(a3726, a3727) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3728 = v4;
        let a3729 = 92u64;
        if nat_eq(a3728, a3729) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3730 = v4;
            let a3731 = 8u64;
            if nat_eq(a3730, a3731) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3732 = v4;
                let a3733 = 12u64;
                if nat_eq(a3732, a3733) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3734 = v4;
                    let a3735 = 10u64;
                    if nat_eq(a3734, a3735) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3736 = v4;
                        let a3737 = 13u64;
                        if nat_eq(a3736, a3737) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3738 = v4;
                            let a3739 = 9u64;
                            if nat_eq(a3738, a3739) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3740 = v4;
                                let a3741 = 32u64;
                                if nat_lt(a3740, a3741) {
                                    let a3742 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3743 = f571(v2)?;
                                    Ok(append_bytes(a3742, a3743))
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

pub fn f529(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f572(Fn21::F573, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f530(v0: Fn17, p3747: List<List<(Bytes, Bytes)>>, p3748: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3746 = (p3747, p3748);
    {
        let mut m3746 = m3746;
        loop {
            let step = {
                let (v1, v2) = m3746;
                let m3744 = v2;
                match m3744.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3745: Fn17 = v0.clone();
                            c3745.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f531(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f532(v0: Fn18, p3753: u64, p3754: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3752 = (p3753, p3754);
    {
        let mut m3752 = m3752;
        loop {
            let step = {
                let (v1, v2) = m3752;
                let m3749 = v1;
                if m3749 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3749 - 1;
                    let m3751 = {
                        let c3750: Fn18 = v0.clone();
                        c3750.apply(v2.clone())?
                    };
                    match m3751 {
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
                Ok(next) => { m3752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f533(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f574(v0)
}

pub fn f534(v0: Fn19, p3759: u64, p3760: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m3758 = (p3759, p3760);
    {
        let mut m3758 = m3758;
        loop {
            let step = {
                let (v1, v2) = m3758;
                let m3755 = v1;
                if m3755 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3755 - 1;
                    let m3757 = {
                        let c3756: Fn19 = v0.clone();
                        c3756.apply(v2.clone())?
                    };
                    match m3757 {
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
                Ok(next) => { m3758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f535(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f575(v0)
}

pub fn f536(v0: u64) -> u32 {
    let a3761 = v0;
    let a3762 = 32u64;
    if nat_lt(a3761, a3762) {
        let a3763 = v0;
        let a3764 = 16u64;
        if nat_lt(a3763, a3764) {
            let a3765 = v0;
            let a3766 = 8u64;
            if nat_lt(a3765, a3766) {
                let a3767 = v0;
                let a3768 = 4u64;
                if nat_lt(a3767, a3768) {
                    let a3769 = v0;
                    let a3770 = 2u64;
                    if nat_lt(a3769, a3770) {
                        let a3771 = v0;
                        let a3772 = 1u64;
                        if nat_lt(a3771, a3772) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3773 = v0;
                        let a3774 = 3u64;
                        if nat_lt(a3773, a3774) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3775 = v0;
                    let a3776 = 6u64;
                    if nat_lt(a3775, a3776) {
                        let a3777 = v0;
                        let a3778 = 5u64;
                        if nat_lt(a3777, a3778) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3779 = v0;
                        let a3780 = 7u64;
                        if nat_lt(a3779, a3780) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3781 = v0;
                let a3782 = 12u64;
                if nat_lt(a3781, a3782) {
                    let a3783 = v0;
                    let a3784 = 10u64;
                    if nat_lt(a3783, a3784) {
                        let a3785 = v0;
                        let a3786 = 9u64;
                        if nat_lt(a3785, a3786) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3787 = v0;
                        let a3788 = 11u64;
                        if nat_lt(a3787, a3788) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3789 = v0;
                    let a3790 = 14u64;
                    if nat_lt(a3789, a3790) {
                        let a3791 = v0;
                        let a3792 = 13u64;
                        if nat_lt(a3791, a3792) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3793 = v0;
                        let a3794 = 15u64;
                        if nat_lt(a3793, a3794) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3795 = v0;
            let a3796 = 24u64;
            if nat_lt(a3795, a3796) {
                let a3797 = v0;
                let a3798 = 20u64;
                if nat_lt(a3797, a3798) {
                    let a3799 = v0;
                    let a3800 = 18u64;
                    if nat_lt(a3799, a3800) {
                        let a3801 = v0;
                        let a3802 = 17u64;
                        if nat_lt(a3801, a3802) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3803 = v0;
                        let a3804 = 19u64;
                        if nat_lt(a3803, a3804) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3805 = v0;
                    let a3806 = 22u64;
                    if nat_lt(a3805, a3806) {
                        let a3807 = v0;
                        let a3808 = 21u64;
                        if nat_lt(a3807, a3808) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3809 = v0;
                        let a3810 = 23u64;
                        if nat_lt(a3809, a3810) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3811 = v0;
                let a3812 = 28u64;
                if nat_lt(a3811, a3812) {
                    let a3813 = v0;
                    let a3814 = 26u64;
                    if nat_lt(a3813, a3814) {
                        let a3815 = v0;
                        let a3816 = 25u64;
                        if nat_lt(a3815, a3816) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3817 = v0;
                        let a3818 = 27u64;
                        if nat_lt(a3817, a3818) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3819 = v0;
                    let a3820 = 30u64;
                    if nat_lt(a3819, a3820) {
                        let a3821 = v0;
                        let a3822 = 29u64;
                        if nat_lt(a3821, a3822) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3823 = v0;
                        let a3824 = 31u64;
                        if nat_lt(a3823, a3824) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3825 = v0;
        let a3826 = 48u64;
        if nat_lt(a3825, a3826) {
            let a3827 = v0;
            let a3828 = 40u64;
            if nat_lt(a3827, a3828) {
                let a3829 = v0;
                let a3830 = 36u64;
                if nat_lt(a3829, a3830) {
                    let a3831 = v0;
                    let a3832 = 34u64;
                    if nat_lt(a3831, a3832) {
                        let a3833 = v0;
                        let a3834 = 33u64;
                        if nat_lt(a3833, a3834) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3835 = v0;
                        let a3836 = 35u64;
                        if nat_lt(a3835, a3836) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3837 = v0;
                    let a3838 = 38u64;
                    if nat_lt(a3837, a3838) {
                        let a3839 = v0;
                        let a3840 = 37u64;
                        if nat_lt(a3839, a3840) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3841 = v0;
                        let a3842 = 39u64;
                        if nat_lt(a3841, a3842) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3843 = v0;
                let a3844 = 44u64;
                if nat_lt(a3843, a3844) {
                    let a3845 = v0;
                    let a3846 = 42u64;
                    if nat_lt(a3845, a3846) {
                        let a3847 = v0;
                        let a3848 = 41u64;
                        if nat_lt(a3847, a3848) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3849 = v0;
                        let a3850 = 43u64;
                        if nat_lt(a3849, a3850) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3851 = v0;
                    let a3852 = 46u64;
                    if nat_lt(a3851, a3852) {
                        let a3853 = v0;
                        let a3854 = 45u64;
                        if nat_lt(a3853, a3854) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3855 = v0;
                        let a3856 = 47u64;
                        if nat_lt(a3855, a3856) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3857 = v0;
            let a3858 = 56u64;
            if nat_lt(a3857, a3858) {
                let a3859 = v0;
                let a3860 = 52u64;
                if nat_lt(a3859, a3860) {
                    let a3861 = v0;
                    let a3862 = 50u64;
                    if nat_lt(a3861, a3862) {
                        let a3863 = v0;
                        let a3864 = 49u64;
                        if nat_lt(a3863, a3864) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3865 = v0;
                        let a3866 = 51u64;
                        if nat_lt(a3865, a3866) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3867 = v0;
                    let a3868 = 54u64;
                    if nat_lt(a3867, a3868) {
                        let a3869 = v0;
                        let a3870 = 53u64;
                        if nat_lt(a3869, a3870) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3871 = v0;
                        let a3872 = 55u64;
                        if nat_lt(a3871, a3872) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3873 = v0;
                let a3874 = 60u64;
                if nat_lt(a3873, a3874) {
                    let a3875 = v0;
                    let a3876 = 58u64;
                    if nat_lt(a3875, a3876) {
                        let a3877 = v0;
                        let a3878 = 57u64;
                        if nat_lt(a3877, a3878) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3879 = v0;
                        let a3880 = 59u64;
                        if nat_lt(a3879, a3880) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3881 = v0;
                    let a3882 = 62u64;
                    if nat_lt(a3881, a3882) {
                        let a3883 = v0;
                        let a3884 = 61u64;
                        if nat_lt(a3883, a3884) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3885 = v0;
                        let a3886 = 63u64;
                        if nat_lt(a3885, a3886) {
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

pub fn f537(v0: u32) -> u32 {
    let a3889 = {
        let a3887 = f576(v0, 6u32, 26u32);
        let a3888 = f576(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3887, a3888)
    };
    let a3890 = f576(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3889, a3890)
}

pub fn f538(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3896 = {
        let a3891 = v0;
        let a3892 = v1;
        fixed_u32::bit_and(a3891, a3892)
    };
    let a3897 = {
        let a3894 = {
            let a3893 = v0;
            fixed_u32::bit_not(a3893)
        };
        let a3895 = v2;
        fixed_u32::bit_and(a3894, a3895)
    };
    fixed_u32::bit_xor(a3896, a3897)
}

pub fn f539(v0: u32) -> u32 {
    let a3900 = {
        let a3898 = f576(v0, 2u32, 30u32);
        let a3899 = f576(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3898, a3899)
    };
    let a3901 = f576(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3900, a3901)
}

pub fn f540(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3910 = {
        let a3906 = {
            let a3902 = v0;
            let a3903 = v1;
            fixed_u32::bit_and(a3902, a3903)
        };
        let a3907 = {
            let a3904 = v0;
            let a3905 = v2;
            fixed_u32::bit_and(a3904, a3905)
        };
        fixed_u32::bit_xor(a3906, a3907)
    };
    let a3911 = {
        let a3908 = v1;
        let a3909 = v2;
        fixed_u32::bit_and(a3908, a3909)
    };
    fixed_u32::bit_xor(a3910, a3911)
}

pub fn f541(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3912, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3912
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3913, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3913
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3914, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3914
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3915, _, _, _, _, _, _, _, _, _, _, _) => {
            h3915
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3916, _, _, _, _, _, _, _, _, _, _) => {
            h3916
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3917, _, _, _, _, _, _, _, _, _) => {
            h3917
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3918, _, _, _, _, _, _, _, _) => {
            h3918
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3919, _, _, _, _, _, _, _) => {
            h3919
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3920, _, _, _, _, _, _) => {
            h3920
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3921, _, _, _, _, _) => {
            h3921
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3922, _, _, _, _) => {
            h3922
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3923, _, _, _) => {
            h3923
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3924, _, _) => {
            h3924
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3925, _) => {
            h3925
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3926) => {
            h3926
        }
    }, f455(f577(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3927, _) => {
            h3927
        }
    }), f455(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3928, _, _, _, _, _, _) => {
            h3928
        }
    }, f455(f578(match v0.clone() {
        Adt26::C0(_, h3929, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3929
        }
    }), match v0 {
        Adt26::C0(h3930, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3930
        }
    }))))
}

pub fn f542(v0: Fn20, p3934: u64, p3935: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3933 = (p3934, p3935);
    {
        let mut m3933 = m3933;
        loop {
            let step = {
                let (v1, v2) = m3933;
                let m3931 = v1;
                if m3931 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3931 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3932: Fn20 = v0.clone();
                        c3932.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3933 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f543(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3936, _) = v0;
        h3936
    };
    let v2: u32 = {
        let (h3937, _) = {
            let (_, h3938) = v0;
            h3938
        };
        h3937
    };
    let v3: u32 = {
        let (_, h3939) = {
            let (_, h3940) = v0;
            h3940
        };
        h3939
    };
    ({
        let a3941 = v1;
        let a3942 = 2u64;
        let a3943 = 0u64;
        nat_quot(a3941, a3942, a3943)
    }, (f544(v2, 1u32), {
        let a3947 = {
            let a3944 = v1;
            let a3945 = 2u64;
            let a3946 = 0u64;
            nat_rem(a3944, a3945, a3946)
        };
        let a3948 = 1u64;
        if nat_eq(a3947, a3948) {
            let a3949 = v3;
            let a3950 = v2;
            fixed_u32::bit_or(a3949, a3950)
        } else {
            v3
        }
    }))
}

pub fn f544(v0: u32, v1: u32) -> u32 {
    let m3953 = {
        let a3951 = v0;
        let a3952 = v1;
        fixed_u32::shift_left(a3951, a3952)
    };
    match m3953 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f545(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f233(v0, {
        let (h3954, _) = v2;
        h3954
    })?;
    let a3956 = 1114111u64;
    let a3957 = {
        let (_, h3955) = v2;
        h3955
    };
    if nat_lt(a3956, a3957) {
        Ok(None::<(u64, u64)>)
    } else {
        let a3958 = 48u64;
        let a3959 = v3;
        if if nat_le(a3958, a3959) {
            let a3960 = v3;
            let a3961 = 57u64;
            nat_le(a3960, a3961)
        } else {
            false
        } {
            Ok(Some(({
                let a3963 = {
                    let (h3962, _) = v2;
                    h3962
                };
                let a3964 = 1u64;
                nat_add(a3963, a3964)?
            }, {
                let a3970 = {
                    let a3966 = {
                        let (_, h3965) = v2;
                        h3965
                    };
                    let a3967 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3966, a3967)?
                };
                let a3971 = {
                    let a3968 = v3;
                    let a3969 = 48u64;
                    nat_sub(a3968, a3969)
                };
                nat_add(a3970, a3971)?
            })))
        } else {
            if if v1 {
                let a3972 = 97u64;
                let a3973 = v3;
                if nat_le(a3972, a3973) {
                    let a3974 = v3;
                    let a3975 = 102u64;
                    nat_le(a3974, a3975)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a3977 = {
                        let (h3976, _) = v2;
                        h3976
                    };
                    let a3978 = 1u64;
                    nat_add(a3977, a3978)?
                }, {
                    let a3984 = {
                        let a3980 = {
                            let (_, h3979) = v2;
                            h3979
                        };
                        let a3981 = 16u64;
                        nat_mul(a3980, a3981)?
                    };
                    let a3985 = {
                        let a3982 = v3;
                        let a3983 = 87u64;
                        nat_sub(a3982, a3983)
                    };
                    nat_add(a3984, a3985)?
                })))
            } else {
                if if v1 {
                    let a3986 = 65u64;
                    let a3987 = v3;
                    if nat_le(a3986, a3987) {
                        let a3988 = v3;
                        let a3989 = 70u64;
                        nat_le(a3988, a3989)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a3991 = {
                            let (h3990, _) = v2;
                            h3990
                        };
                        let a3992 = 1u64;
                        nat_add(a3991, a3992)?
                    }, {
                        let a3998 = {
                            let a3994 = {
                                let (_, h3993) = v2;
                                h3993
                            };
                            let a3995 = 16u64;
                            nat_mul(a3994, a3995)?
                        };
                        let a3999 = {
                            let a3996 = v3;
                            let a3997 = 55u64;
                            nat_sub(a3996, a3997)
                        };
                        nat_add(a3998, a3999)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f546(p4006: List<Bytes>, p4007: u64) -> Bytes {
    let m4005 = (p4006, p4007);
    {
        let mut m4005 = m4005;
        loop {
            let step = {
                let (v0, v1) = m4005;
                let m4000 = v0;
                match m4000.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a4001 = v1;
                        let a4002 = 0u64;
                        if nat_eq(a4001, a4002) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a4003 = v1;
                                let a4004 = 1u64;
                                nat_sub(a4003, a4004)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4005 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(v0: u64) -> List<Bytes> {
    let a4008 = v0;
    let a4009 = 0u64;
    if nat_eq(a4008, a4009) {
        f579()
    } else {
        let a4010 = v0;
        let a4011 = 1u64;
        if nat_eq(a4010, a4011) {
            f580()
        } else {
            let a4012 = v0;
            let a4013 = 2u64;
            if nat_eq(a4012, a4013) {
                f581()
            } else {
                let a4014 = v0;
                let a4015 = 3u64;
                if nat_eq(a4014, a4015) {
                    f582()
                } else {
                    let a4016 = v0;
                    let a4017 = 4u64;
                    if nat_eq(a4016, a4017) {
                        f583()
                    } else {
                        let a4018 = v0;
                        let a4019 = 5u64;
                        if nat_eq(a4018, a4019) {
                            f584()
                        } else {
                            let a4020 = v0;
                            let a4021 = 6u64;
                            if nat_eq(a4020, a4021) {
                                f585()
                            } else {
                                let a4022 = v0;
                                let a4023 = 7u64;
                                if nat_eq(a4022, a4023) {
                                    f586()
                                } else {
                                    let a4024 = v0;
                                    let a4025 = 8u64;
                                    if nat_eq(a4024, a4025) {
                                        f587()
                                    } else {
                                        let a4026 = v0;
                                        let a4027 = 9u64;
                                        if nat_eq(a4026, a4027) {
                                            f588()
                                        } else {
                                            let a4028 = v0;
                                            let a4029 = 10u64;
                                            if nat_eq(a4028, a4029) {
                                                f589()
                                            } else {
                                                let a4030 = v0;
                                                let a4031 = 11u64;
                                                if nat_eq(a4030, a4031) {
                                                    f590()
                                                } else {
                                                    let a4032 = v0;
                                                    let a4033 = 12u64;
                                                    if nat_eq(a4032, a4033) {
                                                        f591()
                                                    } else {
                                                        let a4034 = v0;
                                                        let a4035 = 13u64;
                                                        if nat_eq(a4034, a4035) {
                                                            f592()
                                                        } else {
                                                            let a4036 = v0;
                                                            let a4037 = 14u64;
                                                            if nat_eq(a4036, a4037) {
                                                                f593()
                                                            } else {
                                                                f594()
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

pub fn f548(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a4038 = v0;
        let a4039 = v2;
        if if if nat_eq(a4038, a4039) {
            true
        } else {
            let a4040 = v0;
            let a4041 = 60u64;
            nat_eq(a4040, a4041)
        } {
            true
        } else {
            let a4042 = v0;
            let a4043 = 38u64;
            if nat_eq(a4042, a4043) {
                true
            } else {
                let a4044 = v0;
                let a4045 = 256u64;
                nat_eq(a4044, a4045)
            }
        } {
            true
        } else {
            let a4046 = v0;
            let a4047 = 9u64;
            if nat_eq(a4046, a4047) {
                true
            } else {
                let a4048 = v0;
                let a4049 = 10u64;
                nat_eq(a4048, a4049)
            }
        }
    } else {
        let a4050 = v0;
        let a4051 = 60u64;
        if if nat_eq(a4050, a4051) {
            true
        } else {
            let a4052 = v0;
            let a4053 = 38u64;
            nat_eq(a4052, a4053)
        } {
            true
        } else {
            let a4054 = v0;
            let a4055 = 93u64;
            if nat_eq(a4054, a4055) {
                true
            } else {
                let a4056 = v0;
                let a4057 = 256u64;
                nat_eq(a4056, a4057)
            }
        }
    }
}

pub fn f549(p4070: u64, v1: Bytes, p4071: u64, p4072: Bytes) -> R<Option<Bytes>> {
    let m4069 = (p4070, (p4071, p4072));
    {
        let mut m4069 = m4069;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4069;
                let m4058 = v0;
                if m4058 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4058 - 1;
                    let a4060 = {
                        let a4059 = v1.clone();
                        length_bytes(a4059)
                    };
                    let a4061 = v2;
                    if nat_le(a4060, a4061) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4062 = f458(v1.clone(), v2)?;
                        match m4062 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4064 = v2;
                                    let a4065 = {
                                        let (_, h4063) = v5;
                                        h4063
                                    };
                                    nat_add(a4064, a4065)?
                                }, {
                                    let a4067 = v3;
                                    let a4068 = f595(v1.clone(), v2, {
                                        let (h4066, _) = v5;
                                        h4066
                                    });
                                    append_bytes(a4067, a4068)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: u64) -> Bytes {
    let a4073 = f8(Str::lit("u"));
    let a4074 = f596(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4073, a4074)
}

pub fn f551(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f333(v0.clone(), {
        let a4075 = v1;
        let a4076 = 1u64;
        nat_add(a4075, a4076)?
    })?;
    let v4: Bytes = {
        let a4077 = v3;
        let a4078 = 117u64;
        if nat_eq(a4077, a4078) {
            f597(v0, v1)?
        } else {
            f598(v3)
        }
    };
    let a4079 = v4.clone();
    let a4080 = Bytes::lit(&[]);
    if equal(a4079, a4080) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a4081 = v3;
            let a4082 = 117u64;
            if nat_eq(a4081, a4082) {
                let a4083 = v1;
                let a4084 = 6u64;
                nat_add(a4083, a4084)?
            } else {
                let a4085 = v1;
                let a4086 = 2u64;
                nat_add(a4085, a4086)?
            }
        }, ({
            let a4087 = v2;
            let a4088 = v4;
            append_bytes(a4087, a4088)
        }, 0u64))))
    }
}

pub fn f552(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f333(v0, v1)?;
    let a4089 = 47u64;
    let a4090 = v2;
    if if nat_lt(a4089, a4090) {
        let a4091 = v2;
        let a4092 = 58u64;
        nat_lt(a4091, a4092)
    } else {
        false
    } {
        Ok(Some({
            let a4093 = v1;
            let a4094 = 1u64;
            nat_add(a4093, a4094)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f553(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a4095 = f333(v0, v1)?;
    let a4096 = 48u64;
    if nat_eq(a4095, a4096) {
        Ok(Some({
            let a4097 = v1;
            let a4098 = 1u64;
            nat_add(a4097, a4098)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f554(v0: Bytes) -> R<u64> {
    let (h4099, _) = f274(Fn3::F599(v0.clone()), {
        let a4100 = v0;
        length_bytes(a4100)
    }, 0u64)?;
    Ok(h4099)
}

pub fn f555(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4104 = {
        let a4102 = v0;
        let a4103 = {
            let (h4101, _) = v1;
            h4101
        };
        index_bytes(a4102, a4103)
    };
    match m4104 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a4106 = {
                    let (h4105, _) = v1;
                    h4105
                };
                let a4107 = 1u64;
                nat_add(a4106, a4107)?
            }, {
                let a4111 = {
                    let a4109 = {
                        let (_, h4108) = v1;
                        h4108
                    };
                    let a4110 = 10u64;
                    nat_mul(a4109, a4110)?
                };
                let a4112 = f600(v2)?;
                nat_add(a4111, a4112)?
            })))
        }
    }
}

pub fn f556(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f601(v0, v1)
}

pub fn f557(v0: Bytes) -> R<Option<Bytes>> {
    if f211(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f260(f32(v0.clone(), 8u64, {
            let a4114 = {
                let a4113 = v0;
                length_bytes(a4113)
            };
            let a4115 = 8u64;
            nat_sub(a4114, a4115)
        }))?))
    } else {
        if f211(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f260(f32(v0.clone(), 14u64, {
                let a4117 = {
                    let a4116 = v0;
                    length_bytes(a4116)
                };
                let a4118 = 14u64;
                nat_sub(a4117, a4118)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f558(v0: List<Adt19>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f415(f416(f157(v0.clone()))?, v0, v1)
}

pub fn f559(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt25>), Bytes> {
    Err::<(Bytes, List<Adt25>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f560(v0: Bytes, v1: List<Adt25>) -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((v0, v1))
}

pub fn f561(v0: Bytes, v1: List<Adt19>) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4119 = v1.clone();
    match m4119.uncons() {
        None => {
            Ok(f559(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m4120 = f602(f429(f153(v2, f8(Str::lit("properties")))), v1.clone());
            match m4120.uncons() {
                None => {
                    let m4121 = f603(f416(f604(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m4121 {
                        Ok(v5) => {
                            Ok(f560(f417(v0, {
                                let (h4122, _) = v5;
                                h4122
                            }), {
                                let (_, h4123) = v5;
                                h4123
                            }))
                        }
                        Err(v4) => {
                            Ok(f559(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m4124 = f605(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m4124 {
                        Ok(v9) => {
                            Ok(f560(f417(v0, {
                                let (h4125, _) = v9;
                                h4125
                            }), {
                                let (_, h4126) = v9;
                                h4126
                            }))
                        }
                        Err(v8) => {
                            Ok(f559(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f562(v0: Adt19, v1: Bytes) -> R<bool> {
    let m4127 = v0;
    match m4127 {
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
        Adt19::C4(r4128) => {
            let v2 = (*r4128).clone();
            f562(v2, v1)
        }
        Adt19::C5(r4129) => {
            let v3 = (*r4129).clone();
            f562(v3, v1)
        }
        Adt19::C6 => {
            Ok(false)
        }
        Adt19::C7(r4130, r4131) => {
            let v4 = (*r4130).clone();
            let v5 = (*r4131).clone();
            if f562(v4, v1.clone())? {
                Ok(true)
            } else {
                f562(v5, v1)
            }
        }
        Adt19::C8(v6, r4132, r4133) => {
            let v7 = (*r4132).clone();
            let v8 = (*r4133).clone();
            if if f37(v6, f8(Str::lit("$ref"))) {
                f606(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f562(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f562(v8, v1)
                }
            }
        }
    }
}

pub fn f563(v0: Bytes, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f607(f416(f608(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f564(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4134 = f609(v0.clone());
    match m4134 {
        None => {
            if f37(f156(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4135 = f153(v0, f8(Str::lit("items")));
                match m4135 {
                    None => {
                        Ok(f610(f8(Str::lit("List")), f612()))
                    }
                    Some(v3) => {
                        Ok(f610(f8(Str::lit("List")), f611(v3, {
                            let a4136 = v1;
                            let a4137 = f8(Str::lit("Item"));
                            append_bytes(a4136, a4137)
                        })?))
                    }
                }
            } else {
                f611(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f610(f8(Str::lit("Option")), f611(v2, v1)?))
        }
    }
}

pub fn f565(p4140: List<Adt19>) -> bool {
    let m4139 = p4140;
    {
        let mut m4139 = m4139;
        loop {
            let step = {
                let v0 = m4139;
                let m4138 = v0;
                match m4138.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f613(f479(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt19>, bool>(v2)
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f566(v0: Adt19, v1: Bytes) -> Adt19 {
    let m4141 = v0;
    match m4141 {
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
        Adt19::C4(r4142) => {
            let v13 = (*r4142).clone();
            Adt19::C4(std::rc::Rc::new(v13))
        }
        Adt19::C5(r4143) => {
            let v5 = (*r4143).clone();
            Adt19::C5(std::rc::Rc::new(f566(v5, v1)))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r4144, r4145) => {
            let v14 = (*r4144).clone();
            let v15 = (*r4145).clone();
            Adt19::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt19::C8(v2, r4146, r4147) => {
            let v3 = (*r4146).clone();
            let v4 = (*r4147).clone();
            if f37(v2.clone(), v1.clone()) {
                f566(v4, v1)
            } else {
                Adt19::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f566(v4, v1)))
            }
        }
    }
}

pub fn f567(v0: Adt19, v1: Bytes) -> bool {
    let m4148 = f153(v0, v1);
    match m4148 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f568(v0: List<Adt19>) -> bool {
    let m4149 = v0.clone();
    match m4149.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m4150 = f602(f429(f153(v1, f8(Str::lit("properties")))), v0.clone());
            match m4150.uncons() {
                None => {
                    f615(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f614(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f569(v0: Adt19, v1: Bytes, v2: Adt19) -> Adt19 {
    let m4151 = v0;
    match m4151 {
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
        Adt19::C4(r4152) => {
            let v14 = (*r4152).clone();
            Adt19::C4(std::rc::Rc::new(v14))
        }
        Adt19::C5(r4153) => {
            let v3 = (*r4153).clone();
            Adt19::C5(std::rc::Rc::new(Adt19::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r4154, r4155) => {
            let v15 = (*r4154).clone();
            let v16 = (*r4155).clone();
            Adt19::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt19::C8(v4, r4156, r4157) => {
            let v5 = (*r4156).clone();
            let v6 = (*r4157).clone();
            Adt19::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f570(p4161: List<Adt19>) -> bool {
    let m4160 = p4161;
    {
        let mut m4160 = m4160;
        loop {
            let step = {
                let v0 = m4160;
                let m4158 = v0;
                match m4158.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f613(f479(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f565(f265({
                                let m4159 = f153(v1, f8(Str::lit("anyOf")));
                                match m4159 {
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
                Ok(next) => { m4160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f571(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4206 = {
            let a4182 = {
                let a4170 = {
                    let a4164 = {
                        let a4162 = v1;
                        let a4163 = 128u8;
                        fixed_u8::bit_and(a4162, a4163)
                    };
                    let a4165 = 128u8;
                    if equal(a4164, a4165) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4171 = {
                    let a4168 = {
                        let a4166 = v1;
                        let a4167 = 64u8;
                        fixed_u8::bit_and(a4166, a4167)
                    };
                    let a4169 = 64u8;
                    if equal(a4168, a4169) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4170, a4171)?
            };
            let a4183 = {
                let a4180 = {
                    let a4174 = {
                        let a4172 = v1;
                        let a4173 = 32u8;
                        fixed_u8::bit_and(a4172, a4173)
                    };
                    let a4175 = 32u8;
                    if equal(a4174, a4175) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4181 = {
                    let a4178 = {
                        let a4176 = v1;
                        let a4177 = 16u8;
                        fixed_u8::bit_and(a4176, a4177)
                    };
                    let a4179 = 16u8;
                    if equal(a4178, a4179) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4180, a4181)?
            };
            nat_add(a4182, a4183)?
        };
        let a4207 = {
            let a4204 = {
                let a4192 = {
                    let a4186 = {
                        let a4184 = v1;
                        let a4185 = 8u8;
                        fixed_u8::bit_and(a4184, a4185)
                    };
                    let a4187 = 8u8;
                    if equal(a4186, a4187) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4193 = {
                    let a4190 = {
                        let a4188 = v1;
                        let a4189 = 4u8;
                        fixed_u8::bit_and(a4188, a4189)
                    };
                    let a4191 = 4u8;
                    if equal(a4190, a4191) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4192, a4193)?
            };
            let a4205 = {
                let a4202 = {
                    let a4196 = {
                        let a4194 = v1;
                        let a4195 = 2u8;
                        fixed_u8::bit_and(a4194, a4195)
                    };
                    let a4197 = 2u8;
                    if equal(a4196, a4197) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4203 = {
                    let a4200 = {
                        let a4198 = v1;
                        let a4199 = 1u8;
                        fixed_u8::bit_and(a4198, a4199)
                    };
                    let a4201 = 1u8;
                    if equal(a4200, a4201) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4202, a4203)?
            };
            nat_add(a4204, a4205)?
        };
        nat_add(a4206, a4207)?
    };
    let a4214 = f337(f616(), {
        let a4208 = v2;
        let a4209 = 16u64;
        let a4210 = 0u64;
        nat_quot(a4208, a4209, a4210)
    });
    let a4215 = f337(f616(), {
        let a4211 = v2;
        let a4212 = 16u64;
        let a4213 = 0u64;
        nat_rem(a4211, a4212, a4213)
    });
    Ok(append_bytes(a4214, a4215))
}

pub fn f572(v0: Fn21, p4219: List<List<(Bytes, Bytes)>>, p4220: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4218 = (p4219, p4220);
    {
        let mut m4218 = m4218;
        loop {
            let step = {
                let (v1, v2) = m4218;
                let m4216 = v2;
                match m4216.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4217: Fn21 = v0.clone();
                            c4217.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f574(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4222 = {
        let a4221 = v0.clone();
        length_list(a4221)
    };
    let a4223 = 1u64;
    if nat_le(a4222, a4223) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f617(v0)?))
    }
}

pub fn f575(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h4224, _) = v0;
        h4224
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h4225) = v0;
        h4225
    };
    let m4226 = v1;
    match m4226.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m4227 = v4;
            match m4227.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f618(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f576(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4228 = f393(v0, v1);
    let a4229 = f544(v0, v2);
    fixed_u32::bit_or(a4228, a4229)
}

pub fn f577(v0: u32) -> u32 {
    let a4232 = {
        let a4230 = f576(v0, 17u32, 15u32);
        let a4231 = f576(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4230, a4231)
    };
    let a4233 = f393(v0, 10u32);
    fixed_u32::bit_xor(a4232, a4233)
}

pub fn f578(v0: u32) -> u32 {
    let a4236 = {
        let a4234 = f576(v0, 7u32, 25u32);
        let a4235 = f576(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4234, a4235)
    };
    let a4237 = f393(v0, 3u32);
    fixed_u32::bit_xor(a4236, a4237)
}

pub fn f579() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f580() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f581() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f582() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f583() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f584() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f585() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f586() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f587() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f588() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f589() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f590() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f591() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f592() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f593() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f594() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f595(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f425(v2) {
        true
    } else {
        f426(v2)
    } {
        let a4238 = v2;
        let a4239 = 117u64;
        if !nat_eq(a4238, a4239) {
            let a4240 = v2;
            let a4241 = 85u64;
            !nat_eq(a4240, a4241)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f427(v2) {
            let a4242 = 0u64;
            let a4243 = v1;
            nat_lt(a4242, a4243)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a4244 = v2;
        let a4245 = 65536u64;
        if nat_lt(a4244, a4245) {
            let a4246 = f8(Str::lit("u"));
            let a4247 = f596(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4246, a4247)
        } else {
            let a4248 = f8(Str::lit("U"));
            let a4249 = f596(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4248, a4249)
        }
    }
}

pub fn f596(p4266: u64, p4267: u64, p4268: u64, p4269: Bytes) -> Bytes {
    let m4265 = (p4266, (p4267, (p4268, p4269)));
    {
        let mut m4265 = m4265;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4265;
                let m4250 = v0;
                if m4250 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4250 - 1;
                    let a4251 = v1;
                    let a4252 = 0u64;
                    if if nat_eq(a4251, a4252) {
                        let a4253 = v2;
                        let a4254 = 0u64;
                        nat_eq(a4253, a4254)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4255 = v1;
                            let a4256 = 16u64;
                            let a4257 = 0u64;
                            nat_quot(a4255, a4256, a4257)
                        }, ({
                            let a4258 = v2;
                            let a4259 = 1u64;
                            nat_sub(a4258, a4259)
                        }, {
                            let a4263 = f619({
                                let a4260 = v1;
                                let a4261 = 16u64;
                                let a4262 = 0u64;
                                nat_rem(a4260, a4261, a4262)
                            });
                            let a4264 = v3;
                            append_bytes(a4263, a4264)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(v0: Bytes, v1: u64) -> R<Bytes> {
    let a4272 = f32(v0.clone(), {
        let a4270 = v1;
        let a4271 = 2u64;
        nat_add(a4270, a4271)?
    }, 2u64);
    let a4273 = Bytes::lit(&[0x30, 0x30]);
    if equal(a4272, a4273) {
        let m4276 = f620(f32(v0, {
            let a4274 = v1;
            let a4275 = 4u64;
            nat_add(a4274, a4275)?
        }, 2u64))?;
        match m4276 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f333(v2.clone(), 0u64)?;
                let a4277 = v3;
                let a4278 = 32u64;
                if nat_lt(a4277, a4278) {
                    if f621(v3) {
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

pub fn f598(v0: u64) -> Bytes {
    let a4279 = v0;
    let a4280 = 34u64;
    if nat_eq(a4279, a4280) {
        Bytes::lit(&[0x22])
    } else {
        let a4281 = v0;
        let a4282 = 92u64;
        if nat_eq(a4281, a4282) {
            Bytes::lit(&[0x5c])
        } else {
            let a4283 = v0;
            let a4284 = 98u64;
            if nat_eq(a4283, a4284) {
                Bytes::lit(&[0x08])
            } else {
                let a4285 = v0;
                let a4286 = 102u64;
                if nat_eq(a4285, a4286) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a4287 = v0;
                    let a4288 = 110u64;
                    if nat_eq(a4287, a4288) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a4289 = v0;
                        let a4290 = 114u64;
                        if nat_eq(a4289, a4290) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a4291 = v0;
                            let a4292 = 116u64;
                            if nat_eq(a4291, a4292) {
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

pub fn f599(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m4295 = {
        let a4293 = v0;
        let a4294 = v1;
        index_bytes(a4293, a4294)
    };
    match m4295 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f622(v2)? {
                Ok(Some({
                    let a4296 = v1;
                    let a4297 = 1u64;
                    nat_add(a4296, a4297)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f600(v0: u8) -> R<u64> {
    let a4344 = {
        let v1: u8 = v0;
        let a4342 = {
            let a4318 = {
                let a4306 = {
                    let a4300 = {
                        let a4298 = v1;
                        let a4299 = 128u8;
                        fixed_u8::bit_and(a4298, a4299)
                    };
                    let a4301 = 128u8;
                    if equal(a4300, a4301) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4307 = {
                    let a4304 = {
                        let a4302 = v1;
                        let a4303 = 64u8;
                        fixed_u8::bit_and(a4302, a4303)
                    };
                    let a4305 = 64u8;
                    if equal(a4304, a4305) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4306, a4307)?
            };
            let a4319 = {
                let a4316 = {
                    let a4310 = {
                        let a4308 = v1;
                        let a4309 = 32u8;
                        fixed_u8::bit_and(a4308, a4309)
                    };
                    let a4311 = 32u8;
                    if equal(a4310, a4311) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4317 = {
                    let a4314 = {
                        let a4312 = v1;
                        let a4313 = 16u8;
                        fixed_u8::bit_and(a4312, a4313)
                    };
                    let a4315 = 16u8;
                    if equal(a4314, a4315) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4316, a4317)?
            };
            nat_add(a4318, a4319)?
        };
        let a4343 = {
            let a4340 = {
                let a4328 = {
                    let a4322 = {
                        let a4320 = v1;
                        let a4321 = 8u8;
                        fixed_u8::bit_and(a4320, a4321)
                    };
                    let a4323 = 8u8;
                    if equal(a4322, a4323) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4329 = {
                    let a4326 = {
                        let a4324 = v1;
                        let a4325 = 4u8;
                        fixed_u8::bit_and(a4324, a4325)
                    };
                    let a4327 = 4u8;
                    if equal(a4326, a4327) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4328, a4329)?
            };
            let a4341 = {
                let a4338 = {
                    let a4332 = {
                        let a4330 = v1;
                        let a4331 = 2u8;
                        fixed_u8::bit_and(a4330, a4331)
                    };
                    let a4333 = 2u8;
                    if equal(a4332, a4333) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4339 = {
                    let a4336 = {
                        let a4334 = v1;
                        let a4335 = 1u8;
                        fixed_u8::bit_and(a4334, a4335)
                    };
                    let a4337 = 1u8;
                    if equal(a4336, a4337) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4338, a4339)?
            };
            nat_add(a4340, a4341)?
        };
        nat_add(a4342, a4343)?
    };
    let a4345 = 48u64;
    Ok(nat_sub(a4344, a4345))
}

pub fn f601(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a4346 = v0.clone();
        length_bytes(a4346)
    };
    let a4347 = v1;
    let a4348 = v2;
    if if nat_lt(a4347, a4348) {
        let a4353 = f333(v0, {
            let a4351 = v2;
            let a4352 = {
                let a4349 = v1;
                let a4350 = 1u64;
                nat_add(a4349, a4350)?
            };
            nat_sub(a4351, a4352)
        })?;
        let a4354 = 48u64;
        nat_eq(a4353, a4354)
    } else {
        false
    } {
        Ok(Some({
            let a4355 = v1;
            let a4356 = 1u64;
            nat_add(a4355, a4356)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f602(v0: List<(Bytes, Adt19)>, v1: List<Adt19>) -> List<Bytes> {
    let m4357 = v0;
    match m4357.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f623(v1.clone(), {
                let (h4358, _) = v2.clone();
                h4358
            }) {
                List::cons({
                    let (h4359, _) = v2;
                    h4359
                }, f602(v3, v1))
            } else {
                f602(v3, v1)
            }
        }
    }
}

pub fn f603(v0: bool, v1: Bytes, v2: List<Adt19>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4360 = v2;
    match m4360.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m4361 = f624(v4.clone())?;
            match m4361 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m4362 = f512(v0, v6.clone())?;
                    match m4362 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f88(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4365 = f625(v4, {
                                    let a4363 = v1.clone();
                                    let a4364 = f260(v6)?;
                                    append_bytes(a4363, a4364)
                                })?;
                                match m4365 {
                                    Ok(v9) => {
                                        let m4366 = f603(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m4366 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h4367, _) = v9;
                                                    h4367
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4368, _) = v11;
                                                    h4368
                                                }), f433({
                                                    let (_, h4369) = v9;
                                                    h4369
                                                }, {
                                                    let (_, h4370) = v11;
                                                    h4370
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

pub fn f604(v0: List<Adt19>) -> R<List<Bytes>> {
    let m4371 = v0;
    match m4371.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m4372 = f624(v1)?;
            match m4372 {
                None => {
                    f604(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f604(v2)?))
                }
            }
        }
    }
}

pub fn f605(v0: Bytes, v1: List<Adt19>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f626(f416(f627(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f606(v0: Adt19, v1: Bytes) -> R<bool> {
    let m4373 = f264(Some(v0));
    match m4373 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m4374 = f557(v2)?;
            match m4374 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f37(v3, v1))
                }
            }
        }
    }
}

pub fn f607(v0: bool, v1: Bytes, v2: List<(Bytes, Adt19)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4375 = v2;
    match m4375.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m4377 = f512(v0, {
                let (h4376, _) = v5.clone();
                h4376
            })?;
            match m4377 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h4378, _) = v5;
                        h4378
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f88(v7.clone(), f470(v0, f608(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m4383 = f564({
                            let (_, h4379) = v5.clone();
                            h4379
                        }, {
                            let a4381 = v1.clone();
                            let a4382 = f260({
                                let (h4380, _) = v5.clone();
                                h4380
                            })?;
                            append_bytes(a4381, a4382)
                        })?;
                        match m4383 {
                            Ok(v9) => {
                                let v10: Bytes = if f88({
                                    let (h4384, _) = v5;
                                    h4384
                                }, v3.clone()) {
                                    let (h4385, _) = v9;
                                    h4385
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h4386, _) = v9;
                                        h4386
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m4387 = f607(v0, v1, v6, v3, v4)?;
                                match m4387 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h4388, _) = v12;
                                            h4388
                                        }), f433({
                                            let (_, h4389) = v9;
                                            h4389
                                        }, {
                                            let (_, h4390) = v12;
                                            h4390
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

pub fn f608(v0: List<(Bytes, Adt19)>) -> List<Bytes> {
    let m4391 = v0;
    match m4391.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h4392, _) = v1;
                h4392
            }, f608(v2))
        }
    }
}

pub fn f609(v0: Adt19) -> Option<Adt19> {
    let m4393 = f158(f153(v0, f8(Str::lit("oneOf"))));
    match m4393.uncons() {
        None => {
            None::<Adt19>
        }
        Some((v1, v2)) => {
            let m4394 = v2;
            match m4394.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v3, v4)) => {
                    let m4395 = v4;
                    match m4395.uncons() {
                        None => {
                            if f628(v3.clone()) {
                                Some(v1)
                            } else {
                                if f628(v1) {
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

pub fn f610(v0: Bytes, v1: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4396 = v1;
    match m4396 {
        Ok(v3) => {
            f560(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h4397, _) = v3;
                h4397
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h4398) = v3;
                h4398
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v2)
        }
    }
}

pub fn f611(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4399 = f609(v0.clone());
    match m4399 {
        None => {
            if f37(f156(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4400 = f153(v0, f8(Str::lit("items")));
                match m4400 {
                    None => {
                        Ok(f610(f8(Str::lit("List")), f612()))
                    }
                    Some(v3) => {
                        Ok(f610(f8(Str::lit("List")), f625(v3, {
                            let a4401 = v1;
                            let a4402 = f8(Str::lit("Item"));
                            append_bytes(a4401, a4402)
                        })?))
                    }
                }
            } else {
                f625(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f610(f8(Str::lit("Option")), f625(v2, v1)?))
        }
    }
}

pub fn f612() -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((f482(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f613(p4406: List<(Bytes, Adt19)>, v1: List<Bytes>) -> bool {
    let m4405 = p4406;
    {
        let mut m4405 = m4405;
        loop {
            let step = {
                let v0 = m4405;
                let m4403 = v0;
                match m4403.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f88({
                            let (h4404, _) = v2;
                            h4404
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt19)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt19)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(p4410: List<Adt19>, v1: Bytes, p4411: List<Bytes>) -> bool {
    let m4409 = (p4410, p4411);
    {
        let mut m4409 = m4409;
        loop {
            let step = {
                let (v0, v2) = m4409;
                let m4407 = v0;
                match m4407.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m4408 = f629(v3, v1.clone());
                        match m4408 {
                            None => {
                                Err::<(List<Adt19>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f88(v5.clone(), v2.clone()) {
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
                Ok(next) => { m4409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615(p4414: List<Adt19>, p4415: List<Bytes>) -> bool {
    let m4413 = (p4414, p4415);
    {
        let mut m4413 = m4413;
        loop {
            let step = {
                let (v0, v1) = m4413;
                let m4412 = v0;
                match m4412.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f156(v2, f8(Str::lit("type")));
                        if !f37(v4.clone(), Bytes::lit(&[])) {
                            if !f88(v4.clone(), v1.clone()) {
                                if !if if f37(v4.clone(), f8(Str::lit("integer"))) {
                                    f88(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f37(v4.clone(), f8(Str::lit("number"))) {
                                        f88(f8(Str::lit("integer")), v1.clone())
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
                Ok(next) => { m4413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f617(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f529({
        let (_, h4416) = {
            let (h4417, _) = f630(Fn22::F631, {
                let a4419 = {
                    let a4418 = v0.clone();
                    length_list(a4418)
                };
                let a4420 = 1u64;
                nat_add(a4419, a4420)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4417
        };
        h4416
    }))
}

pub fn f618(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a4425 = {
            let a4423 = {
                let a4421 = v0.clone();
                length_list(a4421)
            };
            let a4424 = {
                let a4422 = v1.clone();
                length_list(a4422)
            };
            nat_add(a4423, a4424)?
        };
        let a4426 = 1u64;
        nat_add(a4425, a4426)?
    };
    Ok(f383({
        let (_, h4427) = {
            let (_, h4428) = {
                let (h4429, _) = f632(Fn23::F633, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h4429
            };
            h4428
        };
        h4427
    }))
}

pub fn f619(v0: u64) -> Bytes {
    f32({
        let a4430 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4430)
    }, v0, 1u64)
}

pub fn f620(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a4431 = v0.clone();
        length_bytes(a4431)
    };
    let a4435 = {
        let a4432 = v1;
        let a4433 = 2u64;
        let a4434 = 0u64;
        nat_rem(a4432, a4433, a4434)
    };
    let a4436 = 0u64;
    if nat_eq(a4435, a4436) {
        let (_, h4437) = {
            let (h4438, _) = f634(Fn24::F635(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h4438
        };
        Ok(h4437)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f621(v0: u64) -> bool {
    let a4439 = v0;
    let a4440 = 8u64;
    if nat_eq(a4439, a4440) {
        true
    } else {
        let a4441 = v0;
        let a4442 = 9u64;
        if nat_eq(a4441, a4442) {
            true
        } else {
            let a4443 = v0;
            let a4444 = 10u64;
            if nat_eq(a4443, a4444) {
                true
            } else {
                let a4445 = v0;
                let a4446 = 12u64;
                if nat_eq(a4445, a4446) {
                    true
                } else {
                    let a4447 = v0;
                    let a4448 = 13u64;
                    nat_eq(a4447, a4448)
                }
            }
        }
    }
}

pub fn f622(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4493 = {
            let a4469 = {
                let a4457 = {
                    let a4451 = {
                        let a4449 = v1;
                        let a4450 = 128u8;
                        fixed_u8::bit_and(a4449, a4450)
                    };
                    let a4452 = 128u8;
                    if equal(a4451, a4452) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4458 = {
                    let a4455 = {
                        let a4453 = v1;
                        let a4454 = 64u8;
                        fixed_u8::bit_and(a4453, a4454)
                    };
                    let a4456 = 64u8;
                    if equal(a4455, a4456) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4457, a4458)?
            };
            let a4470 = {
                let a4467 = {
                    let a4461 = {
                        let a4459 = v1;
                        let a4460 = 32u8;
                        fixed_u8::bit_and(a4459, a4460)
                    };
                    let a4462 = 32u8;
                    if equal(a4461, a4462) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4468 = {
                    let a4465 = {
                        let a4463 = v1;
                        let a4464 = 16u8;
                        fixed_u8::bit_and(a4463, a4464)
                    };
                    let a4466 = 16u8;
                    if equal(a4465, a4466) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4467, a4468)?
            };
            nat_add(a4469, a4470)?
        };
        let a4494 = {
            let a4491 = {
                let a4479 = {
                    let a4473 = {
                        let a4471 = v1;
                        let a4472 = 8u8;
                        fixed_u8::bit_and(a4471, a4472)
                    };
                    let a4474 = 8u8;
                    if equal(a4473, a4474) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4480 = {
                    let a4477 = {
                        let a4475 = v1;
                        let a4476 = 4u8;
                        fixed_u8::bit_and(a4475, a4476)
                    };
                    let a4478 = 4u8;
                    if equal(a4477, a4478) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4479, a4480)?
            };
            let a4492 = {
                let a4489 = {
                    let a4483 = {
                        let a4481 = v1;
                        let a4482 = 2u8;
                        fixed_u8::bit_and(a4481, a4482)
                    };
                    let a4484 = 2u8;
                    if equal(a4483, a4484) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4490 = {
                    let a4487 = {
                        let a4485 = v1;
                        let a4486 = 1u8;
                        fixed_u8::bit_and(a4485, a4486)
                    };
                    let a4488 = 1u8;
                    if equal(a4487, a4488) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4489, a4490)?
            };
            nat_add(a4491, a4492)?
        };
        nat_add(a4493, a4494)?
    };
    let a4495 = 48u64;
    let a4496 = v2;
    if nat_le(a4495, a4496) {
        let a4497 = v2;
        let a4498 = 57u64;
        Ok(nat_le(a4497, a4498))
    } else {
        Ok(false)
    }
}

pub fn f623(p4502: List<Adt19>, v1: Bytes) -> bool {
    let m4501 = p4502;
    {
        let mut m4501 = m4501;
        loop {
            let step = {
                let v0 = m4501;
                let m4499 = v0;
                match m4499.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m4500 = f629(v2, v1.clone());
                        if match m4500 {
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
                Ok(next) => { m4501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f624(v0: Adt19) -> R<Option<Bytes>> {
    let m4503 = f264(f153(v0.clone(), f8(Str::lit("$ref"))));
    match m4503 {
        None => {
            Ok(f264(f153(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f557(v1)
        }
    }
}

pub fn f625(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f636(f527(v0), v1)
}

pub fn f626(v0: bool, v1: Bytes, v2: List<Adt19>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4504 = v2;
    match m4504.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m4505 = f629(v5.clone(), v3.clone());
            match m4505 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m4506 = f512(v0, v7.clone())?;
                    match m4506 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f88(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4509 = f563({
                                    let a4507 = v1.clone();
                                    let a4508 = f260(v7)?;
                                    append_bytes(a4507, a4508)
                                }, f481(f429(f153(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f157(f158(f153(v5, f8(Str::lit("required"))))), true)?;
                                match m4509 {
                                    Ok(v10) => {
                                        let m4510 = f626(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m4510 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f221({
                                                    let (h4511, _) = v10;
                                                    h4511
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4512, _) = v12;
                                                    h4512
                                                }), f433({
                                                    let (_, h4513) = v10;
                                                    h4513
                                                }, {
                                                    let (_, h4514) = v12;
                                                    h4514
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

pub fn f627(v0: List<Adt19>, v1: Bytes) -> List<Bytes> {
    let m4515 = v0;
    match m4515.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4516 = f629(v2, v1.clone());
            match m4516 {
                None => {
                    f627(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f627(v3, v1))
                }
            }
        }
    }
}

pub fn f628(v0: Adt19) -> bool {
    f37(f156(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f629(v0: Adt19, v1: Bytes) -> Option<Bytes> {
    if f88(v1.clone(), f157(f158(f153(v0.clone(), f8(Str::lit("required")))))) {
        let m4517 = f153(v0, f8(Str::lit("properties")));
        match m4517 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4518 = f153(v2, v1);
                match m4518 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f264(f153(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f630(v0: Fn22, p4523: u64, p4524: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4522 = (p4523, p4524);
    {
        let mut m4522 = m4522;
        loop {
            let step = {
                let (v1, v2) = m4522;
                let m4519 = v1;
                if m4519 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4519 - 1;
                    let m4521 = {
                        let c4520: Fn22 = v0.clone();
                        c4520.apply(v2.clone())?
                    };
                    match m4521 {
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
                Ok(next) => { m4522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f637(v0)
}

pub fn f632(v0: Fn23, p4529: u64, p4530: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4528 = (p4529, p4530);
    {
        let mut m4528 = m4528;
        loop {
            let step = {
                let (v1, v2) = m4528;
                let m4525 = v1;
                if m4525 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4525 - 1;
                    let m4527 = {
                        let c4526: Fn23 = v0.clone();
                        c4526.apply(v2.clone())
                    };
                    match m4527 {
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
                Ok(next) => { m4528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f633(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f638(v0)
}

pub fn f634(v0: Fn24, p4535: u64, p4536: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4534 = (p4535, p4536);
    {
        let mut m4534 = m4534;
        loop {
            let step = {
                let (v1, v2) = m4534;
                let m4531 = v1;
                if m4531 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4531 - 1;
                    let m4533 = {
                        let c4532: Fn24 = v0.clone();
                        c4532.apply(v2.clone())?
                    };
                    match m4533 {
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
                Ok(next) => { m4534 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f635(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f639(v0, v1)
}

pub fn f636(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4537 = v0.clone();
    match m4537 {
        Adt19::C0(v2) => {
            if v2 {
                Ok(f612())
            } else {
                Ok(f559(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt19::C1(_, _) => {
            Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C2(_, _, _) => {
            Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C3(_) => {
            Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C4(_) => {
            Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C5(_) => {
            let m4538 = f640(f479(v0.clone()));
            match m4538 {
                None => {
                    let m4539 = f264(f153(v0.clone(), f8(Str::lit("$ref"))));
                    match m4539 {
                        None => {
                            let m4541 = f153(v0.clone(), f8(Str::lit("allOf")));
                            match m4541 {
                                None => {
                                    let m4546 = f153(v0.clone(), f8(Str::lit("enum")));
                                    match m4546 {
                                        None => {
                                            let m4547 = f153(v0.clone(), f8(Str::lit("const")));
                                            match m4547 {
                                                None => {
                                                    let m4549 = f153(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4549 {
                                                        None => {
                                                            let v30: Bytes = f156(v0.clone(), f8(Str::lit("type")));
                                                            if if f37(v30.clone(), Bytes::lit(&[])) {
                                                                !f641(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f612())
                                                            } else {
                                                                if f37(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f560(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f37(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f560(f642(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f37(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f560(f510(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f37(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f560(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f37(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4550 = f429(f153(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4550.uncons() {
                                                                                        None => {
                                                                                            let m4551 = f643(v0);
                                                                                            match m4551 {
                                                                                                None => {
                                                                                                    Ok(f610(f8(Str::lit("List")), f644(f612())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f610(f8(Str::lit("List")), f644(f645(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f560(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f559(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f560(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m4548 = f264(Some(v27));
                                                    match m4548 {
                                                        None => {
                                                            Ok(f559(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f560(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f560(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m4542 = f265(v19);
                                    match m4542.uncons() {
                                        None => {
                                            Ok(f559(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4543 = v21;
                                            match m4543.uncons() {
                                                None => {
                                                    let m4544 = f264(f153(v20, f8(Str::lit("$ref"))));
                                                    match m4544 {
                                                        None => {
                                                            Ok(f559(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4545 = f557(v24)?;
                                                            match m4545 {
                                                                None => {
                                                                    Ok(f559(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f560(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f559(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m4540 = f557(v17)?;
                            match m4540 {
                                None => {
                                    Ok(f559(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f560(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f559(if f37(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt19::C6 => {
            Ok(f612())
        }
        Adt19::C7(_, _) => {
            Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C8(_, _, _) => {
            Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f637(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4552, _) = v0;
        h4552
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4553) = v0;
        h4553
    };
    let m4554 = v1;
    match m4554.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4555 = v4;
            match m4555.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f646(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f638(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4556, _) = v0.clone();
        h4556
    };
    let v2: List<(Bytes, ())> = {
        let (h4557, _) = {
            let (_, h4558) = v0.clone();
            h4558
        };
        h4557
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4559) = {
            let (_, h4560) = v0;
            h4560
        };
        h4559
    };
    let m4561 = v1.clone();
    match m4561.uncons() {
        None => {
            let m4562 = v2;
            match m4562.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4563 = v2.clone();
            match m4563.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f647({
                        let (h4564, _) = v8.clone();
                        h4564
                    }, {
                        let (h4565, _) = v6.clone();
                        h4565
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

pub fn f639(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4566, _) = v1;
        h4566
    };
    let m4568 = {
        let (_, h4567) = v1;
        h4567
    };
    match m4568 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4571 = {
                let a4569 = v0.clone();
                let a4570 = v2;
                index_bytes(a4569, a4570)
            };
            match m4571 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4576 = {
                        let a4574 = v0;
                        let a4575 = {
                            let a4572 = v2;
                            let a4573 = 1u64;
                            nat_add(a4572, a4573)?
                        };
                        index_bytes(a4574, a4575)
                    };
                    match m4576 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f648(v4)? {
                                f648(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4577 = v2;
                                    let a4578 = 2u64;
                                    nat_add(a4577, a4578)?
                                }, Some({
                                    let a4583 = v3;
                                    let a4584 = f337(f649(), {
                                        let a4581 = {
                                            let a4579 = f650(v4)?;
                                            let a4580 = 16u64;
                                            nat_mul(a4579, a4580)?
                                        };
                                        let a4582 = f650(v5)?;
                                        nat_add(a4581, a4582)?
                                    });
                                    append_bytes(a4583, a4584)
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

pub fn f640(p4589: List<(Bytes, Adt19)>) -> Option<Bytes> {
    let m4588 = p4589;
    {
        let mut m4588 = m4588;
        loop {
            let step = {
                let v0 = m4588;
                let m4585 = v0;
                match m4585.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f88({
                            let (h4586, _) = v1.clone();
                            h4586
                        }, f651()) {
                            Err::<List<(Bytes, Adt19)>, Option<Bytes>>(Some({
                                let (h4587, _) = v1;
                                h4587
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4588 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f641(v0: Adt19) -> bool {
    if f567(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f567(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f567(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f567(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f567(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f567(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f642(v0: Adt19) -> Bytes {
    let m4590 = f652(f153(v0, f8(Str::lit("minimum"))));
    match m4590 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f643(v0: Adt19) -> Option<Adt19> {
    let m4591 = f153(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4591 {
        None => {
            let m4592 = f429(f153(v0, f8(Str::lit("patternProperties"))));
            match m4592.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v2, v3)) => {
                    let m4593 = v3;
                    match m4593.uncons() {
                        None => {
                            Some({
                                let (_, h4594) = v2;
                                h4594
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

pub fn f644(v0: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4595 = v0;
    match m4595 {
        Ok(v2) => {
            f560(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4596, _) = v2;
                h4596
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4597) = v2;
                h4597
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v1)
        }
    }
}

pub fn f645(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f653(16u64, v0, v1)
}

pub fn f646(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4602 = {
            let a4600 = {
                let a4598 = v0.clone();
                length_list(a4598)
            };
            let a4601 = {
                let a4599 = v1.clone();
                length_list(a4599)
            };
            nat_add(a4600, a4601)?
        };
        let a4603 = 1u64;
        nat_add(a4602, a4603)?
    };
    Ok(f654({
        let (_, h4604) = {
            let (_, h4605) = {
                let (h4606, _) = f655(Fn25::F656, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4606
            };
            h4605
        };
        h4604
    }))
}

pub fn f647(v0: Bytes, v1: Bytes) -> bool {
    let a4611 = {
        let a4607 = v0;
        let a4608 = v1;
        compare_bytes(a4607, a4608)
    };
    let a4612 = {
        let a4609 = Bytes::lit(&[]);
        let a4610 = Bytes::lit(&[0x01]);
        compare_bytes(a4609, a4610)
    };
    equal(a4611, a4612)
}

pub fn f648(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4657 = {
            let a4633 = {
                let a4621 = {
                    let a4615 = {
                        let a4613 = v1;
                        let a4614 = 128u8;
                        fixed_u8::bit_and(a4613, a4614)
                    };
                    let a4616 = 128u8;
                    if equal(a4615, a4616) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4622 = {
                    let a4619 = {
                        let a4617 = v1;
                        let a4618 = 64u8;
                        fixed_u8::bit_and(a4617, a4618)
                    };
                    let a4620 = 64u8;
                    if equal(a4619, a4620) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4621, a4622)?
            };
            let a4634 = {
                let a4631 = {
                    let a4625 = {
                        let a4623 = v1;
                        let a4624 = 32u8;
                        fixed_u8::bit_and(a4623, a4624)
                    };
                    let a4626 = 32u8;
                    if equal(a4625, a4626) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4632 = {
                    let a4629 = {
                        let a4627 = v1;
                        let a4628 = 16u8;
                        fixed_u8::bit_and(a4627, a4628)
                    };
                    let a4630 = 16u8;
                    if equal(a4629, a4630) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4631, a4632)?
            };
            nat_add(a4633, a4634)?
        };
        let a4658 = {
            let a4655 = {
                let a4643 = {
                    let a4637 = {
                        let a4635 = v1;
                        let a4636 = 8u8;
                        fixed_u8::bit_and(a4635, a4636)
                    };
                    let a4638 = 8u8;
                    if equal(a4637, a4638) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4644 = {
                    let a4641 = {
                        let a4639 = v1;
                        let a4640 = 4u8;
                        fixed_u8::bit_and(a4639, a4640)
                    };
                    let a4642 = 4u8;
                    if equal(a4641, a4642) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4643, a4644)?
            };
            let a4656 = {
                let a4653 = {
                    let a4647 = {
                        let a4645 = v1;
                        let a4646 = 2u8;
                        fixed_u8::bit_and(a4645, a4646)
                    };
                    let a4648 = 2u8;
                    if equal(a4647, a4648) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4654 = {
                    let a4651 = {
                        let a4649 = v1;
                        let a4650 = 1u8;
                        fixed_u8::bit_and(a4649, a4650)
                    };
                    let a4652 = 1u8;
                    if equal(a4651, a4652) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4653, a4654)?
            };
            nat_add(a4655, a4656)?
        };
        nat_add(a4657, a4658)?
    };
    let a4659 = 48u64;
    let a4660 = v2;
    if if nat_le(a4659, a4660) {
        let a4661 = v2;
        let a4662 = 57u64;
        nat_le(a4661, a4662)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4663 = 97u64;
        let a4664 = v2;
        if nat_le(a4663, a4664) {
            let a4665 = v2;
            let a4666 = 102u64;
            Ok(nat_le(a4665, a4666))
        } else {
            Ok(false)
        }
    }
}

pub fn f649() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f650(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4711 = {
            let a4687 = {
                let a4675 = {
                    let a4669 = {
                        let a4667 = v1;
                        let a4668 = 128u8;
                        fixed_u8::bit_and(a4667, a4668)
                    };
                    let a4670 = 128u8;
                    if equal(a4669, a4670) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4676 = {
                    let a4673 = {
                        let a4671 = v1;
                        let a4672 = 64u8;
                        fixed_u8::bit_and(a4671, a4672)
                    };
                    let a4674 = 64u8;
                    if equal(a4673, a4674) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4675, a4676)?
            };
            let a4688 = {
                let a4685 = {
                    let a4679 = {
                        let a4677 = v1;
                        let a4678 = 32u8;
                        fixed_u8::bit_and(a4677, a4678)
                    };
                    let a4680 = 32u8;
                    if equal(a4679, a4680) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4686 = {
                    let a4683 = {
                        let a4681 = v1;
                        let a4682 = 16u8;
                        fixed_u8::bit_and(a4681, a4682)
                    };
                    let a4684 = 16u8;
                    if equal(a4683, a4684) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4685, a4686)?
            };
            nat_add(a4687, a4688)?
        };
        let a4712 = {
            let a4709 = {
                let a4697 = {
                    let a4691 = {
                        let a4689 = v1;
                        let a4690 = 8u8;
                        fixed_u8::bit_and(a4689, a4690)
                    };
                    let a4692 = 8u8;
                    if equal(a4691, a4692) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4698 = {
                    let a4695 = {
                        let a4693 = v1;
                        let a4694 = 4u8;
                        fixed_u8::bit_and(a4693, a4694)
                    };
                    let a4696 = 4u8;
                    if equal(a4695, a4696) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4697, a4698)?
            };
            let a4710 = {
                let a4707 = {
                    let a4701 = {
                        let a4699 = v1;
                        let a4700 = 2u8;
                        fixed_u8::bit_and(a4699, a4700)
                    };
                    let a4702 = 2u8;
                    if equal(a4701, a4702) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4708 = {
                    let a4705 = {
                        let a4703 = v1;
                        let a4704 = 1u8;
                        fixed_u8::bit_and(a4703, a4704)
                    };
                    let a4706 = 1u8;
                    if equal(a4705, a4706) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4707, a4708)?
            };
            nat_add(a4709, a4710)?
        };
        nat_add(a4711, a4712)?
    };
    let a4713 = v2;
    let a4714 = 57u64;
    if nat_le(a4713, a4714) {
        let a4715 = v2;
        let a4716 = 48u64;
        Ok(nat_sub(a4715, a4716))
    } else {
        let a4717 = v2;
        let a4718 = 87u64;
        Ok(nat_sub(a4717, a4718))
    }
}

pub fn f651() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f652(v0: Option<Adt19>) -> Option<u64> {
    let m4719 = v0;
    match m4719 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4720 = v1;
            match m4720 {
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

pub fn f653(v0: u64, v1: Adt19, v2: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4721 = v0;
    if m4721 == 0 {
        Ok(f559(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4721 - 1;
        let m4722 = v1.clone();
        match m4722 {
            Adt19::C0(v4) => {
                if v4 {
                    Ok(f612())
                } else {
                    Ok(f559(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt19::C1(_, _) => {
                Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C2(_, _, _) => {
                Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C3(_) => {
                Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C4(_) => {
                Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C5(_) => {
                let m4723 = f264(f153(v1.clone(), f8(Str::lit("$ref"))));
                match m4723 {
                    None => {
                        if if f567(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f567(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f567(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f560(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f156(v1.clone(), f8(Str::lit("type")));
                            if if f37(v20.clone(), Bytes::lit(&[])) {
                                !f641(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f612())
                            } else {
                                if f37(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f560(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f37(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f560(f642(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f37(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f560(f510(), List::<Adt25>::nil()))
                                        } else {
                                            if f37(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f560(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f37(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4725 = f153(v1, f8(Str::lit("items")));
                                                    match m4725 {
                                                        None => {
                                                            Ok(f610(f8(Str::lit("List")), f612()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f610(f8(Str::lit("List")), f653(v3, v21, {
                                                                let a4726 = v2;
                                                                let a4727 = f8(Str::lit("Item"));
                                                                append_bytes(a4726, a4727)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f37(v20.clone(), f8(Str::lit("object"))) {
                                                        let m4728 = f643(v1);
                                                        match m4728 {
                                                            None => {
                                                                Ok(f610(f8(Str::lit("List")), f644(f612())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f610(f8(Str::lit("List")), f644(f653(v3, v22, {
                                                                    let a4729 = v2;
                                                                    let a4730 = f8(Str::lit("Value"));
                                                                    append_bytes(a4729, a4730)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f559(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m4724 = f557(v18)?;
                        match m4724 {
                            None => {
                                Ok(f559(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f560(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt19::C6 => {
                Ok(f612())
            }
            Adt19::C7(_, _) => {
                Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C8(_, _, _) => {
                Ok(f559(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f654(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f439(Fn10::F657, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f655(v0: Fn25, p4735: u64, p4736: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4734 = (p4735, p4736);
    {
        let mut m4734 = m4734;
        loop {
            let step = {
                let (v1, v2) = m4734;
                let m4731 = v1;
                if m4731 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4731 - 1;
                    let m4733 = {
                        let c4732: Fn25 = v0.clone();
                        c4732.apply(v2.clone())
                    };
                    match m4733 {
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
                Ok(next) => { m4734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f656(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f658(v0)
}

pub fn f657(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f658(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4737, _) = v0.clone();
        h4737
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4738, _) = {
            let (_, h4739) = v0.clone();
            h4739
        };
        h4738
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4740) = {
            let (_, h4741) = v0;
            h4741
        };
        h4740
    };
    let m4742 = v1.clone();
    match m4742.uncons() {
        None => {
            let m4743 = v2;
            match m4743.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4744 = v2.clone();
            match m4744.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f647({
                        let (h4745, _) = v8.clone();
                        h4745
                    }, {
                        let (h4746, _) = v6.clone();
                        h4746
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
