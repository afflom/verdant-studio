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
    F191(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F59(k0, k1) => f59(*k0, k1.clone(), p0),
            Fn0::F191(k0) => f191(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F168,
    F187,
    F280,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F168 => f168(p0, p1),
            Fn1::F187 => f187(p0, p1),
            Fn1::F280 => f280(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F223(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F223(k0) => f223(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F270(Bytes, u64, Bytes, u64),
    F321(Bytes),
    F391(Bytes),
    F396(Bytes, Bytes),
    F397(Bytes),
    F398(Bytes, u64),
    F412(u64, Bytes),
    F458(bool, u64, Bytes),
    F512(Bytes),
    F513(Bytes),
    F549(Bytes),
    F592(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F270(k0, k1, k2, k3) => f270(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F321(k0) => f321(k0.clone(), p0),
            Fn3::F391(k0) => Ok(f391(k0.clone(), p0)),
            Fn3::F396(k0, k1) => f396(k0.clone(), k1.clone(), p0),
            Fn3::F397(k0) => f397(k0.clone(), p0),
            Fn3::F398(k0, k1) => f398(k0.clone(), *k1, p0),
            Fn3::F412(k0, k1) => f412(*k0, k1.clone(), p0),
            Fn3::F458(k0, k1, k2) => f458(*k0, *k1, k2.clone(), p0),
            Fn3::F512(k0) => f512(k0.clone(), p0),
            Fn3::F513(k0) => f513(k0.clone(), p0),
            Fn3::F549(k0) => f549(k0.clone(), p0),
            Fn3::F592(k0) => f592(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F313(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn4::F313(k0, k1, k2) => f313(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F320(Bytes),
    F333(u64, Bytes),
    F431(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F320(k0) => f320(k0.clone(), p0),
            Fn5::F333(k0, k1) => Ok(f333(*k0, k1.clone(), p0)),
            Fn5::F431(k0) => f431(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F371,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F371 => f371(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F374,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F374 => f374(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F376,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F376 => f376(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F379,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F379 => f379(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F433,
    F650,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F433 => f433(p0, p1),
            Fn10::F650 => f650(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F436,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F436 => f436(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F439,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F439 => f439(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F442,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F442 => f442(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F447,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn14::F447 => f447(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F465(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F465(k0) => f465(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F500(bool, Bytes),
    F515(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F500(k0, k1) => f500(*k0, k1.clone(), p0),
            Fn16::F515(k0) => f515(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F524,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F524 => f524(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F526,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F526 => f526(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F528,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F528 => f528(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F536,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F536 => f536(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F566,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F566 => f566(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F624,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F624 => f624(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F626,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F626 => f626(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F628(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F628(k0) => f628(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F649,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F649 => f649(p0),
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
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    let m293 = f133(f85(f72(v0, v1))?)?;
                    match m293 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f48(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m294 = f62(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m294.uncons() {
        None => {
            let v5: Bytes = f134(if v2 {
                f135()
            } else {
                f136()
            }, f137(v1.clone())?);
            if f77(f138(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m295 = f139(f85(f72(v0.clone(), v1.clone()))?, v2)?;
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
            if f77(f138(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f140(v5)?
                } else {
                    f141(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f134(f142(), f137(v1.clone())?), f72(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f134(f142(), f137(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f143()
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
            let m301 = f144(v4.clone(), f8(Str::lit("requirement")))?;
            match m301 {
                None => {
                    Ok(Adt11::C4(f143()))
                }
                Some(v5) => {
                    let m302 = f144(v4, f8(Str::lit("admission")))?;
                    match m302 {
                        None => {
                            Ok(Adt11::C4(f143()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f75(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f75(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f66(f74(v5.clone(), f8(Str::lit("id"))), f74(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f80(f66(f74(v7.clone(), f8(Str::lit("id"))), f74(v8.clone(), f8(Str::lit("construct")))))?;
                            let m303 = f145(f108(v6.clone()), f107(f75(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m303 {
                                None => {
                                    let a305 = {
                                        let a304 = v9.clone();
                                        length_list(a304)
                                    };
                                    let a306 = 0u64;
                                    if nat_eq(a305, a306) {
                                        Ok(Adt11::C4(f143()))
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
                                            Ok(Adt11::C3(List::cons(f147(f8(Str::lit("model/requirements.json")), f148(v7, v5))?, List::cons(f147(f8(Str::lit("model/admissions.json")), f148(v8, v6))?, List::cons(f122(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f123(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f146(v11))
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
            let m312 = f149(f85(f72(v0, v1))?)?;
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
            let m314 = f150(v0.clone(), v1.clone())?;
            match m314 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m315 = f151(v4.clone(), f8(Str::lit("schema")));
                    match m315 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f152(v0, f153(v5.clone())?, v5, f154(v4.clone(), f8(Str::lit("name"))), f155(f156(f151(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
            let m317 = f157(f85(f72(v0.clone(), v1))?)?;
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
                        Ok(Adt11::C3(List::cons(f147(f91(), f148(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f123(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
            let m326 = f158(v0, v1)?;
            match m326 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f159(f160(Some(v4))) {
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
            let m328 = f158(v0.clone(), v1.clone())?;
            match m328 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f77(f37(f161(v4.clone(), f8(Str::lit("schema"))), f162())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f161(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m329 = f163(v4.clone(), f8(Str::lit("source")));
                            match m329 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f164(f163(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f134(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f134(v9.clone(), f8(Str::lit("binding.json"))), f72(v0.clone(), v1));
                        let m330 = v8;
                        match m330 {
                            None => {
                                f165(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f77(f37(f161(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f165(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f134(v9, f137(f161(v7.clone(), f8(Str::lit("url"))))?);
                                    if f77(f88(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h331) => {
                                            h331
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f161(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m332 = f62(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m332.uncons() {
                                            None => {
                                                if f37(f120(f72(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f165(v0, v4, v5, v10, Some((v12, v11)))
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
            let m334 = f150(v0.clone(), v1.clone())?;
            match m334 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f152(v0.clone(), f72(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
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
    f166(v1, v0, v2)
}

pub fn f60(v0: List<Bytes>) -> List<Bytes> {
    f167(Fn1::F168, List::<Bytes>::nil(), v0)
}

pub fn f61(v0: Bytes) -> R<bool> {
    f169(v0.clone(), 0u64, {
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
            let m344 = f170(match v0.clone() {
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
            if if f171(v0.clone(), v2.clone()) {
                f138(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m346 = f158(v0.clone(), v2.clone())?;
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
            f66(f172({
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
                        let m353 = f173(f72(v0.clone(), v2.clone()), f174())?;
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
    let v2: Adt16 = f175(v1.clone())?;
    let v3: List<Bytes> = f176(v1.clone());
    let v4: List<Bytes> = f74(match v0.clone() {
        Adt13::C0(_, _, h357, _, _) => {
            h357
        }
    }, f8(Str::lit("path")));
    let m359 = f177(v2, List::<u64>::nil(), f178(v1.clone(), match v0.clone() {
        Adt13::C0(_, h358, _, _, _) => {
            h358
        }
    }, 0u64)?);
    match m359 {
        None => {
            if f180(f181(match v0.clone() {
                Adt13::C0(_, _, _, _, h361) => {
                    h361
                }
            }, f66(match v0.clone() {
                Adt13::C0(_, _, _, _, h362) => {
                    h362
                }
            }, v4.clone()))?, f181(v4.clone(), f66(match v0.clone() {
                Adt13::C0(_, _, _, _, h363) => {
                    h363
                }
            }, v4.clone()))?, f182(match v0.clone() {
                Adt13::C0(_, _, h364, _, _) => {
                    h364
                }
            })) {
                let m368 = f183(f181(match v0.clone() {
                    Adt13::C0(_, _, _, h365, _) => {
                        h365
                    }
                }, f66(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h366, _) => {
                        h366
                    }
                }))?, f181(v3.clone(), f66(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h367, _) => {
                        h367
                    }
                }))?);
                match m368 {
                    None => {
                        let m371 = f184(f74(match v0 {
                            Adt13::C0(_, h370, _, _, _) => {
                                h370
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m371 {
                            None => {
                                let m372 = f184(v4, List::<Bytes>::nil());
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f179(f66(v3, match v0 {
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
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f179(match v1 {
                Adt12::C0(_, _, _, _, h360) => {
                    h360
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f71(v0: Bytes) -> R<List<Bytes>> {
    f185(f85(v0)?, f8(Str::lit("id = \"")))
}

pub fn f72(v0: Adt1, v1: Bytes) -> Bytes {
    let m374 = f170(match v0 {
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
            List::cons(f161(v2, v1.clone()), f74(v3, v1))
        }
    }
}

pub fn f75(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f160(f158(v0, v1)?))
}

pub fn f76(v0: List<Adt14>) -> List<Bytes> {
    let m378 = v0;
    match m378.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m379 = f164(Some(v1));
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
    Ok(f186(f167(Fn1::F187, List::<Bytes>::nil(), f188(v0)?)))
}

pub fn f81(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m384 = v1;
    match m384.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f138(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f134(v0.clone(), v2), f81(v0, v3))
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
                        if f189(v0.clone(), v1.clone(), v3.clone())? {
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
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f190(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f37(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f190(v1, List::<Bytes>::nil()), v2))))
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
        let (h399, _) = f58(Fn0::F191(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
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
    }, f192())) {
        Ok(Adt11::C1(List::cons(f192(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f86(f80(f81(f192(), f82(match v0.clone() {
            Adt1::C0(_, h410, _) => {
                h410
            }
        }, f192())))?, v1.clone());
        let m411 = f62(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m411.uncons() {
            None => {
                let v5: List<Bytes> = f74(f75(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m412 = f62(v0.clone(), f193(v0.clone(), v2.clone(), v5.clone())?);
                match m412.uncons() {
                    None => {
                        let m413 = f194(v0.clone(), v2, v5)?;
                        match m413 {
                            None => {
                                f195(v0, v1)
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
    }, f136()) {
        if f78(match v0 {
            Adt1::C0(_, h418, _) => {
                h418
            }
        }, f135()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f135(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f136(), List::<Bytes>::nil())
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
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f196(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^app/lexlean.toml$' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, app/lexlean.toml, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          # app/lexlean.toml changes only in its source_package blocks, each a path package that is a\u{a}          # workspace member pinned by version, so every import stays locked (R6); the lock is written by lexlean.\u{a}          if git apply --numstat target/input/change.patch | cut -f3 | grep -qx app/lexlean.toml; then\u{a}            other=$(awk 'index($0, \"diff --git\") == 1 { f = ($3 == \"a/app/lexlean.toml\") } f && /^[+-]/ && !/^(\\+\\+\\+|---) /' target/input/change.patch | grep -v -E '^[+-](\\[\\[source_package\\]\\]|package = \"[a-z0-9.-]+\"|version = \"[0-9]+[.][0-9]+[.][0-9]+\"|kind = \"path\"|path = \"[a-z0-9-]+\"|)$' || true)\u{a}            if [ -n \"$other\" ]; then echo \"::error::propose-command changes app/lexlean.toml only in its source_package blocks, each a pinned path package of the workspace, not: $other\"; exit 1; fi\u{a}          fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd app && \"$ll\" lock) && (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f197(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio app/lexlean.toml model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # main may have moved while this ran, so refresh checks the new pull request at once.\u{a}            gh workflow run refresh.yml --repo \"$GITHUB_REPOSITORY\" --ref main || echo \"::warning::refresh could not be started; it runs on the next push to main\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f98() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, app/lexlean.toml, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: operate starts this workflow\u{a}# right after it opens one, so every regeneration takes this one path.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio app/lexlean.toml model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! (cd app && \"$ll\" lock) && (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f197(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio app/lexlean.toml model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f99() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f100(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f104(v0.clone())?;
    let v2: List<Adt14> = f75(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f111(v1.clone(), f112(v0.clone(), f113(v0.clone())?)?);
    let v4: List<List<Bytes>> = f125(f114(f72(v0, f119()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f198(v4.clone(), f124(v4), f199(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f110(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f101(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f200(f104(v0.clone())?, f112(v0.clone(), f113(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f102(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f201(f75(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f202(f75(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
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
            f66(f76(f160(f163(v1, f8(Str::lit("defers"))))), f107(v2))
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
            let m425 = f163(v1, f8(Str::lit("std")));
            match m425 {
                None => {
                    f108(v2)
                }
                Some(v3) => {
                    List::cons(f161(v3, f8(Str::lit("row"))), f108(v2))
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
    let v3: List<(Bytes, Bytes)> = f93(f203(v2.clone()), f204(f199(v0, v1.clone(), v2)?));
    f205(f80(f206(v1))?, v3.clone(), {
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
            if if f207(v2.clone(), f8(Str::lit("subject"))) {
                f208(v0.clone(), v2.clone())
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
            let m432 = f139(f85(f72(v0.clone(), v2.clone()))?, f209(v2, f135()))?;
            match m432 {
                None => {
                    f112(v0, v3)
                }
                Some(v4) => {
                    Ok(f148(v4, f112(v0, v3)?))
                }
            }
        }
    }
}

pub fn f113(v0: Adt1) -> R<List<Bytes>> {
    Ok(f66(f80(f210(f136(), f82(match v0.clone() {
        Adt1::C0(_, h433, _) => {
            h433
        }
    }, f136())))?, f80(f210(f135(), f82(match v0 {
        Adt1::C0(_, h434, _) => {
            h434
        }
    }, f135())))?))
}

pub fn f114(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f211(v0.clone(), 0u64, {
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
            f212(v1)
        }
    };
    Ok(f213(match v4 {
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
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f214(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f119() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f120(v0: Bytes) -> R<Bytes> {
    Ok(f215(f216(v0)?))
}

pub fn f121(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f217(f84(f85(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f218(f219(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f122(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f220(Adt14::C3(std::rc::Rc::new(f221(f80(f66(v1, f76(f75(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m450 = f222(Fn2::F223(v1), None::<Bytes>, v0);
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
    let v1: Bytes = if f224(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
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
    let v2: Bytes = f225(v1)?;
    let v3: u64 = {
        let a457 = v2.clone();
        length_bytes(a457)
    };
    let v4: u64 = f226(v2.clone())?;
    let a458 = v4;
    let a459 = v3;
    if nat_lt(a458, a459) {
        Ok(Err::<Adt18, Bytes>(f227(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m460 = f228(v2.clone())?;
        match m460 {
            Ok(v6) => {
                let m463 = f229(v2.clone(), v6, {
                    let a461 = v3;
                    let a462 = 2u64;
                    nat_add(a461, a462)?
                })?;
                match m463 {
                    Ok(v8) => {
                        if f230(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt18, Bytes>(f227(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a464 = f231(v2.clone(), v8)?;
                            let a465 = 60u64;
                            if !nat_eq(a464, a465) {
                                Ok(Err::<Adt18, Bytes>(f227(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m468 = f232(v2.clone(), v8, {
                                    let a466 = v3;
                                    let a467 = 2u64;
                                    nat_add(a466, a467)?
                                }, 0u64)?;
                                match m468 {
                                    Ok(v10) => {
                                        let m472 = f229(v2, {
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
                                                    Ok(Err::<Adt18, Bytes>(f227(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
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
    f233(f234(), 10u64, v0)
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
                    let m487 = f235(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m487 {
                        Ok(v7) => {
                            let m488 = f236(v7.clone(), v3)?;
                            match m488 {
                                Ok(v9) => {
                                    if f237(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m489 = f238(v4, f8(Str::lit("targetNamespace")));
                                            match m489 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m490 = f239(v5, v7, v11)?;
                                        match m490 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f240(v13.clone());
                                                let m491 = f241(v14.clone());
                                                match m491 {
                                                    None => {
                                                        let m492 = f242(v13.clone(), v14.clone());
                                                        match m492 {
                                                            None => {
                                                                let m496 = f243({
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
                                                                        }, f219(f244(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
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
                            let a501 = f245(v1)?;
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
            let v3: Bytes = f245(v1)?;
            let a508 = {
                let a507 = v3.clone();
                length_bytes(a507)
            };
            let a509 = 0u64;
            if nat_eq(a508, a509) {
                f132(v2)
            } else {
                let m510 = f246(v3.clone())?;
                match m510 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f161(v4.clone(), f8(Str::lit("id")));
                        if if f37(f161(v4, f8(Str::lit("schema"))), f162()) {
                            f77({
                                let a512 = {
                                    let a511 = v5.clone();
                                    length_bytes(a511)
                                };
                                let a513 = 0u64;
                                nat_eq(a512, a513)
                            })
                        } else {
                            false
                        } {
                            let m514 = f132(v2)?;
                            match m514 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f247(v5), {
                                        let a515 = v3;
                                        let a516 = Bytes::lit(&[0x0a]);
                                        append_bytes(a515, a516)
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

pub fn f133(p524: List<Bytes>) -> R<Option<Bytes>> {
    let m523 = p524;
    {
        let mut m523 = m523;
        loop {
            let step = {
                let v0 = m523;
                let m517 = v0;
                match m517.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m518 = f246(f245(v1)?)?;
                        match m518 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m519 = f163(v3.clone(), f8(Str::lit("source")));
                                match m519 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if if f37(f161(v4.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
                                            let a521 = {
                                                let a520 = f137(f161(v4, f8(Str::lit("url"))))?;
                                                length_bytes(a520)
                                            };
                                            let a522 = 0u64;
                                            nat_eq(a521, a522)
                                        } else {
                                            false
                                        } {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f161(v3, f8(Str::lit("id"))))))
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
                Ok(next) => { m523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f134(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f135() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f136() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f137(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f138(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a525 = v0.clone();
        length_bytes(a525)
    };
    let v3: u64 = {
        let a526 = v1.clone();
        length_bytes(a526)
    };
    let a527 = v3;
    let a528 = v2;
    if nat_le(a527, a528) {
        let m534 = {
            let a531 = v0;
            let a532 = {
                let a529 = v2;
                let a530 = v3;
                nat_sub(a529, a530)
            };
            let a533 = v3;
            slice_bytes(a531, a532, a533)
        };
        match m534 {
            None => {
                false
            }
            Some(v4) => {
                let a535 = v4;
                let a536 = v1;
                equal(a535, a536)
            }
        }
    } else {
        false
    }
}

pub fn f139(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m537 = v0;
    match m537.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f245(v2)?;
            let a539 = {
                let a538 = v4.clone();
                length_bytes(a538)
            };
            let a540 = 0u64;
            if nat_eq(a539, a540) {
                f139(v3, v1)
            } else {
                let m541 = f246(v4)?;
                match m541 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f248(v5.clone())
                        } else {
                            f249(v5.clone())
                        } {
                            let m542 = f139(v3, v1)?;
                            match m542 {
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

pub fn f140(v0: List<Bytes>) -> R<bool> {
    if f250(f157(v0.clone())?) {
        Ok(f77({
            let a544 = {
                let a543 = f251(f157(v0)?);
                length_list(a543)
            };
            let a545 = 0u64;
            nat_eq(a544, a545)
        }))
    } else {
        Ok(false)
    }
}

pub fn f141(v0: List<Bytes>) -> R<bool> {
    if f250(f144(v0.clone(), f8(Str::lit("requirement")))?) {
        if f250(f144(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f77({
                let a547 = {
                    let a546 = f148(f251(f144(v0.clone(), f8(Str::lit("requirement")))?), f251(f144(v0, f8(Str::lit("admission")))?));
                    length_list(a546)
                };
                let a548 = 0u64;
                nat_eq(a547, a548)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f142() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f143() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f144(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m549 = v0;
    match m549.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f245(v2)?;
            let a551 = {
                let a550 = v4.clone();
                length_bytes(a550)
            };
            let a552 = 0u64;
            if nat_eq(a551, a552) {
                f144(v3, v1)
            } else {
                let m553 = f246(v4)?;
                match m553 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f252(v5.clone());
                        let a555 = {
                            let a554 = v6.clone();
                            length_bytes(a554)
                        };
                        let a556 = 0u64;
                        if nat_eq(a555, a556) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m557 = f144(v3, v1.clone())?;
                            match m557 {
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

pub fn f145(p560: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m559 = p560;
    {
        let mut m559 = m559;
        loop {
            let step = {
                let v0 = m559;
                let m558 = v0;
                match m558.uncons() {
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
                Ok(next) => { m559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f147(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f220(Adt14::C3(std::rc::Rc::new(f253(v1))))?))
}

pub fn f148(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m561 = v0;
    match m561.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f148(v3, v1))
        }
    }
}

pub fn f149(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m562 = v0;
    match m562.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f245(v1)?;
            let a564 = {
                let a563 = v3.clone();
                length_bytes(a563)
            };
            let a565 = 0u64;
            if nat_eq(a564, a565) {
                f149(v2)
            } else {
                let m566 = f254(v3.clone())?;
                match m566 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f255(v4.clone()) {
                            let m567 = f149(v2)?;
                            match m567 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f192(), List::cons(f8(Str::lit("/")), List::cons(f256(f154(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a568 = v3;
                                        let a569 = Bytes::lit(&[0x0a]);
                                        append_bytes(a568, a569)
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

pub fn f150(v0: Adt1, v1: Bytes) -> R<Option<Adt19>> {
    f254(f72(v0, v1))
}

pub fn f151(p575: Adt19, v1: Bytes) -> Option<Adt19> {
    let m574 = p575;
    {
        let mut m574 = m574;
        loop {
            let step = {
                let v0 = m574;
                let m570 = v0;
                match m570 {
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
                    Adt19::C5(r571) => {
                        let v5 = (*r571).clone();
                        Ok::<Adt19, Option<Adt19>>(v5)
                    }
                    Adt19::C6 => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C7(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C8(v2, r572, r573) => {
                        let v3 = (*r572).clone();
                        let v4 = (*r573).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt19, Option<Adt19>>(Some(v3))
                        } else {
                            Ok::<Adt19, Option<Adt19>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m576 = f62(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m576.uncons() {
        None => {
            let m578 = f145(f108(List::cons(Adt14::C4(std::rc::Rc::new({
                let m577 = f257(v4.clone());
                match m577 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f107(f75(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m578 {
                None => {
                    f258(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f146(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f153(v0: Adt19) -> R<Bytes> {
    let a579 = f259(v0)?;
    let a580 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a579, a580))
}

pub fn f154(v0: Adt19, v1: Bytes) -> Bytes {
    let m581 = f260(f151(v0, v1));
    match m581 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f155(v0: List<Adt19>) -> List<Bytes> {
    let m582 = v0;
    match m582.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m583 = f260(Some(v1));
            match m583 {
                None => {
                    f155(v2)
                }
                Some(v3) => {
                    List::cons(v3, f155(v2))
                }
            }
        }
    }
}

pub fn f156(v0: Option<Adt19>) -> List<Adt19> {
    let m584 = v0;
    match m584 {
        None => {
            List::<Adt19>::nil()
        }
        Some(v1) => {
            f261(v1)
        }
    }
}

pub fn f157(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m585 = v0;
    match m585.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f245(v1)?;
            let a587 = {
                let a586 = v3.clone();
                length_bytes(a586)
            };
            let a588 = 0u64;
            if nat_eq(a587, a588) {
                f157(v2)
            } else {
                let m589 = f246(v3)?;
                match m589 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f207(v4.clone(), f8(Str::lit("agent"))) {
                            if f207(v4.clone(), f8(Str::lit("at"))) {
                                if f207(v4.clone(), f8(Str::lit("quote"))) {
                                    f262(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m590 = f157(v2)?;
                            match m590 {
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

pub fn f158(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f246(f72(v0, v1))
}

pub fn f159(p593: List<Adt14>) -> bool {
    let m592 = p593;
    {
        let mut m592 = m592;
        loop {
            let step = {
                let v0 = m592;
                let m591 = v0;
                match m591.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f37(f161(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f88(f8(Str::lit("studio")), f74(f263(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: Option<Adt14>) -> List<Adt14> {
    let m594 = v0;
    match m594 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f264(v1)
        }
    }
}

pub fn f161(v0: Adt14, v1: Bytes) -> Bytes {
    let m595 = f164(f163(v0, v1));
    match m595 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f162() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f163(p601: Adt14, v1: Bytes) -> Option<Adt14> {
    let m600 = p601;
    {
        let mut m600 = m600;
        loop {
            let step = {
                let v0 = m600;
                let m596 = v0;
                match m596 {
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
                    Adt14::C4(r597) => {
                        let v5 = (*r597).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r598, r599) => {
                        let v3 = (*r598).clone();
                        let v4 = (*r599).clone();
                        if f37(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: Option<Adt14>) -> Option<Bytes> {
    let m602 = v0;
    match m602 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m603 = v1;
            match m603 {
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

pub fn f165(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f121(f72(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f265(v1, v4)?)?), List::cons(f122(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f166(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h604, _) = v2.clone();
        h604
    };
    let v4: u64 = {
        let (h605, _) = {
            let (_, h606) = v2.clone();
            h606
        };
        h605
    };
    let v5: List<Bytes> = {
        let (_, h607) = {
            let (_, h608) = v2;
            h608
        };
        h607
    };
    let m609 = f266(v0.clone(), v3);
    match m609 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a610 = v6;
            let a611 = v1;
            if equal(a610, a611) {
                Ok(Some(({
                    let a612 = v3;
                    let a613 = 1u64;
                    nat_add(a612, a613)?
                }, ({
                    let a614 = v3;
                    let a615 = 1u64;
                    nat_add(a614, a615)?
                }, List::cons(f32(v0, v4, {
                    let a616 = v3;
                    let a617 = v4;
                    nat_sub(a616, a617)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a618 = v3;
                    let a619 = 1u64;
                    nat_add(a618, a619)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f167(v0: Fn1, p623: List<Bytes>, p624: List<Bytes>) -> List<Bytes> {
    let m622 = (p623, p624);
    {
        let mut m622 = m622;
        loop {
            let step = {
                let (v1, v2) = m622;
                let m620 = v2;
                match m620.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c621: Fn1 = v0.clone();
                            c621.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m622 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f169(v0: Bytes, p678: u64, p679: u64) -> R<bool> {
    let m677 = (p678, p679);
    {
        let mut m677 = m677;
        loop {
            let step = {
                let (v1, v2) = m677;
                let m625 = v2;
                if m625 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m625 - 1;
                    let m628 = {
                        let a626 = v0.clone();
                        let a627 = v1;
                        index_bytes(a626, a627)
                    };
                    match m628 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f267({
                                let v5: u8 = v4;
                                let a673 = {
                                    let a649 = {
                                        let a637 = {
                                            let a631 = {
                                                let a629 = v5;
                                                let a630 = 128u8;
                                                fixed_u8::bit_and(a629, a630)
                                            };
                                            let a632 = 128u8;
                                            if equal(a631, a632) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a638 = {
                                            let a635 = {
                                                let a633 = v5;
                                                let a634 = 64u8;
                                                fixed_u8::bit_and(a633, a634)
                                            };
                                            let a636 = 64u8;
                                            if equal(a635, a636) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a637, a638)?
                                    };
                                    let a650 = {
                                        let a647 = {
                                            let a641 = {
                                                let a639 = v5;
                                                let a640 = 32u8;
                                                fixed_u8::bit_and(a639, a640)
                                            };
                                            let a642 = 32u8;
                                            if equal(a641, a642) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a648 = {
                                            let a645 = {
                                                let a643 = v5;
                                                let a644 = 16u8;
                                                fixed_u8::bit_and(a643, a644)
                                            };
                                            let a646 = 16u8;
                                            if equal(a645, a646) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a647, a648)?
                                    };
                                    nat_add(a649, a650)?
                                };
                                let a674 = {
                                    let a671 = {
                                        let a659 = {
                                            let a653 = {
                                                let a651 = v5;
                                                let a652 = 8u8;
                                                fixed_u8::bit_and(a651, a652)
                                            };
                                            let a654 = 8u8;
                                            if equal(a653, a654) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a660 = {
                                            let a657 = {
                                                let a655 = v5;
                                                let a656 = 4u8;
                                                fixed_u8::bit_and(a655, a656)
                                            };
                                            let a658 = 4u8;
                                            if equal(a657, a658) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a659, a660)?
                                    };
                                    let a672 = {
                                        let a669 = {
                                            let a663 = {
                                                let a661 = v5;
                                                let a662 = 2u8;
                                                fixed_u8::bit_and(a661, a662)
                                            };
                                            let a664 = 2u8;
                                            if equal(a663, a664) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a670 = {
                                            let a667 = {
                                                let a665 = v5;
                                                let a666 = 1u8;
                                                fixed_u8::bit_and(a665, a666)
                                            };
                                            let a668 = 1u8;
                                            if equal(a667, a668) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a669, a670)?
                                    };
                                    nat_add(a671, a672)?
                                };
                                nat_add(a673, a674)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a675 = v1;
                                    let a676 = 1u64;
                                    nat_add(a675, a676)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m677 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(p684: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m683 = p684;
    {
        let mut m683 = m683;
        loop {
            let step = {
                let v0 = m683;
                let m680 = v0;
                match m680.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h681, _) = v2;
                            h681
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h682) = v2;
                                h682
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f171(v0: Adt1, v1: Bytes) -> bool {
    let m686 = f170(match v0 {
        Adt1::C0(h685, _, _) => {
            h685
        }
    }, v1);
    match m686 {
        None => {
            false
        }
        Some(v2) => {
            let m687 = v2;
            match m687 {
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

pub fn f172(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m688 = v1;
    match m688.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f268(v3.clone())
            } else {
                f77(f268(v3.clone()))
            } {
                List::cons(f134(v0.clone(), v3), f172(v0, v4, v2))
            } else {
                f172(v0, v4, v2)
            }
        }
    }
}

pub fn f173(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a689 = v0.clone();
        length_bytes(a689)
    };
    let v3: u64 = {
        let a690 = v1.clone();
        length_bytes(a690)
    };
    let v4: u64 = {
        let (h691, _) = f269(Fn3::F270(v1.clone(), v3, v0.clone(), v2), {
            let a692 = v2;
            let a693 = 1u64;
            nat_add(a692, a693)?
        }, 0u64)?;
        h691
    };
    if f224(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f174() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f175(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f271(match v0.clone() {
        Adt12::C0(h694, _, _, _, _) => {
            h694
        }
    }, 0u64)?, f271(match v0.clone() {
        Adt12::C0(_, h695, _, _, _) => {
            h695
        }
    }, 0u64)?, f271(match v0.clone() {
        Adt12::C0(_, _, h696, _, _) => {
            h696
        }
    }, 0u64)?, f271(match v0 {
        Adt12::C0(_, _, _, h697, _) => {
            h697
        }
    }, 0u64)?))
}

pub fn f176(v0: Adt12) -> List<Bytes> {
    f66(match v0.clone() {
        Adt12::C0(h698, _, _, _, _) => {
            h698
        }
    }, f66(match v0.clone() {
        Adt12::C0(_, _, h699, _, _) => {
            h699
        }
    }, f66(match v0.clone() {
        Adt12::C0(_, _, _, h700, _) => {
            h700
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h701) => {
            h701
        }
    })))
}

pub fn f177(v0: Adt16, p707: List<u64>, p708: List<Adt20>) -> Option<u64> {
    let m706 = (p707, p708);
    {
        let mut m706 = m706;
        loop {
            let step = {
                let (v1, v2) = m706;
                let m702 = v2;
                match m702.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f272(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h703) => {
                                h703
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h704, _) => {
                                    h704
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h705, _) => {
                                    h705
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m709 = v1;
    match m709.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f273(v0.clone(), v3)?), f178(v0, v4, {
                let a710 = v2;
                let a711 = 1u64;
                nat_add(a710, a711)?
            })?))
        }
    }
}

pub fn f179(p715: List<Bytes>, p716: u64) -> Bytes {
    let m714 = (p715, p716);
    {
        let mut m714 = m714;
        loop {
            let step = {
                let (v0, v1) = m714;
                let m712 = v0;
                match m712.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m713 = v1;
                        if m713 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m713 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m714 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f274(v0.clone(), v1.clone()) {
        if f274(v1, v0) {
            f275(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f181(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m717 = v0;
    match m717.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f276(v2, v1.clone())?, f181(v3, v1)?))
        }
    }
}

pub fn f182(v0: List<Adt14>) -> List<u64> {
    let m718 = v0;
    match m718.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f277(v1), f182(v2))
        }
    }
}

pub fn f183(p721: List<u64>, v1: List<u64>) -> Option<u64> {
    let m720 = p721;
    {
        let mut m720 = m720;
        loop {
            let step = {
                let v0 = m720;
                let m719 = v0;
                match m719.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f278(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
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

pub fn f184(p724: List<Bytes>, p725: List<Bytes>) -> Option<Bytes> {
    let m723 = (p724, p725);
    {
        let mut m723 = m723;
        loop {
            let step = {
                let (v0, v1) = m723;
                let m722 = v0;
                match m722.uncons() {
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
                Ok(next) => { m723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m726 = v0;
    match m726.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m727 = f279(v2, v1.clone())?;
            match m727 {
                None => {
                    f185(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f185(v3, v1)?))
                }
            }
        }
    }
}

pub fn f186(v0: List<Bytes>) -> List<Bytes> {
    f167(Fn1::F280, List::<Bytes>::nil(), v0)
}

pub fn f187(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f281(v0, v1)
}

pub fn f188(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f282(f283(f284(v0))?))
}

pub fn f189(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m728 = f158(v0, v2)?;
    match m728 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f285(v1, f161(v3.clone(), f8(Str::lit("id"))))?;
            let m729 = v4.clone();
            match m729.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m731 = f164(f163({
                        let m730 = f163(v3.clone(), f8(Str::lit("source")));
                        match m730 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m731 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f77(f88(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f214(f286(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f190(p734: List<Bytes>, p735: List<Bytes>) -> List<Bytes> {
    let m733 = (p734, p735);
    {
        let mut m733 = m733;
        loop {
            let step = {
                let (v0, v1) = m733;
                let m732 = v0;
                match m732.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f287(v0, v1)
}

pub fn f192() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f193(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m736 = v1;
    match m736.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt19 = f288(v0.clone(), v3)?;
            if f88(f289(f154(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f290(v5)?, f193(v0, v4, v2)?))
            } else {
                f193(v0, v4, v2)
            }
        }
    }
}

pub fn f194(v0: Adt1, p740: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m739 = p740;
    {
        let mut m739 = m739;
        loop {
            let step = {
                let v1 = m739;
                let m737 = v1;
                match m737.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt19 = f288(v0.clone(), v3.clone())?;
                        if if f88(f289(f154(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f37(f153({
                                let m738 = f151(v5.clone(), f8(Str::lit("schema")));
                                match m738 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f72(v0.clone(), f290(v5)?))
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
                Ok(next) => { m739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f77(f78(match v0.clone() {
        Adt1::C0(_, h741, _) => {
            h741
        }
    }, f142())) {
        Ok(Adt11::C1(List::cons(f142(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f86(f80(f291(f82(match v0.clone() {
            Adt1::C0(_, h742, _) => {
                h742
            }
        }, f142())))?, v1);
        let m743 = f62(v0.clone(), List::cons(f91(), List::cons(f119(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m743.uncons() {
            None => {
                let v5: (bool, Bytes) = f292(v0.clone(), v2)?;
                let (h744, _) = v5;
                if h744 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h745) = v5;
                        h745
                    }))
                } else {
                    let m746 = f90(v0.clone());
                    match m746.uncons() {
                        None => {
                            let m747 = f92(v0.clone())?;
                            match m747.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h748) = v5;
                                        h748
                                    }, List::cons(f7(f80(f293(f104(v0.clone())?, f112(v0.clone(), f113(v0.clone())?)?))?), List::cons(f294(v0)?, List::<Bytes>::nil()))))))
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

pub fn f196(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f295(f75(v0, f91())?)?)))
}

pub fn f197() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f198(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt23>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m749 = v1;
    match m749.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f66(f296(v5.clone(), v2.clone()), f297(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f298(v0.clone(), v5.clone()), List::cons(if f88(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a751 = {
                    let a750 = v7.clone();
                    length_list(a750)
                };
                let a752 = 0u64;
                if nat_eq(a751, a752) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f123(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f198(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f199(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt23>> {
    let v3: List<(Bytes, Bytes)> = f299(v1);
    let v4: List<(Bytes, Bytes)> = f300(v0, v2);
    f301(v3.clone(), f93(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f200(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m753 = v1;
    match m753.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f207(v2.clone(), f8(Str::lit("domain"))) {
                f302(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f161(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f161(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m754 = f163(v2.clone(), f8(Str::lit("role")));
                    match m754 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f161(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f161(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f161(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f303(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m755 = f163(v2.clone(), f8(Str::lit("origin")));
                    match m755 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f161(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m756 = f163(v2, f8(Str::lit("evidence")));
                    match m756 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f161(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f200(v0, v3))
            } else {
                f200(v0, v3)
            }
        }
    }
}

pub fn f201(v0: List<Adt14>) -> List<Bytes> {
    let m757 = v0;
    match m757.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f161(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f161(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f161(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f201(v2))
        }
    }
}

pub fn f202(v0: List<Adt14>) -> List<Bytes> {
    let m758 = v0;
    match m758.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f161(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f304(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f202(v2))
        }
    }
}

pub fn f203(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m759 = v0;
    match m759.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f161(v1.clone(), f8(Str::lit("subject"))), f161(v1, f8(Str::lit("object")))), f203(v2))
        }
    }
}

pub fn f204(v0: List<Adt23>) -> List<(Bytes, Bytes)> {
    let m760 = v0;
    match m760.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt23::C0(h761, _, _) => {
                    h761
                }
            }, match v1 {
                Adt23::C0(_, _, h762) => {
                    h762
                }
            }), f204(v2))
        }
    }
}

pub fn f205(p769: List<Bytes>, v1: List<(Bytes, Bytes)>, p770: u64) -> R<List<Bytes>> {
    let m768 = (p769, p770);
    {
        let mut m768 = m768;
        loop {
            let step = {
                let (v0, v2) = m768;
                let m763 = v2;
                if m763 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m763 - 1;
                    let v4: List<Bytes> = f80(f66(v0.clone(), f305(v0.clone(), v1.clone())))?;
                    let a766 = {
                        let a764 = v4.clone();
                        length_list(a764)
                    };
                    let a767 = {
                        let a765 = v0.clone();
                        length_list(a765)
                    };
                    if nat_eq(a766, a767) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
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

pub fn f206(v0: List<Adt14>) -> List<Bytes> {
    let m771 = v0;
    match m771.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m772 = f163(v1, f8(Str::lit("std")));
            match m772 {
                None => {
                    f206(v2)
                }
                Some(v3) => {
                    List::cons(f161(v3.clone(), f8(Str::lit("authority"))), if f207(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f161(v3, f8(Str::lit("row"))), f206(v2))
                    } else {
                        f206(v2)
                    })
                }
            }
        }
    }
}

pub fn f207(v0: Adt14, v1: Bytes) -> bool {
    f77({
        let a774 = {
            let a773 = f161(v0, v1);
            length_bytes(a773)
        };
        let a775 = 0u64;
        nat_eq(a774, a775)
    })
}

pub fn f208(v0: Adt15, v1: Adt14) -> bool {
    if f306(v0.clone(), v1.clone(), f161(v1.clone(), f8(Str::lit("subject")))) {
        f306(v0, v1.clone(), f161(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f209(v0: Bytes, v1: Bytes) -> bool {
    let m780 = {
        let a777 = v0;
        let a778 = 0u64;
        let a779 = {
            let a776 = v1.clone();
            length_bytes(a776)
        };
        slice_bytes(a777, a778, a779)
    };
    match m780 {
        None => {
            false
        }
        Some(v2) => {
            let a781 = v2;
            let a782 = v1;
            equal(a781, a782)
        }
    }
}

pub fn f210(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m783 = v1;
    match m783.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f138(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f134(v0.clone(), v2), f210(v0, v3))
            } else {
                f210(v0, v3)
            }
        }
    }
}

pub fn f211(v0: Bytes, p841: u64, p842: u64, p843: Adt17) -> R<Adt17> {
    let m840 = (p841, (p842, p843));
    {
        let mut m840 = m840;
        loop {
            let step = {
                let (v1, (v2, v3)) = m840;
                let m784 = v2;
                if m784 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m784 - 1;
                    let v5: u64 = f307(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h785, _, _, _) => {
                            h785
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a786 = v1;
                            let a787 = 1u64;
                            nat_add(a786, a787)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h788, _, _) => {
                                h788
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h789, _) => {
                                h789
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h790) => {
                                h790
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h791, _, _, _, _) => {
                                h791
                            }
                        } {
                            let a792 = v5;
                            let a793 = 34u64;
                            if nat_eq(a792, a793) {
                                let a796 = f307(v0.clone(), {
                                    let a794 = v1;
                                    let a795 = 1u64;
                                    nat_add(a794, a795)?
                                })?;
                                let a797 = 34u64;
                                if nat_eq(a796, a797) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a798 = v1;
                                        let a799 = 1u64;
                                        nat_add(a798, a799)?
                                    }, (v4, Adt17::C0(true, true, f308(match v3.clone() {
                                        Adt17::C0(_, _, h800, _, _) => {
                                            h800
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h801, _) => {
                                            h801
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h802) => {
                                            h802
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a803 = v1;
                                        let a804 = 1u64;
                                        nat_add(a803, a804)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h805, _, _) => {
                                            h805
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h806, _) => {
                                            h806
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h807) => {
                                            h807
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a808 = v1;
                                    let a809 = 1u64;
                                    nat_add(a808, a809)?
                                }, (v4, Adt17::C0(true, false, f308(match v3.clone() {
                                    Adt17::C0(_, _, h810, _, _) => {
                                        h810
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h811, _) => {
                                        h811
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h812) => {
                                        h812
                                    }
                                }))))
                            }
                        } else {
                            let a813 = v5;
                            let a814 = 34u64;
                            if nat_eq(a813, a814) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a815 = v1;
                                    let a816 = 1u64;
                                    nat_add(a815, a816)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h817, _, _) => {
                                        h817
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h818, _) => {
                                        h818
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h819) => {
                                        h819
                                    }
                                }))))
                            } else {
                                let a820 = v5;
                                let a821 = 44u64;
                                if nat_eq(a820, a821) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a822 = v1;
                                        let a823 = 1u64;
                                        nat_add(a822, a823)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h824, _, _) => {
                                            h824
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h825, _) => {
                                            h825
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h826) => {
                                            h826
                                        }
                                    }))))
                                } else {
                                    let a827 = v5;
                                    let a828 = 13u64;
                                    if nat_eq(a827, a828) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a829 = v1;
                                            let a830 = 1u64;
                                            nat_add(a829, a830)?
                                        }, (v4, v3)))
                                    } else {
                                        let a831 = v5;
                                        let a832 = 10u64;
                                        if nat_eq(a831, a832) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a833 = v1;
                                                let a834 = 1u64;
                                                nat_add(a833, a834)?
                                            }, (v4, f212(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a835 = v1;
                                                let a836 = 1u64;
                                                nat_add(a835, a836)?
                                            }, (v4, Adt17::C0(false, false, f308(match v3.clone() {
                                                Adt17::C0(_, _, h837, _, _) => {
                                                    h837
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h838, _) => {
                                                    h838
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h839) => {
                                                    h839
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
                Ok(next) => { m840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f309(List::cons(match v0.clone() {
        Adt17::C0(_, _, h844, _, _) => {
            h844
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h845, _) => {
            h845
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h846) => {
            h846
        }
    }))
}

pub fn f213(p849: List<List<Bytes>>, p850: List<List<Bytes>>) -> List<List<Bytes>> {
    let m848 = (p849, p850);
    {
        let mut m848 = m848;
        loop {
            let step = {
                let (v0, v1) = m848;
                let m847 = v0;
                match m847.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f310(v0.clone(), 0u64, {
        let a851 = v0;
        length_bytes(a851)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f215(v0: Adt24) -> Bytes {
    let a872 = f311(match v0.clone() {
        Adt24::C0(h852, _, _, _, _, _, _, _) => {
            h852
        }
    });
    let a873 = {
        let a870 = f311(match v0.clone() {
            Adt24::C0(_, h853, _, _, _, _, _, _) => {
                h853
            }
        });
        let a871 = {
            let a868 = f311(match v0.clone() {
                Adt24::C0(_, _, h854, _, _, _, _, _) => {
                    h854
                }
            });
            let a869 = {
                let a866 = f311(match v0.clone() {
                    Adt24::C0(_, _, _, h855, _, _, _, _) => {
                        h855
                    }
                });
                let a867 = {
                    let a864 = f311(match v0.clone() {
                        Adt24::C0(_, _, _, _, h856, _, _, _) => {
                            h856
                        }
                    });
                    let a865 = {
                        let a862 = f311(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h857, _, _) => {
                                h857
                            }
                        });
                        let a863 = {
                            let a860 = f311(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h858, _) => {
                                    h858
                                }
                            });
                            let a861 = f311(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h859) => {
                                    h859
                                }
                            });
                            append_bytes(a860, a861)
                        };
                        append_bytes(a862, a863)
                    };
                    append_bytes(a864, a865)
                };
                append_bytes(a866, a867)
            };
            append_bytes(a868, a869)
        };
        append_bytes(a870, a871)
    };
    append_bytes(a872, a873)
}

pub fn f216(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a874 = v0.clone();
        length_bytes(a874)
    };
    let v2: u64 = {
        let a880 = {
            let a877 = {
                let a875 = v1;
                let a876 = 72u64;
                nat_add(a875, a876)?
            };
            let a878 = 64u64;
            let a879 = 0u64;
            nat_quot(a877, a878, a879)
        };
        let a881 = 64u64;
        nat_mul(a880, a881)?
    };
    let (_, h882) = f312(Fn4::F313(v0, v1, v2), {
        let a883 = v2;
        let a884 = 64u64;
        let a885 = 0u64;
        nat_quot(a883, a884, a885)
    }, (0u64, f314()))?;
    Ok(h882)
}

pub fn f217(p888: List<List<Bytes>>, v1: Bytes, p889: List<Bytes>) -> R<List<Bytes>> {
    let m887 = (p888, p889);
    {
        let mut m887 = m887;
        loop {
            let step = {
                let (v0, v2) = m887;
                let m886 = v0;
                match m886.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f315(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f219(f316(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f317(v0)?))
}

pub fn f219(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m890 = v0;
    match m890.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m891 = v3.clone();
            match m891.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a894 = v2;
                    let a895 = {
                        let a892 = v1.clone();
                        let a893 = f219(v3, v1);
                        append_bytes(a892, a893)
                    };
                    append_bytes(a894, a895)
                }
            }
        }
    }
}

pub fn f220(v0: Adt14) -> R<Bytes> {
    let a896 = f318(v0)?;
    let a897 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a896, a897))
}

pub fn f221(v0: List<Bytes>) -> Adt14 {
    let m898 = v0;
    match m898.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f221(v2)))
        }
    }
}

pub fn f222(v0: Fn2, p902: Option<Bytes>, p903: List<Bytes>) -> Option<Bytes> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v1, v2) = m901;
                let m899 = v2;
                match m899.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c900: Fn2 = v0.clone();
                            c900.apply(v1, v3)
                        }, v4))
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

pub fn f223(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m904 = v1;
    match m904 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a907 = {
                    let a905 = v3;
                    let a906 = v0;
                    append_bytes(a905, a906)
                };
                let a908 = v2;
                append_bytes(a907, a908)
            })
        }
    }
}

pub fn f224(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m913 = {
        let a910 = v0;
        let a911 = v2;
        let a912 = {
            let a909 = v1.clone();
            length_bytes(a909)
        };
        slice_bytes(a910, a911, a912)
    };
    match m913 {
        None => {
            false
        }
        Some(v3) => {
            let a914 = v3;
            let a915 = v1;
            equal(a914, a915)
        }
    }
}

pub fn f225(v0: Bytes) -> R<Bytes> {
    let (_, h916) = {
        let (h917, _) = f319(Fn5::F320(v0.clone()), {
            let a919 = {
                let a918 = v0;
                length_bytes(a918)
            };
            let a920 = 1u64;
            nat_add(a919, a920)?
        }, (0u64, Bytes::lit(&[])))?;
        h917
    };
    Ok(h916)
}

pub fn f226(v0: Bytes) -> R<u64> {
    let (h921, _) = f269(Fn3::F321(v0.clone()), {
        let a923 = {
            let a922 = v0;
            length_bytes(a922)
        };
        let a924 = 1u64;
        nat_add(a923, a924)?
    }, 0u64)?;
    Ok(h921)
}

pub fn f227(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a925 = v0;
        utf8_encode(a925)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f128(v1)?, List::<Bytes>::nil())))))
}

pub fn f228(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f230(v0.clone(), 0u64, Str::lit("<?xml")) {
        f322(f231(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m927 = f323(v0.clone(), {
            let a926 = Str::lit("?>");
            utf8_encode(a926)
        }, 5u64)?;
        match m927 {
            None => {
                Ok(Err::<u64, Bytes>(f227(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m929 = f323(f32(v0.clone(), 0u64, v1), {
                    let a928 = Str::lit("encoding");
                    utf8_encode(a928)
                }, 5u64)?;
                match m929 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a930 = v1;
                            let a931 = 2u64;
                            nat_add(a930, a931)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f324(v0.clone(), {
                            let a932 = v2;
                            let a933 = 8u64;
                            nat_add(a932, a933)?
                        })?;
                        let v4: u64 = f324(v0.clone(), {
                            let a934 = v3;
                            let a935 = 1u64;
                            nat_add(a934, a935)?
                        })?;
                        let _: u64 = f231(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m938 = f323(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a936 = v4;
                                let a937 = 1u64;
                                nat_add(a936, a937)?
                            })?;
                            match m938 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f325(f32(v0, {
                            let a939 = v4;
                            let a940 = 1u64;
                            nat_add(a939, a940)?
                        }, {
                            let a943 = v7;
                            let a944 = {
                                let a941 = v4;
                                let a942 = 1u64;
                                nat_add(a941, a942)?
                            };
                            nat_sub(a943, a944)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a945 = v1;
                                let a946 = 2u64;
                                nat_add(a945, a946)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f227(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f229(v0: Bytes, p951: u64, p952: u64) -> R<Result<u64, Bytes>> {
    let m950 = (p951, p952);
    {
        let mut m950 = m950;
        loop {
            let step = {
                let (v1, v2) = m950;
                let m947 = v2;
                if m947 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f227(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m947 - 1;
                    let v4: u64 = f324(v0.clone(), v1)?;
                    if f230(v0.clone(), v4, Str::lit("<!--")) {
                        let m948 = f326(v0.clone(), v4)?;
                        match m948 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f230(v0.clone(), v4, Str::lit("<?")) {
                            let m949 = f327(v0.clone(), v4)?;
                            match m949 {
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
                Ok(next) => { m950 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: Bytes, v1: u64, v2: Str) -> bool {
    f224(v0, {
        let a953 = v2;
        utf8_encode(a953)
    }, v1)
}

pub fn f231(v0: Bytes, v1: u64) -> R<u64> {
    f328(v0, v1)
}

pub fn f232(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m954 = v2;
    if m954 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m954 - 1;
        let a955 = v3;
        let a956 = 0u64;
        if nat_eq(a955, a956) {
            let v5: u64 = f329(v0.clone(), {
                let a957 = v1;
                let a958 = 1u64;
                nat_add(a957, a958)?
            })?;
            let a961 = v5;
            let a962 = {
                let a959 = v1;
                let a960 = 1u64;
                nat_add(a959, a960)?
            };
            if nat_eq(a961, a962) {
                Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a963 = v1;
                    let a964 = 1u64;
                    nat_add(a963, a964)?
                }, {
                    let a967 = v5;
                    let a968 = {
                        let a965 = v1;
                        let a966 = 1u64;
                        nat_add(a965, a966)?
                    };
                    nat_sub(a967, a968)
                });
                let m969 = f330(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m969 {
                    Ok(v8) => {
                        if f230(v0.clone(), {
                            let (_, h970) = v8.clone();
                            h970
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h971, _) = v8;
                                h971
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a973 = {
                                    let (_, h972) = v8;
                                    h972
                                };
                                let a974 = 2u64;
                                nat_add(a973, a974)?
                            })))
                        } else {
                            let m978 = f232(v0.clone(), {
                                let a976 = {
                                    let (_, h975) = v8;
                                    h975
                                };
                                let a977 = 1u64;
                                nat_add(a976, a977)?
                            }, v4, 1u64)?;
                            match m978 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a980 = {
                                            let (_, h979) = v10.clone();
                                            h979
                                        };
                                        let a981 = 2u64;
                                        nat_add(a980, a981)?
                                    };
                                    let v12: u64 = f329(v0.clone(), v11)?;
                                    if !f37(f32(v0.clone(), v11, {
                                        let a982 = v12;
                                        let a983 = v11;
                                        nat_sub(a982, a983)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h984) = v10;
                                            h984
                                        })?))
                                    } else {
                                        let v13: u64 = f324(v0.clone(), v12)?;
                                        let a985 = f231(v0, v13)?;
                                        let a986 = 62u64;
                                        if nat_eq(a985, a986) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h987, _) = v8;
                                                h987
                                            }), std::rc::Rc::new({
                                                let (h988, _) = v10;
                                                h988
                                            })), {
                                                let a989 = v13;
                                                let a990 = 1u64;
                                                nat_add(a989, a990)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
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
            let a991 = f231(v0.clone(), v1)?;
            let a992 = 256u64;
            if nat_eq(a991, a992) {
                Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f230(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f230(v0.clone(), v1, Str::lit("<!--")) {
                        let m993 = f326(v0.clone(), v1)?;
                        match m993 {
                            Ok(v15) => {
                                f232(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt18, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f230(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m997 = f323(v0.clone(), {
                                let a994 = Str::lit("]]>");
                                utf8_encode(a994)
                            }, {
                                let a995 = v1;
                                let a996 = 9u64;
                                nat_add(a995, a996)?
                            })?;
                            match m997 {
                                None => {
                                    Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m1000 = f232(v0.clone(), {
                                        let a998 = v16;
                                        let a999 = 3u64;
                                        nat_add(a998, a999)?
                                    }, v4, 1u64)?;
                                    match m1000 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f32(v0, {
                                                let a1001 = v1;
                                                let a1002 = 9u64;
                                                nat_add(a1001, a1002)?
                                            }, {
                                                let a1005 = v16;
                                                let a1006 = {
                                                    let a1003 = v1;
                                                    let a1004 = 9u64;
                                                    nat_add(a1003, a1004)?
                                                };
                                                nat_sub(a1005, a1006)
                                            }))), std::rc::Rc::new({
                                                let (h1007, _) = v18;
                                                h1007
                                            })), {
                                                let (_, h1008) = v18;
                                                h1008
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt18, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f230(v0.clone(), v1, Str::lit("<?")) {
                                let m1009 = f327(v0.clone(), v1)?;
                                match m1009 {
                                    Ok(v20) => {
                                        f232(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f230(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a1010 = f231(v0.clone(), v1)?;
                                    let a1011 = 60u64;
                                    if nat_eq(a1010, a1011) {
                                        let m1012 = f232(v0.clone(), v1, v4, 0u64)?;
                                        match m1012 {
                                            Ok(v22) => {
                                                let m1014 = f232(v0, {
                                                    let (_, h1013) = v22;
                                                    h1013
                                                }, v4, 1u64)?;
                                                match m1014 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h1015, _) = v22;
                                                            h1015
                                                        }), std::rc::Rc::new({
                                                            let (h1016, _) = v24;
                                                            h1016
                                                        })), {
                                                            let (_, h1017) = v24;
                                                            h1017
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
                                        let m1018 = f331(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m1018 {
                                            Ok(v26) => {
                                                let m1020 = f232(v0, {
                                                    let (_, h1019) = v26;
                                                    h1019
                                                }, v4, 1u64)?;
                                                match m1020 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h1021, _) = v26;
                                                            h1021
                                                        })), std::rc::Rc::new({
                                                            let (h1022, _) = v28;
                                                            h1022
                                                        })), {
                                                            let (_, h1023) = v28;
                                                            h1023
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

pub fn f233(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1024 = v2;
    let a1025 = 0u64;
    if nat_eq(a1024, a1025) {
        Ok(f332(v0, 0u64))
    } else {
        let (_, h1026) = {
            let (h1027, _) = f319(Fn5::F333(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1027
        };
        Ok(h1026)
    }
}

pub fn f234() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f235(v0: Adt18, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m1028 = v0;
    match m1028 {
        Adt18::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt18::C1(v2, v3, r1033) => {
            let v4 = (*r1033).clone();
            let m1029 = f235(v4, v1);
            match m1029 {
                Ok(v6) => {
                    if f37(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f37(v3.clone(), f334()) {
                            true
                        } else {
                            f37(v3.clone(), f335())
                        } {
                            Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f209(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a1031 = {
                                    let a1030 = v2;
                                    length_bytes(a1030)
                                };
                                let a1032 = 6u64;
                                nat_sub(a1031, a1032)
                            });
                            if f37(v7.clone(), f8(Str::lit("xmlns"))) {
                                Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f37(v7.clone(), f8(Str::lit("xml"))) {
                                    !f37(v3.clone(), f334())
                                } else {
                                    false
                                } {
                                    Err::<List<(Bytes, Bytes)>, Bytes>(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f37(v3.clone(), f334()) {
                                        true
                                    } else {
                                        f37(v3.clone(), f335())
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

pub fn f236(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f336(v1.clone())?;
    if f37(v2.clone(), Bytes::lit(&[])) {
        let m1034 = f337(v0, Bytes::lit(&[]));
        match m1034 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f37(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f334(), f338(v1)?)))
        } else {
            let m1035 = f337(v0, v2.clone());
            match m1035 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f338(v1)?)))
                }
            }
        }
    }
}

pub fn f237(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f37({
        let (h1036, _) = v0;
        h1036
    }, f339()) {
        f37({
            let (_, h1037) = v0;
            h1037
        }, {
            let a1038 = v1;
            utf8_encode(a1038)
        })
    } else {
        false
    }
}

pub fn f238(p1042: Adt18, v1: Bytes) -> Option<Bytes> {
    let m1041 = p1042;
    {
        let mut m1041 = m1041;
        loop {
            let step = {
                let v0 = m1041;
                let m1039 = v0;
                match m1039 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt18::C1(v2, v3, r1040) => {
                        let v4 = (*r1040).clone();
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
                Ok(next) => { m1041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: Adt18, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m1043 = v0;
    match m1043 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt18::C3(r1054, r1055) => {
            let v3 = (*r1054).clone();
            let v4 = (*r1055).clone();
            let m1044 = f239(v4, v1.clone(), v2.clone())?;
            match m1044 {
                Ok(v6) => {
                    let m1045 = v3;
                    match m1045 {
                        Adt18::C0(v7, r1052, r1053) => {
                            let v8 = (*r1052).clone();
                            let v9 = (*r1053).clone();
                            let m1046 = f235(v8.clone(), v1);
                            match m1046 {
                                Ok(v11) => {
                                    let m1047 = f236(v11.clone(), v7)?;
                                    match m1047 {
                                        Ok(v13) => {
                                            if f237(v13, Str::lit("simpleType")) {
                                                let m1048 = f238(v8, f8(Str::lit("name")));
                                                match m1048 {
                                                    None => {
                                                        Ok(Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m1049 = f340(v14.clone(), v9, v11, v2)?;
                                                        match m1049 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f256(v14)?, ({
                                                                    let (_, h1050) = v16;
                                                                    h1050
                                                                }, {
                                                                    let (h1051, _) = v16;
                                                                    h1051
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

pub fn f240(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1056 = v0;
    match m1056.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1057, _) = v1;
                h1057
            }, f240(v2))
        }
    }
}

pub fn f241(p1060: List<Bytes>) -> Option<Bytes> {
    let m1059 = p1060;
    {
        let mut m1059 = m1059;
        loop {
            let step = {
                let v0 = m1059;
                let m1058 = v0;
                match m1058.uncons() {
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
                Ok(next) => { m1059 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(p1066: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1065 = p1066;
    {
        let mut m1065 = m1065;
        loop {
            let step = {
                let v0 = m1065;
                let m1061 = v0;
                match m1061.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1064 = f341({
                            let (h1062, _) = {
                                let (_, h1063) = v2;
                                h1063
                            };
                            h1062
                        }, v1.clone());
                        match m1064 {
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
                Ok(next) => { m1065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(p1077: u64, p1078: List<(Bytes, (List<Bytes>, Bytes))>, p1079: List<Bytes>, p1080: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1076 = (p1077, (p1078, (p1079, p1080)));
    {
        let mut m1076 = m1076;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1076;
                let m1067 = v0;
                if m1067 == 0 {
                    let m1068 = v1;
                    match m1068.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1067 - 1;
                    let m1069 = v1.clone();
                    match m1069.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1070 = f342(v1, v2.clone());
                            match m1070 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1071) = v9.clone();
                                        h1071
                                    }, (List::cons({
                                        let (h1072, _) = {
                                            let (h1073, _) = v9.clone();
                                            h1073
                                        };
                                        h1072
                                    }, v2), List::cons({
                                        let (_, h1074) = {
                                            let (h1075, _) = v9;
                                            h1075
                                        };
                                        h1074
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1076 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(p1083: List<Bytes>, p1084: List<Bytes>) -> List<Bytes> {
    let m1082 = (p1083, p1084);
    {
        let mut m1082 = m1082;
        loop {
            let step = {
                let (v0, v1) = m1082;
                let m1081 = v0;
                match m1081.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1082 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f317(v0.clone())?;
    let v2: u64 = f343(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a1085 = v1;
        let a1086 = v2;
        nat_sub(a1085, a1086)
    }))
}

pub fn f246(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f344(v0)?;
    let m1090 = f345(v1.clone(), 0u64, {
        let a1088 = {
            let a1087 = v1.clone();
            length_bytes(a1087)
        };
        let a1089 = 2u64;
        nat_add(a1088, a1089)?
    }, 0u64)?;
    match m1090 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a1093 = {
                let (_, h1091) = v2;
                h1091
            };
            let a1094 = {
                let a1092 = v1;
                length_bytes(a1092)
            };
            if nat_eq(a1093, a1094) {
                Ok(Some({
                    let (h1095, _) = v2;
                    h1095
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f247(v0: Bytes) -> Bytes {
    f7(List::cons(f79(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f248(v0: Adt14) -> bool {
    if f207(v0.clone(), f8(Str::lit("domain"))) {
        if f207(v0.clone(), f8(Str::lit("row"))) {
            if f88(f161(v0.clone(), f8(Str::lit("relation"))), f346()) {
                if f347(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f347(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f347(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f347(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f249(v0: Adt14) -> bool {
    if f207(v0.clone(), f8(Str::lit("subject"))) {
        if f207(v0.clone(), f8(Str::lit("object"))) {
            if f207(v0.clone(), f8(Str::lit("evidence"))) {
                f88(f161(v0, f8(Str::lit("relation"))), f348())
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

pub fn f250(v0: Option<List<Adt14>>) -> bool {
    let m1096 = v0;
    match m1096 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f251(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1097 = v0;
    match m1097 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f252(v0: Adt14) -> Bytes {
    if f207(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f207(v0.clone(), f8(Str::lit("id"))) {
            if f207(v0.clone(), f8(Str::lit("statement"))) {
                if f207(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f262(v0, f8(Str::lit("evidence")))
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

pub fn f253(v0: List<Adt14>) -> Adt14 {
    let m1098 = v0;
    match m1098.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f253(v2)))
        }
    }
}

pub fn f254(v0: Bytes) -> R<Option<Adt19>> {
    let v1: Bytes = f344(v0)?;
    let m1102 = f349(v1.clone(), 0u64, {
        let a1100 = {
            let a1099 = v1.clone();
            length_bytes(a1099)
        };
        let a1101 = 2u64;
        nat_add(a1100, a1101)?
    }, 0u64)?;
    match m1102 {
        None => {
            Ok(None::<Adt19>)
        }
        Some(v2) => {
            let a1105 = {
                let (_, h1103) = v2;
                h1103
            };
            let a1106 = {
                let a1104 = v1;
                length_bytes(a1104)
            };
            if nat_eq(a1105, a1106) {
                Ok(Some({
                    let (h1107, _) = v2;
                    h1107
                }))
            } else {
                Ok(None::<Adt19>)
            }
        }
    }
}

pub fn f255(v0: Adt19) -> bool {
    if f350(v0.clone(), f8(Str::lit("name"))) {
        if f351(v0.clone(), f8(Str::lit("schema"))) {
            let m1108 = f257(f155(f156(f151(v0, f8(Str::lit("citation"))))));
            match m1108 {
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

pub fn f256(v0: Bytes) -> R<Bytes> {
    f352(v0.clone(), 0u64, {
        let a1109 = v0;
        length_bytes(a1109)
    }, true, Bytes::lit(&[]))
}

pub fn f257(v0: List<Bytes>) -> Option<Adt14> {
    let m1110 = v0;
    match m1110.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit("req"))) {
                let m1111 = v2;
                match m1111.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f37(v1.clone(), f8(Str::lit("std"))) {
                    let m1112 = v2;
                    match m1112.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1113 = v6;
                                match m1113.uncons() {
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
                        let m1114 = v2;
                        match m1114.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1115 = v10;
                                match m1115.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f221(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f258(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1116 = f62(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1116.uncons() {
        None => {
            let m1117 = f257(v4);
            match m1117 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1118 = f8(Str::lit("Catalog."));
                        let a1119 = f256(v3.clone())?;
                        append_bytes(a1118, a1119)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f256(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1120 = f353(v2, v9.clone(), f256(v3.clone())?)?;
                    match m1120 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f256(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f220(Adt14::C3(std::rc::Rc::new(f253(f354(f75(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f220(Adt14::C3(std::rc::Rc::new(f253(f354(f75(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1124 = f355(f72(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a1122 = {
                                        let a1121 = v10;
                                        length_bytes(a1121)
                                    };
                                    let a1123 = 4u64;
                                    nat_sub(a1122, a1123)
                                }))?;
                                let a1125 = Bytes::lit(&[0x0a]);
                                append_bytes(a1124, a1125)
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

pub fn f259(v0: Adt19) -> R<Bytes> {
    let m1126 = v0;
    match m1126 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f356(v2, v3)
        }
        Adt19::C2(v4, v5, v6) => {
            f357(v4, v5, v6)
        }
        Adt19::C3(v7) => {
            f358(v7)
        }
        Adt19::C4(r1127) => {
            let v8 = (*r1127).clone();
            Ok(f359(f360(v8)?))
        }
        Adt19::C5(r1128) => {
            let v9 = (*r1128).clone();
            f361(f362(v9)?)
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

pub fn f260(v0: Option<Adt19>) -> Option<Bytes> {
    let m1129 = v0;
    match m1129 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1130 = v1;
            match m1130 {
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

pub fn f261(v0: Adt19) -> List<Adt19> {
    let m1131 = v0;
    match m1131 {
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
        Adt19::C4(r1132) => {
            let v3 = (*r1132).clone();
            f261(v3)
        }
        Adt19::C5(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C6 => {
            List::<Adt19>::nil()
        }
        Adt19::C7(r1133, r1134) => {
            let v1 = (*r1133).clone();
            let v2 = (*r1134).clone();
            List::cons(v1, f261(v2))
        }
        Adt19::C8(_, _, _) => {
            List::<Adt19>::nil()
        }
    }
}

pub fn f262(v0: Adt14, v1: Bytes) -> bool {
    let m1135 = f163(v0, v1);
    match m1135 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f263(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1136 = f163(v0, v1);
    match m1136 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f160(f163(v3, v2))
        }
    }
}

pub fn f264(v0: Adt14) -> List<Adt14> {
    let m1137 = v0;
    match m1137 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1138) => {
            let v3 = (*r1138).clone();
            f264(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1139, r1140) => {
            let v1 = (*r1139).clone();
            let v2 = (*r1140).clone();
            List::cons(v1, f264(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f265(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f163(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f118(Str::lit("id"), f161(v0.clone(), f8(Str::lit("id"))))?, List::cons(f118(Str::lit("name"), f161(v0.clone(), f8(Str::lit("name"))))?, List::cons(f118(Str::lit("issuer"), f161(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f118(Str::lit("edition"), f161(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f118(Str::lit("canonical_id"), f161(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f118(Str::lit("source_role"), f161(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f118(Str::lit("retrieval_date"), f161(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f118(Str::lit("url"), f363(v2.clone(), f8(Str::lit("url"))))?, List::cons(f118(Str::lit("media_type"), f363(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f118(Str::lit("license"), f363(v2.clone(), f8(Str::lit("license"))))?, List::cons(f118(Str::lit("redistribution"), f363(v2, f8(Str::lit("redistribution"))))?, List::cons(f118(Str::lit("statement"), f161(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1141 = v1;
        match m1141 {
            None => {
                f7(List::cons(f118(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f118(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f118(Str::lit("artifact"), {
                    let (h1142, _) = v3;
                    h1142
                })?, List::cons(f118(Str::lit("checksum"), {
                    let (_, h1143) = v3;
                    h1143
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f364(f76(f160(f163(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f266(v0: Bytes, v1: u64) -> Option<u8> {
    let a1144 = v0;
    let a1145 = v1;
    index_bytes(a1144, a1145)
}

pub fn f267(v0: u64) -> bool {
    let a1146 = v0;
    let a1147 = 127u64;
    nat_le(a1146, a1147)
}

pub fn f268(v0: Bytes) -> bool {
    f138(v0, f8(Str::lit(".lex.tex")))
}

pub fn f269(v0: Fn3, p1152: u64, p1153: u64) -> R<(u64, bool)> {
    let m1151 = (p1152, p1153);
    {
        let mut m1151 = m1151;
        loop {
            let step = {
                let (v1, v2) = m1151;
                let m1148 = v1;
                if m1148 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1148 - 1;
                    let m1150 = {
                        let c1149: Fn3 = v0.clone();
                        c1149.apply(v2)?
                    };
                    match m1150 {
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
                Ok(next) => { m1151 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f224(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1156 = {
            let a1154 = v4;
            let a1155 = v1;
            nat_add(a1154, a1155)?
        };
        let a1157 = v3;
        if nat_lt(a1156, a1157) {
            Ok(Some({
                let a1158 = v4;
                let a1159 = 1u64;
                nat_add(a1158, a1159)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f271(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1160 = v0;
    match m1160.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f271(v3, {
                let a1161 = v1;
                let a1162 = 1u64;
                nat_add(a1161, a1162)?
            })?))
        }
    }
}

pub fn f272(v0: Adt16, v1: List<u64>, v2: Adt21) -> bool {
    let m1163 = v2;
    match m1163 {
        Adt21::C0(v3, v4) => {
            if f278(v3, match v0.clone() {
                Adt16::C0(h1164, _, _, _) => {
                    h1164
                }
            }) {
                f365(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f278(v5, match v0 {
                Adt16::C0(_, _, h1165, _) => {
                    h1165
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f274(v7, v1) {
                f278(v8, match v0 {
                    Adt16::C0(_, _, _, h1166) => {
                        h1166
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f273(v0: Adt12, v1: Adt14) -> R<Adt21> {
    let m1167 = f163(v1.clone(), f8(Str::lit("std")));
    match m1167 {
        None => {
            let m1172 = f164(f163(v1.clone(), f8(Str::lit("req"))));
            match m1172 {
                None => {
                    let m1174 = f163(v1, f8(Str::lit("der")));
                    match m1174 {
                        None => {
                            Ok(f366(v0))
                        }
                        Some(v5) => {
                            let m1175 = f367(f161(v5.clone(), f8(Str::lit("rule"))));
                            match m1175 {
                                None => {
                                    Ok(f366(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f181(f76(f160(f163(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h1176) => {
                                            h1176
                                        }
                                    })?, f276(f161(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h1177, _) => {
                                            h1177
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f276(v4, match v0 {
                        Adt12::C0(_, _, h1173, _, _) => {
                            h1173
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1168 = f164(f163(v2.clone(), f8(Str::lit("row"))));
            match m1168 {
                None => {
                    Ok(Adt21::C0(f276(f161(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h1169, _, _, _, _) => {
                            h1169
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f276(f161(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h1170, _, _, _, _) => {
                            h1170
                        }
                    })?, Some(f276(v3, match v0 {
                        Adt12::C0(_, h1171, _, _, _) => {
                            h1171
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f274(p1180: List<u64>, v1: List<u64>) -> bool {
    let m1179 = p1180;
    {
        let mut m1179 = m1179;
        loop {
            let step = {
                let v0 = m1179;
                let m1178 = v0;
                match m1178.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f278(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f275(p1185: List<u64>) -> bool {
    let m1184 = p1185;
    {
        let mut m1184 = m1184;
        loop {
            let step = {
                let v0 = m1184;
                let m1181 = v0;
                match m1181.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1182 = v1;
                        let a1183 = 7u64;
                        if nat_le(a1182, a1183) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1186 = f368(v0, v1.clone(), 0u64)?;
    match m1186 {
        None => {
            let a1187 = v1;
            Ok(length_list(a1187))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f277(v0: Adt14) -> u64 {
    let m1188 = f369(f163(v0, f8(Str::lit("stratum"))));
    match m1188 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f278(v0: u64, p1193: List<u64>) -> bool {
    let m1192 = p1193;
    {
        let mut m1192 = m1192;
        loop {
            let step = {
                let v1 = m1192;
                let m1189 = v1;
                match m1189.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1190 = v2;
                        let a1191 = v0;
                        if nat_eq(a1190, a1191) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1192 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f209(v0.clone(), v1) {
        let m1194 = f30(v0, 34u8)?;
        match m1194.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1195 = v3;
                match m1195.uncons() {
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

pub fn f280(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f281(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1196 = v0.clone();
    match m1196.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1197 = v2;
            let a1198 = v1.clone();
            if equal(a1197, a1198) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f282(v0: List<(Bytes, ())>) -> List<Bytes> {
    f186(f370(Fn6::F371, List::<Bytes>::nil(), v0))
}

pub fn f283(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f372(f373(Fn7::F374, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1203 = {
        let (h1199, _) = f375(Fn8::F376, {
            let a1201 = {
                let a1200 = v0;
                length_list(a1200)
            };
            let a1202 = 1u64;
            nat_add(a1201, a1202)?
        }, v1)?;
        h1199
    };
    match m1203.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f284(v0: List<Bytes>) -> List<(Bytes, ())> {
    f377(f378(Fn9::F379, List::<(Bytes, ())>::nil(), v0))
}

pub fn f285(p1206: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1205 = p1206;
    {
        let mut m1205 = m1205;
        loop {
            let step = {
                let v0 = m1205;
                let m1204 = v0;
                match m1204.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f315(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
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

pub fn f286(v0: Adt14, v1: Bytes) -> Bytes {
    if f37(f363(f163(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f287(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1207, _) = v1.clone();
        h1207
    };
    let v3: u64 = {
        let (h1208, _) = {
            let (_, h1209) = v1.clone();
            h1209
        };
        h1208
    };
    let v4: List<Bytes> = {
        let (_, h1210) = {
            let (_, h1211) = v1;
            h1211
        };
        h1210
    };
    let m1212 = f266(v0.clone(), v2);
    match m1212 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1213 = v5;
            let a1214 = 10u8;
            if equal(a1213, a1214) {
                Ok(Some(({
                    let a1215 = v2;
                    let a1216 = 1u64;
                    nat_add(a1215, a1216)?
                }, ({
                    let a1217 = v2;
                    let a1218 = 1u64;
                    nat_add(a1217, a1218)?
                }, List::cons(f380(f32(v0, v3, {
                    let a1219 = v2;
                    let a1220 = v3;
                    nat_sub(a1219, a1220)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1221 = v2;
                    let a1222 = 1u64;
                    nat_add(a1221, a1222)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f288(v0: Adt1, v1: Bytes) -> R<Adt19> {
    let m1223 = f150(v0, v1)?;
    match m1223 {
        None => {
            Ok(Adt19::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f289(v0: Bytes) -> R<Bytes> {
    let a1224 = f8(Str::lit("Catalog."));
    let a1225 = f256(v0)?;
    Ok(append_bytes(a1224, a1225))
}

pub fn f290(v0: Adt19) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f256(f154(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f291(v0: List<Bytes>) -> List<Bytes> {
    let m1226 = v0;
    match m1226.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f138(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f134(f142(), v1), f291(v2))
            } else {
                f291(v2)
            }
        }
    }
}

pub fn f292(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1227 = v1;
    match m1227.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f381(v0.clone(), v2)?;
            let (h1228, _) = v4.clone();
            if h1228 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f292(v0, v3)?;
                let (h1229, _) = v5.clone();
                if h1229 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1232 = {
                            let (_, h1230) = v4;
                            h1230
                        };
                        let a1233 = {
                            let (_, h1231) = v5;
                            h1231
                        };
                        append_bytes(a1232, a1233)
                    }))
                }
            }
        }
    }
}

pub fn f293(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1234 = v1;
    match m1234.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f207(v2.clone(), f8(Str::lit("subject"))) {
                if f208(v0.clone(), v2.clone()) {
                    f293(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f161(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f161(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f161(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f293(v0, v3))
                }
            } else {
                if f302(v0.clone(), v2.clone()) {
                    f293(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f161(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f303(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f293(v0, v3))
                }
            }
        }
    }
}

pub fn f294(v0: Adt1) -> R<Bytes> {
    let a1236 = {
        let a1235 = f105(f106(f124(f125(f114(f72(v0.clone(), f119()))?)), f107(f75(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f108(f75(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1235)
    };
    let a1237 = 0u64;
    if nat_eq(a1236, a1237) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f295(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1238 = v0;
    match m1238.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f123(f76(f160(f163(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f161(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f161(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f382(f85(f161(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f295(v2)?))
        }
    }
}

pub fn f296(v0: Bytes, v1: List<Adt23>) -> List<Bytes> {
    let m1239 = v1;
    match m1239.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(match v2.clone() {
                Adt23::C0(h1240, _, _) => {
                    h1240
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt23::C0(_, h1241, _) => {
                        h1241
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt23::C0(_, _, h1242) => {
                        h1242
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f296(v0, v3))
            } else {
                f296(v0, v3)
            }
        }
    }
}

pub fn f297(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1243 = v1;
    match m1243.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(f161(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f161(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f161(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f297(v0, v3))
            } else {
                if f37(f161(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f161(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f161(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f297(v0, v3))
                } else {
                    f297(v0, v3)
                }
            }
        }
    }
}

pub fn f298(p1248: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1247 = p1248;
    {
        let mut m1247 = m1247;
        loop {
            let step = {
                let v0 = m1247;
                let m1244 = v0;
                match m1244.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1245 = v2;
                        match m1245.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f37(v4, v1.clone()) {
                                    let m1246 = v5;
                                    match m1246.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f383(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1249 = v0;
    match m1249.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1250 = f163(v1, f8(Str::lit("std")));
            match m1250 {
                None => {
                    f299(v2)
                }
                Some(v3) => {
                    if f207(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f161(v3.clone(), f8(Str::lit("row"))), f161(v3, f8(Str::lit("authority")))), f299(v2))
                    } else {
                        f299(v2)
                    }
                }
            }
        }
    }
}

pub fn f300(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1251 = v1;
    match m1251.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f161(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f161(v2, f8(Str::lit("object")));
            if if f88(v4.clone(), match v0.clone() {
                Adt15::C0(h1252, _, _) => {
                    h1252
                }
            }) {
                f88(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1253, _) => {
                        h1253
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f300(v0, v3))
            } else {
                if if f88(v5.clone(), match v0.clone() {
                    Adt15::C0(h1254, _, _) => {
                        h1254
                    }
                }) {
                    f88(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1255, _) => {
                            h1255
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f300(v0, v3))
                } else {
                    f300(v0, v3)
                }
            }
        }
    }
}

pub fn f301(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt23>> {
    let m1256 = v0;
    match m1256.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v4, v5)) => {
            if if f384(v4.clone(), v2.clone()) {
                true
            } else {
                f384(v4.clone(), v3.clone())
            } {
                f301(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt23::C0({
                    let (h1257, _) = v4.clone();
                    h1257
                }, {
                    let a1260 = {
                        let a1259 = f80(f385({
                            let (h1258, _) = v4.clone();
                            h1258
                        }, v1.clone()))?;
                        length_list(a1259)
                    };
                    let a1261 = 1u64;
                    if nat_eq(a1260, a1261) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1262) = v4.clone();
                    h1262
                }), f301(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f302(v0: Adt15, v1: Adt14) -> bool {
    if f88(f161(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1263, _, _) => {
            h1263
        }
    }) {
        f88(f303(v1), match v0 {
            Adt15::C0(_, h1264, _) => {
                h1264
            }
        })
    } else {
        false
    }
}

pub fn f303(v0: Adt14) -> Bytes {
    let m1265 = f163(v0, f8(Str::lit("evidence")));
    match m1265 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f161(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f304(v0: Adt14) -> Bytes {
    if f207(v0.clone(), f8(Str::lit("req"))) {
        let a1266 = f8(Str::lit("req "));
        let a1267 = f161(v0, f8(Str::lit("req")));
        append_bytes(a1266, a1267)
    } else {
        let m1268 = f163(v0.clone(), f8(Str::lit("std")));
        match m1268 {
            None => {
                let m1271 = f163(v0, f8(Str::lit("der")));
                match m1271 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f161(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f123(f76(f160(f163(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f161(v1.clone(), f8(Str::lit("authority"))), List::cons(if f207(v1.clone(), f8(Str::lit("row"))) {
                    let a1269 = f8(Str::lit(" row "));
                    let a1270 = f161(v1, f8(Str::lit("row")));
                    append_bytes(a1269, a1270)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f305(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1272 = v1;
    match m1272.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f88({
                let (h1273, _) = v2.clone();
                h1273
            }, v0.clone()) {
                List::cons({
                    let (_, h1274) = v2;
                    h1274
                }, f305(v0, v3))
            } else {
                if f88({
                    let (_, h1275) = v2;
                    h1275
                }, v0.clone()) {
                    List::cons({
                        let (h1276, _) = v2;
                        h1276
                    }, f305(v0, v3))
                } else {
                    f305(v0, v3)
                }
            }
        }
    }
}

pub fn f306(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f88(v2.clone(), match v0.clone() {
        Adt15::C0(h1277, _, _) => {
            h1277
        }
    }) {
        true
    } else {
        if f88(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1278, _) => {
                h1278
            }
        }) {
            true
        } else {
            if f88(v2, match v0.clone() {
                Adt15::C0(_, _, h1279) => {
                    h1279
                }
            }) {
                true
            } else {
                if f207(v1.clone(), f8(Str::lit("scheme"))) {
                    f88(f161(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1280, _) => {
                            h1280
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f307(v0: Bytes, v1: u64) -> R<u64> {
    let m1283 = {
        let a1281 = v0;
        let a1282 = v1;
        index_bytes(a1281, a1282)
    };
    match m1283 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1328 = {
                let a1304 = {
                    let a1292 = {
                        let a1286 = {
                            let a1284 = v3;
                            let a1285 = 128u8;
                            fixed_u8::bit_and(a1284, a1285)
                        };
                        let a1287 = 128u8;
                        if equal(a1286, a1287) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1293 = {
                        let a1290 = {
                            let a1288 = v3;
                            let a1289 = 64u8;
                            fixed_u8::bit_and(a1288, a1289)
                        };
                        let a1291 = 64u8;
                        if equal(a1290, a1291) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1292, a1293)?
                };
                let a1305 = {
                    let a1302 = {
                        let a1296 = {
                            let a1294 = v3;
                            let a1295 = 32u8;
                            fixed_u8::bit_and(a1294, a1295)
                        };
                        let a1297 = 32u8;
                        if equal(a1296, a1297) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1303 = {
                        let a1300 = {
                            let a1298 = v3;
                            let a1299 = 16u8;
                            fixed_u8::bit_and(a1298, a1299)
                        };
                        let a1301 = 16u8;
                        if equal(a1300, a1301) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1302, a1303)?
                };
                nat_add(a1304, a1305)?
            };
            let a1329 = {
                let a1326 = {
                    let a1314 = {
                        let a1308 = {
                            let a1306 = v3;
                            let a1307 = 8u8;
                            fixed_u8::bit_and(a1306, a1307)
                        };
                        let a1309 = 8u8;
                        if equal(a1308, a1309) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1315 = {
                        let a1312 = {
                            let a1310 = v3;
                            let a1311 = 4u8;
                            fixed_u8::bit_and(a1310, a1311)
                        };
                        let a1313 = 4u8;
                        if equal(a1312, a1313) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1314, a1315)?
                };
                let a1327 = {
                    let a1324 = {
                        let a1318 = {
                            let a1316 = v3;
                            let a1317 = 2u8;
                            fixed_u8::bit_and(a1316, a1317)
                        };
                        let a1319 = 2u8;
                        if equal(a1318, a1319) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1325 = {
                        let a1322 = {
                            let a1320 = v3;
                            let a1321 = 1u8;
                            fixed_u8::bit_and(a1320, a1321)
                        };
                        let a1323 = 1u8;
                        if equal(a1322, a1323) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1324, a1325)?
                };
                nat_add(a1326, a1327)?
            };
            nat_add(a1328, a1329)
        }
    }
}

pub fn f308(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1333 = {
        let a1330 = v1;
        let a1331 = v2;
        let a1332 = 1u64;
        slice_bytes(a1330, a1331, a1332)
    };
    match m1333 {
        None => {
            v0
        }
        Some(v3) => {
            let a1334 = v0;
            let a1335 = v3;
            append_bytes(a1334, a1335)
        }
    }
}

pub fn f309(p1338: List<Bytes>, p1339: List<Bytes>) -> List<Bytes> {
    let m1337 = (p1338, p1339);
    {
        let mut m1337 = m1337;
        loop {
            let step = {
                let (v0, v1) = m1337;
                let m1336 = v0;
                match m1336.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: Bytes, p1411: u64, p1412: u64, p1413: Bytes) -> R<Bytes> {
    let m1410 = (p1411, (p1412, p1413));
    {
        let mut m1410 = m1410;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1410;
                let m1340 = v2;
                if m1340 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1340 - 1;
                    let m1343 = {
                        let a1341 = v0.clone();
                        let a1342 = v1;
                        index_bytes(a1341, a1342)
                    };
                    match m1343 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1388 = {
                                    let a1364 = {
                                        let a1352 = {
                                            let a1346 = {
                                                let a1344 = v6;
                                                let a1345 = 128u8;
                                                fixed_u8::bit_and(a1344, a1345)
                                            };
                                            let a1347 = 128u8;
                                            if equal(a1346, a1347) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1353 = {
                                            let a1350 = {
                                                let a1348 = v6;
                                                let a1349 = 64u8;
                                                fixed_u8::bit_and(a1348, a1349)
                                            };
                                            let a1351 = 64u8;
                                            if equal(a1350, a1351) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1352, a1353)?
                                    };
                                    let a1365 = {
                                        let a1362 = {
                                            let a1356 = {
                                                let a1354 = v6;
                                                let a1355 = 32u8;
                                                fixed_u8::bit_and(a1354, a1355)
                                            };
                                            let a1357 = 32u8;
                                            if equal(a1356, a1357) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1363 = {
                                            let a1360 = {
                                                let a1358 = v6;
                                                let a1359 = 16u8;
                                                fixed_u8::bit_and(a1358, a1359)
                                            };
                                            let a1361 = 16u8;
                                            if equal(a1360, a1361) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1362, a1363)?
                                    };
                                    nat_add(a1364, a1365)?
                                };
                                let a1389 = {
                                    let a1386 = {
                                        let a1374 = {
                                            let a1368 = {
                                                let a1366 = v6;
                                                let a1367 = 8u8;
                                                fixed_u8::bit_and(a1366, a1367)
                                            };
                                            let a1369 = 8u8;
                                            if equal(a1368, a1369) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1375 = {
                                            let a1372 = {
                                                let a1370 = v6;
                                                let a1371 = 4u8;
                                                fixed_u8::bit_and(a1370, a1371)
                                            };
                                            let a1373 = 4u8;
                                            if equal(a1372, a1373) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1374, a1375)?
                                    };
                                    let a1387 = {
                                        let a1384 = {
                                            let a1378 = {
                                                let a1376 = v6;
                                                let a1377 = 2u8;
                                                fixed_u8::bit_and(a1376, a1377)
                                            };
                                            let a1379 = 2u8;
                                            if equal(a1378, a1379) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1385 = {
                                            let a1382 = {
                                                let a1380 = v6;
                                                let a1381 = 1u8;
                                                fixed_u8::bit_and(a1380, a1381)
                                            };
                                            let a1383 = 1u8;
                                            if equal(a1382, a1383) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1384, a1385)?
                                    };
                                    nat_add(a1386, a1387)?
                                };
                                nat_add(a1388, a1389)?
                            };
                            let a1390 = v7;
                            let a1391 = 34u64;
                            if if nat_eq(a1390, a1391) {
                                true
                            } else {
                                let a1392 = v7;
                                let a1393 = 92u64;
                                nat_eq(a1392, a1393)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1394 = v1;
                                    let a1395 = 1u64;
                                    nat_add(a1394, a1395)?
                                }, (v4, {
                                    let a1398 = v3;
                                    let a1399 = {
                                        let a1396 = Bytes::lit(&[0x5c]);
                                        let a1397 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1396, a1397)
                                    };
                                    append_bytes(a1398, a1399)
                                })))
                            } else {
                                let a1400 = v7;
                                let a1401 = 10u64;
                                if nat_eq(a1400, a1401) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1402 = v1;
                                        let a1403 = 1u64;
                                        nat_add(a1402, a1403)?
                                    }, (v4, {
                                        let a1404 = v3;
                                        let a1405 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1404, a1405)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1406 = v1;
                                        let a1407 = 1u64;
                                        nat_add(a1406, a1407)?
                                    }, (v4, {
                                        let a1408 = v3;
                                        let a1409 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1408, a1409)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: u32) -> Bytes {
    let a1442 = f386({
        let a1414 = f387(v0, 28u32);
        let a1415 = 15u32;
        fixed_u32::bit_and(a1414, a1415)
    });
    let a1443 = {
        let a1440 = f386({
            let a1416 = f387(v0, 24u32);
            let a1417 = 15u32;
            fixed_u32::bit_and(a1416, a1417)
        });
        let a1441 = {
            let a1438 = f386({
                let a1418 = f387(v0, 20u32);
                let a1419 = 15u32;
                fixed_u32::bit_and(a1418, a1419)
            });
            let a1439 = {
                let a1436 = f386({
                    let a1420 = f387(v0, 16u32);
                    let a1421 = 15u32;
                    fixed_u32::bit_and(a1420, a1421)
                });
                let a1437 = {
                    let a1434 = f386({
                        let a1422 = f387(v0, 12u32);
                        let a1423 = 15u32;
                        fixed_u32::bit_and(a1422, a1423)
                    });
                    let a1435 = {
                        let a1432 = f386({
                            let a1424 = f387(v0, 8u32);
                            let a1425 = 15u32;
                            fixed_u32::bit_and(a1424, a1425)
                        });
                        let a1433 = {
                            let a1430 = f386({
                                let a1426 = f387(v0, 4u32);
                                let a1427 = 15u32;
                                fixed_u32::bit_and(a1426, a1427)
                            });
                            let a1431 = f386({
                                let a1428 = v0;
                                let a1429 = 15u32;
                                fixed_u32::bit_and(a1428, a1429)
                            });
                            append_bytes(a1430, a1431)
                        };
                        append_bytes(a1432, a1433)
                    };
                    append_bytes(a1434, a1435)
                };
                append_bytes(a1436, a1437)
            };
            append_bytes(a1438, a1439)
        };
        append_bytes(a1440, a1441)
    };
    append_bytes(a1442, a1443)
}

pub fn f312(v0: Fn4, p1447: u64, p1448: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1446 = (p1447, p1448);
    {
        let mut m1446 = m1446;
        loop {
            let step = {
                let (v1, v2) = m1446;
                let m1444 = v1;
                if m1444 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1444 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1445: Fn4 = v0.clone();
                        c1445.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1450 = {
            let (h1449, _) = v3.clone();
            h1449
        };
        let a1451 = 64u64;
        nat_add(a1450, a1451)?
    }, f388({
        let (_, h1452) = v3;
        h1452
    }, f389(v0, v1, v2, {
        let (h1453, _) = v3;
        h1453
    })?)?))
}

pub fn f314() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f315(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1454 = f30(v1, 34u8)?;
    match m1454.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1455 = v3;
            match m1455.uncons() {
                None => {
                    Ok(f390(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f316(v0: List<Bytes>) -> List<Bytes> {
    let m1456 = v0;
    match m1456.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit("authority = []"))) {
                f316(v2)
            } else {
                List::cons(v1, f316(v2))
            }
        }
    }
}

pub fn f317(v0: Bytes) -> R<u64> {
    let (h1457, _) = f269(Fn3::F391(v0.clone()), {
        let a1458 = v0.clone();
        length_bytes(a1458)
    }, {
        let a1459 = v0;
        length_bytes(a1459)
    })?;
    Ok(h1457)
}

pub fn f318(v0: Adt14) -> R<Bytes> {
    let m1460 = v0;
    match m1460 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f356(v2, v3)
        }
        Adt14::C2(v4) => {
            f358(v4)
        }
        Adt14::C3(r1461) => {
            let v5 = (*r1461).clone();
            Ok(f359(f392(v5)?))
        }
        Adt14::C4(r1462) => {
            let v6 = (*r1462).clone();
            f361(f393(v6)?)
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

pub fn f319(v0: Fn5, p1467: u64, p1468: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1466 = (p1467, p1468);
    {
        let mut m1466 = m1466;
        loop {
            let step = {
                let (v1, v2) = m1466;
                let m1463 = v1;
                if m1463 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1463 - 1;
                    let m1465 = {
                        let c1464: Fn5 = v0.clone();
                        c1464.apply(v2.clone())?
                    };
                    match m1465 {
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
                Ok(next) => { m1466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f320(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f394(v0, v1)
}

pub fn f321(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f395(v0, v1)
}

pub fn f322(v0: u64) -> bool {
    let a1469 = v0;
    let a1470 = 32u64;
    if if nat_eq(a1469, a1470) {
        true
    } else {
        let a1471 = v0;
        let a1472 = 9u64;
        nat_eq(a1471, a1472)
    } {
        true
    } else {
        let a1473 = v0;
        let a1474 = 10u64;
        if nat_eq(a1473, a1474) {
            true
        } else {
            let a1475 = v0;
            let a1476 = 13u64;
            nat_eq(a1475, a1476)
        }
    }
}

pub fn f323(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1477, _) = f269(Fn3::F396(v1.clone(), v0.clone()), {
            let a1479 = {
                let a1478 = v0.clone();
                length_bytes(a1478)
            };
            let a1480 = 1u64;
            nat_add(a1479, a1480)?
        }, v2)?;
        h1477
    };
    if f224(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f324(v0: Bytes, v1: u64) -> R<u64> {
    let (h1481, _) = f269(Fn3::F397(v0.clone()), {
        let a1483 = {
            let a1482 = v0;
            length_bytes(a1482)
        };
        let a1484 = 1u64;
        nat_add(a1483, a1484)?
    }, v1)?;
    Ok(h1481)
}

pub fn f325(v0: Bytes) -> R<bool> {
    let a1486 = {
        let a1485 = v0.clone();
        length_bytes(a1485)
    };
    let a1487 = 5u64;
    if nat_eq(a1486, a1487) {
        let a1488 = f328(v0.clone(), 0u64)?;
        let a1489 = 85u64;
        if if nat_eq(a1488, a1489) {
            true
        } else {
            let a1490 = f328(v0.clone(), 0u64)?;
            let a1491 = 117u64;
            nat_eq(a1490, a1491)
        } {
            let a1492 = f328(v0.clone(), 1u64)?;
            let a1493 = 84u64;
            if if nat_eq(a1492, a1493) {
                true
            } else {
                let a1494 = f328(v0.clone(), 1u64)?;
                let a1495 = 116u64;
                nat_eq(a1494, a1495)
            } {
                let a1496 = f328(v0.clone(), 2u64)?;
                let a1497 = 70u64;
                if if nat_eq(a1496, a1497) {
                    true
                } else {
                    let a1498 = f328(v0.clone(), 2u64)?;
                    let a1499 = 102u64;
                    nat_eq(a1498, a1499)
                } {
                    let a1500 = f328(v0.clone(), 3u64)?;
                    let a1501 = 45u64;
                    if nat_eq(a1500, a1501) {
                        let a1502 = f328(v0, 4u64)?;
                        let a1503 = 56u64;
                        Ok(nat_eq(a1502, a1503))
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

pub fn f326(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1507 = f323(v0.clone(), {
        let a1504 = Str::lit("--");
        utf8_encode(a1504)
    }, {
        let a1505 = v1;
        let a1506 = 4u64;
        nat_add(a1505, a1506)?
    })?;
    match m1507 {
        None => {
            Ok(Err::<u64, Bytes>(f227(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1510 = f231(v0, {
                let a1508 = v2;
                let a1509 = 2u64;
                nat_add(a1508, a1509)?
            })?;
            let a1511 = 62u64;
            if nat_eq(a1510, a1511) {
                Ok(Ok::<u64, Bytes>({
                    let a1512 = v2;
                    let a1513 = 3u64;
                    nat_add(a1512, a1513)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f227(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f327(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f329(v0.clone(), {
        let a1514 = v1;
        let a1515 = 2u64;
        nat_add(a1514, a1515)?
    })?;
    let a1518 = v2;
    let a1519 = {
        let a1516 = v1;
        let a1517 = 2u64;
        nat_add(a1516, a1517)?
    };
    if nat_eq(a1518, a1519) {
        Ok(Err::<u64, Bytes>(f227(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1522 = v2;
        let a1523 = {
            let a1520 = v1;
            let a1521 = 5u64;
            nat_add(a1520, a1521)?
        };
        if if nat_eq(a1522, a1523) {
            let a1526 = f231(v0.clone(), {
                let a1524 = v1;
                let a1525 = 2u64;
                nat_add(a1524, a1525)?
            })?;
            let a1527 = 120u64;
            if if nat_eq(a1526, a1527) {
                true
            } else {
                let a1530 = f231(v0.clone(), {
                    let a1528 = v1;
                    let a1529 = 2u64;
                    nat_add(a1528, a1529)?
                })?;
                let a1531 = 88u64;
                nat_eq(a1530, a1531)
            } {
                let a1534 = f231(v0.clone(), {
                    let a1532 = v1;
                    let a1533 = 3u64;
                    nat_add(a1532, a1533)?
                })?;
                let a1535 = 109u64;
                if if nat_eq(a1534, a1535) {
                    true
                } else {
                    let a1538 = f231(v0.clone(), {
                        let a1536 = v1;
                        let a1537 = 3u64;
                        nat_add(a1536, a1537)?
                    })?;
                    let a1539 = 77u64;
                    nat_eq(a1538, a1539)
                } {
                    let a1542 = f231(v0.clone(), {
                        let a1540 = v1;
                        let a1541 = 4u64;
                        nat_add(a1540, a1541)?
                    })?;
                    let a1543 = 108u64;
                    if nat_eq(a1542, a1543) {
                        true
                    } else {
                        let a1546 = f231(v0.clone(), {
                            let a1544 = v1;
                            let a1545 = 4u64;
                            nat_add(a1544, a1545)?
                        })?;
                        let a1547 = 76u64;
                        nat_eq(a1546, a1547)
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
            Ok(Err::<u64, Bytes>(f227(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1549 = f323(v0, {
                let a1548 = Str::lit("?>");
                utf8_encode(a1548)
            }, v2)?;
            match m1549 {
                None => {
                    Ok(Err::<u64, Bytes>(f227(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1550 = v3;
                        let a1551 = 2u64;
                        nat_add(a1550, a1551)?
                    }))
                }
            }
        }
    }
}

pub fn f328(v0: Bytes, v1: u64) -> R<u64> {
    let m1554 = {
        let a1552 = v0;
        let a1553 = v1;
        index_bytes(a1552, a1553)
    };
    match m1554 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1599 = {
                let a1575 = {
                    let a1563 = {
                        let a1557 = {
                            let a1555 = v3;
                            let a1556 = 128u8;
                            fixed_u8::bit_and(a1555, a1556)
                        };
                        let a1558 = 128u8;
                        if equal(a1557, a1558) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1564 = {
                        let a1561 = {
                            let a1559 = v3;
                            let a1560 = 64u8;
                            fixed_u8::bit_and(a1559, a1560)
                        };
                        let a1562 = 64u8;
                        if equal(a1561, a1562) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1563, a1564)?
                };
                let a1576 = {
                    let a1573 = {
                        let a1567 = {
                            let a1565 = v3;
                            let a1566 = 32u8;
                            fixed_u8::bit_and(a1565, a1566)
                        };
                        let a1568 = 32u8;
                        if equal(a1567, a1568) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1574 = {
                        let a1571 = {
                            let a1569 = v3;
                            let a1570 = 16u8;
                            fixed_u8::bit_and(a1569, a1570)
                        };
                        let a1572 = 16u8;
                        if equal(a1571, a1572) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1573, a1574)?
                };
                nat_add(a1575, a1576)?
            };
            let a1600 = {
                let a1597 = {
                    let a1585 = {
                        let a1579 = {
                            let a1577 = v3;
                            let a1578 = 8u8;
                            fixed_u8::bit_and(a1577, a1578)
                        };
                        let a1580 = 8u8;
                        if equal(a1579, a1580) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1586 = {
                        let a1583 = {
                            let a1581 = v3;
                            let a1582 = 4u8;
                            fixed_u8::bit_and(a1581, a1582)
                        };
                        let a1584 = 4u8;
                        if equal(a1583, a1584) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1585, a1586)?
                };
                let a1598 = {
                    let a1595 = {
                        let a1589 = {
                            let a1587 = v3;
                            let a1588 = 2u8;
                            fixed_u8::bit_and(a1587, a1588)
                        };
                        let a1590 = 2u8;
                        if equal(a1589, a1590) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1596 = {
                        let a1593 = {
                            let a1591 = v3;
                            let a1592 = 1u8;
                            fixed_u8::bit_and(a1591, a1592)
                        };
                        let a1594 = 1u8;
                        if equal(a1593, a1594) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1595, a1596)?
                };
                nat_add(a1597, a1598)?
            };
            nat_add(a1599, a1600)
        }
    }
}

pub fn f329(v0: Bytes, v1: u64) -> R<u64> {
    let (h1601, _) = f269(Fn3::F398(v0.clone(), v1), {
        let a1603 = {
            let a1602 = v0;
            length_bytes(a1602)
        };
        let a1604 = 1u64;
        nat_add(a1603, a1604)?
    }, v1)?;
    Ok(h1601)
}

pub fn f330(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1605 = v2;
    if m1605 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1605 - 1;
        let v5: u64 = f324(v0.clone(), v1)?;
        let a1606 = f231(v0.clone(), v5)?;
        let a1607 = 62u64;
        if if nat_eq(a1606, a1607) {
            true
        } else {
            f230(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1608 = v5;
            let a1609 = v1;
            if nat_eq(a1608, a1609) {
                Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f329(v0.clone(), v5)?;
                let a1610 = v6;
                let a1611 = v5;
                if nat_eq(a1610, a1611) {
                    Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1612 = v6;
                        let a1613 = v5;
                        nat_sub(a1612, a1613)
                    });
                    if f399(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f324(v0.clone(), v6)?;
                        let a1614 = f231(v0.clone(), v8)?;
                        let a1615 = 61u64;
                        if !nat_eq(a1614, a1615) {
                            Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f324(v0.clone(), {
                                let a1616 = v8;
                                let a1617 = 1u64;
                                nat_add(a1616, a1617)?
                            })?;
                            let v10: u64 = f231(v0.clone(), v9)?;
                            let a1618 = v10;
                            let a1619 = 34u64;
                            if !if nat_eq(a1618, a1619) {
                                true
                            } else {
                                let a1620 = v10;
                                let a1621 = 39u64;
                                nat_eq(a1620, a1621)
                            } {
                                Ok(Err::<(Adt18, u64), Bytes>(f227(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1624 = f331(v0.clone(), {
                                    let a1622 = v9;
                                    let a1623 = 1u64;
                                    nat_add(a1622, a1623)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1624 {
                                    Ok(v12) => {
                                        let m1626 = f330(v0, {
                                            let (_, h1625) = v12;
                                            h1625
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1626 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1627, _) = v12;
                                                    h1627
                                                }, std::rc::Rc::new({
                                                    let (h1628, _) = v14;
                                                    h1628
                                                })), {
                                                    let (_, h1629) = v14;
                                                    h1629
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

pub fn f331(v0: Bytes, p1669: u64, p1670: u64, p1671: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1668 = (p1669, (p1670, p1671));
    {
        let mut m1668 = m1668;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1668;
                let m1630 = v2;
                if m1630 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f227(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1630 - 1;
                    let v7: u64 = f231(v0.clone(), v1)?;
                    if if v4 {
                        let a1631 = v7;
                        let a1632 = v5;
                        nat_eq(a1631, a1632)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1633 = v1;
                            let a1634 = 1u64;
                            nat_add(a1633, a1634)?
                        }))))
                    } else {
                        let a1635 = v7;
                        let a1636 = 256u64;
                        if nat_eq(a1635, a1636) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f227(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1637 = v7;
                            let a1638 = 60u64;
                            if nat_eq(a1637, a1638) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f227(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1639 = v7;
                                let a1640 = 38u64;
                                if nat_eq(a1639, a1640) {
                                    let m1641 = f400(v0.clone(), v1)?;
                                    match m1641 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1642) = v9;
                                                h1642
                                            }, (v6, {
                                                let a1644 = v3;
                                                let a1645 = {
                                                    let (h1643, _) = v9;
                                                    h1643
                                                };
                                                append_bytes(a1644, a1645)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1646 = v7;
                                        let a1647 = 9u64;
                                        if if nat_eq(a1646, a1647) {
                                            true
                                        } else {
                                            let a1648 = v7;
                                            let a1649 = 10u64;
                                            nat_eq(a1648, a1649)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1650 = v1;
                                                let a1651 = 1u64;
                                                nat_add(a1650, a1651)?
                                            }, (v6, {
                                                let a1652 = v3;
                                                let a1653 = Bytes::lit(&[0x20]);
                                                append_bytes(a1652, a1653)
                                            })))
                                        } else {
                                            let v10: u64 = f401(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1656 = v3;
                                                let a1657 = f32(v0.clone(), v1, {
                                                    let a1654 = v10;
                                                    let a1655 = v1;
                                                    nat_sub(a1654, a1655)
                                                });
                                                append_bytes(a1656, a1657)
                                            })))
                                        }
                                    } else {
                                        let a1658 = v7;
                                        let a1659 = 93u64;
                                        if nat_eq(a1658, a1659) {
                                            if f230(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f227(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1660 = v1;
                                                    let a1661 = 1u64;
                                                    nat_add(a1660, a1661)?
                                                }, (v6, {
                                                    let a1662 = v3;
                                                    let a1663 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a1662, a1663)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f401(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1666 = v3;
                                                let a1667 = f32(v0.clone(), v1, {
                                                    let a1664 = v11;
                                                    let a1665 = v1;
                                                    nat_sub(a1664, a1665)
                                                });
                                                append_bytes(a1666, a1667)
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
                Ok(next) => { m1668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: Bytes, v1: u64) -> Bytes {
    let m1675 = {
        let a1672 = v0;
        let a1673 = v1;
        let a1674 = 1u64;
        slice_bytes(a1672, a1673, a1674)
    };
    match m1675 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f333(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f402(v1, v0, v2)
}

pub fn f334() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f335() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f336(v0: Bytes) -> R<Bytes> {
    let m1676 = f323(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1676 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f337(p1681: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1680 = p1681;
    {
        let mut m1680 = m1680;
        loop {
            let step = {
                let v0 = m1680;
                let m1677 = v0;
                match m1677.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f37({
                            let (h1678, _) = v2;
                            h1678
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1679) = v2;
                                h1679
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1680 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: Bytes) -> R<Bytes> {
    let m1682 = f323(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1682 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1683 = v1;
                let a1684 = 1u64;
                nat_add(a1683, a1684)?
            }, {
                let a1688 = {
                    let a1685 = v0;
                    length_bytes(a1685)
                };
                let a1689 = {
                    let a1686 = v1;
                    let a1687 = 1u64;
                    nat_add(a1686, a1687)?
                };
                nat_sub(a1688, a1689)
            }))
        }
    }
}

pub fn f339() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f340(v0: Bytes, v1: Adt18, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v4: Bytes = f256(v0.clone())?;
    let m1690 = f403(v1, v2)?;
    match m1690 {
        Ok(v6) => {
            let m1691 = v6;
            match m1691 {
                None => {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = {
                        let (h1692, _) = v7.clone();
                        h1692
                    };
                    let v9: Adt18 = {
                        let (h1693, _) = {
                            let (_, h1694) = v7.clone();
                            h1694
                        };
                        h1693
                    };
                    let v10: Adt18 = {
                        let (h1695, _) = {
                            let (_, h1696) = {
                                let (_, h1697) = v7.clone();
                                h1697
                            };
                            h1696
                        };
                        h1695
                    };
                    let v11: List<(Bytes, Bytes)> = {
                        let (_, h1698) = {
                            let (_, h1699) = {
                                let (_, h1700) = v7;
                                h1700
                            };
                            h1699
                        };
                        h1698
                    };
                    if f37(v8.clone(), f8(Str::lit("union"))) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f37(v8, f8(Str::lit("list"))) {
                            let m1701 = f238(v9, f8(Str::lit("itemType")));
                            match m1701 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1702 = f405(v11, v3, v12)?;
                                    match m1702 {
                                        Ok(v14) => {
                                            if f209({
                                                let (h1703, _) = v14.clone();
                                                h1703
                                            }, f8(Str::lit("(List "))) {
                                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f406(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons({
                                                    let (h1704, _) = v14;
                                                    h1704
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), {
                                                    let (_, h1705) = v14;
                                                    h1705
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1706 = f238(v9, f8(Str::lit("base")));
                            match m1706 {
                                None => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1707 = f405(v11.clone(), v3, v15)?;
                                    match m1707 {
                                        Ok(v17) => {
                                            let m1708 = f407(v10, v11)?;
                                            match m1708 {
                                                Ok(v19) => {
                                                    let v20: List<Adt19> = f408(v19, List::<Bytes>::nil())?;
                                                    let m1709 = v20.clone();
                                                    match m1709.uncons() {
                                                        None => {
                                                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f406(v4, {
                                                                let (h1710, _) = v17;
                                                                h1710
                                                            }), {
                                                                let (_, h1711) = v17;
                                                                h1711
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1712 = f409(f410(f155(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1712 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f411(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(v16, v0)))
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
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(f404(v5, v0)))
        }
    }
}

pub fn f341(p1715: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1714 = p1715;
    {
        let mut m1714 = m1714;
        loop {
            let step = {
                let v0 = m1714;
                let m1713 = v0;
                match m1713.uncons() {
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
                Ok(next) => { m1714 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f342(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1716 = v0;
    match m1716.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1719 = f341({
                let (h1717, _) = {
                    let (_, h1718) = v2.clone();
                    h1718
                };
                h1717
            }, v1.clone());
            match m1719 {
                None => {
                    Some((({
                        let (h1720, _) = v2;
                        h1720
                    }, {
                        let (_, h1721) = {
                            let (_, h1722) = v2;
                            h1722
                        };
                        h1721
                    }), v3))
                }
                Some(_) => {
                    let m1723 = f342(v3, v1);
                    match m1723 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1724, _) = v5;
                                h1724
                            }, List::cons(v2, {
                                let (_, h1725) = v5;
                                h1725
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f343(v0: Bytes, v1: u64) -> R<u64> {
    let (h1726, _) = f269(Fn3::F412(v1, v0), v1, 0u64)?;
    Ok(h1726)
}

pub fn f344(v0: Bytes) -> R<Bytes> {
    f413(v0.clone(), 0u64, {
        let a1727 = v0;
        length_bytes(a1727)
    }, false, false, Bytes::lit(&[]))
}

pub fn f345(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1728 = v2;
    if m1728 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1728 - 1;
        let v5: u64 = f328(v0.clone(), v1)?;
        let a1729 = v3;
        let a1730 = 1u64;
        if nat_eq(a1729, a1730) {
            let m1731 = f345(v0.clone(), v1, v4, 0u64)?;
            match m1731 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f328(v0.clone(), {
                        let (_, h1732) = v6.clone();
                        h1732
                    })?;
                    let a1733 = v7;
                    let a1734 = 44u64;
                    if nat_eq(a1733, a1734) {
                        let m1738 = f345(v0, {
                            let a1736 = {
                                let (_, h1735) = v6;
                                h1735
                            };
                            let a1737 = 1u64;
                            nat_add(a1736, a1737)?
                        }, v4, 1u64)?;
                        match m1738 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1739, _) = v6;
                                    h1739
                                }), std::rc::Rc::new({
                                    let (h1740, _) = v8;
                                    h1740
                                })), {
                                    let (_, h1741) = v8;
                                    h1741
                                })))
                            }
                        }
                    } else {
                        let a1742 = v7;
                        let a1743 = 93u64;
                        if nat_eq(a1742, a1743) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1744, _) = v6;
                                h1744
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1746 = {
                                    let (_, h1745) = v6;
                                    h1745
                                };
                                let a1747 = 1u64;
                                nat_add(a1746, a1747)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1748 = v3;
            let a1749 = 2u64;
            if nat_eq(a1748, a1749) {
                let m1752 = f414(v0.clone(), {
                    let a1750 = v1;
                    let a1751 = 1u64;
                    nat_add(a1750, a1751)?
                })?;
                match m1752 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1753 = v5;
                        let a1754 = 34u64;
                        if if nat_eq(a1753, a1754) {
                            let a1756 = f328(v0.clone(), {
                                let (h1755, _) = v9.clone();
                                h1755
                            })?;
                            let a1757 = 58u64;
                            nat_eq(a1756, a1757)
                        } else {
                            false
                        } {
                            let m1761 = f345(v0.clone(), {
                                let a1759 = {
                                    let (h1758, _) = v9;
                                    h1758
                                };
                                let a1760 = 1u64;
                                nat_add(a1759, a1760)?
                            }, v4, 0u64)?;
                            match m1761 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f328(v0.clone(), {
                                        let (_, h1762) = v10.clone();
                                        h1762
                                    })?;
                                    let a1763 = v11;
                                    let a1764 = 44u64;
                                    if nat_eq(a1763, a1764) {
                                        let m1768 = f345(v0, {
                                            let a1766 = {
                                                let (_, h1765) = v10;
                                                h1765
                                            };
                                            let a1767 = 1u64;
                                            nat_add(a1766, a1767)?
                                        }, v4, 2u64)?;
                                        match m1768 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1769) = v9;
                                                    h1769
                                                }, std::rc::Rc::new({
                                                    let (h1770, _) = v10;
                                                    h1770
                                                }), std::rc::Rc::new({
                                                    let (h1771, _) = v12;
                                                    h1771
                                                })), {
                                                    let (_, h1772) = v12;
                                                    h1772
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1773 = v11;
                                        let a1774 = 125u64;
                                        if nat_eq(a1773, a1774) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1775) = v9;
                                                h1775
                                            }, std::rc::Rc::new({
                                                let (h1776, _) = v10;
                                                h1776
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1778 = {
                                                    let (_, h1777) = v10;
                                                    h1777
                                                };
                                                let a1779 = 1u64;
                                                nat_add(a1778, a1779)?
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
                let a1780 = v5;
                let a1781 = 34u64;
                if nat_eq(a1780, a1781) {
                    let m1784 = f414(v0, {
                        let a1782 = v1;
                        let a1783 = 1u64;
                        nat_add(a1782, a1783)?
                    })?;
                    match m1784 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1785) = v13;
                                h1785
                            }), {
                                let (h1786, _) = v13;
                                h1786
                            })))
                        }
                    }
                } else {
                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1787 = v1;
                            let a1788 = 4u64;
                            nat_add(a1787, a1788)?
                        })))
                    } else {
                        if f37(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1789 = v1;
                                let a1790 = 5u64;
                                nat_add(a1789, a1790)?
                            })))
                        } else {
                            let a1791 = v5;
                            let a1792 = 91u64;
                            if nat_eq(a1791, a1792) {
                                let a1795 = f328(v0.clone(), {
                                    let a1793 = v1;
                                    let a1794 = 1u64;
                                    nat_add(a1793, a1794)?
                                })?;
                                let a1796 = 93u64;
                                if nat_eq(a1795, a1796) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1797 = v1;
                                        let a1798 = 2u64;
                                        nat_add(a1797, a1798)?
                                    })))
                                } else {
                                    let m1801 = f345(v0, {
                                        let a1799 = v1;
                                        let a1800 = 1u64;
                                        nat_add(a1799, a1800)?
                                    }, v4, 1u64)?;
                                    match m1801 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1802, _) = v14;
                                                h1802
                                            })), {
                                                let (_, h1803) = v14;
                                                h1803
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1804 = v5;
                                let a1805 = 123u64;
                                if nat_eq(a1804, a1805) {
                                    let a1808 = f328(v0.clone(), {
                                        let a1806 = v1;
                                        let a1807 = 1u64;
                                        nat_add(a1806, a1807)?
                                    })?;
                                    let a1809 = 125u64;
                                    if nat_eq(a1808, a1809) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1810 = v1;
                                            let a1811 = 2u64;
                                            nat_add(a1810, a1811)?
                                        })))
                                    } else {
                                        let m1814 = f345(v0, {
                                            let a1812 = v1;
                                            let a1813 = 1u64;
                                            nat_add(a1812, a1813)?
                                        }, v4, 2u64)?;
                                        match m1814 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1815, _) = v15;
                                                    h1815
                                                })), {
                                                    let (_, h1816) = v15;
                                                    h1816
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1817 = f415(v0, v1)?;
                                    match m1817 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1818) = v16;
                                                h1818
                                            }, {
                                                let (h1819, _) = v16;
                                                h1819
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

pub fn f346() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f347(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1820 = f163(v0, v1);
    match m1820 {
        None => {
            false
        }
        Some(v3) => {
            f207(v3, v2)
        }
    }
}

pub fn f348() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f349(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt19, u64)>> {
    let m1821 = v2;
    if m1821 == 0 {
        Ok(None::<(Adt19, u64)>)
    } else {
        let v4 = m1821 - 1;
        let v5: u64 = f328(v0.clone(), v1)?;
        let a1822 = v3;
        let a1823 = 1u64;
        if nat_eq(a1822, a1823) {
            let m1824 = f349(v0.clone(), v1, v4, 0u64)?;
            match m1824 {
                None => {
                    Ok(None::<(Adt19, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f328(v0.clone(), {
                        let (_, h1825) = v6.clone();
                        h1825
                    })?;
                    let a1826 = v7;
                    let a1827 = 44u64;
                    if nat_eq(a1826, a1827) {
                        let m1831 = f349(v0, {
                            let a1829 = {
                                let (_, h1828) = v6;
                                h1828
                            };
                            let a1830 = 1u64;
                            nat_add(a1829, a1830)?
                        }, v4, 1u64)?;
                        match m1831 {
                            None => {
                                Ok(None::<(Adt19, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt19::C7(std::rc::Rc::new({
                                    let (h1832, _) = v6;
                                    h1832
                                }), std::rc::Rc::new({
                                    let (h1833, _) = v8;
                                    h1833
                                })), {
                                    let (_, h1834) = v8;
                                    h1834
                                })))
                            }
                        }
                    } else {
                        let a1835 = v7;
                        let a1836 = 93u64;
                        if nat_eq(a1835, a1836) {
                            Ok(Some((Adt19::C7(std::rc::Rc::new({
                                let (h1837, _) = v6;
                                h1837
                            }), std::rc::Rc::new(Adt19::C6)), {
                                let a1839 = {
                                    let (_, h1838) = v6;
                                    h1838
                                };
                                let a1840 = 1u64;
                                nat_add(a1839, a1840)?
                            })))
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1841 = v3;
            let a1842 = 2u64;
            if nat_eq(a1841, a1842) {
                let m1845 = f414(v0.clone(), {
                    let a1843 = v1;
                    let a1844 = 1u64;
                    nat_add(a1843, a1844)?
                })?;
                match m1845 {
                    None => {
                        Ok(None::<(Adt19, u64)>)
                    }
                    Some(v9) => {
                        let a1846 = v5;
                        let a1847 = 34u64;
                        if if nat_eq(a1846, a1847) {
                            let a1849 = f328(v0.clone(), {
                                let (h1848, _) = v9.clone();
                                h1848
                            })?;
                            let a1850 = 58u64;
                            nat_eq(a1849, a1850)
                        } else {
                            false
                        } {
                            let m1854 = f349(v0.clone(), {
                                let a1852 = {
                                    let (h1851, _) = v9;
                                    h1851
                                };
                                let a1853 = 1u64;
                                nat_add(a1852, a1853)?
                            }, v4, 0u64)?;
                            match m1854 {
                                None => {
                                    Ok(None::<(Adt19, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f328(v0.clone(), {
                                        let (_, h1855) = v10.clone();
                                        h1855
                                    })?;
                                    let a1856 = v11;
                                    let a1857 = 44u64;
                                    if nat_eq(a1856, a1857) {
                                        let m1861 = f349(v0, {
                                            let a1859 = {
                                                let (_, h1858) = v10;
                                                h1858
                                            };
                                            let a1860 = 1u64;
                                            nat_add(a1859, a1860)?
                                        }, v4, 2u64)?;
                                        match m1861 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt19::C8({
                                                    let (_, h1862) = v9;
                                                    h1862
                                                }, std::rc::Rc::new({
                                                    let (h1863, _) = v10;
                                                    h1863
                                                }), std::rc::Rc::new({
                                                    let (h1864, _) = v12;
                                                    h1864
                                                })), {
                                                    let (_, h1865) = v12;
                                                    h1865
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1866 = v11;
                                        let a1867 = 125u64;
                                        if nat_eq(a1866, a1867) {
                                            Ok(Some((Adt19::C8({
                                                let (_, h1868) = v9;
                                                h1868
                                            }, std::rc::Rc::new({
                                                let (h1869, _) = v10;
                                                h1869
                                            }), std::rc::Rc::new(Adt19::C6)), {
                                                let a1871 = {
                                                    let (_, h1870) = v10;
                                                    h1870
                                                };
                                                let a1872 = 1u64;
                                                nat_add(a1871, a1872)?
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
                let a1873 = v5;
                let a1874 = 34u64;
                if nat_eq(a1873, a1874) {
                    let m1877 = f414(v0, {
                        let a1875 = v1;
                        let a1876 = 1u64;
                        nat_add(a1875, a1876)?
                    })?;
                    match m1877 {
                        None => {
                            Ok(None::<(Adt19, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt19::C3({
                                let (_, h1878) = v13;
                                h1878
                            }), {
                                let (h1879, _) = v13;
                                h1879
                            })))
                        }
                    }
                } else {
                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt19::C0(true), {
                            let a1880 = v1;
                            let a1881 = 4u64;
                            nat_add(a1880, a1881)?
                        })))
                    } else {
                        if f37(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt19::C0(false), {
                                let a1882 = v1;
                                let a1883 = 5u64;
                                nat_add(a1882, a1883)?
                            })))
                        } else {
                            let a1884 = v5;
                            let a1885 = 91u64;
                            if nat_eq(a1884, a1885) {
                                let a1888 = f328(v0.clone(), {
                                    let a1886 = v1;
                                    let a1887 = 1u64;
                                    nat_add(a1886, a1887)?
                                })?;
                                let a1889 = 93u64;
                                if nat_eq(a1888, a1889) {
                                    Ok(Some((Adt19::C4(std::rc::Rc::new(Adt19::C6)), {
                                        let a1890 = v1;
                                        let a1891 = 2u64;
                                        nat_add(a1890, a1891)?
                                    })))
                                } else {
                                    let m1894 = f349(v0, {
                                        let a1892 = v1;
                                        let a1893 = 1u64;
                                        nat_add(a1892, a1893)?
                                    }, v4, 1u64)?;
                                    match m1894 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt19::C4(std::rc::Rc::new({
                                                let (h1895, _) = v14;
                                                h1895
                                            })), {
                                                let (_, h1896) = v14;
                                                h1896
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1897 = v5;
                                let a1898 = 123u64;
                                if nat_eq(a1897, a1898) {
                                    let a1901 = f328(v0.clone(), {
                                        let a1899 = v1;
                                        let a1900 = 1u64;
                                        nat_add(a1899, a1900)?
                                    })?;
                                    let a1902 = 125u64;
                                    if nat_eq(a1901, a1902) {
                                        Ok(Some((Adt19::C5(std::rc::Rc::new(Adt19::C6)), {
                                            let a1903 = v1;
                                            let a1904 = 2u64;
                                            nat_add(a1903, a1904)?
                                        })))
                                    } else {
                                        let m1907 = f349(v0, {
                                            let a1905 = v1;
                                            let a1906 = 1u64;
                                            nat_add(a1905, a1906)?
                                        }, v4, 2u64)?;
                                        match m1907 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt19::C5(std::rc::Rc::new({
                                                    let (h1908, _) = v15;
                                                    h1908
                                                })), {
                                                    let (_, h1909) = v15;
                                                    h1909
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1910 = f416(v0, v1)?;
                                    match m1910 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1911) = v16;
                                                h1911
                                            }, {
                                                let (h1912, _) = v16;
                                                h1912
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

pub fn f350(v0: Adt19, v1: Bytes) -> bool {
    let a1914 = {
        let a1913 = f154(v0, v1);
        length_bytes(a1913)
    };
    let a1915 = 0u64;
    !nat_eq(a1914, a1915)
}

pub fn f351(v0: Adt19, v1: Bytes) -> bool {
    let m1916 = f151(v0, v1);
    match m1916 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f352(v0: Bytes, p1983: u64, p1984: u64, p1985: bool, p1986: Bytes) -> R<Bytes> {
    let m1982 = (p1983, (p1984, (p1985, p1986)));
    {
        let mut m1982 = m1982;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1982;
                let m1917 = v2;
                if m1917 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1917 - 1;
                    let m1920 = {
                        let a1918 = v0.clone();
                        let a1919 = v1;
                        index_bytes(a1918, a1919)
                    };
                    match m1920 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1965 = {
                                    let a1941 = {
                                        let a1929 = {
                                            let a1923 = {
                                                let a1921 = v7;
                                                let a1922 = 128u8;
                                                fixed_u8::bit_and(a1921, a1922)
                                            };
                                            let a1924 = 128u8;
                                            if equal(a1923, a1924) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1930 = {
                                            let a1927 = {
                                                let a1925 = v7;
                                                let a1926 = 64u8;
                                                fixed_u8::bit_and(a1925, a1926)
                                            };
                                            let a1928 = 64u8;
                                            if equal(a1927, a1928) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1929, a1930)?
                                    };
                                    let a1942 = {
                                        let a1939 = {
                                            let a1933 = {
                                                let a1931 = v7;
                                                let a1932 = 32u8;
                                                fixed_u8::bit_and(a1931, a1932)
                                            };
                                            let a1934 = 32u8;
                                            if equal(a1933, a1934) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1940 = {
                                            let a1937 = {
                                                let a1935 = v7;
                                                let a1936 = 16u8;
                                                fixed_u8::bit_and(a1935, a1936)
                                            };
                                            let a1938 = 16u8;
                                            if equal(a1937, a1938) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1939, a1940)?
                                    };
                                    nat_add(a1941, a1942)?
                                };
                                let a1966 = {
                                    let a1963 = {
                                        let a1951 = {
                                            let a1945 = {
                                                let a1943 = v7;
                                                let a1944 = 8u8;
                                                fixed_u8::bit_and(a1943, a1944)
                                            };
                                            let a1946 = 8u8;
                                            if equal(a1945, a1946) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1952 = {
                                            let a1949 = {
                                                let a1947 = v7;
                                                let a1948 = 4u8;
                                                fixed_u8::bit_and(a1947, a1948)
                                            };
                                            let a1950 = 4u8;
                                            if equal(a1949, a1950) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1951, a1952)?
                                    };
                                    let a1964 = {
                                        let a1961 = {
                                            let a1955 = {
                                                let a1953 = v7;
                                                let a1954 = 2u8;
                                                fixed_u8::bit_and(a1953, a1954)
                                            };
                                            let a1956 = 2u8;
                                            if equal(a1955, a1956) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1962 = {
                                            let a1959 = {
                                                let a1957 = v7;
                                                let a1958 = 1u8;
                                                fixed_u8::bit_and(a1957, a1958)
                                            };
                                            let a1960 = 1u8;
                                            if equal(a1959, a1960) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1961, a1962)?
                                    };
                                    nat_add(a1963, a1964)?
                                };
                                nat_add(a1965, a1966)?
                            };
                            if f417(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1967 = v1;
                                    let a1968 = 1u64;
                                    nat_add(a1967, a1968)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f418(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1969 = v1;
                                        let a1970 = 1u64;
                                        nat_add(a1969, a1970)?
                                    }, (v5, (false, {
                                        let a1974 = v4;
                                        let a1975 = f32({
                                            let a1971 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1971)
                                        }, {
                                            let a1972 = v8;
                                            let a1973 = 97u64;
                                            nat_sub(a1972, a1973)
                                        }, 1u64);
                                        append_bytes(a1974, a1975)
                                    }))))
                                } else {
                                    if if f418(v8) {
                                        true
                                    } else {
                                        if f419(v8) {
                                            true
                                        } else {
                                            f420(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1976 = v1;
                                            let a1977 = 1u64;
                                            nat_add(a1976, a1977)?
                                        }, (v5, (false, {
                                            let a1978 = v4;
                                            let a1979 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1978, a1979)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1980 = v1;
                                            let a1981 = 1u64;
                                            nat_add(a1980, a1981)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1982 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: Adt19, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt19)> = f421(f422(f151(v0.clone(), f8(Str::lit("$defs")))), f422(f151(v0.clone(), f8(Str::lit("definitions")))));
    let m1987 = f423(4096u64, v3.clone(), List::<Bytes>::nil(), f424(v3)?)?;
    match m1987 {
        Ok(v5) => {
            let m1988 = f425(4096u64, f426(f427(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1988 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f219(f190(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f354(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1989 = v0;
    match m1989.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f37(f161(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f428(v5, v1, v2))
            } else {
                List::cons(v4, f354(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f355(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f219(f429(f85(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f356(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1990 = 0u64;
        let a1991 = v1;
        nat_lt(a1990, a1991)
    } else {
        false
    } {
        let a1992 = Bytes::lit(&[0x2d]);
        let a1993 = f128(v1)?;
        Ok(append_bytes(a1992, a1993))
    } else {
        f128(v1)
    }
}

pub fn f357(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f128(v1)?;
    let v4: u64 = {
        let a1994 = v3.clone();
        length_bytes(a1994)
    };
    let v5: Bytes = {
        let a1995 = v2;
        let a1996 = v4;
        if nat_lt(a1995, a1996) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1997 = v4;
                let a1998 = v2;
                nat_sub(a1997, a1998)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1999 = v4;
                let a2000 = v2;
                nat_sub(a1999, a2000)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f430({
                let a2001 = v2;
                let a2002 = v4;
                nat_sub(a2001, a2002)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a2003 = f8(Str::lit("-"));
        let a2004 = v5;
        Ok(append_bytes(a2003, a2004))
    } else {
        Ok(v5)
    }
}

pub fn f358(v0: Bytes) -> R<Bytes> {
    let a2010 = {
        let a2008 = Bytes::lit(&[0x22]);
        let a2009 = {
            let (_, h2005) = {
                let (h2006, _) = f319(Fn5::F431(v0.clone()), {
                    let a2007 = v0;
                    length_bytes(a2007)
                }, (0u64, Bytes::lit(&[])))?;
                h2006
            };
            h2005
        };
        append_bytes(a2008, a2009)
    };
    let a2011 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2010, a2011))
}

pub fn f359(v0: List<Bytes>) -> Bytes {
    let a2014 = {
        let a2012 = Bytes::lit(&[0x5b]);
        let a2013 = f123(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2012, a2013)
    };
    let a2015 = Bytes::lit(&[0x5d]);
    append_bytes(a2014, a2015)
}

pub fn f360(v0: Adt19) -> R<List<Bytes>> {
    let m2016 = v0;
    match m2016 {
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
        Adt19::C7(r2017, r2018) => {
            let v10 = (*r2017).clone();
            let v11 = (*r2018).clone();
            Ok(List::cons(f259(v10)?, f360(v11)?))
        }
        Adt19::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f361(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f432(Fn10::F433, List::<(Bytes, Bytes)>::nil(), f434(v0)?);
    let v2: List<Bytes> = f435(Fn11::F436, List::<Bytes>::nil(), v1)?;
    let a2021 = {
        let a2019 = Bytes::lit(&[0x7b]);
        let a2020 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2019, a2020)
    };
    let a2022 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2021, a2022))
}

pub fn f362(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m2023 = v0;
    match m2023 {
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
        Adt19::C8(v12, r2024, r2025) => {
            let v13 = (*r2024).clone();
            let v14 = (*r2025).clone();
            Ok(List::cons((v12, f259(v13)?), f362(v14)?))
        }
    }
}

pub fn f363(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m2026 = v0;
    match m2026 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f161(v2, v1)
        }
    }
}

pub fn f364(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f219(f437(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f365(v0: Adt16, v1: Option<u64>) -> bool {
    let m2027 = v1;
    match m2027 {
        None => {
            true
        }
        Some(v2) => {
            f278(v2, match v0 {
                Adt16::C0(_, h2028, _, _) => {
                    h2028
                }
            })
        }
    }
}

pub fn f366(v0: Adt12) -> Adt21 {
    Adt21::C1({
        let a2030 = match v0 {
            Adt12::C0(_, _, h2029, _, _) => {
                h2029
            }
        };
        length_list(a2030)
    })
}

pub fn f367(v0: Bytes) -> Option<Adt22> {
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

pub fn f368(v0: Bytes, p2035: List<Bytes>, p2036: u64) -> R<Option<u64>> {
    let m2034 = (p2035, p2036);
    {
        let mut m2034 = m2034;
        loop {
            let step = {
                let (v1, v2) = m2034;
                let m2031 = v1;
                match m2031.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f37(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2032 = v2;
                                let a2033 = 1u64;
                                nat_add(a2032, a2033)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: Option<Adt14>) -> Option<u64> {
    let m2037 = v0;
    match m2037 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2038 = v1;
            match m2038 {
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

pub fn f370(v0: Fn6, p2042: List<Bytes>, p2043: List<(Bytes, ())>) -> List<Bytes> {
    let m2041 = (p2042, p2043);
    {
        let mut m2041 = m2041;
        loop {
            let step = {
                let (v1, v2) = m2041;
                let m2039 = v2;
                match m2039.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c2040: Fn6 = v0.clone();
                            c2040.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h2044, _) = v1;
        h2044
    }, v0)
}

pub fn f372(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f438(Fn12::F439, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f373(v0: Fn7, p2048: List<List<(Bytes, ())>>, p2049: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m2047 = (p2048, p2049);
    {
        let mut m2047 = m2047;
        loop {
            let step = {
                let (v1, v2) = m2047;
                let m2045 = v2;
                match m2045.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c2046: Fn7 = v0.clone();
                            c2046.apply(v1, v3)
                        }, v4))
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

pub fn f374(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f375(v0: Fn8, p2054: u64, p2055: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m2053 = (p2054, p2055);
    {
        let mut m2053 = m2053;
        loop {
            let step = {
                let (v1, v2) = m2053;
                let m2050 = v1;
                if m2050 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2050 - 1;
                    let m2052 = {
                        let c2051: Fn8 = v0.clone();
                        c2051.apply(v2.clone())?
                    };
                    match m2052 {
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
                Ok(next) => { m2053 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f440(v0)
}

pub fn f377(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f441(Fn13::F442, List::<(Bytes, ())>::nil(), v0)
}

pub fn f378(v0: Fn9, p2059: List<(Bytes, ())>, p2060: List<Bytes>) -> List<(Bytes, ())> {
    let m2058 = (p2059, p2060);
    {
        let mut m2058 = m2058;
        loop {
            let step = {
                let (v1, v2) = m2058;
                let m2056 = v2;
                match m2056.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c2057: Fn9 = v0.clone();
                            c2057.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f380(v0: Bytes) -> Bytes {
    if f138(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2062 = {
                let a2061 = v0;
                length_bytes(a2061)
            };
            let a2063 = 1u64;
            nat_sub(a2062, a2063)
        })
    } else {
        v0
    }
}

pub fn f381(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f85(f72(v0.clone(), v1.clone()))?;
    let m2064 = f157(v2.clone())?;
    match m2064 {
        None => {
            f444(v0, v1, v2)
        }
        Some(v3) => {
            f443(v0, v1, v3)
        }
    }
}

pub fn f382(v0: List<Bytes>) -> List<Bytes> {
    let m2065 = v0;
    match m2065.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a2067 = {
                    let a2066 = v1.clone();
                    length_bytes(a2066)
                };
                let a2068 = 0u64;
                if nat_eq(a2067, a2068) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f382(v2))
        }
    }
}

pub fn f383(v0: List<Bytes>) -> Bytes {
    let m2069 = v0;
    match m2069.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f384(v0: (Bytes, Bytes), p2072: List<(Bytes, Bytes)>) -> bool {
    let m2071 = p2072;
    {
        let mut m2071 = m2071;
        loop {
            let step = {
                let v1 = m2071;
                let m2070 = v1;
                match m2070.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f445(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2071 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m2073 = v1;
    match m2073.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37({
                let (h2074, _) = v2;
                h2074
            }, v0.clone()) {
                List::cons({
                    let (_, h2075) = v2;
                    h2075
                }, f385(v0, v3))
            } else {
                f385(v0, v3)
            }
        }
    }
}

pub fn f386(v0: u32) -> Bytes {
    let a2078 = {
        let a2076 = v0;
        let a2077 = 8u32;
        fixed_u32::bit_and(a2076, a2077)
    };
    let a2079 = 0u32;
    if equal(a2078, a2079) {
        let a2082 = {
            let a2080 = v0;
            let a2081 = 4u32;
            fixed_u32::bit_and(a2080, a2081)
        };
        let a2083 = 0u32;
        if equal(a2082, a2083) {
            let a2086 = {
                let a2084 = v0;
                let a2085 = 2u32;
                fixed_u32::bit_and(a2084, a2085)
            };
            let a2087 = 0u32;
            if equal(a2086, a2087) {
                let a2090 = {
                    let a2088 = v0;
                    let a2089 = 1u32;
                    fixed_u32::bit_and(a2088, a2089)
                };
                let a2091 = 0u32;
                if equal(a2090, a2091) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2094 = {
                    let a2092 = v0;
                    let a2093 = 1u32;
                    fixed_u32::bit_and(a2092, a2093)
                };
                let a2095 = 0u32;
                if equal(a2094, a2095) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2098 = {
                let a2096 = v0;
                let a2097 = 2u32;
                fixed_u32::bit_and(a2096, a2097)
            };
            let a2099 = 0u32;
            if equal(a2098, a2099) {
                let a2102 = {
                    let a2100 = v0;
                    let a2101 = 1u32;
                    fixed_u32::bit_and(a2100, a2101)
                };
                let a2103 = 0u32;
                if equal(a2102, a2103) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2106 = {
                    let a2104 = v0;
                    let a2105 = 1u32;
                    fixed_u32::bit_and(a2104, a2105)
                };
                let a2107 = 0u32;
                if equal(a2106, a2107) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2110 = {
            let a2108 = v0;
            let a2109 = 4u32;
            fixed_u32::bit_and(a2108, a2109)
        };
        let a2111 = 0u32;
        if equal(a2110, a2111) {
            let a2114 = {
                let a2112 = v0;
                let a2113 = 2u32;
                fixed_u32::bit_and(a2112, a2113)
            };
            let a2115 = 0u32;
            if equal(a2114, a2115) {
                let a2118 = {
                    let a2116 = v0;
                    let a2117 = 1u32;
                    fixed_u32::bit_and(a2116, a2117)
                };
                let a2119 = 0u32;
                if equal(a2118, a2119) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2122 = {
                    let a2120 = v0;
                    let a2121 = 1u32;
                    fixed_u32::bit_and(a2120, a2121)
                };
                let a2123 = 0u32;
                if equal(a2122, a2123) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2126 = {
                let a2124 = v0;
                let a2125 = 2u32;
                fixed_u32::bit_and(a2124, a2125)
            };
            let a2127 = 0u32;
            if equal(a2126, a2127) {
                let a2130 = {
                    let a2128 = v0;
                    let a2129 = 1u32;
                    fixed_u32::bit_and(a2128, a2129)
                };
                let a2131 = 0u32;
                if equal(a2130, a2131) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2134 = {
                    let a2132 = v0;
                    let a2133 = 1u32;
                    fixed_u32::bit_and(a2132, a2133)
                };
                let a2135 = 0u32;
                if equal(a2134, a2135) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f387(v0: u32, v1: u32) -> u32 {
    let m2138 = {
        let a2136 = v0;
        let a2137 = v1;
        fixed_u32::shift_right(a2136, a2137)
    };
    match m2138 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f388(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f446(Fn14::F447, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2139, _, _, _, _, _, _, _) => {
            h2139
        }
    }, match v0.clone() {
        Adt24::C0(_, h2140, _, _, _, _, _, _) => {
            h2140
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2141, _, _, _, _, _) => {
            h2141
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2142, _, _, _, _) => {
            h2142
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2143, _, _, _) => {
            h2143
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2144, _, _) => {
            h2144
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2145, _) => {
            h2145
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2146) => {
            h2146
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f448(match v0.clone() {
        Adt24::C0(h2147, _, _, _, _, _, _, _) => {
            h2147
        }
    }, match v2.clone() {
        Adt27::C0(h2148, _, _, _, _, _, _, _, _, _) => {
            h2148
        }
    }), f448(match v0.clone() {
        Adt24::C0(_, h2149, _, _, _, _, _, _) => {
            h2149
        }
    }, match v2.clone() {
        Adt27::C0(_, h2150, _, _, _, _, _, _, _, _) => {
            h2150
        }
    }), f448(match v0.clone() {
        Adt24::C0(_, _, h2151, _, _, _, _, _) => {
            h2151
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2152, _, _, _, _, _, _, _) => {
            h2152
        }
    }), f448(match v0.clone() {
        Adt24::C0(_, _, _, h2153, _, _, _, _) => {
            h2153
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2154, _, _, _, _, _, _) => {
            h2154
        }
    }), f448(match v0.clone() {
        Adt24::C0(_, _, _, _, h2155, _, _, _) => {
            h2155
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2156, _, _, _, _, _) => {
            h2156
        }
    }), f448(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2157, _, _) => {
            h2157
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2158, _, _, _, _) => {
            h2158
        }
    }), f448(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2159, _) => {
            h2159
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2160, _, _, _) => {
            h2160
        }
    }), f448(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2161) => {
            h2161
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2162, _, _) => {
            h2162
        }
    })))
}

pub fn f389(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f449(v0.clone(), v1, v2, v3)?, f449(v0.clone(), v1, v2, {
        let a2163 = v3;
        let a2164 = 4u64;
        nat_add(a2163, a2164)?
    })?, f449(v0.clone(), v1, v2, {
        let a2165 = v3;
        let a2166 = 8u64;
        nat_add(a2165, a2166)?
    })?, f449(v0.clone(), v1, v2, {
        let a2167 = v3;
        let a2168 = 12u64;
        nat_add(a2167, a2168)?
    })?, f449(v0.clone(), v1, v2, {
        let a2169 = v3;
        let a2170 = 16u64;
        nat_add(a2169, a2170)?
    })?, f449(v0.clone(), v1, v2, {
        let a2171 = v3;
        let a2172 = 20u64;
        nat_add(a2171, a2172)?
    })?, f449(v0.clone(), v1, v2, {
        let a2173 = v3;
        let a2174 = 24u64;
        nat_add(a2173, a2174)?
    })?, f449(v0.clone(), v1, v2, {
        let a2175 = v3;
        let a2176 = 28u64;
        nat_add(a2175, a2176)?
    })?, f449(v0.clone(), v1, v2, {
        let a2177 = v3;
        let a2178 = 32u64;
        nat_add(a2177, a2178)?
    })?, f449(v0.clone(), v1, v2, {
        let a2179 = v3;
        let a2180 = 36u64;
        nat_add(a2179, a2180)?
    })?, f449(v0.clone(), v1, v2, {
        let a2181 = v3;
        let a2182 = 40u64;
        nat_add(a2181, a2182)?
    })?, f449(v0.clone(), v1, v2, {
        let a2183 = v3;
        let a2184 = 44u64;
        nat_add(a2183, a2184)?
    })?, f449(v0.clone(), v1, v2, {
        let a2185 = v3;
        let a2186 = 48u64;
        nat_add(a2185, a2186)?
    })?, f449(v0.clone(), v1, v2, {
        let a2187 = v3;
        let a2188 = 52u64;
        nat_add(a2187, a2188)?
    })?, f449(v0.clone(), v1, v2, {
        let a2189 = v3;
        let a2190 = 56u64;
        nat_add(a2189, a2190)?
    })?, f449(v0, v1, v2, {
        let a2191 = v3;
        let a2192 = 60u64;
        nat_add(a2191, a2192)?
    })?))
}

pub fn f390(p2195: List<Bytes>, v1: Bytes) -> bool {
    let m2194 = p2195;
    {
        let mut m2194 = m2194;
        loop {
            let step = {
                let v0 = m2194;
                let m2193 = v0;
                match m2193.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f209(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Bytes, v1: u64) -> Option<u64> {
    let a2196 = 0u64;
    let a2197 = v1;
    if nat_lt(a2196, a2197) {
        if f450(v0, {
            let a2198 = v1;
            let a2199 = 1u64;
            nat_sub(a2198, a2199)
        }) {
            Some({
                let a2200 = v1;
                let a2201 = 1u64;
                nat_sub(a2200, a2201)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f392(v0: Adt14) -> R<List<Bytes>> {
    let m2202 = v0;
    match m2202 {
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
        Adt14::C6(r2203, r2204) => {
            let v7 = (*r2203).clone();
            let v8 = (*r2204).clone();
            Ok(List::cons(f318(v7)?, f392(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f393(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m2205 = v0;
    match m2205 {
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
        Adt14::C7(v9, r2206, r2207) => {
            let v10 = (*r2206).clone();
            let v11 = (*r2207).clone();
            Ok(List::cons((v9, f318(v10)?), f393(v11)?))
        }
    }
}

pub fn f394(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2208, _) = v1;
        h2208
    };
    let a2210 = {
        let a2209 = v0.clone();
        length_bytes(a2209)
    };
    let a2211 = v2;
    if nat_le(a2210, a2211) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2212 = f231(v0.clone(), v2)?;
        let a2213 = 13u64;
        if nat_eq(a2212, a2213) {
            Ok(Some(({
                let a2216 = f231(v0, {
                    let a2214 = v2;
                    let a2215 = 1u64;
                    nat_add(a2214, a2215)?
                })?;
                let a2217 = 10u64;
                if nat_eq(a2216, a2217) {
                    let a2218 = v2;
                    let a2219 = 2u64;
                    nat_add(a2218, a2219)?
                } else {
                    let a2220 = v2;
                    let a2221 = 1u64;
                    nat_add(a2220, a2221)?
                }
            }, {
                let a2223 = {
                    let (_, h2222) = v1;
                    h2222
                };
                let a2224 = Bytes::lit(&[0x0a]);
                append_bytes(a2223, a2224)
            })))
        } else {
            let v4: u64 = {
                let m2225 = f323(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m2225 {
                    None => {
                        let a2226 = v0.clone();
                        length_bytes(a2226)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2230 = {
                    let (_, h2227) = v1;
                    h2227
                };
                let a2231 = f32(v0, v2, {
                    let a2228 = v4;
                    let a2229 = v2;
                    nat_sub(a2228, a2229)
                });
                append_bytes(a2230, a2231)
            })))
        }
    }
}

pub fn f395(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2232 = f451(v0, v1)?;
    match m2232 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f452({
                let (h2233, _) = v2;
                h2233
            }) {
                Ok(Some({
                    let a2235 = v1;
                    let a2236 = {
                        let (_, h2234) = v2;
                        h2234
                    };
                    nat_add(a2235, a2236)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f396(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f453(v1, v0, v2)
}

pub fn f397(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f454(v0, v1)
}

pub fn f398(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f455(v0, v1, v2)
}

pub fn f399(v0: Bytes, p2239: List<Bytes>) -> bool {
    let m2238 = p2239;
    {
        let mut m2238 = m2238;
        loop {
            let step = {
                let v1 = m2238;
                let m2237 = v1;
                match m2237.uncons() {
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
                Ok(next) => { m2238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f230(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2242 = f231(v0.clone(), {
                let a2240 = v1;
                let a2241 = 2u64;
                nat_add(a2240, a2241)?
            })?;
            let a2243 = 120u64;
            nat_eq(a2242, a2243)
        };
        let v3: u64 = if v2 {
            let a2244 = v1;
            let a2245 = 3u64;
            nat_add(a2244, a2245)?
        } else {
            let a2246 = v1;
            let a2247 = 2u64;
            nat_add(a2246, a2247)?
        };
        let v4: (u64, u64) = f456(v0.clone(), v2, v3)?;
        let a2249 = {
            let (h2248, _) = v4;
            h2248
        };
        let a2250 = v3;
        if if nat_eq(a2249, a2250) {
            true
        } else {
            let a2252 = f231(v0, {
                let (h2251, _) = v4;
                h2251
            })?;
            let a2253 = 59u64;
            !nat_eq(a2252, a2253)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f227(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f452({
                let (_, h2254) = v4;
                h2254
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f457({
                    let (_, h2255) = v4;
                    h2255
                })?, {
                    let a2257 = {
                        let (h2256, _) = v4;
                        h2256
                    };
                    let a2258 = 1u64;
                    nat_add(a2257, a2258)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f227(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f230(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a2259 = v1;
                let a2260 = 4u64;
                nat_add(a2259, a2260)?
            })))
        } else {
            if f230(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a2261 = v1;
                    let a2262 = 4u64;
                    nat_add(a2261, a2262)?
                })))
            } else {
                if f230(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a2263 = v1;
                        let a2264 = 5u64;
                        nat_add(a2263, a2264)?
                    })))
                } else {
                    if f230(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a2265 = v1;
                            let a2266 = 6u64;
                            nat_add(a2265, a2266)?
                        })))
                    } else {
                        if f230(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a2267 = v1;
                                let a2268 = 6u64;
                                nat_add(a2267, a2268)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f227(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f401(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2269, _) = f269(Fn3::F458(v1, v2, v0.clone()), {
        let a2271 = {
            let a2270 = v0;
            length_bytes(a2270)
        };
        let a2272 = 1u64;
        nat_add(a2271, a2272)?
    }, v3)?;
    Ok(h2269)
}

pub fn f402(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2273, _) = v2;
        h2273
    };
    let a2274 = v3;
    let a2275 = 0u64;
    if nat_eq(a2274, a2275) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2276 = v3;
            let a2277 = v1;
            let a2278 = 0u64;
            nat_quot(a2276, a2277, a2278)
        }, {
            let a2283 = f332(v0, {
                let a2279 = v3;
                let a2280 = v1;
                let a2281 = 0u64;
                nat_rem(a2279, a2280, a2281)
            });
            let a2284 = {
                let (_, h2282) = v2;
                h2282
            };
            append_bytes(a2283, a2284)
        }))
    }
}

pub fn f403(p2295: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>> {
    let m2294 = p2295;
    {
        let mut m2294 = m2294;
        loop {
            let step = {
                let v0 = m2294;
                let m2285 = v0;
                match m2285 {
                    Adt18::C0(_, _, _) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C1(_, _, _) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(None::<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>)))
                    }
                    Adt18::C3(r2292, r2293) => {
                        let v2 = (*r2292).clone();
                        let v3 = (*r2293).clone();
                        let m2286 = v2;
                        match m2286 {
                            Adt18::C0(v4, r2290, r2291) => {
                                let v5 = (*r2290).clone();
                                let v6 = (*r2291).clone();
                                let m2287 = f235(v5.clone(), v1.clone());
                                match m2287 {
                                    Ok(v8) => {
                                        let m2288 = f236(v8.clone(), v4)?;
                                        match m2288 {
                                            Ok(v10) => {
                                                if if f237(v10.clone(), Str::lit("restriction")) {
                                                    true
                                                } else {
                                                    if f237(v10.clone(), Str::lit("list")) {
                                                        true
                                                    } else {
                                                        f237(v10.clone(), Str::lit("union"))
                                                    }
                                                } {
                                                    Err::<Adt18, R<Result<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>>>(Ok(Ok::<Option<(Bytes, (Adt18, (Adt18, List<(Bytes, Bytes)>)))>, Bytes>(Some(({
                                                        let (_, h2289) = v10;
                                                        h2289
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
                Ok(next) => { m2294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f405(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let m2296 = f236(v0, v2.clone())?;
    match m2296 {
        Ok(v4) => {
            if f37({
                let (h2297, _) = v4.clone();
                h2297
            }, f339()) {
                let m2299 = f459({
                    let (_, h2298) = v4;
                    h2298
                }, 16u64);
                match m2299 {
                    Ok(v6) => {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v5))
                    }
                }
            } else {
                if f37({
                    let (h2300, _) = v4.clone();
                    h2300
                }, v1) {
                    let v7: Bytes = f256({
                        let (_, h2301) = v4;
                        h2301
                    })?;
                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2302, _) = v4;
                        h2302
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<(Bytes, List<Bytes>), Bytes>(v3))
        }
    }
}

pub fn f406(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f407(v0: Adt18, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt19>, Bytes>> {
    let m2303 = v0;
    match m2303 {
        Adt18::C0(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C1(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C2(_) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt18::C3(r2310, r2311) => {
            let v2 = (*r2310).clone();
            let v3 = (*r2311).clone();
            let m2304 = f407(v3, v1.clone())?;
            match m2304 {
                Ok(v5) => {
                    let m2305 = v2;
                    match m2305 {
                        Adt18::C0(v6, r2309, _) => {
                            let v7 = (*r2309).clone();
                            let m2306 = f235(v7.clone(), v1);
                            match m2306 {
                                Ok(v10) => {
                                    let m2307 = f236(v10, v6)?;
                                    match m2307 {
                                        Ok(v12) => {
                                            if f237(v12, Str::lit("enumeration")) {
                                                let m2308 = f238(v7, f8(Str::lit("value")));
                                                match m2308 {
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

pub fn f408(v0: List<Adt19>, v1: List<Bytes>) -> R<List<Adt19>> {
    let m2312 = v0;
    match m2312.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f460(v2.clone())?;
            if f88(v4.clone(), v1.clone()) {
                f408(v3, v1)
            } else {
                Ok(List::cons(v2, f408(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f409(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2313 = v1;
    match m2313.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2314 = f461(v0, v3.clone())?;
            match m2314 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f460(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f88(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2315 = f409(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2315 {
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

pub fn f410(p2319: List<Bytes>) -> R<bool> {
    let m2318 = p2319;
    {
        let mut m2318 = m2318;
        loop {
            let step = {
                let v0 = m2318;
                let m2316 = v0;
                match m2316.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2317 = f462(v1)?;
                        match m2317 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f88(v3, f463(false, v2.clone())?) {
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
                Ok(next) => { m2318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f219(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f412(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2320 = v2;
    let a2321 = v0;
    if nat_lt(a2320, a2321) {
        if f450(v1, v2) {
            Ok(Some({
                let a2322 = v2;
                let a2323 = 1u64;
                nat_add(a2322, a2323)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f413(v0: Bytes, p2403: u64, p2404: u64, p2405: bool, p2406: bool, p2407: Bytes) -> R<Bytes> {
    let m2402 = (p2403, (p2404, (p2405, (p2406, p2407))));
    {
        let mut m2402 = m2402;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m2402;
                let m2324 = v2;
                if m2324 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m2324 - 1;
                    let m2327 = {
                        let a2325 = v0.clone();
                        let a2326 = v1;
                        index_bytes(a2325, a2326)
                    };
                    match m2327 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a2372 = {
                                    let a2348 = {
                                        let a2336 = {
                                            let a2330 = {
                                                let a2328 = v8;
                                                let a2329 = 128u8;
                                                fixed_u8::bit_and(a2328, a2329)
                                            };
                                            let a2331 = 128u8;
                                            if equal(a2330, a2331) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2337 = {
                                            let a2334 = {
                                                let a2332 = v8;
                                                let a2333 = 64u8;
                                                fixed_u8::bit_and(a2332, a2333)
                                            };
                                            let a2335 = 64u8;
                                            if equal(a2334, a2335) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2336, a2337)?
                                    };
                                    let a2349 = {
                                        let a2346 = {
                                            let a2340 = {
                                                let a2338 = v8;
                                                let a2339 = 32u8;
                                                fixed_u8::bit_and(a2338, a2339)
                                            };
                                            let a2341 = 32u8;
                                            if equal(a2340, a2341) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2347 = {
                                            let a2344 = {
                                                let a2342 = v8;
                                                let a2343 = 16u8;
                                                fixed_u8::bit_and(a2342, a2343)
                                            };
                                            let a2345 = 16u8;
                                            if equal(a2344, a2345) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2346, a2347)?
                                    };
                                    nat_add(a2348, a2349)?
                                };
                                let a2373 = {
                                    let a2370 = {
                                        let a2358 = {
                                            let a2352 = {
                                                let a2350 = v8;
                                                let a2351 = 8u8;
                                                fixed_u8::bit_and(a2350, a2351)
                                            };
                                            let a2353 = 8u8;
                                            if equal(a2352, a2353) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2359 = {
                                            let a2356 = {
                                                let a2354 = v8;
                                                let a2355 = 4u8;
                                                fixed_u8::bit_and(a2354, a2355)
                                            };
                                            let a2357 = 4u8;
                                            if equal(a2356, a2357) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2358, a2359)?
                                    };
                                    let a2371 = {
                                        let a2368 = {
                                            let a2362 = {
                                                let a2360 = v8;
                                                let a2361 = 2u8;
                                                fixed_u8::bit_and(a2360, a2361)
                                            };
                                            let a2363 = 2u8;
                                            if equal(a2362, a2363) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2369 = {
                                            let a2366 = {
                                                let a2364 = v8;
                                                let a2365 = 1u8;
                                                fixed_u8::bit_and(a2364, a2365)
                                            };
                                            let a2367 = 1u8;
                                            if equal(a2366, a2367) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2368, a2369)?
                                    };
                                    nat_add(a2370, a2371)?
                                };
                                nat_add(a2372, a2373)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a2374 = v1;
                                    let a2375 = 1u64;
                                    nat_add(a2374, a2375)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a2376 = v9;
                                    let a2377 = 34u64;
                                    !nat_eq(a2376, a2377)
                                }, (if v4 {
                                    false
                                } else {
                                    let a2378 = v9;
                                    let a2379 = 92u64;
                                    nat_eq(a2378, a2379)
                                }, {
                                    let a2380 = v5;
                                    let a2381 = v10;
                                    append_bytes(a2380, a2381)
                                })))))
                            } else {
                                let a2382 = v9;
                                let a2383 = 32u64;
                                if if if if nat_eq(a2382, a2383) {
                                    true
                                } else {
                                    let a2384 = v9;
                                    let a2385 = 9u64;
                                    nat_eq(a2384, a2385)
                                } {
                                    true
                                } else {
                                    let a2386 = v9;
                                    let a2387 = 10u64;
                                    nat_eq(a2386, a2387)
                                } {
                                    true
                                } else {
                                    let a2388 = v9;
                                    let a2389 = 13u64;
                                    nat_eq(a2388, a2389)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a2390 = v1;
                                        let a2391 = 1u64;
                                        nat_add(a2390, a2391)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f37(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2392 = v1;
                                            let a2393 = 4u64;
                                            nat_add(a2392, a2393)?
                                        }, (v6, (false, (false, {
                                            let a2394 = v5;
                                            let a2395 = f8(Str::lit("[]"));
                                            append_bytes(a2394, a2395)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2396 = v1;
                                            let a2397 = 1u64;
                                            nat_add(a2396, a2397)?
                                        }, (v6, ({
                                            let a2398 = v9;
                                            let a2399 = 34u64;
                                            nat_eq(a2398, a2399)
                                        }, (false, {
                                            let a2400 = v5;
                                            let a2401 = v10;
                                            append_bytes(a2400, a2401)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f464(Fn15::F465(v0.clone()), {
        let a2409 = {
            let a2408 = v0;
            length_bytes(a2408)
        };
        let a2410 = 1u64;
        nat_add(a2409, a2410)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a2414 = {
        let (_, h2411) = {
            let (_, h2412) = {
                let (h2413, _) = v2.clone();
                h2413
            };
            h2412
        };
        h2411
    };
    let a2415 = 1u64;
    if nat_eq(a2414, a2415) {
        Ok(Some(({
            let (h2416, _) = {
                let (h2417, _) = v2.clone();
                h2417
            };
            h2416
        }, {
            let (h2418, _) = {
                let (_, h2419) = {
                    let (h2420, _) = v2;
                    h2420
                };
                h2419
            };
            h2418
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f415(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2421 = f328(v0.clone(), v1)?;
        let a2422 = 45u64;
        nat_eq(a2421, a2422)
    };
    let v3: u64 = if v2 {
        let a2423 = v1;
        let a2424 = 1u64;
        nat_add(a2423, a2424)?
    } else {
        v1
    };
    let v4: u64 = f466(v0.clone(), v3)?;
    let v5: u64 = {
        let a2425 = v4;
        let a2426 = v3;
        nat_sub(a2425, a2426)
    };
    let v6: bool = {
        let a2427 = f328(v0.clone(), v4)?;
        let a2428 = 46u64;
        nat_eq(a2427, a2428)
    };
    let v7: u64 = if v6 {
        let a2429 = v4;
        let a2430 = 1u64;
        nat_add(a2429, a2430)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f466(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f328(v0.clone(), v8)?;
    let v10: bool = {
        let a2431 = v9;
        let a2432 = 101u64;
        if nat_eq(a2431, a2432) {
            true
        } else {
            let a2433 = v9;
            let a2434 = 69u64;
            nat_eq(a2433, a2434)
        }
    };
    let v11: u64 = f328(v0.clone(), {
        let a2435 = v8;
        let a2436 = 1u64;
        nat_add(a2435, a2436)?
    })?;
    let v12: bool = if v10 {
        let a2437 = v11;
        let a2438 = 43u64;
        if nat_eq(a2437, a2438) {
            true
        } else {
            let a2439 = v11;
            let a2440 = 45u64;
            nat_eq(a2439, a2440)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2441 = v8;
            let a2442 = 2u64;
            nat_add(a2441, a2442)?
        } else {
            let a2443 = v8;
            let a2444 = 1u64;
            nat_add(a2443, a2444)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f466(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2445 = v14;
        let a2446 = v13;
        nat_sub(a2445, a2446)
    });
    let v16: u64 = f467(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2448 = {
            let a2447 = v15;
            length_bytes(a2447)
        };
        let a2449 = v16;
        nat_sub(a2448, a2449)
    });
    let a2450 = v5;
    let a2451 = 0u64;
    if if nat_eq(a2450, a2451) {
        true
    } else {
        let a2452 = 1u64;
        let a2453 = v5;
        if if nat_lt(a2452, a2453) {
            let a2454 = f328(v0.clone(), v3)?;
            let a2455 = 48u64;
            nat_eq(a2454, a2455)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2456 = v8;
                let a2457 = v7;
                nat_eq(a2456, a2457)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2458 = v14;
                    let a2459 = v13;
                    nat_eq(a2458, a2459)
                } else {
                    false
                } {
                    true
                } else {
                    let a2461 = 18u64;
                    let a2462 = {
                        let a2460 = v17.clone();
                        length_bytes(a2460)
                    };
                    nat_lt(a2461, a2462)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2466 = {
            let a2464 = {
                let a2463 = v17.clone();
                length_bytes(a2463)
            };
            let a2465 = 0u64;
            if nat_eq(a2464, a2465) {
                Some(0u64)
            } else {
                f468(v17)?
            }
        };
        match m2466 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2471 = f469(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2467 = v8;
                    let a2468 = v7;
                    nat_sub(a2467, a2468)
                }), if v12 {
                    let a2469 = v11;
                    let a2470 = 45u64;
                    nat_eq(a2469, a2470)
                } else {
                    false
                }, v18)?;
                match m2471 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2473 = 19u64;
                        let a2474 = {
                            let a2472 = v19.clone();
                            length_bytes(a2472)
                        };
                        if nat_lt(a2473, a2474) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2475 = f468(v19)?;
                            match m2475 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2476 = 0u64;
                                        let a2477 = v20;
                                        nat_lt(a2476, a2477)
                                    } else {
                                        false
                                    };
                                    if f470(v21, v20) {
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

pub fn f416(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2478 = f328(v0.clone(), v1)?;
        let a2479 = 45u64;
        nat_eq(a2478, a2479)
    };
    let v3: u64 = if v2 {
        let a2480 = v1;
        let a2481 = 1u64;
        nat_add(a2480, a2481)?
    } else {
        v1
    };
    let v4: u64 = f466(v0.clone(), v3)?;
    let v5: u64 = {
        let a2482 = v4;
        let a2483 = v3;
        nat_sub(a2482, a2483)
    };
    let v6: bool = {
        let a2484 = f328(v0.clone(), v4)?;
        let a2485 = 46u64;
        nat_eq(a2484, a2485)
    };
    let v7: u64 = if v6 {
        let a2486 = v4;
        let a2487 = 1u64;
        nat_add(a2486, a2487)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f466(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f328(v0.clone(), v8)?;
    let v10: bool = {
        let a2488 = v9;
        let a2489 = 101u64;
        if nat_eq(a2488, a2489) {
            true
        } else {
            let a2490 = v9;
            let a2491 = 69u64;
            nat_eq(a2490, a2491)
        }
    };
    let v11: u64 = f328(v0.clone(), {
        let a2492 = v8;
        let a2493 = 1u64;
        nat_add(a2492, a2493)?
    })?;
    let v12: bool = if v10 {
        let a2494 = v11;
        let a2495 = 43u64;
        if nat_eq(a2494, a2495) {
            true
        } else {
            let a2496 = v11;
            let a2497 = 45u64;
            nat_eq(a2496, a2497)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2498 = v8;
            let a2499 = 2u64;
            nat_add(a2498, a2499)?
        } else {
            let a2500 = v8;
            let a2501 = 1u64;
            nat_add(a2500, a2501)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f466(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2502 = v14;
        let a2503 = v13;
        nat_sub(a2502, a2503)
    });
    let v16: u64 = f467(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2505 = {
            let a2504 = v15;
            length_bytes(a2504)
        };
        let a2506 = v16;
        nat_sub(a2505, a2506)
    });
    let a2507 = v5;
    let a2508 = 0u64;
    if if nat_eq(a2507, a2508) {
        true
    } else {
        let a2509 = 1u64;
        let a2510 = v5;
        if if nat_lt(a2509, a2510) {
            let a2511 = f328(v0.clone(), v3)?;
            let a2512 = 48u64;
            nat_eq(a2511, a2512)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2513 = v8;
                let a2514 = v7;
                nat_eq(a2513, a2514)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2515 = v14;
                    let a2516 = v13;
                    nat_eq(a2515, a2516)
                } else {
                    false
                } {
                    true
                } else {
                    let a2518 = 18u64;
                    let a2519 = {
                        let a2517 = v17.clone();
                        length_bytes(a2517)
                    };
                    nat_lt(a2518, a2519)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2523 = {
            let a2521 = {
                let a2520 = v17.clone();
                length_bytes(a2520)
            };
            let a2522 = 0u64;
            if nat_eq(a2521, a2522) {
                Some(0u64)
            } else {
                f468(v17)?
            }
        };
        match m2523 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2528 = f471(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2524 = v8;
                    let a2525 = v7;
                    nat_sub(a2524, a2525)
                }), if v12 {
                    let a2526 = v11;
                    let a2527 = 45u64;
                    nat_eq(a2526, a2527)
                } else {
                    false
                }, v18)?;
                match m2528 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2531 = 19u64;
                        let a2532 = {
                            let a2530 = {
                                let (h2529, _) = v19.clone();
                                h2529
                            };
                            length_bytes(a2530)
                        };
                        if nat_lt(a2531, a2532) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2534 = f468({
                                let (h2533, _) = v19.clone();
                                h2533
                            })?;
                            match m2534 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2535 = 0u64;
                                        let a2536 = v20;
                                        nat_lt(a2535, a2536)
                                    } else {
                                        false
                                    };
                                    if f470(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2538 = {
                                                let (_, h2537) = v19.clone();
                                                h2537
                                            };
                                            let a2539 = 0u64;
                                            if nat_eq(a2538, a2539) {
                                                Adt19::C1(v21, v20)
                                            } else {
                                                Adt19::C2(v21, v20, {
                                                    let (_, h2540) = v19;
                                                    h2540
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

pub fn f417(v0: u64) -> bool {
    let a2541 = v0;
    let a2542 = 95u64;
    if nat_eq(a2541, a2542) {
        true
    } else {
        let a2543 = v0;
        let a2544 = 45u64;
        if nat_eq(a2543, a2544) {
            true
        } else {
            let a2545 = v0;
            let a2546 = 32u64;
            if nat_eq(a2545, a2546) {
                true
            } else {
                let a2547 = v0;
                let a2548 = 46u64;
                if nat_eq(a2547, a2548) {
                    true
                } else {
                    let a2549 = v0;
                    let a2550 = 47u64;
                    nat_eq(a2549, a2550)
                }
            }
        }
    }
}

pub fn f418(v0: u64) -> bool {
    let a2551 = 97u64;
    let a2552 = v0;
    if nat_le(a2551, a2552) {
        let a2553 = v0;
        let a2554 = 122u64;
        nat_le(a2553, a2554)
    } else {
        false
    }
}

pub fn f419(v0: u64) -> bool {
    let a2555 = 65u64;
    let a2556 = v0;
    if nat_le(a2555, a2556) {
        let a2557 = v0;
        let a2558 = 90u64;
        nat_le(a2557, a2558)
    } else {
        false
    }
}

pub fn f420(v0: u64) -> bool {
    let a2559 = 48u64;
    let a2560 = v0;
    if nat_le(a2559, a2560) {
        let a2561 = v0;
        let a2562 = 57u64;
        nat_le(a2561, a2562)
    } else {
        false
    }
}

pub fn f421(v0: List<(Bytes, Adt19)>, v1: List<(Bytes, Adt19)>) -> List<(Bytes, Adt19)> {
    let m2563 = v0;
    match m2563.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f421(v3, v1))
        }
    }
}

pub fn f422(v0: Option<Adt19>) -> List<(Bytes, Adt19)> {
    let m2564 = v0;
    match m2564 {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some(v1) => {
            f472(v1)
        }
    }
}

pub fn f423(v0: u64, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt19)>, Bytes>> {
    let m2565 = v0;
    if m2565 == 0 {
        let m2566 = v1;
        match m2566.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2565 - 1;
        let m2567 = v1.clone();
        match m2567.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                let m2568 = f473(v1.clone(), v2.clone(), v3.clone())?;
                match m2568 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f219(f424(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2571 = f423(v6, f474(v1, {
                            let (h2569, _) = v9.clone();
                            h2569
                        }), List::cons(f256({
                            let (h2570, _) = v9.clone();
                            h2570
                        })?, v2), v3)?;
                        match m2571 {
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

pub fn f424(v0: List<(Bytes, Adt19)>) -> R<List<Bytes>> {
    let m2572 = v0;
    match m2572.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f256({
                let (h2573, _) = v1;
                h2573
            })?, f424(v2)?))
        }
    }
}

pub fn f425(p2581: u64, p2582: List<Adt25>, p2583: List<Bytes>, p2584: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2580 = (p2581, (p2582, (p2583, p2584)));
    {
        let mut m2580 = m2580;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2580;
                let m2574 = v0;
                if m2574 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2574 - 1;
                    let m2575 = v1;
                    match m2575.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2576 = v5;
                            match m2576 {
                                Adt25::C0(v8, v9) => {
                                    if f37(v8.clone(), f475()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f88(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2577 = f477(v8.clone(), v9)?;
                                            match m2577 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f426({
                                                        let (_, h2578) = v11;
                                                        h2578
                                                    }, List::cons(Adt25::C1({
                                                        let (h2579, _) = v11;
                                                        h2579
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
                                    if f88(f475(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f475(), v2), List::cons(f476(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2585 = v0;
    match m2585.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f426(v3, v1))
        }
    }
}

pub fn f427(v0: List<(Bytes, Adt19)>) -> R<List<Adt25>> {
    let m2586 = v0;
    match m2586.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f256({
                let (h2587, _) = v1;
                h2587
            })?, {
                let (_, h2588) = v1;
                h2588
            }), f427(v2)?))
        }
    }
}

pub fn f428(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2589 = v0;
    match m2589.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f37(f161(v3.clone(), v1.clone()), v2.clone()) {
                f428(v4, v1, v2)
            } else {
                List::cons(v3, f428(v4, v1, v2))
            }
        }
    }
}

pub fn f429(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2590 = v0;
    match m2590.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f209(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f219(f437(f80(List::cons(v1, f478(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f429(v3, v1)?))
            }
        }
    }
}

pub fn f430(v0: u64) -> Bytes {
    let m2591 = v0;
    if m2591 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2591 - 1;
        let a2592 = f8(Str::lit("0"));
        let a2593 = f430(v1);
        append_bytes(a2592, a2593)
    }
}

pub fn f431(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f479(v0, v1)
}

pub fn f432(v0: Fn10, p2597: List<(Bytes, Bytes)>, p2598: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2596 = (p2597, p2598);
    {
        let mut m2596 = m2596;
        loop {
            let step = {
                let (v1, v2) = m2596;
                let m2594 = v2;
                match m2594.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2595: Fn10 = v0.clone();
                            c2595.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f480(v0, v1)
}

pub fn f434(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f481(v0)
}

pub fn f435(v0: Fn11, p2602: List<Bytes>, p2603: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2601 = (p2602, p2603);
    {
        let mut m2601 = m2601;
        loop {
            let step = {
                let (v1, v2) = m2601;
                let m2599 = v2;
                match m2599.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2600: Fn11 = v0.clone();
                            c2600.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f482(v1)?, v0))
}

pub fn f437(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2604 = v0;
    match m2604.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f214(v1)?, f437(v2)?))
        }
    }
}

pub fn f438(v0: Fn12, p2608: List<List<(Bytes, ())>>, p2609: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2607 = (p2608, p2609);
    {
        let mut m2607 = m2607;
        loop {
            let step = {
                let (v1, v2) = m2607;
                let m2605 = v2;
                match m2605.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2606: Fn12 = v0.clone();
                            c2606.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f440(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2611 = {
        let a2610 = v0.clone();
        length_list(a2610)
    };
    let a2612 = 1u64;
    if nat_le(a2611, a2612) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f483(v0)?))
    }
}

pub fn f441(v0: Fn13, p2616: List<(Bytes, ())>, p2617: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2615 = (p2616, p2617);
    {
        let mut m2615 = m2615;
        loop {
            let step = {
                let (v1, v2) = m2615;
                let m2613 = v2;
                match m2613.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2614: Fn13 = v0.clone();
                            c2614.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f443(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f74(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f484(v3.clone(), f74(f75(v0, f91())?, f8(Str::lit("at"))));
    let a2619 = {
        let a2618 = v4.clone();
        length_list(a2618)
    };
    let a2620 = 0u64;
    if nat_eq(a2619, a2620) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2623 = {
            let a2621 = v4;
            length_list(a2621)
        };
        let a2624 = {
            let a2622 = v3;
            length_list(a2622)
        };
        if nat_eq(a2623, a2624) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f485(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f444(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f251(f144(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f251(f144(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f66(f74(v3.clone(), f8(Str::lit("id"))), f74(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f74(f75(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f484(v5.clone(), f66(v6.clone(), f74(f75(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2626 = {
        let a2625 = v7.clone();
        length_list(a2625)
    };
    let a2627 = 0u64;
    if nat_eq(a2626, a2627) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f77({
            let a2630 = {
                let a2628 = v7;
                length_list(a2628)
            };
            let a2631 = {
                let a2629 = v5;
                length_list(a2629)
            };
            nat_eq(a2630, a2631)
        }) {
            Ok((false, f485(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f80(f66(f484(f486(v4.clone()), f71(f72(v0, f8(Str::lit("model/authorities.toml"))))?), f484(f487(v4), f66(v6, f74(v3, f8(Str::lit("id")))))))?;
            let a2633 = {
                let a2632 = v8.clone();
                length_list(a2632)
            };
            let a2634 = 0u64;
            if nat_eq(a2633, a2634) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f485(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f445(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f37({
        let (h2635, _) = v0;
        h2635
    }, {
        let (h2636, _) = v1;
        h2636
    }) {
        f37({
            let (_, h2637) = v0;
            h2637
        }, {
            let (_, h2638) = v1;
            h2638
        })
    } else {
        false
    }
}

pub fn f446(v0: Fn14, p2642: u64, p2643: Adt27) -> R<Adt27> {
    let m2641 = (p2642, p2643);
    {
        let mut m2641 = m2641;
        loop {
            let step = {
                let (v1, v2) = m2641;
                let m2639 = v1;
                if m2639 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2639 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2640: Fn14 = v0.clone();
                        c2640.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Adt27) -> R<Adt27> {
    f488(v0)
}

pub fn f448(v0: u32, v1: u32) -> u32 {
    let m2646 = {
        let a2644 = f489(v0);
        let a2645 = f489(v1);
        fixed_u64::checked_add(a2644, a2645)
    };
    match m2646 {
        None => {
            0u32
        }
        Some(v2) => {
            f490(v2)
        }
    }
}

pub fn f449(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2649 = {
        let a2647 = v3;
        let a2648 = 8u64;
        nat_add(a2647, a2648)?
    };
    let a2650 = v2;
    if nat_eq(a2649, a2650) {
        Ok(f491({
            let a2653 = {
                let a2651 = v1;
                let a2652 = 8u64;
                nat_mul(a2651, a2652)?
            };
            let a2654 = 4294967296u64;
            let a2655 = 0u64;
            nat_quot(a2653, a2654, a2655)
        }))
    } else {
        let a2658 = {
            let a2656 = v3;
            let a2657 = 4u64;
            nat_add(a2656, a2657)?
        };
        let a2659 = v2;
        if nat_eq(a2658, a2659) {
            Ok(f491({
                let a2662 = {
                    let a2660 = v1;
                    let a2661 = 8u64;
                    nat_mul(a2660, a2661)?
                };
                let a2663 = 4294967296u64;
                let a2664 = 0u64;
                nat_rem(a2662, a2663, a2664)
            }))
        } else {
            Ok(f492(f493(v0.clone(), v1, v3), f493(v0.clone(), v1, {
                let a2665 = v3;
                let a2666 = 1u64;
                nat_add(a2665, a2666)?
            }), f493(v0.clone(), v1, {
                let a2667 = v3;
                let a2668 = 2u64;
                nat_add(a2667, a2668)?
            }), f493(v0, v1, {
                let a2669 = v3;
                let a2670 = 3u64;
                nat_add(a2669, a2670)?
            })))
        }
    }
}

pub fn f450(v0: Bytes, v1: u64) -> bool {
    let m2671 = f266(v0, v1);
    match m2671 {
        None => {
            false
        }
        Some(v2) => {
            f494(v2)
        }
    }
}

pub fn f451(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2674 = {
        let a2672 = v0.clone();
        let a2673 = v1;
        index_bytes(a2672, a2673)
    };
    match m2674 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2719 = {
                    let a2695 = {
                        let a2683 = {
                            let a2677 = {
                                let a2675 = v3;
                                let a2676 = 128u8;
                                fixed_u8::bit_and(a2675, a2676)
                            };
                            let a2678 = 128u8;
                            if equal(a2677, a2678) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2684 = {
                            let a2681 = {
                                let a2679 = v3;
                                let a2680 = 64u8;
                                fixed_u8::bit_and(a2679, a2680)
                            };
                            let a2682 = 64u8;
                            if equal(a2681, a2682) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2683, a2684)?
                    };
                    let a2696 = {
                        let a2693 = {
                            let a2687 = {
                                let a2685 = v3;
                                let a2686 = 32u8;
                                fixed_u8::bit_and(a2685, a2686)
                            };
                            let a2688 = 32u8;
                            if equal(a2687, a2688) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2694 = {
                            let a2691 = {
                                let a2689 = v3;
                                let a2690 = 16u8;
                                fixed_u8::bit_and(a2689, a2690)
                            };
                            let a2692 = 16u8;
                            if equal(a2691, a2692) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2693, a2694)?
                    };
                    nat_add(a2695, a2696)?
                };
                let a2720 = {
                    let a2717 = {
                        let a2705 = {
                            let a2699 = {
                                let a2697 = v3;
                                let a2698 = 8u8;
                                fixed_u8::bit_and(a2697, a2698)
                            };
                            let a2700 = 8u8;
                            if equal(a2699, a2700) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2706 = {
                            let a2703 = {
                                let a2701 = v3;
                                let a2702 = 4u8;
                                fixed_u8::bit_and(a2701, a2702)
                            };
                            let a2704 = 4u8;
                            if equal(a2703, a2704) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2705, a2706)?
                    };
                    let a2718 = {
                        let a2715 = {
                            let a2709 = {
                                let a2707 = v3;
                                let a2708 = 2u8;
                                fixed_u8::bit_and(a2707, a2708)
                            };
                            let a2710 = 2u8;
                            if equal(a2709, a2710) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2716 = {
                            let a2713 = {
                                let a2711 = v3;
                                let a2712 = 1u8;
                                fixed_u8::bit_and(a2711, a2712)
                            };
                            let a2714 = 1u8;
                            if equal(a2713, a2714) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2715, a2716)?
                    };
                    nat_add(a2717, a2718)?
                };
                nat_add(a2719, a2720)?
            };
            let a2721 = v4;
            let a2722 = 128u64;
            if nat_lt(a2721, a2722) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2723 = 194u64;
                let a2724 = v4;
                if if nat_le(a2723, a2724) {
                    let a2725 = v4;
                    let a2726 = 223u64;
                    nat_le(a2725, a2726)
                } else {
                    false
                } {
                    let m2729 = f495(v0, {
                        let a2727 = v1;
                        let a2728 = 1u64;
                        nat_add(a2727, a2728)?
                    })?;
                    match m2729 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f496({
                                let a2734 = {
                                    let a2732 = {
                                        let a2730 = v4;
                                        let a2731 = 192u64;
                                        nat_sub(a2730, a2731)
                                    };
                                    let a2733 = 64u64;
                                    nat_mul(a2732, a2733)?
                                };
                                let a2735 = v5;
                                nat_add(a2734, a2735)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2736 = 224u64;
                    let a2737 = v4;
                    if if nat_le(a2736, a2737) {
                        let a2738 = v4;
                        let a2739 = 239u64;
                        nat_le(a2738, a2739)
                    } else {
                        false
                    } {
                        let m2742 = f495(v0.clone(), {
                            let a2740 = v1;
                            let a2741 = 1u64;
                            nat_add(a2740, a2741)?
                        })?;
                        match m2742 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2745 = f495(v0, {
                                    let a2743 = v1;
                                    let a2744 = 2u64;
                                    nat_add(a2743, a2744)?
                                })?;
                                match m2745 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f496({
                                            let a2754 = {
                                                let a2748 = {
                                                    let a2746 = v4;
                                                    let a2747 = 224u64;
                                                    nat_sub(a2746, a2747)
                                                };
                                                let a2749 = 4096u64;
                                                nat_mul(a2748, a2749)?
                                            };
                                            let a2755 = {
                                                let a2752 = {
                                                    let a2750 = v6;
                                                    let a2751 = 64u64;
                                                    nat_mul(a2750, a2751)?
                                                };
                                                let a2753 = v7;
                                                nat_add(a2752, a2753)?
                                            };
                                            nat_add(a2754, a2755)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2756 = 240u64;
                        let a2757 = v4;
                        if if nat_le(a2756, a2757) {
                            let a2758 = v4;
                            let a2759 = 244u64;
                            nat_le(a2758, a2759)
                        } else {
                            false
                        } {
                            let m2762 = f495(v0.clone(), {
                                let a2760 = v1;
                                let a2761 = 1u64;
                                nat_add(a2760, a2761)?
                            })?;
                            match m2762 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2765 = f495(v0.clone(), {
                                        let a2763 = v1;
                                        let a2764 = 2u64;
                                        nat_add(a2763, a2764)?
                                    })?;
                                    match m2765 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2768 = f495(v0, {
                                                let a2766 = v1;
                                                let a2767 = 3u64;
                                                nat_add(a2766, a2767)?
                                            })?;
                                            match m2768 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f496({
                                                        let a2781 = {
                                                            let a2771 = {
                                                                let a2769 = v4;
                                                                let a2770 = 240u64;
                                                                nat_sub(a2769, a2770)
                                                            };
                                                            let a2772 = 262144u64;
                                                            nat_mul(a2771, a2772)?
                                                        };
                                                        let a2782 = {
                                                            let a2779 = {
                                                                let a2773 = v8;
                                                                let a2774 = 4096u64;
                                                                nat_mul(a2773, a2774)?
                                                            };
                                                            let a2780 = {
                                                                let a2777 = {
                                                                    let a2775 = v9;
                                                                    let a2776 = 64u64;
                                                                    nat_mul(a2775, a2776)?
                                                                };
                                                                let a2778 = v10;
                                                                nat_add(a2777, a2778)?
                                                            };
                                                            nat_add(a2779, a2780)?
                                                        };
                                                        nat_add(a2781, a2782)?
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

pub fn f452(v0: u64) -> bool {
    let a2783 = v0;
    let a2784 = 9u64;
    if if if nat_eq(a2783, a2784) {
        true
    } else {
        let a2785 = v0;
        let a2786 = 10u64;
        nat_eq(a2785, a2786)
    } {
        true
    } else {
        let a2787 = v0;
        let a2788 = 13u64;
        nat_eq(a2787, a2788)
    } {
        true
    } else {
        let a2789 = 32u64;
        let a2790 = v0;
        if if if nat_le(a2789, a2790) {
            let a2791 = v0;
            let a2792 = 55295u64;
            nat_le(a2791, a2792)
        } else {
            false
        } {
            true
        } else {
            let a2793 = 57344u64;
            let a2794 = v0;
            if nat_le(a2793, a2794) {
                let a2795 = v0;
                let a2796 = 65533u64;
                nat_le(a2795, a2796)
            } else {
                false
            }
        } {
            true
        } else {
            let a2797 = 65536u64;
            let a2798 = v0;
            if nat_le(a2797, a2798) {
                let a2799 = v0;
                let a2800 = 1114111u64;
                nat_le(a2799, a2800)
            } else {
                false
            }
        }
    }
}

pub fn f453(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f224(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2802 = v2;
        let a2803 = {
            let a2801 = v0;
            length_bytes(a2801)
        };
        if nat_lt(a2802, a2803) {
            Ok(Some({
                let a2804 = v2;
                let a2805 = 1u64;
                nat_add(a2804, a2805)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f454(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f322(f231(v0, v1)?) {
        Ok(Some({
            let a2806 = v1;
            let a2807 = 1u64;
            nat_add(a2806, a2807)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f455(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2808 = f451(v0, v2)?;
    match m2808 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2809 = v2;
            let a2810 = v1;
            if if nat_eq(a2809, a2810) {
                f497({
                    let (h2811, _) = v3;
                    h2811
                })
            } else {
                f498({
                    let (h2812, _) = v3;
                    h2812
                })
            } {
                Ok(Some({
                    let a2814 = v2;
                    let a2815 = {
                        let (_, h2813) = v3;
                        h2813
                    };
                    nat_add(a2814, a2815)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f456(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2816, _) = f499(Fn16::F500(v1, v0.clone()), {
        let a2818 = {
            let a2817 = v0;
            length_bytes(a2817)
        };
        let a2819 = 1u64;
        nat_add(a2818, a2819)?
    }, (v2, 0u64))?;
    Ok(h2816)
}

pub fn f457(v0: u64) -> R<Bytes> {
    let a2820 = v0;
    let a2821 = 128u64;
    if nat_lt(a2820, a2821) {
        Ok(f501(v0))
    } else {
        let a2822 = v0;
        let a2823 = 2048u64;
        if nat_lt(a2822, a2823) {
            let a2834 = f501({
                let a2827 = 192u64;
                let a2828 = {
                    let a2824 = v0;
                    let a2825 = 64u64;
                    let a2826 = 0u64;
                    nat_quot(a2824, a2825, a2826)
                };
                nat_add(a2827, a2828)?
            });
            let a2835 = f501({
                let a2832 = 128u64;
                let a2833 = {
                    let a2829 = v0;
                    let a2830 = 64u64;
                    let a2831 = 0u64;
                    nat_rem(a2829, a2830, a2831)
                };
                nat_add(a2832, a2833)?
            });
            Ok(append_bytes(a2834, a2835))
        } else {
            let a2836 = v0;
            let a2837 = 65536u64;
            if nat_lt(a2836, a2837) {
                let a2858 = f501({
                    let a2841 = 224u64;
                    let a2842 = {
                        let a2838 = v0;
                        let a2839 = 4096u64;
                        let a2840 = 0u64;
                        nat_quot(a2838, a2839, a2840)
                    };
                    nat_add(a2841, a2842)?
                });
                let a2859 = {
                    let a2856 = f501({
                        let a2849 = 128u64;
                        let a2850 = {
                            let a2846 = {
                                let a2843 = v0;
                                let a2844 = 64u64;
                                let a2845 = 0u64;
                                nat_quot(a2843, a2844, a2845)
                            };
                            let a2847 = 64u64;
                            let a2848 = 0u64;
                            nat_rem(a2846, a2847, a2848)
                        };
                        nat_add(a2849, a2850)?
                    });
                    let a2857 = f501({
                        let a2854 = 128u64;
                        let a2855 = {
                            let a2851 = v0;
                            let a2852 = 64u64;
                            let a2853 = 0u64;
                            nat_rem(a2851, a2852, a2853)
                        };
                        nat_add(a2854, a2855)?
                    });
                    append_bytes(a2856, a2857)
                };
                Ok(append_bytes(a2858, a2859))
            } else {
                let a2890 = f501({
                    let a2863 = 240u64;
                    let a2864 = {
                        let a2860 = v0;
                        let a2861 = 262144u64;
                        let a2862 = 0u64;
                        nat_quot(a2860, a2861, a2862)
                    };
                    nat_add(a2863, a2864)?
                });
                let a2891 = {
                    let a2888 = f501({
                        let a2871 = 128u64;
                        let a2872 = {
                            let a2868 = {
                                let a2865 = v0;
                                let a2866 = 4096u64;
                                let a2867 = 0u64;
                                nat_quot(a2865, a2866, a2867)
                            };
                            let a2869 = 64u64;
                            let a2870 = 0u64;
                            nat_rem(a2868, a2869, a2870)
                        };
                        nat_add(a2871, a2872)?
                    });
                    let a2889 = {
                        let a2886 = f501({
                            let a2879 = 128u64;
                            let a2880 = {
                                let a2876 = {
                                    let a2873 = v0;
                                    let a2874 = 64u64;
                                    let a2875 = 0u64;
                                    nat_quot(a2873, a2874, a2875)
                                };
                                let a2877 = 64u64;
                                let a2878 = 0u64;
                                nat_rem(a2876, a2877, a2878)
                            };
                            nat_add(a2879, a2880)?
                        });
                        let a2887 = f501({
                            let a2884 = 128u64;
                            let a2885 = {
                                let a2881 = v0;
                                let a2882 = 64u64;
                                let a2883 = 0u64;
                                nat_rem(a2881, a2882, a2883)
                            };
                            nat_add(a2884, a2885)?
                        });
                        append_bytes(a2886, a2887)
                    };
                    append_bytes(a2888, a2889)
                };
                Ok(append_bytes(a2890, a2891))
            }
        }
    }
}

pub fn f458(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f502(v2, v0, v1, v3)
}

pub fn f459(p2895: Bytes, p2896: u64) -> Result<Bytes, Bytes> {
    let m2894 = (p2895, p2896);
    {
        let mut m2894 = m2894;
        loop {
            let step = {
                let (v0, v1) = m2894;
                let m2892 = v1;
                if m2892 == 0 {
                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Err::<Bytes, Bytes>(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines"))))
                } else {
                    let v2 = m2892 - 1;
                    if f399(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)"))))
                    } else {
                        if f37(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Nat"))))
                        } else {
                            if f37(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f8(Str::lit("Int"))))
                            } else {
                                if f37(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), Result<Bytes, Bytes>>(Ok::<Bytes, Bytes>(f503()))
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
                                                let m2893 = f504(v0.clone());
                                                match m2893 {
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
                Ok(next) => { m2894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: Adt19) -> R<Bytes> {
    f259(v0)
}

pub fn f461(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m2897 = v1;
    match m2897 {
        Adt19::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(v7, v8) => {
            Ok(Some({
                let a2908 = if if v7 {
                    let a2906 = 0u64;
                    let a2907 = v8;
                    nat_lt(a2906, a2907)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2909 = f356(false, v8)?;
                append_bytes(a2908, a2909)
            }))
        }
        Adt19::C2(v3, v4, v5) => {
            let m2900 = f506({
                let a2898 = f8(Str::lit("n"));
                let a2899 = f357(false, v4, v5)?;
                append_bytes(a2898, a2899)
            })?;
            match m2900 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2904 = f8(Str::lit("nMinus"));
                        let a2905 = f32(v6.clone(), 1u64, {
                            let a2902 = {
                                let a2901 = v6;
                                length_bytes(a2901)
                            };
                            let a2903 = 1u64;
                            nat_sub(a2902, a2903)
                        });
                        append_bytes(a2904, a2905)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C3(v2) => {
            f505(v0, v2)
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

pub fn f462(v0: Bytes) -> R<Option<Bytes>> {
    if f507({
        let a2910 = v0.clone();
        length_bytes(a2910)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f508(v0)?;
        if f509(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2913 = {
                let a2911 = v1.clone();
                let a2912 = 0u64;
                index_bytes(a2911, a2912)
            };
            match m2913 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f420({
                        let v3: u8 = v2;
                        let a2958 = {
                            let a2934 = {
                                let a2922 = {
                                    let a2916 = {
                                        let a2914 = v3;
                                        let a2915 = 128u8;
                                        fixed_u8::bit_and(a2914, a2915)
                                    };
                                    let a2917 = 128u8;
                                    if equal(a2916, a2917) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2923 = {
                                    let a2920 = {
                                        let a2918 = v3;
                                        let a2919 = 64u8;
                                        fixed_u8::bit_and(a2918, a2919)
                                    };
                                    let a2921 = 64u8;
                                    if equal(a2920, a2921) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2922, a2923)?
                            };
                            let a2935 = {
                                let a2932 = {
                                    let a2926 = {
                                        let a2924 = v3;
                                        let a2925 = 32u8;
                                        fixed_u8::bit_and(a2924, a2925)
                                    };
                                    let a2927 = 32u8;
                                    if equal(a2926, a2927) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2933 = {
                                    let a2930 = {
                                        let a2928 = v3;
                                        let a2929 = 16u8;
                                        fixed_u8::bit_and(a2928, a2929)
                                    };
                                    let a2931 = 16u8;
                                    if equal(a2930, a2931) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2932, a2933)?
                            };
                            nat_add(a2934, a2935)?
                        };
                        let a2959 = {
                            let a2956 = {
                                let a2944 = {
                                    let a2938 = {
                                        let a2936 = v3;
                                        let a2937 = 8u8;
                                        fixed_u8::bit_and(a2936, a2937)
                                    };
                                    let a2939 = 8u8;
                                    if equal(a2938, a2939) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2945 = {
                                    let a2942 = {
                                        let a2940 = v3;
                                        let a2941 = 4u8;
                                        fixed_u8::bit_and(a2940, a2941)
                                    };
                                    let a2943 = 4u8;
                                    if equal(a2942, a2943) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2944, a2945)?
                            };
                            let a2957 = {
                                let a2954 = {
                                    let a2948 = {
                                        let a2946 = v3;
                                        let a2947 = 2u8;
                                        fixed_u8::bit_and(a2946, a2947)
                                    };
                                    let a2949 = 2u8;
                                    if equal(a2948, a2949) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2955 = {
                                    let a2952 = {
                                        let a2950 = v3;
                                        let a2951 = 1u8;
                                        fixed_u8::bit_and(a2950, a2951)
                                    };
                                    let a2953 = 1u8;
                                    if equal(a2952, a2953) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2954, a2955)?
                            };
                            nat_add(a2956, a2957)?
                        };
                        nat_add(a2958, a2959)?
                    }) {
                        Ok(Some({
                            let a2960 = f8(Str::lit("n"));
                            let a2961 = v1;
                            append_bytes(a2960, a2961)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f510({
            let a2962 = v0.clone();
            length_bytes(a2962)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f463(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2963 = v1;
    match m2963.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2964 = f505(v0, v2)?;
            match m2964 {
                None => {
                    f463(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f463(v0, v3)?))
                }
            }
        }
    }
}

pub fn f464(v0: Fn15, p2969: u64, p2970: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2968 = (p2969, p2970);
    {
        let mut m2968 = m2968;
        loop {
            let step = {
                let (v1, v2) = m2968;
                let m2965 = v1;
                if m2965 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2965 - 1;
                    let m2967 = {
                        let c2966: Fn15 = v0.clone();
                        c2966.apply(v2.clone())?
                    };
                    match m2967 {
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
                Ok(next) => { m2968 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f511(v0, v1)
}

pub fn f466(v0: Bytes, v1: u64) -> R<u64> {
    let (h2971, _) = f269(Fn3::F512(v0.clone()), {
        let a2973 = {
            let a2972 = v0;
            length_bytes(a2972)
        };
        let a2974 = 1u64;
        nat_add(a2973, a2974)?
    }, v1)?;
    Ok(h2971)
}

pub fn f467(v0: Bytes) -> R<u64> {
    let (h2975, _) = f269(Fn3::F513(v0.clone()), {
        let a2977 = {
            let a2976 = v0;
            length_bytes(a2976)
        };
        let a2978 = 1u64;
        nat_add(a2977, a2978)?
    }, 0u64)?;
    Ok(h2975)
}

pub fn f468(v0: Bytes) -> R<Option<u64>> {
    if f514(v0.clone())? {
        Ok(Some({
            let (_, h2979) = {
                let (h2980, _) = f499(Fn16::F515(v0.clone()), {
                    let a2981 = v0;
                    length_bytes(a2981)
                }, (0u64, 0u64))?;
                h2980
            };
            h2979
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f469(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2983 = {
            let a2982 = v1.clone();
            length_bytes(a2982)
        };
        let a2984 = f516(v1)?;
        nat_sub(a2983, a2984)
    });
    let v5: Bytes = {
        let a2985 = v0;
        let a2986 = v4.clone();
        append_bytes(a2985, a2986)
    };
    let v6: u64 = f467(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2988 = {
            let a2987 = v5;
            length_bytes(a2987)
        };
        let a2989 = v6;
        nat_sub(a2988, a2989)
    });
    let v8: u64 = {
        let a2990 = v4;
        length_bytes(a2990)
    };
    let v9: u64 = {
        let a2991 = v7.clone();
        length_bytes(a2991)
    };
    let a2992 = v9;
    let a2993 = 0u64;
    if nat_eq(a2992, a2993) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a2994 = v3;
            let a2995 = v8;
            nat_lt(a2994, a2995)
        } {
            let v10: u64 = if v2 {
                let a2996 = v3;
                let a2997 = v8;
                nat_add(a2996, a2997)?
            } else {
                let a2998 = v8;
                let a2999 = v3;
                nat_sub(a2998, a2999)
            };
            let a3000 = v10;
            let a3001 = f516(v7.clone())?;
            if nat_le(a3000, a3001) {
                Ok(Some(f32(v7, 0u64, {
                    let a3002 = v9;
                    let a3003 = v10;
                    nat_sub(a3002, a3003)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a3004 = v3;
                let a3005 = v8;
                nat_sub(a3004, a3005)
            };
            let a3008 = {
                let a3006 = v9;
                let a3007 = v11;
                nat_add(a3006, a3007)?
            };
            let a3009 = 19u64;
            if nat_le(a3008, a3009) {
                Ok(Some({
                    let a3010 = v7;
                    let a3011 = f430(v11);
                    append_bytes(a3010, a3011)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f470(v0: bool, v1: u64) -> bool {
    if v0 {
        let a3012 = 0u64;
        let a3013 = v1;
        if nat_lt(a3012, a3013) {
            let a3014 = v1;
            let a3015 = 9223372036854775808u64;
            nat_le(a3014, a3015)
        } else {
            false
        }
    } else {
        let a3016 = v1;
        let a3017 = 9223372036854775807u64;
        nat_le(a3016, a3017)
    }
}

pub fn f471(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3019 = {
            let a3018 = v1.clone();
            length_bytes(a3018)
        };
        let a3020 = f516(v1)?;
        nat_sub(a3019, a3020)
    });
    let v5: Bytes = {
        let a3021 = v0;
        let a3022 = v4.clone();
        append_bytes(a3021, a3022)
    };
    let v6: u64 = f467(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3024 = {
            let a3023 = v5;
            length_bytes(a3023)
        };
        let a3025 = v6;
        nat_sub(a3024, a3025)
    });
    let v8: u64 = {
        let a3026 = v4;
        length_bytes(a3026)
    };
    let v9: u64 = {
        let a3027 = v7.clone();
        length_bytes(a3027)
    };
    let a3028 = v9;
    let a3029 = 0u64;
    if nat_eq(a3028, a3029) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if if v2 {
            true
        } else {
            let a3030 = v3;
            let a3031 = v8;
            nat_lt(a3030, a3031)
        } {
            let v10: u64 = if v2 {
                let a3032 = v3;
                let a3033 = v8;
                nat_add(a3032, a3033)?
            } else {
                let a3034 = v8;
                let a3035 = v3;
                nat_sub(a3034, a3035)
            };
            let v11: u64 = f516(v7.clone())?;
            let a3036 = v10;
            let a3037 = v11;
            if nat_le(a3036, a3037) {
                Ok(Some((f32(v7, 0u64, {
                    let a3038 = v9;
                    let a3039 = v10;
                    nat_sub(a3038, a3039)
                }), 0u64)))
            } else {
                let a3042 = {
                    let a3040 = v10;
                    let a3041 = v11;
                    nat_sub(a3040, a3041)
                };
                let a3043 = 19u64;
                if nat_le(a3042, a3043) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3044 = v9;
                        let a3045 = v11;
                        nat_sub(a3044, a3045)
                    }), {
                        let a3046 = v10;
                        let a3047 = v11;
                        nat_sub(a3046, a3047)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3048 = v3;
                let a3049 = v8;
                nat_sub(a3048, a3049)
            };
            let a3052 = {
                let a3050 = v9;
                let a3051 = v12;
                nat_add(a3050, a3051)?
            };
            let a3053 = 19u64;
            if nat_le(a3052, a3053) {
                Ok(Some(({
                    let a3054 = v7;
                    let a3055 = f430(v12);
                    append_bytes(a3054, a3055)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f472(v0: Adt19) -> List<(Bytes, Adt19)> {
    let m3056 = v0;
    match m3056 {
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
        Adt19::C5(r3057) => {
            let v4 = (*r3057).clone();
            f472(v4)
        }
        Adt19::C6 => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C7(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C8(v1, r3058, r3059) => {
            let v2 = (*r3058).clone();
            let v3 = (*r3059).clone();
            List::cons((v1, v2), f472(v3))
        }
    }
}

pub fn f473(p3064: List<(Bytes, Adt19)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt19)>> {
    let m3063 = p3064;
    {
        let mut m3063 = m3063;
        loop {
            let step = {
                let v0 = m3063;
                let m3060 = v0;
                match m3060.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(None::<(Bytes, Adt19)>))
                    }
                    Some((v3, v4)) => {
                        if f517(f518({
                            let (_, h3061) = v3.clone();
                            h3061
                        }, List::<Bytes>::nil())?, f256({
                            let (h3062, _) = v3.clone();
                            h3062
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(v0: List<(Bytes, Adt19)>, v1: Bytes) -> List<(Bytes, Adt19)> {
    let m3065 = v0;
    match m3065.uncons() {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some((v2, v3)) => {
            if f37({
                let (h3066, _) = v2.clone();
                h3066
            }, v1.clone()) {
                f474(v3, v1)
            } else {
                List::cons(v2, f474(v3, v1))
            }
        }
    }
}

pub fn f475() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f476() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f477(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f519(v0, f520(v1))
}

pub fn f478(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m3067 = v0;
    match m3067.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f478(v3, false))
            } else {
                f478(v3, true)
            }
        }
    }
}

pub fn f479(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3071 = {
        let a3069 = v0.clone();
        let a3070 = {
            let (h3068, _) = v1.clone();
            h3068
        };
        index_bytes(a3069, a3070)
    };
    match m3071 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3073 = {
                    let (h3072, _) = v1.clone();
                    h3072
                };
                let a3074 = 1u64;
                nat_add(a3073, a3074)?
            }, {
                let a3077 = {
                    let (_, h3075) = v1;
                    h3075
                };
                let a3078 = f521(v0, {
                    let (h3076, _) = v1;
                    h3076
                }, v2)?;
                append_bytes(a3077, a3078)
            })))
        }
    }
}

pub fn f480(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3079 = v0.clone();
    match m3079.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3082 = {
                let (h3080, _) = v2;
                h3080
            };
            let a3083 = {
                let (h3081, _) = v1.clone();
                h3081
            };
            if equal(a3082, a3083) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f481(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f522(f523(Fn17::F524, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3088 = {
        let (h3084, _) = f525(Fn18::F526, {
            let a3086 = {
                let a3085 = v0;
                length_list(a3085)
            };
            let a3087 = 1u64;
            nat_add(a3086, a3087)?
        }, v1)?;
        h3084
    };
    match m3088.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f482(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3093 = {
        let a3090 = f358({
            let (h3089, _) = v0;
            h3089
        })?;
        let a3091 = Bytes::lit(&[0x3a]);
        append_bytes(a3090, a3091)
    };
    let a3094 = {
        let (_, h3092) = v0;
        h3092
    };
    Ok(append_bytes(a3093, a3094))
}

pub fn f483(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f372({
        let (_, h3095) = {
            let (h3096, _) = f527(Fn19::F528, {
                let a3098 = {
                    let a3097 = v0.clone();
                    length_list(a3097)
                };
                let a3099 = 1u64;
                nat_add(a3098, a3099)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h3096
        };
        h3095
    }))
}

pub fn f484(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3100 = v0;
    match m3100.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f88(v2.clone(), v1.clone()) {
                f484(v3, v1)
            } else {
                List::cons(v2, f484(v3, v1))
            }
        }
    }
}

pub fn f485(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f486(v0: List<Adt14>) -> List<Bytes> {
    let m3101 = v0;
    match m3101.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3102 = f163(v1, f8(Str::lit("std")));
            match m3102 {
                None => {
                    f486(v2)
                }
                Some(v3) => {
                    List::cons(f161(v3, f8(Str::lit("authority"))), f486(v2))
                }
            }
        }
    }
}

pub fn f487(v0: List<Adt14>) -> List<Bytes> {
    let m3103 = v0;
    match m3103.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m3104 = f164(f163(v1, f8(Str::lit("req"))));
            match m3104 {
                None => {
                    f487(v2)
                }
                Some(v3) => {
                    List::cons(v3, f487(v2))
                }
            }
        }
    }
}

pub fn f488(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f529(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h3105) => {
            h3105
        }
    });
    let v2: u32 = f448(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h3106, _, _) => {
            h3106
        }
    }, f448(f530(match v0.clone() {
        Adt27::C0(_, _, _, _, h3107, _, _, _, _, _) => {
            h3107
        }
    }), f448(f531(match v0.clone() {
        Adt27::C0(_, _, _, _, h3108, _, _, _, _, _) => {
            h3108
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h3109, _, _, _, _) => {
            h3109
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h3110, _, _, _) => {
            h3110
        }
    }), f448(v1, {
        let m3113 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h3112, _) => {
                h3112
            }
        };
        match m3113 {
            Adt26::C0(h3111, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3111
            }
        }
    }))));
    let v3: u32 = f448(f532(match v0.clone() {
        Adt27::C0(h3114, _, _, _, _, _, _, _, _, _) => {
            h3114
        }
    }), f533(match v0.clone() {
        Adt27::C0(h3115, _, _, _, _, _, _, _, _, _) => {
            h3115
        }
    }, match v0.clone() {
        Adt27::C0(_, h3116, _, _, _, _, _, _, _, _) => {
            h3116
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h3117, _, _, _, _, _, _, _) => {
            h3117
        }
    }));
    Ok(Adt27::C0(f448(v2, v3), match v0.clone() {
        Adt27::C0(h3118, _, _, _, _, _, _, _, _, _) => {
            h3118
        }
    }, match v0.clone() {
        Adt27::C0(_, h3119, _, _, _, _, _, _, _, _) => {
            h3119
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h3120, _, _, _, _, _, _, _) => {
            h3120
        }
    }, f448(match v0.clone() {
        Adt27::C0(_, _, _, h3121, _, _, _, _, _, _) => {
            h3121
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h3122, _, _, _, _, _) => {
            h3122
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h3123, _, _, _, _) => {
            h3123
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h3124, _, _, _) => {
            h3124
        }
    }, f534(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h3125, _) => {
            h3125
        }
    }), {
        let a3127 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h3126) => {
                h3126
            }
        };
        let a3128 = 1u64;
        nat_add(a3127, a3128)?
    }))
}

pub fn f489(v0: u32) -> u64 {
    let m3130 = {
        let a3129 = v0;
        fixed_u64::convert(i128::from(a3129))
    };
    match m3130 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f490(v0: u64) -> u32 {
    let m3134 = {
        let a3133 = {
            let a3131 = v0;
            let a3132 = 4294967295u64;
            fixed_u64::bit_and(a3131, a3132)
        };
        fixed_u32::convert(i128::from(a3133))
    };
    match m3134 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f491(v0: u64) -> u32 {
    let (_, h3135) = {
        let (_, h3136) = f535(Fn20::F536, 32u64, (v0, (1u32, 0u32)));
        h3136
    };
    h3135
}

pub fn f492(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3141 = f537(v0, 24u32);
    let a3142 = {
        let a3139 = f537(v1, 16u32);
        let a3140 = {
            let a3137 = f537(v2, 8u32);
            let a3138 = v3;
            fixed_u32::bit_or(a3137, a3138)
        };
        fixed_u32::bit_or(a3139, a3140)
    };
    fixed_u32::bit_or(a3141, a3142)
}

pub fn f493(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3143 = v2;
    let a3144 = v1;
    if nat_lt(a3143, a3144) {
        let m3147 = {
            let a3145 = v0;
            let a3146 = v2;
            index_bytes(a3145, a3146)
        };
        match m3147 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3149 = {
                    let a3148 = v3;
                    fixed_u32::convert(i128::from(a3148))
                };
                match m3149 {
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
        let a3150 = v2;
        let a3151 = v1;
        if nat_eq(a3150, a3151) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f494(v0: u8) -> bool {
    let a3152 = v0;
    let a3153 = 32u8;
    if if if equal(a3152, a3153) {
        true
    } else {
        let a3154 = v0;
        let a3155 = 9u8;
        equal(a3154, a3155)
    } {
        true
    } else {
        let a3156 = v0;
        let a3157 = 10u8;
        equal(a3156, a3157)
    } {
        true
    } else {
        let a3158 = v0;
        let a3159 = 12u8;
        if equal(a3158, a3159) {
            true
        } else {
            let a3160 = v0;
            let a3161 = 13u8;
            equal(a3160, a3161)
        }
    }
}

pub fn f495(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3164 = {
        let a3162 = v0;
        let a3163 = v1;
        index_bytes(a3162, a3163)
    };
    match m3164 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3209 = {
                    let a3185 = {
                        let a3173 = {
                            let a3167 = {
                                let a3165 = v3;
                                let a3166 = 128u8;
                                fixed_u8::bit_and(a3165, a3166)
                            };
                            let a3168 = 128u8;
                            if equal(a3167, a3168) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3174 = {
                            let a3171 = {
                                let a3169 = v3;
                                let a3170 = 64u8;
                                fixed_u8::bit_and(a3169, a3170)
                            };
                            let a3172 = 64u8;
                            if equal(a3171, a3172) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3173, a3174)?
                    };
                    let a3186 = {
                        let a3183 = {
                            let a3177 = {
                                let a3175 = v3;
                                let a3176 = 32u8;
                                fixed_u8::bit_and(a3175, a3176)
                            };
                            let a3178 = 32u8;
                            if equal(a3177, a3178) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3184 = {
                            let a3181 = {
                                let a3179 = v3;
                                let a3180 = 16u8;
                                fixed_u8::bit_and(a3179, a3180)
                            };
                            let a3182 = 16u8;
                            if equal(a3181, a3182) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3183, a3184)?
                    };
                    nat_add(a3185, a3186)?
                };
                let a3210 = {
                    let a3207 = {
                        let a3195 = {
                            let a3189 = {
                                let a3187 = v3;
                                let a3188 = 8u8;
                                fixed_u8::bit_and(a3187, a3188)
                            };
                            let a3190 = 8u8;
                            if equal(a3189, a3190) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3196 = {
                            let a3193 = {
                                let a3191 = v3;
                                let a3192 = 4u8;
                                fixed_u8::bit_and(a3191, a3192)
                            };
                            let a3194 = 4u8;
                            if equal(a3193, a3194) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3195, a3196)?
                    };
                    let a3208 = {
                        let a3205 = {
                            let a3199 = {
                                let a3197 = v3;
                                let a3198 = 2u8;
                                fixed_u8::bit_and(a3197, a3198)
                            };
                            let a3200 = 2u8;
                            if equal(a3199, a3200) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3206 = {
                            let a3203 = {
                                let a3201 = v3;
                                let a3202 = 1u8;
                                fixed_u8::bit_and(a3201, a3202)
                            };
                            let a3204 = 1u8;
                            if equal(a3203, a3204) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3205, a3206)?
                    };
                    nat_add(a3207, a3208)?
                };
                nat_add(a3209, a3210)?
            };
            let a3211 = 128u64;
            let a3212 = v4;
            if if nat_le(a3211, a3212) {
                let a3213 = v4;
                let a3214 = 191u64;
                nat_le(a3213, a3214)
            } else {
                false
            } {
                Ok(Some({
                    let a3215 = v4;
                    let a3216 = 128u64;
                    nat_sub(a3215, a3216)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f496(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3217 = v1;
    let a3218 = v0;
    if if nat_le(a3217, a3218) {
        let a3219 = v0;
        let a3220 = v2;
        if nat_le(a3219, a3220) {
            let a3221 = v0;
            let a3222 = 55296u64;
            if nat_lt(a3221, a3222) {
                true
            } else {
                let a3223 = 57343u64;
                let a3224 = v0;
                nat_lt(a3223, a3224)
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

pub fn f497(v0: u64) -> bool {
    let a3225 = v0;
    let a3226 = 58u64;
    if if if nat_eq(a3225, a3226) {
        true
    } else {
        let a3227 = v0;
        let a3228 = 95u64;
        nat_eq(a3227, a3228)
    } {
        true
    } else {
        let a3229 = 65u64;
        let a3230 = v0;
        if if nat_le(a3229, a3230) {
            let a3231 = v0;
            let a3232 = 90u64;
            nat_le(a3231, a3232)
        } else {
            false
        } {
            true
        } else {
            let a3233 = 97u64;
            let a3234 = v0;
            if nat_le(a3233, a3234) {
                let a3235 = v0;
                let a3236 = 122u64;
                nat_le(a3235, a3236)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a3237 = 192u64;
        let a3238 = v0;
        if if if if nat_le(a3237, a3238) {
            let a3239 = v0;
            let a3240 = 214u64;
            nat_le(a3239, a3240)
        } else {
            false
        } {
            true
        } else {
            let a3241 = 216u64;
            let a3242 = v0;
            if nat_le(a3241, a3242) {
                let a3243 = v0;
                let a3244 = 246u64;
                nat_le(a3243, a3244)
            } else {
                false
            }
        } {
            true
        } else {
            let a3245 = 248u64;
            let a3246 = v0;
            if if nat_le(a3245, a3246) {
                let a3247 = v0;
                let a3248 = 767u64;
                nat_le(a3247, a3248)
            } else {
                false
            } {
                true
            } else {
                let a3249 = 880u64;
                let a3250 = v0;
                if nat_le(a3249, a3250) {
                    let a3251 = v0;
                    let a3252 = 893u64;
                    nat_le(a3251, a3252)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a3253 = 895u64;
            let a3254 = v0;
            if if if if nat_le(a3253, a3254) {
                let a3255 = v0;
                let a3256 = 8191u64;
                nat_le(a3255, a3256)
            } else {
                false
            } {
                true
            } else {
                let a3257 = 8204u64;
                let a3258 = v0;
                if nat_le(a3257, a3258) {
                    let a3259 = v0;
                    let a3260 = 8205u64;
                    nat_le(a3259, a3260)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3261 = 8304u64;
                let a3262 = v0;
                if if nat_le(a3261, a3262) {
                    let a3263 = v0;
                    let a3264 = 8591u64;
                    nat_le(a3263, a3264)
                } else {
                    false
                } {
                    true
                } else {
                    let a3265 = 11264u64;
                    let a3266 = v0;
                    if nat_le(a3265, a3266) {
                        let a3267 = v0;
                        let a3268 = 12271u64;
                        nat_le(a3267, a3268)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a3269 = 12289u64;
                let a3270 = v0;
                if if if nat_le(a3269, a3270) {
                    let a3271 = v0;
                    let a3272 = 55295u64;
                    nat_le(a3271, a3272)
                } else {
                    false
                } {
                    true
                } else {
                    let a3273 = 63744u64;
                    let a3274 = v0;
                    if nat_le(a3273, a3274) {
                        let a3275 = v0;
                        let a3276 = 64975u64;
                        nat_le(a3275, a3276)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a3277 = 65008u64;
                    let a3278 = v0;
                    if if nat_le(a3277, a3278) {
                        let a3279 = v0;
                        let a3280 = 65533u64;
                        nat_le(a3279, a3280)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a3281 = 65536u64;
                        let a3282 = v0;
                        if nat_le(a3281, a3282) {
                            let a3283 = v0;
                            let a3284 = 983039u64;
                            nat_le(a3283, a3284)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f498(v0: u64) -> bool {
    if f497(v0) {
        true
    } else {
        let a3285 = v0;
        let a3286 = 45u64;
        if if if nat_eq(a3285, a3286) {
            true
        } else {
            let a3287 = v0;
            let a3288 = 46u64;
            nat_eq(a3287, a3288)
        } {
            true
        } else {
            let a3289 = 48u64;
            let a3290 = v0;
            if nat_le(a3289, a3290) {
                let a3291 = v0;
                let a3292 = 57u64;
                nat_le(a3291, a3292)
            } else {
                false
            }
        } {
            true
        } else {
            let a3293 = v0;
            let a3294 = 183u64;
            if if nat_eq(a3293, a3294) {
                true
            } else {
                let a3295 = 768u64;
                let a3296 = v0;
                if nat_le(a3295, a3296) {
                    let a3297 = v0;
                    let a3298 = 879u64;
                    nat_le(a3297, a3298)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3299 = 8255u64;
                let a3300 = v0;
                if nat_le(a3299, a3300) {
                    let a3301 = v0;
                    let a3302 = 8256u64;
                    nat_le(a3301, a3302)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f499(v0: Fn16, p3307: u64, p3308: (u64, u64)) -> R<((u64, u64), bool)> {
    let m3306 = (p3307, p3308);
    {
        let mut m3306 = m3306;
        loop {
            let step = {
                let (v1, v2) = m3306;
                let m3303 = v1;
                if m3303 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3303 - 1;
                    let m3305 = {
                        let c3304: Fn16 = v0.clone();
                        c3304.apply(v2)?
                    };
                    match m3305 {
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
                Ok(next) => { m3306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f538(v1, v0, v2)
}

pub fn f501(v0: u64) -> Bytes {
    f539(f540({
        let a3309 = v0;
        let a3310 = 16u64;
        let a3311 = 0u64;
        nat_quot(a3309, a3310, a3311)
    }), {
        let a3312 = v0;
        let a3313 = 16u64;
        let a3314 = 0u64;
        nat_rem(a3312, a3313, a3314)
    })
}

pub fn f502(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f541(f231(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3315 = v3;
            let a3316 = 1u64;
            nat_add(a3315, a3316)?
        }))
    }
}

pub fn f503() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f504(v0: Bytes) -> Option<Bytes> {
    if f399(v0.clone(), List::cons(f8(Str::lit("string")), List::cons(f8(Str::lit("boolean")), List::cons(f8(Str::lit("decimal")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::cons(f8(Str::lit("duration")), List::cons(f8(Str::lit("dateTime")), List::cons(f8(Str::lit("time")), List::cons(f8(Str::lit("date")), List::cons(f8(Str::lit("gYearMonth")), List::cons(f8(Str::lit("gYear")), List::cons(f8(Str::lit("gMonthDay")), List::cons(f8(Str::lit("gDay")), List::cons(f8(Str::lit("gMonth")), List::cons(f8(Str::lit("hexBinary")), List::cons(f8(Str::lit("base64Binary")), List::cons(f8(Str::lit("anyURI")), List::cons(f8(Str::lit("QName")), List::cons(f8(Str::lit("NOTATION")), List::<Bytes>::nil())))))))))))))))))))) {
        Some(f8(Str::lit("anyAtomicType")))
    } else {
        if f37(v0.clone(), f8(Str::lit("normalizedString"))) {
            Some(f8(Str::lit("string")))
        } else {
            if f37(v0.clone(), f8(Str::lit("token"))) {
                Some(f8(Str::lit("normalizedString")))
            } else {
                if f399(v0.clone(), List::cons(f8(Str::lit("language")), List::cons(f8(Str::lit("NMTOKEN")), List::cons(f8(Str::lit("Name")), List::<Bytes>::nil())))) {
                    Some(f8(Str::lit("token")))
                } else {
                    if f37(v0.clone(), f8(Str::lit("NCName"))) {
                        Some(f8(Str::lit("Name")))
                    } else {
                        if f399(v0.clone(), List::cons(f8(Str::lit("ID")), List::cons(f8(Str::lit("IDREF")), List::cons(f8(Str::lit("ENTITY")), List::<Bytes>::nil())))) {
                            Some(f8(Str::lit("NCName")))
                        } else {
                            if f37(v0.clone(), f8(Str::lit("integer"))) {
                                Some(f8(Str::lit("decimal")))
                            } else {
                                if f399(v0.clone(), List::cons(f8(Str::lit("nonPositiveInteger")), List::cons(f8(Str::lit("long")), List::cons(f8(Str::lit("nonNegativeInteger")), List::<Bytes>::nil())))) {
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
                                                    if f399(v0.clone(), List::cons(f8(Str::lit("unsignedLong")), List::cons(f8(Str::lit("positiveInteger")), List::<Bytes>::nil()))) {
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
                                                                    if f399(v0.clone(), List::cons(f8(Str::lit("yearMonthDuration")), List::cons(f8(Str::lit("dayTimeDuration")), List::<Bytes>::nil()))) {
                                                                        Some(f8(Str::lit("duration")))
                                                                    } else {
                                                                        if f37(v0.clone(), f8(Str::lit("dateTimeStamp"))) {
                                                                            Some(f8(Str::lit("dateTime")))
                                                                        } else {
                                                                            if f399(v0, List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))))) {
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

pub fn f505(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f506(v1)
    } else {
        f462(v1)
    }
}

pub fn f506(v0: Bytes) -> R<Option<Bytes>> {
    let m3318 = f542({
        let a3317 = v0.clone();
        length_bytes(a3317)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3318 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3320 = {
                let a3319 = v1.clone();
                length_bytes(a3319)
            };
            let a3321 = 0u64;
            if nat_eq(a3320, a3321) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f507(p3377: u64, v1: Bytes, p3378: u64) -> R<bool> {
    let m3376 = (p3377, p3378);
    {
        let mut m3376 = m3376;
        loop {
            let step = {
                let (v0, v2) = m3376;
                let m3322 = v0;
                if m3322 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3322 - 1;
                    let m3325 = {
                        let a3323 = v1.clone();
                        let a3324 = v2;
                        index_bytes(a3323, a3324)
                    };
                    match m3325 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3372 = {
                                let v5: u8 = v4;
                                let a3370 = {
                                    let a3346 = {
                                        let a3334 = {
                                            let a3328 = {
                                                let a3326 = v5;
                                                let a3327 = 128u8;
                                                fixed_u8::bit_and(a3326, a3327)
                                            };
                                            let a3329 = 128u8;
                                            if equal(a3328, a3329) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3335 = {
                                            let a3332 = {
                                                let a3330 = v5;
                                                let a3331 = 64u8;
                                                fixed_u8::bit_and(a3330, a3331)
                                            };
                                            let a3333 = 64u8;
                                            if equal(a3332, a3333) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3334, a3335)?
                                    };
                                    let a3347 = {
                                        let a3344 = {
                                            let a3338 = {
                                                let a3336 = v5;
                                                let a3337 = 32u8;
                                                fixed_u8::bit_and(a3336, a3337)
                                            };
                                            let a3339 = 32u8;
                                            if equal(a3338, a3339) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3345 = {
                                            let a3342 = {
                                                let a3340 = v5;
                                                let a3341 = 16u8;
                                                fixed_u8::bit_and(a3340, a3341)
                                            };
                                            let a3343 = 16u8;
                                            if equal(a3342, a3343) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3344, a3345)?
                                    };
                                    nat_add(a3346, a3347)?
                                };
                                let a3371 = {
                                    let a3368 = {
                                        let a3356 = {
                                            let a3350 = {
                                                let a3348 = v5;
                                                let a3349 = 8u8;
                                                fixed_u8::bit_and(a3348, a3349)
                                            };
                                            let a3351 = 8u8;
                                            if equal(a3350, a3351) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3357 = {
                                            let a3354 = {
                                                let a3352 = v5;
                                                let a3353 = 4u8;
                                                fixed_u8::bit_and(a3352, a3353)
                                            };
                                            let a3355 = 4u8;
                                            if equal(a3354, a3355) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3356, a3357)?
                                    };
                                    let a3369 = {
                                        let a3366 = {
                                            let a3360 = {
                                                let a3358 = v5;
                                                let a3359 = 2u8;
                                                fixed_u8::bit_and(a3358, a3359)
                                            };
                                            let a3361 = 2u8;
                                            if equal(a3360, a3361) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a3367 = {
                                            let a3364 = {
                                                let a3362 = v5;
                                                let a3363 = 1u8;
                                                fixed_u8::bit_and(a3362, a3363)
                                            };
                                            let a3365 = 1u8;
                                            if equal(a3364, a3365) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a3366, a3367)?
                                    };
                                    nat_add(a3368, a3369)?
                                };
                                nat_add(a3370, a3371)?
                            };
                            let a3373 = 128u64;
                            if nat_lt(a3372, a3373) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3374 = v2;
                                    let a3375 = 1u64;
                                    nat_add(a3374, a3375)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f352(v0.clone(), 0u64, {
        let a3379 = v0;
        length_bytes(a3379)
    }, false, Bytes::lit(&[]))?;
    let m3382 = {
        let a3380 = v1.clone();
        let a3381 = 0u64;
        index_bytes(a3380, a3381)
    };
    match m3382 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3427 = {
                    let a3403 = {
                        let a3391 = {
                            let a3385 = {
                                let a3383 = v3;
                                let a3384 = 128u8;
                                fixed_u8::bit_and(a3383, a3384)
                            };
                            let a3386 = 128u8;
                            if equal(a3385, a3386) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3392 = {
                            let a3389 = {
                                let a3387 = v3;
                                let a3388 = 64u8;
                                fixed_u8::bit_and(a3387, a3388)
                            };
                            let a3390 = 64u8;
                            if equal(a3389, a3390) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3391, a3392)?
                    };
                    let a3404 = {
                        let a3401 = {
                            let a3395 = {
                                let a3393 = v3;
                                let a3394 = 32u8;
                                fixed_u8::bit_and(a3393, a3394)
                            };
                            let a3396 = 32u8;
                            if equal(a3395, a3396) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3402 = {
                            let a3399 = {
                                let a3397 = v3;
                                let a3398 = 16u8;
                                fixed_u8::bit_and(a3397, a3398)
                            };
                            let a3400 = 16u8;
                            if equal(a3399, a3400) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3401, a3402)?
                    };
                    nat_add(a3403, a3404)?
                };
                let a3428 = {
                    let a3425 = {
                        let a3413 = {
                            let a3407 = {
                                let a3405 = v3;
                                let a3406 = 8u8;
                                fixed_u8::bit_and(a3405, a3406)
                            };
                            let a3408 = 8u8;
                            if equal(a3407, a3408) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3414 = {
                            let a3411 = {
                                let a3409 = v3;
                                let a3410 = 4u8;
                                fixed_u8::bit_and(a3409, a3410)
                            };
                            let a3412 = 4u8;
                            if equal(a3411, a3412) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3413, a3414)?
                    };
                    let a3426 = {
                        let a3423 = {
                            let a3417 = {
                                let a3415 = v3;
                                let a3416 = 2u8;
                                fixed_u8::bit_and(a3415, a3416)
                            };
                            let a3418 = 2u8;
                            if equal(a3417, a3418) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3424 = {
                            let a3421 = {
                                let a3419 = v3;
                                let a3420 = 1u8;
                                fixed_u8::bit_and(a3419, a3420)
                            };
                            let a3422 = 1u8;
                            if equal(a3421, a3422) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3423, a3424)?
                    };
                    nat_add(a3425, a3426)?
                };
                nat_add(a3427, a3428)?
            };
            if f419(v4) {
                let a3435 = f32({
                    let a3429 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3429)
                }, {
                    let a3430 = v4;
                    let a3431 = 65u64;
                    nat_sub(a3430, a3431)
                }, 1u64);
                let a3436 = f32(v1.clone(), 1u64, {
                    let a3433 = {
                        let a3432 = v1;
                        length_bytes(a3432)
                    };
                    let a3434 = 1u64;
                    nat_sub(a3433, a3434)
                });
                Ok(append_bytes(a3435, a3436))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f509(v0: Bytes) -> R<bool> {
    let m3439 = {
        let a3437 = v0;
        let a3438 = 0u64;
        index_bytes(a3437, a3438)
    };
    match m3439 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f418({
                let v2: u8 = v1;
                let a3484 = {
                    let a3460 = {
                        let a3448 = {
                            let a3442 = {
                                let a3440 = v2;
                                let a3441 = 128u8;
                                fixed_u8::bit_and(a3440, a3441)
                            };
                            let a3443 = 128u8;
                            if equal(a3442, a3443) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3449 = {
                            let a3446 = {
                                let a3444 = v2;
                                let a3445 = 64u8;
                                fixed_u8::bit_and(a3444, a3445)
                            };
                            let a3447 = 64u8;
                            if equal(a3446, a3447) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3448, a3449)?
                    };
                    let a3461 = {
                        let a3458 = {
                            let a3452 = {
                                let a3450 = v2;
                                let a3451 = 32u8;
                                fixed_u8::bit_and(a3450, a3451)
                            };
                            let a3453 = 32u8;
                            if equal(a3452, a3453) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3459 = {
                            let a3456 = {
                                let a3454 = v2;
                                let a3455 = 16u8;
                                fixed_u8::bit_and(a3454, a3455)
                            };
                            let a3457 = 16u8;
                            if equal(a3456, a3457) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3458, a3459)?
                    };
                    nat_add(a3460, a3461)?
                };
                let a3485 = {
                    let a3482 = {
                        let a3470 = {
                            let a3464 = {
                                let a3462 = v2;
                                let a3463 = 8u8;
                                fixed_u8::bit_and(a3462, a3463)
                            };
                            let a3465 = 8u8;
                            if equal(a3464, a3465) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3471 = {
                            let a3468 = {
                                let a3466 = v2;
                                let a3467 = 4u8;
                                fixed_u8::bit_and(a3466, a3467)
                            };
                            let a3469 = 4u8;
                            if equal(a3468, a3469) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3470, a3471)?
                    };
                    let a3483 = {
                        let a3480 = {
                            let a3474 = {
                                let a3472 = v2;
                                let a3473 = 2u8;
                                fixed_u8::bit_and(a3472, a3473)
                            };
                            let a3475 = 2u8;
                            if equal(a3474, a3475) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3481 = {
                            let a3478 = {
                                let a3476 = v2;
                                let a3477 = 1u8;
                                fixed_u8::bit_and(a3476, a3477)
                            };
                            let a3479 = 1u8;
                            if equal(a3478, a3479) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3480, a3481)?
                    };
                    nat_add(a3482, a3483)?
                };
                nat_add(a3484, a3485)?
            }) {
                Ok(true)
            } else {
                Ok(f419({
                    let v3: u8 = v1;
                    let a3530 = {
                        let a3506 = {
                            let a3494 = {
                                let a3488 = {
                                    let a3486 = v3;
                                    let a3487 = 128u8;
                                    fixed_u8::bit_and(a3486, a3487)
                                };
                                let a3489 = 128u8;
                                if equal(a3488, a3489) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a3495 = {
                                let a3492 = {
                                    let a3490 = v3;
                                    let a3491 = 64u8;
                                    fixed_u8::bit_and(a3490, a3491)
                                };
                                let a3493 = 64u8;
                                if equal(a3492, a3493) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3494, a3495)?
                        };
                        let a3507 = {
                            let a3504 = {
                                let a3498 = {
                                    let a3496 = v3;
                                    let a3497 = 32u8;
                                    fixed_u8::bit_and(a3496, a3497)
                                };
                                let a3499 = 32u8;
                                if equal(a3498, a3499) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a3505 = {
                                let a3502 = {
                                    let a3500 = v3;
                                    let a3501 = 16u8;
                                    fixed_u8::bit_and(a3500, a3501)
                                };
                                let a3503 = 16u8;
                                if equal(a3502, a3503) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3504, a3505)?
                        };
                        nat_add(a3506, a3507)?
                    };
                    let a3531 = {
                        let a3528 = {
                            let a3516 = {
                                let a3510 = {
                                    let a3508 = v3;
                                    let a3509 = 8u8;
                                    fixed_u8::bit_and(a3508, a3509)
                                };
                                let a3511 = 8u8;
                                if equal(a3510, a3511) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a3517 = {
                                let a3514 = {
                                    let a3512 = v3;
                                    let a3513 = 4u8;
                                    fixed_u8::bit_and(a3512, a3513)
                                };
                                let a3515 = 4u8;
                                if equal(a3514, a3515) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3516, a3517)?
                        };
                        let a3529 = {
                            let a3526 = {
                                let a3520 = {
                                    let a3518 = v3;
                                    let a3519 = 2u8;
                                    fixed_u8::bit_and(a3518, a3519)
                                };
                                let a3521 = 2u8;
                                if equal(a3520, a3521) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a3527 = {
                                let a3524 = {
                                    let a3522 = v3;
                                    let a3523 = 1u8;
                                    fixed_u8::bit_and(a3522, a3523)
                                };
                                let a3525 = 1u8;
                                if equal(a3524, a3525) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a3526, a3527)?
                        };
                        nat_add(a3528, a3529)?
                    };
                    nat_add(a3530, a3531)?
                }))
            }
        }
    }
}

pub fn f510(p3544: u64, v1: Bytes, p3545: u64, p3546: Bytes) -> R<Option<Bytes>> {
    let m3543 = (p3544, (p3545, p3546));
    {
        let mut m3543 = m3543;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3543;
                let m3532 = v0;
                if m3532 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3532 - 1;
                    let a3534 = {
                        let a3533 = v1.clone();
                        length_bytes(a3533)
                    };
                    let a3535 = v2;
                    if nat_le(a3534, a3535) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3536 = f451(v1.clone(), v2)?;
                        match m3536 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3538 = v2;
                                    let a3539 = {
                                        let (_, h3537) = v5;
                                        h3537
                                    };
                                    nat_add(a3538, a3539)?
                                }, {
                                    let a3541 = v3;
                                    let a3542 = f543({
                                        let (h3540, _) = v5;
                                        h3540
                                    });
                                    append_bytes(a3541, a3542)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h3547, _) = v1.clone();
        h3547
    };
    let v3: Bytes = {
        let (h3548, _) = {
            let (_, h3549) = v1.clone();
            h3549
        };
        h3548
    };
    let a3552 = {
        let (_, h3550) = {
            let (_, h3551) = v1;
            h3551
        };
        h3550
    };
    let a3553 = 0u64;
    if nat_eq(a3552, a3553) {
        let v4: u64 = f328(v0.clone(), v2)?;
        let a3554 = v4;
        let a3555 = 34u64;
        if nat_eq(a3554, a3555) {
            Ok(Some(({
                let a3556 = v2;
                let a3557 = 1u64;
                nat_add(a3556, a3557)?
            }, (v3, 1u64))))
        } else {
            let a3558 = v4;
            let a3559 = 92u64;
            if nat_eq(a3558, a3559) {
                f544(v0, v2, v3)
            } else {
                let a3560 = v4;
                let a3561 = 32u64;
                if if nat_lt(a3560, a3561) {
                    true
                } else {
                    let a3562 = v4;
                    let a3563 = 256u64;
                    nat_eq(a3562, a3563)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a3564 = v2;
                        let a3565 = 1u64;
                        nat_add(a3564, a3565)?
                    }, ({
                        let a3566 = v3;
                        let a3567 = f32(v0, v2, 1u64);
                        append_bytes(a3566, a3567)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f512(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f545(v0, v1)
}

pub fn f513(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f546(v0, v1)
}

pub fn f514(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3568 = v0.clone();
        length_bytes(a3568)
    };
    let a3569 = 0u64;
    let a3570 = v1;
    if nat_lt(a3569, a3570) {
        let a3571 = f547(v0.clone())?;
        let a3572 = v1;
        if nat_eq(a3571, a3572) {
            let a3573 = 1u64;
            let a3574 = v1;
            if nat_lt(a3573, a3574) {
                let m3577 = {
                    let a3575 = v0;
                    let a3576 = 0u64;
                    index_bytes(a3575, a3576)
                };
                match m3577 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3578 = v2;
                        let a3579 = 48u8;
                        Ok(!equal(a3578, a3579))
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

pub fn f515(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f548(v0, v1)
}

pub fn f516(v0: Bytes) -> R<u64> {
    let (h3580, _) = f269(Fn3::F549(v0.clone()), {
        let a3582 = {
            let a3581 = v0;
            length_bytes(a3581)
        };
        let a3583 = 1u64;
        nat_add(a3582, a3583)?
    }, 0u64)?;
    Ok(h3580)
}

pub fn f517(p3586: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m3585 = p3586;
    {
        let mut m3585 = m3585;
        loop {
            let step = {
                let v0 = m3585;
                let m3584 = v0;
                match m3584.uncons() {
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
                Ok(next) => { m3585 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f518(v0: Adt19, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3587 = v0;
    match m3587 {
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
        Adt19::C4(r3590) => {
            let v2 = (*r3590).clone();
            f518(v2, v1)
        }
        Adt19::C5(r3591) => {
            let v3 = (*r3591).clone();
            f518(v3, v1)
        }
        Adt19::C6 => {
            Ok(v1)
        }
        Adt19::C7(r3592, r3593) => {
            let v4 = (*r3592).clone();
            let v5 = (*r3593).clone();
            f518(v4, f518(v5, v1)?)
        }
        Adt19::C8(v6, r3594, r3595) => {
            let v7 = (*r3594).clone();
            let v8 = (*r3595).clone();
            let v9: List<Bytes> = f518(v7.clone(), f518(v8, v1)?)?;
            if f37(v6, f8(Str::lit("$ref"))) {
                let m3588 = f260(Some(v7));
                match m3588 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m3589 = f550(v10)?;
                        match m3589 {
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

pub fn f519(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3596 = f151(v1.clone(), f8(Str::lit("enum")));
    match m3596 {
        None => {
            let m3598 = f151(v1.clone(), f8(Str::lit("oneOf")));
            match m3598 {
                None => {
                    if f555(v1.clone(), v0.clone())? {
                        let m3599 = f556(v0.clone(), f422(f151(v1.clone(), f8(Str::lit("properties")))), f155(f156(f151(v1, f8(Str::lit("required"))))), true)?;
                        match m3599 {
                            Ok(v7) => {
                                Ok(f553(f411(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f219({
                                    let (h3600, _) = v7;
                                    h3600
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h3601) = v7;
                                    h3601
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt25>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m3602 = f422(f151(v1.clone(), f8(Str::lit("properties"))));
                        match m3602.uncons() {
                            None => {
                                let m3605 = f557(v1, {
                                    let a3603 = v0.clone();
                                    let a3604 = f8(Str::lit("Value"));
                                    append_bytes(a3603, a3604)
                                })?;
                                match m3605 {
                                    Ok(v9) => {
                                        Ok(f553(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h3606, _) = v9;
                                            h3606
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3607) = v9;
                                            h3607
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m3608 = f556(v0.clone(), f422(f151(v1.clone(), f8(Str::lit("properties")))), f155(f156(f151(v1, f8(Str::lit("required"))))), false)?;
                                match m3608 {
                                    Ok(v13) => {
                                        Ok(f553(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m3610 = {
                                                let (h3609, _) = v13.clone();
                                                h3609
                                            };
                                            match m3610.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f219({
                                                        let (h3611, _) = v13;
                                                        h3611
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3612) = v13;
                                            h3612
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
                    f554(v0, f261(v5))
                }
            }
        }
        Some(v2) => {
            let m3597 = f551(f261(v2), List::<Bytes>::nil())?;
            match m3597 {
                Ok(v4) => {
                    Ok(f553(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f219(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f552(v3, v0))
                }
            }
        }
    }
}

pub fn f520(v0: Adt19) -> Adt19 {
    let v2: Adt19 = {
        let m3613 = f151(v0.clone(), f8(Str::lit("anyOf")));
        match m3613 {
            None => {
                v0
            }
            Some(v1) => {
                if f558(f261(v1.clone())) {
                    f559(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f560(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f561(f261(v1.clone()))
                    } {
                        v0
                    } else {
                        f562(f559(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt19 = {
        let m3614 = f151(v2.clone(), f8(Str::lit("oneOf")));
        match m3614 {
            None => {
                v2
            }
            Some(v3) => {
                if f558(f261(v3)) {
                    f559(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m3615 = f151(v4.clone(), f8(Str::lit("allOf")));
    match m3615 {
        None => {
            v4
        }
        Some(v5) => {
            if f563(f261(v5)) {
                f559(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f521(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3660 = {
            let a3636 = {
                let a3624 = {
                    let a3618 = {
                        let a3616 = v3;
                        let a3617 = 128u8;
                        fixed_u8::bit_and(a3616, a3617)
                    };
                    let a3619 = 128u8;
                    if equal(a3618, a3619) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3625 = {
                    let a3622 = {
                        let a3620 = v3;
                        let a3621 = 64u8;
                        fixed_u8::bit_and(a3620, a3621)
                    };
                    let a3623 = 64u8;
                    if equal(a3622, a3623) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3624, a3625)?
            };
            let a3637 = {
                let a3634 = {
                    let a3628 = {
                        let a3626 = v3;
                        let a3627 = 32u8;
                        fixed_u8::bit_and(a3626, a3627)
                    };
                    let a3629 = 32u8;
                    if equal(a3628, a3629) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3635 = {
                    let a3632 = {
                        let a3630 = v3;
                        let a3631 = 16u8;
                        fixed_u8::bit_and(a3630, a3631)
                    };
                    let a3633 = 16u8;
                    if equal(a3632, a3633) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3634, a3635)?
            };
            nat_add(a3636, a3637)?
        };
        let a3661 = {
            let a3658 = {
                let a3646 = {
                    let a3640 = {
                        let a3638 = v3;
                        let a3639 = 8u8;
                        fixed_u8::bit_and(a3638, a3639)
                    };
                    let a3641 = 8u8;
                    if equal(a3640, a3641) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3647 = {
                    let a3644 = {
                        let a3642 = v3;
                        let a3643 = 4u8;
                        fixed_u8::bit_and(a3642, a3643)
                    };
                    let a3645 = 4u8;
                    if equal(a3644, a3645) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3646, a3647)?
            };
            let a3659 = {
                let a3656 = {
                    let a3650 = {
                        let a3648 = v3;
                        let a3649 = 2u8;
                        fixed_u8::bit_and(a3648, a3649)
                    };
                    let a3651 = 2u8;
                    if equal(a3650, a3651) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3657 = {
                    let a3654 = {
                        let a3652 = v3;
                        let a3653 = 1u8;
                        fixed_u8::bit_and(a3652, a3653)
                    };
                    let a3655 = 1u8;
                    if equal(a3654, a3655) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3656, a3657)?
            };
            nat_add(a3658, a3659)?
        };
        nat_add(a3660, a3661)?
    };
    let a3662 = v4;
    let a3663 = 34u64;
    if nat_eq(a3662, a3663) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3664 = v4;
        let a3665 = 92u64;
        if nat_eq(a3664, a3665) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3666 = v4;
            let a3667 = 8u64;
            if nat_eq(a3666, a3667) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3668 = v4;
                let a3669 = 12u64;
                if nat_eq(a3668, a3669) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3670 = v4;
                    let a3671 = 10u64;
                    if nat_eq(a3670, a3671) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3672 = v4;
                        let a3673 = 13u64;
                        if nat_eq(a3672, a3673) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3674 = v4;
                            let a3675 = 9u64;
                            if nat_eq(a3674, a3675) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3676 = v4;
                                let a3677 = 32u64;
                                if nat_lt(a3676, a3677) {
                                    let a3678 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3679 = f564(v2)?;
                                    Ok(append_bytes(a3678, a3679))
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

pub fn f522(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f565(Fn21::F566, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f523(v0: Fn17, p3683: List<List<(Bytes, Bytes)>>, p3684: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3682 = (p3683, p3684);
    {
        let mut m3682 = m3682;
        loop {
            let step = {
                let (v1, v2) = m3682;
                let m3680 = v2;
                match m3680.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3681: Fn17 = v0.clone();
                            c3681.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f525(v0: Fn18, p3689: u64, p3690: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3688 = (p3689, p3690);
    {
        let mut m3688 = m3688;
        loop {
            let step = {
                let (v1, v2) = m3688;
                let m3685 = v1;
                if m3685 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3685 - 1;
                    let m3687 = {
                        let c3686: Fn18 = v0.clone();
                        c3686.apply(v2.clone())?
                    };
                    match m3687 {
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
                Ok(next) => { m3688 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f567(v0)
}

pub fn f527(v0: Fn19, p3695: u64, p3696: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m3694 = (p3695, p3696);
    {
        let mut m3694 = m3694;
        loop {
            let step = {
                let (v1, v2) = m3694;
                let m3691 = v1;
                if m3691 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3691 - 1;
                    let m3693 = {
                        let c3692: Fn19 = v0.clone();
                        c3692.apply(v2.clone())?
                    };
                    match m3693 {
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
                Ok(next) => { m3694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f568(v0)
}

pub fn f529(v0: u64) -> u32 {
    let a3697 = v0;
    let a3698 = 32u64;
    if nat_lt(a3697, a3698) {
        let a3699 = v0;
        let a3700 = 16u64;
        if nat_lt(a3699, a3700) {
            let a3701 = v0;
            let a3702 = 8u64;
            if nat_lt(a3701, a3702) {
                let a3703 = v0;
                let a3704 = 4u64;
                if nat_lt(a3703, a3704) {
                    let a3705 = v0;
                    let a3706 = 2u64;
                    if nat_lt(a3705, a3706) {
                        let a3707 = v0;
                        let a3708 = 1u64;
                        if nat_lt(a3707, a3708) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3709 = v0;
                        let a3710 = 3u64;
                        if nat_lt(a3709, a3710) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3711 = v0;
                    let a3712 = 6u64;
                    if nat_lt(a3711, a3712) {
                        let a3713 = v0;
                        let a3714 = 5u64;
                        if nat_lt(a3713, a3714) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3715 = v0;
                        let a3716 = 7u64;
                        if nat_lt(a3715, a3716) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3717 = v0;
                let a3718 = 12u64;
                if nat_lt(a3717, a3718) {
                    let a3719 = v0;
                    let a3720 = 10u64;
                    if nat_lt(a3719, a3720) {
                        let a3721 = v0;
                        let a3722 = 9u64;
                        if nat_lt(a3721, a3722) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3723 = v0;
                        let a3724 = 11u64;
                        if nat_lt(a3723, a3724) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3725 = v0;
                    let a3726 = 14u64;
                    if nat_lt(a3725, a3726) {
                        let a3727 = v0;
                        let a3728 = 13u64;
                        if nat_lt(a3727, a3728) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3729 = v0;
                        let a3730 = 15u64;
                        if nat_lt(a3729, a3730) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3731 = v0;
            let a3732 = 24u64;
            if nat_lt(a3731, a3732) {
                let a3733 = v0;
                let a3734 = 20u64;
                if nat_lt(a3733, a3734) {
                    let a3735 = v0;
                    let a3736 = 18u64;
                    if nat_lt(a3735, a3736) {
                        let a3737 = v0;
                        let a3738 = 17u64;
                        if nat_lt(a3737, a3738) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3739 = v0;
                        let a3740 = 19u64;
                        if nat_lt(a3739, a3740) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3741 = v0;
                    let a3742 = 22u64;
                    if nat_lt(a3741, a3742) {
                        let a3743 = v0;
                        let a3744 = 21u64;
                        if nat_lt(a3743, a3744) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3745 = v0;
                        let a3746 = 23u64;
                        if nat_lt(a3745, a3746) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3747 = v0;
                let a3748 = 28u64;
                if nat_lt(a3747, a3748) {
                    let a3749 = v0;
                    let a3750 = 26u64;
                    if nat_lt(a3749, a3750) {
                        let a3751 = v0;
                        let a3752 = 25u64;
                        if nat_lt(a3751, a3752) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3753 = v0;
                        let a3754 = 27u64;
                        if nat_lt(a3753, a3754) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3755 = v0;
                    let a3756 = 30u64;
                    if nat_lt(a3755, a3756) {
                        let a3757 = v0;
                        let a3758 = 29u64;
                        if nat_lt(a3757, a3758) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3759 = v0;
                        let a3760 = 31u64;
                        if nat_lt(a3759, a3760) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3761 = v0;
        let a3762 = 48u64;
        if nat_lt(a3761, a3762) {
            let a3763 = v0;
            let a3764 = 40u64;
            if nat_lt(a3763, a3764) {
                let a3765 = v0;
                let a3766 = 36u64;
                if nat_lt(a3765, a3766) {
                    let a3767 = v0;
                    let a3768 = 34u64;
                    if nat_lt(a3767, a3768) {
                        let a3769 = v0;
                        let a3770 = 33u64;
                        if nat_lt(a3769, a3770) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3771 = v0;
                        let a3772 = 35u64;
                        if nat_lt(a3771, a3772) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3773 = v0;
                    let a3774 = 38u64;
                    if nat_lt(a3773, a3774) {
                        let a3775 = v0;
                        let a3776 = 37u64;
                        if nat_lt(a3775, a3776) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3777 = v0;
                        let a3778 = 39u64;
                        if nat_lt(a3777, a3778) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3779 = v0;
                let a3780 = 44u64;
                if nat_lt(a3779, a3780) {
                    let a3781 = v0;
                    let a3782 = 42u64;
                    if nat_lt(a3781, a3782) {
                        let a3783 = v0;
                        let a3784 = 41u64;
                        if nat_lt(a3783, a3784) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3785 = v0;
                        let a3786 = 43u64;
                        if nat_lt(a3785, a3786) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3787 = v0;
                    let a3788 = 46u64;
                    if nat_lt(a3787, a3788) {
                        let a3789 = v0;
                        let a3790 = 45u64;
                        if nat_lt(a3789, a3790) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3791 = v0;
                        let a3792 = 47u64;
                        if nat_lt(a3791, a3792) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3793 = v0;
            let a3794 = 56u64;
            if nat_lt(a3793, a3794) {
                let a3795 = v0;
                let a3796 = 52u64;
                if nat_lt(a3795, a3796) {
                    let a3797 = v0;
                    let a3798 = 50u64;
                    if nat_lt(a3797, a3798) {
                        let a3799 = v0;
                        let a3800 = 49u64;
                        if nat_lt(a3799, a3800) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3801 = v0;
                        let a3802 = 51u64;
                        if nat_lt(a3801, a3802) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3803 = v0;
                    let a3804 = 54u64;
                    if nat_lt(a3803, a3804) {
                        let a3805 = v0;
                        let a3806 = 53u64;
                        if nat_lt(a3805, a3806) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3807 = v0;
                        let a3808 = 55u64;
                        if nat_lt(a3807, a3808) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3809 = v0;
                let a3810 = 60u64;
                if nat_lt(a3809, a3810) {
                    let a3811 = v0;
                    let a3812 = 58u64;
                    if nat_lt(a3811, a3812) {
                        let a3813 = v0;
                        let a3814 = 57u64;
                        if nat_lt(a3813, a3814) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3815 = v0;
                        let a3816 = 59u64;
                        if nat_lt(a3815, a3816) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3817 = v0;
                    let a3818 = 62u64;
                    if nat_lt(a3817, a3818) {
                        let a3819 = v0;
                        let a3820 = 61u64;
                        if nat_lt(a3819, a3820) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3821 = v0;
                        let a3822 = 63u64;
                        if nat_lt(a3821, a3822) {
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

pub fn f530(v0: u32) -> u32 {
    let a3825 = {
        let a3823 = f569(v0, 6u32, 26u32);
        let a3824 = f569(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3823, a3824)
    };
    let a3826 = f569(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3825, a3826)
}

pub fn f531(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3832 = {
        let a3827 = v0;
        let a3828 = v1;
        fixed_u32::bit_and(a3827, a3828)
    };
    let a3833 = {
        let a3830 = {
            let a3829 = v0;
            fixed_u32::bit_not(a3829)
        };
        let a3831 = v2;
        fixed_u32::bit_and(a3830, a3831)
    };
    fixed_u32::bit_xor(a3832, a3833)
}

pub fn f532(v0: u32) -> u32 {
    let a3836 = {
        let a3834 = f569(v0, 2u32, 30u32);
        let a3835 = f569(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3834, a3835)
    };
    let a3837 = f569(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3836, a3837)
}

pub fn f533(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3846 = {
        let a3842 = {
            let a3838 = v0;
            let a3839 = v1;
            fixed_u32::bit_and(a3838, a3839)
        };
        let a3843 = {
            let a3840 = v0;
            let a3841 = v2;
            fixed_u32::bit_and(a3840, a3841)
        };
        fixed_u32::bit_xor(a3842, a3843)
    };
    let a3847 = {
        let a3844 = v1;
        let a3845 = v2;
        fixed_u32::bit_and(a3844, a3845)
    };
    fixed_u32::bit_xor(a3846, a3847)
}

pub fn f534(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3848, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3848
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3849, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3849
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3850, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3850
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3851, _, _, _, _, _, _, _, _, _, _, _) => {
            h3851
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3852, _, _, _, _, _, _, _, _, _, _) => {
            h3852
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3853, _, _, _, _, _, _, _, _, _) => {
            h3853
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3854, _, _, _, _, _, _, _, _) => {
            h3854
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3855, _, _, _, _, _, _, _) => {
            h3855
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3856, _, _, _, _, _, _) => {
            h3856
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3857, _, _, _, _, _) => {
            h3857
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3858, _, _, _, _) => {
            h3858
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3859, _, _, _) => {
            h3859
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3860, _, _) => {
            h3860
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3861, _) => {
            h3861
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3862) => {
            h3862
        }
    }, f448(f570(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3863, _) => {
            h3863
        }
    }), f448(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3864, _, _, _, _, _, _) => {
            h3864
        }
    }, f448(f571(match v0.clone() {
        Adt26::C0(_, h3865, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3865
        }
    }), match v0 {
        Adt26::C0(h3866, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3866
        }
    }))))
}

pub fn f535(v0: Fn20, p3870: u64, p3871: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3869 = (p3870, p3871);
    {
        let mut m3869 = m3869;
        loop {
            let step = {
                let (v1, v2) = m3869;
                let m3867 = v1;
                if m3867 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3867 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3868: Fn20 = v0.clone();
                        c3868.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f536(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3872, _) = v0;
        h3872
    };
    let v2: u32 = {
        let (h3873, _) = {
            let (_, h3874) = v0;
            h3874
        };
        h3873
    };
    let v3: u32 = {
        let (_, h3875) = {
            let (_, h3876) = v0;
            h3876
        };
        h3875
    };
    ({
        let a3877 = v1;
        let a3878 = 2u64;
        let a3879 = 0u64;
        nat_quot(a3877, a3878, a3879)
    }, (f537(v2, 1u32), {
        let a3883 = {
            let a3880 = v1;
            let a3881 = 2u64;
            let a3882 = 0u64;
            nat_rem(a3880, a3881, a3882)
        };
        let a3884 = 1u64;
        if nat_eq(a3883, a3884) {
            let a3885 = v3;
            let a3886 = v2;
            fixed_u32::bit_or(a3885, a3886)
        } else {
            v3
        }
    }))
}

pub fn f537(v0: u32, v1: u32) -> u32 {
    let m3889 = {
        let a3887 = v0;
        let a3888 = v1;
        fixed_u32::shift_left(a3887, a3888)
    };
    match m3889 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f538(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f231(v0, {
        let (h3890, _) = v2;
        h3890
    })?;
    let a3892 = 1114111u64;
    let a3893 = {
        let (_, h3891) = v2;
        h3891
    };
    if nat_lt(a3892, a3893) {
        Ok(None::<(u64, u64)>)
    } else {
        let a3894 = 48u64;
        let a3895 = v3;
        if if nat_le(a3894, a3895) {
            let a3896 = v3;
            let a3897 = 57u64;
            nat_le(a3896, a3897)
        } else {
            false
        } {
            Ok(Some(({
                let a3899 = {
                    let (h3898, _) = v2;
                    h3898
                };
                let a3900 = 1u64;
                nat_add(a3899, a3900)?
            }, {
                let a3906 = {
                    let a3902 = {
                        let (_, h3901) = v2;
                        h3901
                    };
                    let a3903 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3902, a3903)?
                };
                let a3907 = {
                    let a3904 = v3;
                    let a3905 = 48u64;
                    nat_sub(a3904, a3905)
                };
                nat_add(a3906, a3907)?
            })))
        } else {
            if if v1 {
                let a3908 = 97u64;
                let a3909 = v3;
                if nat_le(a3908, a3909) {
                    let a3910 = v3;
                    let a3911 = 102u64;
                    nat_le(a3910, a3911)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a3913 = {
                        let (h3912, _) = v2;
                        h3912
                    };
                    let a3914 = 1u64;
                    nat_add(a3913, a3914)?
                }, {
                    let a3920 = {
                        let a3916 = {
                            let (_, h3915) = v2;
                            h3915
                        };
                        let a3917 = 16u64;
                        nat_mul(a3916, a3917)?
                    };
                    let a3921 = {
                        let a3918 = v3;
                        let a3919 = 87u64;
                        nat_sub(a3918, a3919)
                    };
                    nat_add(a3920, a3921)?
                })))
            } else {
                if if v1 {
                    let a3922 = 65u64;
                    let a3923 = v3;
                    if nat_le(a3922, a3923) {
                        let a3924 = v3;
                        let a3925 = 70u64;
                        nat_le(a3924, a3925)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a3927 = {
                            let (h3926, _) = v2;
                            h3926
                        };
                        let a3928 = 1u64;
                        nat_add(a3927, a3928)?
                    }, {
                        let a3934 = {
                            let a3930 = {
                                let (_, h3929) = v2;
                                h3929
                            };
                            let a3931 = 16u64;
                            nat_mul(a3930, a3931)?
                        };
                        let a3935 = {
                            let a3932 = v3;
                            let a3933 = 55u64;
                            nat_sub(a3932, a3933)
                        };
                        nat_add(a3934, a3935)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f539(p3942: List<Bytes>, p3943: u64) -> Bytes {
    let m3941 = (p3942, p3943);
    {
        let mut m3941 = m3941;
        loop {
            let step = {
                let (v0, v1) = m3941;
                let m3936 = v0;
                match m3936.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a3937 = v1;
                        let a3938 = 0u64;
                        if nat_eq(a3937, a3938) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a3939 = v1;
                                let a3940 = 1u64;
                                nat_sub(a3939, a3940)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: u64) -> List<Bytes> {
    let a3944 = v0;
    let a3945 = 0u64;
    if nat_eq(a3944, a3945) {
        f572()
    } else {
        let a3946 = v0;
        let a3947 = 1u64;
        if nat_eq(a3946, a3947) {
            f573()
        } else {
            let a3948 = v0;
            let a3949 = 2u64;
            if nat_eq(a3948, a3949) {
                f574()
            } else {
                let a3950 = v0;
                let a3951 = 3u64;
                if nat_eq(a3950, a3951) {
                    f575()
                } else {
                    let a3952 = v0;
                    let a3953 = 4u64;
                    if nat_eq(a3952, a3953) {
                        f576()
                    } else {
                        let a3954 = v0;
                        let a3955 = 5u64;
                        if nat_eq(a3954, a3955) {
                            f577()
                        } else {
                            let a3956 = v0;
                            let a3957 = 6u64;
                            if nat_eq(a3956, a3957) {
                                f578()
                            } else {
                                let a3958 = v0;
                                let a3959 = 7u64;
                                if nat_eq(a3958, a3959) {
                                    f579()
                                } else {
                                    let a3960 = v0;
                                    let a3961 = 8u64;
                                    if nat_eq(a3960, a3961) {
                                        f580()
                                    } else {
                                        let a3962 = v0;
                                        let a3963 = 9u64;
                                        if nat_eq(a3962, a3963) {
                                            f581()
                                        } else {
                                            let a3964 = v0;
                                            let a3965 = 10u64;
                                            if nat_eq(a3964, a3965) {
                                                f582()
                                            } else {
                                                let a3966 = v0;
                                                let a3967 = 11u64;
                                                if nat_eq(a3966, a3967) {
                                                    f583()
                                                } else {
                                                    let a3968 = v0;
                                                    let a3969 = 12u64;
                                                    if nat_eq(a3968, a3969) {
                                                        f584()
                                                    } else {
                                                        let a3970 = v0;
                                                        let a3971 = 13u64;
                                                        if nat_eq(a3970, a3971) {
                                                            f585()
                                                        } else {
                                                            let a3972 = v0;
                                                            let a3973 = 14u64;
                                                            if nat_eq(a3972, a3973) {
                                                                f586()
                                                            } else {
                                                                f587()
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

pub fn f541(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a3974 = v0;
        let a3975 = v2;
        if if if nat_eq(a3974, a3975) {
            true
        } else {
            let a3976 = v0;
            let a3977 = 60u64;
            nat_eq(a3976, a3977)
        } {
            true
        } else {
            let a3978 = v0;
            let a3979 = 38u64;
            if nat_eq(a3978, a3979) {
                true
            } else {
                let a3980 = v0;
                let a3981 = 256u64;
                nat_eq(a3980, a3981)
            }
        } {
            true
        } else {
            let a3982 = v0;
            let a3983 = 9u64;
            if nat_eq(a3982, a3983) {
                true
            } else {
                let a3984 = v0;
                let a3985 = 10u64;
                nat_eq(a3984, a3985)
            }
        }
    } else {
        let a3986 = v0;
        let a3987 = 60u64;
        if if nat_eq(a3986, a3987) {
            true
        } else {
            let a3988 = v0;
            let a3989 = 38u64;
            nat_eq(a3988, a3989)
        } {
            true
        } else {
            let a3990 = v0;
            let a3991 = 93u64;
            if nat_eq(a3990, a3991) {
                true
            } else {
                let a3992 = v0;
                let a3993 = 256u64;
                nat_eq(a3992, a3993)
            }
        }
    }
}

pub fn f542(p4006: u64, v1: Bytes, p4007: u64, p4008: Bytes) -> R<Option<Bytes>> {
    let m4005 = (p4006, (p4007, p4008));
    {
        let mut m4005 = m4005;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4005;
                let m3994 = v0;
                if m3994 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3994 - 1;
                    let a3996 = {
                        let a3995 = v1.clone();
                        length_bytes(a3995)
                    };
                    let a3997 = v2;
                    if nat_le(a3996, a3997) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3998 = f451(v1.clone(), v2)?;
                        match m3998 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4000 = v2;
                                    let a4001 = {
                                        let (_, h3999) = v5;
                                        h3999
                                    };
                                    nat_add(a4000, a4001)?
                                }, {
                                    let a4003 = v3;
                                    let a4004 = f588(v1.clone(), v2, {
                                        let (h4002, _) = v5;
                                        h4002
                                    });
                                    append_bytes(a4003, a4004)
                                })))
                            }
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

pub fn f543(v0: u64) -> Bytes {
    let a4009 = f8(Str::lit("u"));
    let a4010 = f589(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4009, a4010)
}

pub fn f544(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f328(v0.clone(), {
        let a4011 = v1;
        let a4012 = 1u64;
        nat_add(a4011, a4012)?
    })?;
    let v4: Bytes = {
        let a4013 = v3;
        let a4014 = 117u64;
        if nat_eq(a4013, a4014) {
            f590(v0, v1)?
        } else {
            f591(v3)
        }
    };
    let a4015 = v4.clone();
    let a4016 = Bytes::lit(&[]);
    if equal(a4015, a4016) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a4017 = v3;
            let a4018 = 117u64;
            if nat_eq(a4017, a4018) {
                let a4019 = v1;
                let a4020 = 6u64;
                nat_add(a4019, a4020)?
            } else {
                let a4021 = v1;
                let a4022 = 2u64;
                nat_add(a4021, a4022)?
            }
        }, ({
            let a4023 = v2;
            let a4024 = v4;
            append_bytes(a4023, a4024)
        }, 0u64))))
    }
}

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f328(v0, v1)?;
    let a4025 = 47u64;
    let a4026 = v2;
    if if nat_lt(a4025, a4026) {
        let a4027 = v2;
        let a4028 = 58u64;
        nat_lt(a4027, a4028)
    } else {
        false
    } {
        Ok(Some({
            let a4029 = v1;
            let a4030 = 1u64;
            nat_add(a4029, a4030)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f546(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a4031 = f328(v0, v1)?;
    let a4032 = 48u64;
    if nat_eq(a4031, a4032) {
        Ok(Some({
            let a4033 = v1;
            let a4034 = 1u64;
            nat_add(a4033, a4034)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f547(v0: Bytes) -> R<u64> {
    let (h4035, _) = f269(Fn3::F592(v0.clone()), {
        let a4036 = v0;
        length_bytes(a4036)
    }, 0u64)?;
    Ok(h4035)
}

pub fn f548(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4040 = {
        let a4038 = v0;
        let a4039 = {
            let (h4037, _) = v1;
            h4037
        };
        index_bytes(a4038, a4039)
    };
    match m4040 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a4042 = {
                    let (h4041, _) = v1;
                    h4041
                };
                let a4043 = 1u64;
                nat_add(a4042, a4043)?
            }, {
                let a4047 = {
                    let a4045 = {
                        let (_, h4044) = v1;
                        h4044
                    };
                    let a4046 = 10u64;
                    nat_mul(a4045, a4046)?
                };
                let a4048 = f593(v2)?;
                nat_add(a4047, a4048)?
            })))
        }
    }
}

pub fn f549(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f594(v0, v1)
}

pub fn f550(v0: Bytes) -> R<Option<Bytes>> {
    if f209(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f256(f32(v0.clone(), 8u64, {
            let a4050 = {
                let a4049 = v0;
                length_bytes(a4049)
            };
            let a4051 = 8u64;
            nat_sub(a4050, a4051)
        }))?))
    } else {
        if f209(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f256(f32(v0.clone(), 14u64, {
                let a4053 = {
                    let a4052 = v0;
                    length_bytes(a4052)
                };
                let a4054 = 14u64;
                nat_sub(a4053, a4054)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f551(v0: List<Adt19>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f409(f410(f155(v0.clone()))?, v0, v1)
}

pub fn f552(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt25>), Bytes> {
    Err::<(Bytes, List<Adt25>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f553(v0: Bytes, v1: List<Adt25>) -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((v0, v1))
}

pub fn f554(v0: Bytes, v1: List<Adt19>) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4055 = v1.clone();
    match m4055.uncons() {
        None => {
            Ok(f552(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m4056 = f595(f422(f151(v2, f8(Str::lit("properties")))), v1.clone());
            match m4056.uncons() {
                None => {
                    let m4057 = f596(f410(f597(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m4057 {
                        Ok(v5) => {
                            Ok(f553(f411(v0, {
                                let (h4058, _) = v5;
                                h4058
                            }), {
                                let (_, h4059) = v5;
                                h4059
                            }))
                        }
                        Err(v4) => {
                            Ok(f552(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m4060 = f598(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m4060 {
                        Ok(v9) => {
                            Ok(f553(f411(v0, {
                                let (h4061, _) = v9;
                                h4061
                            }), {
                                let (_, h4062) = v9;
                                h4062
                            }))
                        }
                        Err(v8) => {
                            Ok(f552(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f555(v0: Adt19, v1: Bytes) -> R<bool> {
    let m4063 = v0;
    match m4063 {
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
        Adt19::C4(r4064) => {
            let v2 = (*r4064).clone();
            f555(v2, v1)
        }
        Adt19::C5(r4065) => {
            let v3 = (*r4065).clone();
            f555(v3, v1)
        }
        Adt19::C6 => {
            Ok(false)
        }
        Adt19::C7(r4066, r4067) => {
            let v4 = (*r4066).clone();
            let v5 = (*r4067).clone();
            if f555(v4, v1.clone())? {
                Ok(true)
            } else {
                f555(v5, v1)
            }
        }
        Adt19::C8(v6, r4068, r4069) => {
            let v7 = (*r4068).clone();
            let v8 = (*r4069).clone();
            if if f37(v6, f8(Str::lit("$ref"))) {
                f599(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f555(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f555(v8, v1)
                }
            }
        }
    }
}

pub fn f556(v0: Bytes, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f600(f410(f601(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f557(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4070 = f602(v0.clone());
    match m4070 {
        None => {
            if f37(f154(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4071 = f151(v0, f8(Str::lit("items")));
                match m4071 {
                    None => {
                        Ok(f603(f8(Str::lit("List")), f605()))
                    }
                    Some(v3) => {
                        Ok(f603(f8(Str::lit("List")), f604(v3, {
                            let a4072 = v1;
                            let a4073 = f8(Str::lit("Item"));
                            append_bytes(a4072, a4073)
                        })?))
                    }
                }
            } else {
                f604(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f603(f8(Str::lit("Option")), f604(v2, v1)?))
        }
    }
}

pub fn f558(p4076: List<Adt19>) -> bool {
    let m4075 = p4076;
    {
        let mut m4075 = m4075;
        loop {
            let step = {
                let v0 = m4075;
                let m4074 = v0;
                match m4074.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f606(f472(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt19>, bool>(v2)
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(v0: Adt19, v1: Bytes) -> Adt19 {
    let m4077 = v0;
    match m4077 {
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
        Adt19::C4(r4078) => {
            let v13 = (*r4078).clone();
            Adt19::C4(std::rc::Rc::new(v13))
        }
        Adt19::C5(r4079) => {
            let v5 = (*r4079).clone();
            Adt19::C5(std::rc::Rc::new(f559(v5, v1)))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r4080, r4081) => {
            let v14 = (*r4080).clone();
            let v15 = (*r4081).clone();
            Adt19::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt19::C8(v2, r4082, r4083) => {
            let v3 = (*r4082).clone();
            let v4 = (*r4083).clone();
            if f37(v2.clone(), v1.clone()) {
                f559(v4, v1)
            } else {
                Adt19::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f559(v4, v1)))
            }
        }
    }
}

pub fn f560(v0: Adt19, v1: Bytes) -> bool {
    let m4084 = f151(v0, v1);
    match m4084 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f561(v0: List<Adt19>) -> bool {
    let m4085 = v0.clone();
    match m4085.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m4086 = f595(f422(f151(v1, f8(Str::lit("properties")))), v0.clone());
            match m4086.uncons() {
                None => {
                    f608(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f607(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f562(v0: Adt19, v1: Bytes, v2: Adt19) -> Adt19 {
    let m4087 = v0;
    match m4087 {
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
        Adt19::C4(r4088) => {
            let v14 = (*r4088).clone();
            Adt19::C4(std::rc::Rc::new(v14))
        }
        Adt19::C5(r4089) => {
            let v3 = (*r4089).clone();
            Adt19::C5(std::rc::Rc::new(Adt19::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r4090, r4091) => {
            let v15 = (*r4090).clone();
            let v16 = (*r4091).clone();
            Adt19::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt19::C8(v4, r4092, r4093) => {
            let v5 = (*r4092).clone();
            let v6 = (*r4093).clone();
            Adt19::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f563(p4097: List<Adt19>) -> bool {
    let m4096 = p4097;
    {
        let mut m4096 = m4096;
        loop {
            let step = {
                let v0 = m4096;
                let m4094 = v0;
                match m4094.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f606(f472(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f558(f261({
                                let m4095 = f151(v1, f8(Str::lit("anyOf")));
                                match m4095 {
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
                Ok(next) => { m4096 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f564(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4142 = {
            let a4118 = {
                let a4106 = {
                    let a4100 = {
                        let a4098 = v1;
                        let a4099 = 128u8;
                        fixed_u8::bit_and(a4098, a4099)
                    };
                    let a4101 = 128u8;
                    if equal(a4100, a4101) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4107 = {
                    let a4104 = {
                        let a4102 = v1;
                        let a4103 = 64u8;
                        fixed_u8::bit_and(a4102, a4103)
                    };
                    let a4105 = 64u8;
                    if equal(a4104, a4105) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4106, a4107)?
            };
            let a4119 = {
                let a4116 = {
                    let a4110 = {
                        let a4108 = v1;
                        let a4109 = 32u8;
                        fixed_u8::bit_and(a4108, a4109)
                    };
                    let a4111 = 32u8;
                    if equal(a4110, a4111) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4117 = {
                    let a4114 = {
                        let a4112 = v1;
                        let a4113 = 16u8;
                        fixed_u8::bit_and(a4112, a4113)
                    };
                    let a4115 = 16u8;
                    if equal(a4114, a4115) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4116, a4117)?
            };
            nat_add(a4118, a4119)?
        };
        let a4143 = {
            let a4140 = {
                let a4128 = {
                    let a4122 = {
                        let a4120 = v1;
                        let a4121 = 8u8;
                        fixed_u8::bit_and(a4120, a4121)
                    };
                    let a4123 = 8u8;
                    if equal(a4122, a4123) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4129 = {
                    let a4126 = {
                        let a4124 = v1;
                        let a4125 = 4u8;
                        fixed_u8::bit_and(a4124, a4125)
                    };
                    let a4127 = 4u8;
                    if equal(a4126, a4127) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4128, a4129)?
            };
            let a4141 = {
                let a4138 = {
                    let a4132 = {
                        let a4130 = v1;
                        let a4131 = 2u8;
                        fixed_u8::bit_and(a4130, a4131)
                    };
                    let a4133 = 2u8;
                    if equal(a4132, a4133) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4139 = {
                    let a4136 = {
                        let a4134 = v1;
                        let a4135 = 1u8;
                        fixed_u8::bit_and(a4134, a4135)
                    };
                    let a4137 = 1u8;
                    if equal(a4136, a4137) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4138, a4139)?
            };
            nat_add(a4140, a4141)?
        };
        nat_add(a4142, a4143)?
    };
    let a4150 = f332(f609(), {
        let a4144 = v2;
        let a4145 = 16u64;
        let a4146 = 0u64;
        nat_quot(a4144, a4145, a4146)
    });
    let a4151 = f332(f609(), {
        let a4147 = v2;
        let a4148 = 16u64;
        let a4149 = 0u64;
        nat_rem(a4147, a4148, a4149)
    });
    Ok(append_bytes(a4150, a4151))
}

pub fn f565(v0: Fn21, p4155: List<List<(Bytes, Bytes)>>, p4156: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4154 = (p4155, p4156);
    {
        let mut m4154 = m4154;
        loop {
            let step = {
                let (v1, v2) = m4154;
                let m4152 = v2;
                match m4152.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4153: Fn21 = v0.clone();
                            c4153.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f566(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f567(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4158 = {
        let a4157 = v0.clone();
        length_list(a4157)
    };
    let a4159 = 1u64;
    if nat_le(a4158, a4159) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f610(v0)?))
    }
}

pub fn f568(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h4160, _) = v0;
        h4160
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h4161) = v0;
        h4161
    };
    let m4162 = v1;
    match m4162.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m4163 = v4;
            match m4163.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f611(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f569(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4164 = f387(v0, v1);
    let a4165 = f537(v0, v2);
    fixed_u32::bit_or(a4164, a4165)
}

pub fn f570(v0: u32) -> u32 {
    let a4168 = {
        let a4166 = f569(v0, 17u32, 15u32);
        let a4167 = f569(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4166, a4167)
    };
    let a4169 = f387(v0, 10u32);
    fixed_u32::bit_xor(a4168, a4169)
}

pub fn f571(v0: u32) -> u32 {
    let a4172 = {
        let a4170 = f569(v0, 7u32, 25u32);
        let a4171 = f569(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4170, a4171)
    };
    let a4173 = f387(v0, 3u32);
    fixed_u32::bit_xor(a4172, a4173)
}

pub fn f572() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f573() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f574() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f575() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f576() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f577() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f578() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f579() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f580() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f581() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f582() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f583() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f584() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f585() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f586() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f587() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f588(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f418(v2) {
        true
    } else {
        f419(v2)
    } {
        let a4174 = v2;
        let a4175 = 117u64;
        if !nat_eq(a4174, a4175) {
            let a4176 = v2;
            let a4177 = 85u64;
            !nat_eq(a4176, a4177)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f420(v2) {
            let a4178 = 0u64;
            let a4179 = v1;
            nat_lt(a4178, a4179)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a4180 = v2;
        let a4181 = 65536u64;
        if nat_lt(a4180, a4181) {
            let a4182 = f8(Str::lit("u"));
            let a4183 = f589(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4182, a4183)
        } else {
            let a4184 = f8(Str::lit("U"));
            let a4185 = f589(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4184, a4185)
        }
    }
}

pub fn f589(p4202: u64, p4203: u64, p4204: u64, p4205: Bytes) -> Bytes {
    let m4201 = (p4202, (p4203, (p4204, p4205)));
    {
        let mut m4201 = m4201;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4201;
                let m4186 = v0;
                if m4186 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4186 - 1;
                    let a4187 = v1;
                    let a4188 = 0u64;
                    if if nat_eq(a4187, a4188) {
                        let a4189 = v2;
                        let a4190 = 0u64;
                        nat_eq(a4189, a4190)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4191 = v1;
                            let a4192 = 16u64;
                            let a4193 = 0u64;
                            nat_quot(a4191, a4192, a4193)
                        }, ({
                            let a4194 = v2;
                            let a4195 = 1u64;
                            nat_sub(a4194, a4195)
                        }, {
                            let a4199 = f612({
                                let a4196 = v1;
                                let a4197 = 16u64;
                                let a4198 = 0u64;
                                nat_rem(a4196, a4197, a4198)
                            });
                            let a4200 = v3;
                            append_bytes(a4199, a4200)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: Bytes, v1: u64) -> R<Bytes> {
    let a4208 = f32(v0.clone(), {
        let a4206 = v1;
        let a4207 = 2u64;
        nat_add(a4206, a4207)?
    }, 2u64);
    let a4209 = Bytes::lit(&[0x30, 0x30]);
    if equal(a4208, a4209) {
        let m4212 = f613(f32(v0, {
            let a4210 = v1;
            let a4211 = 4u64;
            nat_add(a4210, a4211)?
        }, 2u64))?;
        match m4212 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f328(v2.clone(), 0u64)?;
                let a4213 = v3;
                let a4214 = 32u64;
                if nat_lt(a4213, a4214) {
                    if f614(v3) {
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

pub fn f591(v0: u64) -> Bytes {
    let a4215 = v0;
    let a4216 = 34u64;
    if nat_eq(a4215, a4216) {
        Bytes::lit(&[0x22])
    } else {
        let a4217 = v0;
        let a4218 = 92u64;
        if nat_eq(a4217, a4218) {
            Bytes::lit(&[0x5c])
        } else {
            let a4219 = v0;
            let a4220 = 98u64;
            if nat_eq(a4219, a4220) {
                Bytes::lit(&[0x08])
            } else {
                let a4221 = v0;
                let a4222 = 102u64;
                if nat_eq(a4221, a4222) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a4223 = v0;
                    let a4224 = 110u64;
                    if nat_eq(a4223, a4224) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a4225 = v0;
                        let a4226 = 114u64;
                        if nat_eq(a4225, a4226) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a4227 = v0;
                            let a4228 = 116u64;
                            if nat_eq(a4227, a4228) {
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

pub fn f592(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m4231 = {
        let a4229 = v0;
        let a4230 = v1;
        index_bytes(a4229, a4230)
    };
    match m4231 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f615(v2)? {
                Ok(Some({
                    let a4232 = v1;
                    let a4233 = 1u64;
                    nat_add(a4232, a4233)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f593(v0: u8) -> R<u64> {
    let a4280 = {
        let v1: u8 = v0;
        let a4278 = {
            let a4254 = {
                let a4242 = {
                    let a4236 = {
                        let a4234 = v1;
                        let a4235 = 128u8;
                        fixed_u8::bit_and(a4234, a4235)
                    };
                    let a4237 = 128u8;
                    if equal(a4236, a4237) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4243 = {
                    let a4240 = {
                        let a4238 = v1;
                        let a4239 = 64u8;
                        fixed_u8::bit_and(a4238, a4239)
                    };
                    let a4241 = 64u8;
                    if equal(a4240, a4241) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4242, a4243)?
            };
            let a4255 = {
                let a4252 = {
                    let a4246 = {
                        let a4244 = v1;
                        let a4245 = 32u8;
                        fixed_u8::bit_and(a4244, a4245)
                    };
                    let a4247 = 32u8;
                    if equal(a4246, a4247) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4253 = {
                    let a4250 = {
                        let a4248 = v1;
                        let a4249 = 16u8;
                        fixed_u8::bit_and(a4248, a4249)
                    };
                    let a4251 = 16u8;
                    if equal(a4250, a4251) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4252, a4253)?
            };
            nat_add(a4254, a4255)?
        };
        let a4279 = {
            let a4276 = {
                let a4264 = {
                    let a4258 = {
                        let a4256 = v1;
                        let a4257 = 8u8;
                        fixed_u8::bit_and(a4256, a4257)
                    };
                    let a4259 = 8u8;
                    if equal(a4258, a4259) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4265 = {
                    let a4262 = {
                        let a4260 = v1;
                        let a4261 = 4u8;
                        fixed_u8::bit_and(a4260, a4261)
                    };
                    let a4263 = 4u8;
                    if equal(a4262, a4263) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4264, a4265)?
            };
            let a4277 = {
                let a4274 = {
                    let a4268 = {
                        let a4266 = v1;
                        let a4267 = 2u8;
                        fixed_u8::bit_and(a4266, a4267)
                    };
                    let a4269 = 2u8;
                    if equal(a4268, a4269) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4275 = {
                    let a4272 = {
                        let a4270 = v1;
                        let a4271 = 1u8;
                        fixed_u8::bit_and(a4270, a4271)
                    };
                    let a4273 = 1u8;
                    if equal(a4272, a4273) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4274, a4275)?
            };
            nat_add(a4276, a4277)?
        };
        nat_add(a4278, a4279)?
    };
    let a4281 = 48u64;
    Ok(nat_sub(a4280, a4281))
}

pub fn f594(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a4282 = v0.clone();
        length_bytes(a4282)
    };
    let a4283 = v1;
    let a4284 = v2;
    if if nat_lt(a4283, a4284) {
        let a4289 = f328(v0, {
            let a4287 = v2;
            let a4288 = {
                let a4285 = v1;
                let a4286 = 1u64;
                nat_add(a4285, a4286)?
            };
            nat_sub(a4287, a4288)
        })?;
        let a4290 = 48u64;
        nat_eq(a4289, a4290)
    } else {
        false
    } {
        Ok(Some({
            let a4291 = v1;
            let a4292 = 1u64;
            nat_add(a4291, a4292)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f595(v0: List<(Bytes, Adt19)>, v1: List<Adt19>) -> List<Bytes> {
    let m4293 = v0;
    match m4293.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f616(v1.clone(), {
                let (h4294, _) = v2.clone();
                h4294
            }) {
                List::cons({
                    let (h4295, _) = v2;
                    h4295
                }, f595(v3, v1))
            } else {
                f595(v3, v1)
            }
        }
    }
}

pub fn f596(v0: bool, v1: Bytes, v2: List<Adt19>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4296 = v2;
    match m4296.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m4297 = f617(v4.clone())?;
            match m4297 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m4298 = f505(v0, v6.clone())?;
                    match m4298 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f88(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4301 = f618(v4, {
                                    let a4299 = v1.clone();
                                    let a4300 = f256(v6)?;
                                    append_bytes(a4299, a4300)
                                })?;
                                match m4301 {
                                    Ok(v9) => {
                                        let m4302 = f596(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m4302 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h4303, _) = v9;
                                                    h4303
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4304, _) = v11;
                                                    h4304
                                                }), f426({
                                                    let (_, h4305) = v9;
                                                    h4305
                                                }, {
                                                    let (_, h4306) = v11;
                                                    h4306
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

pub fn f597(v0: List<Adt19>) -> R<List<Bytes>> {
    let m4307 = v0;
    match m4307.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m4308 = f617(v1)?;
            match m4308 {
                None => {
                    f597(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f597(v2)?))
                }
            }
        }
    }
}

pub fn f598(v0: Bytes, v1: List<Adt19>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f619(f410(f620(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f599(v0: Adt19, v1: Bytes) -> R<bool> {
    let m4309 = f260(Some(v0));
    match m4309 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m4310 = f550(v2)?;
            match m4310 {
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

pub fn f600(v0: bool, v1: Bytes, v2: List<(Bytes, Adt19)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4311 = v2;
    match m4311.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m4313 = f505(v0, {
                let (h4312, _) = v5.clone();
                h4312
            })?;
            match m4313 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h4314, _) = v5;
                        h4314
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f88(v7.clone(), f463(v0, f601(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m4319 = f557({
                            let (_, h4315) = v5.clone();
                            h4315
                        }, {
                            let a4317 = v1.clone();
                            let a4318 = f256({
                                let (h4316, _) = v5.clone();
                                h4316
                            })?;
                            append_bytes(a4317, a4318)
                        })?;
                        match m4319 {
                            Ok(v9) => {
                                let v10: Bytes = if f88({
                                    let (h4320, _) = v5;
                                    h4320
                                }, v3.clone()) {
                                    let (h4321, _) = v9;
                                    h4321
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h4322, _) = v9;
                                        h4322
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m4323 = f600(v0, v1, v6, v3, v4)?;
                                match m4323 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h4324, _) = v12;
                                            h4324
                                        }), f426({
                                            let (_, h4325) = v9;
                                            h4325
                                        }, {
                                            let (_, h4326) = v12;
                                            h4326
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

pub fn f601(v0: List<(Bytes, Adt19)>) -> List<Bytes> {
    let m4327 = v0;
    match m4327.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h4328, _) = v1;
                h4328
            }, f601(v2))
        }
    }
}

pub fn f602(v0: Adt19) -> Option<Adt19> {
    let m4329 = f156(f151(v0, f8(Str::lit("oneOf"))));
    match m4329.uncons() {
        None => {
            None::<Adt19>
        }
        Some((v1, v2)) => {
            let m4330 = v2;
            match m4330.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v3, v4)) => {
                    let m4331 = v4;
                    match m4331.uncons() {
                        None => {
                            if f621(v3.clone()) {
                                Some(v1)
                            } else {
                                if f621(v1) {
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

pub fn f603(v0: Bytes, v1: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4332 = v1;
    match m4332 {
        Ok(v3) => {
            f553(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h4333, _) = v3;
                h4333
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h4334) = v3;
                h4334
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v2)
        }
    }
}

pub fn f604(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4335 = f602(v0.clone());
    match m4335 {
        None => {
            if f37(f154(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m4336 = f151(v0, f8(Str::lit("items")));
                match m4336 {
                    None => {
                        Ok(f603(f8(Str::lit("List")), f605()))
                    }
                    Some(v3) => {
                        Ok(f603(f8(Str::lit("List")), f618(v3, {
                            let a4337 = v1;
                            let a4338 = f8(Str::lit("Item"));
                            append_bytes(a4337, a4338)
                        })?))
                    }
                }
            } else {
                f618(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f603(f8(Str::lit("Option")), f618(v2, v1)?))
        }
    }
}

pub fn f605() -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((f475(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f606(p4342: List<(Bytes, Adt19)>, v1: List<Bytes>) -> bool {
    let m4341 = p4342;
    {
        let mut m4341 = m4341;
        loop {
            let step = {
                let v0 = m4341;
                let m4339 = v0;
                match m4339.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f88({
                            let (h4340, _) = v2;
                            h4340
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt19)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt19)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4341 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f607(p4346: List<Adt19>, v1: Bytes, p4347: List<Bytes>) -> bool {
    let m4345 = (p4346, p4347);
    {
        let mut m4345 = m4345;
        loop {
            let step = {
                let (v0, v2) = m4345;
                let m4343 = v0;
                match m4343.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m4344 = f622(v3, v1.clone());
                        match m4344 {
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
                Ok(next) => { m4345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f608(p4350: List<Adt19>, p4351: List<Bytes>) -> bool {
    let m4349 = (p4350, p4351);
    {
        let mut m4349 = m4349;
        loop {
            let step = {
                let (v0, v1) = m4349;
                let m4348 = v0;
                match m4348.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f154(v2, f8(Str::lit("type")));
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
                Ok(next) => { m4349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f609() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f610(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f522({
        let (_, h4352) = {
            let (h4353, _) = f623(Fn22::F624, {
                let a4355 = {
                    let a4354 = v0.clone();
                    length_list(a4354)
                };
                let a4356 = 1u64;
                nat_add(a4355, a4356)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4353
        };
        h4352
    }))
}

pub fn f611(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a4361 = {
            let a4359 = {
                let a4357 = v0.clone();
                length_list(a4357)
            };
            let a4360 = {
                let a4358 = v1.clone();
                length_list(a4358)
            };
            nat_add(a4359, a4360)?
        };
        let a4362 = 1u64;
        nat_add(a4361, a4362)?
    };
    Ok(f377({
        let (_, h4363) = {
            let (_, h4364) = {
                let (h4365, _) = f625(Fn23::F626, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h4365
            };
            h4364
        };
        h4363
    }))
}

pub fn f612(v0: u64) -> Bytes {
    f32({
        let a4366 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4366)
    }, v0, 1u64)
}

pub fn f613(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a4367 = v0.clone();
        length_bytes(a4367)
    };
    let a4371 = {
        let a4368 = v1;
        let a4369 = 2u64;
        let a4370 = 0u64;
        nat_rem(a4368, a4369, a4370)
    };
    let a4372 = 0u64;
    if nat_eq(a4371, a4372) {
        let (_, h4373) = {
            let (h4374, _) = f627(Fn24::F628(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h4374
        };
        Ok(h4373)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f614(v0: u64) -> bool {
    let a4375 = v0;
    let a4376 = 8u64;
    if nat_eq(a4375, a4376) {
        true
    } else {
        let a4377 = v0;
        let a4378 = 9u64;
        if nat_eq(a4377, a4378) {
            true
        } else {
            let a4379 = v0;
            let a4380 = 10u64;
            if nat_eq(a4379, a4380) {
                true
            } else {
                let a4381 = v0;
                let a4382 = 12u64;
                if nat_eq(a4381, a4382) {
                    true
                } else {
                    let a4383 = v0;
                    let a4384 = 13u64;
                    nat_eq(a4383, a4384)
                }
            }
        }
    }
}

pub fn f615(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4429 = {
            let a4405 = {
                let a4393 = {
                    let a4387 = {
                        let a4385 = v1;
                        let a4386 = 128u8;
                        fixed_u8::bit_and(a4385, a4386)
                    };
                    let a4388 = 128u8;
                    if equal(a4387, a4388) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4394 = {
                    let a4391 = {
                        let a4389 = v1;
                        let a4390 = 64u8;
                        fixed_u8::bit_and(a4389, a4390)
                    };
                    let a4392 = 64u8;
                    if equal(a4391, a4392) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4393, a4394)?
            };
            let a4406 = {
                let a4403 = {
                    let a4397 = {
                        let a4395 = v1;
                        let a4396 = 32u8;
                        fixed_u8::bit_and(a4395, a4396)
                    };
                    let a4398 = 32u8;
                    if equal(a4397, a4398) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4404 = {
                    let a4401 = {
                        let a4399 = v1;
                        let a4400 = 16u8;
                        fixed_u8::bit_and(a4399, a4400)
                    };
                    let a4402 = 16u8;
                    if equal(a4401, a4402) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4403, a4404)?
            };
            nat_add(a4405, a4406)?
        };
        let a4430 = {
            let a4427 = {
                let a4415 = {
                    let a4409 = {
                        let a4407 = v1;
                        let a4408 = 8u8;
                        fixed_u8::bit_and(a4407, a4408)
                    };
                    let a4410 = 8u8;
                    if equal(a4409, a4410) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4416 = {
                    let a4413 = {
                        let a4411 = v1;
                        let a4412 = 4u8;
                        fixed_u8::bit_and(a4411, a4412)
                    };
                    let a4414 = 4u8;
                    if equal(a4413, a4414) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4415, a4416)?
            };
            let a4428 = {
                let a4425 = {
                    let a4419 = {
                        let a4417 = v1;
                        let a4418 = 2u8;
                        fixed_u8::bit_and(a4417, a4418)
                    };
                    let a4420 = 2u8;
                    if equal(a4419, a4420) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4426 = {
                    let a4423 = {
                        let a4421 = v1;
                        let a4422 = 1u8;
                        fixed_u8::bit_and(a4421, a4422)
                    };
                    let a4424 = 1u8;
                    if equal(a4423, a4424) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4425, a4426)?
            };
            nat_add(a4427, a4428)?
        };
        nat_add(a4429, a4430)?
    };
    let a4431 = 48u64;
    let a4432 = v2;
    if nat_le(a4431, a4432) {
        let a4433 = v2;
        let a4434 = 57u64;
        Ok(nat_le(a4433, a4434))
    } else {
        Ok(false)
    }
}

pub fn f616(p4438: List<Adt19>, v1: Bytes) -> bool {
    let m4437 = p4438;
    {
        let mut m4437 = m4437;
        loop {
            let step = {
                let v0 = m4437;
                let m4435 = v0;
                match m4435.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m4436 = f622(v2, v1.clone());
                        if match m4436 {
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
                Ok(next) => { m4437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f617(v0: Adt19) -> R<Option<Bytes>> {
    let m4439 = f260(f151(v0.clone(), f8(Str::lit("$ref"))));
    match m4439 {
        None => {
            Ok(f260(f151(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f550(v1)
        }
    }
}

pub fn f618(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f629(f520(v0), v1)
}

pub fn f619(v0: bool, v1: Bytes, v2: List<Adt19>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m4440 = v2;
    match m4440.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m4441 = f622(v5.clone(), v3.clone());
            match m4441 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m4442 = f505(v0, v7.clone())?;
                    match m4442 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f88(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4445 = f556({
                                    let a4443 = v1.clone();
                                    let a4444 = f256(v7)?;
                                    append_bytes(a4443, a4444)
                                }, f474(f422(f151(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f155(f156(f151(v5, f8(Str::lit("required"))))), true)?;
                                match m4445 {
                                    Ok(v10) => {
                                        let m4446 = f619(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m4446 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f219({
                                                    let (h4447, _) = v10;
                                                    h4447
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4448, _) = v12;
                                                    h4448
                                                }), f426({
                                                    let (_, h4449) = v10;
                                                    h4449
                                                }, {
                                                    let (_, h4450) = v12;
                                                    h4450
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

pub fn f620(v0: List<Adt19>, v1: Bytes) -> List<Bytes> {
    let m4451 = v0;
    match m4451.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4452 = f622(v2, v1.clone());
            match m4452 {
                None => {
                    f620(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f620(v3, v1))
                }
            }
        }
    }
}

pub fn f621(v0: Adt19) -> bool {
    f37(f154(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f622(v0: Adt19, v1: Bytes) -> Option<Bytes> {
    if f88(v1.clone(), f155(f156(f151(v0.clone(), f8(Str::lit("required")))))) {
        let m4453 = f151(v0, f8(Str::lit("properties")));
        match m4453 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4454 = f151(v2, v1);
                match m4454 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f260(f151(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f623(v0: Fn22, p4459: u64, p4460: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4458 = (p4459, p4460);
    {
        let mut m4458 = m4458;
        loop {
            let step = {
                let (v1, v2) = m4458;
                let m4455 = v1;
                if m4455 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4455 - 1;
                    let m4457 = {
                        let c4456: Fn22 = v0.clone();
                        c4456.apply(v2.clone())?
                    };
                    match m4457 {
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
                Ok(next) => { m4458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f624(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f630(v0)
}

pub fn f625(v0: Fn23, p4465: u64, p4466: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4464 = (p4465, p4466);
    {
        let mut m4464 = m4464;
        loop {
            let step = {
                let (v1, v2) = m4464;
                let m4461 = v1;
                if m4461 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4461 - 1;
                    let m4463 = {
                        let c4462: Fn23 = v0.clone();
                        c4462.apply(v2.clone())
                    };
                    match m4463 {
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
                Ok(next) => { m4464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f631(v0)
}

pub fn f627(v0: Fn24, p4471: u64, p4472: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4470 = (p4471, p4472);
    {
        let mut m4470 = m4470;
        loop {
            let step = {
                let (v1, v2) = m4470;
                let m4467 = v1;
                if m4467 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4467 - 1;
                    let m4469 = {
                        let c4468: Fn24 = v0.clone();
                        c4468.apply(v2.clone())?
                    };
                    match m4469 {
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
                Ok(next) => { m4470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f628(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f632(v0, v1)
}

pub fn f629(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4473 = v0.clone();
    match m4473 {
        Adt19::C0(v2) => {
            if v2 {
                Ok(f605())
            } else {
                Ok(f552(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt19::C1(_, _) => {
            Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C2(_, _, _) => {
            Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C3(_) => {
            Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C4(_) => {
            Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C5(_) => {
            let m4474 = f633(f472(v0.clone()));
            match m4474 {
                None => {
                    let m4475 = f260(f151(v0.clone(), f8(Str::lit("$ref"))));
                    match m4475 {
                        None => {
                            let m4477 = f151(v0.clone(), f8(Str::lit("allOf")));
                            match m4477 {
                                None => {
                                    let m4482 = f151(v0.clone(), f8(Str::lit("enum")));
                                    match m4482 {
                                        None => {
                                            let m4483 = f151(v0.clone(), f8(Str::lit("const")));
                                            match m4483 {
                                                None => {
                                                    let m4485 = f151(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4485 {
                                                        None => {
                                                            let v30: Bytes = f154(v0.clone(), f8(Str::lit("type")));
                                                            if if f37(v30.clone(), Bytes::lit(&[])) {
                                                                !f634(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f605())
                                                            } else {
                                                                if f37(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f553(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f37(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f553(f635(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f37(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f553(f503(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f37(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f553(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f37(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4486 = f422(f151(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4486.uncons() {
                                                                                        None => {
                                                                                            let m4487 = f636(v0);
                                                                                            match m4487 {
                                                                                                None => {
                                                                                                    Ok(f603(f8(Str::lit("List")), f637(f605())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f603(f8(Str::lit("List")), f637(f638(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f553(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f552(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f553(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m4484 = f260(Some(v27));
                                                    match m4484 {
                                                        None => {
                                                            Ok(f552(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f553(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f553(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m4478 = f261(v19);
                                    match m4478.uncons() {
                                        None => {
                                            Ok(f552(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4479 = v21;
                                            match m4479.uncons() {
                                                None => {
                                                    let m4480 = f260(f151(v20, f8(Str::lit("$ref"))));
                                                    match m4480 {
                                                        None => {
                                                            Ok(f552(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4481 = f550(v24)?;
                                                            match m4481 {
                                                                None => {
                                                                    Ok(f552(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f553(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f552(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m4476 = f550(v17)?;
                            match m4476 {
                                None => {
                                    Ok(f552(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f553(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f552(if f37(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt19::C6 => {
            Ok(f605())
        }
        Adt19::C7(_, _) => {
            Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C8(_, _, _) => {
            Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f630(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4488, _) = v0;
        h4488
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4489) = v0;
        h4489
    };
    let m4490 = v1;
    match m4490.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4491 = v4;
            match m4491.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f639(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f631(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4492, _) = v0.clone();
        h4492
    };
    let v2: List<(Bytes, ())> = {
        let (h4493, _) = {
            let (_, h4494) = v0.clone();
            h4494
        };
        h4493
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4495) = {
            let (_, h4496) = v0;
            h4496
        };
        h4495
    };
    let m4497 = v1.clone();
    match m4497.uncons() {
        None => {
            let m4498 = v2;
            match m4498.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4499 = v2.clone();
            match m4499.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f640({
                        let (h4500, _) = v8.clone();
                        h4500
                    }, {
                        let (h4501, _) = v6.clone();
                        h4501
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

pub fn f632(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4502, _) = v1;
        h4502
    };
    let m4504 = {
        let (_, h4503) = v1;
        h4503
    };
    match m4504 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4507 = {
                let a4505 = v0.clone();
                let a4506 = v2;
                index_bytes(a4505, a4506)
            };
            match m4507 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4512 = {
                        let a4510 = v0;
                        let a4511 = {
                            let a4508 = v2;
                            let a4509 = 1u64;
                            nat_add(a4508, a4509)?
                        };
                        index_bytes(a4510, a4511)
                    };
                    match m4512 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f641(v4)? {
                                f641(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4513 = v2;
                                    let a4514 = 2u64;
                                    nat_add(a4513, a4514)?
                                }, Some({
                                    let a4519 = v3;
                                    let a4520 = f332(f642(), {
                                        let a4517 = {
                                            let a4515 = f643(v4)?;
                                            let a4516 = 16u64;
                                            nat_mul(a4515, a4516)?
                                        };
                                        let a4518 = f643(v5)?;
                                        nat_add(a4517, a4518)?
                                    });
                                    append_bytes(a4519, a4520)
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

pub fn f633(p4525: List<(Bytes, Adt19)>) -> Option<Bytes> {
    let m4524 = p4525;
    {
        let mut m4524 = m4524;
        loop {
            let step = {
                let v0 = m4524;
                let m4521 = v0;
                match m4521.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f88({
                            let (h4522, _) = v1.clone();
                            h4522
                        }, f644()) {
                            Err::<List<(Bytes, Adt19)>, Option<Bytes>>(Some({
                                let (h4523, _) = v1;
                                h4523
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f634(v0: Adt19) -> bool {
    if f560(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f560(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f560(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f560(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f560(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f560(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f635(v0: Adt19) -> Bytes {
    let m4526 = f645(f151(v0, f8(Str::lit("minimum"))));
    match m4526 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f636(v0: Adt19) -> Option<Adt19> {
    let m4527 = f151(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4527 {
        None => {
            let m4528 = f422(f151(v0, f8(Str::lit("patternProperties"))));
            match m4528.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v2, v3)) => {
                    let m4529 = v3;
                    match m4529.uncons() {
                        None => {
                            Some({
                                let (_, h4530) = v2;
                                h4530
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

pub fn f637(v0: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4531 = v0;
    match m4531 {
        Ok(v2) => {
            f553(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4532, _) = v2;
                h4532
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4533) = v2;
                h4533
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v1)
        }
    }
}

pub fn f638(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f646(16u64, v0, v1)
}

pub fn f639(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4538 = {
            let a4536 = {
                let a4534 = v0.clone();
                length_list(a4534)
            };
            let a4537 = {
                let a4535 = v1.clone();
                length_list(a4535)
            };
            nat_add(a4536, a4537)?
        };
        let a4539 = 1u64;
        nat_add(a4538, a4539)?
    };
    Ok(f647({
        let (_, h4540) = {
            let (_, h4541) = {
                let (h4542, _) = f648(Fn25::F649, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4542
            };
            h4541
        };
        h4540
    }))
}

pub fn f640(v0: Bytes, v1: Bytes) -> bool {
    let a4547 = {
        let a4543 = v0;
        let a4544 = v1;
        compare_bytes(a4543, a4544)
    };
    let a4548 = {
        let a4545 = Bytes::lit(&[]);
        let a4546 = Bytes::lit(&[0x01]);
        compare_bytes(a4545, a4546)
    };
    equal(a4547, a4548)
}

pub fn f641(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4593 = {
            let a4569 = {
                let a4557 = {
                    let a4551 = {
                        let a4549 = v1;
                        let a4550 = 128u8;
                        fixed_u8::bit_and(a4549, a4550)
                    };
                    let a4552 = 128u8;
                    if equal(a4551, a4552) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4558 = {
                    let a4555 = {
                        let a4553 = v1;
                        let a4554 = 64u8;
                        fixed_u8::bit_and(a4553, a4554)
                    };
                    let a4556 = 64u8;
                    if equal(a4555, a4556) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4557, a4558)?
            };
            let a4570 = {
                let a4567 = {
                    let a4561 = {
                        let a4559 = v1;
                        let a4560 = 32u8;
                        fixed_u8::bit_and(a4559, a4560)
                    };
                    let a4562 = 32u8;
                    if equal(a4561, a4562) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4568 = {
                    let a4565 = {
                        let a4563 = v1;
                        let a4564 = 16u8;
                        fixed_u8::bit_and(a4563, a4564)
                    };
                    let a4566 = 16u8;
                    if equal(a4565, a4566) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4567, a4568)?
            };
            nat_add(a4569, a4570)?
        };
        let a4594 = {
            let a4591 = {
                let a4579 = {
                    let a4573 = {
                        let a4571 = v1;
                        let a4572 = 8u8;
                        fixed_u8::bit_and(a4571, a4572)
                    };
                    let a4574 = 8u8;
                    if equal(a4573, a4574) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4580 = {
                    let a4577 = {
                        let a4575 = v1;
                        let a4576 = 4u8;
                        fixed_u8::bit_and(a4575, a4576)
                    };
                    let a4578 = 4u8;
                    if equal(a4577, a4578) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4579, a4580)?
            };
            let a4592 = {
                let a4589 = {
                    let a4583 = {
                        let a4581 = v1;
                        let a4582 = 2u8;
                        fixed_u8::bit_and(a4581, a4582)
                    };
                    let a4584 = 2u8;
                    if equal(a4583, a4584) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4590 = {
                    let a4587 = {
                        let a4585 = v1;
                        let a4586 = 1u8;
                        fixed_u8::bit_and(a4585, a4586)
                    };
                    let a4588 = 1u8;
                    if equal(a4587, a4588) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4589, a4590)?
            };
            nat_add(a4591, a4592)?
        };
        nat_add(a4593, a4594)?
    };
    let a4595 = 48u64;
    let a4596 = v2;
    if if nat_le(a4595, a4596) {
        let a4597 = v2;
        let a4598 = 57u64;
        nat_le(a4597, a4598)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4599 = 97u64;
        let a4600 = v2;
        if nat_le(a4599, a4600) {
            let a4601 = v2;
            let a4602 = 102u64;
            Ok(nat_le(a4601, a4602))
        } else {
            Ok(false)
        }
    }
}

pub fn f642() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f643(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4647 = {
            let a4623 = {
                let a4611 = {
                    let a4605 = {
                        let a4603 = v1;
                        let a4604 = 128u8;
                        fixed_u8::bit_and(a4603, a4604)
                    };
                    let a4606 = 128u8;
                    if equal(a4605, a4606) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4612 = {
                    let a4609 = {
                        let a4607 = v1;
                        let a4608 = 64u8;
                        fixed_u8::bit_and(a4607, a4608)
                    };
                    let a4610 = 64u8;
                    if equal(a4609, a4610) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4611, a4612)?
            };
            let a4624 = {
                let a4621 = {
                    let a4615 = {
                        let a4613 = v1;
                        let a4614 = 32u8;
                        fixed_u8::bit_and(a4613, a4614)
                    };
                    let a4616 = 32u8;
                    if equal(a4615, a4616) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4622 = {
                    let a4619 = {
                        let a4617 = v1;
                        let a4618 = 16u8;
                        fixed_u8::bit_and(a4617, a4618)
                    };
                    let a4620 = 16u8;
                    if equal(a4619, a4620) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4621, a4622)?
            };
            nat_add(a4623, a4624)?
        };
        let a4648 = {
            let a4645 = {
                let a4633 = {
                    let a4627 = {
                        let a4625 = v1;
                        let a4626 = 8u8;
                        fixed_u8::bit_and(a4625, a4626)
                    };
                    let a4628 = 8u8;
                    if equal(a4627, a4628) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4634 = {
                    let a4631 = {
                        let a4629 = v1;
                        let a4630 = 4u8;
                        fixed_u8::bit_and(a4629, a4630)
                    };
                    let a4632 = 4u8;
                    if equal(a4631, a4632) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4633, a4634)?
            };
            let a4646 = {
                let a4643 = {
                    let a4637 = {
                        let a4635 = v1;
                        let a4636 = 2u8;
                        fixed_u8::bit_and(a4635, a4636)
                    };
                    let a4638 = 2u8;
                    if equal(a4637, a4638) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4644 = {
                    let a4641 = {
                        let a4639 = v1;
                        let a4640 = 1u8;
                        fixed_u8::bit_and(a4639, a4640)
                    };
                    let a4642 = 1u8;
                    if equal(a4641, a4642) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4643, a4644)?
            };
            nat_add(a4645, a4646)?
        };
        nat_add(a4647, a4648)?
    };
    let a4649 = v2;
    let a4650 = 57u64;
    if nat_le(a4649, a4650) {
        let a4651 = v2;
        let a4652 = 48u64;
        Ok(nat_sub(a4651, a4652))
    } else {
        let a4653 = v2;
        let a4654 = 87u64;
        Ok(nat_sub(a4653, a4654))
    }
}

pub fn f644() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f645(v0: Option<Adt19>) -> Option<u64> {
    let m4655 = v0;
    match m4655 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4656 = v1;
            match m4656 {
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

pub fn f646(v0: u64, v1: Adt19, v2: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4657 = v0;
    if m4657 == 0 {
        Ok(f552(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4657 - 1;
        let m4658 = v1.clone();
        match m4658 {
            Adt19::C0(v4) => {
                if v4 {
                    Ok(f605())
                } else {
                    Ok(f552(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt19::C1(_, _) => {
                Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C2(_, _, _) => {
                Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C3(_) => {
                Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C4(_) => {
                Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C5(_) => {
                let m4659 = f260(f151(v1.clone(), f8(Str::lit("$ref"))));
                match m4659 {
                    None => {
                        if if f560(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f560(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f560(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f553(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f154(v1.clone(), f8(Str::lit("type")));
                            if if f37(v20.clone(), Bytes::lit(&[])) {
                                !f634(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f605())
                            } else {
                                if f37(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f553(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f37(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f553(f635(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f37(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f553(f503(), List::<Adt25>::nil()))
                                        } else {
                                            if f37(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f553(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f37(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4661 = f151(v1, f8(Str::lit("items")));
                                                    match m4661 {
                                                        None => {
                                                            Ok(f603(f8(Str::lit("List")), f605()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f603(f8(Str::lit("List")), f646(v3, v21, {
                                                                let a4662 = v2;
                                                                let a4663 = f8(Str::lit("Item"));
                                                                append_bytes(a4662, a4663)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f37(v20.clone(), f8(Str::lit("object"))) {
                                                        let m4664 = f636(v1);
                                                        match m4664 {
                                                            None => {
                                                                Ok(f603(f8(Str::lit("List")), f637(f605())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f603(f8(Str::lit("List")), f637(f646(v3, v22, {
                                                                    let a4665 = v2;
                                                                    let a4666 = f8(Str::lit("Value"));
                                                                    append_bytes(a4665, a4666)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f552(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m4660 = f550(v18)?;
                        match m4660 {
                            None => {
                                Ok(f552(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f553(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt19::C6 => {
                Ok(f605())
            }
            Adt19::C7(_, _) => {
                Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C8(_, _, _) => {
                Ok(f552(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f647(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f432(Fn10::F650, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f648(v0: Fn25, p4671: u64, p4672: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4670 = (p4671, p4672);
    {
        let mut m4670 = m4670;
        loop {
            let step = {
                let (v1, v2) = m4670;
                let m4667 = v1;
                if m4667 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4667 - 1;
                    let m4669 = {
                        let c4668: Fn25 = v0.clone();
                        c4668.apply(v2.clone())
                    };
                    match m4669 {
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
                Ok(next) => { m4670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f649(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f651(v0)
}

pub fn f650(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f651(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4673, _) = v0.clone();
        h4673
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4674, _) = {
            let (_, h4675) = v0.clone();
            h4675
        };
        h4674
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4676) = {
            let (_, h4677) = v0;
            h4677
        };
        h4676
    };
    let m4678 = v1.clone();
    match m4678.uncons() {
        None => {
            let m4679 = v2;
            match m4679.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4680 = v2.clone();
            match m4680.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f640({
                        let (h4681, _) = v8.clone();
                        h4681
                    }, {
                        let (h4682, _) = v6.clone();
                        h4682
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

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
