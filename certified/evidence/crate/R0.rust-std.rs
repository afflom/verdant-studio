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
    C0(Bytes, Adt14),
    C1(Bytes),
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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
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
    F155(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F114(k0, k1) => f114(*k0, k1.clone(), p0),
            Fn0::F155(k0) => f155(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F134,
    F169,
    F183,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F134 => f134(p0, p1),
            Fn1::F169 => f169(p0, p1),
            Fn1::F183 => f183(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F151(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F151(k0) => f151(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F172(Bytes, u64, Bytes, u64),
    F245(Bytes),
    F246(u64, Bytes),
    F297(Bytes),
    F340(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F172(k0, k1, k2, k3) => f172(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F245(k0) => Ok(f245(k0.clone(), p0)),
            Fn3::F246(k0, k1) => f246(*k0, k1.clone(), p0),
            Fn3::F297(k0) => f297(k0.clone(), p0),
            Fn3::F340(k0) => f340(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F197(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt21)) -> R<(u64, Adt21)> {
        match self {
            Fn4::F197(k0, k1, k2) => f197(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F232,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F232 => f232(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F235,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F235 => f235(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F237,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F237 => f237(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F240,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F240 => f240(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F258(Bytes),
    F305(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F258(k0) => f258(k0.clone(), p0),
            Fn9::F305(k0, k1) => Ok(f305(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F260,
    F370,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F260 => f260(p0, p1),
            Fn10::F370 => f370(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F263,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F263 => f263(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F265,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F265 => f265(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F268,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F268 => f268(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F270,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn14::F270 => f270(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F275(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F275(k0) => f275(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F300(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F300(k0) => f300(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F309,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F309 => f309(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F311,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F311 => f311(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F313,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F313 => f313(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F321,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F321 => f321(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F332,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F332 => f332(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F354,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F354 => f354(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F356,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F356 => f356(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F358(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F358(k0) => f358(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F369,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F369 => f369(p0),
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
                }), Adt9::C0(f4(f5())?)))
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
                        f12(v3)
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
                                    })?, {
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
                    Ok((f14(v0, 1u64, v2.clone()), Adt9::C0(f4(v2)?)))
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
                    Ok((f14(v0, 2u64, v5.clone()), Adt9::C2(f4(v5)?)))
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

pub fn f4(v0: Bytes) -> R<Str> {
    let a78 = f21(v0.clone(), 0u64, {
        let a77 = v0;
        length_bytes(a77)
    })?;
    let a79 = Str::lit("");
    Ok(join(a78, a79))
}

pub fn f5() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.err"))
}

pub fn f6(v0: Adt0, v1: u8, v2: Bytes, v3: Bytes) -> (Adt0, Adt9) {
    (f14(v0, 9u64, Bytes::lit(&[])), Adt9::C13(v1, v2, v3))
}

pub fn f7(v0: List<Bytes>) -> Bytes {
    let m80 = v0;
    match m80.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, v2)) => {
            let a81 = v1;
            let a82 = f7(v2);
            append_bytes(a81, a82)
        }
    }
}

pub fn f8(v0: Str) -> Bytes {
    let a83 = v0;
    utf8_encode(a83)
}

pub fn f9(v0: Adt0, v1: Option<Bytes>) -> R<(Adt0, Adt9)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h84, _, _, _, _, _, _, _) => {
            h84
        }
    };
    f2(f22(v0.clone(), Adt1::C0(List::cons((match v0 {
        Adt0::C0(_, _, _, _, h85, _, _, _, _, _) => {
            h85
        }
    }, v1), match v2.clone() {
        Adt1::C0(h86, _, _) => {
            h86
        }
    }), match v2.clone() {
        Adt1::C0(_, h87, _) => {
            h87
        }
    }, match v2 {
        Adt1::C0(_, _, h88) => {
            h88
        }
    })))
}

pub fn f10(v0: Adt0, v1: List<Bytes>) -> R<(Adt0, Adt9)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h89, _, _, _, _, _, _, _) => {
            h89
        }
    };
    f2(f22(v0.clone(), Adt1::C0(match v2.clone() {
        Adt1::C0(h90, _, _) => {
            h90
        }
    }, List::cons((match v0 {
        Adt0::C0(_, _, _, _, h91, _, _, _, _, _) => {
            h91
        }
    }, v1), match v2.clone() {
        Adt1::C0(_, h92, _) => {
            h92
        }
    }), match v2 {
        Adt1::C0(_, _, h93) => {
            h93
        }
    })))
}

pub fn f11(v0: Adt0, v1: Bytes) -> (Adt0, Adt9) {
    f6(v0.clone(), 1u8, Bytes::lit(&[]), f7(List::cons(v1, List::cons(f8(Str::lit(": ")), List::cons(match v0 {
        Adt0::C0(_, _, _, _, h94, _, _, _, _, _) => {
            h94
        }
    }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f12(v0: Adt0) -> R<(Adt0, Adt9)> {
    Ok((f14(v0.clone(), 4u64, match v0.clone() {
        Adt0::C0(_, _, _, _, h95, _, _, _, _, _) => {
            h95
        }
    }), Adt9::C10(Adt10::C0(f4(match v0.clone() {
        Adt0::C0(_, _, _, _, _, h96, _, _, _, _) => {
            h96
        }
    })?, f23(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h97, _, _, _) => {
            h97
        }
    })?, Str::lit("."), f24(match v0 {
        Adt0::C0(_, h98, _, _, _, _, _, _, _, _) => {
            h98
        }
    })?, f4(f19())?, f4(f5())?))))
}

pub fn f13(v0: Adt0) -> R<(Adt0, Adt9)> {
    let m100 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h99, _) => {
            h99
        }
    };
    match m100.uncons() {
        None => {
            Ok(f6(v0.clone(), 0u8, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, _, _, h101) => {
                    h101
                }
            }, Bytes::lit(&[])))
        }
        Some((v1, _)) => {
            Ok((f14(v0, 5u64, {
                let (h102, _) = v1.clone();
                h102
            }), Adt9::C4(f4(f25({
                let (h103, _) = v1;
                h103
            })?)?)))
        }
    }
}

pub fn f14(v0: Adt0, v1: u64, v2: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h104, _, _, _, _, _, _, _, _, _) => {
            h104
        }
    }, match v0.clone() {
        Adt0::C0(_, h105, _, _, _, _, _, _, _, _) => {
            h105
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h106, _, _, _, _, _, _, _) => {
            h106
        }
    }, v1, v2, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h107, _, _, _, _) => {
            h107
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h108, _, _, _) => {
            h108
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h109, _, _) => {
            h109
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h110, _) => {
            h110
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h111) => {
            h111
        }
    })
}

pub fn f15(v0: Adt0) -> (Adt0, Adt9) {
    (v0.clone(), Adt9::C11(List::cons(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h112, _, _) => {
            h112
        }
    }, List::<u64>::nil()), 600000u64))
}

pub fn f16(p119: List<(u64, Option<i32>)>, v1: u64) -> Option<Option<i32>> {
    let m118 = p119;
    {
        let mut m118 = m118;
        loop {
            let step = {
                let v0 = m118;
                let m113 = v0;
                match m113.uncons() {
                    None => {
                        Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(None::<Option<i32>>)
                    }
                    Some((v2, v3)) => {
                        let a115 = {
                            let (h114, _) = v2;
                            h114
                        };
                        let a116 = v1;
                        if nat_eq(a115, a116) {
                            Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(Some({
                                let (_, h117) = v2;
                                h117
                            }))
                        } else {
                            Ok::<List<(u64, Option<i32>)>, Option<Option<i32>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Option<i32>) -> bool {
    let m120 = v0;
    match m120 {
        None => {
            false
        }
        Some(v1) => {
            let a121 = v1;
            let a122 = 0i32;
            equal(a121, a122)
        }
    }
}

pub fn f18(v0: Adt0) -> R<(Adt0, Adt9)> {
    let v1: Adt1 = match v0.clone() {
        Adt0::C0(_, _, h123, _, _, _, _, _, _, _) => {
            h123
        }
    };
    f2(f22(v0.clone(), Adt1::C0(match v1.clone() {
        Adt1::C0(h124, _, _) => {
            h124
        }
    }, match v1.clone() {
        Adt1::C0(_, h125, _) => {
            h125
        }
    }, List::cons(match v0 {
        Adt0::C0(_, _, _, _, h126, _, _, _, _, _) => {
            h126
        }
    }, match v1 {
        Adt1::C0(_, _, h127) => {
            h127
        }
    }))))
}

pub fn f19() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f20(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m128 = v0;
    match m128.uncons() {
        None => {
            Ok(Adt11::C4(f26()))
        }
        Some((v2, v3)) => {
            if f27(v2.clone(), f8(Str::lit("check"))) {
                f28(v1)
            } else {
                if f27(v2.clone(), f8(Str::lit("next"))) {
                    f29(v1, v3)
                } else {
                    if f27(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f30(v1))
                    } else {
                        if f27(v2.clone(), f8(Str::lit("publish"))) {
                            f31(v1)
                        } else {
                            if f27(v2.clone(), f8(Str::lit("agenda"))) {
                                f32(v1)
                            } else {
                                let m129 = v3;
                                match m129.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f26()))
                                    }
                                    Some((v4, v5)) => {
                                        if f27(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f33(v1, v4)
                                        } else {
                                            if f27(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                f34(v1, v4)
                                            } else {
                                                if f27(v2.clone(), f8(Str::lit("admit"))) {
                                                    f35(v1, v4)
                                                } else {
                                                    if f27(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                        f36(v1, v4)
                                                    } else {
                                                        if f27(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                            f37(v1, v4)
                                                        } else {
                                                            if f27(v2, f8(Str::lit("propose-encoding"))) {
                                                                let m130 = v5;
                                                                match m130.uncons() {
                                                                    None => {
                                                                        Ok(Adt11::C4(f26()))
                                                                    }
                                                                    Some((v6, v7)) => {
                                                                        f38(v1, v4, v6, v7)
                                                                    }
                                                                }
                                                            } else {
                                                                Ok(Adt11::C4(f26()))
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

pub fn f21(v0: Bytes, v1: u64, v2: u64) -> R<List<Str>> {
    let m131 = v2;
    if m131 == 0 {
        Ok(List::<Str>::nil())
    } else {
        let v3 = m131 - 1;
        let m134 = {
            let a132 = v0.clone();
            let a133 = v1;
            index_bytes(a132, a133)
        };
        match m134 {
            None => {
                Ok(List::<Str>::nil())
            }
            Some(v4) => {
                Ok(List::cons(f39(v4)?, f21(v0, {
                    let a135 = v1;
                    let a136 = 1u64;
                    nat_add(a135, a136)?
                }, v3)?))
            }
        }
    }
}

pub fn f22(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h137, _, _, _, _, _, _, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt0::C0(_, h138, _, _, _, _, _, _, _, _) => {
            h138
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, h139, _, _, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h140, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h141, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h142, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h143, _) => {
            h143
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h144) => {
            h144
        }
    })
}

pub fn f23(v0: List<Bytes>) -> R<List<Str>> {
    let m145 = v0;
    match m145.uncons() {
        None => {
            Ok(List::<Str>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f4(v1)?, f23(v2)?))
        }
    }
}

pub fn f24(v0: List<(Bytes, Bytes)>) -> R<List<(Str, Str)>> {
    let m146 = v0;
    match m146.uncons() {
        None => {
            Ok(List::<(Str, Str)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f4({
                let (h147, _) = v1;
                h147
            })?, f4({
                let (_, h148) = v1;
                h148
            })?), f24(v2)?))
        }
    }
}

pub fn f25(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f40(v0.clone(), 47u8)?;
    let v2: Bytes = f41(v1);
    let a151 = {
        let a149 = v0.clone();
        length_bytes(a149)
    };
    let a152 = {
        let a150 = v2.clone();
        length_bytes(a150)
    };
    if nat_eq(a151, a152) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f42(v0.clone(), 0u64, {
            let a157 = {
                let a153 = v0;
                length_bytes(a153)
            };
            let a158 = {
                let a155 = {
                    let a154 = v2;
                    length_bytes(a154)
                };
                let a156 = 1u64;
                nat_add(a155, a156)?
            };
            nat_sub(a157, a158)
        }))
    }
}

pub fn f26() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | admit JSONL | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f27(v0: Bytes, v1: Bytes) -> bool {
    let a159 = v0;
    let a160 = v1;
    equal(a159, a160)
}

pub fn f28(v0: Adt1) -> R<Adt11> {
    let m161 = f43(v0.clone(), f44());
    match m161.uncons() {
        None => {
            let m162 = f45(v0.clone(), f44())?;
            match m162.uncons() {
                None => {
                    let m165 = f46(match v0.clone() {
                        Adt1::C0(_, h163, _) => {
                            h163
                        }
                    }, f47(f48(), f49(match v0.clone() {
                        Adt1::C0(_, h164, _) => {
                            h164
                        }
                    }, false)));
                    match m165.uncons() {
                        None => {
                            let m167 = f43(v0.clone(), f49(match v0.clone() {
                                Adt1::C0(_, h166, _) => {
                                    h166
                                }
                            }, true));
                            match m167.uncons() {
                                None => {
                                    let m169 = f50(v0.clone(), f49(match v0.clone() {
                                        Adt1::C0(_, h168, _) => {
                                            h168
                                        }
                                    }, true))?;
                                    match m169 {
                                        None => {
                                            let v10: (bool, Bytes) = f51(Adt13::C0(Adt12::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f54(v0.clone())?, f55(f56(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f55(f56(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f56(v0.clone(), f8(Str::lit("model/admissions.json")))?, f56(v0.clone(), f8(Str::lit("model/strata.json")))?, f57(f56(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f49(match v0 {
                                                Adt1::C0(_, h170, _) => {
                                                    h170
                                                }
                                            }, true)))?;
                                            let (h171, _) = v10;
                                            if h171 {
                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                    let (_, h172) = v10;
                                                    h172
                                                }))
                                            } else {
                                                Ok(Adt11::C4({
                                                    let (_, h173) = v10;
                                                    h173
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

pub fn f29(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f58(f59(match v0.clone() {
        Adt1::C0(_, h174, _) => {
            h174
        }
    }, f60())) {
        Ok(Adt11::C1(List::cons(f60(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f61(f62(f60(), f63(match v0.clone() {
            Adt1::C0(_, h175, _) => {
                h175
            }
        }, f60())))?;
        let m176 = f43(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m176.uncons() {
            None => {
                let m177 = f64(v0, f65(v2, v1))?;
                match m177 {
                    None => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])))
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

pub fn f30(v0: Adt1) -> Adt11 {
    if f58(f66(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h178) => {
            h178
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f67(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f58(f66(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h179) => {
                h179
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f67(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f31(v0: Adt1) -> R<Adt11> {
    let m180 = f43(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m180.uncons() {
        None => {
            Ok(Adt11::C3(List::cons((f68(), f69()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f70()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f71()), List::cons((f8(Str::lit("docs/model.md")), f72(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f73()), List::<(Bytes, Bytes)>::nil()))))), f8(Str::lit("published .github/workflows/pages.yml, .github/workflows/operate.yml, .github/CODEOWNERS, .github/secret_scanning.yml and docs/model.md\u{a}"))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f32(v0: Adt1) -> R<Adt11> {
    let m181 = f43(v0.clone(), List::cons(f74(), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m181.uncons() {
        None => {
            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f75(f76(f77(f78(f53(v0.clone(), f74()))?)), f79(f56(v0, f8(Str::lit("model/admissions.json")))?)))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f33(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m182 = f43(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m182.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f78(v4.clone())?;
            if f58(f80(f81(v5.clone()), f82())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f83(Str::lit("id"), v6.clone())?, List::cons(f83(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f83(Str::lit("artifact"), f74())?, List::cons(f83(Str::lit("checksum"), f84(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f74(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f85(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f86(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f87(f76(f77(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f34(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m183 = f43(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m183.uncons() {
        None => {
            let m184 = f88(f89(f53(v0, v1))?)?;
            match m184 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f87(f90(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f35(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m185 = f43(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil())))));
    match m185.uncons() {
        None => {
            let v4: List<Bytes> = f89(f53(v0.clone(), v1))?;
            let m186 = f91(v4.clone(), f8(Str::lit("requirement")))?;
            match m186 {
                None => {
                    Ok(Adt11::C4(f92()))
                }
                Some(v5) => {
                    let m187 = f91(v4, f8(Str::lit("admission")))?;
                    match m187 {
                        None => {
                            Ok(Adt11::C4(f92()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f56(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f56(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f47(f55(v5.clone(), f8(Str::lit("id"))), f55(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f61(f47(f55(v7.clone(), f8(Str::lit("id"))), f55(v8.clone(), f8(Str::lit("construct")))))?;
                            let a189 = {
                                let a188 = v9.clone();
                                length_list(a188)
                            };
                            let a190 = 0u64;
                            if nat_eq(a189, a190) {
                                Ok(Adt11::C4(f92()))
                            } else {
                                if f58({
                                    let a193 = {
                                        let a191 = f61(f47(v9.clone(), v10.clone()))?;
                                        length_list(a191)
                                    };
                                    let a194 = {
                                        let a192 = f47(v9.clone(), v10);
                                        length_list(a192)
                                    };
                                    nat_eq(a193, a194)
                                }) {
                                    Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                } else {
                                    Ok(Adt11::C3(List::cons(f93(f8(Str::lit("model/requirements.json")), f94(v7, v5))?, List::cons(f93(f8(Str::lit("model/admissions.json")), f94(v8, v6))?, List::cons(f86(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f87(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f36(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m195 = f43(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m195.uncons() {
        None => {
            let m196 = f95(v0, v1)?;
            match m196 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f96(f97(Some(v4))) {
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

pub fn f37(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m197 = f43(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m197.uncons() {
        None => {
            let m198 = f95(v0.clone(), v1.clone())?;
            match m198 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f58(f27(f98(v4.clone(), f8(Str::lit("schema"))), f99())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f98(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m199 = f100(v4.clone(), f8(Str::lit("source")));
                            match m199 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f101(f100(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f102(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f102(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m200 = v8;
                        match m200 {
                            None => {
                                f103(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f58(f27(f98(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f103(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f102(v9, f104(f98(v7.clone(), f8(Str::lit("url"))))?);
                                    if f58(f66(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h201) => {
                                            h201
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f98(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m202 = f43(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m202.uncons() {
                                            None => {
                                                if f27(f84(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f103(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f38(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m203 = f43(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil()))))));
    match m203.uncons() {
        None => {
            let m204 = f105(v3);
            match m204 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v6) => {
                    let m205 = f95(v0.clone(), v1.clone())?;
                    match m205 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                        }
                        Some(v7) => {
                            let v8: Bytes = {
                                let a206 = f8(Str::lit("Catalog."));
                                let a207 = f106(v2.clone())?;
                                append_bytes(a206, a207)
                            };
                            let v9: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f106(v2.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                            let m208 = f107(v7, v8.clone(), f106(v2.clone())?)?;
                            match m208 {
                                Ok(v11) => {
                                    Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f106(v2)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), f53(v0.clone(), v1)), List::cons((v9.clone(), v11), List::cons((f8(Str::lit("model/admissions.json")), f108(Adt14::C3(std::rc::Rc::new(f109(f110(f56(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v8.clone())), std::rc::Rc::new(v6)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f108(Adt14::C3(std::rc::Rc::new(f109(f110(f56(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                        let a212 = f111(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f42(v9.clone(), 4u64, {
                                            let a210 = {
                                                let a209 = v9;
                                                length_bytes(a209)
                                            };
                                            let a211 = 4u64;
                                            nat_sub(a210, a211)
                                        }))?;
                                        let a213 = Bytes::lit(&[0x0a]);
                                        append_bytes(a212, a213)
                                    }), List::cons(f86(v0, List::cons(v8.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v8, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                }
                                Err(v10) => {
                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f39(v0: u8) -> R<Str> {
    let m262 = {
        let a260 = f112();
        let a261 = {
            let v1: u8 = v0;
            let a258 = {
                let a234 = {
                    let a222 = {
                        let a216 = {
                            let a214 = v1;
                            let a215 = 128u8;
                            fixed_u8::bit_and(a214, a215)
                        };
                        let a217 = 128u8;
                        if equal(a216, a217) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a223 = {
                        let a220 = {
                            let a218 = v1;
                            let a219 = 64u8;
                            fixed_u8::bit_and(a218, a219)
                        };
                        let a221 = 64u8;
                        if equal(a220, a221) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a222, a223)?
                };
                let a235 = {
                    let a232 = {
                        let a226 = {
                            let a224 = v1;
                            let a225 = 32u8;
                            fixed_u8::bit_and(a224, a225)
                        };
                        let a227 = 32u8;
                        if equal(a226, a227) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a233 = {
                        let a230 = {
                            let a228 = v1;
                            let a229 = 16u8;
                            fixed_u8::bit_and(a228, a229)
                        };
                        let a231 = 16u8;
                        if equal(a230, a231) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a232, a233)?
                };
                nat_add(a234, a235)?
            };
            let a259 = {
                let a256 = {
                    let a244 = {
                        let a238 = {
                            let a236 = v1;
                            let a237 = 8u8;
                            fixed_u8::bit_and(a236, a237)
                        };
                        let a239 = 8u8;
                        if equal(a238, a239) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a245 = {
                        let a242 = {
                            let a240 = v1;
                            let a241 = 4u8;
                            fixed_u8::bit_and(a240, a241)
                        };
                        let a243 = 4u8;
                        if equal(a242, a243) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a244, a245)?
                };
                let a257 = {
                    let a254 = {
                        let a248 = {
                            let a246 = v1;
                            let a247 = 2u8;
                            fixed_u8::bit_and(a246, a247)
                        };
                        let a249 = 2u8;
                        if equal(a248, a249) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a255 = {
                        let a252 = {
                            let a250 = v1;
                            let a251 = 1u8;
                            fixed_u8::bit_and(a250, a251)
                        };
                        let a253 = 1u8;
                        if equal(a252, a253) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a254, a255)?
                };
                nat_add(a256, a257)?
            };
            nat_add(a258, a259)?
        };
        index_list(a260, a261)
    };
    match m262 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f40(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a263 = v0.clone();
        length_bytes(a263)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h264, _) = f113(Fn0::F114(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h264
    };
    let v4: u64 = {
        let (h265, _) = {
            let (_, h266) = v3.clone();
            h266
        };
        h265
    };
    Ok(f115(List::cons(f42(v0, v4, {
        let a267 = v2;
        let a268 = v4;
        nat_sub(a267, a268)
    }), {
        let (_, h269) = {
            let (_, h270) = v3;
            h270
        };
        h269
    })))
}

pub fn f41(p274: List<Bytes>) -> Bytes {
    let m273 = p274;
    {
        let mut m273 = m273;
        loop {
            let step = {
                let v0 = m273;
                let m271 = v0;
                match m271.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m272 = v2.clone();
                        match m272.uncons() {
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
                Ok(next) => { m273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m278 = {
        let a275 = v0;
        let a276 = v1;
        let a277 = v2;
        slice_bytes(a275, a276, a277)
    };
    match m278 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f43(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m279 = v1;
    match m279.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m281 = f116(match v0.clone() {
                Adt1::C0(h280, _, _) => {
                    h280
                }
            }, v2.clone());
            match m281 {
                None => {
                    List::cons(v2, f43(v0, v3))
                }
                Some(_) => {
                    f43(v0, v3)
                }
            }
        }
    }
}

pub fn f44() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f74(), List::<Bytes>::nil())))))))
}

pub fn f45(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m282 = v1;
    match m282.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f117(v0.clone(), v2.clone()) {
                f118(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m283 = f95(v0.clone(), v2.clone())?;
                match m283 {
                    None => {
                        Ok(List::cons(v2, f45(v0, v3)?))
                    }
                    Some(_) => {
                        f45(v0, v3)
                    }
                }
            } else {
                f45(v0, v3)
            }
        }
    }
}

pub fn f46(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m284 = v1;
    match m284.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f59(v0.clone(), v2.clone()) {
                f46(v0, v3)
            } else {
                List::cons(v2, f46(v0, v3))
            }
        }
    }
}

pub fn f47(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m285 = v0;
    match m285.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f47(v3, v1))
        }
    }
}

pub fn f48() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f49(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m286 = v0;
    match m286.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f47(f119({
                let (h287, _) = v2;
                h287
            }, {
                let (_, h288) = v2;
                h288
            }, v1), f49(v3, v1))
        }
    }
}

pub fn f50(v0: Adt1, p292: List<Bytes>) -> R<Option<Bytes>> {
    let m291 = p292;
    {
        let mut m291 = m291;
        loop {
            let step = {
                let v1 = m291;
                let m289 = v1;
                match m289.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m290 = f120(f53(v0.clone(), v2.clone()), f121())?;
                        match m290 {
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
                Ok(next) => { m291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h293, _, _, _, _) => {
            h293
        }
    };
    let v2: Adt15 = f122(v1.clone())?;
    let v3: List<Bytes> = f123(v1.clone());
    let v4: List<Bytes> = f55(match v0.clone() {
        Adt13::C0(_, _, h294, _, _) => {
            h294
        }
    }, f8(Str::lit("path")));
    let m296 = f124(v2, List::<u64>::nil(), f125(v1.clone(), match v0.clone() {
        Adt13::C0(_, h295, _, _, _) => {
            h295
        }
    }, 0u64)?);
    match m296 {
        None => {
            if f127(f128(match v0.clone() {
                Adt13::C0(_, _, _, _, h298) => {
                    h298
                }
            }, f47(match v0.clone() {
                Adt13::C0(_, _, _, _, h299) => {
                    h299
                }
            }, v4.clone()))?, f128(v4.clone(), f47(match v0.clone() {
                Adt13::C0(_, _, _, _, h300) => {
                    h300
                }
            }, v4))?, f129(match v0.clone() {
                Adt13::C0(_, _, h301, _, _) => {
                    h301
                }
            })) {
                let m305 = f130(f128(match v0.clone() {
                    Adt13::C0(_, _, _, h302, _) => {
                        h302
                    }
                }, f47(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h303, _) => {
                        h303
                    }
                }))?, f128(v3.clone(), f47(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h304, _) => {
                        h304
                    }
                }))?);
                match m305 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f87(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f126(f47(v3, match v0 {
                            Adt13::C0(_, _, _, h306, _) => {
                                h306
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f126(match v1 {
                Adt12::C0(_, _, _, _, h297) => {
                    h297
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f52(v0: Bytes) -> R<List<Bytes>> {
    f131(f89(v0)?, f8(Str::lit("id = \"")))
}

pub fn f53(v0: Adt1, v1: Bytes) -> Bytes {
    let m308 = f116(match v0 {
        Adt1::C0(h307, _, _) => {
            h307
        }
    }, v1);
    match m308 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m309 = v2;
            match m309 {
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
    let m310 = f78(f53(v0, f74()))?;
    match m310.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f76(v2))
        }
    }
}

pub fn f55(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m311 = v0;
    match m311.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f98(v2, v1.clone()), f55(v3, v1))
        }
    }
}

pub fn f56(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f97(f95(v0, v1)?))
}

pub fn f57(v0: List<Adt14>) -> List<Bytes> {
    let m312 = v0;
    match m312.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m313 = f101(Some(v1));
            match m313 {
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

pub fn f59(p317: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m316 = p317;
    {
        let mut m316 = m316;
        loop {
            let step = {
                let v0 = m316;
                let m314 = v0;
                match m314.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f27({
                            let (h315, _) = v2;
                            h315
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f61(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f132(f133(Fn1::F134, List::<Bytes>::nil(), f135(v0)?)))
}

pub fn f62(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m318 = v1;
    match m318.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f118(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f102(v0.clone(), v2), f62(v0, v3))
            } else {
                f62(v0, v3)
            }
        }
    }
}

pub fn f63(p323: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m322 = p323;
    {
        let mut m322 = m322;
        loop {
            let step = {
                let v0 = m322;
                let m319 = v0;
                match m319.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f27({
                            let (h320, _) = v2;
                            h320
                        }, v1.clone()) {
                            let (_, h321) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h321)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt1, p326: List<Bytes>) -> R<Option<Bytes>> {
    let m325 = p326;
    {
        let mut m325 = m325;
        loop {
            let step = {
                let v1 = m325;
                let m324 = v1;
                match m324.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        if f136(v0.clone(), v2.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v2)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m327 = v0;
    match m327.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f66(v2.clone(), v1.clone()) {
                f65(v3, v1)
            } else {
                List::cons(v2, f65(v3, v1))
            }
        }
    }
}

pub fn f66(v0: Bytes, p330: List<Bytes>) -> bool {
    let m329 = p330;
    {
        let mut m329 = m329;
        loop {
            let step = {
                let v1 = m329;
                let m328 = v1;
                match m328.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f27(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
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

pub fn f67(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f68() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f69() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f70() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          { echo \"proposed a change to the API's sources, exported again as $(basename studio/.lexlean/production/*/)\"; git diff --stat -- studio/src README.md; } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#ingest-authority }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          gh pr create --head \"$branch\" --base main --title \"verdant_studio $ARGS\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f71() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f72(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f137(f56(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f138(f56(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f73() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f74() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f75(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m331 = v0;
    match m331.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f66(v2.clone(), v1.clone()) {
                f75(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f75(v3, v1))
            }
        }
    }
}

pub fn f76(v0: List<List<Bytes>>) -> List<Bytes> {
    let m332 = v0;
    match m332.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m333 = v1;
            match m333.uncons() {
                None => {
                    f76(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f76(v2))
                }
            }
        }
    }
}

pub fn f77(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m334 = v0;
    match m334.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f78(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f139(v0.clone(), 0u64, {
        let a335 = v0;
        length_bytes(a335)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a338 = {
            let a337 = match v1.clone() {
                Adt16::C0(_, _, h336, _, _) => {
                    h336
                }
            };
            length_bytes(a337)
        };
        let a339 = 0u64;
        if if nat_eq(a338, a339) {
            let m341 = match v1.clone() {
                Adt16::C0(_, _, _, h340, _) => {
                    h340
                }
            };
            match m341.uncons() {
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
            f140(v1)
        }
    };
    Ok(f141(match v4 {
        Adt16::C0(_, _, _, _, h342) => {
            h342
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f79(v0: List<Adt14>) -> List<Bytes> {
    let m343 = v0;
    match m343.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m344 = f100(v1, f8(Str::lit("std")));
            match m344 {
                None => {
                    f79(v2)
                }
                Some(v3) => {
                    List::cons(f98(v3, f8(Str::lit("row"))), f79(v2))
                }
            }
        }
    }
}

pub fn f80(p349: List<Bytes>, p350: List<Bytes>) -> bool {
    let m348 = (p349, p350);
    {
        let mut m348 = m348;
        loop {
            let step = {
                let (v0, v1) = m348;
                let m345 = v0;
                match m345.uncons() {
                    None => {
                        let m346 = v1;
                        match m346.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m347 = v1;
                        match m347.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f27(v4, v6) {
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
                Ok(next) => { m348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<List<Bytes>>) -> List<Bytes> {
    let m351 = v0;
    match m351.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f82() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f83(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f142(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f84(v0: Bytes) -> R<Bytes> {
    Ok(f143(f144(v0)?))
}

pub fn f85(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f145(f146(f89(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f147(f148(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f86(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f108(Adt14::C3(std::rc::Rc::new(f149(f61(f47(v1, f57(f56(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f87(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m352 = f150(Fn2::F151(v1), None::<Bytes>, v0);
    match m352 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f88(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m353 = v0;
    match m353.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f152(v1)?;
            let a355 = {
                let a354 = v3.clone();
                length_bytes(a354)
            };
            let a356 = 0u64;
            if nat_eq(a355, a356) {
                f88(v2)
            } else {
                let m357 = f153(v3.clone())?;
                match m357 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f98(v4.clone(), f8(Str::lit("id")));
                        if if f27(f98(v4, f8(Str::lit("schema"))), f99()) {
                            f58({
                                let a359 = {
                                    let a358 = v5.clone();
                                    length_bytes(a358)
                                };
                                let a360 = 0u64;
                                nat_eq(a359, a360)
                            })
                        } else {
                            false
                        } {
                            let m361 = f88(v2)?;
                            match m361 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f154(v5), {
                                        let a362 = v3;
                                        let a363 = Bytes::lit(&[0x0a]);
                                        append_bytes(a362, a363)
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

pub fn f89(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a364 = v0.clone();
        length_bytes(a364)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h365, _) = f113(Fn0::F155(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h365
    };
    let v3: u64 = {
        let (h366, _) = {
            let (_, h367) = v2.clone();
            h367
        };
        h366
    };
    let v4: List<Bytes> = {
        let (_, h368) = {
            let (_, h369) = v2;
            h369
        };
        h368
    };
    let a370 = v3;
    let a371 = v1;
    if nat_lt(a370, a371) {
        Ok(f115(List::cons(f42(v0, v3, {
            let a372 = v1;
            let a373 = v3;
            nat_sub(a372, a373)
        }), v4)))
    } else {
        Ok(f115(v4))
    }
}

pub fn f90(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m374 = v0;
    match m374.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h375, _) = v1;
                h375
            }, f90(v2))
        }
    }
}

pub fn f91(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m376 = v0;
    match m376.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f152(v2)?;
            let a378 = {
                let a377 = v4.clone();
                length_bytes(a377)
            };
            let a379 = 0u64;
            if nat_eq(a378, a379) {
                f91(v3, v1)
            } else {
                let m380 = f153(v4)?;
                match m380 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f156(v5.clone());
                        let a382 = {
                            let a381 = v6.clone();
                            length_bytes(a381)
                        };
                        let a383 = 0u64;
                        if nat_eq(a382, a383) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m384 = f91(v3, v1.clone())?;
                            match m384 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f27(v6, v1) {
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

pub fn f92() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f93(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f108(Adt14::C3(std::rc::Rc::new(f109(v1))))?))
}

pub fn f94(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m385 = v0;
    match m385.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f94(v3, v1))
        }
    }
}

pub fn f95(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f153(f53(v0, v1))
}

pub fn f96(p388: List<Adt14>) -> bool {
    let m387 = p388;
    {
        let mut m387 = m387;
        loop {
            let step = {
                let v0 = m387;
                let m386 = v0;
                match m386.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f27(f98(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f66(f8(Str::lit("studio")), f55(f157(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Option<Adt14>) -> List<Adt14> {
    let m389 = v0;
    match m389 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f158(v1)
        }
    }
}

pub fn f98(v0: Adt14, v1: Bytes) -> Bytes {
    let m390 = f101(f100(v0, v1));
    match m390 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f99() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f100(p396: Adt14, v1: Bytes) -> Option<Adt14> {
    let m395 = p396;
    {
        let mut m395 = m395;
        loop {
            let step = {
                let v0 = m395;
                let m391 = v0;
                match m391 {
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
                    Adt14::C4(r392) => {
                        let v5 = (*r392).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r393, r394) => {
                        let v3 = (*r393).clone();
                        let v4 = (*r394).clone();
                        if f27(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Option<Adt14>) -> Option<Bytes> {
    let m397 = v0;
    match m397 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m398 = v1;
            match m398 {
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

pub fn f102(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f103(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f85(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f159(v1, v4)?)?), List::cons(f86(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f104(v0: Bytes) -> R<Bytes> {
    Ok(f41(f40(v0, 47u8)?))
}

pub fn f105(v0: List<Bytes>) -> Option<Adt14> {
    let m399 = v0;
    match m399.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f27(v1.clone(), f8(Str::lit("req"))) {
                let m400 = v2;
                match m400.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f27(v1.clone(), f8(Str::lit("std"))) {
                    let m401 = v2;
                    match m401.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m402 = v6;
                                match m402.uncons() {
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
                    if f27(v1, f8(Str::lit("der"))) {
                        let m403 = v2;
                        match m403.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m404 = v10;
                                match m404.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f149(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f106(v0: Bytes) -> R<Bytes> {
    f160(v0.clone(), 0u64, {
        let a405 = v0;
        length_bytes(a405)
    }, true, Bytes::lit(&[]))
}

pub fn f107(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m406 = f161(4096u64, f162(f163(f164(f100(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt17::C0(v2, v0), List::<Adt17>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m406 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f148(f165(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f108(v0: Adt14) -> R<Bytes> {
    let a407 = f166(v0)?;
    let a408 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a407, a408))
}

pub fn f109(v0: List<Adt14>) -> Adt14 {
    let m409 = v0;
    match m409.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f109(v2)))
        }
    }
}

pub fn f110(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m410 = v0;
    match m410.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f110(v3, v1))
        }
    }
}

pub fn f111(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f148(f167(f89(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f112() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f113(v0: Fn0, p415: u64, p416: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m414 = (p415, p416);
    {
        let mut m414 = m414;
        loop {
            let step = {
                let (v1, v2) = m414;
                let m411 = v1;
                if m411 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m411 - 1;
                    let m413 = {
                        let c412: Fn0 = v0.clone();
                        c412.apply(v2.clone())?
                    };
                    match m413 {
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
                Ok(next) => { m414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f168(v1, v0, v2)
}

pub fn f115(v0: List<Bytes>) -> List<Bytes> {
    f133(Fn1::F169, List::<Bytes>::nil(), v0)
}

pub fn f116(p421: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m420 = p421;
    {
        let mut m420 = m420;
        loop {
            let step = {
                let v0 = m420;
                let m417 = v0;
                match m417.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f27({
                            let (h418, _) = v2;
                            h418
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h419) = v2;
                                h419
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m420 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: Adt1, v1: Bytes) -> bool {
    let m423 = f116(match v0 {
        Adt1::C0(h422, _, _) => {
            h422
        }
    }, v1);
    match m423 {
        None => {
            false
        }
        Some(v2) => {
            let m424 = v2;
            match m424 {
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
        let a425 = v0.clone();
        length_bytes(a425)
    };
    let v3: u64 = {
        let a426 = v1.clone();
        length_bytes(a426)
    };
    let a427 = v3;
    let a428 = v2;
    if nat_le(a427, a428) {
        let m434 = {
            let a431 = v0;
            let a432 = {
                let a429 = v2;
                let a430 = v3;
                nat_sub(a429, a430)
            };
            let a433 = v3;
            slice_bytes(a431, a432, a433)
        };
        match m434 {
            None => {
                false
            }
            Some(v4) => {
                let a435 = v4;
                let a436 = v1;
                equal(a435, a436)
            }
        }
    } else {
        false
    }
}

pub fn f119(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m437 = v1;
    match m437.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f170(v3.clone())
            } else {
                f58(f170(v3.clone()))
            } {
                List::cons(f102(v0.clone(), v3), f119(v0, v4, v2))
            } else {
                f119(v0, v4, v2)
            }
        }
    }
}

pub fn f120(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a438 = v0.clone();
        length_bytes(a438)
    };
    let v3: u64 = {
        let a439 = v1.clone();
        length_bytes(a439)
    };
    let v4: u64 = {
        let (h440, _) = f171(Fn3::F172(v1.clone(), v3, v0.clone(), v2), {
            let a441 = v2;
            let a442 = 1u64;
            nat_add(a441, a442)?
        }, 0u64)?;
        h440
    };
    if f173(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f121() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f122(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f174(match v0.clone() {
        Adt12::C0(h443, _, _, _, _) => {
            h443
        }
    }, 0u64)?, f174(match v0.clone() {
        Adt12::C0(_, h444, _, _, _) => {
            h444
        }
    }, 0u64)?, f174(match v0.clone() {
        Adt12::C0(_, _, h445, _, _) => {
            h445
        }
    }, 0u64)?, f174(match v0 {
        Adt12::C0(_, _, _, h446, _) => {
            h446
        }
    }, 0u64)?))
}

pub fn f123(v0: Adt12) -> List<Bytes> {
    f47(match v0.clone() {
        Adt12::C0(h447, _, _, _, _) => {
            h447
        }
    }, f47(match v0.clone() {
        Adt12::C0(_, _, h448, _, _) => {
            h448
        }
    }, f47(match v0.clone() {
        Adt12::C0(_, _, _, h449, _) => {
            h449
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h450) => {
            h450
        }
    })))
}

pub fn f124(v0: Adt15, p456: List<u64>, p457: List<Adt18>) -> Option<u64> {
    let m455 = (p456, p457);
    {
        let mut m455 = m455;
        loop {
            let step = {
                let (v1, v2) = m455;
                let m451 = v2;
                match m451.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f175(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h452) => {
                                h452
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h453, _) => {
                                    h453
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h454, _) => {
                                    h454
                                }
                            }))
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

pub fn f125(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m458 = v1;
    match m458.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f176(v0.clone(), v3)?), f125(v0, v4, {
                let a459 = v2;
                let a460 = 1u64;
                nat_add(a459, a460)?
            })?))
        }
    }
}

pub fn f126(p464: List<Bytes>, p465: u64) -> Bytes {
    let m463 = (p464, p465);
    {
        let mut m463 = m463;
        loop {
            let step = {
                let (v0, v1) = m463;
                let m461 = v0;
                match m461.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m462 = v1;
                        if m462 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m462 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m463 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
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

pub fn f128(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m466 = v0;
    match m466.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f179(v2, v1.clone())?, f128(v3, v1)?))
        }
    }
}

pub fn f129(v0: List<Adt14>) -> List<u64> {
    let m467 = v0;
    match m467.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f180(v1), f129(v2))
        }
    }
}

pub fn f130(p470: List<u64>, v1: List<u64>) -> Option<u64> {
    let m469 = p470;
    {
        let mut m469 = m469;
        loop {
            let step = {
                let v0 = m469;
                let m468 = v0;
                match m468.uncons() {
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
                Ok(next) => { m469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m471 = v0;
    match m471.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m472 = f182(v2, v1.clone())?;
            match m472 {
                None => {
                    f131(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f131(v3, v1)?))
                }
            }
        }
    }
}

pub fn f132(v0: List<Bytes>) -> List<Bytes> {
    f133(Fn1::F183, List::<Bytes>::nil(), v0)
}

pub fn f133(v0: Fn1, p476: List<Bytes>, p477: List<Bytes>) -> List<Bytes> {
    let m475 = (p476, p477);
    {
        let mut m475 = m475;
        loop {
            let step = {
                let (v1, v2) = m475;
                let m473 = v2;
                match m473.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c474: Fn1 = v0.clone();
                            c474.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f184(v0, v1)
}

pub fn f135(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f185(f186(f187(v0))?))
}

pub fn f136(v0: Adt1, v1: Bytes) -> R<bool> {
    let m478 = f95(v0.clone(), v1)?;
    match m478 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let v3: List<Bytes> = f188(f146(f89(f53(v0, f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f98(v2.clone(), f8(Str::lit("id"))))?;
            let m479 = v3.clone();
            match m479.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m481 = f101(f100({
                        let m480 = f100(v2.clone(), f8(Str::lit("source")));
                        match m480 {
                            None => {
                                Adt14::C5
                            }
                            Some(v6) => {
                                v6
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m481 {
                        None => {
                            Ok(false)
                        }
                        Some(v7) => {
                            Ok(f58(f66(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f142(f189(v2, v7))?, List::<Bytes>::nil()))), v3)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f137(v0: List<Adt14>) -> List<Bytes> {
    let m482 = v0;
    match m482.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f98(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f98(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f98(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f137(v2))
        }
    }
}

pub fn f138(v0: List<Adt14>) -> List<Bytes> {
    let m483 = v0;
    match m483.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f98(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f190(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f138(v2))
        }
    }
}

pub fn f139(v0: Bytes, p541: u64, p542: u64, p543: Adt16) -> R<Adt16> {
    let m540 = (p541, (p542, p543));
    {
        let mut m540 = m540;
        loop {
            let step = {
                let (v1, (v2, v3)) = m540;
                let m484 = v2;
                if m484 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m484 - 1;
                    let v5: u64 = f191(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h485, _, _, _) => {
                            h485
                        }
                    } {
                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                            let a486 = v1;
                            let a487 = 1u64;
                            nat_add(a486, a487)?
                        }, (v4, Adt16::C0(true, false, match v3.clone() {
                            Adt16::C0(_, _, h488, _, _) => {
                                h488
                            }
                        }, match v3.clone() {
                            Adt16::C0(_, _, _, h489, _) => {
                                h489
                            }
                        }, match v3 {
                            Adt16::C0(_, _, _, _, h490) => {
                                h490
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt16::C0(h491, _, _, _, _) => {
                                h491
                            }
                        } {
                            let a492 = v5;
                            let a493 = 34u64;
                            if nat_eq(a492, a493) {
                                let a496 = f191(v0.clone(), {
                                    let a494 = v1;
                                    let a495 = 1u64;
                                    nat_add(a494, a495)?
                                })?;
                                let a497 = 34u64;
                                if nat_eq(a496, a497) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a498 = v1;
                                        let a499 = 1u64;
                                        nat_add(a498, a499)?
                                    }, (v4, Adt16::C0(true, true, f192(match v3.clone() {
                                        Adt16::C0(_, _, h500, _, _) => {
                                            h500
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h501, _) => {
                                            h501
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h502) => {
                                            h502
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a503 = v1;
                                        let a504 = 1u64;
                                        nat_add(a503, a504)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h505, _, _) => {
                                            h505
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h506, _) => {
                                            h506
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h507) => {
                                            h507
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a508 = v1;
                                    let a509 = 1u64;
                                    nat_add(a508, a509)?
                                }, (v4, Adt16::C0(true, false, f192(match v3.clone() {
                                    Adt16::C0(_, _, h510, _, _) => {
                                        h510
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h511, _) => {
                                        h511
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h512) => {
                                        h512
                                    }
                                }))))
                            }
                        } else {
                            let a513 = v5;
                            let a514 = 34u64;
                            if nat_eq(a513, a514) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a515 = v1;
                                    let a516 = 1u64;
                                    nat_add(a515, a516)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h517, _, _) => {
                                        h517
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h518, _) => {
                                        h518
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h519) => {
                                        h519
                                    }
                                }))))
                            } else {
                                let a520 = v5;
                                let a521 = 44u64;
                                if nat_eq(a520, a521) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a522 = v1;
                                        let a523 = 1u64;
                                        nat_add(a522, a523)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h524, _, _) => {
                                            h524
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h525, _) => {
                                            h525
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h526) => {
                                            h526
                                        }
                                    }))))
                                } else {
                                    let a527 = v5;
                                    let a528 = 13u64;
                                    if nat_eq(a527, a528) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a529 = v1;
                                            let a530 = 1u64;
                                            nat_add(a529, a530)?
                                        }, (v4, v3)))
                                    } else {
                                        let a531 = v5;
                                        let a532 = 10u64;
                                        if nat_eq(a531, a532) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a533 = v1;
                                                let a534 = 1u64;
                                                nat_add(a533, a534)?
                                            }, (v4, f140(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a535 = v1;
                                                let a536 = 1u64;
                                                nat_add(a535, a536)?
                                            }, (v4, Adt16::C0(false, false, f192(match v3.clone() {
                                                Adt16::C0(_, _, h537, _, _) => {
                                                    h537
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h538, _) => {
                                                    h538
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h539) => {
                                                    h539
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
                Ok(next) => { m540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f193(List::cons(match v0.clone() {
        Adt16::C0(_, _, h544, _, _) => {
            h544
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h545, _) => {
            h545
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h546) => {
            h546
        }
    }))
}

pub fn f141(p549: List<List<Bytes>>, p550: List<List<Bytes>>) -> List<List<Bytes>> {
    let m548 = (p549, p550);
    {
        let mut m548 = m548;
        loop {
            let step = {
                let (v0, v1) = m548;
                let m547 = v0;
                match m547.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m548 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f194(v0.clone(), 0u64, {
        let a551 = v0;
        length_bytes(a551)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f143(v0: Adt21) -> Bytes {
    let a572 = f195(match v0.clone() {
        Adt21::C0(h552, _, _, _, _, _, _, _) => {
            h552
        }
    });
    let a573 = {
        let a570 = f195(match v0.clone() {
            Adt21::C0(_, h553, _, _, _, _, _, _) => {
                h553
            }
        });
        let a571 = {
            let a568 = f195(match v0.clone() {
                Adt21::C0(_, _, h554, _, _, _, _, _) => {
                    h554
                }
            });
            let a569 = {
                let a566 = f195(match v0.clone() {
                    Adt21::C0(_, _, _, h555, _, _, _, _) => {
                        h555
                    }
                });
                let a567 = {
                    let a564 = f195(match v0.clone() {
                        Adt21::C0(_, _, _, _, h556, _, _, _) => {
                            h556
                        }
                    });
                    let a565 = {
                        let a562 = f195(match v0.clone() {
                            Adt21::C0(_, _, _, _, _, h557, _, _) => {
                                h557
                            }
                        });
                        let a563 = {
                            let a560 = f195(match v0.clone() {
                                Adt21::C0(_, _, _, _, _, _, h558, _) => {
                                    h558
                                }
                            });
                            let a561 = f195(match v0 {
                                Adt21::C0(_, _, _, _, _, _, _, h559) => {
                                    h559
                                }
                            });
                            append_bytes(a560, a561)
                        };
                        append_bytes(a562, a563)
                    };
                    append_bytes(a564, a565)
                };
                append_bytes(a566, a567)
            };
            append_bytes(a568, a569)
        };
        append_bytes(a570, a571)
    };
    append_bytes(a572, a573)
}

pub fn f144(v0: Bytes) -> R<Adt21> {
    let v1: u64 = {
        let a574 = v0.clone();
        length_bytes(a574)
    };
    let v2: u64 = {
        let a580 = {
            let a577 = {
                let a575 = v1;
                let a576 = 72u64;
                nat_add(a575, a576)?
            };
            let a578 = 64u64;
            let a579 = 0u64;
            nat_quot(a577, a578, a579)
        };
        let a581 = 64u64;
        nat_mul(a580, a581)?
    };
    let (_, h582) = f196(Fn4::F197(v0, v1, v2), {
        let a583 = v2;
        let a584 = 64u64;
        let a585 = 0u64;
        nat_quot(a583, a584, a585)
    }, (0u64, f198()))?;
    Ok(h582)
}

pub fn f145(p588: List<List<Bytes>>, v1: Bytes, p589: List<Bytes>) -> R<List<Bytes>> {
    let m587 = (p588, p589);
    {
        let mut m587 = m587;
        loop {
            let step = {
                let (v0, v2) = m587;
                let m586 = v0;
                match m586.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f199(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f148(f200(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m587 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(p592: List<Bytes>, p593: List<Bytes>, p594: List<List<Bytes>>) -> List<List<Bytes>> {
    let m591 = (p592, (p593, p594));
    {
        let mut m591 = m591;
        loop {
            let step = {
                let (v0, (v1, v2)) = m591;
                let m590 = v0;
                match m590.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f165(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f27(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f165(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f147(v0: Bytes) -> R<Bytes> {
    Ok(f42(v0.clone(), 0u64, f201(v0)?))
}

pub fn f148(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m595 = v0;
    match m595.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m596 = v3.clone();
            match m596.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a599 = v2;
                    let a600 = {
                        let a597 = v1.clone();
                        let a598 = f148(v3, v1);
                        append_bytes(a597, a598)
                    };
                    append_bytes(a599, a600)
                }
            }
        }
    }
}

pub fn f149(v0: List<Bytes>) -> Adt14 {
    let m601 = v0;
    match m601.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f149(v2)))
        }
    }
}

pub fn f150(v0: Fn2, p605: Option<Bytes>, p606: List<Bytes>) -> Option<Bytes> {
    let m604 = (p605, p606);
    {
        let mut m604 = m604;
        loop {
            let step = {
                let (v1, v2) = m604;
                let m602 = v2;
                match m602.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c603: Fn2 = v0.clone();
                            c603.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m604 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m607 = v1;
    match m607 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a610 = {
                    let a608 = v3;
                    let a609 = v0;
                    append_bytes(a608, a609)
                };
                let a611 = v2;
                append_bytes(a610, a611)
            })
        }
    }
}

pub fn f152(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f201(v0.clone())?;
    let v2: u64 = f202(v0.clone(), v1)?;
    Ok(f42(v0, v2, {
        let a612 = v1;
        let a613 = v2;
        nat_sub(a612, a613)
    }))
}

pub fn f153(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f203(v0)?;
    let m617 = f204(v1.clone(), 0u64, {
        let a615 = {
            let a614 = v1.clone();
            length_bytes(a614)
        };
        let a616 = 2u64;
        nat_add(a615, a616)?
    }, 0u64)?;
    match m617 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a620 = {
                let (_, h618) = v2;
                h618
            };
            let a621 = {
                let a619 = v1;
                length_bytes(a619)
            };
            if nat_eq(a620, a621) {
                Ok(Some({
                    let (h622, _) = v2;
                    h622
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f154(v0: Bytes) -> Bytes {
    f7(List::cons(f60(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f155(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f205(v0, v1)
}

pub fn f156(v0: Adt14) -> Bytes {
    if f206(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f206(v0.clone(), f8(Str::lit("id"))) {
            if f206(v0.clone(), f8(Str::lit("statement"))) {
                if f206(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f207(v0, f8(Str::lit("evidence")))
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

pub fn f157(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m623 = f100(v0, v1);
    match m623 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f97(f100(v3, v2))
        }
    }
}

pub fn f158(v0: Adt14) -> List<Adt14> {
    let m624 = v0;
    match m624 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r625) => {
            let v3 = (*r625).clone();
            f158(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r626, r627) => {
            let v1 = (*r626).clone();
            let v2 = (*r627).clone();
            List::cons(v1, f158(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f159(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f100(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f83(Str::lit("id"), f98(v0.clone(), f8(Str::lit("id"))))?, List::cons(f83(Str::lit("name"), f98(v0.clone(), f8(Str::lit("name"))))?, List::cons(f83(Str::lit("issuer"), f98(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f83(Str::lit("edition"), f98(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f83(Str::lit("canonical_id"), f98(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f83(Str::lit("source_role"), f98(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f83(Str::lit("retrieval_date"), f98(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f83(Str::lit("url"), f208(v2.clone(), f8(Str::lit("url"))))?, List::cons(f83(Str::lit("media_type"), f208(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f83(Str::lit("license"), f208(v2.clone(), f8(Str::lit("license"))))?, List::cons(f83(Str::lit("redistribution"), f208(v2, f8(Str::lit("redistribution"))))?, List::cons(f83(Str::lit("statement"), f98(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m628 = v1;
        match m628 {
            None => {
                f7(List::cons(f83(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f83(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f83(Str::lit("artifact"), {
                    let (h629, _) = v3;
                    h629
                })?, List::cons(f83(Str::lit("checksum"), {
                    let (_, h630) = v3;
                    h630
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f209(f57(f97(f100(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f160(v0: Bytes, p697: u64, p698: u64, p699: bool, p700: Bytes) -> R<Bytes> {
    let m696 = (p697, (p698, (p699, p700)));
    {
        let mut m696 = m696;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m696;
                let m631 = v2;
                if m631 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m631 - 1;
                    let m634 = {
                        let a632 = v0.clone();
                        let a633 = v1;
                        index_bytes(a632, a633)
                    };
                    match m634 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a679 = {
                                    let a655 = {
                                        let a643 = {
                                            let a637 = {
                                                let a635 = v7;
                                                let a636 = 128u8;
                                                fixed_u8::bit_and(a635, a636)
                                            };
                                            let a638 = 128u8;
                                            if equal(a637, a638) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a644 = {
                                            let a641 = {
                                                let a639 = v7;
                                                let a640 = 64u8;
                                                fixed_u8::bit_and(a639, a640)
                                            };
                                            let a642 = 64u8;
                                            if equal(a641, a642) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a643, a644)?
                                    };
                                    let a656 = {
                                        let a653 = {
                                            let a647 = {
                                                let a645 = v7;
                                                let a646 = 32u8;
                                                fixed_u8::bit_and(a645, a646)
                                            };
                                            let a648 = 32u8;
                                            if equal(a647, a648) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a654 = {
                                            let a651 = {
                                                let a649 = v7;
                                                let a650 = 16u8;
                                                fixed_u8::bit_and(a649, a650)
                                            };
                                            let a652 = 16u8;
                                            if equal(a651, a652) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a653, a654)?
                                    };
                                    nat_add(a655, a656)?
                                };
                                let a680 = {
                                    let a677 = {
                                        let a665 = {
                                            let a659 = {
                                                let a657 = v7;
                                                let a658 = 8u8;
                                                fixed_u8::bit_and(a657, a658)
                                            };
                                            let a660 = 8u8;
                                            if equal(a659, a660) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a666 = {
                                            let a663 = {
                                                let a661 = v7;
                                                let a662 = 4u8;
                                                fixed_u8::bit_and(a661, a662)
                                            };
                                            let a664 = 4u8;
                                            if equal(a663, a664) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a665, a666)?
                                    };
                                    let a678 = {
                                        let a675 = {
                                            let a669 = {
                                                let a667 = v7;
                                                let a668 = 2u8;
                                                fixed_u8::bit_and(a667, a668)
                                            };
                                            let a670 = 2u8;
                                            if equal(a669, a670) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a676 = {
                                            let a673 = {
                                                let a671 = v7;
                                                let a672 = 1u8;
                                                fixed_u8::bit_and(a671, a672)
                                            };
                                            let a674 = 1u8;
                                            if equal(a673, a674) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a675, a676)?
                                    };
                                    nat_add(a677, a678)?
                                };
                                nat_add(a679, a680)?
                            };
                            if f210(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a681 = v1;
                                    let a682 = 1u64;
                                    nat_add(a681, a682)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f211(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a683 = v1;
                                        let a684 = 1u64;
                                        nat_add(a683, a684)?
                                    }, (v5, (false, {
                                        let a688 = v4;
                                        let a689 = f42({
                                            let a685 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a685)
                                        }, {
                                            let a686 = v8;
                                            let a687 = 97u64;
                                            nat_sub(a686, a687)
                                        }, 1u64);
                                        append_bytes(a688, a689)
                                    }))))
                                } else {
                                    if if f211(v8) {
                                        true
                                    } else {
                                        if f212(v8) {
                                            true
                                        } else {
                                            f213(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a690 = v1;
                                            let a691 = 1u64;
                                            nat_add(a690, a691)?
                                        }, (v5, (false, {
                                            let a692 = v4;
                                            let a693 = f42(v0.clone(), v1, 1u64);
                                            append_bytes(a692, a693)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a694 = v1;
                                            let a695 = 1u64;
                                            nat_add(a694, a695)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m696 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(p708: u64, p709: List<Adt17>, p710: List<Bytes>, p711: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m707 = (p708, (p709, (p710, p711)));
    {
        let mut m707 = m707;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m707;
                let m701 = v0;
                if m701 == 0 {
                    Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m701 - 1;
                    let m702 = v1;
                    match m702.uncons() {
                        None => {
                            Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m703 = v5;
                            match m703 {
                                Adt17::C0(v8, v9) => {
                                    if f66(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m704 = f214(v8.clone(), v9)?;
                                        match m704 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f162({
                                                    let (_, h705) = v11;
                                                    h705
                                                }, List::cons(Adt17::C1({
                                                    let (h706, _) = v11;
                                                    h706
                                                }), v6)), (List::cons(v8, v2), v3))))
                                            }
                                            Err(v10) => {
                                                Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                            }
                                        }
                                    }
                                }
                                Adt17::C1(v7) => {
                                    Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                            }
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

pub fn f162(v0: List<Adt17>, v1: List<Adt17>) -> List<Adt17> {
    let m712 = v0;
    match m712.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f162(v3, v1))
        }
    }
}

pub fn f163(v0: List<(Bytes, Adt14)>) -> R<List<Adt17>> {
    let m713 = v0;
    match m713.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt17::C0(f106({
                let (h714, _) = v1;
                h714
            })?, {
                let (_, h715) = v1;
                h715
            }), f163(v2)?))
        }
    }
}

pub fn f164(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m716 = v0;
    match m716 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f215(v1)
        }
    }
}

pub fn f165(p719: List<Bytes>, p720: List<Bytes>) -> List<Bytes> {
    let m718 = (p719, p720);
    {
        let mut m718 = m718;
        loop {
            let step = {
                let (v0, v1) = m718;
                let m717 = v0;
                match m717.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Adt14) -> R<Bytes> {
    let m721 = v0;
    match m721 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f216(v2, v3)
        }
        Adt14::C2(v4) => {
            f217(v4)
        }
        Adt14::C3(r722) => {
            let v5 = (*r722).clone();
            Ok(f218(f219(v5)?))
        }
        Adt14::C4(r723) => {
            let v6 = (*r723).clone();
            f220(f221(v6)?)
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

pub fn f167(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m724 = v0;
    match m724.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f222(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f148(f223(f61(List::cons(v1, f224(f40(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f167(v3, v1)?))
            }
        }
    }
}

pub fn f168(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h725, _) = v2.clone();
        h725
    };
    let v4: u64 = {
        let (h726, _) = {
            let (_, h727) = v2.clone();
            h727
        };
        h726
    };
    let v5: List<Bytes> = {
        let (_, h728) = {
            let (_, h729) = v2;
            h729
        };
        h728
    };
    let m730 = f225(v0.clone(), v3);
    match m730 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a731 = v6;
            let a732 = v1;
            if equal(a731, a732) {
                Ok(Some(({
                    let a733 = v3;
                    let a734 = 1u64;
                    nat_add(a733, a734)?
                }, ({
                    let a735 = v3;
                    let a736 = 1u64;
                    nat_add(a735, a736)?
                }, List::cons(f42(v0, v4, {
                    let a737 = v3;
                    let a738 = v4;
                    nat_sub(a737, a738)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a739 = v3;
                    let a740 = 1u64;
                    nat_add(a739, a740)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f169(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f170(v0: Bytes) -> bool {
    f118(v0, f8(Str::lit(".lex.tex")))
}

pub fn f171(v0: Fn3, p745: u64, p746: u64) -> R<(u64, bool)> {
    let m744 = (p745, p746);
    {
        let mut m744 = m744;
        loop {
            let step = {
                let (v1, v2) = m744;
                let m741 = v1;
                if m741 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m741 - 1;
                    let m743 = {
                        let c742: Fn3 = v0.clone();
                        c742.apply(v2)?
                    };
                    match m743 {
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
                Ok(next) => { m744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f173(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a749 = {
            let a747 = v4;
            let a748 = v1;
            nat_add(a747, a748)?
        };
        let a750 = v3;
        if nat_lt(a749, a750) {
            Ok(Some({
                let a751 = v4;
                let a752 = 1u64;
                nat_add(a751, a752)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f173(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m757 = {
        let a754 = v0;
        let a755 = v2;
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
        Some(v3) => {
            let a758 = v3;
            let a759 = v1;
            equal(a758, a759)
        }
    }
}

pub fn f174(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m760 = v0;
    match m760.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f174(v3, {
                let a761 = v1;
                let a762 = 1u64;
                nat_add(a761, a762)?
            })?))
        }
    }
}

pub fn f175(v0: Adt15, v1: List<u64>, v2: Adt19) -> bool {
    let m763 = v2;
    match m763 {
        Adt19::C0(v3, v4) => {
            if f181(v3, match v0.clone() {
                Adt15::C0(h764, _, _, _) => {
                    h764
                }
            }) {
                f226(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f181(v5, match v0 {
                Adt15::C0(_, _, h765, _) => {
                    h765
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f177(v7, v1) {
                f181(v8, match v0 {
                    Adt15::C0(_, _, _, h766) => {
                        h766
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f176(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m767 = f100(v1.clone(), f8(Str::lit("std")));
    match m767 {
        None => {
            let m772 = f101(f100(v1.clone(), f8(Str::lit("req"))));
            match m772 {
                None => {
                    let m774 = f100(v1, f8(Str::lit("der")));
                    match m774 {
                        None => {
                            Ok(f227(v0))
                        }
                        Some(v5) => {
                            let m775 = f228(f98(v5.clone(), f8(Str::lit("rule"))));
                            match m775 {
                                None => {
                                    Ok(f227(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f128(f57(f97(f100(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h776) => {
                                            h776
                                        }
                                    })?, f179(f98(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h777, _) => {
                                            h777
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f179(v4, match v0 {
                        Adt12::C0(_, _, h773, _, _) => {
                            h773
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m768 = f101(f100(v2.clone(), f8(Str::lit("row"))));
            match m768 {
                None => {
                    Ok(Adt19::C0(f179(f98(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h769, _, _, _, _) => {
                            h769
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f179(f98(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h770, _, _, _, _) => {
                            h770
                        }
                    })?, Some(f179(v3, match v0 {
                        Adt12::C0(_, h771, _, _, _) => {
                            h771
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f177(p780: List<u64>, v1: List<u64>) -> bool {
    let m779 = p780;
    {
        let mut m779 = m779;
        loop {
            let step = {
                let v0 = m779;
                let m778 = v0;
                match m778.uncons() {
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
                Ok(next) => { m779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(p785: List<u64>) -> bool {
    let m784 = p785;
    {
        let mut m784 = m784;
        loop {
            let step = {
                let v0 = m784;
                let m781 = v0;
                match m781.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a782 = v1;
                        let a783 = 7u64;
                        if nat_le(a782, a783) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m786 = f229(v0, v1.clone(), 0u64)?;
    match m786 {
        None => {
            let a787 = v1;
            Ok(length_list(a787))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f180(v0: Adt14) -> u64 {
    let m788 = f230(f100(v0, f8(Str::lit("stratum"))));
    match m788 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f181(v0: u64, p793: List<u64>) -> bool {
    let m792 = p793;
    {
        let mut m792 = m792;
        loop {
            let step = {
                let v1 = m792;
                let m789 = v1;
                match m789.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a790 = v2;
                        let a791 = v0;
                        if nat_eq(a790, a791) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f222(v0.clone(), v1) {
        let m794 = f40(v0, 34u8)?;
        match m794.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m795 = v3;
                match m795.uncons() {
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
    let m796 = v0.clone();
    match m796.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a797 = v2;
            let a798 = v1.clone();
            if equal(a797, a798) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f185(v0: List<(Bytes, ())>) -> List<Bytes> {
    f132(f231(Fn5::F232, List::<Bytes>::nil(), v0))
}

pub fn f186(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f233(f234(Fn6::F235, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m803 = {
        let (h799, _) = f236(Fn7::F237, {
            let a801 = {
                let a800 = v0;
                length_list(a800)
            };
            let a802 = 1u64;
            nat_add(a801, a802)?
        }, v1)?;
        h799
    };
    match m803.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f187(v0: List<Bytes>) -> List<(Bytes, ())> {
    f238(f239(Fn8::F240, List::<(Bytes, ())>::nil(), v0))
}

pub fn f188(p806: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m805 = p806;
    {
        let mut m805 = m805;
        loop {
            let step = {
                let v0 = m805;
                let m804 = v0;
                match m804.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f199(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Adt14, v1: Bytes) -> Bytes {
    if f27(f208(f100(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f190(v0: Adt14) -> Bytes {
    if f206(v0.clone(), f8(Str::lit("req"))) {
        let a807 = f8(Str::lit("req "));
        let a808 = f98(v0, f8(Str::lit("req")));
        append_bytes(a807, a808)
    } else {
        let m809 = f100(v0.clone(), f8(Str::lit("std")));
        match m809 {
            None => {
                let m812 = f100(v0, f8(Str::lit("der")));
                match m812 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f98(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f87(f57(f97(f100(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f98(v1.clone(), f8(Str::lit("authority"))), List::cons(if f206(v1.clone(), f8(Str::lit("row"))) {
                    let a810 = f8(Str::lit(" row "));
                    let a811 = f98(v1, f8(Str::lit("row")));
                    append_bytes(a810, a811)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f191(v0: Bytes, v1: u64) -> R<u64> {
    let m815 = {
        let a813 = v0;
        let a814 = v1;
        index_bytes(a813, a814)
    };
    match m815 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a860 = {
                let a836 = {
                    let a824 = {
                        let a818 = {
                            let a816 = v3;
                            let a817 = 128u8;
                            fixed_u8::bit_and(a816, a817)
                        };
                        let a819 = 128u8;
                        if equal(a818, a819) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a825 = {
                        let a822 = {
                            let a820 = v3;
                            let a821 = 64u8;
                            fixed_u8::bit_and(a820, a821)
                        };
                        let a823 = 64u8;
                        if equal(a822, a823) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a824, a825)?
                };
                let a837 = {
                    let a834 = {
                        let a828 = {
                            let a826 = v3;
                            let a827 = 32u8;
                            fixed_u8::bit_and(a826, a827)
                        };
                        let a829 = 32u8;
                        if equal(a828, a829) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a835 = {
                        let a832 = {
                            let a830 = v3;
                            let a831 = 16u8;
                            fixed_u8::bit_and(a830, a831)
                        };
                        let a833 = 16u8;
                        if equal(a832, a833) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a834, a835)?
                };
                nat_add(a836, a837)?
            };
            let a861 = {
                let a858 = {
                    let a846 = {
                        let a840 = {
                            let a838 = v3;
                            let a839 = 8u8;
                            fixed_u8::bit_and(a838, a839)
                        };
                        let a841 = 8u8;
                        if equal(a840, a841) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a847 = {
                        let a844 = {
                            let a842 = v3;
                            let a843 = 4u8;
                            fixed_u8::bit_and(a842, a843)
                        };
                        let a845 = 4u8;
                        if equal(a844, a845) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a846, a847)?
                };
                let a859 = {
                    let a856 = {
                        let a850 = {
                            let a848 = v3;
                            let a849 = 2u8;
                            fixed_u8::bit_and(a848, a849)
                        };
                        let a851 = 2u8;
                        if equal(a850, a851) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a857 = {
                        let a854 = {
                            let a852 = v3;
                            let a853 = 1u8;
                            fixed_u8::bit_and(a852, a853)
                        };
                        let a855 = 1u8;
                        if equal(a854, a855) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a856, a857)?
                };
                nat_add(a858, a859)?
            };
            nat_add(a860, a861)
        }
    }
}

pub fn f192(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m865 = {
        let a862 = v1;
        let a863 = v2;
        let a864 = 1u64;
        slice_bytes(a862, a863, a864)
    };
    match m865 {
        None => {
            v0
        }
        Some(v3) => {
            let a866 = v0;
            let a867 = v3;
            append_bytes(a866, a867)
        }
    }
}

pub fn f193(p870: List<Bytes>, p871: List<Bytes>) -> List<Bytes> {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v0, v1) = m869;
                let m868 = v0;
                match m868.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes, p943: u64, p944: u64, p945: Bytes) -> R<Bytes> {
    let m942 = (p943, (p944, p945));
    {
        let mut m942 = m942;
        loop {
            let step = {
                let (v1, (v2, v3)) = m942;
                let m872 = v2;
                if m872 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m872 - 1;
                    let m875 = {
                        let a873 = v0.clone();
                        let a874 = v1;
                        index_bytes(a873, a874)
                    };
                    match m875 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a920 = {
                                    let a896 = {
                                        let a884 = {
                                            let a878 = {
                                                let a876 = v6;
                                                let a877 = 128u8;
                                                fixed_u8::bit_and(a876, a877)
                                            };
                                            let a879 = 128u8;
                                            if equal(a878, a879) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a885 = {
                                            let a882 = {
                                                let a880 = v6;
                                                let a881 = 64u8;
                                                fixed_u8::bit_and(a880, a881)
                                            };
                                            let a883 = 64u8;
                                            if equal(a882, a883) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a884, a885)?
                                    };
                                    let a897 = {
                                        let a894 = {
                                            let a888 = {
                                                let a886 = v6;
                                                let a887 = 32u8;
                                                fixed_u8::bit_and(a886, a887)
                                            };
                                            let a889 = 32u8;
                                            if equal(a888, a889) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a895 = {
                                            let a892 = {
                                                let a890 = v6;
                                                let a891 = 16u8;
                                                fixed_u8::bit_and(a890, a891)
                                            };
                                            let a893 = 16u8;
                                            if equal(a892, a893) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a894, a895)?
                                    };
                                    nat_add(a896, a897)?
                                };
                                let a921 = {
                                    let a918 = {
                                        let a906 = {
                                            let a900 = {
                                                let a898 = v6;
                                                let a899 = 8u8;
                                                fixed_u8::bit_and(a898, a899)
                                            };
                                            let a901 = 8u8;
                                            if equal(a900, a901) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a907 = {
                                            let a904 = {
                                                let a902 = v6;
                                                let a903 = 4u8;
                                                fixed_u8::bit_and(a902, a903)
                                            };
                                            let a905 = 4u8;
                                            if equal(a904, a905) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a906, a907)?
                                    };
                                    let a919 = {
                                        let a916 = {
                                            let a910 = {
                                                let a908 = v6;
                                                let a909 = 2u8;
                                                fixed_u8::bit_and(a908, a909)
                                            };
                                            let a911 = 2u8;
                                            if equal(a910, a911) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a917 = {
                                            let a914 = {
                                                let a912 = v6;
                                                let a913 = 1u8;
                                                fixed_u8::bit_and(a912, a913)
                                            };
                                            let a915 = 1u8;
                                            if equal(a914, a915) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a916, a917)?
                                    };
                                    nat_add(a918, a919)?
                                };
                                nat_add(a920, a921)?
                            };
                            let a922 = v7;
                            let a923 = 34u64;
                            if if nat_eq(a922, a923) {
                                true
                            } else {
                                let a924 = v7;
                                let a925 = 92u64;
                                nat_eq(a924, a925)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a926 = v1;
                                    let a927 = 1u64;
                                    nat_add(a926, a927)?
                                }, (v4, {
                                    let a930 = v3;
                                    let a931 = {
                                        let a928 = Bytes::lit(&[0x5c]);
                                        let a929 = f42(v0.clone(), v1, 1u64);
                                        append_bytes(a928, a929)
                                    };
                                    append_bytes(a930, a931)
                                })))
                            } else {
                                let a932 = v7;
                                let a933 = 10u64;
                                if nat_eq(a932, a933) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a934 = v1;
                                        let a935 = 1u64;
                                        nat_add(a934, a935)?
                                    }, (v4, {
                                        let a936 = v3;
                                        let a937 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a936, a937)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a938 = v1;
                                        let a939 = 1u64;
                                        nat_add(a938, a939)?
                                    }, (v4, {
                                        let a940 = v3;
                                        let a941 = f42(v0.clone(), v1, 1u64);
                                        append_bytes(a940, a941)
                                    })))
                                }
                            }
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

pub fn f195(v0: u32) -> Bytes {
    let a974 = f241({
        let a946 = f242(v0, 28u32);
        let a947 = 15u32;
        fixed_u32::bit_and(a946, a947)
    });
    let a975 = {
        let a972 = f241({
            let a948 = f242(v0, 24u32);
            let a949 = 15u32;
            fixed_u32::bit_and(a948, a949)
        });
        let a973 = {
            let a970 = f241({
                let a950 = f242(v0, 20u32);
                let a951 = 15u32;
                fixed_u32::bit_and(a950, a951)
            });
            let a971 = {
                let a968 = f241({
                    let a952 = f242(v0, 16u32);
                    let a953 = 15u32;
                    fixed_u32::bit_and(a952, a953)
                });
                let a969 = {
                    let a966 = f241({
                        let a954 = f242(v0, 12u32);
                        let a955 = 15u32;
                        fixed_u32::bit_and(a954, a955)
                    });
                    let a967 = {
                        let a964 = f241({
                            let a956 = f242(v0, 8u32);
                            let a957 = 15u32;
                            fixed_u32::bit_and(a956, a957)
                        });
                        let a965 = {
                            let a962 = f241({
                                let a958 = f242(v0, 4u32);
                                let a959 = 15u32;
                                fixed_u32::bit_and(a958, a959)
                            });
                            let a963 = f241({
                                let a960 = v0;
                                let a961 = 15u32;
                                fixed_u32::bit_and(a960, a961)
                            });
                            append_bytes(a962, a963)
                        };
                        append_bytes(a964, a965)
                    };
                    append_bytes(a966, a967)
                };
                append_bytes(a968, a969)
            };
            append_bytes(a970, a971)
        };
        append_bytes(a972, a973)
    };
    append_bytes(a974, a975)
}

pub fn f196(v0: Fn4, p979: u64, p980: (u64, Adt21)) -> R<(u64, Adt21)> {
    let m978 = (p979, p980);
    {
        let mut m978 = m978;
        loop {
            let step = {
                let (v1, v2) = m978;
                let m976 = v1;
                if m976 == 0 {
                    Err::<(u64, (u64, Adt21)), R<(u64, Adt21)>>(Ok(v2))
                } else {
                    let v3 = m976 - 1;
                    Ok::<(u64, (u64, Adt21)), R<(u64, Adt21)>>((v3, {
                        let c977: Fn4 = v0.clone();
                        c977.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt21)) -> R<(u64, Adt21)> {
    Ok(({
        let a982 = {
            let (h981, _) = v3.clone();
            h981
        };
        let a983 = 64u64;
        nat_add(a982, a983)?
    }, f243({
        let (_, h984) = v3;
        h984
    }, f244(v0, v1, v2, {
        let (h985, _) = v3;
        h985
    })?)?))
}

pub fn f198() -> Adt21 {
    Adt21::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f199(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f66(v1, f52(f148(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f200(v0: List<Bytes>) -> List<Bytes> {
    let m986 = v0;
    match m986.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f27(v1.clone(), f8(Str::lit("authority = []"))) {
                f200(v2)
            } else {
                List::cons(v1, f200(v2))
            }
        }
    }
}

pub fn f201(v0: Bytes) -> R<u64> {
    let (h987, _) = f171(Fn3::F245(v0.clone()), {
        let a988 = v0.clone();
        length_bytes(a988)
    }, {
        let a989 = v0;
        length_bytes(a989)
    })?;
    Ok(h987)
}

pub fn f202(v0: Bytes, v1: u64) -> R<u64> {
    let (h990, _) = f171(Fn3::F246(v1, v0), v1, 0u64)?;
    Ok(h990)
}

pub fn f203(v0: Bytes) -> R<Bytes> {
    f247(v0.clone(), 0u64, {
        let a991 = v0;
        length_bytes(a991)
    }, false, false, Bytes::lit(&[]))
}

pub fn f204(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m992 = v2;
    if m992 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m992 - 1;
        let v5: u64 = f248(v0.clone(), v1)?;
        let a993 = v3;
        let a994 = 1u64;
        if nat_eq(a993, a994) {
            let m995 = f204(v0.clone(), v1, v4, 0u64)?;
            match m995 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f248(v0.clone(), {
                        let (_, h996) = v6.clone();
                        h996
                    })?;
                    let a997 = v7;
                    let a998 = 44u64;
                    if nat_eq(a997, a998) {
                        let m1002 = f204(v0, {
                            let a1000 = {
                                let (_, h999) = v6;
                                h999
                            };
                            let a1001 = 1u64;
                            nat_add(a1000, a1001)?
                        }, v4, 1u64)?;
                        match m1002 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1003, _) = v6;
                                    h1003
                                }), std::rc::Rc::new({
                                    let (h1004, _) = v8;
                                    h1004
                                })), {
                                    let (_, h1005) = v8;
                                    h1005
                                })))
                            }
                        }
                    } else {
                        let a1006 = v7;
                        let a1007 = 93u64;
                        if nat_eq(a1006, a1007) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1008, _) = v6;
                                h1008
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1010 = {
                                    let (_, h1009) = v6;
                                    h1009
                                };
                                let a1011 = 1u64;
                                nat_add(a1010, a1011)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1012 = v3;
            let a1013 = 2u64;
            if nat_eq(a1012, a1013) {
                let m1016 = f249(v0.clone(), {
                    let a1014 = v1;
                    let a1015 = 1u64;
                    nat_add(a1014, a1015)?
                })?;
                match m1016 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1017 = v5;
                        let a1018 = 34u64;
                        if if nat_eq(a1017, a1018) {
                            let a1020 = f248(v0.clone(), {
                                let (h1019, _) = v9.clone();
                                h1019
                            })?;
                            let a1021 = 58u64;
                            nat_eq(a1020, a1021)
                        } else {
                            false
                        } {
                            let m1025 = f204(v0.clone(), {
                                let a1023 = {
                                    let (h1022, _) = v9;
                                    h1022
                                };
                                let a1024 = 1u64;
                                nat_add(a1023, a1024)?
                            }, v4, 0u64)?;
                            match m1025 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f248(v0.clone(), {
                                        let (_, h1026) = v10.clone();
                                        h1026
                                    })?;
                                    let a1027 = v11;
                                    let a1028 = 44u64;
                                    if nat_eq(a1027, a1028) {
                                        let m1032 = f204(v0, {
                                            let a1030 = {
                                                let (_, h1029) = v10;
                                                h1029
                                            };
                                            let a1031 = 1u64;
                                            nat_add(a1030, a1031)?
                                        }, v4, 2u64)?;
                                        match m1032 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1033) = v9;
                                                    h1033
                                                }, std::rc::Rc::new({
                                                    let (h1034, _) = v10;
                                                    h1034
                                                }), std::rc::Rc::new({
                                                    let (h1035, _) = v12;
                                                    h1035
                                                })), {
                                                    let (_, h1036) = v12;
                                                    h1036
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1037 = v11;
                                        let a1038 = 125u64;
                                        if nat_eq(a1037, a1038) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1039) = v9;
                                                h1039
                                            }, std::rc::Rc::new({
                                                let (h1040, _) = v10;
                                                h1040
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1042 = {
                                                    let (_, h1041) = v10;
                                                    h1041
                                                };
                                                let a1043 = 1u64;
                                                nat_add(a1042, a1043)?
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
                let a1044 = v5;
                let a1045 = 34u64;
                if nat_eq(a1044, a1045) {
                    let m1048 = f249(v0, {
                        let a1046 = v1;
                        let a1047 = 1u64;
                        nat_add(a1046, a1047)?
                    })?;
                    match m1048 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1049) = v13;
                                h1049
                            }), {
                                let (h1050, _) = v13;
                                h1050
                            })))
                        }
                    }
                } else {
                    if f27(f42(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1051 = v1;
                            let a1052 = 4u64;
                            nat_add(a1051, a1052)?
                        })))
                    } else {
                        if f27(f42(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1053 = v1;
                                let a1054 = 5u64;
                                nat_add(a1053, a1054)?
                            })))
                        } else {
                            let a1055 = v5;
                            let a1056 = 91u64;
                            if nat_eq(a1055, a1056) {
                                let a1059 = f248(v0.clone(), {
                                    let a1057 = v1;
                                    let a1058 = 1u64;
                                    nat_add(a1057, a1058)?
                                })?;
                                let a1060 = 93u64;
                                if nat_eq(a1059, a1060) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1061 = v1;
                                        let a1062 = 2u64;
                                        nat_add(a1061, a1062)?
                                    })))
                                } else {
                                    let m1065 = f204(v0, {
                                        let a1063 = v1;
                                        let a1064 = 1u64;
                                        nat_add(a1063, a1064)?
                                    }, v4, 1u64)?;
                                    match m1065 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1066, _) = v14;
                                                h1066
                                            })), {
                                                let (_, h1067) = v14;
                                                h1067
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1068 = v5;
                                let a1069 = 123u64;
                                if nat_eq(a1068, a1069) {
                                    let a1072 = f248(v0.clone(), {
                                        let a1070 = v1;
                                        let a1071 = 1u64;
                                        nat_add(a1070, a1071)?
                                    })?;
                                    let a1073 = 125u64;
                                    if nat_eq(a1072, a1073) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1074 = v1;
                                            let a1075 = 2u64;
                                            nat_add(a1074, a1075)?
                                        })))
                                    } else {
                                        let m1078 = f204(v0, {
                                            let a1076 = v1;
                                            let a1077 = 1u64;
                                            nat_add(a1076, a1077)?
                                        }, v4, 2u64)?;
                                        match m1078 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1079, _) = v15;
                                                    h1079
                                                })), {
                                                    let (_, h1080) = v15;
                                                    h1080
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1081 = f250(v0, v1)?;
                                    match m1081 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1082) = v16;
                                                h1082
                                            }, {
                                                let (h1083, _) = v16;
                                                h1083
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

pub fn f205(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1084, _) = v1.clone();
        h1084
    };
    let v3: u64 = {
        let (h1085, _) = {
            let (_, h1086) = v1.clone();
            h1086
        };
        h1085
    };
    let v4: List<Bytes> = {
        let (_, h1087) = {
            let (_, h1088) = v1;
            h1088
        };
        h1087
    };
    let m1089 = f225(v0.clone(), v2);
    match m1089 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1090 = v5;
            let a1091 = 10u8;
            if equal(a1090, a1091) {
                Ok(Some(({
                    let a1092 = v2;
                    let a1093 = 1u64;
                    nat_add(a1092, a1093)?
                }, ({
                    let a1094 = v2;
                    let a1095 = 1u64;
                    nat_add(a1094, a1095)?
                }, List::cons(f251(f42(v0, v3, {
                    let a1096 = v2;
                    let a1097 = v3;
                    nat_sub(a1096, a1097)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1098 = v2;
                    let a1099 = 1u64;
                    nat_add(a1098, a1099)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f206(v0: Adt14, v1: Bytes) -> bool {
    f58({
        let a1101 = {
            let a1100 = f98(v0, v1);
            length_bytes(a1100)
        };
        let a1102 = 0u64;
        nat_eq(a1101, a1102)
    })
}

pub fn f207(v0: Adt14, v1: Bytes) -> bool {
    let m1103 = f100(v0, v1);
    match m1103 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f208(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1104 = v0;
    match m1104 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f98(v2, v1)
        }
    }
}

pub fn f209(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f148(f223(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f210(v0: u64) -> bool {
    let a1105 = v0;
    let a1106 = 95u64;
    if nat_eq(a1105, a1106) {
        true
    } else {
        let a1107 = v0;
        let a1108 = 45u64;
        if nat_eq(a1107, a1108) {
            true
        } else {
            let a1109 = v0;
            let a1110 = 32u64;
            if nat_eq(a1109, a1110) {
                true
            } else {
                let a1111 = v0;
                let a1112 = 46u64;
                if nat_eq(a1111, a1112) {
                    true
                } else {
                    let a1113 = v0;
                    let a1114 = 47u64;
                    nat_eq(a1113, a1114)
                }
            }
        }
    }
}

pub fn f211(v0: u64) -> bool {
    let a1115 = 97u64;
    let a1116 = v0;
    if nat_le(a1115, a1116) {
        let a1117 = v0;
        let a1118 = 122u64;
        nat_le(a1117, a1118)
    } else {
        false
    }
}

pub fn f212(v0: u64) -> bool {
    let a1119 = 65u64;
    let a1120 = v0;
    if nat_le(a1119, a1120) {
        let a1121 = v0;
        let a1122 = 90u64;
        nat_le(a1121, a1122)
    } else {
        false
    }
}

pub fn f213(v0: u64) -> bool {
    let a1123 = 48u64;
    let a1124 = v0;
    if nat_le(a1123, a1124) {
        let a1125 = v0;
        let a1126 = 57u64;
        nat_le(a1125, a1126)
    } else {
        false
    }
}

pub fn f214(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1127 = f100(v1.clone(), f8(Str::lit("enum")));
    match m1127 {
        None => {
            let m1129 = f255(v0.clone(), f164(f100(v1.clone(), f8(Str::lit("properties")))), f57(f97(f100(v1, f8(Str::lit("required"))))))?;
            match m1129 {
                Ok(v6) => {
                    Ok(f254(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1131 = {
                            let (h1130, _) = v6.clone();
                            h1130
                        };
                        match m1131.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f148({
                                    let (h1132, _) = v6;
                                    h1132
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1133) = v6;
                        h1133
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt17>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1128 = f252(f57(f158(v2)))?;
            match m1128 {
                Ok(v4) => {
                    Ok(f254(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f148(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt17>::nil()))
                }
                Err(v3) => {
                    Ok(f253(v3, v0))
                }
            }
        }
    }
}

pub fn f215(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1134 = v0;
    match m1134 {
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
        Adt14::C4(r1135) => {
            let v4 = (*r1135).clone();
            f215(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1136, r1137) => {
            let v2 = (*r1136).clone();
            let v3 = (*r1137).clone();
            List::cons((v1, v2), f215(v3))
        }
    }
}

pub fn f216(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1138 = 0u64;
        let a1139 = v1;
        nat_lt(a1138, a1139)
    } else {
        false
    } {
        let a1140 = Bytes::lit(&[0x2d]);
        let a1141 = f256(v1)?;
        Ok(append_bytes(a1140, a1141))
    } else {
        f256(v1)
    }
}

pub fn f217(v0: Bytes) -> R<Bytes> {
    let a1147 = {
        let a1145 = Bytes::lit(&[0x22]);
        let a1146 = {
            let (_, h1142) = {
                let (h1143, _) = f257(Fn9::F258(v0.clone()), {
                    let a1144 = v0;
                    length_bytes(a1144)
                }, (0u64, Bytes::lit(&[])))?;
                h1143
            };
            h1142
        };
        append_bytes(a1145, a1146)
    };
    let a1148 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1147, a1148))
}

pub fn f218(v0: List<Bytes>) -> Bytes {
    let a1151 = {
        let a1149 = Bytes::lit(&[0x5b]);
        let a1150 = f87(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1149, a1150)
    };
    let a1152 = Bytes::lit(&[0x5d]);
    append_bytes(a1151, a1152)
}

pub fn f219(v0: Adt14) -> R<List<Bytes>> {
    let m1153 = v0;
    match m1153 {
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
        Adt14::C6(r1154, r1155) => {
            let v7 = (*r1154).clone();
            let v8 = (*r1155).clone();
            Ok(List::cons(f166(v7)?, f219(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f220(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f259(Fn10::F260, List::<(Bytes, Bytes)>::nil(), f261(v0)?);
    let v2: List<Bytes> = f262(Fn11::F263, List::<Bytes>::nil(), v1)?;
    let a1158 = {
        let a1156 = Bytes::lit(&[0x7b]);
        let a1157 = f87(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1156, a1157)
    };
    let a1159 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1158, a1159))
}

pub fn f221(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1160 = v0;
    match m1160 {
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
        Adt14::C7(v9, r1161, r1162) => {
            let v10 = (*r1161).clone();
            let v11 = (*r1162).clone();
            Ok(List::cons((v9, f166(v10)?), f221(v11)?))
        }
    }
}

pub fn f222(v0: Bytes, v1: Bytes) -> bool {
    let m1167 = {
        let a1164 = v0;
        let a1165 = 0u64;
        let a1166 = {
            let a1163 = v1.clone();
            length_bytes(a1163)
        };
        slice_bytes(a1164, a1165, a1166)
    };
    match m1167 {
        None => {
            false
        }
        Some(v2) => {
            let a1168 = v2;
            let a1169 = v1;
            equal(a1168, a1169)
        }
    }
}

pub fn f223(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1170 = v0;
    match m1170.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f142(v1)?, f223(v2)?))
        }
    }
}

pub fn f224(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1171 = v0;
    match m1171.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f224(v3, false))
            } else {
                f224(v3, true)
            }
        }
    }
}

pub fn f225(v0: Bytes, v1: u64) -> Option<u8> {
    let a1172 = v0;
    let a1173 = v1;
    index_bytes(a1172, a1173)
}

pub fn f226(v0: Adt15, v1: Option<u64>) -> bool {
    let m1174 = v1;
    match m1174 {
        None => {
            true
        }
        Some(v2) => {
            f181(v2, match v0 {
                Adt15::C0(_, h1175, _, _) => {
                    h1175
                }
            })
        }
    }
}

pub fn f227(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1177 = match v0 {
            Adt12::C0(_, _, h1176, _, _) => {
                h1176
            }
        };
        length_list(a1177)
    })
}

pub fn f228(v0: Bytes) -> Option<Adt20> {
    if f27(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt20::C0)
    } else {
        if f27(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt20::C1)
        } else {
            if f27(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt20::C2)
            } else {
                None::<Adt20>
            }
        }
    }
}

pub fn f229(v0: Bytes, p1182: List<Bytes>, p1183: u64) -> R<Option<u64>> {
    let m1181 = (p1182, p1183);
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let (v1, v2) = m1181;
                let m1178 = v1;
                match m1178.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f27(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1179 = v2;
                                let a1180 = 1u64;
                                nat_add(a1179, a1180)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1181 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: Option<Adt14>) -> Option<u64> {
    let m1184 = v0;
    match m1184 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1185 = v1;
            match m1185 {
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

pub fn f231(v0: Fn5, p1189: List<Bytes>, p1190: List<(Bytes, ())>) -> List<Bytes> {
    let m1188 = (p1189, p1190);
    {
        let mut m1188 = m1188;
        loop {
            let step = {
                let (v1, v2) = m1188;
                let m1186 = v2;
                match m1186.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1187: Fn5 = v0.clone();
                            c1187.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1191, _) = v1;
        h1191
    }, v0)
}

pub fn f233(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f264(Fn12::F265, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f234(v0: Fn6, p1195: List<List<(Bytes, ())>>, p1196: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1194 = (p1195, p1196);
    {
        let mut m1194 = m1194;
        loop {
            let step = {
                let (v1, v2) = m1194;
                let m1192 = v2;
                match m1192.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1193: Fn6 = v0.clone();
                            c1193.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f236(v0: Fn7, p1201: u64, p1202: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1200 = (p1201, p1202);
    {
        let mut m1200 = m1200;
        loop {
            let step = {
                let (v1, v2) = m1200;
                let m1197 = v1;
                if m1197 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1197 - 1;
                    let m1199 = {
                        let c1198: Fn7 = v0.clone();
                        c1198.apply(v2.clone())?
                    };
                    match m1199 {
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
                Ok(next) => { m1200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f266(v0)
}

pub fn f238(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f267(Fn13::F268, List::<(Bytes, ())>::nil(), v0)
}

pub fn f239(v0: Fn8, p1206: List<(Bytes, ())>, p1207: List<Bytes>) -> List<(Bytes, ())> {
    let m1205 = (p1206, p1207);
    {
        let mut m1205 = m1205;
        loop {
            let step = {
                let (v1, v2) = m1205;
                let m1203 = v2;
                match m1203.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1204: Fn8 = v0.clone();
                            c1204.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f241(v0: u32) -> Bytes {
    let a1210 = {
        let a1208 = v0;
        let a1209 = 8u32;
        fixed_u32::bit_and(a1208, a1209)
    };
    let a1211 = 0u32;
    if equal(a1210, a1211) {
        let a1214 = {
            let a1212 = v0;
            let a1213 = 4u32;
            fixed_u32::bit_and(a1212, a1213)
        };
        let a1215 = 0u32;
        if equal(a1214, a1215) {
            let a1218 = {
                let a1216 = v0;
                let a1217 = 2u32;
                fixed_u32::bit_and(a1216, a1217)
            };
            let a1219 = 0u32;
            if equal(a1218, a1219) {
                let a1222 = {
                    let a1220 = v0;
                    let a1221 = 1u32;
                    fixed_u32::bit_and(a1220, a1221)
                };
                let a1223 = 0u32;
                if equal(a1222, a1223) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1226 = {
                    let a1224 = v0;
                    let a1225 = 1u32;
                    fixed_u32::bit_and(a1224, a1225)
                };
                let a1227 = 0u32;
                if equal(a1226, a1227) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1230 = {
                let a1228 = v0;
                let a1229 = 2u32;
                fixed_u32::bit_and(a1228, a1229)
            };
            let a1231 = 0u32;
            if equal(a1230, a1231) {
                let a1234 = {
                    let a1232 = v0;
                    let a1233 = 1u32;
                    fixed_u32::bit_and(a1232, a1233)
                };
                let a1235 = 0u32;
                if equal(a1234, a1235) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1238 = {
                    let a1236 = v0;
                    let a1237 = 1u32;
                    fixed_u32::bit_and(a1236, a1237)
                };
                let a1239 = 0u32;
                if equal(a1238, a1239) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1242 = {
            let a1240 = v0;
            let a1241 = 4u32;
            fixed_u32::bit_and(a1240, a1241)
        };
        let a1243 = 0u32;
        if equal(a1242, a1243) {
            let a1246 = {
                let a1244 = v0;
                let a1245 = 2u32;
                fixed_u32::bit_and(a1244, a1245)
            };
            let a1247 = 0u32;
            if equal(a1246, a1247) {
                let a1250 = {
                    let a1248 = v0;
                    let a1249 = 1u32;
                    fixed_u32::bit_and(a1248, a1249)
                };
                let a1251 = 0u32;
                if equal(a1250, a1251) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1254 = {
                    let a1252 = v0;
                    let a1253 = 1u32;
                    fixed_u32::bit_and(a1252, a1253)
                };
                let a1255 = 0u32;
                if equal(a1254, a1255) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1258 = {
                let a1256 = v0;
                let a1257 = 2u32;
                fixed_u32::bit_and(a1256, a1257)
            };
            let a1259 = 0u32;
            if equal(a1258, a1259) {
                let a1262 = {
                    let a1260 = v0;
                    let a1261 = 1u32;
                    fixed_u32::bit_and(a1260, a1261)
                };
                let a1263 = 0u32;
                if equal(a1262, a1263) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1266 = {
                    let a1264 = v0;
                    let a1265 = 1u32;
                    fixed_u32::bit_and(a1264, a1265)
                };
                let a1267 = 0u32;
                if equal(a1266, a1267) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f242(v0: u32, v1: u32) -> u32 {
    let m1270 = {
        let a1268 = v0;
        let a1269 = v1;
        fixed_u32::shift_right(a1268, a1269)
    };
    match m1270 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f243(v0: Adt21, v1: Adt22) -> R<Adt21> {
    let v2: Adt23 = f269(Fn14::F270, 64u64, Adt23::C0(match v0.clone() {
        Adt21::C0(h1271, _, _, _, _, _, _, _) => {
            h1271
        }
    }, match v0.clone() {
        Adt21::C0(_, h1272, _, _, _, _, _, _) => {
            h1272
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h1273, _, _, _, _, _) => {
            h1273
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, h1274, _, _, _, _) => {
            h1274
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, h1275, _, _, _) => {
            h1275
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1276, _, _) => {
            h1276
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1277, _) => {
            h1277
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, h1278) => {
            h1278
        }
    }, v1, 0u64))?;
    Ok(Adt21::C0(f271(match v0.clone() {
        Adt21::C0(h1279, _, _, _, _, _, _, _) => {
            h1279
        }
    }, match v2.clone() {
        Adt23::C0(h1280, _, _, _, _, _, _, _, _, _) => {
            h1280
        }
    }), f271(match v0.clone() {
        Adt21::C0(_, h1281, _, _, _, _, _, _) => {
            h1281
        }
    }, match v2.clone() {
        Adt23::C0(_, h1282, _, _, _, _, _, _, _, _) => {
            h1282
        }
    }), f271(match v0.clone() {
        Adt21::C0(_, _, h1283, _, _, _, _, _) => {
            h1283
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1284, _, _, _, _, _, _, _) => {
            h1284
        }
    }), f271(match v0.clone() {
        Adt21::C0(_, _, _, h1285, _, _, _, _) => {
            h1285
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1286, _, _, _, _, _, _) => {
            h1286
        }
    }), f271(match v0.clone() {
        Adt21::C0(_, _, _, _, h1287, _, _, _) => {
            h1287
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1288, _, _, _, _, _) => {
            h1288
        }
    }), f271(match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1289, _, _) => {
            h1289
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1290, _, _, _, _) => {
            h1290
        }
    }), f271(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1291, _) => {
            h1291
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1292, _, _, _) => {
            h1292
        }
    }), f271(match v0 {
        Adt21::C0(_, _, _, _, _, _, _, h1293) => {
            h1293
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1294, _, _) => {
            h1294
        }
    })))
}

pub fn f244(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f272(v0.clone(), v1, v2, v3)?, f272(v0.clone(), v1, v2, {
        let a1295 = v3;
        let a1296 = 4u64;
        nat_add(a1295, a1296)?
    })?, f272(v0.clone(), v1, v2, {
        let a1297 = v3;
        let a1298 = 8u64;
        nat_add(a1297, a1298)?
    })?, f272(v0.clone(), v1, v2, {
        let a1299 = v3;
        let a1300 = 12u64;
        nat_add(a1299, a1300)?
    })?, f272(v0.clone(), v1, v2, {
        let a1301 = v3;
        let a1302 = 16u64;
        nat_add(a1301, a1302)?
    })?, f272(v0.clone(), v1, v2, {
        let a1303 = v3;
        let a1304 = 20u64;
        nat_add(a1303, a1304)?
    })?, f272(v0.clone(), v1, v2, {
        let a1305 = v3;
        let a1306 = 24u64;
        nat_add(a1305, a1306)?
    })?, f272(v0.clone(), v1, v2, {
        let a1307 = v3;
        let a1308 = 28u64;
        nat_add(a1307, a1308)?
    })?, f272(v0.clone(), v1, v2, {
        let a1309 = v3;
        let a1310 = 32u64;
        nat_add(a1309, a1310)?
    })?, f272(v0.clone(), v1, v2, {
        let a1311 = v3;
        let a1312 = 36u64;
        nat_add(a1311, a1312)?
    })?, f272(v0.clone(), v1, v2, {
        let a1313 = v3;
        let a1314 = 40u64;
        nat_add(a1313, a1314)?
    })?, f272(v0.clone(), v1, v2, {
        let a1315 = v3;
        let a1316 = 44u64;
        nat_add(a1315, a1316)?
    })?, f272(v0.clone(), v1, v2, {
        let a1317 = v3;
        let a1318 = 48u64;
        nat_add(a1317, a1318)?
    })?, f272(v0.clone(), v1, v2, {
        let a1319 = v3;
        let a1320 = 52u64;
        nat_add(a1319, a1320)?
    })?, f272(v0.clone(), v1, v2, {
        let a1321 = v3;
        let a1322 = 56u64;
        nat_add(a1321, a1322)?
    })?, f272(v0, v1, v2, {
        let a1323 = v3;
        let a1324 = 60u64;
        nat_add(a1323, a1324)?
    })?))
}

pub fn f245(v0: Bytes, v1: u64) -> Option<u64> {
    let a1325 = 0u64;
    let a1326 = v1;
    if nat_lt(a1325, a1326) {
        if f273(v0, {
            let a1327 = v1;
            let a1328 = 1u64;
            nat_sub(a1327, a1328)
        }) {
            Some({
                let a1329 = v1;
                let a1330 = 1u64;
                nat_sub(a1329, a1330)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f246(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1331 = v2;
    let a1332 = v0;
    if nat_lt(a1331, a1332) {
        if f273(v1, v2) {
            Ok(Some({
                let a1333 = v2;
                let a1334 = 1u64;
                nat_add(a1333, a1334)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f247(v0: Bytes, p1414: u64, p1415: u64, p1416: bool, p1417: bool, p1418: Bytes) -> R<Bytes> {
    let m1413 = (p1414, (p1415, (p1416, (p1417, p1418))));
    {
        let mut m1413 = m1413;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1413;
                let m1335 = v2;
                if m1335 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1335 - 1;
                    let m1338 = {
                        let a1336 = v0.clone();
                        let a1337 = v1;
                        index_bytes(a1336, a1337)
                    };
                    match m1338 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1383 = {
                                    let a1359 = {
                                        let a1347 = {
                                            let a1341 = {
                                                let a1339 = v8;
                                                let a1340 = 128u8;
                                                fixed_u8::bit_and(a1339, a1340)
                                            };
                                            let a1342 = 128u8;
                                            if equal(a1341, a1342) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1348 = {
                                            let a1345 = {
                                                let a1343 = v8;
                                                let a1344 = 64u8;
                                                fixed_u8::bit_and(a1343, a1344)
                                            };
                                            let a1346 = 64u8;
                                            if equal(a1345, a1346) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1347, a1348)?
                                    };
                                    let a1360 = {
                                        let a1357 = {
                                            let a1351 = {
                                                let a1349 = v8;
                                                let a1350 = 32u8;
                                                fixed_u8::bit_and(a1349, a1350)
                                            };
                                            let a1352 = 32u8;
                                            if equal(a1351, a1352) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1358 = {
                                            let a1355 = {
                                                let a1353 = v8;
                                                let a1354 = 16u8;
                                                fixed_u8::bit_and(a1353, a1354)
                                            };
                                            let a1356 = 16u8;
                                            if equal(a1355, a1356) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1357, a1358)?
                                    };
                                    nat_add(a1359, a1360)?
                                };
                                let a1384 = {
                                    let a1381 = {
                                        let a1369 = {
                                            let a1363 = {
                                                let a1361 = v8;
                                                let a1362 = 8u8;
                                                fixed_u8::bit_and(a1361, a1362)
                                            };
                                            let a1364 = 8u8;
                                            if equal(a1363, a1364) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1370 = {
                                            let a1367 = {
                                                let a1365 = v8;
                                                let a1366 = 4u8;
                                                fixed_u8::bit_and(a1365, a1366)
                                            };
                                            let a1368 = 4u8;
                                            if equal(a1367, a1368) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1369, a1370)?
                                    };
                                    let a1382 = {
                                        let a1379 = {
                                            let a1373 = {
                                                let a1371 = v8;
                                                let a1372 = 2u8;
                                                fixed_u8::bit_and(a1371, a1372)
                                            };
                                            let a1374 = 2u8;
                                            if equal(a1373, a1374) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1380 = {
                                            let a1377 = {
                                                let a1375 = v8;
                                                let a1376 = 1u8;
                                                fixed_u8::bit_and(a1375, a1376)
                                            };
                                            let a1378 = 1u8;
                                            if equal(a1377, a1378) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1379, a1380)?
                                    };
                                    nat_add(a1381, a1382)?
                                };
                                nat_add(a1383, a1384)?
                            };
                            let v10: Bytes = f42(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1385 = v1;
                                    let a1386 = 1u64;
                                    nat_add(a1385, a1386)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1387 = v9;
                                    let a1388 = 34u64;
                                    !nat_eq(a1387, a1388)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1389 = v9;
                                    let a1390 = 92u64;
                                    nat_eq(a1389, a1390)
                                }, {
                                    let a1391 = v5;
                                    let a1392 = v10;
                                    append_bytes(a1391, a1392)
                                })))))
                            } else {
                                let a1393 = v9;
                                let a1394 = 32u64;
                                if if if if nat_eq(a1393, a1394) {
                                    true
                                } else {
                                    let a1395 = v9;
                                    let a1396 = 9u64;
                                    nat_eq(a1395, a1396)
                                } {
                                    true
                                } else {
                                    let a1397 = v9;
                                    let a1398 = 10u64;
                                    nat_eq(a1397, a1398)
                                } {
                                    true
                                } else {
                                    let a1399 = v9;
                                    let a1400 = 13u64;
                                    nat_eq(a1399, a1400)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1401 = v1;
                                        let a1402 = 1u64;
                                        nat_add(a1401, a1402)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f27(f42(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1403 = v1;
                                            let a1404 = 4u64;
                                            nat_add(a1403, a1404)?
                                        }, (v6, (false, (false, {
                                            let a1405 = v5;
                                            let a1406 = f8(Str::lit("[]"));
                                            append_bytes(a1405, a1406)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1407 = v1;
                                            let a1408 = 1u64;
                                            nat_add(a1407, a1408)?
                                        }, (v6, ({
                                            let a1409 = v9;
                                            let a1410 = 34u64;
                                            nat_eq(a1409, a1410)
                                        }, (false, {
                                            let a1411 = v5;
                                            let a1412 = v10;
                                            append_bytes(a1411, a1412)
                                        })))))
                                    }
                                }
                            }
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

pub fn f248(v0: Bytes, v1: u64) -> R<u64> {
    let m1421 = {
        let a1419 = v0;
        let a1420 = v1;
        index_bytes(a1419, a1420)
    };
    match m1421 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1466 = {
                let a1442 = {
                    let a1430 = {
                        let a1424 = {
                            let a1422 = v3;
                            let a1423 = 128u8;
                            fixed_u8::bit_and(a1422, a1423)
                        };
                        let a1425 = 128u8;
                        if equal(a1424, a1425) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1431 = {
                        let a1428 = {
                            let a1426 = v3;
                            let a1427 = 64u8;
                            fixed_u8::bit_and(a1426, a1427)
                        };
                        let a1429 = 64u8;
                        if equal(a1428, a1429) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1430, a1431)?
                };
                let a1443 = {
                    let a1440 = {
                        let a1434 = {
                            let a1432 = v3;
                            let a1433 = 32u8;
                            fixed_u8::bit_and(a1432, a1433)
                        };
                        let a1435 = 32u8;
                        if equal(a1434, a1435) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1441 = {
                        let a1438 = {
                            let a1436 = v3;
                            let a1437 = 16u8;
                            fixed_u8::bit_and(a1436, a1437)
                        };
                        let a1439 = 16u8;
                        if equal(a1438, a1439) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1440, a1441)?
                };
                nat_add(a1442, a1443)?
            };
            let a1467 = {
                let a1464 = {
                    let a1452 = {
                        let a1446 = {
                            let a1444 = v3;
                            let a1445 = 8u8;
                            fixed_u8::bit_and(a1444, a1445)
                        };
                        let a1447 = 8u8;
                        if equal(a1446, a1447) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1453 = {
                        let a1450 = {
                            let a1448 = v3;
                            let a1449 = 4u8;
                            fixed_u8::bit_and(a1448, a1449)
                        };
                        let a1451 = 4u8;
                        if equal(a1450, a1451) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1452, a1453)?
                };
                let a1465 = {
                    let a1462 = {
                        let a1456 = {
                            let a1454 = v3;
                            let a1455 = 2u8;
                            fixed_u8::bit_and(a1454, a1455)
                        };
                        let a1457 = 2u8;
                        if equal(a1456, a1457) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1463 = {
                        let a1460 = {
                            let a1458 = v3;
                            let a1459 = 1u8;
                            fixed_u8::bit_and(a1458, a1459)
                        };
                        let a1461 = 1u8;
                        if equal(a1460, a1461) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1462, a1463)?
                };
                nat_add(a1464, a1465)?
            };
            nat_add(a1466, a1467)
        }
    }
}

pub fn f249(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f274(Fn15::F275(v0.clone()), {
        let a1469 = {
            let a1468 = v0;
            length_bytes(a1468)
        };
        let a1470 = 1u64;
        nat_add(a1469, a1470)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1474 = {
        let (_, h1471) = {
            let (_, h1472) = {
                let (h1473, _) = v2.clone();
                h1473
            };
            h1472
        };
        h1471
    };
    let a1475 = 1u64;
    if nat_eq(a1474, a1475) {
        Ok(Some(({
            let (h1476, _) = {
                let (h1477, _) = v2.clone();
                h1477
            };
            h1476
        }, {
            let (h1478, _) = {
                let (_, h1479) = {
                    let (h1480, _) = v2;
                    h1480
                };
                h1479
            };
            h1478
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1481 = f248(v0.clone(), v1)?;
        let a1482 = 45u64;
        nat_eq(a1481, a1482)
    };
    let v3: u64 = if v2 {
        let a1483 = v1;
        let a1484 = 1u64;
        nat_add(a1483, a1484)?
    } else {
        v1
    };
    let v4: u64 = f276(v0.clone(), v3)?;
    let a1487 = 19u64;
    let a1488 = {
        let a1485 = v4;
        let a1486 = v3;
        nat_sub(a1485, a1486)
    };
    if nat_lt(a1487, a1488) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1491 = f277(f42(v0, v3, {
            let a1489 = v4;
            let a1490 = v3;
            nat_sub(a1489, a1490)
        }))?;
        match m1491 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f278(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f251(v0: Bytes) -> Bytes {
    if f118(v0.clone(), Bytes::lit(&[0x0d])) {
        f42(v0.clone(), 0u64, {
            let a1493 = {
                let a1492 = v0;
                length_bytes(a1492)
            };
            let a1494 = 1u64;
            nat_sub(a1493, a1494)
        })
    } else {
        v0
    }
}

pub fn f252(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1495 = v0;
    match m1495.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f279(v1.clone())?;
            if f280(v3.clone())? {
                let m1496 = f252(v2)?;
                match m1496 {
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

pub fn f253(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt17>), Bytes> {
    Err::<(Bytes, List<Adt17>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f254(v0: Bytes, v1: List<Adt17>) -> Result<(Bytes, List<Adt17>), Bytes> {
    Ok::<(Bytes, List<Adt17>), Bytes>((v0, v1))
}

pub fn f255(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt17>), Bytes>> {
    let m1497 = v1;
    match m1497.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::<Bytes>::nil(), List::<Adt17>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f279({
                let (h1498, _) = v3.clone();
                h1498
            })?;
            if f280(v5.clone())? {
                let m1503 = f281({
                    let (_, h1499) = v3.clone();
                    h1499
                }, {
                    let a1501 = v0.clone();
                    let a1502 = f106({
                        let (h1500, _) = v3.clone();
                        h1500
                    })?;
                    append_bytes(a1501, a1502)
                })?;
                match m1503 {
                    Ok(v7) => {
                        let v8: Bytes = if f66({
                            let (h1504, _) = v3;
                            h1504
                        }, v2.clone()) {
                            let (h1505, _) = v7;
                            h1505
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1506, _) = v7;
                                h1506
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1507 = f255(v0, v4, v2)?;
                        match m1507 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1508, _) = v10;
                                    h1508
                                }), f162({
                                    let (_, h1509) = v7;
                                    h1509
                                }, {
                                    let (_, h1510) = v10;
                                    h1510
                                }))))
                            }
                            Err(v9) => {
                                Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(v9))
                            }
                        }
                    }
                    Err(v6) => {
                        Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(v6))
                    }
                }
            } else {
                Ok(Err::<(List<Bytes>, List<Adt17>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                    let (h1511, _) = v3;
                    h1511
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f256(v0: u64) -> R<Bytes> {
    f282(f283(), 10u64, v0)
}

pub fn f257(v0: Fn9, p1516: u64, p1517: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1515 = (p1516, p1517);
    {
        let mut m1515 = m1515;
        loop {
            let step = {
                let (v1, v2) = m1515;
                let m1512 = v1;
                if m1512 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1512 - 1;
                    let m1514 = {
                        let c1513: Fn9 = v0.clone();
                        c1513.apply(v2.clone())?
                    };
                    match m1514 {
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
                Ok(next) => { m1515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f258(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f284(v0, v1)
}

pub fn f259(v0: Fn10, p1521: List<(Bytes, Bytes)>, p1522: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1520 = (p1521, p1522);
    {
        let mut m1520 = m1520;
        loop {
            let step = {
                let (v1, v2) = m1520;
                let m1518 = v2;
                match m1518.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1519: Fn10 = v0.clone();
                            c1519.apply(v1, v3)
                        }, v4))
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

pub fn f260(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f285(v0, v1)
}

pub fn f261(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f286(v0)
}

pub fn f262(v0: Fn11, p1526: List<Bytes>, p1527: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1525 = (p1526, p1527);
    {
        let mut m1525 = m1525;
        loop {
            let step = {
                let (v1, v2) = m1525;
                let m1523 = v2;
                match m1523.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1524: Fn11 = v0.clone();
                            c1524.apply(v1, v3)?
                        }, v4))
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

pub fn f263(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f287(v1)?, v0))
}

pub fn f264(v0: Fn12, p1531: List<List<(Bytes, ())>>, p1532: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v1, v2) = m1530;
                let m1528 = v2;
                match m1528.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1529: Fn12 = v0.clone();
                            c1529.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f265(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f266(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1534 = {
        let a1533 = v0.clone();
        length_list(a1533)
    };
    let a1535 = 1u64;
    if nat_le(a1534, a1535) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f288(v0)?))
    }
}

pub fn f267(v0: Fn13, p1539: List<(Bytes, ())>, p1540: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1538 = (p1539, p1540);
    {
        let mut m1538 = m1538;
        loop {
            let step = {
                let (v1, v2) = m1538;
                let m1536 = v2;
                match m1536.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1537: Fn13 = v0.clone();
                            c1537.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f268(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f269(v0: Fn14, p1544: u64, p1545: Adt23) -> R<Adt23> {
    let m1543 = (p1544, p1545);
    {
        let mut m1543 = m1543;
        loop {
            let step = {
                let (v1, v2) = m1543;
                let m1541 = v1;
                if m1541 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1541 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1542: Fn14 = v0.clone();
                        c1542.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Adt23) -> R<Adt23> {
    f289(v0)
}

pub fn f271(v0: u32, v1: u32) -> u32 {
    let m1548 = {
        let a1546 = f290(v0);
        let a1547 = f290(v1);
        fixed_u64::checked_add(a1546, a1547)
    };
    match m1548 {
        None => {
            0u32
        }
        Some(v2) => {
            f291(v2)
        }
    }
}

pub fn f272(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1551 = {
        let a1549 = v3;
        let a1550 = 8u64;
        nat_add(a1549, a1550)?
    };
    let a1552 = v2;
    if nat_eq(a1551, a1552) {
        Ok(f292({
            let a1555 = {
                let a1553 = v1;
                let a1554 = 8u64;
                nat_mul(a1553, a1554)?
            };
            let a1556 = 4294967296u64;
            let a1557 = 0u64;
            nat_quot(a1555, a1556, a1557)
        }))
    } else {
        let a1560 = {
            let a1558 = v3;
            let a1559 = 4u64;
            nat_add(a1558, a1559)?
        };
        let a1561 = v2;
        if nat_eq(a1560, a1561) {
            Ok(f292({
                let a1564 = {
                    let a1562 = v1;
                    let a1563 = 8u64;
                    nat_mul(a1562, a1563)?
                };
                let a1565 = 4294967296u64;
                let a1566 = 0u64;
                nat_rem(a1564, a1565, a1566)
            }))
        } else {
            Ok(f293(f294(v0.clone(), v1, v3), f294(v0.clone(), v1, {
                let a1567 = v3;
                let a1568 = 1u64;
                nat_add(a1567, a1568)?
            }), f294(v0.clone(), v1, {
                let a1569 = v3;
                let a1570 = 2u64;
                nat_add(a1569, a1570)?
            }), f294(v0, v1, {
                let a1571 = v3;
                let a1572 = 3u64;
                nat_add(a1571, a1572)?
            })))
        }
    }
}

pub fn f273(v0: Bytes, v1: u64) -> bool {
    let m1573 = f225(v0, v1);
    match m1573 {
        None => {
            false
        }
        Some(v2) => {
            f295(v2)
        }
    }
}

pub fn f274(v0: Fn15, p1578: u64, p1579: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1577 = (p1578, p1579);
    {
        let mut m1577 = m1577;
        loop {
            let step = {
                let (v1, v2) = m1577;
                let m1574 = v1;
                if m1574 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1574 - 1;
                    let m1576 = {
                        let c1575: Fn15 = v0.clone();
                        c1575.apply(v2.clone())?
                    };
                    match m1576 {
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
                Ok(next) => { m1577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f296(v0, v1)
}

pub fn f276(v0: Bytes, v1: u64) -> R<u64> {
    let (h1580, _) = f171(Fn3::F297(v0.clone()), {
        let a1582 = {
            let a1581 = v0;
            length_bytes(a1581)
        };
        let a1583 = 1u64;
        nat_add(a1582, a1583)?
    }, v1)?;
    Ok(h1580)
}

pub fn f277(v0: Bytes) -> R<Option<u64>> {
    if f298(v0.clone())? {
        Ok(Some({
            let (_, h1584) = {
                let (h1585, _) = f299(Fn16::F300(v0.clone()), {
                    let a1586 = v0;
                    length_bytes(a1586)
                }, (0u64, 0u64))?;
                h1585
            };
            h1584
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f278(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1587 = 0u64;
        let a1588 = v1;
        if nat_lt(a1587, a1588) {
            let a1589 = v1;
            let a1590 = 9223372036854775808u64;
            nat_le(a1589, a1590)
        } else {
            false
        }
    } else {
        let a1591 = v1;
        let a1592 = 9223372036854775807u64;
        nat_le(a1591, a1592)
    }
}

pub fn f279(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f160(v0.clone(), 0u64, {
        let a1593 = v0;
        length_bytes(a1593)
    }, false, Bytes::lit(&[]))?;
    let m1596 = {
        let a1594 = v1.clone();
        let a1595 = 0u64;
        index_bytes(a1594, a1595)
    };
    match m1596 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1641 = {
                    let a1617 = {
                        let a1605 = {
                            let a1599 = {
                                let a1597 = v3;
                                let a1598 = 128u8;
                                fixed_u8::bit_and(a1597, a1598)
                            };
                            let a1600 = 128u8;
                            if equal(a1599, a1600) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1606 = {
                            let a1603 = {
                                let a1601 = v3;
                                let a1602 = 64u8;
                                fixed_u8::bit_and(a1601, a1602)
                            };
                            let a1604 = 64u8;
                            if equal(a1603, a1604) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1605, a1606)?
                    };
                    let a1618 = {
                        let a1615 = {
                            let a1609 = {
                                let a1607 = v3;
                                let a1608 = 32u8;
                                fixed_u8::bit_and(a1607, a1608)
                            };
                            let a1610 = 32u8;
                            if equal(a1609, a1610) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1616 = {
                            let a1613 = {
                                let a1611 = v3;
                                let a1612 = 16u8;
                                fixed_u8::bit_and(a1611, a1612)
                            };
                            let a1614 = 16u8;
                            if equal(a1613, a1614) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1615, a1616)?
                    };
                    nat_add(a1617, a1618)?
                };
                let a1642 = {
                    let a1639 = {
                        let a1627 = {
                            let a1621 = {
                                let a1619 = v3;
                                let a1620 = 8u8;
                                fixed_u8::bit_and(a1619, a1620)
                            };
                            let a1622 = 8u8;
                            if equal(a1621, a1622) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1628 = {
                            let a1625 = {
                                let a1623 = v3;
                                let a1624 = 4u8;
                                fixed_u8::bit_and(a1623, a1624)
                            };
                            let a1626 = 4u8;
                            if equal(a1625, a1626) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1627, a1628)?
                    };
                    let a1640 = {
                        let a1637 = {
                            let a1631 = {
                                let a1629 = v3;
                                let a1630 = 2u8;
                                fixed_u8::bit_and(a1629, a1630)
                            };
                            let a1632 = 2u8;
                            if equal(a1631, a1632) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1638 = {
                            let a1635 = {
                                let a1633 = v3;
                                let a1634 = 1u8;
                                fixed_u8::bit_and(a1633, a1634)
                            };
                            let a1636 = 1u8;
                            if equal(a1635, a1636) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1637, a1638)?
                    };
                    nat_add(a1639, a1640)?
                };
                nat_add(a1641, a1642)?
            };
            if f212(v4) {
                let a1649 = f42({
                    let a1643 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1643)
                }, {
                    let a1644 = v4;
                    let a1645 = 65u64;
                    nat_sub(a1644, a1645)
                }, 1u64);
                let a1650 = f42(v1.clone(), 1u64, {
                    let a1647 = {
                        let a1646 = v1;
                        length_bytes(a1646)
                    };
                    let a1648 = 1u64;
                    nat_sub(a1647, a1648)
                });
                Ok(append_bytes(a1649, a1650))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f280(v0: Bytes) -> R<bool> {
    let m1653 = {
        let a1651 = v0;
        let a1652 = 0u64;
        index_bytes(a1651, a1652)
    };
    match m1653 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f211({
                let v2: u8 = v1;
                let a1698 = {
                    let a1674 = {
                        let a1662 = {
                            let a1656 = {
                                let a1654 = v2;
                                let a1655 = 128u8;
                                fixed_u8::bit_and(a1654, a1655)
                            };
                            let a1657 = 128u8;
                            if equal(a1656, a1657) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1663 = {
                            let a1660 = {
                                let a1658 = v2;
                                let a1659 = 64u8;
                                fixed_u8::bit_and(a1658, a1659)
                            };
                            let a1661 = 64u8;
                            if equal(a1660, a1661) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1662, a1663)?
                    };
                    let a1675 = {
                        let a1672 = {
                            let a1666 = {
                                let a1664 = v2;
                                let a1665 = 32u8;
                                fixed_u8::bit_and(a1664, a1665)
                            };
                            let a1667 = 32u8;
                            if equal(a1666, a1667) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1673 = {
                            let a1670 = {
                                let a1668 = v2;
                                let a1669 = 16u8;
                                fixed_u8::bit_and(a1668, a1669)
                            };
                            let a1671 = 16u8;
                            if equal(a1670, a1671) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1672, a1673)?
                    };
                    nat_add(a1674, a1675)?
                };
                let a1699 = {
                    let a1696 = {
                        let a1684 = {
                            let a1678 = {
                                let a1676 = v2;
                                let a1677 = 8u8;
                                fixed_u8::bit_and(a1676, a1677)
                            };
                            let a1679 = 8u8;
                            if equal(a1678, a1679) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1685 = {
                            let a1682 = {
                                let a1680 = v2;
                                let a1681 = 4u8;
                                fixed_u8::bit_and(a1680, a1681)
                            };
                            let a1683 = 4u8;
                            if equal(a1682, a1683) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1684, a1685)?
                    };
                    let a1697 = {
                        let a1694 = {
                            let a1688 = {
                                let a1686 = v2;
                                let a1687 = 2u8;
                                fixed_u8::bit_and(a1686, a1687)
                            };
                            let a1689 = 2u8;
                            if equal(a1688, a1689) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1695 = {
                            let a1692 = {
                                let a1690 = v2;
                                let a1691 = 1u8;
                                fixed_u8::bit_and(a1690, a1691)
                            };
                            let a1693 = 1u8;
                            if equal(a1692, a1693) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1694, a1695)?
                    };
                    nat_add(a1696, a1697)?
                };
                nat_add(a1698, a1699)?
            }) {
                Ok(true)
            } else {
                Ok(f212({
                    let v3: u8 = v1;
                    let a1744 = {
                        let a1720 = {
                            let a1708 = {
                                let a1702 = {
                                    let a1700 = v3;
                                    let a1701 = 128u8;
                                    fixed_u8::bit_and(a1700, a1701)
                                };
                                let a1703 = 128u8;
                                if equal(a1702, a1703) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1709 = {
                                let a1706 = {
                                    let a1704 = v3;
                                    let a1705 = 64u8;
                                    fixed_u8::bit_and(a1704, a1705)
                                };
                                let a1707 = 64u8;
                                if equal(a1706, a1707) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1708, a1709)?
                        };
                        let a1721 = {
                            let a1718 = {
                                let a1712 = {
                                    let a1710 = v3;
                                    let a1711 = 32u8;
                                    fixed_u8::bit_and(a1710, a1711)
                                };
                                let a1713 = 32u8;
                                if equal(a1712, a1713) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1719 = {
                                let a1716 = {
                                    let a1714 = v3;
                                    let a1715 = 16u8;
                                    fixed_u8::bit_and(a1714, a1715)
                                };
                                let a1717 = 16u8;
                                if equal(a1716, a1717) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1718, a1719)?
                        };
                        nat_add(a1720, a1721)?
                    };
                    let a1745 = {
                        let a1742 = {
                            let a1730 = {
                                let a1724 = {
                                    let a1722 = v3;
                                    let a1723 = 8u8;
                                    fixed_u8::bit_and(a1722, a1723)
                                };
                                let a1725 = 8u8;
                                if equal(a1724, a1725) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1731 = {
                                let a1728 = {
                                    let a1726 = v3;
                                    let a1727 = 4u8;
                                    fixed_u8::bit_and(a1726, a1727)
                                };
                                let a1729 = 4u8;
                                if equal(a1728, a1729) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1730, a1731)?
                        };
                        let a1743 = {
                            let a1740 = {
                                let a1734 = {
                                    let a1732 = v3;
                                    let a1733 = 2u8;
                                    fixed_u8::bit_and(a1732, a1733)
                                };
                                let a1735 = 2u8;
                                if equal(a1734, a1735) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1741 = {
                                let a1738 = {
                                    let a1736 = v3;
                                    let a1737 = 1u8;
                                    fixed_u8::bit_and(a1736, a1737)
                                };
                                let a1739 = 1u8;
                                if equal(a1738, a1739) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1740, a1741)?
                        };
                        nat_add(a1742, a1743)?
                    };
                    nat_add(a1744, a1745)?
                }))
            }
        }
    }
}

pub fn f281(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1746 = f301(v0.clone());
    match m1746 {
        None => {
            if f27(f98(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1747 = f100(v0, f8(Str::lit("items")));
                match m1747 {
                    None => {
                        Ok(f253(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f302(f8(Str::lit("List")), f303(v3, {
                            let a1748 = v1;
                            let a1749 = f8(Str::lit("Item"));
                            append_bytes(a1748, a1749)
                        })?))
                    }
                }
            } else {
                f303(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f302(f8(Str::lit("Option")), f303(v2, v1)?))
        }
    }
}

pub fn f282(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1750 = v2;
    let a1751 = 0u64;
    if nat_eq(a1750, a1751) {
        Ok(f304(v0, 0u64))
    } else {
        let (_, h1752) = {
            let (h1753, _) = f257(Fn9::F305(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1753
        };
        Ok(h1752)
    }
}

pub fn f283() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f284(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1757 = {
        let a1755 = v0.clone();
        let a1756 = {
            let (h1754, _) = v1.clone();
            h1754
        };
        index_bytes(a1755, a1756)
    };
    match m1757 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1759 = {
                    let (h1758, _) = v1.clone();
                    h1758
                };
                let a1760 = 1u64;
                nat_add(a1759, a1760)?
            }, {
                let a1763 = {
                    let (_, h1761) = v1;
                    h1761
                };
                let a1764 = f306(v0, {
                    let (h1762, _) = v1;
                    h1762
                }, v2)?;
                append_bytes(a1763, a1764)
            })))
        }
    }
}

pub fn f285(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1765 = v0.clone();
    match m1765.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1768 = {
                let (h1766, _) = v2;
                h1766
            };
            let a1769 = {
                let (h1767, _) = v1.clone();
                h1767
            };
            if equal(a1768, a1769) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f286(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f307(f308(Fn17::F309, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1774 = {
        let (h1770, _) = f310(Fn18::F311, {
            let a1772 = {
                let a1771 = v0;
                length_list(a1771)
            };
            let a1773 = 1u64;
            nat_add(a1772, a1773)?
        }, v1)?;
        h1770
    };
    match m1774.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f287(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1779 = {
        let a1776 = f217({
            let (h1775, _) = v0;
            h1775
        })?;
        let a1777 = Bytes::lit(&[0x3a]);
        append_bytes(a1776, a1777)
    };
    let a1780 = {
        let (_, h1778) = v0;
        h1778
    };
    Ok(append_bytes(a1779, a1780))
}

pub fn f288(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f233({
        let (_, h1781) = {
            let (h1782, _) = f312(Fn19::F313, {
                let a1784 = {
                    let a1783 = v0.clone();
                    length_list(a1783)
                };
                let a1785 = 1u64;
                nat_add(a1784, a1785)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1782
        };
        h1781
    }))
}

pub fn f289(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f314(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1786) => {
            h1786
        }
    });
    let v2: u32 = f271(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1787, _, _) => {
            h1787
        }
    }, f271(f315(match v0.clone() {
        Adt23::C0(_, _, _, _, h1788, _, _, _, _, _) => {
            h1788
        }
    }), f271(f316(match v0.clone() {
        Adt23::C0(_, _, _, _, h1789, _, _, _, _, _) => {
            h1789
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1790, _, _, _, _) => {
            h1790
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1791, _, _, _) => {
            h1791
        }
    }), f271(v1, {
        let m1794 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1793, _) => {
                h1793
            }
        };
        match m1794 {
            Adt22::C0(h1792, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1792
            }
        }
    }))));
    let v3: u32 = f271(f317(match v0.clone() {
        Adt23::C0(h1795, _, _, _, _, _, _, _, _, _) => {
            h1795
        }
    }), f318(match v0.clone() {
        Adt23::C0(h1796, _, _, _, _, _, _, _, _, _) => {
            h1796
        }
    }, match v0.clone() {
        Adt23::C0(_, h1797, _, _, _, _, _, _, _, _) => {
            h1797
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1798, _, _, _, _, _, _, _) => {
            h1798
        }
    }));
    Ok(Adt23::C0(f271(v2, v3), match v0.clone() {
        Adt23::C0(h1799, _, _, _, _, _, _, _, _, _) => {
            h1799
        }
    }, match v0.clone() {
        Adt23::C0(_, h1800, _, _, _, _, _, _, _, _) => {
            h1800
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1801, _, _, _, _, _, _, _) => {
            h1801
        }
    }, f271(match v0.clone() {
        Adt23::C0(_, _, _, h1802, _, _, _, _, _, _) => {
            h1802
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1803, _, _, _, _, _) => {
            h1803
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1804, _, _, _, _) => {
            h1804
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1805, _, _, _) => {
            h1805
        }
    }, f319(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1806, _) => {
            h1806
        }
    }), {
        let a1808 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1807) => {
                h1807
            }
        };
        let a1809 = 1u64;
        nat_add(a1808, a1809)?
    }))
}

pub fn f290(v0: u32) -> u64 {
    let m1811 = {
        let a1810 = v0;
        fixed_u64::convert(i128::from(a1810))
    };
    match m1811 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f291(v0: u64) -> u32 {
    let m1815 = {
        let a1814 = {
            let a1812 = v0;
            let a1813 = 4294967295u64;
            fixed_u64::bit_and(a1812, a1813)
        };
        fixed_u32::convert(i128::from(a1814))
    };
    match m1815 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f292(v0: u64) -> u32 {
    let (_, h1816) = {
        let (_, h1817) = f320(Fn20::F321, 32u64, (v0, (1u32, 0u32)));
        h1817
    };
    h1816
}

pub fn f293(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1822 = f322(v0, 24u32);
    let a1823 = {
        let a1820 = f322(v1, 16u32);
        let a1821 = {
            let a1818 = f322(v2, 8u32);
            let a1819 = v3;
            fixed_u32::bit_or(a1818, a1819)
        };
        fixed_u32::bit_or(a1820, a1821)
    };
    fixed_u32::bit_or(a1822, a1823)
}

pub fn f294(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1824 = v2;
    let a1825 = v1;
    if nat_lt(a1824, a1825) {
        let m1828 = {
            let a1826 = v0;
            let a1827 = v2;
            index_bytes(a1826, a1827)
        };
        match m1828 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1830 = {
                    let a1829 = v3;
                    fixed_u32::convert(i128::from(a1829))
                };
                match m1830 {
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
        let a1831 = v2;
        let a1832 = v1;
        if nat_eq(a1831, a1832) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f295(v0: u8) -> bool {
    let a1833 = v0;
    let a1834 = 32u8;
    if if if equal(a1833, a1834) {
        true
    } else {
        let a1835 = v0;
        let a1836 = 9u8;
        equal(a1835, a1836)
    } {
        true
    } else {
        let a1837 = v0;
        let a1838 = 10u8;
        equal(a1837, a1838)
    } {
        true
    } else {
        let a1839 = v0;
        let a1840 = 12u8;
        if equal(a1839, a1840) {
            true
        } else {
            let a1841 = v0;
            let a1842 = 13u8;
            equal(a1841, a1842)
        }
    }
}

pub fn f296(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1843, _) = v1.clone();
        h1843
    };
    let v3: Bytes = {
        let (h1844, _) = {
            let (_, h1845) = v1.clone();
            h1845
        };
        h1844
    };
    let a1848 = {
        let (_, h1846) = {
            let (_, h1847) = v1;
            h1847
        };
        h1846
    };
    let a1849 = 0u64;
    if nat_eq(a1848, a1849) {
        let v4: u64 = f248(v0.clone(), v2)?;
        let a1850 = v4;
        let a1851 = 34u64;
        if nat_eq(a1850, a1851) {
            Ok(Some(({
                let a1852 = v2;
                let a1853 = 1u64;
                nat_add(a1852, a1853)?
            }, (v3, 1u64))))
        } else {
            let a1854 = v4;
            let a1855 = 92u64;
            if nat_eq(a1854, a1855) {
                f323(v0, v2, v3)
            } else {
                let a1856 = v4;
                let a1857 = 32u64;
                if if nat_lt(a1856, a1857) {
                    true
                } else {
                    let a1858 = v4;
                    let a1859 = 256u64;
                    nat_eq(a1858, a1859)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1860 = v2;
                        let a1861 = 1u64;
                        nat_add(a1860, a1861)?
                    }, ({
                        let a1862 = v3;
                        let a1863 = f42(v0, v2, 1u64);
                        append_bytes(a1862, a1863)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f297(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f324(v0, v1)
}

pub fn f298(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1864 = v0.clone();
        length_bytes(a1864)
    };
    let a1865 = 0u64;
    let a1866 = v1;
    if nat_lt(a1865, a1866) {
        let a1867 = f325(v0.clone())?;
        let a1868 = v1;
        if nat_eq(a1867, a1868) {
            let a1869 = 1u64;
            let a1870 = v1;
            if nat_lt(a1869, a1870) {
                let m1873 = {
                    let a1871 = v0;
                    let a1872 = 0u64;
                    index_bytes(a1871, a1872)
                };
                match m1873 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1874 = v2;
                        let a1875 = 48u8;
                        Ok(!equal(a1874, a1875))
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

pub fn f299(v0: Fn16, p1880: u64, p1881: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1879 = (p1880, p1881);
    {
        let mut m1879 = m1879;
        loop {
            let step = {
                let (v1, v2) = m1879;
                let m1876 = v1;
                if m1876 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1876 - 1;
                    let m1878 = {
                        let c1877: Fn16 = v0.clone();
                        c1877.apply(v2)?
                    };
                    match m1878 {
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
                Ok(next) => { m1879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f326(v0, v1)
}

pub fn f301(v0: Adt14) -> Option<Adt14> {
    let m1882 = f97(f100(v0, f8(Str::lit("oneOf"))));
    match m1882.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m1883 = v2;
            match m1883.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m1884 = v4;
                    match m1884.uncons() {
                        None => {
                            if f327(v3.clone()) {
                                Some(v1)
                            } else {
                                if f327(v1) {
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

pub fn f302(v0: Bytes, v1: Result<(Bytes, List<Adt17>), Bytes>) -> Result<(Bytes, List<Adt17>), Bytes> {
    let m1885 = v1;
    match m1885 {
        Ok(v3) => {
            f254(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h1886, _) = v3;
                h1886
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h1887) = v3;
                h1887
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt17>), Bytes>(v2)
        }
    }
}

pub fn f303(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1888 = f301(v0.clone());
    match m1888 {
        None => {
            if f27(f98(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1889 = f100(v0, f8(Str::lit("items")));
                match m1889 {
                    None => {
                        Ok(f253(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f302(f8(Str::lit("List")), f328(v3, {
                            let a1890 = v1;
                            let a1891 = f8(Str::lit("Item"));
                            append_bytes(a1890, a1891)
                        })?))
                    }
                }
            } else {
                f328(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f302(f8(Str::lit("Option")), f328(v2, v1)?))
        }
    }
}

pub fn f304(v0: Bytes, v1: u64) -> Bytes {
    let m1895 = {
        let a1892 = v0;
        let a1893 = v1;
        let a1894 = 1u64;
        slice_bytes(a1892, a1893, a1894)
    };
    match m1895 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f305(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f329(v1, v0, v2)
}

pub fn f306(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1940 = {
            let a1916 = {
                let a1904 = {
                    let a1898 = {
                        let a1896 = v3;
                        let a1897 = 128u8;
                        fixed_u8::bit_and(a1896, a1897)
                    };
                    let a1899 = 128u8;
                    if equal(a1898, a1899) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1905 = {
                    let a1902 = {
                        let a1900 = v3;
                        let a1901 = 64u8;
                        fixed_u8::bit_and(a1900, a1901)
                    };
                    let a1903 = 64u8;
                    if equal(a1902, a1903) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1904, a1905)?
            };
            let a1917 = {
                let a1914 = {
                    let a1908 = {
                        let a1906 = v3;
                        let a1907 = 32u8;
                        fixed_u8::bit_and(a1906, a1907)
                    };
                    let a1909 = 32u8;
                    if equal(a1908, a1909) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1915 = {
                    let a1912 = {
                        let a1910 = v3;
                        let a1911 = 16u8;
                        fixed_u8::bit_and(a1910, a1911)
                    };
                    let a1913 = 16u8;
                    if equal(a1912, a1913) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1914, a1915)?
            };
            nat_add(a1916, a1917)?
        };
        let a1941 = {
            let a1938 = {
                let a1926 = {
                    let a1920 = {
                        let a1918 = v3;
                        let a1919 = 8u8;
                        fixed_u8::bit_and(a1918, a1919)
                    };
                    let a1921 = 8u8;
                    if equal(a1920, a1921) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1927 = {
                    let a1924 = {
                        let a1922 = v3;
                        let a1923 = 4u8;
                        fixed_u8::bit_and(a1922, a1923)
                    };
                    let a1925 = 4u8;
                    if equal(a1924, a1925) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1926, a1927)?
            };
            let a1939 = {
                let a1936 = {
                    let a1930 = {
                        let a1928 = v3;
                        let a1929 = 2u8;
                        fixed_u8::bit_and(a1928, a1929)
                    };
                    let a1931 = 2u8;
                    if equal(a1930, a1931) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1937 = {
                    let a1934 = {
                        let a1932 = v3;
                        let a1933 = 1u8;
                        fixed_u8::bit_and(a1932, a1933)
                    };
                    let a1935 = 1u8;
                    if equal(a1934, a1935) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1936, a1937)?
            };
            nat_add(a1938, a1939)?
        };
        nat_add(a1940, a1941)?
    };
    let a1942 = v4;
    let a1943 = 34u64;
    if nat_eq(a1942, a1943) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1944 = v4;
        let a1945 = 92u64;
        if nat_eq(a1944, a1945) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1946 = v4;
            let a1947 = 8u64;
            if nat_eq(a1946, a1947) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1948 = v4;
                let a1949 = 12u64;
                if nat_eq(a1948, a1949) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1950 = v4;
                    let a1951 = 10u64;
                    if nat_eq(a1950, a1951) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1952 = v4;
                        let a1953 = 13u64;
                        if nat_eq(a1952, a1953) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1954 = v4;
                            let a1955 = 9u64;
                            if nat_eq(a1954, a1955) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1956 = v4;
                                let a1957 = 32u64;
                                if nat_lt(a1956, a1957) {
                                    let a1958 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1959 = f330(v2)?;
                                    Ok(append_bytes(a1958, a1959))
                                } else {
                                    Ok(f42(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f307(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f331(Fn21::F332, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f308(v0: Fn17, p1963: List<List<(Bytes, Bytes)>>, p1964: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m1962 = (p1963, p1964);
    {
        let mut m1962 = m1962;
        loop {
            let step = {
                let (v1, v2) = m1962;
                let m1960 = v2;
                match m1960.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c1961: Fn17 = v0.clone();
                            c1961.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1962 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f310(v0: Fn18, p1969: u64, p1970: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m1968 = (p1969, p1970);
    {
        let mut m1968 = m1968;
        loop {
            let step = {
                let (v1, v2) = m1968;
                let m1965 = v1;
                if m1965 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1965 - 1;
                    let m1967 = {
                        let c1966: Fn18 = v0.clone();
                        c1966.apply(v2.clone())?
                    };
                    match m1967 {
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
                Ok(next) => { m1968 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f333(v0)
}

pub fn f312(v0: Fn19, p1975: u64, p1976: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m1974 = (p1975, p1976);
    {
        let mut m1974 = m1974;
        loop {
            let step = {
                let (v1, v2) = m1974;
                let m1971 = v1;
                if m1971 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1971 - 1;
                    let m1973 = {
                        let c1972: Fn19 = v0.clone();
                        c1972.apply(v2.clone())?
                    };
                    match m1973 {
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
                Ok(next) => { m1974 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f334(v0)
}

pub fn f314(v0: u64) -> u32 {
    let a1977 = v0;
    let a1978 = 32u64;
    if nat_lt(a1977, a1978) {
        let a1979 = v0;
        let a1980 = 16u64;
        if nat_lt(a1979, a1980) {
            let a1981 = v0;
            let a1982 = 8u64;
            if nat_lt(a1981, a1982) {
                let a1983 = v0;
                let a1984 = 4u64;
                if nat_lt(a1983, a1984) {
                    let a1985 = v0;
                    let a1986 = 2u64;
                    if nat_lt(a1985, a1986) {
                        let a1987 = v0;
                        let a1988 = 1u64;
                        if nat_lt(a1987, a1988) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1989 = v0;
                        let a1990 = 3u64;
                        if nat_lt(a1989, a1990) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1991 = v0;
                    let a1992 = 6u64;
                    if nat_lt(a1991, a1992) {
                        let a1993 = v0;
                        let a1994 = 5u64;
                        if nat_lt(a1993, a1994) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1995 = v0;
                        let a1996 = 7u64;
                        if nat_lt(a1995, a1996) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1997 = v0;
                let a1998 = 12u64;
                if nat_lt(a1997, a1998) {
                    let a1999 = v0;
                    let a2000 = 10u64;
                    if nat_lt(a1999, a2000) {
                        let a2001 = v0;
                        let a2002 = 9u64;
                        if nat_lt(a2001, a2002) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2003 = v0;
                        let a2004 = 11u64;
                        if nat_lt(a2003, a2004) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2005 = v0;
                    let a2006 = 14u64;
                    if nat_lt(a2005, a2006) {
                        let a2007 = v0;
                        let a2008 = 13u64;
                        if nat_lt(a2007, a2008) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2009 = v0;
                        let a2010 = 15u64;
                        if nat_lt(a2009, a2010) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2011 = v0;
            let a2012 = 24u64;
            if nat_lt(a2011, a2012) {
                let a2013 = v0;
                let a2014 = 20u64;
                if nat_lt(a2013, a2014) {
                    let a2015 = v0;
                    let a2016 = 18u64;
                    if nat_lt(a2015, a2016) {
                        let a2017 = v0;
                        let a2018 = 17u64;
                        if nat_lt(a2017, a2018) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2019 = v0;
                        let a2020 = 19u64;
                        if nat_lt(a2019, a2020) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2021 = v0;
                    let a2022 = 22u64;
                    if nat_lt(a2021, a2022) {
                        let a2023 = v0;
                        let a2024 = 21u64;
                        if nat_lt(a2023, a2024) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2025 = v0;
                        let a2026 = 23u64;
                        if nat_lt(a2025, a2026) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2027 = v0;
                let a2028 = 28u64;
                if nat_lt(a2027, a2028) {
                    let a2029 = v0;
                    let a2030 = 26u64;
                    if nat_lt(a2029, a2030) {
                        let a2031 = v0;
                        let a2032 = 25u64;
                        if nat_lt(a2031, a2032) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2033 = v0;
                        let a2034 = 27u64;
                        if nat_lt(a2033, a2034) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2035 = v0;
                    let a2036 = 30u64;
                    if nat_lt(a2035, a2036) {
                        let a2037 = v0;
                        let a2038 = 29u64;
                        if nat_lt(a2037, a2038) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2039 = v0;
                        let a2040 = 31u64;
                        if nat_lt(a2039, a2040) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2041 = v0;
        let a2042 = 48u64;
        if nat_lt(a2041, a2042) {
            let a2043 = v0;
            let a2044 = 40u64;
            if nat_lt(a2043, a2044) {
                let a2045 = v0;
                let a2046 = 36u64;
                if nat_lt(a2045, a2046) {
                    let a2047 = v0;
                    let a2048 = 34u64;
                    if nat_lt(a2047, a2048) {
                        let a2049 = v0;
                        let a2050 = 33u64;
                        if nat_lt(a2049, a2050) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2051 = v0;
                        let a2052 = 35u64;
                        if nat_lt(a2051, a2052) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2053 = v0;
                    let a2054 = 38u64;
                    if nat_lt(a2053, a2054) {
                        let a2055 = v0;
                        let a2056 = 37u64;
                        if nat_lt(a2055, a2056) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2057 = v0;
                        let a2058 = 39u64;
                        if nat_lt(a2057, a2058) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2059 = v0;
                let a2060 = 44u64;
                if nat_lt(a2059, a2060) {
                    let a2061 = v0;
                    let a2062 = 42u64;
                    if nat_lt(a2061, a2062) {
                        let a2063 = v0;
                        let a2064 = 41u64;
                        if nat_lt(a2063, a2064) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2065 = v0;
                        let a2066 = 43u64;
                        if nat_lt(a2065, a2066) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2067 = v0;
                    let a2068 = 46u64;
                    if nat_lt(a2067, a2068) {
                        let a2069 = v0;
                        let a2070 = 45u64;
                        if nat_lt(a2069, a2070) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2071 = v0;
                        let a2072 = 47u64;
                        if nat_lt(a2071, a2072) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2073 = v0;
            let a2074 = 56u64;
            if nat_lt(a2073, a2074) {
                let a2075 = v0;
                let a2076 = 52u64;
                if nat_lt(a2075, a2076) {
                    let a2077 = v0;
                    let a2078 = 50u64;
                    if nat_lt(a2077, a2078) {
                        let a2079 = v0;
                        let a2080 = 49u64;
                        if nat_lt(a2079, a2080) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2081 = v0;
                        let a2082 = 51u64;
                        if nat_lt(a2081, a2082) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2083 = v0;
                    let a2084 = 54u64;
                    if nat_lt(a2083, a2084) {
                        let a2085 = v0;
                        let a2086 = 53u64;
                        if nat_lt(a2085, a2086) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2087 = v0;
                        let a2088 = 55u64;
                        if nat_lt(a2087, a2088) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2089 = v0;
                let a2090 = 60u64;
                if nat_lt(a2089, a2090) {
                    let a2091 = v0;
                    let a2092 = 58u64;
                    if nat_lt(a2091, a2092) {
                        let a2093 = v0;
                        let a2094 = 57u64;
                        if nat_lt(a2093, a2094) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2095 = v0;
                        let a2096 = 59u64;
                        if nat_lt(a2095, a2096) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2097 = v0;
                    let a2098 = 62u64;
                    if nat_lt(a2097, a2098) {
                        let a2099 = v0;
                        let a2100 = 61u64;
                        if nat_lt(a2099, a2100) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2101 = v0;
                        let a2102 = 63u64;
                        if nat_lt(a2101, a2102) {
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

pub fn f315(v0: u32) -> u32 {
    let a2105 = {
        let a2103 = f335(v0, 6u32, 26u32);
        let a2104 = f335(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2103, a2104)
    };
    let a2106 = f335(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2105, a2106)
}

pub fn f316(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2112 = {
        let a2107 = v0;
        let a2108 = v1;
        fixed_u32::bit_and(a2107, a2108)
    };
    let a2113 = {
        let a2110 = {
            let a2109 = v0;
            fixed_u32::bit_not(a2109)
        };
        let a2111 = v2;
        fixed_u32::bit_and(a2110, a2111)
    };
    fixed_u32::bit_xor(a2112, a2113)
}

pub fn f317(v0: u32) -> u32 {
    let a2116 = {
        let a2114 = f335(v0, 2u32, 30u32);
        let a2115 = f335(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2114, a2115)
    };
    let a2117 = f335(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2116, a2117)
}

pub fn f318(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2126 = {
        let a2122 = {
            let a2118 = v0;
            let a2119 = v1;
            fixed_u32::bit_and(a2118, a2119)
        };
        let a2123 = {
            let a2120 = v0;
            let a2121 = v2;
            fixed_u32::bit_and(a2120, a2121)
        };
        fixed_u32::bit_xor(a2122, a2123)
    };
    let a2127 = {
        let a2124 = v1;
        let a2125 = v2;
        fixed_u32::bit_and(a2124, a2125)
    };
    fixed_u32::bit_xor(a2126, a2127)
}

pub fn f319(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2128, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2128
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2129, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2129
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2130, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2130
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2131, _, _, _, _, _, _, _, _, _, _, _) => {
            h2131
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2132, _, _, _, _, _, _, _, _, _, _) => {
            h2132
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2133, _, _, _, _, _, _, _, _, _) => {
            h2133
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2134, _, _, _, _, _, _, _, _) => {
            h2134
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2135, _, _, _, _, _, _, _) => {
            h2135
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2136, _, _, _, _, _, _) => {
            h2136
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2137, _, _, _, _, _) => {
            h2137
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2138, _, _, _, _) => {
            h2138
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2139, _, _, _) => {
            h2139
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2140, _, _) => {
            h2140
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2141, _) => {
            h2141
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2142) => {
            h2142
        }
    }, f271(f336(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2143, _) => {
            h2143
        }
    }), f271(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2144, _, _, _, _, _, _) => {
            h2144
        }
    }, f271(f337(match v0.clone() {
        Adt22::C0(_, h2145, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2145
        }
    }), match v0 {
        Adt22::C0(h2146, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2146
        }
    }))))
}

pub fn f320(v0: Fn20, p2150: u64, p2151: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2149 = (p2150, p2151);
    {
        let mut m2149 = m2149;
        loop {
            let step = {
                let (v1, v2) = m2149;
                let m2147 = v1;
                if m2147 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2147 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2148: Fn20 = v0.clone();
                        c2148.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f321(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2152, _) = v0;
        h2152
    };
    let v2: u32 = {
        let (h2153, _) = {
            let (_, h2154) = v0;
            h2154
        };
        h2153
    };
    let v3: u32 = {
        let (_, h2155) = {
            let (_, h2156) = v0;
            h2156
        };
        h2155
    };
    ({
        let a2157 = v1;
        let a2158 = 2u64;
        let a2159 = 0u64;
        nat_quot(a2157, a2158, a2159)
    }, (f322(v2, 1u32), {
        let a2163 = {
            let a2160 = v1;
            let a2161 = 2u64;
            let a2162 = 0u64;
            nat_rem(a2160, a2161, a2162)
        };
        let a2164 = 1u64;
        if nat_eq(a2163, a2164) {
            let a2165 = v3;
            let a2166 = v2;
            fixed_u32::bit_or(a2165, a2166)
        } else {
            v3
        }
    }))
}

pub fn f322(v0: u32, v1: u32) -> u32 {
    let m2169 = {
        let a2167 = v0;
        let a2168 = v1;
        fixed_u32::shift_left(a2167, a2168)
    };
    match m2169 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f323(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f248(v0.clone(), {
        let a2170 = v1;
        let a2171 = 1u64;
        nat_add(a2170, a2171)?
    })?;
    let v4: Bytes = {
        let a2172 = v3;
        let a2173 = 117u64;
        if nat_eq(a2172, a2173) {
            f338(v0, v1)?
        } else {
            f339(v3)
        }
    };
    let a2174 = v4.clone();
    let a2175 = Bytes::lit(&[]);
    if equal(a2174, a2175) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2176 = v3;
            let a2177 = 117u64;
            if nat_eq(a2176, a2177) {
                let a2178 = v1;
                let a2179 = 6u64;
                nat_add(a2178, a2179)?
            } else {
                let a2180 = v1;
                let a2181 = 2u64;
                nat_add(a2180, a2181)?
            }
        }, ({
            let a2182 = v2;
            let a2183 = v4;
            append_bytes(a2182, a2183)
        }, 0u64))))
    }
}

pub fn f324(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f248(v0, v1)?;
    let a2184 = 47u64;
    let a2185 = v2;
    if if nat_lt(a2184, a2185) {
        let a2186 = v2;
        let a2187 = 58u64;
        nat_lt(a2186, a2187)
    } else {
        false
    } {
        Ok(Some({
            let a2188 = v1;
            let a2189 = 1u64;
            nat_add(a2188, a2189)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f325(v0: Bytes) -> R<u64> {
    let (h2190, _) = f171(Fn3::F340(v0.clone()), {
        let a2191 = v0;
        length_bytes(a2191)
    }, 0u64)?;
    Ok(h2190)
}

pub fn f326(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2195 = {
        let a2193 = v0;
        let a2194 = {
            let (h2192, _) = v1;
            h2192
        };
        index_bytes(a2193, a2194)
    };
    match m2195 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2197 = {
                    let (h2196, _) = v1;
                    h2196
                };
                let a2198 = 1u64;
                nat_add(a2197, a2198)?
            }, {
                let a2202 = {
                    let a2200 = {
                        let (_, h2199) = v1;
                        h2199
                    };
                    let a2201 = 10u64;
                    nat_mul(a2200, a2201)?
                };
                let a2203 = f341(v2)?;
                nat_add(a2202, a2203)?
            })))
        }
    }
}

pub fn f327(v0: Adt14) -> bool {
    f27(f98(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f328(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m2204 = f342(f215(v0.clone()));
    match m2204 {
        None => {
            let m2205 = f101(f100(v0.clone(), f8(Str::lit("$ref"))));
            match m2205 {
                None => {
                    let m2207 = f100(v0.clone(), f8(Str::lit("enum")));
                    match m2207 {
                        None => {
                            let m2208 = f100(v0.clone(), f8(Str::lit("const")));
                            match m2208 {
                                None => {
                                    let v8: Bytes = f98(v0.clone(), f8(Str::lit("type")));
                                    if f27(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f254(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                    } else {
                                        if f27(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f254(f344(v0), List::<Adt17>::nil()))
                                        } else {
                                            if f27(v8.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f254(f8(Str::lit("Bool")), List::<Adt17>::nil()))
                                            } else {
                                                if f27(v8.clone(), f8(Str::lit("object"))) {
                                                    Ok(f254(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                                                } else {
                                                    Ok(f253(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2209 = f101(Some(v6));
                                    match m2209 {
                                        None => {
                                            Ok(f253(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f254(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f254(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2206 = f343(v3)?;
                    match m2206 {
                        None => {
                            Ok(f253(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f254(v4, List::<Adt17>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f253(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f329(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2210, _) = v2;
        h2210
    };
    let a2211 = v3;
    let a2212 = 0u64;
    if nat_eq(a2211, a2212) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2213 = v3;
            let a2214 = v1;
            let a2215 = 0u64;
            nat_quot(a2213, a2214, a2215)
        }, {
            let a2220 = f304(v0, {
                let a2216 = v3;
                let a2217 = v1;
                let a2218 = 0u64;
                nat_rem(a2216, a2217, a2218)
            });
            let a2221 = {
                let (_, h2219) = v2;
                h2219
            };
            append_bytes(a2220, a2221)
        }))
    }
}

pub fn f330(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2266 = {
            let a2242 = {
                let a2230 = {
                    let a2224 = {
                        let a2222 = v1;
                        let a2223 = 128u8;
                        fixed_u8::bit_and(a2222, a2223)
                    };
                    let a2225 = 128u8;
                    if equal(a2224, a2225) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2231 = {
                    let a2228 = {
                        let a2226 = v1;
                        let a2227 = 64u8;
                        fixed_u8::bit_and(a2226, a2227)
                    };
                    let a2229 = 64u8;
                    if equal(a2228, a2229) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2230, a2231)?
            };
            let a2243 = {
                let a2240 = {
                    let a2234 = {
                        let a2232 = v1;
                        let a2233 = 32u8;
                        fixed_u8::bit_and(a2232, a2233)
                    };
                    let a2235 = 32u8;
                    if equal(a2234, a2235) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2241 = {
                    let a2238 = {
                        let a2236 = v1;
                        let a2237 = 16u8;
                        fixed_u8::bit_and(a2236, a2237)
                    };
                    let a2239 = 16u8;
                    if equal(a2238, a2239) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2240, a2241)?
            };
            nat_add(a2242, a2243)?
        };
        let a2267 = {
            let a2264 = {
                let a2252 = {
                    let a2246 = {
                        let a2244 = v1;
                        let a2245 = 8u8;
                        fixed_u8::bit_and(a2244, a2245)
                    };
                    let a2247 = 8u8;
                    if equal(a2246, a2247) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2253 = {
                    let a2250 = {
                        let a2248 = v1;
                        let a2249 = 4u8;
                        fixed_u8::bit_and(a2248, a2249)
                    };
                    let a2251 = 4u8;
                    if equal(a2250, a2251) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2252, a2253)?
            };
            let a2265 = {
                let a2262 = {
                    let a2256 = {
                        let a2254 = v1;
                        let a2255 = 2u8;
                        fixed_u8::bit_and(a2254, a2255)
                    };
                    let a2257 = 2u8;
                    if equal(a2256, a2257) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2263 = {
                    let a2260 = {
                        let a2258 = v1;
                        let a2259 = 1u8;
                        fixed_u8::bit_and(a2258, a2259)
                    };
                    let a2261 = 1u8;
                    if equal(a2260, a2261) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2262, a2263)?
            };
            nat_add(a2264, a2265)?
        };
        nat_add(a2266, a2267)?
    };
    let a2274 = f304(f345(), {
        let a2268 = v2;
        let a2269 = 16u64;
        let a2270 = 0u64;
        nat_quot(a2268, a2269, a2270)
    });
    let a2275 = f304(f345(), {
        let a2271 = v2;
        let a2272 = 16u64;
        let a2273 = 0u64;
        nat_rem(a2271, a2272, a2273)
    });
    Ok(append_bytes(a2274, a2275))
}

pub fn f331(v0: Fn21, p2279: List<List<(Bytes, Bytes)>>, p2280: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2278 = (p2279, p2280);
    {
        let mut m2278 = m2278;
        loop {
            let step = {
                let (v1, v2) = m2278;
                let m2276 = v2;
                match m2276.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2277: Fn21 = v0.clone();
                            c2277.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f333(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2282 = {
        let a2281 = v0.clone();
        length_list(a2281)
    };
    let a2283 = 1u64;
    if nat_le(a2282, a2283) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f346(v0)?))
    }
}

pub fn f334(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2284, _) = v0;
        h2284
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2285) = v0;
        h2285
    };
    let m2286 = v1;
    match m2286.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2287 = v4;
            match m2287.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f347(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f335(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2288 = f242(v0, v1);
    let a2289 = f322(v0, v2);
    fixed_u32::bit_or(a2288, a2289)
}

pub fn f336(v0: u32) -> u32 {
    let a2292 = {
        let a2290 = f335(v0, 17u32, 15u32);
        let a2291 = f335(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2290, a2291)
    };
    let a2293 = f242(v0, 10u32);
    fixed_u32::bit_xor(a2292, a2293)
}

pub fn f337(v0: u32) -> u32 {
    let a2296 = {
        let a2294 = f335(v0, 7u32, 25u32);
        let a2295 = f335(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2294, a2295)
    };
    let a2297 = f242(v0, 3u32);
    fixed_u32::bit_xor(a2296, a2297)
}

pub fn f338(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2300 = f42(v0.clone(), {
        let a2298 = v1;
        let a2299 = 2u64;
        nat_add(a2298, a2299)?
    }, 2u64);
    let a2301 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2300, a2301) {
        let m2304 = f348(f42(v0, {
            let a2302 = v1;
            let a2303 = 4u64;
            nat_add(a2302, a2303)?
        }, 2u64))?;
        match m2304 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f248(v2.clone(), 0u64)?;
                let a2305 = v3;
                let a2306 = 32u64;
                if nat_lt(a2305, a2306) {
                    if f349(v3) {
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

pub fn f339(v0: u64) -> Bytes {
    let a2307 = v0;
    let a2308 = 34u64;
    if nat_eq(a2307, a2308) {
        Bytes::lit(&[0x22])
    } else {
        let a2309 = v0;
        let a2310 = 92u64;
        if nat_eq(a2309, a2310) {
            Bytes::lit(&[0x5c])
        } else {
            let a2311 = v0;
            let a2312 = 98u64;
            if nat_eq(a2311, a2312) {
                Bytes::lit(&[0x08])
            } else {
                let a2313 = v0;
                let a2314 = 102u64;
                if nat_eq(a2313, a2314) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2315 = v0;
                    let a2316 = 110u64;
                    if nat_eq(a2315, a2316) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2317 = v0;
                        let a2318 = 114u64;
                        if nat_eq(a2317, a2318) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2319 = v0;
                            let a2320 = 116u64;
                            if nat_eq(a2319, a2320) {
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

pub fn f340(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2323 = {
        let a2321 = v0;
        let a2322 = v1;
        index_bytes(a2321, a2322)
    };
    match m2323 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f350(v2)? {
                Ok(Some({
                    let a2324 = v1;
                    let a2325 = 1u64;
                    nat_add(a2324, a2325)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f341(v0: u8) -> R<u64> {
    let a2372 = {
        let v1: u8 = v0;
        let a2370 = {
            let a2346 = {
                let a2334 = {
                    let a2328 = {
                        let a2326 = v1;
                        let a2327 = 128u8;
                        fixed_u8::bit_and(a2326, a2327)
                    };
                    let a2329 = 128u8;
                    if equal(a2328, a2329) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2335 = {
                    let a2332 = {
                        let a2330 = v1;
                        let a2331 = 64u8;
                        fixed_u8::bit_and(a2330, a2331)
                    };
                    let a2333 = 64u8;
                    if equal(a2332, a2333) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2334, a2335)?
            };
            let a2347 = {
                let a2344 = {
                    let a2338 = {
                        let a2336 = v1;
                        let a2337 = 32u8;
                        fixed_u8::bit_and(a2336, a2337)
                    };
                    let a2339 = 32u8;
                    if equal(a2338, a2339) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2345 = {
                    let a2342 = {
                        let a2340 = v1;
                        let a2341 = 16u8;
                        fixed_u8::bit_and(a2340, a2341)
                    };
                    let a2343 = 16u8;
                    if equal(a2342, a2343) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2344, a2345)?
            };
            nat_add(a2346, a2347)?
        };
        let a2371 = {
            let a2368 = {
                let a2356 = {
                    let a2350 = {
                        let a2348 = v1;
                        let a2349 = 8u8;
                        fixed_u8::bit_and(a2348, a2349)
                    };
                    let a2351 = 8u8;
                    if equal(a2350, a2351) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2357 = {
                    let a2354 = {
                        let a2352 = v1;
                        let a2353 = 4u8;
                        fixed_u8::bit_and(a2352, a2353)
                    };
                    let a2355 = 4u8;
                    if equal(a2354, a2355) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2356, a2357)?
            };
            let a2369 = {
                let a2366 = {
                    let a2360 = {
                        let a2358 = v1;
                        let a2359 = 2u8;
                        fixed_u8::bit_and(a2358, a2359)
                    };
                    let a2361 = 2u8;
                    if equal(a2360, a2361) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2367 = {
                    let a2364 = {
                        let a2362 = v1;
                        let a2363 = 1u8;
                        fixed_u8::bit_and(a2362, a2363)
                    };
                    let a2365 = 1u8;
                    if equal(a2364, a2365) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2366, a2367)?
            };
            nat_add(a2368, a2369)?
        };
        nat_add(a2370, a2371)?
    };
    let a2373 = 48u64;
    Ok(nat_sub(a2372, a2373))
}

pub fn f342(p2379: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2378 = p2379;
    {
        let mut m2378 = m2378;
        loop {
            let step = {
                let v0 = m2378;
                let m2374 = v0;
                match m2374.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f66({
                            let (h2375, _) = v1.clone();
                            h2375
                        }, f351()) {
                            true
                        } else {
                            f66({
                                let (h2376, _) = v1.clone();
                                h2376
                            }, f352())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2377, _) = v1;
                                h2377
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: Bytes) -> R<Option<Bytes>> {
    if f222(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f106(f42(v0.clone(), 8u64, {
            let a2381 = {
                let a2380 = v0;
                length_bytes(a2380)
            };
            let a2382 = 8u64;
            nat_sub(a2381, a2382)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f344(v0: Adt14) -> Bytes {
    let m2383 = f230(f100(v0, f8(Str::lit("minimum"))));
    match m2383 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f345() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f346(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f307({
        let (_, h2384) = {
            let (h2385, _) = f353(Fn22::F354, {
                let a2387 = {
                    let a2386 = v0.clone();
                    length_list(a2386)
                };
                let a2388 = 1u64;
                nat_add(a2387, a2388)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2385
        };
        h2384
    }))
}

pub fn f347(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2393 = {
            let a2391 = {
                let a2389 = v0.clone();
                length_list(a2389)
            };
            let a2392 = {
                let a2390 = v1.clone();
                length_list(a2390)
            };
            nat_add(a2391, a2392)?
        };
        let a2394 = 1u64;
        nat_add(a2393, a2394)?
    };
    Ok(f238({
        let (_, h2395) = {
            let (_, h2396) = {
                let (h2397, _) = f355(Fn23::F356, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2397
            };
            h2396
        };
        h2395
    }))
}

pub fn f348(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2398 = v0.clone();
        length_bytes(a2398)
    };
    let a2402 = {
        let a2399 = v1;
        let a2400 = 2u64;
        let a2401 = 0u64;
        nat_rem(a2399, a2400, a2401)
    };
    let a2403 = 0u64;
    if nat_eq(a2402, a2403) {
        let (_, h2404) = {
            let (h2405, _) = f357(Fn24::F358(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2405
        };
        Ok(h2404)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f349(v0: u64) -> bool {
    let a2406 = v0;
    let a2407 = 8u64;
    if nat_eq(a2406, a2407) {
        true
    } else {
        let a2408 = v0;
        let a2409 = 9u64;
        if nat_eq(a2408, a2409) {
            true
        } else {
            let a2410 = v0;
            let a2411 = 10u64;
            if nat_eq(a2410, a2411) {
                true
            } else {
                let a2412 = v0;
                let a2413 = 12u64;
                if nat_eq(a2412, a2413) {
                    true
                } else {
                    let a2414 = v0;
                    let a2415 = 13u64;
                    nat_eq(a2414, a2415)
                }
            }
        }
    }
}

pub fn f350(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2460 = {
            let a2436 = {
                let a2424 = {
                    let a2418 = {
                        let a2416 = v1;
                        let a2417 = 128u8;
                        fixed_u8::bit_and(a2416, a2417)
                    };
                    let a2419 = 128u8;
                    if equal(a2418, a2419) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2425 = {
                    let a2422 = {
                        let a2420 = v1;
                        let a2421 = 64u8;
                        fixed_u8::bit_and(a2420, a2421)
                    };
                    let a2423 = 64u8;
                    if equal(a2422, a2423) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2424, a2425)?
            };
            let a2437 = {
                let a2434 = {
                    let a2428 = {
                        let a2426 = v1;
                        let a2427 = 32u8;
                        fixed_u8::bit_and(a2426, a2427)
                    };
                    let a2429 = 32u8;
                    if equal(a2428, a2429) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2435 = {
                    let a2432 = {
                        let a2430 = v1;
                        let a2431 = 16u8;
                        fixed_u8::bit_and(a2430, a2431)
                    };
                    let a2433 = 16u8;
                    if equal(a2432, a2433) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2434, a2435)?
            };
            nat_add(a2436, a2437)?
        };
        let a2461 = {
            let a2458 = {
                let a2446 = {
                    let a2440 = {
                        let a2438 = v1;
                        let a2439 = 8u8;
                        fixed_u8::bit_and(a2438, a2439)
                    };
                    let a2441 = 8u8;
                    if equal(a2440, a2441) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2447 = {
                    let a2444 = {
                        let a2442 = v1;
                        let a2443 = 4u8;
                        fixed_u8::bit_and(a2442, a2443)
                    };
                    let a2445 = 4u8;
                    if equal(a2444, a2445) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2446, a2447)?
            };
            let a2459 = {
                let a2456 = {
                    let a2450 = {
                        let a2448 = v1;
                        let a2449 = 2u8;
                        fixed_u8::bit_and(a2448, a2449)
                    };
                    let a2451 = 2u8;
                    if equal(a2450, a2451) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2457 = {
                    let a2454 = {
                        let a2452 = v1;
                        let a2453 = 1u8;
                        fixed_u8::bit_and(a2452, a2453)
                    };
                    let a2455 = 1u8;
                    if equal(a2454, a2455) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2456, a2457)?
            };
            nat_add(a2458, a2459)?
        };
        nat_add(a2460, a2461)?
    };
    let a2462 = 48u64;
    let a2463 = v2;
    if nat_le(a2462, a2463) {
        let a2464 = v2;
        let a2465 = 57u64;
        Ok(nat_le(a2464, a2465))
    } else {
        Ok(false)
    }
}

pub fn f351() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f352() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f353(v0: Fn22, p2470: u64, p2471: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2469 = (p2470, p2471);
    {
        let mut m2469 = m2469;
        loop {
            let step = {
                let (v1, v2) = m2469;
                let m2466 = v1;
                if m2466 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2466 - 1;
                    let m2468 = {
                        let c2467: Fn22 = v0.clone();
                        c2467.apply(v2.clone())?
                    };
                    match m2468 {
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
                Ok(next) => { m2469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f359(v0)
}

pub fn f355(v0: Fn23, p2476: u64, p2477: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2475 = (p2476, p2477);
    {
        let mut m2475 = m2475;
        loop {
            let step = {
                let (v1, v2) = m2475;
                let m2472 = v1;
                if m2472 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2472 - 1;
                    let m2474 = {
                        let c2473: Fn23 = v0.clone();
                        c2473.apply(v2.clone())
                    };
                    match m2474 {
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
                Ok(next) => { m2475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f360(v0)
}

pub fn f357(v0: Fn24, p2482: u64, p2483: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2481 = (p2482, p2483);
    {
        let mut m2481 = m2481;
        loop {
            let step = {
                let (v1, v2) = m2481;
                let m2478 = v1;
                if m2478 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2478 - 1;
                    let m2480 = {
                        let c2479: Fn24 = v0.clone();
                        c2479.apply(v2.clone())?
                    };
                    match m2480 {
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
                Ok(next) => { m2481 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f361(v0, v1)
}

pub fn f359(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2484, _) = v0;
        h2484
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2485) = v0;
        h2485
    };
    let m2486 = v1;
    match m2486.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2487 = v4;
            match m2487.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f362(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f360(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2488, _) = v0.clone();
        h2488
    };
    let v2: List<(Bytes, ())> = {
        let (h2489, _) = {
            let (_, h2490) = v0.clone();
            h2490
        };
        h2489
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2491) = {
            let (_, h2492) = v0;
            h2492
        };
        h2491
    };
    let m2493 = v1.clone();
    match m2493.uncons() {
        None => {
            let m2494 = v2;
            match m2494.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2495 = v2.clone();
            match m2495.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f363({
                        let (h2496, _) = v8.clone();
                        h2496
                    }, {
                        let (h2497, _) = v6.clone();
                        h2497
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

pub fn f361(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2498, _) = v1;
        h2498
    };
    let m2500 = {
        let (_, h2499) = v1;
        h2499
    };
    match m2500 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2503 = {
                let a2501 = v0.clone();
                let a2502 = v2;
                index_bytes(a2501, a2502)
            };
            match m2503 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2508 = {
                        let a2506 = v0;
                        let a2507 = {
                            let a2504 = v2;
                            let a2505 = 1u64;
                            nat_add(a2504, a2505)?
                        };
                        index_bytes(a2506, a2507)
                    };
                    match m2508 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f364(v4)? {
                                f364(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2509 = v2;
                                    let a2510 = 2u64;
                                    nat_add(a2509, a2510)?
                                }, Some({
                                    let a2515 = v3;
                                    let a2516 = f304(f365(), {
                                        let a2513 = {
                                            let a2511 = f366(v4)?;
                                            let a2512 = 16u64;
                                            nat_mul(a2511, a2512)?
                                        };
                                        let a2514 = f366(v5)?;
                                        nat_add(a2513, a2514)?
                                    });
                                    append_bytes(a2515, a2516)
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

pub fn f362(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2521 = {
            let a2519 = {
                let a2517 = v0.clone();
                length_list(a2517)
            };
            let a2520 = {
                let a2518 = v1.clone();
                length_list(a2518)
            };
            nat_add(a2519, a2520)?
        };
        let a2522 = 1u64;
        nat_add(a2521, a2522)?
    };
    Ok(f367({
        let (_, h2523) = {
            let (_, h2524) = {
                let (h2525, _) = f368(Fn25::F369, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2525
            };
            h2524
        };
        h2523
    }))
}

pub fn f363(v0: Bytes, v1: Bytes) -> bool {
    let a2530 = {
        let a2526 = v0;
        let a2527 = v1;
        compare_bytes(a2526, a2527)
    };
    let a2531 = {
        let a2528 = Bytes::lit(&[]);
        let a2529 = Bytes::lit(&[0x01]);
        compare_bytes(a2528, a2529)
    };
    equal(a2530, a2531)
}

pub fn f364(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2576 = {
            let a2552 = {
                let a2540 = {
                    let a2534 = {
                        let a2532 = v1;
                        let a2533 = 128u8;
                        fixed_u8::bit_and(a2532, a2533)
                    };
                    let a2535 = 128u8;
                    if equal(a2534, a2535) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2541 = {
                    let a2538 = {
                        let a2536 = v1;
                        let a2537 = 64u8;
                        fixed_u8::bit_and(a2536, a2537)
                    };
                    let a2539 = 64u8;
                    if equal(a2538, a2539) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2540, a2541)?
            };
            let a2553 = {
                let a2550 = {
                    let a2544 = {
                        let a2542 = v1;
                        let a2543 = 32u8;
                        fixed_u8::bit_and(a2542, a2543)
                    };
                    let a2545 = 32u8;
                    if equal(a2544, a2545) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2551 = {
                    let a2548 = {
                        let a2546 = v1;
                        let a2547 = 16u8;
                        fixed_u8::bit_and(a2546, a2547)
                    };
                    let a2549 = 16u8;
                    if equal(a2548, a2549) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2550, a2551)?
            };
            nat_add(a2552, a2553)?
        };
        let a2577 = {
            let a2574 = {
                let a2562 = {
                    let a2556 = {
                        let a2554 = v1;
                        let a2555 = 8u8;
                        fixed_u8::bit_and(a2554, a2555)
                    };
                    let a2557 = 8u8;
                    if equal(a2556, a2557) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2563 = {
                    let a2560 = {
                        let a2558 = v1;
                        let a2559 = 4u8;
                        fixed_u8::bit_and(a2558, a2559)
                    };
                    let a2561 = 4u8;
                    if equal(a2560, a2561) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2562, a2563)?
            };
            let a2575 = {
                let a2572 = {
                    let a2566 = {
                        let a2564 = v1;
                        let a2565 = 2u8;
                        fixed_u8::bit_and(a2564, a2565)
                    };
                    let a2567 = 2u8;
                    if equal(a2566, a2567) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2573 = {
                    let a2570 = {
                        let a2568 = v1;
                        let a2569 = 1u8;
                        fixed_u8::bit_and(a2568, a2569)
                    };
                    let a2571 = 1u8;
                    if equal(a2570, a2571) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2572, a2573)?
            };
            nat_add(a2574, a2575)?
        };
        nat_add(a2576, a2577)?
    };
    let a2578 = 48u64;
    let a2579 = v2;
    if if nat_le(a2578, a2579) {
        let a2580 = v2;
        let a2581 = 57u64;
        nat_le(a2580, a2581)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2582 = 97u64;
        let a2583 = v2;
        if nat_le(a2582, a2583) {
            let a2584 = v2;
            let a2585 = 102u64;
            Ok(nat_le(a2584, a2585))
        } else {
            Ok(false)
        }
    }
}

pub fn f365() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f366(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2630 = {
            let a2606 = {
                let a2594 = {
                    let a2588 = {
                        let a2586 = v1;
                        let a2587 = 128u8;
                        fixed_u8::bit_and(a2586, a2587)
                    };
                    let a2589 = 128u8;
                    if equal(a2588, a2589) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2595 = {
                    let a2592 = {
                        let a2590 = v1;
                        let a2591 = 64u8;
                        fixed_u8::bit_and(a2590, a2591)
                    };
                    let a2593 = 64u8;
                    if equal(a2592, a2593) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2594, a2595)?
            };
            let a2607 = {
                let a2604 = {
                    let a2598 = {
                        let a2596 = v1;
                        let a2597 = 32u8;
                        fixed_u8::bit_and(a2596, a2597)
                    };
                    let a2599 = 32u8;
                    if equal(a2598, a2599) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2605 = {
                    let a2602 = {
                        let a2600 = v1;
                        let a2601 = 16u8;
                        fixed_u8::bit_and(a2600, a2601)
                    };
                    let a2603 = 16u8;
                    if equal(a2602, a2603) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2604, a2605)?
            };
            nat_add(a2606, a2607)?
        };
        let a2631 = {
            let a2628 = {
                let a2616 = {
                    let a2610 = {
                        let a2608 = v1;
                        let a2609 = 8u8;
                        fixed_u8::bit_and(a2608, a2609)
                    };
                    let a2611 = 8u8;
                    if equal(a2610, a2611) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2617 = {
                    let a2614 = {
                        let a2612 = v1;
                        let a2613 = 4u8;
                        fixed_u8::bit_and(a2612, a2613)
                    };
                    let a2615 = 4u8;
                    if equal(a2614, a2615) {
                        4u64
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
                        let a2619 = 2u8;
                        fixed_u8::bit_and(a2618, a2619)
                    };
                    let a2621 = 2u8;
                    if equal(a2620, a2621) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2627 = {
                    let a2624 = {
                        let a2622 = v1;
                        let a2623 = 1u8;
                        fixed_u8::bit_and(a2622, a2623)
                    };
                    let a2625 = 1u8;
                    if equal(a2624, a2625) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2626, a2627)?
            };
            nat_add(a2628, a2629)?
        };
        nat_add(a2630, a2631)?
    };
    let a2632 = v2;
    let a2633 = 57u64;
    if nat_le(a2632, a2633) {
        let a2634 = v2;
        let a2635 = 48u64;
        Ok(nat_sub(a2634, a2635))
    } else {
        let a2636 = v2;
        let a2637 = 87u64;
        Ok(nat_sub(a2636, a2637))
    }
}

pub fn f367(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f259(Fn10::F370, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f368(v0: Fn25, p2642: u64, p2643: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2641 = (p2642, p2643);
    {
        let mut m2641 = m2641;
        loop {
            let step = {
                let (v1, v2) = m2641;
                let m2638 = v1;
                if m2638 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2638 - 1;
                    let m2640 = {
                        let c2639: Fn25 = v0.clone();
                        c2639.apply(v2.clone())
                    };
                    match m2640 {
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
                Ok(next) => { m2641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f371(v0)
}

pub fn f370(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f371(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2644, _) = v0.clone();
        h2644
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2645, _) = {
            let (_, h2646) = v0.clone();
            h2646
        };
        h2645
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2647) = {
            let (_, h2648) = v0;
            h2648
        };
        h2647
    };
    let m2649 = v1.clone();
    match m2649.uncons() {
        None => {
            let m2650 = v2;
            match m2650.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2651 = v2.clone();
            match m2651.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f363({
                        let (h2652, _) = v8.clone();
                        h2652
                    }, {
                        let (h2653, _) = v6.clone();
                        h2653
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
