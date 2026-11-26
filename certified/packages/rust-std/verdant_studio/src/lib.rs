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
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Bytes, Bytes, Bytes),
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
    C0(bool, bool, Bytes, List<Bytes>, List<List<Bytes>>),
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
pub enum Fn0 {
    F77(u8, Bytes),
    F192(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F192(k0) => f192(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F122,
    F247,
    F348,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F122 => f122(p0, p1),
            Fn1::F247 => f247(p0, p1),
            Fn1::F348 => f348(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F215(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F215(k0) => f215(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F318(Bytes),
    F319(Bytes),
    F335(Bytes, u64, Bytes, u64),
    F412(Bytes),
    F413(u64, Bytes),
    F420(Bytes),
    F481(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F318(k0) => f318(k0.clone(), p0),
            Fn3::F319(k0) => f319(k0.clone(), p0),
            Fn3::F335(k0, k1, k2, k3) => f335(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F412(k0) => Ok(f412(k0.clone(), p0)),
            Fn3::F413(k0, k1) => f413(*k0, k1.clone(), p0),
            Fn3::F420(k0) => f420(k0.clone(), p0),
            Fn3::F481(k0) => f481(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F322(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F322(k0) => f322(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F402(Bytes),
    F516(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F402(k0) => f402(k0.clone(), p0),
            Fn5::F516(k0, k1) => Ok(f516(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F404,
    F505,
    F667,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F404 => f404(p0, p1),
            Fn6::F505 => f505(p0, p1),
            Fn6::F667 => f667(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F407,
    F506,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F407 => f407(p0, p1),
            Fn7::F506 => f506(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F436,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn8::F436 => f436(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F439,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn9::F439 => f439(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F441,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn10::F441 => f441(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F444,
}

impl Fn11 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn11::F444 => f444(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F464(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn12::F464(k0, k1, k2) => f464(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F486,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn13::F486 => f486(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F489,
}

impl Fn14 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn14::F489 => f489(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F520,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F520 => f520(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F522,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F522 => f522(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F525(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F525(k0) => f525(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F542,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F542 => f542(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F549,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F549 => f549(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F553,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F553 => f553(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F599,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F599 => f599(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F602,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F602 => f602(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F629,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F629 => f629(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F653,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F653 => f653(p0),
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
    let m465 = f56(v0.clone(), f139());
    match m465.uncons() {
        None => {
            let m466 = f140(v0.clone(), f139())?;
            match m466.uncons() {
                None => {
                    let m469 = f141(match v0.clone() {
                        Adt1::C0(_, h467, _, _) => {
                            h467
                        }
                    }, f126(f142(), f143(match v0.clone() {
                        Adt1::C0(_, h468, _, _) => {
                            h468
                        }
                    }, false)));
                    match m469.uncons() {
                        None => {
                            let m471 = f56(v0.clone(), f143(match v0.clone() {
                                Adt1::C0(_, h470, _, _) => {
                                    h470
                                }
                            }, true));
                            match m471.uncons() {
                                None => {
                                    let m472 = f56(v0.clone(), f144());
                                    match m472.uncons() {
                                        None => {
                                            let m473 = f145(v0.clone())?;
                                            match m473 {
                                                None => {
                                                    let m475 = f146(v0.clone(), f143(match v0.clone() {
                                                        Adt1::C0(_, h474, _, _) => {
                                                            h474
                                                        }
                                                    }, true))?;
                                                    match m475 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f147(Adt17::C0(Adt16::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f148(v0.clone())?, f149(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f149(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f150(f97(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f143(match v0 {
                                                                Adt1::C0(_, h476, _, _) => {
                                                                    h476
                                                                }
                                                            }, true)))?;
                                                            let (h477, _) = v13;
                                                            if h477 {
                                                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h478) = v13;
                                                                    h478
                                                                }))
                                                            } else {
                                                                Ok(Adt12::C4({
                                                                    let (_, h479) = v13;
                                                                    h479
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

pub fn f83(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f151(match v0.clone() {
        Adt1::C0(_, h480, _, _) => {
            h480
        }
    }, f152())) {
        Ok(Adt12::C1(List::cons(f152(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f153(f154(f152(), f155(match v0.clone() {
            Adt1::C0(_, h481, _, _) => {
                h481
            }
        }, f152())))?;
        let m482 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m482.uncons() {
            None => {
                let m483 = f156(v0.clone(), f157(f158(f96(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?, f159(v2, v1.clone()))?;
                match m483 {
                    None => {
                        f160(v0, v1)
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
    let m484 = f125(v0.clone());
    match m484.uncons() {
        None => {
            let m485 = f126(f56(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f161(), List::<Bytes>::nil())))), f127(v0.clone())?);
            match m485.uncons() {
                None => {
                    Ok(Adt12::C3(f162(f163(v0.clone())?, List::cons((f164(), f165()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f166()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f167()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f168()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f169()), List::cons((f8(Str::lit("docs/catalog.md")), f170(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f171(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f172(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f173()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m490 = {
        let a487 = v0;
        let a488 = 0u64;
        let a489 = {
            let a486 = v1.clone();
            length_bytes(a486)
        };
        slice_bytes(a487, a488, a489)
    };
    match m490 {
        None => {
            false
        }
        Some(v2) => {
            let a491 = v2;
            let a492 = v1;
            equal(a491, a492)
        }
    }
}

pub fn f86(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f174(v3.clone(), v1.clone(), f175(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f174(v3.clone(), v1.clone(), f176(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f174(v3.clone(), v1.clone(), f175(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                    f174(v3.clone(), v1.clone(), f177(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                        f174(v3.clone(), v1.clone(), f178(v3, v1, false)?)
                    } else {
                        if f32(v0, f8(Str::lit("stage-matrix"))) {
                            f174(v3.clone(), v1.clone(), f178(v3, v1, true)?)
                        } else {
                            Ok(Adt12::C4(f79()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f87(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("admit"))) {
        f179(v3, v1)
    } else {
        if f32(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f180(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("ingest-authority"))) {
                f181(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                    f182(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                        f183(v3, v1)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("propose-encoding"))) {
                            let m493 = v2;
                            match m493.uncons() {
                                None => {
                                    Ok(Adt12::C4(f79()))
                                }
                                Some((v4, v5)) => {
                                    f184(v3, v1, v4, v5)
                                }
                            }
                        } else {
                            if f32(v0.clone(), f8(Str::lit("admit-translated"))) {
                                let m494 = v2;
                                match m494.uncons() {
                                    None => {
                                        Ok(Adt12::C4(f79()))
                                    }
                                    Some((v6, v7)) => {
                                        f185(v3, v1, v6, v7)
                                    }
                                }
                            } else {
                                if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                                    f186(v3, v1)
                                } else {
                                    if f32(v0, f8(Str::lit("record-error"))) {
                                        f187(v3, v1, v2)
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
    f188(v0.clone(), 0u64, {
        let a495 = v0;
        length_bytes(a495)
    })
}

pub fn f89(p500: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m499 = p500;
    {
        let mut m499 = m499;
        loop {
            let step = {
                let v0 = m499;
                let m496 = v0;
                match m496.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h497, _) = v2;
                            h497
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h498) = v2;
                                h498
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m499 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m504 = {
        let a502 = match v0 {
            Adt1::C0(h501, _, _, _) => {
                h501
            }
        };
        let a503 = f40(v1);
        index_list(a502, a503)
    };
    match m504 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m505 = f189(f190(v0.clone(), v1.clone()), v1.clone());
    match m505 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Adt2) -> Option<Adt14> {
    let m506 = v0;
    match m506 {
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
        Adt2::C6(r513) => {
            let v10 = (*r513).clone();
            let m507 = f92(v10);
            match m507 {
                None => {
                    None::<Adt14>
                }
                Some(v11) => {
                    Some(Adt14::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt2::C7(r514) => {
            let v12 = (*r514).clone();
            let m508 = f92(v12);
            match m508 {
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
        Adt2::C9(r515, r516) => {
            let v14 = (*r515).clone();
            let v15 = (*r516).clone();
            let m509 = f92(v14);
            match m509 {
                None => {
                    None::<Adt14>
                }
                Some(v16) => {
                    let m510 = f92(v15);
                    match m510 {
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
        Adt2::C10(v18, r517, r518) => {
            let v19 = (*r517).clone();
            let v20 = (*r518).clone();
            let m511 = f92(v19);
            match m511 {
                None => {
                    None::<Adt14>
                }
                Some(v21) => {
                    let m512 = f92(v20);
                    match m512 {
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
    let m519 = v0;
    match m519 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m520 = v1;
            match m520 {
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

pub fn f94(p526: Adt14, v1: Bytes) -> Option<Adt14> {
    let m525 = p526;
    {
        let mut m525 = m525;
        loop {
            let step = {
                let v0 = m525;
                let m521 = v0;
                match m521 {
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
                    Adt14::C4(r522) => {
                        let v5 = (*r522).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r523, r524) => {
                        let v3 = (*r523).clone();
                        let v4 = (*r524).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m525 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m527 = v0;
    match m527.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m528 = f191(v2, v1.clone())?;
            match m528 {
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
        let a529 = v0.clone();
        length_bytes(a529)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h530, _) = f76(Fn0::F192(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h530
    };
    let v3: u64 = {
        let (h531, _) = {
            let (_, h532) = v2.clone();
            h532
        };
        h531
    };
    let v4: List<Bytes> = {
        let (_, h533) = {
            let (_, h534) = v2;
            h534
        };
        h533
    };
    let a535 = v3;
    let a536 = v1;
    if nat_lt(a535, a536) {
        Ok(f78(List::cons(f46(v0, v3, {
            let a537 = v1;
            let a538 = v3;
            nat_sub(a537, a538)
        }), v4)))
    } else {
        Ok(f78(v4))
    }
}

pub fn f97(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f193(f57(v0, v1)?))
}

pub fn f98(v0: List<Adt14>, v1: Bytes) -> List<Adt14> {
    let m539 = v0;
    match m539.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f194(v2, v1.clone()), f98(v3, v1))
        }
    }
}

pub fn f99(p544: List<Bytes>, p545: List<Bytes>) -> bool {
    let m543 = (p544, p545);
    {
        let mut m543 = m543;
        loop {
            let step = {
                let (v0, v1) = m543;
                let m540 = v0;
                match m540.uncons() {
                    None => {
                        let m541 = v1;
                        match m541.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m542 = v1;
                        match m542.uncons() {
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
                Ok(next) => { m543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: List<Adt14>) -> List<Bytes> {
    let m546 = v0;
    match m546.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m547 = f94(v1, f8(Str::lit("std")));
            match m547 {
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
    let a548 = f195(v0)?;
    let a549 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a548, a549))
}

pub fn f102(v0: List<Adt14>) -> Adt14 {
    let m550 = v0;
    match m550.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f102(v2)))
        }
    }
}

pub fn f103(v0: Adt13) -> Bytes {
    let m551 = v0;
    match m551 {
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
    let a553 = {
        let a552 = v2.clone();
        length_bytes(a552)
    };
    let a554 = 0u64;
    if nat_eq(a553, a554) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m555 = f196(v0.clone(), f60())?;
        match m555 {
            None => {
                if f51(f52(f199(), match v0.clone() {
                    Adt1::C0(_, _, h556, _) => {
                        h556
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f199()))
                } else {
                    let m557 = f56(v0.clone(), List::cons(f200(), f201(f60())?));
                    match m557.uncons() {
                        None => {
                            Ok(Adt12::C3(f202(v0.clone(), f60(), v1.clone(), v2.clone(), f203(f64(v0, f200()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f197(v3.clone())?, f198(v3, v2)))
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

pub fn f115(v0: Bytes, p585: u64, p586: u64, p587: bool, p588: bool, p589: u64, p590: Bytes) -> R<Bytes> {
    let m584 = (p585, (p586, (p587, (p588, (p589, p590)))));
    {
        let mut m584 = m584;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m584;
                let m558 = v2;
                if m558 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f204(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m558 - 1;
                    let m561 = {
                        let a559 = v0.clone();
                        let a560 = v1;
                        index_bytes(a559, a560)
                    };
                    match m561 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f204(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f72(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a562 = v1;
                                    let a563 = 1u64;
                                    nat_add(a562, a563)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a564 = v9;
                                    let a565 = 34u64;
                                    !nat_eq(a564, a565)
                                }, (if v4 {
                                    false
                                } else {
                                    let a566 = v9;
                                    let a567 = 92u64;
                                    nat_eq(a566, a567)
                                }, (v5, v6))))))
                            } else {
                                let a568 = v9;
                                let a569 = 32u64;
                                if if if if nat_eq(a568, a569) {
                                    true
                                } else {
                                    let a570 = v9;
                                    let a571 = 9u64;
                                    nat_eq(a570, a571)
                                } {
                                    true
                                } else {
                                    let a572 = v9;
                                    let a573 = 10u64;
                                    nat_eq(a572, a573)
                                } {
                                    true
                                } else {
                                    let a574 = v9;
                                    let a575 = 13u64;
                                    nat_eq(a574, a575)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a576 = v1;
                                        let a577 = 1u64;
                                        nat_add(a576, a577)?
                                    }, (v7, (false, (false, ({
                                        let a578 = v1;
                                        let a579 = 1u64;
                                        nat_add(a578, a579)?
                                    }, f204(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a580 = v1;
                                        let a581 = 1u64;
                                        nat_add(a580, a581)?
                                    }, (v7, ({
                                        let a582 = v9;
                                        let a583 = 34u64;
                                        nat_eq(a582, a583)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m584 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<u64> {
    let m593 = {
        let a591 = v0;
        let a592 = v1;
        index_bytes(a591, a592)
    };
    match m593 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a638 = {
                let a614 = {
                    let a602 = {
                        let a596 = {
                            let a594 = v3;
                            let a595 = 128u8;
                            fixed_u8::bit_and(a594, a595)
                        };
                        let a597 = 128u8;
                        if equal(a596, a597) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a603 = {
                        let a600 = {
                            let a598 = v3;
                            let a599 = 64u8;
                            fixed_u8::bit_and(a598, a599)
                        };
                        let a601 = 64u8;
                        if equal(a600, a601) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a602, a603)?
                };
                let a615 = {
                    let a612 = {
                        let a606 = {
                            let a604 = v3;
                            let a605 = 32u8;
                            fixed_u8::bit_and(a604, a605)
                        };
                        let a607 = 32u8;
                        if equal(a606, a607) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a613 = {
                        let a610 = {
                            let a608 = v3;
                            let a609 = 16u8;
                            fixed_u8::bit_and(a608, a609)
                        };
                        let a611 = 16u8;
                        if equal(a610, a611) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a612, a613)?
                };
                nat_add(a614, a615)?
            };
            let a639 = {
                let a636 = {
                    let a624 = {
                        let a618 = {
                            let a616 = v3;
                            let a617 = 8u8;
                            fixed_u8::bit_and(a616, a617)
                        };
                        let a619 = 8u8;
                        if equal(a618, a619) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a625 = {
                        let a622 = {
                            let a620 = v3;
                            let a621 = 4u8;
                            fixed_u8::bit_and(a620, a621)
                        };
                        let a623 = 4u8;
                        if equal(a622, a623) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a624, a625)?
                };
                let a637 = {
                    let a634 = {
                        let a628 = {
                            let a626 = v3;
                            let a627 = 2u8;
                            fixed_u8::bit_and(a626, a627)
                        };
                        let a629 = 2u8;
                        if equal(a628, a629) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a635 = {
                        let a632 = {
                            let a630 = v3;
                            let a631 = 1u8;
                            fixed_u8::bit_and(a630, a631)
                        };
                        let a633 = 1u8;
                        if equal(a632, a633) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a634, a635)?
                };
                nat_add(a636, a637)?
            };
            nat_add(a638, a639)
        }
    }
}

pub fn f117(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f205(v0.clone(), v1, {
        let a641 = {
            let a640 = v0.clone();
            length_bytes(a640)
        };
        let a642 = 1u64;
        nat_add(a641, a642)?
    })?;
    let a643 = f116(v0.clone(), v2)?;
    let a644 = 34u64;
    if nat_eq(a643, a644) {
        Ok(Some(({
            let a645 = v2;
            let a646 = 1u64;
            nat_add(a645, a646)?
        }, f46(v0, v1, {
            let a647 = v2;
            let a648 = v1;
            nat_sub(a647, a648)
        }))))
    } else {
        f206(v0.clone(), v1, {
            let a650 = {
                let a649 = v0;
                length_bytes(a649)
            };
            let a651 = 1u64;
            nat_add(a650, a651)?
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
        let a652 = f116(v0.clone(), v1)?;
        let a653 = 45u64;
        nat_eq(a652, a653)
    };
    let v3: u64 = if v2 {
        let a654 = v1;
        let a655 = 1u64;
        nat_add(a654, a655)?
    } else {
        v1
    };
    let v4: u64 = f207(v0.clone(), v3)?;
    let v5: u64 = {
        let a656 = v4;
        let a657 = v3;
        nat_sub(a656, a657)
    };
    let v6: bool = {
        let a658 = f116(v0.clone(), v4)?;
        let a659 = 46u64;
        nat_eq(a658, a659)
    };
    let v7: u64 = if v6 {
        let a660 = v4;
        let a661 = 1u64;
        nat_add(a660, a661)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f207(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f116(v0.clone(), v8)?;
    let v10: bool = {
        let a662 = v9;
        let a663 = 101u64;
        if nat_eq(a662, a663) {
            true
        } else {
            let a664 = v9;
            let a665 = 69u64;
            nat_eq(a664, a665)
        }
    };
    let v11: u64 = f116(v0.clone(), {
        let a666 = v8;
        let a667 = 1u64;
        nat_add(a666, a667)?
    })?;
    let v12: bool = f118(v10, f208({
        let a668 = v11;
        let a669 = 43u64;
        nat_eq(a668, a669)
    }, {
        let a670 = v11;
        let a671 = 45u64;
        nat_eq(a670, a671)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a672 = v8;
            let a673 = 2u64;
            nat_add(a672, a673)?
        } else {
            let a674 = v8;
            let a675 = 1u64;
            nat_add(a674, a675)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f207(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a676 = v14;
        let a677 = v13;
        nat_sub(a676, a677)
    });
    let v16: u64 = f209(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a679 = {
            let a678 = v15;
            length_bytes(a678)
        };
        let a680 = v16;
        nat_sub(a679, a680)
    });
    let a681 = v5;
    let a682 = 0u64;
    if if nat_eq(a681, a682) {
        true
    } else {
        if f118({
            let a683 = 1u64;
            let a684 = v5;
            nat_lt(a683, a684)
        }, {
            let a685 = f116(v0.clone(), v3)?;
            let a686 = 48u64;
            nat_eq(a685, a686)
        }) {
            true
        } else {
            if f118(v6, {
                let a687 = v8;
                let a688 = v7;
                nat_eq(a687, a688)
            }) {
                true
            } else {
                f208(if v10 {
                    let a689 = v14;
                    let a690 = v13;
                    nat_eq(a689, a690)
                } else {
                    false
                }, {
                    let a692 = 18u64;
                    let a693 = {
                        let a691 = v17.clone();
                        length_bytes(a691)
                    };
                    nat_lt(a692, a693)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m697 = {
            let a695 = {
                let a694 = v17.clone();
                length_bytes(a694)
            };
            let a696 = 0u64;
            if nat_eq(a695, a696) {
                Some(0u64)
            } else {
                f210(v17)?
            }
        };
        match m697 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m702 = f211(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a698 = v8;
                    let a699 = v7;
                    nat_sub(a698, a699)
                }), f118(v12, {
                    let a700 = v11;
                    let a701 = 45u64;
                    nat_eq(a700, a701)
                }), v18)?;
                match m702 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a705 = 19u64;
                        let a706 = {
                            let a704 = {
                                let (h703, _) = v19.clone();
                                h703
                            };
                            length_bytes(a704)
                        };
                        if nat_lt(a705, a706) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m708 = f210({
                                let (h707, _) = v19.clone();
                                h707
                            })?;
                            match m708 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f118(v2, {
                                        let a709 = 0u64;
                                        let a710 = v20;
                                        nat_lt(a709, a710)
                                    });
                                    if f212(v21, v20) {
                                        Ok(Some((v14, {
                                            let a712 = {
                                                let (_, h711) = v19.clone();
                                                h711
                                            };
                                            let a713 = 0u64;
                                            if nat_eq(a712, a713) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h714) = v19;
                                                    h714
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
        let (h715, _) = v2.clone();
        h715
    };
    let v4: u64 = {
        let (h716, _) = {
            let (_, h717) = v2.clone();
            h717
        };
        h716
    };
    let v5: List<Bytes> = {
        let (_, h718) = {
            let (_, h719) = v2;
            h719
        };
        h718
    };
    let m720 = f213(v0.clone(), v3);
    match m720 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a721 = v6;
            let a722 = v1;
            if equal(a721, a722) {
                Ok(Some(({
                    let a723 = v3;
                    let a724 = 1u64;
                    nat_add(a723, a724)?
                }, ({
                    let a725 = v3;
                    let a726 = 1u64;
                    nat_add(a725, a726)?
                }, List::cons(f46(v0, v4, {
                    let a727 = v3;
                    let a728 = v4;
                    nat_sub(a727, a728)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a729 = v3;
                    let a730 = 1u64;
                    nat_add(a729, a730)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f121(v0: Fn1, p734: List<Bytes>, p735: List<Bytes>) -> List<Bytes> {
    let m733 = (p734, p735);
    {
        let mut m733 = m733;
        loop {
            let step = {
                let (v1, v2) = m733;
                let m731 = v2;
                match m731.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c732: Fn1 = v0.clone();
                            c732.apply(v1, v3)
                        }, v4))
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

pub fn f122(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m736 = f214(Fn2::F215(v1), None::<Bytes>, v0);
    match m736 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil()))))))))))))))))))))))))))
}

pub fn f125(v0: Adt1) -> List<Bytes> {
    if f151(match v0.clone() {
        Adt1::C0(_, h737, _, _) => {
            h737
        }
    }, f216()) {
        if f151(match v0 {
            Adt1::C0(_, h738, _, _) => {
                h738
            }
        }, f217()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f217(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f216(), List::<Bytes>::nil())
    }
}

pub fn f126(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m739 = v0;
    match m739.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f218(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f137(v0)?)))))
}

pub fn f128(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f148(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f149(f97(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f129(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m740 = v0;
    match m740.uncons() {
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
    let m741 = v0;
    match m741.uncons() {
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
    let m742 = v0;
    match m742.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f126(f150(f193(f94(v1, f8(Str::lit("defers"))))), f131(v2))
        }
    }
}

pub fn f132(v0: List<Adt14>) -> List<Bytes> {
    let m743 = v0;
    match m743.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m744 = f94(v1, f8(Str::lit("std")));
            match m744 {
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
    let m745 = v0;
    match m745.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f162(f219(v2.clone()), f220(f221(v0, v1.clone(), v2)?));
    f222(f153(f223(v1))?, v3.clone(), {
        let a747 = {
            let a746 = v3;
            length_list(a746)
        };
        let a748 = 1u64;
        nat_add(a747, a748)?
    })
}

pub fn f135(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m749 = v1;
    match m749.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f224(v2.clone(), f8(Str::lit("subject"))) {
                f225(v0.clone(), v2.clone())
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
    let m750 = v1;
    match m750.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m751 = f226(f96(f64(v0.clone(), v2.clone()))?, f85(v2, f217()))?;
            match m751 {
                None => {
                    f136(v0, v3)
                }
                Some(v4) => {
                    Ok(f227(v4, f136(v0, v3)?))
                }
            }
        }
    }
}

pub fn f137(v0: Adt1) -> R<List<Bytes>> {
    Ok(f126(f153(f228(f216(), f155(match v0.clone() {
        Adt1::C0(_, h752, _, _) => {
            h752
        }
    }, f216())))?, f153(f228(f217(), f155(match v0 {
        Adt1::C0(_, h753, _, _) => {
            h753
        }
    }, f217())))?))
}

pub fn f138(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f139() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f218(), List::<Bytes>::nil())))))))
}

pub fn f140(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m754 = v1;
    match m754.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m755 = f57(v0.clone(), v2.clone())?;
                match m755 {
                    None => {
                        Ok(List::cons(v2, f140(v0, v3)?))
                    }
                    Some(_) => {
                        f140(v0, v3)
                    }
                }
            } else {
                f140(v0, v3)
            }
        }
    }
}

pub fn f141(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m756 = v1;
    match m756.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f151(v0.clone(), v2.clone()) {
                f141(v0, v3)
            } else {
                List::cons(v2, f141(v0, v3))
            }
        }
    }
}

pub fn f142() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f143(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m757 = v0;
    match m757.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f126(f229({
                let (h758, _) = v2;
                h758
            }, {
                let (_, h759) = v2;
                h759
            }, v1), f143(v3, v1))
        }
    }
}

pub fn f144() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f145(v0: Adt1) -> R<Option<Bytes>> {
    f230(f231(v0, f144())?)
}

pub fn f146(v0: Adt1, p763: List<Bytes>) -> R<Option<Bytes>> {
    let m762 = p763;
    {
        let mut m762 = m762;
        loop {
            let step = {
                let v1 = m762;
                let m760 = v1;
                match m760.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m761 = f232(f64(v0.clone(), v2.clone()), f233())?;
                        match m761 {
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
                Ok(next) => { m762 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Adt17) -> R<(bool, Bytes)> {
    let v1: Adt16 = match v0.clone() {
        Adt17::C0(h764, _, _, _, _) => {
            h764
        }
    };
    let v2: Adt18 = f234(v1.clone())?;
    let v3: List<Bytes> = f235(v1.clone());
    let v4: List<Bytes> = f149(match v0.clone() {
        Adt17::C0(_, _, h765, _, _) => {
            h765
        }
    }, f8(Str::lit("path")));
    let m767 = f236(v2, List::<u64>::nil(), f237(v1.clone(), match v0.clone() {
        Adt17::C0(_, h766, _, _, _) => {
            h766
        }
    }, 0u64)?);
    match m767 {
        None => {
            if f239(f240(match v0.clone() {
                Adt17::C0(_, _, _, _, h769) => {
                    h769
                }
            }, f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h770) => {
                    h770
                }
            }, v4.clone()))?, f240(v4.clone(), f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h771) => {
                    h771
                }
            }, v4.clone()))?, f241(match v0.clone() {
                Adt17::C0(_, _, h772, _, _) => {
                    h772
                }
            })) {
                let m776 = f242(f240(match v0.clone() {
                    Adt17::C0(_, _, _, h773, _) => {
                        h773
                    }
                }, f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h774, _) => {
                        h774
                    }
                }))?, f240(v3.clone(), f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h775, _) => {
                        h775
                    }
                }))?);
                match m776 {
                    None => {
                        let m779 = f243(f149(match v0 {
                            Adt17::C0(_, h778, _, _, _) => {
                                h778
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m779 {
                            None => {
                                let m780 = f243(v4, List::<Bytes>::nil());
                                match m780 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f238(f126(v3, match v0 {
                            Adt17::C0(_, _, _, h777, _) => {
                                h777
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f238(match v1 {
                Adt16::C0(_, _, _, _, h768) => {
                    h768
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f148(v0: Adt1) -> R<List<Bytes>> {
    let m781 = f244(f64(v0, f218()))?;
    match m781.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f245(v2))
        }
    }
}

pub fn f149(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m782 = v0;
    match m782.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f149(v3, v1))
        }
    }
}

pub fn f150(v0: List<Adt14>) -> List<Bytes> {
    let m783 = v0;
    match m783.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m784 = f93(Some(v1));
            match m784 {
                None => {
                    f150(v2)
                }
                Some(v3) => {
                    List::cons(v3, f150(v2))
                }
            }
        }
    }
}

pub fn f151(p788: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m787 = p788;
    {
        let mut m787 = m787;
        loop {
            let step = {
                let v0 = m787;
                let m785 = v0;
                match m785.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h786, _) = v2;
                            h786
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m787 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f153(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f246(f121(Fn1::F247, List::<Bytes>::nil(), f248(v0)?)))
}

pub fn f154(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m789 = v1;
    match m789.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f249(v0.clone(), v2), f154(v0, v3))
            } else {
                f154(v0, v3)
            }
        }
    }
}

pub fn f155(p794: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m793 = p794;
    {
        let mut m793 = m793;
        loop {
            let step = {
                let v0 = m793;
                let m790 = v0;
                match m790.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h791, _) = v2;
                            h791
                        }, v1.clone()) {
                            let (_, h792) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h792)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m793 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p797: List<Bytes>) -> R<Option<Bytes>> {
    let m796 = p797;
    {
        let mut m796 = m796;
        loop {
            let step = {
                let v2 = m796;
                let m795 = v2;
                match m795.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f250(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m798 = v0;
    match m798.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f251(v1.clone())?, v1), f157(v2)?))
        }
    }
}

pub fn f158(p801: List<Bytes>, p802: List<Bytes>, p803: List<List<Bytes>>) -> List<List<Bytes>> {
    let m800 = (p801, (p802, p803));
    {
        let mut m800 = m800;
        loop {
            let step = {
                let (v0, (v1, v2)) = m800;
                let m799 = v0;
                match m799.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f252(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f252(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m800 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m804 = v0;
    match m804.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f159(v3, v1)
            } else {
                List::cons(v2, f159(v3, v1))
            }
        }
    }
}

pub fn f160(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f151(match v0.clone() {
        Adt1::C0(_, h805, _, _) => {
            h805
        }
    }, f253())) {
        Ok(Adt12::C1(List::cons(f253(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f159(f153(f154(f253(), f155(match v0.clone() {
            Adt1::C0(_, h806, _, _) => {
                h806
            }
        }, f253())))?, v1.clone());
        let m807 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m807.uncons() {
            None => {
                let v5: List<Bytes> = f149(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m808 = f56(v0.clone(), f254(v0.clone(), v2.clone(), v5.clone())?);
                match m808.uncons() {
                    None => {
                        let m809 = f255(v0.clone(), v2, v5)?;
                        match m809 {
                            None => {
                                f256(v0, v1)
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

pub fn f161() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f162(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m810 = v0;
    match m810.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f162(v3, v1))
        }
    }
}

pub fn f163(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m811 = f97(v0.clone(), f161())?;
    match m811.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f257(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f164() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f165() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f166() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f258(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op > target/attempt.txt 2>&1; then\u{a}                tee -a target/operation.txt < target/attempt.txt\u{a}                last=$op\u{a}              else\u{a}                cat target/attempt.txt\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f167() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f258(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f168() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f169() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f170(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f128(v0.clone())?;
    let v2: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f135(v1.clone(), f136(v0.clone(), f137(v0.clone())?)?);
    let v4: List<List<Bytes>> = f259(f244(f64(v0, f218()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f260(v4.clone(), f245(v4), f221(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f134(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f171(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f261(f128(v0.clone())?, f136(v0.clone(), f137(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f172(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f262(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f263(f97(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f173() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f174(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m812 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m812.uncons() {
        None => {
            if f264(f96(f64(v0, v1))?)? {
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

pub fn f175(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m813 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m813.uncons() {
        None => {
            let v5: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f265(v5)?
                } else {
                    f266(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f249(f267(), f268(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f249(f267(), f268(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f269()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f176(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m814 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m814.uncons() {
        None => {
            let m815 = f270(f96(f64(v0.clone(), v1.clone()))?)?;
            match m815 {
                None => {
                    Ok(Adt12::C4(f271()))
                }
                Some(v4) => {
                    let m816 = f272(f96(f64(v0, v1))?)?;
                    match m816 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f273(v5)))
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
    let m817 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m817.uncons() {
        None => {
            let m818 = f274(f96(f64(v0, v1))?)?;
            match m818 {
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

pub fn f178(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m819 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m819.uncons() {
        None => {
            let v5: Bytes = f249(if v2 {
                f217()
            } else {
                f216()
            }, f268(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m820 = f226(f96(f64(v0.clone(), v1.clone()))?, v2)?;
                match m820 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a822 = {
                            let a821 = v6;
                            length_list(a821)
                        };
                        let a823 = 0u64;
                        if nat_eq(a822, a823) {
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

pub fn f179(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m824 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m824.uncons() {
        None => {
            let v4: List<Bytes> = f96(f64(v0.clone(), v1))?;
            let m825 = f275(v4.clone(), f8(Str::lit("requirement")))?;
            match m825 {
                None => {
                    Ok(Adt12::C4(f269()))
                }
                Some(v5) => {
                    let m826 = f275(v4, f8(Str::lit("admission")))?;
                    match m826 {
                        None => {
                            Ok(Adt12::C4(f269()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f126(f149(v5.clone(), f8(Str::lit("id"))), f149(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f153(f126(f149(v7.clone(), f8(Str::lit("id"))), f149(v8.clone(), f8(Str::lit("construct")))))?;
                            let m827 = f276(f132(v6.clone()), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m827 {
                                None => {
                                    let a829 = {
                                        let a828 = v9.clone();
                                        length_list(a828)
                                    };
                                    let a830 = 0u64;
                                    if nat_eq(a829, a830) {
                                        Ok(Adt12::C4(f269()))
                                    } else {
                                        if f51({
                                            let a833 = {
                                                let a831 = f153(f126(v9.clone(), v10.clone()))?;
                                                length_list(a831)
                                            };
                                            let a834 = {
                                                let a832 = f126(v9.clone(), v10);
                                                length_list(a832)
                                            };
                                            nat_eq(a833, a834)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f278(f8(Str::lit("model/requirements.json")), f227(v7, v5))?, List::cons(f278(f8(Str::lit("model/admissions.json")), f227(v8, v6))?, List::cons(f279(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f123(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f277(v11))
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

pub fn f180(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m835 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m835.uncons() {
        None => {
            let m836 = f91(v0.clone(), v1.clone())?;
            match m836 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m837 = f280(v4.clone(), f8(Str::lit("schema")));
                    match m837 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f281(v0, f282(v5.clone())?, v5, f283(v4.clone(), f8(Str::lit("name"))), f284(f285(f280(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f181(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m838 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m838.uncons() {
        None => {
            let m839 = f57(v0.clone(), v1.clone())?;
            match m839 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f286())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m840 = f94(v4.clone(), f8(Str::lit("source")));
                            match m840 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f93(f94(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f249(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f249(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m841 = v8;
                        match m841 {
                            None => {
                                f287(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f287(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f249(v9, f268(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h842, _) => {
                                            h842
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m843 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m843.uncons() {
                                            None => {
                                                if f32(f288(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f287(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f182(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m844 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m844.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f244(v4.clone())?;
            if f51(f99(f289(v5.clone()), f290())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f291(Str::lit("id"), v6.clone())?, List::cons(f291(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f291(Str::lit("artifact"), f218())?, List::cons(f291(Str::lit("checksum"), f288(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f218(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f292(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f279(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f123(f245(f259(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f183(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m845 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m845.uncons() {
        None => {
            let m846 = f57(v0, v1)?;
            match m846 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f293(f193(Some(v4))) {
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

pub fn f184(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m847 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m847.uncons() {
        None => {
            let m848 = f91(v0.clone(), v1.clone())?;
            match m848 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f281(v0.clone(), f64(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f185(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m849 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil())))))));
    match m849.uncons() {
        None => {
            let m850 = f294(v3.clone());
            match m850.uncons() {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                }
                Some((v6, v7)) => {
                    if f85(v6.clone(), f8(Str::lit("read-"))) {
                        f295(v0.clone(), f64(v0, v1), v2, List::cons(v6, v7), f296(v3))
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

pub fn f186(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m851 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f161(), List::<Bytes>::nil())));
    match m851.uncons() {
        None => {
            let m852 = f297(f96(f64(v0.clone(), v1))?)?;
            match m852 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f97(v0, f161())?;
                    let v6: List<Bytes> = f149(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f153(f149(v5.clone(), f8(Str::lit("at"))))?;
                    let a854 = {
                        let a853 = v6.clone();
                        length_list(a853)
                    };
                    let a855 = 0u64;
                    if if nat_eq(a854, a855) {
                        true
                    } else {
                        f51({
                            let a858 = {
                                let a856 = f153(f126(v6.clone(), v7.clone()))?;
                                length_list(a856)
                            };
                            let a859 = {
                                let a857 = f126(v6.clone(), v7);
                                length_list(a857)
                            };
                            nat_eq(a858, a859)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f278(f161(), f227(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f123(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f187(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m860 = v2;
    match m860.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m861 = v4;
            match m861.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m862 = v6.clone();
                    match m862.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m863 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f298(), List::<Bytes>::nil())));
                            match m863.uncons() {
                                None => {
                                    let v11: Bytes = f123(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt14> = f97(v0, f298())?;
                                    if f299(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f278(f298(), f227(v13, List::cons(f300(v11.clone(), v12, v5, v3), List::<Adt14>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f188(v0: Bytes, p917: u64, p918: u64) -> R<bool> {
    let m916 = (p917, p918);
    {
        let mut m916 = m916;
        loop {
            let step = {
                let (v1, v2) = m916;
                let m864 = v2;
                if m864 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m864 - 1;
                    let m867 = {
                        let a865 = v0.clone();
                        let a866 = v1;
                        index_bytes(a865, a866)
                    };
                    match m867 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f301({
                                let v5: u8 = v4;
                                let a912 = {
                                    let a888 = {
                                        let a876 = {
                                            let a870 = {
                                                let a868 = v5;
                                                let a869 = 128u8;
                                                fixed_u8::bit_and(a868, a869)
                                            };
                                            let a871 = 128u8;
                                            if equal(a870, a871) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a877 = {
                                            let a874 = {
                                                let a872 = v5;
                                                let a873 = 64u8;
                                                fixed_u8::bit_and(a872, a873)
                                            };
                                            let a875 = 64u8;
                                            if equal(a874, a875) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a876, a877)?
                                    };
                                    let a889 = {
                                        let a886 = {
                                            let a880 = {
                                                let a878 = v5;
                                                let a879 = 32u8;
                                                fixed_u8::bit_and(a878, a879)
                                            };
                                            let a881 = 32u8;
                                            if equal(a880, a881) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a887 = {
                                            let a884 = {
                                                let a882 = v5;
                                                let a883 = 16u8;
                                                fixed_u8::bit_and(a882, a883)
                                            };
                                            let a885 = 16u8;
                                            if equal(a884, a885) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a886, a887)?
                                    };
                                    nat_add(a888, a889)?
                                };
                                let a913 = {
                                    let a910 = {
                                        let a898 = {
                                            let a892 = {
                                                let a890 = v5;
                                                let a891 = 8u8;
                                                fixed_u8::bit_and(a890, a891)
                                            };
                                            let a893 = 8u8;
                                            if equal(a892, a893) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a899 = {
                                            let a896 = {
                                                let a894 = v5;
                                                let a895 = 4u8;
                                                fixed_u8::bit_and(a894, a895)
                                            };
                                            let a897 = 4u8;
                                            if equal(a896, a897) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a898, a899)?
                                    };
                                    let a911 = {
                                        let a908 = {
                                            let a902 = {
                                                let a900 = v5;
                                                let a901 = 2u8;
                                                fixed_u8::bit_and(a900, a901)
                                            };
                                            let a903 = 2u8;
                                            if equal(a902, a903) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a909 = {
                                            let a906 = {
                                                let a904 = v5;
                                                let a905 = 1u8;
                                                fixed_u8::bit_and(a904, a905)
                                            };
                                            let a907 = 1u8;
                                            if equal(a906, a907) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a908, a909)?
                                    };
                                    nat_add(a910, a911)?
                                };
                                nat_add(a912, a913)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a914 = v1;
                                    let a915 = 1u64;
                                    nat_add(a914, a915)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(p923: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m922 = p923;
    {
        let mut m922 = m922;
        loop {
            let step = {
                let v0 = m922;
                let m919 = v0;
                match m919.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h920, _) = v2;
                            h920
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h921) = v2;
                                h921
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m922 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m927 = {
        let a925 = match v0 {
            Adt1::C0(_, _, _, h924) => {
                h924
            }
        };
        let a926 = f40(v1);
        index_list(a925, a926)
    };
    match m927 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f191(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m928 = f44(v0, 34u8)?;
        match m928.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m929 = v3;
                match m929.uncons() {
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

pub fn f192(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f302(v0, v1)
}

pub fn f193(v0: Option<Adt14>) -> List<Adt14> {
    let m930 = v0;
    match m930 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f303(v1)
        }
    }
}

pub fn f194(v0: Adt14, v1: Bytes) -> Adt14 {
    let m931 = v0;
    match m931 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r932) => {
            let v10 = (*r932).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r933) => {
            let v5 = (*r933).clone();
            Adt14::C4(std::rc::Rc::new(f194(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r934, r935) => {
            let v11 = (*r934).clone();
            let v12 = (*r935).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r936, r937) => {
            let v3 = (*r936).clone();
            let v4 = (*r937).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt14::C7(v2, std::rc::Rc::new(f304(v3, v1.clone())), std::rc::Rc::new(f194(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f194(v4, v1)))
            }
        }
    }
}

pub fn f195(v0: Adt14) -> R<Bytes> {
    let m938 = v0;
    match m938 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f305(v2, v3)
        }
        Adt14::C2(v4) => {
            f306(v4)
        }
        Adt14::C3(r939) => {
            let v5 = (*r939).clone();
            Ok(f307(f308(v5)?))
        }
        Adt14::C4(r940) => {
            let v6 = (*r940).clone();
            f309(f310(v6)?)
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

pub fn f196(v0: Adt1, p944: List<Adt13>) -> R<Option<Adt13>> {
    let m943 = p944;
    {
        let mut m943 = m943;
        loop {
            let step = {
                let v1 = m943;
                let m941 = v1;
                match m941.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f197(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h942, _) => {
                                h942
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
                Ok(next) => { m943 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt13) -> R<Bytes> {
    Ok(f249(f311(), f268(f312(v0))?))
}

pub fn f198(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f312(v0), List::<Bytes>::nil())))))
}

pub fn f199() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f200() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f201(v0: List<Adt13>) -> R<List<Bytes>> {
    let m945 = v0;
    match m945.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f197(v1)?, f201(v2)?))
        }
    }
}

pub fn f202(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m946 = v1;
    match m946.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f313(v5.clone(), v2.clone(), v3.clone(), f288(f64(v0.clone(), f197(v5)?))?, v4.clone())?), f202(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f203(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f314(v0.clone())?;
    let v2: u64 = f315(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a947 = v1;
        let a948 = v2;
        nat_sub(a947, a948)
    }))
}

pub fn f204(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a951 = v3;
    let a952 = f46(v0, v1, {
        let a949 = v2;
        let a950 = v1;
        nat_sub(a949, a950)
    });
    append_bytes(a951, a952)
}

pub fn f205(v0: Bytes, p965: u64, p966: u64) -> R<u64> {
    let m964 = (p965, p966);
    {
        let mut m964 = m964;
        loop {
            let step = {
                let (v1, v2) = m964;
                let m953 = v2;
                if m953 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m953 - 1;
                    let v4: u64 = f116(v0.clone(), v1)?;
                    let a954 = v4;
                    let a955 = 34u64;
                    if nat_eq(a954, a955) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a956 = v4;
                        let a957 = 92u64;
                        if nat_eq(a956, a957) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a958 = v4;
                            let a959 = 32u64;
                            if nat_lt(a958, a959) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a960 = v4;
                                let a961 = 256u64;
                                if nat_eq(a960, a961) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a962 = v1;
                                        let a963 = 1u64;
                                        nat_add(a962, a963)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m964 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes, p992: u64, p993: u64, p994: Bytes) -> R<Option<(u64, Bytes)>> {
    let m991 = (p992, (p993, p994));
    {
        let mut m991 = m991;
        loop {
            let step = {
                let (v1, (v2, v3)) = m991;
                let m967 = v2;
                if m967 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m967 - 1;
                    let v5: u64 = f205(v0.clone(), v1, {
                        let a971 = {
                            let a969 = {
                                let a968 = v0.clone();
                                length_bytes(a968)
                            };
                            let a970 = v1;
                            nat_sub(a969, a970)
                        };
                        let a972 = 1u64;
                        nat_add(a971, a972)?
                    })?;
                    let v6: Bytes = {
                        let a975 = v3;
                        let a976 = f46(v0.clone(), v1, {
                            let a973 = v5;
                            let a974 = v1;
                            nat_sub(a973, a974)
                        });
                        append_bytes(a975, a976)
                    };
                    let v7: u64 = f116(v0.clone(), v5)?;
                    let a977 = v7;
                    let a978 = 34u64;
                    if nat_eq(a977, a978) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a979 = v5;
                            let a980 = 1u64;
                            nat_add(a979, a980)?
                        }, v6))))
                    } else {
                        let a981 = v7;
                        let a982 = 92u64;
                        if nat_eq(a981, a982) {
                            let m983 = f316(v0.clone(), v5, v6)?;
                            match m983 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a986 = {
                                        let (_, h984) = {
                                            let (_, h985) = v8.clone();
                                            h985
                                        };
                                        h984
                                    };
                                    let a987 = 0u64;
                                    if nat_eq(a986, a987) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h988, _) = v8;
                                            h988
                                        }, (v4, {
                                            let (h989, _) = {
                                                let (_, h990) = v8;
                                                h990
                                            };
                                            h989
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
                Ok(next) => { m991 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: u64) -> R<u64> {
    let (h995, _) = f317(Fn3::F318(v0.clone()), {
        let a997 = {
            let a996 = v0;
            length_bytes(a996)
        };
        let a998 = 1u64;
        nat_add(a997, a998)?
    }, v1)?;
    Ok(h995)
}

pub fn f208(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f209(v0: Bytes) -> R<u64> {
    let (h999, _) = f317(Fn3::F319(v0.clone()), {
        let a1001 = {
            let a1000 = v0;
            length_bytes(a1000)
        };
        let a1002 = 1u64;
        nat_add(a1001, a1002)?
    }, 0u64)?;
    Ok(h999)
}

pub fn f210(v0: Bytes) -> R<Option<u64>> {
    if f320(v0.clone())? {
        Ok(Some({
            let (_, h1003) = {
                let (h1004, _) = f321(Fn4::F322(v0.clone()), {
                    let a1005 = v0;
                    length_bytes(a1005)
                }, (0u64, 0u64))?;
                h1004
            };
            h1003
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f211(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a1007 = {
            let a1006 = v1.clone();
            length_bytes(a1006)
        };
        let a1008 = f323(v1)?;
        nat_sub(a1007, a1008)
    });
    let v5: Bytes = {
        let a1009 = v0;
        let a1010 = v4.clone();
        append_bytes(a1009, a1010)
    };
    let v6: u64 = f209(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a1012 = {
            let a1011 = v5;
            length_bytes(a1011)
        };
        let a1013 = v6;
        nat_sub(a1012, a1013)
    });
    let v8: u64 = {
        let a1014 = v4;
        length_bytes(a1014)
    };
    let v9: u64 = {
        let a1015 = v7.clone();
        length_bytes(a1015)
    };
    let a1016 = v9;
    let a1017 = 0u64;
    if nat_eq(a1016, a1017) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f208(v2, {
            let a1018 = v3;
            let a1019 = v8;
            nat_lt(a1018, a1019)
        }) {
            let v10: u64 = if v2 {
                let a1020 = v3;
                let a1021 = v8;
                nat_add(a1020, a1021)?
            } else {
                let a1022 = v8;
                let a1023 = v3;
                nat_sub(a1022, a1023)
            };
            let v11: u64 = f323(v7.clone())?;
            let a1024 = v10;
            let a1025 = v11;
            if nat_le(a1024, a1025) {
                Ok(Some((f46(v7, 0u64, {
                    let a1026 = v9;
                    let a1027 = v10;
                    nat_sub(a1026, a1027)
                }), 0u64)))
            } else {
                let a1030 = {
                    let a1028 = v10;
                    let a1029 = v11;
                    nat_sub(a1028, a1029)
                };
                let a1031 = 19u64;
                if nat_le(a1030, a1031) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1032 = v9;
                        let a1033 = v11;
                        nat_sub(a1032, a1033)
                    }), {
                        let a1034 = v10;
                        let a1035 = v11;
                        nat_sub(a1034, a1035)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1036 = v3;
                let a1037 = v8;
                nat_sub(a1036, a1037)
            };
            let a1040 = {
                let a1038 = v9;
                let a1039 = v12;
                nat_add(a1038, a1039)?
            };
            let a1041 = 19u64;
            if nat_le(a1040, a1041) {
                Ok(Some(({
                    let a1042 = v7;
                    let a1043 = f324(v12);
                    append_bytes(a1042, a1043)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f212(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1044 = 0u64;
        let a1045 = v1;
        if nat_lt(a1044, a1045) {
            let a1046 = v1;
            let a1047 = 9223372036854775808u64;
            nat_le(a1046, a1047)
        } else {
            false
        }
    } else {
        let a1048 = v1;
        let a1049 = 9223372036854775807u64;
        nat_le(a1048, a1049)
    }
}

pub fn f213(v0: Bytes, v1: u64) -> Option<u8> {
    let a1050 = v0;
    let a1051 = v1;
    index_bytes(a1050, a1051)
}

pub fn f214(v0: Fn2, p1055: Option<Bytes>, p1056: List<Bytes>) -> Option<Bytes> {
    let m1054 = (p1055, p1056);
    {
        let mut m1054 = m1054;
        loop {
            let step = {
                let (v1, v2) = m1054;
                let m1052 = v2;
                match m1052.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1053: Fn2 = v0.clone();
                            c1053.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1054 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1057 = v1;
    match m1057 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1060 = {
                    let a1058 = v3;
                    let a1059 = v0;
                    append_bytes(a1058, a1059)
                };
                let a1061 = v2;
                append_bytes(a1060, a1061)
            })
        }
    }
}

pub fn f216() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f217() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f218() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f219(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1062 = v0;
    match m1062.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f219(v2))
        }
    }
}

pub fn f220(v0: List<Adt19>) -> List<(Bytes, Bytes)> {
    let m1063 = v0;
    match m1063.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt19::C0(h1064, _, _) => {
                    h1064
                }
            }, match v1 {
                Adt19::C0(_, _, h1065) => {
                    h1065
                }
            }), f220(v2))
        }
    }
}

pub fn f221(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt19>> {
    let v3: List<(Bytes, Bytes)> = f325(v1);
    let v4: List<(Bytes, Bytes)> = f326(v0, v2);
    f327(v3.clone(), f162(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f222(p1072: List<Bytes>, v1: List<(Bytes, Bytes)>, p1073: u64) -> R<List<Bytes>> {
    let m1071 = (p1072, p1073);
    {
        let mut m1071 = m1071;
        loop {
            let step = {
                let (v0, v2) = m1071;
                let m1066 = v2;
                if m1066 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1066 - 1;
                    let v4: List<Bytes> = f153(f126(v0.clone(), f328(v0.clone(), v1.clone())))?;
                    let a1069 = {
                        let a1067 = v4.clone();
                        length_list(a1067)
                    };
                    let a1070 = {
                        let a1068 = v0.clone();
                        length_list(a1068)
                    };
                    if nat_eq(a1069, a1070) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1071 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: List<Adt14>) -> List<Bytes> {
    let m1074 = v0;
    match m1074.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1075 = f94(v1, f8(Str::lit("std")));
            match m1075 {
                None => {
                    f223(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f224(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f223(v2))
                    } else {
                        f223(v2)
                    })
                }
            }
        }
    }
}

pub fn f224(v0: Adt14, v1: Bytes) -> bool {
    f51({
        let a1077 = {
            let a1076 = f58(v0, v1);
            length_bytes(a1076)
        };
        let a1078 = 0u64;
        nat_eq(a1077, a1078)
    })
}

pub fn f225(v0: Adt15, v1: Adt14) -> bool {
    if f329(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f329(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f226(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m1079 = v0;
    match m1079.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f203(v2)?;
            let a1081 = {
                let a1080 = v4.clone();
                length_bytes(a1080)
            };
            let a1082 = 0u64;
            if nat_eq(a1081, a1082) {
                f226(v3, v1)
            } else {
                let m1083 = f330(v4)?;
                match m1083 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f331(v5.clone())
                        } else {
                            f332(v5.clone())
                        } {
                            let m1084 = f226(v3, v1)?;
                            match m1084 {
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

pub fn f227(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m1085 = v0;
    match m1085.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f227(v3, v1))
        }
    }
}

pub fn f228(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1086 = v1;
    match m1086.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f249(v0.clone(), v2), f228(v0, v3))
            } else {
                f228(v0, v3)
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1087 = v1;
    match m1087.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f333(v3.clone())
            } else {
                f51(f333(v3.clone()))
            } {
                List::cons(f249(v0.clone(), v3), f229(v0, v4, v2))
            } else {
                f229(v0, v4, v2)
            }
        }
    }
}

pub fn f230(p1090: List<Bytes>) -> R<Option<Bytes>> {
    let m1089 = p1090;
    {
        let mut m1089 = m1089;
        loop {
            let step = {
                let v0 = m1089;
                let m1088 = v0;
                match m1088.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f334(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f203(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1089 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1091 = v1;
    match m1091.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f96(f64(v0.clone(), v2))?, f231(v0, v3)?))
        }
    }
}

pub fn f232(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1092 = v0.clone();
        length_bytes(a1092)
    };
    let v3: u64 = {
        let a1093 = v1.clone();
        length_bytes(a1093)
    };
    let v4: u64 = {
        let (h1094, _) = f317(Fn3::F335(v1.clone(), v3, v0.clone(), v2), {
            let a1095 = v2;
            let a1096 = 1u64;
            nat_add(a1095, a1096)?
        }, 0u64)?;
        h1094
    };
    if f336(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f233() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f234(v0: Adt16) -> R<Adt18> {
    Ok(Adt18::C0(f337(match v0.clone() {
        Adt16::C0(h1097, _, _, _, _) => {
            h1097
        }
    }, 0u64)?, f337(match v0.clone() {
        Adt16::C0(_, h1098, _, _, _) => {
            h1098
        }
    }, 0u64)?, f337(match v0.clone() {
        Adt16::C0(_, _, h1099, _, _) => {
            h1099
        }
    }, 0u64)?, f337(match v0 {
        Adt16::C0(_, _, _, h1100, _) => {
            h1100
        }
    }, 0u64)?))
}

pub fn f235(v0: Adt16) -> List<Bytes> {
    f126(match v0.clone() {
        Adt16::C0(h1101, _, _, _, _) => {
            h1101
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, h1102, _, _) => {
            h1102
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, _, h1103, _) => {
            h1103
        }
    }, match v0 {
        Adt16::C0(_, _, _, _, h1104) => {
            h1104
        }
    })))
}

pub fn f236(v0: Adt18, p1110: List<u64>, p1111: List<Adt20>) -> Option<u64> {
    let m1109 = (p1110, p1111);
    {
        let mut m1109 = m1109;
        loop {
            let step = {
                let (v1, v2) = m1109;
                let m1105 = v2;
                match m1105.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f338(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1106) => {
                                h1106
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1107, _) => {
                                    h1107
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1108, _) => {
                                    h1108
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Adt16, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m1112 = v1;
    match m1112.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f339(v0.clone(), v3)?), f237(v0, v4, {
                let a1113 = v2;
                let a1114 = 1u64;
                nat_add(a1113, a1114)?
            })?))
        }
    }
}

pub fn f238(p1118: List<Bytes>, p1119: u64) -> Bytes {
    let m1117 = (p1118, p1119);
    {
        let mut m1117 = m1117;
        loop {
            let step = {
                let (v0, v1) = m1117;
                let m1115 = v0;
                match m1115.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1116 = v1;
                        if m1116 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1116 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
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

pub fn f239(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f340(v0.clone(), v1.clone()) {
        if f340(v1, v0) {
            f341(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f240(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1120 = v0;
    match m1120.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f342(v2, v1.clone())?, f240(v3, v1)?))
        }
    }
}

pub fn f241(v0: List<Adt14>) -> List<u64> {
    let m1121 = v0;
    match m1121.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f343(v1), f241(v2))
        }
    }
}

pub fn f242(p1124: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1123 = p1124;
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let v0 = m1123;
                let m1122 = v0;
                match m1122.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f344(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f243(p1127: List<Bytes>, p1128: List<Bytes>) -> Option<Bytes> {
    let m1126 = (p1127, p1128);
    {
        let mut m1126 = m1126;
        loop {
            let step = {
                let (v0, v1) = m1126;
                let m1125 = v0;
                match m1125.uncons() {
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
                Ok(next) => { m1126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt23 = f345(v0.clone(), 0u64, {
        let a1129 = v0;
        length_bytes(a1129)
    }, Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt23 = {
        let a1132 = {
            let a1131 = match v1.clone() {
                Adt23::C0(_, _, h1130, _, _) => {
                    h1130
                }
            };
            length_bytes(a1131)
        };
        let a1133 = 0u64;
        if if nat_eq(a1132, a1133) {
            let m1135 = match v1.clone() {
                Adt23::C0(_, _, _, h1134, _) => {
                    h1134
                }
            };
            match m1135.uncons() {
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
            f346(v1)
        }
    };
    Ok(f347(match v4 {
        Adt23::C0(_, _, _, _, h1136) => {
            h1136
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f245(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1137 = v0;
    match m1137.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1138 = v1;
            match m1138.uncons() {
                None => {
                    f245(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f245(v2))
                }
            }
        }
    }
}

pub fn f246(v0: List<Bytes>) -> List<Bytes> {
    f121(Fn1::F348, List::<Bytes>::nil(), v0)
}

pub fn f247(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f349(v0, v1)
}

pub fn f248(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f350(f351(f352(v0))?))
}

pub fn f249(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f250(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1139 = f57(v0, v2)?;
    match m1139 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f353(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1140 = v4.clone();
            match m1140.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1142 = f93(f94({
                        let m1141 = f94(v3.clone(), f8(Str::lit("source")));
                        match m1141 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1142 {
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

pub fn f251(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1143 = v0;
    match m1143.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f356(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1147 = f44(f46(v1.clone(), 6u64, {
                    let a1145 = {
                        let a1144 = v1;
                        length_bytes(a1144)
                    };
                    let a1146 = 6u64;
                    nat_sub(a1145, a1146)
                }), 34u8)?;
                match m1147.uncons() {
                    None => {
                        f251(v2)
                    }
                    Some((v3, v4)) => {
                        let m1148 = v4;
                        match m1148.uncons() {
                            None => {
                                f251(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f251(v2)?))
                            }
                        }
                    }
                }
            } else {
                f251(v2)
            }
        }
    }
}

pub fn f252(p1151: List<Bytes>, p1152: List<Bytes>) -> List<Bytes> {
    let m1150 = (p1151, p1152);
    {
        let mut m1150 = m1150;
        loop {
            let step = {
                let (v0, v1) = m1150;
                let m1149 = v0;
                match m1149.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f254(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1153 = v1;
    match m1153.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f357(v0.clone(), v3)?;
            if f52(f358(f283(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f359(v5)?, f254(v0, v4, v2)?))
            } else {
                f254(v0, v4, v2)
            }
        }
    }
}

pub fn f255(v0: Adt1, p1157: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1156 = p1157;
    {
        let mut m1156 = m1156;
        loop {
            let step = {
                let v1 = m1156;
                let m1154 = v1;
                match m1154.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f357(v0.clone(), v3.clone())?;
                        if if f52(f358(f283(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f282({
                                let m1155 = f280(v5.clone(), f8(Str::lit("schema")));
                                match m1155 {
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
                Ok(next) => { m1156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f151(match v0.clone() {
        Adt1::C0(_, h1158, _, _) => {
            h1158
        }
    }, f267())) {
        Ok(Adt12::C1(List::cons(f267(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f159(f153(f360(f155(match v0.clone() {
            Adt1::C0(_, h1159, _, _) => {
                h1159
            }
        }, f267())))?, v1);
        let m1160 = f56(v0.clone(), List::cons(f161(), List::cons(f218(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1160.uncons() {
            None => {
                let v5: (bool, Bytes) = f361(v0.clone(), v2)?;
                let (h1161, _) = v5;
                if h1161 {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h1162) = v5;
                        h1162
                    }))
                } else {
                    let m1163 = f125(v0.clone());
                    match m1163.uncons() {
                        None => {
                            let m1164 = f127(v0.clone())?;
                            match m1164.uncons() {
                                None => {
                                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h1165) = v5;
                                        h1165
                                    }, List::cons(f7(f153(f362(f128(v0.clone())?, f136(v0.clone(), f137(v0.clone())?)?))?), List::cons(f363(v0)?, List::<Bytes>::nil()))))))
                                }
                                Some((v8, v9)) => {
                                    Ok(Adt12::C0(List::cons(v8, v9)))
                                }
                            }
                        }
                        Some((v6, v7)) => {
                            Ok(Adt12::C1(List::cons(v6, v7)))
                        }
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt12::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f257(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f364(f97(v0, f161())?)?)))
}

pub fn f258() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f259(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1166 = v0;
    match m1166.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f260(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt19>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m1167 = v1;
    match m1167.uncons() {
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
                let a1169 = {
                    let a1168 = v7.clone();
                    length_list(a1168)
                };
                let a1170 = 0u64;
                if nat_eq(a1169, a1170) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f123(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f260(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f261(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1171 = v1;
    match m1171.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f224(v2.clone(), f8(Str::lit("domain"))) {
                f368(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1172 = f94(v2.clone(), f8(Str::lit("role")));
                    match m1172 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f369(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1173 = f94(v2.clone(), f8(Str::lit("origin")));
                    match m1173 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1174 = f94(v2, f8(Str::lit("evidence")));
                    match m1174 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f261(v0, v3))
            } else {
                f261(v0, v3)
            }
        }
    }
}

pub fn f262(v0: List<Adt14>) -> List<Bytes> {
    let m1175 = v0;
    match m1175.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f262(v2))
        }
    }
}

pub fn f263(v0: List<Adt14>) -> List<Bytes> {
    let m1176 = v0;
    match m1176.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f370(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f263(v2))
        }
    }
}

pub fn f264(p1182: List<Bytes>) -> R<bool> {
    let m1181 = p1182;
    {
        let mut m1181 = m1181;
        loop {
            let step = {
                let v0 = m1181;
                let m1177 = v0;
                match m1177.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1179 = {
                            let a1178 = f203(v1)?;
                            length_bytes(a1178)
                        };
                        let a1180 = 0u64;
                        if nat_eq(a1179, a1180) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
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

pub fn f265(v0: List<Bytes>) -> R<bool> {
    if f371(f297(v0.clone())?) {
        Ok(f51({
            let a1184 = {
                let a1183 = f372(f297(v0)?);
                length_list(a1183)
            };
            let a1185 = 0u64;
            nat_eq(a1184, a1185)
        }))
    } else {
        Ok(false)
    }
}

pub fn f266(v0: List<Bytes>) -> R<bool> {
    if f371(f275(v0.clone(), f8(Str::lit("requirement")))?) {
        if f371(f275(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1187 = {
                    let a1186 = f227(f372(f275(v0.clone(), f8(Str::lit("requirement")))?), f372(f275(v0, f8(Str::lit("admission")))?));
                    length_list(a1186)
                };
                let a1188 = 0u64;
                nat_eq(a1187, a1188)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f267() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f268(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f269() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f270(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1189 = v0;
    match m1189.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f203(v1)?;
            let a1191 = {
                let a1190 = v3.clone();
                length_bytes(a1190)
            };
            let a1192 = 0u64;
            if nat_eq(a1191, a1192) {
                f270(v2)
            } else {
                let m1193 = f330(v3.clone())?;
                match m1193 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f373(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1194 = f270(v2)?;
                            match m1194 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f374(v5), {
                                        let a1195 = v3;
                                        let a1196 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1195, a1196)
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

pub fn f271() -> Bytes {
    let a1197 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1197)
}

pub fn f272(p1202: List<Bytes>) -> R<Option<Bytes>> {
    let m1201 = p1202;
    {
        let mut m1201 = m1201;
        loop {
            let step = {
                let v0 = m1201;
                let m1198 = v0;
                match m1198.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1199 = f330(f203(v1)?)?;
                        match m1199 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1200 = f94(v3.clone(), f8(Str::lit("source")));
                                match m1200 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f375(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Bytes) -> Bytes {
    let a1207 = {
        let a1203 = Str::lit("binding ");
        utf8_encode(a1203)
    };
    let a1208 = {
        let a1205 = v0;
        let a1206 = {
            let a1204 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1204)
        };
        append_bytes(a1205, a1206)
    };
    append_bytes(a1207, a1208)
}

pub fn f274(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1209 = v0;
    match m1209.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f203(v1)?;
            let a1211 = {
                let a1210 = v3.clone();
                length_bytes(a1210)
            };
            let a1212 = 0u64;
            if nat_eq(a1211, a1212) {
                f274(v2)
            } else {
                let m1213 = f43(v3.clone())?;
                match m1213 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f376(v4.clone()) {
                            let m1214 = f274(v2)?;
                            match m1214 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f253(), List::cons(f8(Str::lit("/")), List::cons(f377(f283(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1215 = v3;
                                        let a1216 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1215, a1216)
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

pub fn f275(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m1217 = v0;
    match m1217.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f203(v2)?;
            let a1219 = {
                let a1218 = v4.clone();
                length_bytes(a1218)
            };
            let a1220 = 0u64;
            if nat_eq(a1219, a1220) {
                f275(v3, v1)
            } else {
                let m1221 = f330(v4)?;
                match m1221 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f378(v5.clone());
                        let a1223 = {
                            let a1222 = v6.clone();
                            length_bytes(a1222)
                        };
                        let a1224 = 0u64;
                        if nat_eq(a1223, a1224) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m1225 = f275(v3, v1.clone())?;
                            match m1225 {
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

pub fn f276(p1228: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1227 = p1228;
    {
        let mut m1227 = m1227;
        loop {
            let step = {
                let v0 = m1227;
                let m1226 = v0;
                match m1226.uncons() {
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
                Ok(next) => { m1227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f278(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f101(Adt14::C3(std::rc::Rc::new(f102(v1))))?))
}

pub fn f279(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f101(Adt14::C3(std::rc::Rc::new(f379(f153(f126(v1, f150(f97(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f280(p1234: Adt2, v1: Bytes) -> Option<Adt2> {
    let m1233 = p1234;
    {
        let mut m1233 = m1233;
        loop {
            let step = {
                let v0 = m1233;
                let m1229 = v0;
                match m1229 {
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
                    Adt2::C7(r1230) => {
                        let v5 = (*r1230).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C8 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(v2, r1231, r1232) => {
                        let v3 = (*r1231).clone();
                        let v4 = (*r1232).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
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

pub fn f281(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1235 = f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1235.uncons() {
        None => {
            let m1237 = f276(f132(List::cons(Adt14::C4(std::rc::Rc::new({
                let m1236 = f380(v4.clone());
                match m1236 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1237 {
                None => {
                    f381(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f277(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f282(v0: Adt2) -> R<Bytes> {
    let a1238 = f382(v0)?;
    let a1239 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1238, a1239))
}

pub fn f283(v0: Adt2, v1: Bytes) -> Bytes {
    let m1240 = f383(f280(v0, v1));
    match m1240 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f284(v0: List<Adt2>) -> List<Bytes> {
    let m1241 = v0;
    match m1241.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1242 = f383(Some(v1));
            match m1242 {
                None => {
                    f284(v2)
                }
                Some(v3) => {
                    List::cons(v3, f284(v2))
                }
            }
        }
    }
}

pub fn f285(v0: Option<Adt2>) -> List<Adt2> {
    let m1243 = v0;
    match m1243 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f384(v1)
        }
    }
}

pub fn f286() -> Bytes {
    f385()
}

pub fn f287(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f292(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f386(v1, v4)?)?), List::cons(f279(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f288(v0: Bytes) -> R<Bytes> {
    Ok(f387(f388(v0)?))
}

pub fn f289(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1244 = v0;
    match m1244.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f290() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f291(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f354(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f292(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f389(f158(f96(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f390(f391(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f293(p1247: List<Adt14>) -> bool {
    let m1246 = p1247;
    {
        let mut m1246 = m1246;
        loop {
            let step = {
                let v0 = m1246;
                let m1245 = v0;
                match m1245.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f149(f392(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: List<Bytes>) -> List<Bytes> {
    let m1248 = v0;
    match m1248.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f393(v1.clone()) {
                List::<Bytes>::nil()
            } else {
                List::cons(v1, f294(v2))
            }
        }
    }
}

pub fn f295(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: List<Bytes>) -> R<Adt12> {
    let m1249 = f380(v4);
    match m1249 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m1250 = f276(f132(List::cons(Adt14::C4(std::rc::Rc::new(v5.clone())), List::<Adt14>::nil())), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1250 {
                None => {
                    f394(v0, v2.clone(), Adt14::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f379(v3)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt14::C2(f288(v1.clone())?)), std::rc::Rc::new(Adt14::C5))))))), std::rc::Rc::new(v5)), f395(v2, f396(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f277(v6))
                }
            }
        }
    }
}

pub fn f296(p1253: List<Bytes>) -> List<Bytes> {
    let m1252 = p1253;
    {
        let mut m1252 = m1252;
        loop {
            let step = {
                let v0 = m1252;
                let m1251 = v0;
                match m1251.uncons() {
                    None => {
                        Err::<List<Bytes>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v1, v2)) => {
                        if f393(v1.clone()) {
                            Err::<List<Bytes>, List<Bytes>>(List::cons(v1, v2))
                        } else {
                            Ok::<List<Bytes>, List<Bytes>>(v2)
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

pub fn f297(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m1254 = v0;
    match m1254.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f203(v1)?;
            let a1256 = {
                let a1255 = v3.clone();
                length_bytes(a1255)
            };
            let a1257 = 0u64;
            if nat_eq(a1256, a1257) {
                f297(v2)
            } else {
                let m1258 = f330(v3)?;
                match m1258 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f224(v4.clone(), f8(Str::lit("agent"))) {
                            if f224(v4.clone(), f8(Str::lit("at"))) {
                                if f224(v4.clone(), f8(Str::lit("quote"))) {
                                    f397(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1259 = f297(v2)?;
                            match m1259 {
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

pub fn f298() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f299(p1262: List<Adt14>, v1: Bytes, v2: Bytes) -> bool {
    let m1261 = p1262;
    {
        let mut m1261 = m1261;
        loop {
            let step = {
                let v0 = m1261;
                let m1260 = v0;
                match m1260.uncons() {
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
                Ok(next) => { m1261 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt14 {
    Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt14::C2(v2)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt14::C2(v0)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5))))))))))))
}

pub fn f301(v0: u64) -> bool {
    let a1263 = v0;
    let a1264 = 127u64;
    nat_le(a1263, a1264)
}

pub fn f302(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1265, _) = v1.clone();
        h1265
    };
    let v3: u64 = {
        let (h1266, _) = {
            let (_, h1267) = v1.clone();
            h1267
        };
        h1266
    };
    let v4: List<Bytes> = {
        let (_, h1268) = {
            let (_, h1269) = v1;
            h1269
        };
        h1268
    };
    let m1270 = f213(v0.clone(), v2);
    match m1270 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1271 = v5;
            let a1272 = 10u8;
            if equal(a1271, a1272) {
                Ok(Some(({
                    let a1273 = v2;
                    let a1274 = 1u64;
                    nat_add(a1273, a1274)?
                }, ({
                    let a1275 = v2;
                    let a1276 = 1u64;
                    nat_add(a1275, a1276)?
                }, List::cons(f398(f46(v0, v3, {
                    let a1277 = v2;
                    let a1278 = v3;
                    nat_sub(a1277, a1278)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1279 = v2;
                    let a1280 = 1u64;
                    nat_add(a1279, a1280)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f303(v0: Adt14) -> List<Adt14> {
    let m1281 = v0;
    match m1281 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1282) => {
            let v3 = (*r1282).clone();
            f303(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1283, r1284) => {
            let v1 = (*r1283).clone();
            let v2 = (*r1284).clone();
            List::cons(v1, f303(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f304(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1285 = v0;
    match m1285 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r1286) => {
            let v10 = (*r1286).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r1287) => {
            let v5 = (*r1287).clone();
            Adt14::C4(std::rc::Rc::new(f304(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1288, r1289) => {
            let v11 = (*r1288).clone();
            let v12 = (*r1289).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r1290, r1291) => {
            let v3 = (*r1290).clone();
            let v4 = (*r1291).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt14::C7(v2, std::rc::Rc::new(f399(v3, v1.clone())), std::rc::Rc::new(f304(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f304(v4, v1)))
            }
        }
    }
}

pub fn f305(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1292 = 0u64;
        let a1293 = v1;
        nat_lt(a1292, a1293)
    } else {
        false
    } {
        let a1294 = Bytes::lit(&[0x2d]);
        let a1295 = f400(v1)?;
        Ok(append_bytes(a1294, a1295))
    } else {
        f400(v1)
    }
}

pub fn f306(v0: Bytes) -> R<Bytes> {
    let a1301 = {
        let a1299 = Bytes::lit(&[0x22]);
        let a1300 = {
            let (_, h1296) = {
                let (h1297, _) = f401(Fn5::F402(v0.clone()), {
                    let a1298 = v0;
                    length_bytes(a1298)
                }, (0u64, Bytes::lit(&[])))?;
                h1297
            };
            h1296
        };
        append_bytes(a1299, a1300)
    };
    let a1302 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1301, a1302))
}

pub fn f307(v0: List<Bytes>) -> Bytes {
    let a1305 = {
        let a1303 = Bytes::lit(&[0x5b]);
        let a1304 = f123(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1303, a1304)
    };
    let a1306 = Bytes::lit(&[0x5d]);
    append_bytes(a1305, a1306)
}

pub fn f308(v0: Adt14) -> R<List<Bytes>> {
    let m1307 = v0;
    match m1307 {
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
        Adt14::C6(r1308, r1309) => {
            let v7 = (*r1308).clone();
            let v8 = (*r1309).clone();
            Ok(List::cons(f195(v7)?, f308(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f309(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f403(Fn6::F404, List::<(Bytes, Bytes)>::nil(), f405(v0)?);
    let v2: List<Bytes> = f406(Fn7::F407, List::<Bytes>::nil(), v1)?;
    let a1312 = {
        let a1310 = Bytes::lit(&[0x7b]);
        let a1311 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1310, a1311)
    };
    let a1313 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1312, a1313))
}

pub fn f310(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1314 = v0;
    match m1314 {
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
        Adt14::C7(v9, r1315, r1316) => {
            let v10 = (*r1315).clone();
            let v11 = (*r1316).clone();
            Ok(List::cons((v9, f195(v10)?), f310(v11)?))
        }
    }
}

pub fn f311() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f312(v0: Adt13) -> Bytes {
    let m1317 = v0;
    match m1317 {
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

pub fn f313(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f408(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f408(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f408(f8(Str::lit("name")), f409(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f408(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f408(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f408(f8(Str::lit("statement")), f410(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f408(f8(Str::lit("media_type")), f411(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f408(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f408(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f408(f8(Str::lit("url")), f198(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f314(v0: Bytes) -> R<u64> {
    let (h1318, _) = f317(Fn3::F412(v0.clone()), {
        let a1319 = v0.clone();
        length_bytes(a1319)
    }, {
        let a1320 = v0;
        length_bytes(a1320)
    })?;
    Ok(h1318)
}

pub fn f315(v0: Bytes, v1: u64) -> R<u64> {
    let (h1321, _) = f317(Fn3::F413(v1, v0), v1, 0u64)?;
    Ok(h1321)
}

pub fn f316(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f116(v0.clone(), {
        let a1322 = v1;
        let a1323 = 1u64;
        nat_add(a1322, a1323)?
    })?;
    let v4: Bytes = {
        let a1324 = v3;
        let a1325 = 117u64;
        if nat_eq(a1324, a1325) {
            f414(v0, v1)?
        } else {
            f415(v3)
        }
    };
    let a1326 = v4.clone();
    let a1327 = Bytes::lit(&[]);
    if equal(a1326, a1327) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1328 = v3;
            let a1329 = 117u64;
            if nat_eq(a1328, a1329) {
                let a1330 = v1;
                let a1331 = 6u64;
                nat_add(a1330, a1331)?
            } else {
                let a1332 = v1;
                let a1333 = 2u64;
                nat_add(a1332, a1333)?
            }
        }, ({
            let a1334 = v2;
            let a1335 = v4;
            append_bytes(a1334, a1335)
        }, 0u64))))
    }
}

pub fn f317(v0: Fn3, p1340: u64, p1341: u64) -> R<(u64, bool)> {
    let m1339 = (p1340, p1341);
    {
        let mut m1339 = m1339;
        loop {
            let step = {
                let (v1, v2) = m1339;
                let m1336 = v1;
                if m1336 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1336 - 1;
                    let m1338 = {
                        let c1337: Fn3 = v0.clone();
                        c1337.apply(v2)?
                    };
                    match m1338 {
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
                Ok(next) => { m1339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f416(v0, v1)
}

pub fn f319(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f417(v0, v1)
}

pub fn f320(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1342 = v0.clone();
        length_bytes(a1342)
    };
    let a1343 = 0u64;
    let a1344 = v1;
    if nat_lt(a1343, a1344) {
        let a1345 = f418(v0.clone())?;
        let a1346 = v1;
        if nat_eq(a1345, a1346) {
            let a1347 = 1u64;
            let a1348 = v1;
            if nat_lt(a1347, a1348) {
                let m1351 = {
                    let a1349 = v0;
                    let a1350 = 0u64;
                    index_bytes(a1349, a1350)
                };
                match m1351 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1352 = v2;
                        let a1353 = 48u8;
                        Ok(!equal(a1352, a1353))
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

pub fn f321(v0: Fn4, p1358: u64, p1359: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1357 = (p1358, p1359);
    {
        let mut m1357 = m1357;
        loop {
            let step = {
                let (v1, v2) = m1357;
                let m1354 = v1;
                if m1354 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1354 - 1;
                    let m1356 = {
                        let c1355: Fn4 = v0.clone();
                        c1355.apply(v2)?
                    };
                    match m1356 {
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
                Ok(next) => { m1357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f419(v0, v1)
}

pub fn f323(v0: Bytes) -> R<u64> {
    let (h1360, _) = f317(Fn3::F420(v0.clone()), {
        let a1362 = {
            let a1361 = v0;
            length_bytes(a1361)
        };
        let a1363 = 1u64;
        nat_add(a1362, a1363)?
    }, 0u64)?;
    Ok(h1360)
}

pub fn f324(v0: u64) -> Bytes {
    let m1364 = v0;
    if m1364 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1364 - 1;
        let a1365 = f8(Str::lit("0"));
        let a1366 = f324(v1);
        append_bytes(a1365, a1366)
    }
}

pub fn f325(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1367 = v0;
    match m1367.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1368 = f94(v1, f8(Str::lit("std")));
            match m1368 {
                None => {
                    f325(v2)
                }
                Some(v3) => {
                    if f224(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f325(v2))
                    } else {
                        f325(v2)
                    }
                }
            }
        }
    }
}

pub fn f326(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1369 = v1;
    match m1369.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt15::C0(h1370, _, _) => {
                    h1370
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1371, _) => {
                        h1371
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f326(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt15::C0(h1372, _, _) => {
                        h1372
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1373, _) => {
                            h1373
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f326(v0, v3))
                } else {
                    f326(v0, v3)
                }
            }
        }
    }
}

pub fn f327(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt19>> {
    let m1374 = v0;
    match m1374.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v4, v5)) => {
            if if f421(v4.clone(), v2.clone()) {
                true
            } else {
                f421(v4.clone(), v3.clone())
            } {
                f327(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt19::C0({
                    let (h1375, _) = v4.clone();
                    h1375
                }, {
                    let a1378 = {
                        let a1377 = f153(f422({
                            let (h1376, _) = v4.clone();
                            h1376
                        }, v1.clone()))?;
                        length_list(a1377)
                    };
                    let a1379 = 1u64;
                    if nat_eq(a1378, a1379) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1380) = v4.clone();
                    h1380
                }), f327(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f328(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1381 = v1;
    match m1381.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1382, _) = v2.clone();
                h1382
            }, v0.clone()) {
                List::cons({
                    let (_, h1383) = v2;
                    h1383
                }, f328(v0, v3))
            } else {
                if f52({
                    let (_, h1384) = v2;
                    h1384
                }, v0.clone()) {
                    List::cons({
                        let (h1385, _) = v2;
                        h1385
                    }, f328(v0, v3))
                } else {
                    f328(v0, v3)
                }
            }
        }
    }
}

pub fn f329(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt15::C0(h1386, _, _) => {
            h1386
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1387, _) => {
                h1387
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt15::C0(_, _, h1388) => {
                    h1388
                }
            }) {
                true
            } else {
                if f224(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1389, _) => {
                            h1389
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f330(v0: Bytes) -> R<Option<Adt14>> {
    let m1390 = f43(v0)?;
    match m1390 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v1) => {
            Ok(f92(v1))
        }
    }
}

pub fn f331(v0: Adt14) -> bool {
    if f224(v0.clone(), f8(Str::lit("domain"))) {
        if f224(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f423()) {
                if f424(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f424(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f424(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f424(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f332(v0: Adt14) -> bool {
    if f224(v0.clone(), f8(Str::lit("subject"))) {
        if f224(v0.clone(), f8(Str::lit("object"))) {
            if f224(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f425())
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

pub fn f333(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f334(v0: Bytes) -> R<bool> {
    let v1: Bytes = f203(v0)?;
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
        let a1392 = {
            let a1391 = v1;
            length_bytes(a1391)
        };
        let a1393 = v2;
        nat_sub(a1392, a1393)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1394 = f232(v3.clone(), f8(Str::lit(")")))?;
        match m1394 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1395 = v4;
                    let a1396 = 1u64;
                    nat_add(a1395, a1396)?
                }, {
                    let a1400 = {
                        let a1397 = v3;
                        length_bytes(a1397)
                    };
                    let a1401 = {
                        let a1398 = v4;
                        let a1399 = 1u64;
                        nat_add(a1398, a1399)?
                    };
                    nat_sub(a1400, a1401)
                })
            }
        }
    } else {
        v3
    };
    let a1402 = 0u64;
    let a1403 = v2;
    if nat_lt(a1402, a1403) {
        f426(v5)
    } else {
        Ok(false)
    }
}

pub fn f335(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f336(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1406 = {
            let a1404 = v4;
            let a1405 = v1;
            nat_add(a1404, a1405)?
        };
        let a1407 = v3;
        if nat_lt(a1406, a1407) {
            Ok(Some({
                let a1408 = v4;
                let a1409 = 1u64;
                nat_add(a1408, a1409)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f336(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1414 = {
        let a1411 = v0;
        let a1412 = v2;
        let a1413 = {
            let a1410 = v1.clone();
            length_bytes(a1410)
        };
        slice_bytes(a1411, a1412, a1413)
    };
    match m1414 {
        None => {
            false
        }
        Some(v3) => {
            let a1415 = v3;
            let a1416 = v1;
            equal(a1415, a1416)
        }
    }
}

pub fn f337(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1417 = v0;
    match m1417.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f337(v3, {
                let a1418 = v1;
                let a1419 = 1u64;
                nat_add(a1418, a1419)?
            })?))
        }
    }
}

pub fn f338(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1420 = v2;
    match m1420 {
        Adt21::C0(v3, v4) => {
            if f344(v3, match v0.clone() {
                Adt18::C0(h1421, _, _, _) => {
                    h1421
                }
            }) {
                f427(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f344(v5, match v0 {
                Adt18::C0(_, _, h1422, _) => {
                    h1422
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f340(v7, v1) {
                f344(v8, match v0 {
                    Adt18::C0(_, _, _, h1423) => {
                        h1423
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f339(v0: Adt16, v1: Adt14) -> R<Adt21> {
    let m1424 = f94(v1.clone(), f8(Str::lit("std")));
    match m1424 {
        None => {
            let m1429 = f93(f94(v1.clone(), f8(Str::lit("req"))));
            match m1429 {
                None => {
                    let m1431 = f94(v1, f8(Str::lit("der")));
                    match m1431 {
                        None => {
                            Ok(f428(v0))
                        }
                        Some(v5) => {
                            let m1432 = f429(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1432 {
                                None => {
                                    Ok(f428(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f240(f150(f193(f94(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt16::C0(_, _, _, _, h1433) => {
                                            h1433
                                        }
                                    })?, f342(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt16::C0(_, _, _, h1434, _) => {
                                            h1434
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f342(v4, match v0 {
                        Adt16::C0(_, _, h1430, _, _) => {
                            h1430
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1425 = f93(f94(v2.clone(), f8(Str::lit("row"))));
            match m1425 {
                None => {
                    Ok(Adt21::C0(f342(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt16::C0(h1426, _, _, _, _) => {
                            h1426
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f342(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt16::C0(h1427, _, _, _, _) => {
                            h1427
                        }
                    })?, Some(f342(v3, match v0 {
                        Adt16::C0(_, h1428, _, _, _) => {
                            h1428
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f340(p1437: List<u64>, v1: List<u64>) -> bool {
    let m1436 = p1437;
    {
        let mut m1436 = m1436;
        loop {
            let step = {
                let v0 = m1436;
                let m1435 = v0;
                match m1435.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f344(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f341(p1442: List<u64>) -> bool {
    let m1441 = p1442;
    {
        let mut m1441 = m1441;
        loop {
            let step = {
                let v0 = m1441;
                let m1438 = v0;
                match m1438.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1439 = v1;
                        let a1440 = 7u64;
                        if nat_le(a1439, a1440) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f342(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1443 = f430(v0, v1.clone(), 0u64)?;
    match m1443 {
        None => {
            let a1444 = v1;
            Ok(length_list(a1444))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f343(v0: Adt14) -> u64 {
    let m1445 = f431(f94(v0, f8(Str::lit("stratum"))));
    match m1445 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f344(v0: u64, p1450: List<u64>) -> bool {
    let m1449 = p1450;
    {
        let mut m1449 = m1449;
        loop {
            let step = {
                let v1 = m1449;
                let m1446 = v1;
                match m1446.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1447 = v2;
                        let a1448 = v0;
                        if nat_eq(a1447, a1448) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: Bytes, p1508: u64, p1509: u64, p1510: Adt23) -> R<Adt23> {
    let m1507 = (p1508, (p1509, p1510));
    {
        let mut m1507 = m1507;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1507;
                let m1451 = v2;
                if m1451 == 0 {
                    Err::<(u64, (u64, Adt23)), R<Adt23>>(Ok(v3))
                } else {
                    let v4 = m1451 - 1;
                    let v5: u64 = f432(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt23::C0(_, h1452, _, _, _) => {
                            h1452
                        }
                    } {
                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                            let a1453 = v1;
                            let a1454 = 1u64;
                            nat_add(a1453, a1454)?
                        }, (v4, Adt23::C0(true, false, match v3.clone() {
                            Adt23::C0(_, _, h1455, _, _) => {
                                h1455
                            }
                        }, match v3.clone() {
                            Adt23::C0(_, _, _, h1456, _) => {
                                h1456
                            }
                        }, match v3 {
                            Adt23::C0(_, _, _, _, h1457) => {
                                h1457
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt23::C0(h1458, _, _, _, _) => {
                                h1458
                            }
                        } {
                            let a1459 = v5;
                            let a1460 = 34u64;
                            if nat_eq(a1459, a1460) {
                                let a1463 = f432(v0.clone(), {
                                    let a1461 = v1;
                                    let a1462 = 1u64;
                                    nat_add(a1461, a1462)?
                                })?;
                                let a1464 = 34u64;
                                if nat_eq(a1463, a1464) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1465 = v1;
                                        let a1466 = 1u64;
                                        nat_add(a1465, a1466)?
                                    }, (v4, Adt23::C0(true, true, f433(match v3.clone() {
                                        Adt23::C0(_, _, h1467, _, _) => {
                                            h1467
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt23::C0(_, _, _, h1468, _) => {
                                            h1468
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1469) => {
                                            h1469
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1470 = v1;
                                        let a1471 = 1u64;
                                        nat_add(a1470, a1471)?
                                    }, (v4, Adt23::C0(false, false, match v3.clone() {
                                        Adt23::C0(_, _, h1472, _, _) => {
                                            h1472
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1473, _) => {
                                            h1473
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1474) => {
                                            h1474
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1475 = v1;
                                    let a1476 = 1u64;
                                    nat_add(a1475, a1476)?
                                }, (v4, Adt23::C0(true, false, f433(match v3.clone() {
                                    Adt23::C0(_, _, h1477, _, _) => {
                                        h1477
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt23::C0(_, _, _, h1478, _) => {
                                        h1478
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1479) => {
                                        h1479
                                    }
                                }))))
                            }
                        } else {
                            let a1480 = v5;
                            let a1481 = 34u64;
                            if nat_eq(a1480, a1481) {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1482 = v1;
                                    let a1483 = 1u64;
                                    nat_add(a1482, a1483)?
                                }, (v4, Adt23::C0(true, false, match v3.clone() {
                                    Adt23::C0(_, _, h1484, _, _) => {
                                        h1484
                                    }
                                }, match v3.clone() {
                                    Adt23::C0(_, _, _, h1485, _) => {
                                        h1485
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1486) => {
                                        h1486
                                    }
                                }))))
                            } else {
                                let a1487 = v5;
                                let a1488 = 44u64;
                                if nat_eq(a1487, a1488) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1489 = v1;
                                        let a1490 = 1u64;
                                        nat_add(a1489, a1490)?
                                    }, (v4, Adt23::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt23::C0(_, _, h1491, _, _) => {
                                            h1491
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1492, _) => {
                                            h1492
                                        }
                                    }), match v3 {
                                        Adt23::C0(_, _, _, _, h1493) => {
                                            h1493
                                        }
                                    }))))
                                } else {
                                    let a1494 = v5;
                                    let a1495 = 13u64;
                                    if nat_eq(a1494, a1495) {
                                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                            let a1496 = v1;
                                            let a1497 = 1u64;
                                            nat_add(a1496, a1497)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1498 = v5;
                                        let a1499 = 10u64;
                                        if nat_eq(a1498, a1499) {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1500 = v1;
                                                let a1501 = 1u64;
                                                nat_add(a1500, a1501)?
                                            }, (v4, f346(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1502 = v1;
                                                let a1503 = 1u64;
                                                nat_add(a1502, a1503)?
                                            }, (v4, Adt23::C0(false, false, f433(match v3.clone() {
                                                Adt23::C0(_, _, h1504, _, _) => {
                                                    h1504
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt23::C0(_, _, _, h1505, _) => {
                                                    h1505
                                                }
                                            }, match v3 {
                                                Adt23::C0(_, _, _, _, h1506) => {
                                                    h1506
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
                Ok(next) => { m1507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(v0: Adt23) -> Adt23 {
    Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f434(List::cons(match v0.clone() {
        Adt23::C0(_, _, h1511, _, _) => {
            h1511
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1512, _) => {
            h1512
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt23::C0(_, _, _, _, h1513) => {
            h1513
        }
    }))
}

pub fn f347(p1516: List<List<Bytes>>, p1517: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1515 = (p1516, p1517);
    {
        let mut m1515 = m1515;
        loop {
            let step = {
                let (v0, v1) = m1515;
                let m1514 = v0;
                match m1514.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f348(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f349(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1518 = v0.clone();
    match m1518.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1519 = v2;
            let a1520 = v1.clone();
            if equal(a1519, a1520) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f350(v0: List<(Bytes, ())>) -> List<Bytes> {
    f246(f435(Fn8::F436, List::<Bytes>::nil(), v0))
}

pub fn f351(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f437(f438(Fn9::F439, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1525 = {
        let (h1521, _) = f440(Fn10::F441, {
            let a1523 = {
                let a1522 = v0;
                length_list(a1522)
            };
            let a1524 = 1u64;
            nat_add(a1523, a1524)?
        }, v1)?;
        h1521
    };
    match m1525.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f352(v0: List<Bytes>) -> List<(Bytes, ())> {
    f442(f443(Fn11::F444, List::<(Bytes, ())>::nil(), v0))
}

pub fn f353(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1526 = f44(v1, 34u8)?;
    match m1526.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1527 = v3;
            match m1527.uncons() {
                None => {
                    Ok(f445(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f354(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f446(v0.clone(), 0u64, {
        let a1528 = v0;
        length_bytes(a1528)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f355(v0: Adt14, v1: Bytes) -> Bytes {
    if f32(f447(f94(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f356(v0: Bytes, v1: Bytes, p1541: u64, p1542: u64) -> R<bool> {
    let m1540 = (p1541, p1542);
    {
        let mut m1540 = m1540;
        loop {
            let step = {
                let (v2, v3) = m1540;
                let m1529 = v3;
                if m1529 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1529 - 1;
                    let m1532 = {
                        let a1530 = v1.clone();
                        let a1531 = v2;
                        index_bytes(a1530, a1531)
                    };
                    match m1532 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1535 = {
                                let a1533 = v0.clone();
                                let a1534 = v2;
                                index_bytes(a1533, a1534)
                            };
                            match m1535 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1536 = v5;
                                    let a1537 = v6;
                                    if equal(a1536, a1537) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1538 = v2;
                                            let a1539 = 1u64;
                                            nat_add(a1538, a1539)?
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
                Ok(next) => { m1540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1543 = f91(v0, v1)?;
    match m1543 {
        None => {
            Ok(Adt2::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f358(v0: Bytes) -> R<Bytes> {
    let a1544 = f8(Str::lit("Catalog."));
    let a1545 = f377(v0)?;
    Ok(append_bytes(a1544, a1545))
}

pub fn f359(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f377(f283(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f360(v0: List<Bytes>) -> List<Bytes> {
    let m1546 = v0;
    match m1546.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f249(f267(), v1), f360(v2))
            } else {
                f360(v2)
            }
        }
    }
}

pub fn f361(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1547 = v1;
    match m1547.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f448(v0.clone(), v2)?;
            let (h1548, _) = v4.clone();
            if h1548 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f361(v0, v3)?;
                let (h1549, _) = v5.clone();
                if h1549 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1552 = {
                            let (_, h1550) = v4;
                            h1550
                        };
                        let a1553 = {
                            let (_, h1551) = v5;
                            h1551
                        };
                        append_bytes(a1552, a1553)
                    }))
                }
            }
        }
    }
}

pub fn f362(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1554 = v1;
    match m1554.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f224(v2.clone(), f8(Str::lit("subject"))) {
                if f225(v0.clone(), v2.clone()) {
                    f362(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f362(v0, v3))
                }
            } else {
                if f368(v0.clone(), v2.clone()) {
                    f362(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f369(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f362(v0, v3))
                }
            }
        }
    }
}

pub fn f363(v0: Adt1) -> R<Bytes> {
    let a1556 = {
        let a1555 = f129(f130(f245(f259(f244(f64(v0.clone(), f218()))?)), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(f97(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1555)
    };
    let a1557 = 0u64;
    if nat_eq(a1556, a1557) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f364(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1558 = v0;
    match m1558.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f123(f150(f193(f94(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f449(f96(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f364(v2)?))
        }
    }
}

pub fn f365(v0: Bytes, v1: List<Adt19>) -> List<Bytes> {
    let m1559 = v1;
    match m1559.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt19::C0(h1560, _, _) => {
                    h1560
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt19::C0(_, h1561, _) => {
                        h1561
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt19::C0(_, _, h1562) => {
                        h1562
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f365(v0, v3))
            } else {
                f365(v0, v3)
            }
        }
    }
}

pub fn f366(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1563 = v1;
    match m1563.uncons() {
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

pub fn f367(p1568: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1567 = p1568;
    {
        let mut m1567 = m1567;
        loop {
            let step = {
                let v0 = m1567;
                let m1564 = v0;
                match m1564.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1565 = v2;
                        match m1565.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1566 = v5;
                                    match m1566.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f450(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(v0: Adt15, v1: Adt14) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1569, _, _) => {
            h1569
        }
    }) {
        f52(f369(v1), match v0 {
            Adt15::C0(_, h1570, _) => {
                h1570
            }
        })
    } else {
        false
    }
}

pub fn f369(v0: Adt14) -> Bytes {
    let m1571 = f94(v0, f8(Str::lit("evidence")));
    match m1571 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f370(v0: Adt14) -> Bytes {
    if f224(v0.clone(), f8(Str::lit("req"))) {
        let a1572 = f8(Str::lit("req "));
        let a1573 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1572, a1573)
    } else {
        let m1574 = f94(v0.clone(), f8(Str::lit("std")));
        match m1574 {
            None => {
                let m1577 = f94(v0, f8(Str::lit("der")));
                match m1577 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f123(f150(f193(f94(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f224(v1.clone(), f8(Str::lit("row"))) {
                    let a1575 = f8(Str::lit(" row "));
                    let a1576 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1575, a1576)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f371(v0: Option<List<Adt14>>) -> bool {
    let m1578 = v0;
    match m1578 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f372(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1579 = v0;
    match m1579 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f373(v0: Bytes, v1: Bytes) -> bool {
    let a1580 = v0;
    let a1581 = f385();
    if equal(a1580, a1581) {
        let a1585 = {
            let a1583 = {
                let a1582 = v1;
                length_bytes(a1582)
            };
            let a1584 = 0u64;
            nat_eq(a1583, a1584)
        };
        bool_not(a1585)
    } else {
        false
    }
}

pub fn f374(v0: Bytes) -> Bytes {
    f7(List::cons(f152(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f375(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1587 = v0;
    let a1588 = {
        let a1586 = Str::lit("redistributable");
        utf8_encode(a1586)
    };
    if equal(a1587, a1588) {
        f451(v1)
    } else {
        Ok(false)
    }
}

pub fn f376(v0: Adt2) -> bool {
    if f452(v0.clone(), f8(Str::lit("name"))) {
        if f453(v0.clone(), f8(Str::lit("schema"))) {
            let m1589 = f380(f284(f285(f280(v0, f8(Str::lit("citation"))))));
            match m1589 {
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

pub fn f377(v0: Bytes) -> R<Bytes> {
    f454(v0.clone(), 0u64, {
        let a1590 = v0;
        length_bytes(a1590)
    }, true, Bytes::lit(&[]))
}

pub fn f378(v0: Adt14) -> Bytes {
    if f224(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f224(v0.clone(), f8(Str::lit("id"))) {
            if f224(v0.clone(), f8(Str::lit("statement"))) {
                if f224(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f397(v0, f8(Str::lit("evidence")))
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

pub fn f379(v0: List<Bytes>) -> Adt14 {
    let m1591 = v0;
    match m1591.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f379(v2)))
        }
    }
}

pub fn f380(v0: List<Bytes>) -> Option<Adt14> {
    let m1592 = v0;
    match m1592.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1593 = v2;
                match m1593.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1594 = v2;
                    match m1594.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1595 = v6;
                                match m1595.uncons() {
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
                        let m1596 = v2;
                        match m1596.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1597 = v10;
                                match m1597.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f379(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f381(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1598 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1598.uncons() {
        None => {
            let m1599 = f380(v4);
            match m1599 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1600 = f8(Str::lit("Catalog."));
                        let a1601 = f377(v3.clone())?;
                        append_bytes(a1600, a1601)
                    };
                    let m1602 = f455(v2, v9, f377(v3.clone())?)?;
                    match m1602 {
                        Ok(v11) => {
                            f394(v0, v3.clone(), v8, v11, f8(Str::lit("Encoding entered through propose-encoding.")), List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f377(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::<(Bytes, Bytes)>::nil()))
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

pub fn f382(v0: Adt2) -> R<Bytes> {
    let m1603 = v0;
    match m1603 {
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
            f305(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f456(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f457(v9)
        }
        Adt2::C6(r1604) => {
            let v10 = (*r1604).clone();
            Ok(f307(f458(v10)?))
        }
        Adt2::C7(r1605) => {
            let v11 = (*r1605).clone();
            f459(f460(v11)?)
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

pub fn f383(v0: Option<Adt2>) -> Option<Bytes> {
    let m1606 = v0;
    match m1606 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1607 = v1;
            match m1607 {
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

pub fn f384(v0: Adt2) -> List<Adt2> {
    let m1608 = v0;
    match m1608 {
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
        Adt2::C6(r1609) => {
            let v3 = (*r1609).clone();
            f384(v3)
        }
        Adt2::C7(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C8 => {
            List::<Adt2>::nil()
        }
        Adt2::C9(r1610, r1611) => {
            let v1 = (*r1610).clone();
            let v2 = (*r1611).clone();
            List::cons(v1, f384(v2))
        }
        Adt2::C10(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f385() -> Bytes {
    let a1612 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1612)
}

pub fn f386(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f94(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f291(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f291(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f291(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f291(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f291(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f291(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f291(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f291(Str::lit("url"), f447(v2.clone(), f8(Str::lit("url"))))?, List::cons(f291(Str::lit("media_type"), f447(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f291(Str::lit("license"), f447(v2.clone(), f8(Str::lit("license"))))?, List::cons(f291(Str::lit("redistribution"), f447(v2, f8(Str::lit("redistribution"))))?, List::cons(f291(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1613 = v1;
        match m1613 {
            None => {
                f7(List::cons(f291(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f291(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f291(Str::lit("artifact"), {
                    let (h1614, _) = v3;
                    h1614
                })?, List::cons(f291(Str::lit("checksum"), {
                    let (_, h1615) = v3;
                    h1615
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f461(f150(f193(f94(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f387(v0: Adt24) -> Bytes {
    let a1636 = f462(match v0.clone() {
        Adt24::C0(h1616, _, _, _, _, _, _, _) => {
            h1616
        }
    });
    let a1637 = {
        let a1634 = f462(match v0.clone() {
            Adt24::C0(_, h1617, _, _, _, _, _, _) => {
                h1617
            }
        });
        let a1635 = {
            let a1632 = f462(match v0.clone() {
                Adt24::C0(_, _, h1618, _, _, _, _, _) => {
                    h1618
                }
            });
            let a1633 = {
                let a1630 = f462(match v0.clone() {
                    Adt24::C0(_, _, _, h1619, _, _, _, _) => {
                        h1619
                    }
                });
                let a1631 = {
                    let a1628 = f462(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1620, _, _, _) => {
                            h1620
                        }
                    });
                    let a1629 = {
                        let a1626 = f462(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1621, _, _) => {
                                h1621
                            }
                        });
                        let a1627 = {
                            let a1624 = f462(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1622, _) => {
                                    h1622
                                }
                            });
                            let a1625 = f462(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1623) => {
                                    h1623
                                }
                            });
                            append_bytes(a1624, a1625)
                        };
                        append_bytes(a1626, a1627)
                    };
                    append_bytes(a1628, a1629)
                };
                append_bytes(a1630, a1631)
            };
            append_bytes(a1632, a1633)
        };
        append_bytes(a1634, a1635)
    };
    append_bytes(a1636, a1637)
}

pub fn f388(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1638 = v0.clone();
        length_bytes(a1638)
    };
    let v2: u64 = {
        let a1644 = {
            let a1641 = {
                let a1639 = v1;
                let a1640 = 72u64;
                nat_add(a1639, a1640)?
            };
            let a1642 = 64u64;
            let a1643 = 0u64;
            nat_quot(a1641, a1642, a1643)
        };
        let a1645 = 64u64;
        nat_mul(a1644, a1645)?
    };
    let (_, h1646) = f463(Fn12::F464(v0, v1, v2), {
        let a1647 = v2;
        let a1648 = 64u64;
        let a1649 = 0u64;
        nat_quot(a1647, a1648, a1649)
    }, (0u64, f465()))?;
    Ok(h1646)
}

pub fn f389(p1652: List<List<Bytes>>, v1: Bytes, p1653: List<Bytes>) -> R<List<Bytes>> {
    let m1651 = (p1652, p1653);
    {
        let mut m1651 = m1651;
        loop {
            let step = {
                let (v0, v2) = m1651;
                let m1650 = v0;
                match m1650.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f466(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f391(f467(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1651 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f314(v0)?))
}

pub fn f391(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1654 = v0;
    match m1654.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1655 = v3.clone();
            match m1655.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1658 = v2;
                    let a1659 = {
                        let a1656 = v1.clone();
                        let a1657 = f391(v3, v1);
                        append_bytes(a1656, a1657)
                    };
                    append_bytes(a1658, a1659)
                }
            }
        }
    }
}

pub fn f392(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1660 = f94(v0, v1);
    match m1660 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f193(f94(v3, v2))
        }
    }
}

pub fn f393(v0: Bytes) -> bool {
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

pub fn f394(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a1661 = f8(Str::lit("Catalog."));
        let a1662 = f377(v1.clone())?;
        append_bytes(a1661, a1662)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f377(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f468(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f469(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f469(f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v7.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(v4)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a1666 = f470(f64(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a1664 = {
                let a1663 = v7;
                length_bytes(a1663)
            };
            let a1665 = 4u64;
            nat_sub(a1664, a1665)
        }))?;
        let a1667 = f8(Str::lit("\u{a}"));
        append_bytes(a1666, a1667)
    }), List::cons(f279(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f395(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f377(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f396(v0: Bytes) -> R<Bytes> {
    if f85(v0.clone(), f8(Str::lit("-- "))) {
        let m1668 = f232(v0.clone(), f8(Str::lit("\u{a}")))?;
        match m1668 {
            None => {
                Ok(v0)
            }
            Some(v1) => {
                Ok(f46(v0.clone(), {
                    let a1669 = v1;
                    let a1670 = 1u64;
                    nat_add(a1669, a1670)?
                }, {
                    let a1674 = {
                        let a1671 = v0;
                        length_bytes(a1671)
                    };
                    let a1675 = {
                        let a1672 = v1;
                        let a1673 = 1u64;
                        nat_add(a1672, a1673)?
                    };
                    nat_sub(a1674, a1675)
                }))
            }
        }
    } else {
        Ok(v0)
    }
}

pub fn f397(v0: Adt14, v1: Bytes) -> bool {
    let m1676 = f94(v0, v1);
    match m1676 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f398(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1678 = {
                let a1677 = v0;
                length_bytes(a1677)
            };
            let a1679 = 1u64;
            nat_sub(a1678, a1679)
        })
    } else {
        v0
    }
}

pub fn f399(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1680 = v0;
    match m1680 {
        Adt14::C0(v3) => {
            Adt14::C0(v3)
        }
        Adt14::C1(v4, v5) => {
            Adt14::C1(v4, v5)
        }
        Adt14::C2(v2) => {
            Adt14::C2(f471(f60(), v2, v1))
        }
        Adt14::C3(r1681) => {
            let v6 = (*r1681).clone();
            Adt14::C3(std::rc::Rc::new(v6))
        }
        Adt14::C4(r1682) => {
            let v7 = (*r1682).clone();
            Adt14::C4(std::rc::Rc::new(v7))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1683, r1684) => {
            let v8 = (*r1683).clone();
            let v9 = (*r1684).clone();
            Adt14::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt14::C7(v10, r1685, r1686) => {
            let v11 = (*r1685).clone();
            let v12 = (*r1686).clone();
            Adt14::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f400(v0: u64) -> R<Bytes> {
    f472(f473(), 10u64, v0)
}

pub fn f401(v0: Fn5, p1691: u64, p1692: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1690 = (p1691, p1692);
    {
        let mut m1690 = m1690;
        loop {
            let step = {
                let (v1, v2) = m1690;
                let m1687 = v1;
                if m1687 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1687 - 1;
                    let m1689 = {
                        let c1688: Fn5 = v0.clone();
                        c1688.apply(v2.clone())?
                    };
                    match m1689 {
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
                Ok(next) => { m1690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f474(v0, v1)
}

pub fn f403(v0: Fn6, p1696: List<(Bytes, Bytes)>, p1697: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1695 = (p1696, p1697);
    {
        let mut m1695 = m1695;
        loop {
            let step = {
                let (v1, v2) = m1695;
                let m1693 = v2;
                match m1693.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1694: Fn6 = v0.clone();
                            c1694.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1695 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f475(v0, v1)
}

pub fn f405(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f476(v0)
}

pub fn f406(v0: Fn7, p1701: List<Bytes>, p1702: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1700 = (p1701, p1702);
    {
        let mut m1700 = m1700;
        loop {
            let step = {
                let (v1, v2) = m1700;
                let m1698 = v2;
                match m1698.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1699: Fn7 = v0.clone();
                            c1699.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f477(v1)?, v0))
}

pub fn f408(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f306(v0)?, List::cons(f8(Str::lit(":")), List::cons(f306(v1)?, List::<Bytes>::nil())))))
}

pub fn f409(v0: Adt13) -> Bytes {
    let m1703 = v0;
    match m1703 {
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

pub fn f410(v0: Adt13) -> Bytes {
    let m1704 = v0;
    match m1704 {
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

pub fn f411(v0: Adt13) -> Bytes {
    let m1705 = v0;
    match m1705 {
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

pub fn f412(v0: Bytes, v1: u64) -> Option<u64> {
    let a1706 = 0u64;
    let a1707 = v1;
    if nat_lt(a1706, a1707) {
        if f478(v0, {
            let a1708 = v1;
            let a1709 = 1u64;
            nat_sub(a1708, a1709)
        }) {
            Some({
                let a1710 = v1;
                let a1711 = 1u64;
                nat_sub(a1710, a1711)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f413(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1712 = v2;
    let a1713 = v0;
    if nat_lt(a1712, a1713) {
        if f478(v1, v2) {
            Ok(Some({
                let a1714 = v2;
                let a1715 = 1u64;
                nat_add(a1714, a1715)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f414(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1718 = f46(v0.clone(), {
        let a1716 = v1;
        let a1717 = 2u64;
        nat_add(a1716, a1717)?
    }, 2u64);
    let a1719 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1718, a1719) {
        let m1722 = f479(f46(v0, {
            let a1720 = v1;
            let a1721 = 4u64;
            nat_add(a1720, a1721)?
        }, 2u64))?;
        match m1722 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f116(v2.clone(), 0u64)?;
                let a1723 = v3;
                let a1724 = 32u64;
                if nat_lt(a1723, a1724) {
                    if f480(v3) {
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

pub fn f415(v0: u64) -> Bytes {
    let a1725 = v0;
    let a1726 = 34u64;
    if nat_eq(a1725, a1726) {
        Bytes::lit(&[0x22])
    } else {
        let a1727 = v0;
        let a1728 = 92u64;
        if nat_eq(a1727, a1728) {
            Bytes::lit(&[0x5c])
        } else {
            let a1729 = v0;
            let a1730 = 98u64;
            if nat_eq(a1729, a1730) {
                Bytes::lit(&[0x08])
            } else {
                let a1731 = v0;
                let a1732 = 102u64;
                if nat_eq(a1731, a1732) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1733 = v0;
                    let a1734 = 110u64;
                    if nat_eq(a1733, a1734) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1735 = v0;
                        let a1736 = 114u64;
                        if nat_eq(a1735, a1736) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1737 = v0;
                            let a1738 = 116u64;
                            if nat_eq(a1737, a1738) {
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

pub fn f416(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f116(v0, v1)?;
    let a1739 = 47u64;
    let a1740 = v2;
    if if nat_lt(a1739, a1740) {
        let a1741 = v2;
        let a1742 = 58u64;
        nat_lt(a1741, a1742)
    } else {
        false
    } {
        Ok(Some({
            let a1743 = v1;
            let a1744 = 1u64;
            nat_add(a1743, a1744)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f417(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1745 = f116(v0, v1)?;
    let a1746 = 48u64;
    if nat_eq(a1745, a1746) {
        Ok(Some({
            let a1747 = v1;
            let a1748 = 1u64;
            nat_add(a1747, a1748)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f418(v0: Bytes) -> R<u64> {
    let (h1749, _) = f317(Fn3::F481(v0.clone()), {
        let a1750 = v0;
        length_bytes(a1750)
    }, 0u64)?;
    Ok(h1749)
}

pub fn f419(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1754 = {
        let a1752 = v0;
        let a1753 = {
            let (h1751, _) = v1;
            h1751
        };
        index_bytes(a1752, a1753)
    };
    match m1754 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1756 = {
                    let (h1755, _) = v1;
                    h1755
                };
                let a1757 = 1u64;
                nat_add(a1756, a1757)?
            }, {
                let a1761 = {
                    let a1759 = {
                        let (_, h1758) = v1;
                        h1758
                    };
                    let a1760 = 10u64;
                    nat_mul(a1759, a1760)?
                };
                let a1762 = f482(v2)?;
                nat_add(a1761, a1762)?
            })))
        }
    }
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f483(v0, v1)
}

pub fn f421(v0: (Bytes, Bytes), p1765: List<(Bytes, Bytes)>) -> bool {
    let m1764 = p1765;
    {
        let mut m1764 = m1764;
        loop {
            let step = {
                let v1 = m1764;
                let m1763 = v1;
                match m1763.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f484(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1764 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f422(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1766 = v1;
    match m1766.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1767, _) = v2;
                h1767
            }, v0.clone()) {
                List::cons({
                    let (_, h1768) = v2;
                    h1768
                }, f422(v0, v3))
            } else {
                f422(v0, v3)
            }
        }
    }
}

pub fn f423() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f424(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1769 = f94(v0, v1);
    match m1769 {
        None => {
            false
        }
        Some(v3) => {
            f224(v3, v2)
        }
    }
}

pub fn f425() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f426(v0: Bytes) -> R<bool> {
    let m1770 = f232(v0.clone(), f8(Str::lit("&&")))?;
    if match m1770 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1771 = f232(v0, f8(Str::lit("||")))?;
        match m1771 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f427(v0: Adt18, v1: Option<u64>) -> bool {
    let m1772 = v1;
    match m1772 {
        None => {
            true
        }
        Some(v2) => {
            f344(v2, match v0 {
                Adt18::C0(_, h1773, _, _) => {
                    h1773
                }
            })
        }
    }
}

pub fn f428(v0: Adt16) -> Adt21 {
    Adt21::C1({
        let a1775 = match v0 {
            Adt16::C0(_, _, h1774, _, _) => {
                h1774
            }
        };
        length_list(a1775)
    })
}

pub fn f429(v0: Bytes) -> Option<Adt22> {
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

pub fn f430(v0: Bytes, p1780: List<Bytes>, p1781: u64) -> R<Option<u64>> {
    let m1779 = (p1780, p1781);
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let (v1, v2) = m1779;
                let m1776 = v1;
                match m1776.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1777 = v2;
                                let a1778 = 1u64;
                                nat_add(a1777, a1778)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f431(v0: Option<Adt14>) -> Option<u64> {
    let m1782 = v0;
    match m1782 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1783 = v1;
            match m1783 {
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

pub fn f432(v0: Bytes, v1: u64) -> R<u64> {
    let m1786 = {
        let a1784 = v0;
        let a1785 = v1;
        index_bytes(a1784, a1785)
    };
    match m1786 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f433(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1790 = {
        let a1787 = v1;
        let a1788 = v2;
        let a1789 = 1u64;
        slice_bytes(a1787, a1788, a1789)
    };
    match m1790 {
        None => {
            v0
        }
        Some(v3) => {
            let a1791 = v0;
            let a1792 = v3;
            append_bytes(a1791, a1792)
        }
    }
}

pub fn f434(p1795: List<Bytes>, p1796: List<Bytes>) -> List<Bytes> {
    let m1794 = (p1795, p1796);
    {
        let mut m1794 = m1794;
        loop {
            let step = {
                let (v0, v1) = m1794;
                let m1793 = v0;
                match m1793.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f435(v0: Fn8, p1800: List<Bytes>, p1801: List<(Bytes, ())>) -> List<Bytes> {
    let m1799 = (p1800, p1801);
    {
        let mut m1799 = m1799;
        loop {
            let step = {
                let (v1, v2) = m1799;
                let m1797 = v2;
                match m1797.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1798: Fn8 = v0.clone();
                            c1798.apply(v1, v3)
                        }, v4))
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

pub fn f436(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1802, _) = v1;
        h1802
    }, v0)
}

pub fn f437(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f485(Fn13::F486, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f438(v0: Fn9, p1806: List<List<(Bytes, ())>>, p1807: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1805 = (p1806, p1807);
    {
        let mut m1805 = m1805;
        loop {
            let step = {
                let (v1, v2) = m1805;
                let m1803 = v2;
                match m1803.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1804: Fn9 = v0.clone();
                            c1804.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f440(v0: Fn10, p1812: u64, p1813: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1811 = (p1812, p1813);
    {
        let mut m1811 = m1811;
        loop {
            let step = {
                let (v1, v2) = m1811;
                let m1808 = v1;
                if m1808 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1808 - 1;
                    let m1810 = {
                        let c1809: Fn10 = v0.clone();
                        c1809.apply(v2.clone())?
                    };
                    match m1810 {
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
                Ok(next) => { m1811 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f487(v0)
}

pub fn f442(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f488(Fn14::F489, List::<(Bytes, ())>::nil(), v0)
}

pub fn f443(v0: Fn11, p1817: List<(Bytes, ())>, p1818: List<Bytes>) -> List<(Bytes, ())> {
    let m1816 = (p1817, p1818);
    {
        let mut m1816 = m1816;
        loop {
            let step = {
                let (v1, v2) = m1816;
                let m1814 = v2;
                match m1814.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1815: Fn11 = v0.clone();
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

pub fn f444(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f445(p1823: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1822 = p1823;
    {
        let mut m1822 = m1822;
        loop {
            let step = {
                let v0 = m1822;
                let m1819 = v0;
                match m1819.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1820, _) = v2;
                            h1820
                        }) {
                            let (_, h1821) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1821)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
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

pub fn f446(v0: Bytes, p1849: u64, p1850: u64, p1851: Bytes) -> R<Bytes> {
    let m1848 = (p1849, (p1850, p1851));
    {
        let mut m1848 = m1848;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1848;
                let m1824 = v2;
                if m1824 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1824 - 1;
                    let m1827 = {
                        let a1825 = v0.clone();
                        let a1826 = v1;
                        index_bytes(a1825, a1826)
                    };
                    match m1827 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f208({
                                let a1828 = v6;
                                let a1829 = 34u64;
                                nat_eq(a1828, a1829)
                            }, {
                                let a1830 = v6;
                                let a1831 = 92u64;
                                nat_eq(a1830, a1831)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1832 = v1;
                                    let a1833 = 1u64;
                                    nat_add(a1832, a1833)?
                                }, (v4, {
                                    let a1836 = v3;
                                    let a1837 = {
                                        let a1834 = f8(Str::lit("\\"));
                                        let a1835 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1834, a1835)
                                    };
                                    append_bytes(a1836, a1837)
                                })))
                            } else {
                                let a1838 = v6;
                                let a1839 = 10u64;
                                if nat_eq(a1838, a1839) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1840 = v1;
                                        let a1841 = 1u64;
                                        nat_add(a1840, a1841)?
                                    }, (v4, {
                                        let a1842 = v3;
                                        let a1843 = f8(Str::lit("\\n"));
                                        append_bytes(a1842, a1843)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1844 = v1;
                                        let a1845 = 1u64;
                                        nat_add(a1844, a1845)?
                                    }, (v4, {
                                        let a1846 = v3;
                                        let a1847 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1846, a1847)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1852 = v0;
    match m1852 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f448(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
    let m1853 = f297(v2.clone())?;
    match m1853 {
        None => {
            f491(v0, v1, v2)
        }
        Some(v3) => {
            f490(v0, v1, v3)
        }
    }
}

pub fn f449(v0: List<Bytes>) -> List<Bytes> {
    let m1854 = v0;
    match m1854.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1856 = {
                    let a1855 = v1.clone();
                    length_bytes(a1855)
                };
                let a1857 = 0u64;
                if nat_eq(a1856, a1857) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f449(v2))
        }
    }
}

pub fn f450(v0: List<Bytes>) -> Bytes {
    let m1858 = v0;
    match m1858.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f451(v0: Bytes) -> R<bool> {
    let m1860 = {
        let a1859 = v0.clone();
        length_bytes(a1859)
    };
    if m1860 == 0 {
        Ok(true)
    } else {
        let v1 = m1860 - 1;
        let m1863 = {
            let a1861 = v0;
            let a1862 = v1;
            index_bytes(a1861, a1862)
        };
        match m1863 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f492({
                    let v3: u8 = v2;
                    let a1908 = {
                        let a1884 = {
                            let a1872 = {
                                let a1866 = {
                                    let a1864 = v3;
                                    let a1865 = 128u8;
                                    fixed_u8::bit_and(a1864, a1865)
                                };
                                let a1867 = 128u8;
                                if equal(a1866, a1867) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1873 = {
                                let a1870 = {
                                    let a1868 = v3;
                                    let a1869 = 64u8;
                                    fixed_u8::bit_and(a1868, a1869)
                                };
                                let a1871 = 64u8;
                                if equal(a1870, a1871) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1872, a1873)?
                        };
                        let a1885 = {
                            let a1882 = {
                                let a1876 = {
                                    let a1874 = v3;
                                    let a1875 = 32u8;
                                    fixed_u8::bit_and(a1874, a1875)
                                };
                                let a1877 = 32u8;
                                if equal(a1876, a1877) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1883 = {
                                let a1880 = {
                                    let a1878 = v3;
                                    let a1879 = 16u8;
                                    fixed_u8::bit_and(a1878, a1879)
                                };
                                let a1881 = 16u8;
                                if equal(a1880, a1881) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1882, a1883)?
                        };
                        nat_add(a1884, a1885)?
                    };
                    let a1909 = {
                        let a1906 = {
                            let a1894 = {
                                let a1888 = {
                                    let a1886 = v3;
                                    let a1887 = 8u8;
                                    fixed_u8::bit_and(a1886, a1887)
                                };
                                let a1889 = 8u8;
                                if equal(a1888, a1889) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1895 = {
                                let a1892 = {
                                    let a1890 = v3;
                                    let a1891 = 4u8;
                                    fixed_u8::bit_and(a1890, a1891)
                                };
                                let a1893 = 4u8;
                                if equal(a1892, a1893) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1894, a1895)?
                        };
                        let a1907 = {
                            let a1904 = {
                                let a1898 = {
                                    let a1896 = v3;
                                    let a1897 = 2u8;
                                    fixed_u8::bit_and(a1896, a1897)
                                };
                                let a1899 = 2u8;
                                if equal(a1898, a1899) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1905 = {
                                let a1902 = {
                                    let a1900 = v3;
                                    let a1901 = 1u8;
                                    fixed_u8::bit_and(a1900, a1901)
                                };
                                let a1903 = 1u8;
                                if equal(a1902, a1903) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1904, a1905)?
                        };
                        nat_add(a1906, a1907)?
                    };
                    nat_add(a1908, a1909)?
                }))
            }
        }
    }
}

pub fn f452(v0: Adt2, v1: Bytes) -> bool {
    let a1911 = {
        let a1910 = f283(v0, v1);
        length_bytes(a1910)
    };
    let a1912 = 0u64;
    !nat_eq(a1911, a1912)
}

pub fn f453(v0: Adt2, v1: Bytes) -> bool {
    let m1913 = f280(v0, v1);
    match m1913 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f454(v0: Bytes, p1934: u64, p1935: u64, p1936: bool, p1937: Bytes) -> R<Bytes> {
    let m1933 = (p1934, (p1935, (p1936, p1937)));
    {
        let mut m1933 = m1933;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1933;
                let m1914 = v2;
                if m1914 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1914 - 1;
                    let m1917 = {
                        let a1915 = v0.clone();
                        let a1916 = v1;
                        index_bytes(a1915, a1916)
                    };
                    match m1917 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f493(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1918 = v1;
                                    let a1919 = 1u64;
                                    nat_add(a1918, a1919)?
                                }, (v5, (true, v4))))
                            } else {
                                if f118(f494(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1920 = v1;
                                        let a1921 = 1u64;
                                        nat_add(a1920, a1921)?
                                    }, (v5, (false, {
                                        let a1925 = v4;
                                        let a1926 = f46({
                                            let a1922 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1922)
                                        }, {
                                            let a1923 = v7;
                                            let a1924 = 97u64;
                                            nat_sub(a1923, a1924)
                                        }, 1u64);
                                        append_bytes(a1925, a1926)
                                    }))))
                                } else {
                                    if f208(f494(v7), f208(f495(v7), f496(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1927 = v1;
                                            let a1928 = 1u64;
                                            nat_add(a1927, a1928)?
                                        }, (v5, (false, {
                                            let a1929 = v4;
                                            let a1930 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1929, a1930)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1931 = v1;
                                            let a1932 = 1u64;
                                            nat_add(a1931, a1932)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1933 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f497(f498(f280(v0.clone(), f8(Str::lit("$defs")))), f498(f280(v0.clone(), f8(Str::lit("definitions")))));
    let m1938 = f499(4096u64, v3.clone(), List::<Bytes>::nil(), f500(v3)?)?;
    match m1938 {
        Ok(v5) => {
            let m1939 = f501(4096u64, f502(f503(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1939 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f391(f252(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f456(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f400(v1)?;
    let v4: u64 = {
        let a1940 = v3.clone();
        length_bytes(a1940)
    };
    let v5: Bytes = {
        let a1941 = v2;
        let a1942 = v4;
        if nat_lt(a1941, a1942) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1943 = v4;
                let a1944 = v2;
                nat_sub(a1943, a1944)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1945 = v4;
                let a1946 = v2;
                nat_sub(a1945, a1946)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f324({
                let a1947 = v2;
                let a1948 = v4;
                nat_sub(a1947, a1948)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1949 = f8(Str::lit("-"));
        let a1950 = v5;
        Ok(append_bytes(a1949, a1950))
    } else {
        Ok(v5)
    }
}

pub fn f457(v0: Bytes) -> R<Bytes> {
    let a1955 = f205(v0.clone(), 0u64, {
        let a1952 = {
            let a1951 = v0.clone();
            length_bytes(a1951)
        };
        let a1953 = 1u64;
        nat_add(a1952, a1953)?
    })?;
    let a1956 = {
        let a1954 = v0.clone();
        length_bytes(a1954)
    };
    if nat_eq(a1955, a1956) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1962 = {
            let a1960 = Bytes::lit(&[0x22]);
            let a1961 = f504(v0.clone(), 0u64, {
                let a1958 = {
                    let a1957 = v0;
                    length_bytes(a1957)
                };
                let a1959 = 1u64;
                nat_add(a1958, a1959)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1960, a1961)
        };
        let a1963 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1962, a1963))
    }
}

pub fn f458(v0: Adt2) -> R<List<Bytes>> {
    let m1964 = v0;
    match m1964 {
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
        Adt2::C9(r1965, r1966) => {
            let v12 = (*r1965).clone();
            let v13 = (*r1966).clone();
            Ok(List::cons(f382(v12)?, f458(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f459(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f403(Fn6::F505, List::<(Bytes, Bytes)>::nil(), f405(v0)?);
    let v2: List<Bytes> = f406(Fn7::F506, List::<Bytes>::nil(), v1)?;
    let a1969 = {
        let a1967 = Bytes::lit(&[0x7b]);
        let a1968 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1967, a1968)
    };
    let a1970 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1969, a1970))
}

pub fn f460(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1971 = v0;
    match m1971 {
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
        Adt2::C10(v14, r1972, r1973) => {
            let v15 = (*r1972).clone();
            let v16 = (*r1973).clone();
            Ok(List::cons((v14, f382(v15)?), f460(v16)?))
        }
    }
}

pub fn f461(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f391(f507(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f462(v0: u32) -> Bytes {
    let a2002 = f508({
        let a1974 = f509(v0, 28u32);
        let a1975 = 15u32;
        fixed_u32::bit_and(a1974, a1975)
    });
    let a2003 = {
        let a2000 = f508({
            let a1976 = f509(v0, 24u32);
            let a1977 = 15u32;
            fixed_u32::bit_and(a1976, a1977)
        });
        let a2001 = {
            let a1998 = f508({
                let a1978 = f509(v0, 20u32);
                let a1979 = 15u32;
                fixed_u32::bit_and(a1978, a1979)
            });
            let a1999 = {
                let a1996 = f508({
                    let a1980 = f509(v0, 16u32);
                    let a1981 = 15u32;
                    fixed_u32::bit_and(a1980, a1981)
                });
                let a1997 = {
                    let a1994 = f508({
                        let a1982 = f509(v0, 12u32);
                        let a1983 = 15u32;
                        fixed_u32::bit_and(a1982, a1983)
                    });
                    let a1995 = {
                        let a1992 = f508({
                            let a1984 = f509(v0, 8u32);
                            let a1985 = 15u32;
                            fixed_u32::bit_and(a1984, a1985)
                        });
                        let a1993 = {
                            let a1990 = f508({
                                let a1986 = f509(v0, 4u32);
                                let a1987 = 15u32;
                                fixed_u32::bit_and(a1986, a1987)
                            });
                            let a1991 = f508({
                                let a1988 = v0;
                                let a1989 = 15u32;
                                fixed_u32::bit_and(a1988, a1989)
                            });
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
    };
    append_bytes(a2002, a2003)
}

pub fn f463(v0: Fn12, p2007: u64, p2008: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m2006 = (p2007, p2008);
    {
        let mut m2006 = m2006;
        loop {
            let step = {
                let (v1, v2) = m2006;
                let m2004 = v1;
                if m2004 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m2004 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c2005: Fn12 = v0.clone();
                        c2005.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2006 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a2010 = {
            let (h2009, _) = v3.clone();
            h2009
        };
        let a2011 = 64u64;
        nat_add(a2010, a2011)?
    }, f510({
        let (_, h2012) = v3;
        h2012
    }, f511(v0, v1, v2, {
        let (h2013, _) = v3;
        h2013
    })?)?))
}

pub fn f465() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f466(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2014 = f44(v1, 34u8)?;
    match m2014.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2015 = v3;
            match m2015.uncons() {
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

pub fn f467(v0: List<Bytes>) -> List<Bytes> {
    let m2016 = v0;
    match m2016.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f467(v2)
            } else {
                List::cons(v1, f467(v2))
            }
        }
    }
}

pub fn f468(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2017 = v0;
    match m2017.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f468(v3, v1))
        }
    }
}

pub fn f469(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m2018 = v0;
    match m2018.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f513(v5, v1, v2))
            } else {
                List::cons(v4, f469(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f470(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f391(f514(f96(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f471(p2021: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2020 = p2021;
    {
        let mut m2020 = m2020;
        loop {
            let step = {
                let v0 = m2020;
                let m2019 = v0;
                match m2019.uncons() {
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
                Ok(next) => { m2020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2022 = v2;
    let a2023 = 0u64;
    if nat_eq(a2022, a2023) {
        Ok(f515(v0, 0u64))
    } else {
        let (_, h2024) = {
            let (h2025, _) = f401(Fn5::F516(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2025
        };
        Ok(h2024)
    }
}

pub fn f473() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f474(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2029 = {
        let a2027 = v0.clone();
        let a2028 = {
            let (h2026, _) = v1.clone();
            h2026
        };
        index_bytes(a2027, a2028)
    };
    match m2029 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2031 = {
                    let (h2030, _) = v1.clone();
                    h2030
                };
                let a2032 = 1u64;
                nat_add(a2031, a2032)?
            }, {
                let a2035 = {
                    let (_, h2033) = v1;
                    h2033
                };
                let a2036 = f517(v0, {
                    let (h2034, _) = v1;
                    h2034
                }, v2)?;
                append_bytes(a2035, a2036)
            })))
        }
    }
}

pub fn f475(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2037 = v0.clone();
    match m2037.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2040 = {
                let (h2038, _) = v2;
                h2038
            };
            let a2041 = {
                let (h2039, _) = v1.clone();
                h2039
            };
            if equal(a2040, a2041) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f476(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f518(f519(Fn15::F520, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2046 = {
        let (h2042, _) = f521(Fn16::F522, {
            let a2044 = {
                let a2043 = v0;
                length_list(a2043)
            };
            let a2045 = 1u64;
            nat_add(a2044, a2045)?
        }, v1)?;
        h2042
    };
    match m2046.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f477(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2051 = {
        let a2048 = f306({
            let (h2047, _) = v0;
            h2047
        })?;
        let a2049 = Bytes::lit(&[0x3a]);
        append_bytes(a2048, a2049)
    };
    let a2052 = {
        let (_, h2050) = v0;
        h2050
    };
    Ok(append_bytes(a2051, a2052))
}

pub fn f478(v0: Bytes, v1: u64) -> bool {
    let m2053 = f213(v0, v1);
    match m2053 {
        None => {
            false
        }
        Some(v2) => {
            f523(v2)
        }
    }
}

pub fn f479(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2054 = v0.clone();
        length_bytes(a2054)
    };
    let a2058 = {
        let a2055 = v1;
        let a2056 = 2u64;
        let a2057 = 0u64;
        nat_rem(a2055, a2056, a2057)
    };
    let a2059 = 0u64;
    if nat_eq(a2058, a2059) {
        let (_, h2060) = {
            let (h2061, _) = f524(Fn17::F525(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2061
        };
        Ok(h2060)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f480(v0: u64) -> bool {
    let a2062 = v0;
    let a2063 = 8u64;
    if nat_eq(a2062, a2063) {
        true
    } else {
        let a2064 = v0;
        let a2065 = 9u64;
        if nat_eq(a2064, a2065) {
            true
        } else {
            let a2066 = v0;
            let a2067 = 10u64;
            if nat_eq(a2066, a2067) {
                true
            } else {
                let a2068 = v0;
                let a2069 = 12u64;
                if nat_eq(a2068, a2069) {
                    true
                } else {
                    let a2070 = v0;
                    let a2071 = 13u64;
                    nat_eq(a2070, a2071)
                }
            }
        }
    }
}

pub fn f481(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2074 = {
        let a2072 = v0;
        let a2073 = v1;
        index_bytes(a2072, a2073)
    };
    match m2074 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f526(v2)? {
                Ok(Some({
                    let a2075 = v1;
                    let a2076 = 1u64;
                    nat_add(a2075, a2076)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f482(v0: u8) -> R<u64> {
    let a2123 = {
        let v1: u8 = v0;
        let a2121 = {
            let a2097 = {
                let a2085 = {
                    let a2079 = {
                        let a2077 = v1;
                        let a2078 = 128u8;
                        fixed_u8::bit_and(a2077, a2078)
                    };
                    let a2080 = 128u8;
                    if equal(a2079, a2080) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2086 = {
                    let a2083 = {
                        let a2081 = v1;
                        let a2082 = 64u8;
                        fixed_u8::bit_and(a2081, a2082)
                    };
                    let a2084 = 64u8;
                    if equal(a2083, a2084) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2085, a2086)?
            };
            let a2098 = {
                let a2095 = {
                    let a2089 = {
                        let a2087 = v1;
                        let a2088 = 32u8;
                        fixed_u8::bit_and(a2087, a2088)
                    };
                    let a2090 = 32u8;
                    if equal(a2089, a2090) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2096 = {
                    let a2093 = {
                        let a2091 = v1;
                        let a2092 = 16u8;
                        fixed_u8::bit_and(a2091, a2092)
                    };
                    let a2094 = 16u8;
                    if equal(a2093, a2094) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2095, a2096)?
            };
            nat_add(a2097, a2098)?
        };
        let a2122 = {
            let a2119 = {
                let a2107 = {
                    let a2101 = {
                        let a2099 = v1;
                        let a2100 = 8u8;
                        fixed_u8::bit_and(a2099, a2100)
                    };
                    let a2102 = 8u8;
                    if equal(a2101, a2102) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2108 = {
                    let a2105 = {
                        let a2103 = v1;
                        let a2104 = 4u8;
                        fixed_u8::bit_and(a2103, a2104)
                    };
                    let a2106 = 4u8;
                    if equal(a2105, a2106) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2107, a2108)?
            };
            let a2120 = {
                let a2117 = {
                    let a2111 = {
                        let a2109 = v1;
                        let a2110 = 2u8;
                        fixed_u8::bit_and(a2109, a2110)
                    };
                    let a2112 = 2u8;
                    if equal(a2111, a2112) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2118 = {
                    let a2115 = {
                        let a2113 = v1;
                        let a2114 = 1u8;
                        fixed_u8::bit_and(a2113, a2114)
                    };
                    let a2116 = 1u8;
                    if equal(a2115, a2116) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2117, a2118)?
            };
            nat_add(a2119, a2120)?
        };
        nat_add(a2121, a2122)?
    };
    let a2124 = 48u64;
    Ok(nat_sub(a2123, a2124))
}

pub fn f483(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2125 = v0.clone();
        length_bytes(a2125)
    };
    let a2126 = v1;
    let a2127 = v2;
    if if nat_lt(a2126, a2127) {
        let a2132 = f116(v0, {
            let a2130 = v2;
            let a2131 = {
                let a2128 = v1;
                let a2129 = 1u64;
                nat_add(a2128, a2129)?
            };
            nat_sub(a2130, a2131)
        })?;
        let a2133 = 48u64;
        nat_eq(a2132, a2133)
    } else {
        false
    } {
        Ok(Some({
            let a2134 = v1;
            let a2135 = 1u64;
            nat_add(a2134, a2135)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f484(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f118(f32({
        let (h2136, _) = v0;
        h2136
    }, {
        let (h2137, _) = v1;
        h2137
    }), f32({
        let (_, h2138) = v0;
        h2138
    }, {
        let (_, h2139) = v1;
        h2139
    }))
}

pub fn f485(v0: Fn13, p2143: List<List<(Bytes, ())>>, p2144: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2142 = (p2143, p2144);
    {
        let mut m2142 = m2142;
        loop {
            let step = {
                let (v1, v2) = m2142;
                let m2140 = v2;
                match m2140.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2141: Fn13 = v0.clone();
                            c2141.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f486(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f487(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2146 = {
        let a2145 = v0.clone();
        length_list(a2145)
    };
    let a2147 = 1u64;
    if nat_le(a2146, a2147) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f527(v0)?))
    }
}

pub fn f488(v0: Fn14, p2151: List<(Bytes, ())>, p2152: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2150 = (p2151, p2152);
    {
        let mut m2150 = m2150;
        loop {
            let step = {
                let (v1, v2) = m2150;
                let m2148 = v2;
                match m2148.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2149: Fn14 = v0.clone();
                            c2149.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f490(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f149(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f149(f97(v0, f161())?, f8(Str::lit("at"))));
    let a2154 = {
        let a2153 = v4.clone();
        length_list(a2153)
    };
    let a2155 = 0u64;
    if nat_eq(a2154, a2155) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2158 = {
            let a2156 = v4;
            length_list(a2156)
        };
        let a2159 = {
            let a2157 = v3;
            length_list(a2157)
        };
        if nat_eq(a2158, a2159) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f528(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f491(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f372(f275(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f372(f275(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f126(f149(v3.clone(), f8(Str::lit("id"))), f149(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f149(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f126(v6.clone(), f149(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2161 = {
        let a2160 = v7.clone();
        length_list(a2160)
    };
    let a2162 = 0u64;
    if nat_eq(a2161, a2162) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2165 = {
                let a2163 = v7;
                length_list(a2163)
            };
            let a2166 = {
                let a2164 = v5;
                length_list(a2164)
            };
            nat_eq(a2165, a2166)
        }) {
            Ok((false, f528(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f153(f126(f62(f529(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f530(v4), f126(v6, f149(v3, f8(Str::lit("id")))))))?;
            let a2168 = {
                let a2167 = v8.clone();
                length_list(a2167)
            };
            let a2169 = 0u64;
            if nat_eq(a2168, a2169) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f528(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f492(v0: u64) -> bool {
    let a2170 = v0;
    let a2171 = 47u64;
    nat_eq(a2170, a2171)
}

pub fn f493(v0: u64) -> bool {
    f208({
        let a2172 = v0;
        let a2173 = 95u64;
        nat_eq(a2172, a2173)
    }, f208({
        let a2174 = v0;
        let a2175 = 45u64;
        nat_eq(a2174, a2175)
    }, f208({
        let a2176 = v0;
        let a2177 = 32u64;
        nat_eq(a2176, a2177)
    }, f208({
        let a2178 = v0;
        let a2179 = 46u64;
        nat_eq(a2178, a2179)
    }, {
        let a2180 = v0;
        let a2181 = 47u64;
        nat_eq(a2180, a2181)
    }))))
}

pub fn f494(v0: u64) -> bool {
    f118({
        let a2182 = 97u64;
        let a2183 = v0;
        nat_le(a2182, a2183)
    }, {
        let a2184 = v0;
        let a2185 = 122u64;
        nat_le(a2184, a2185)
    })
}

pub fn f495(v0: u64) -> bool {
    f118({
        let a2186 = 65u64;
        let a2187 = v0;
        nat_le(a2186, a2187)
    }, {
        let a2188 = v0;
        let a2189 = 90u64;
        nat_le(a2188, a2189)
    })
}

pub fn f496(v0: u64) -> bool {
    f118({
        let a2190 = 48u64;
        let a2191 = v0;
        nat_le(a2190, a2191)
    }, {
        let a2192 = v0;
        let a2193 = 57u64;
        nat_le(a2192, a2193)
    })
}

pub fn f497(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2194 = v0;
    match m2194.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f497(v3, v1))
        }
    }
}

pub fn f498(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2195 = v0;
    match m2195 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f531(v1)
        }
    }
}

pub fn f499(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2196 = v0;
    if m2196 == 0 {
        let m2197 = v1;
        match m2197.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2196 - 1;
        let m2198 = v1.clone();
        match m2198.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2199 = f532(v1.clone(), v2.clone(), v3.clone())?;
                match m2199 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f391(f500(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2202 = f499(v6, f533(v1, {
                            let (h2200, _) = v9.clone();
                            h2200
                        }), List::cons(f534({
                            let (h2201, _) = v9.clone();
                            h2201
                        })?, v2), v3)?;
                        match m2202 {
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

pub fn f500(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2203 = v0;
    match m2203.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f534({
                let (h2204, _) = v1;
                h2204
            })?, f500(v2)?))
        }
    }
}

pub fn f501(p2210: u64, p2211: List<Adt25>, p2212: List<Bytes>, p2213: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2209 = (p2210, (p2211, (p2212, p2213)));
    {
        let mut m2209 = m2209;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2209;
                let m2205 = v0;
                if m2205 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2205 - 1;
                    let m2206 = v1;
                    match m2206.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2207 = v5;
                            match m2207 {
                                Adt25::C0(v8, v9) => {
                                    if f32(v8.clone(), f535()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2208 = f537(v8.clone(), v9)?;
                                            match m2208 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f502(f538(v11.clone()), List::cons(Adt25::C1(f539(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f52(f535(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f535(), v2), List::cons(f536(), v3)))))
                                    }
                                }
                            }
                        }
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

pub fn f502(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2214 = v0;
    match m2214.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f502(v3, v1))
        }
    }
}

pub fn f503(v0: List<(Bytes, Adt2)>) -> R<List<Adt25>> {
    let m2215 = v0;
    match m2215.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f534({
                let (h2216, _) = v1;
                h2216
            })?, {
                let (_, h2217) = v1;
                h2217
            }), f503(v2)?))
        }
    }
}

pub fn f504(v0: Bytes, p2236: u64, p2237: u64, p2238: Bytes) -> R<Bytes> {
    let m2235 = (p2236, (p2237, p2238));
    {
        let mut m2235 = m2235;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2235;
                let m2218 = v2;
                if m2218 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2218 - 1;
                    let v5: u64 = f205(v0.clone(), v1, {
                        let a2222 = {
                            let a2220 = {
                                let a2219 = v0.clone();
                                length_bytes(a2219)
                            };
                            let a2221 = v1;
                            nat_sub(a2220, a2221)
                        };
                        let a2223 = 1u64;
                        nat_add(a2222, a2223)?
                    })?;
                    let v6: Bytes = {
                        let a2226 = v3;
                        let a2227 = f46(v0.clone(), v1, {
                            let a2224 = v5;
                            let a2225 = v1;
                            nat_sub(a2224, a2225)
                        });
                        append_bytes(a2226, a2227)
                    };
                    let m2230 = {
                        let a2228 = v0.clone();
                        let a2229 = v5;
                        index_bytes(a2228, a2229)
                    };
                    match m2230 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2231 = v5;
                                let a2232 = 1u64;
                                nat_add(a2231, a2232)?
                            }, (v4, {
                                let a2233 = v6;
                                let a2234 = f517(v0.clone(), v5, v7)?;
                                append_bytes(a2233, a2234)
                            })))
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

pub fn f505(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f475(v0, v1)
}

pub fn f506(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f540(v1)?, v0))
}

pub fn f507(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2239 = v0;
    match m2239.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f354(v1)?, f507(v2)?))
        }
    }
}

pub fn f508(v0: u32) -> Bytes {
    let a2242 = {
        let a2240 = v0;
        let a2241 = 8u32;
        fixed_u32::bit_and(a2240, a2241)
    };
    let a2243 = 0u32;
    if equal(a2242, a2243) {
        let a2246 = {
            let a2244 = v0;
            let a2245 = 4u32;
            fixed_u32::bit_and(a2244, a2245)
        };
        let a2247 = 0u32;
        if equal(a2246, a2247) {
            let a2250 = {
                let a2248 = v0;
                let a2249 = 2u32;
                fixed_u32::bit_and(a2248, a2249)
            };
            let a2251 = 0u32;
            if equal(a2250, a2251) {
                let a2254 = {
                    let a2252 = v0;
                    let a2253 = 1u32;
                    fixed_u32::bit_and(a2252, a2253)
                };
                let a2255 = 0u32;
                if equal(a2254, a2255) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2258 = {
                    let a2256 = v0;
                    let a2257 = 1u32;
                    fixed_u32::bit_and(a2256, a2257)
                };
                let a2259 = 0u32;
                if equal(a2258, a2259) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2262 = {
                let a2260 = v0;
                let a2261 = 2u32;
                fixed_u32::bit_and(a2260, a2261)
            };
            let a2263 = 0u32;
            if equal(a2262, a2263) {
                let a2266 = {
                    let a2264 = v0;
                    let a2265 = 1u32;
                    fixed_u32::bit_and(a2264, a2265)
                };
                let a2267 = 0u32;
                if equal(a2266, a2267) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2270 = {
                    let a2268 = v0;
                    let a2269 = 1u32;
                    fixed_u32::bit_and(a2268, a2269)
                };
                let a2271 = 0u32;
                if equal(a2270, a2271) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2274 = {
            let a2272 = v0;
            let a2273 = 4u32;
            fixed_u32::bit_and(a2272, a2273)
        };
        let a2275 = 0u32;
        if equal(a2274, a2275) {
            let a2278 = {
                let a2276 = v0;
                let a2277 = 2u32;
                fixed_u32::bit_and(a2276, a2277)
            };
            let a2279 = 0u32;
            if equal(a2278, a2279) {
                let a2282 = {
                    let a2280 = v0;
                    let a2281 = 1u32;
                    fixed_u32::bit_and(a2280, a2281)
                };
                let a2283 = 0u32;
                if equal(a2282, a2283) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2286 = {
                    let a2284 = v0;
                    let a2285 = 1u32;
                    fixed_u32::bit_and(a2284, a2285)
                };
                let a2287 = 0u32;
                if equal(a2286, a2287) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2290 = {
                let a2288 = v0;
                let a2289 = 2u32;
                fixed_u32::bit_and(a2288, a2289)
            };
            let a2291 = 0u32;
            if equal(a2290, a2291) {
                let a2294 = {
                    let a2292 = v0;
                    let a2293 = 1u32;
                    fixed_u32::bit_and(a2292, a2293)
                };
                let a2295 = 0u32;
                if equal(a2294, a2295) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2298 = {
                    let a2296 = v0;
                    let a2297 = 1u32;
                    fixed_u32::bit_and(a2296, a2297)
                };
                let a2299 = 0u32;
                if equal(a2298, a2299) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f509(v0: u32, v1: u32) -> u32 {
    let m2302 = {
        let a2300 = v0;
        let a2301 = v1;
        fixed_u32::shift_right(a2300, a2301)
    };
    match m2302 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f510(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f541(Fn18::F542, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2303, _, _, _, _, _, _, _) => {
            h2303
        }
    }, match v0.clone() {
        Adt24::C0(_, h2304, _, _, _, _, _, _) => {
            h2304
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2305, _, _, _, _, _) => {
            h2305
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2306, _, _, _, _) => {
            h2306
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2307, _, _, _) => {
            h2307
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2308, _, _) => {
            h2308
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2309, _) => {
            h2309
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2310) => {
            h2310
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f543(match v0.clone() {
        Adt24::C0(h2311, _, _, _, _, _, _, _) => {
            h2311
        }
    }, match v2.clone() {
        Adt27::C0(h2312, _, _, _, _, _, _, _, _, _) => {
            h2312
        }
    }), f543(match v0.clone() {
        Adt24::C0(_, h2313, _, _, _, _, _, _) => {
            h2313
        }
    }, match v2.clone() {
        Adt27::C0(_, h2314, _, _, _, _, _, _, _, _) => {
            h2314
        }
    }), f543(match v0.clone() {
        Adt24::C0(_, _, h2315, _, _, _, _, _) => {
            h2315
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2316, _, _, _, _, _, _, _) => {
            h2316
        }
    }), f543(match v0.clone() {
        Adt24::C0(_, _, _, h2317, _, _, _, _) => {
            h2317
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2318, _, _, _, _, _, _) => {
            h2318
        }
    }), f543(match v0.clone() {
        Adt24::C0(_, _, _, _, h2319, _, _, _) => {
            h2319
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2320, _, _, _, _, _) => {
            h2320
        }
    }), f543(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2321, _, _) => {
            h2321
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2322, _, _, _, _) => {
            h2322
        }
    }), f543(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2323, _) => {
            h2323
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2324, _, _, _) => {
            h2324
        }
    }), f543(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2325) => {
            h2325
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2326, _, _) => {
            h2326
        }
    })))
}

pub fn f511(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f544(v0.clone(), v1, v2, v3)?, f544(v0.clone(), v1, v2, {
        let a2327 = v3;
        let a2328 = 4u64;
        nat_add(a2327, a2328)?
    })?, f544(v0.clone(), v1, v2, {
        let a2329 = v3;
        let a2330 = 8u64;
        nat_add(a2329, a2330)?
    })?, f544(v0.clone(), v1, v2, {
        let a2331 = v3;
        let a2332 = 12u64;
        nat_add(a2331, a2332)?
    })?, f544(v0.clone(), v1, v2, {
        let a2333 = v3;
        let a2334 = 16u64;
        nat_add(a2333, a2334)?
    })?, f544(v0.clone(), v1, v2, {
        let a2335 = v3;
        let a2336 = 20u64;
        nat_add(a2335, a2336)?
    })?, f544(v0.clone(), v1, v2, {
        let a2337 = v3;
        let a2338 = 24u64;
        nat_add(a2337, a2338)?
    })?, f544(v0.clone(), v1, v2, {
        let a2339 = v3;
        let a2340 = 28u64;
        nat_add(a2339, a2340)?
    })?, f544(v0.clone(), v1, v2, {
        let a2341 = v3;
        let a2342 = 32u64;
        nat_add(a2341, a2342)?
    })?, f544(v0.clone(), v1, v2, {
        let a2343 = v3;
        let a2344 = 36u64;
        nat_add(a2343, a2344)?
    })?, f544(v0.clone(), v1, v2, {
        let a2345 = v3;
        let a2346 = 40u64;
        nat_add(a2345, a2346)?
    })?, f544(v0.clone(), v1, v2, {
        let a2347 = v3;
        let a2348 = 44u64;
        nat_add(a2347, a2348)?
    })?, f544(v0.clone(), v1, v2, {
        let a2349 = v3;
        let a2350 = 48u64;
        nat_add(a2349, a2350)?
    })?, f544(v0.clone(), v1, v2, {
        let a2351 = v3;
        let a2352 = 52u64;
        nat_add(a2351, a2352)?
    })?, f544(v0.clone(), v1, v2, {
        let a2353 = v3;
        let a2354 = 56u64;
        nat_add(a2353, a2354)?
    })?, f544(v0, v1, v2, {
        let a2355 = v3;
        let a2356 = 60u64;
        nat_add(a2355, a2356)?
    })?))
}

pub fn f512(p2359: List<Bytes>, v1: Bytes) -> bool {
    let m2358 = p2359;
    {
        let mut m2358 = m2358;
        loop {
            let step = {
                let v0 = m2358;
                let m2357 = v0;
                match m2357.uncons() {
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
                Ok(next) => { m2358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2360 = v0;
    match m2360.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f513(v4, v1, v2)
            } else {
                List::cons(v3, f513(v4, v1, v2))
            }
        }
    }
}

pub fn f514(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2361 = v0;
    match m2361.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f391(f507(f153(List::cons(v1, f545(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f514(v3, v1)?))
            }
        }
    }
}

pub fn f515(v0: Bytes, v1: u64) -> Bytes {
    let m2365 = {
        let a2362 = v0;
        let a2363 = v1;
        let a2364 = 1u64;
        slice_bytes(a2362, a2363, a2364)
    };
    match m2365 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f516(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f546(v1, v0, v2)
}

pub fn f517(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2410 = {
            let a2386 = {
                let a2374 = {
                    let a2368 = {
                        let a2366 = v3;
                        let a2367 = 128u8;
                        fixed_u8::bit_and(a2366, a2367)
                    };
                    let a2369 = 128u8;
                    if equal(a2368, a2369) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2375 = {
                    let a2372 = {
                        let a2370 = v3;
                        let a2371 = 64u8;
                        fixed_u8::bit_and(a2370, a2371)
                    };
                    let a2373 = 64u8;
                    if equal(a2372, a2373) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2374, a2375)?
            };
            let a2387 = {
                let a2384 = {
                    let a2378 = {
                        let a2376 = v3;
                        let a2377 = 32u8;
                        fixed_u8::bit_and(a2376, a2377)
                    };
                    let a2379 = 32u8;
                    if equal(a2378, a2379) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2385 = {
                    let a2382 = {
                        let a2380 = v3;
                        let a2381 = 16u8;
                        fixed_u8::bit_and(a2380, a2381)
                    };
                    let a2383 = 16u8;
                    if equal(a2382, a2383) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2384, a2385)?
            };
            nat_add(a2386, a2387)?
        };
        let a2411 = {
            let a2408 = {
                let a2396 = {
                    let a2390 = {
                        let a2388 = v3;
                        let a2389 = 8u8;
                        fixed_u8::bit_and(a2388, a2389)
                    };
                    let a2391 = 8u8;
                    if equal(a2390, a2391) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2397 = {
                    let a2394 = {
                        let a2392 = v3;
                        let a2393 = 4u8;
                        fixed_u8::bit_and(a2392, a2393)
                    };
                    let a2395 = 4u8;
                    if equal(a2394, a2395) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2396, a2397)?
            };
            let a2409 = {
                let a2406 = {
                    let a2400 = {
                        let a2398 = v3;
                        let a2399 = 2u8;
                        fixed_u8::bit_and(a2398, a2399)
                    };
                    let a2401 = 2u8;
                    if equal(a2400, a2401) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2407 = {
                    let a2404 = {
                        let a2402 = v3;
                        let a2403 = 1u8;
                        fixed_u8::bit_and(a2402, a2403)
                    };
                    let a2405 = 1u8;
                    if equal(a2404, a2405) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2406, a2407)?
            };
            nat_add(a2408, a2409)?
        };
        nat_add(a2410, a2411)?
    };
    let a2412 = v4;
    let a2413 = 34u64;
    if nat_eq(a2412, a2413) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2414 = v4;
        let a2415 = 92u64;
        if nat_eq(a2414, a2415) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2416 = v4;
            let a2417 = 8u64;
            if nat_eq(a2416, a2417) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2418 = v4;
                let a2419 = 12u64;
                if nat_eq(a2418, a2419) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2420 = v4;
                    let a2421 = 10u64;
                    if nat_eq(a2420, a2421) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2422 = v4;
                        let a2423 = 13u64;
                        if nat_eq(a2422, a2423) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2424 = v4;
                            let a2425 = 9u64;
                            if nat_eq(a2424, a2425) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2426 = v4;
                                let a2427 = 32u64;
                                if nat_lt(a2426, a2427) {
                                    let a2428 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2429 = f547(v2)?;
                                    Ok(append_bytes(a2428, a2429))
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

pub fn f518(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f548(Fn19::F549, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f519(v0: Fn15, p2433: List<List<(Bytes, Bytes)>>, p2434: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2432 = (p2433, p2434);
    {
        let mut m2432 = m2432;
        loop {
            let step = {
                let (v1, v2) = m2432;
                let m2430 = v2;
                match m2430.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2431: Fn15 = v0.clone();
                            c2431.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f520(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f521(v0: Fn16, p2439: u64, p2440: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2438 = (p2439, p2440);
    {
        let mut m2438 = m2438;
        loop {
            let step = {
                let (v1, v2) = m2438;
                let m2435 = v1;
                if m2435 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2435 - 1;
                    let m2437 = {
                        let c2436: Fn16 = v0.clone();
                        c2436.apply(v2.clone())?
                    };
                    match m2437 {
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
                Ok(next) => { m2438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f550(v0)
}

pub fn f523(v0: u8) -> bool {
    let a2441 = v0;
    let a2442 = 32u8;
    if if if equal(a2441, a2442) {
        true
    } else {
        let a2443 = v0;
        let a2444 = 9u8;
        equal(a2443, a2444)
    } {
        true
    } else {
        let a2445 = v0;
        let a2446 = 10u8;
        equal(a2445, a2446)
    } {
        true
    } else {
        let a2447 = v0;
        let a2448 = 12u8;
        if equal(a2447, a2448) {
            true
        } else {
            let a2449 = v0;
            let a2450 = 13u8;
            equal(a2449, a2450)
        }
    }
}

pub fn f524(v0: Fn17, p2455: u64, p2456: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2454 = (p2455, p2456);
    {
        let mut m2454 = m2454;
        loop {
            let step = {
                let (v1, v2) = m2454;
                let m2451 = v1;
                if m2451 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2451 - 1;
                    let m2453 = {
                        let c2452: Fn17 = v0.clone();
                        c2452.apply(v2.clone())?
                    };
                    match m2453 {
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
                Ok(next) => { m2454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f551(v0, v1)
}

pub fn f526(v0: u8) -> R<bool> {
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
    let a2503 = 48u64;
    let a2504 = v2;
    if nat_le(a2503, a2504) {
        let a2505 = v2;
        let a2506 = 57u64;
        Ok(nat_le(a2505, a2506))
    } else {
        Ok(false)
    }
}

pub fn f527(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f437({
        let (_, h2507) = {
            let (h2508, _) = f552(Fn20::F553, {
                let a2510 = {
                    let a2509 = v0.clone();
                    length_list(a2509)
                };
                let a2511 = 1u64;
                nat_add(a2510, a2511)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2508
        };
        h2507
    }))
}

pub fn f528(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f529(v0: List<Adt14>) -> List<Bytes> {
    let m2512 = v0;
    match m2512.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2513 = f94(v1, f8(Str::lit("std")));
            match m2513 {
                None => {
                    f529(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f529(v2))
                }
            }
        }
    }
}

pub fn f530(v0: List<Adt14>) -> List<Bytes> {
    let m2514 = v0;
    match m2514.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2515 = f93(f94(v1, f8(Str::lit("req"))));
            match m2515 {
                None => {
                    f530(v2)
                }
                Some(v3) => {
                    List::cons(v3, f530(v2))
                }
            }
        }
    }
}

pub fn f531(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2516 = v0;
    match m2516 {
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
        Adt2::C7(r2517) => {
            let v4 = (*r2517).clone();
            f531(v4)
        }
        Adt2::C8 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C9(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(v1, r2518, r2519) => {
            let v2 = (*r2518).clone();
            let v3 = (*r2519).clone();
            List::cons((v1, v2), f531(v3))
        }
    }
}

pub fn f532(p2524: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2523 = p2524;
    {
        let mut m2523 = m2523;
        loop {
            let step = {
                let v0 = m2523;
                let m2520 = v0;
                match m2520.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f554(f555({
                            let (_, h2521) = v3.clone();
                            h2521
                        }, List::<Bytes>::nil())?, f534({
                            let (h2522, _) = v3.clone();
                            h2522
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f533(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2525 = v0;
    match m2525.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2526, _) = v2.clone();
                h2526
            }, v1.clone()) {
                f533(v3, v1)
            } else {
                List::cons(v2, f533(v3, v1))
            }
        }
    }
}

pub fn f534(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f377(v0)?;
    if f52(v1.clone(), f556()) {
        f557(v1)
    } else {
        Ok(v1)
    }
}

pub fn f535() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f536() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f537(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    f558(v0, f559(v1))
}

pub fn f538(v0: Adt28) -> List<Adt25> {
    let m2527 = v0;
    match m2527 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f539(v0: Adt28) -> Bytes {
    let m2528 = v0;
    match m2528 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f540(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2533 = {
        let a2530 = f457({
            let (h2529, _) = v0;
            h2529
        })?;
        let a2531 = Bytes::lit(&[0x3a]);
        append_bytes(a2530, a2531)
    };
    let a2534 = {
        let (_, h2532) = v0;
        h2532
    };
    Ok(append_bytes(a2533, a2534))
}

pub fn f541(v0: Fn18, p2538: u64, p2539: Adt27) -> R<Adt27> {
    let m2537 = (p2538, p2539);
    {
        let mut m2537 = m2537;
        loop {
            let step = {
                let (v1, v2) = m2537;
                let m2535 = v1;
                if m2535 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2535 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2536: Fn18 = v0.clone();
                        c2536.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542(v0: Adt27) -> R<Adt27> {
    f560(v0)
}

pub fn f543(v0: u32, v1: u32) -> u32 {
    let m2542 = {
        let a2540 = f561(v0);
        let a2541 = f561(v1);
        fixed_u64::checked_add(a2540, a2541)
    };
    match m2542 {
        None => {
            0u32
        }
        Some(v2) => {
            f562(v2)
        }
    }
}

pub fn f544(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2545 = {
        let a2543 = v3;
        let a2544 = 8u64;
        nat_add(a2543, a2544)?
    };
    let a2546 = v2;
    if nat_eq(a2545, a2546) {
        Ok(f563({
            let a2549 = {
                let a2547 = v1;
                let a2548 = 8u64;
                nat_mul(a2547, a2548)?
            };
            let a2550 = 4294967296u64;
            let a2551 = 0u64;
            nat_quot(a2549, a2550, a2551)
        }))
    } else {
        let a2554 = {
            let a2552 = v3;
            let a2553 = 4u64;
            nat_add(a2552, a2553)?
        };
        let a2555 = v2;
        if nat_eq(a2554, a2555) {
            Ok(f563({
                let a2558 = {
                    let a2556 = v1;
                    let a2557 = 8u64;
                    nat_mul(a2556, a2557)?
                };
                let a2559 = 4294967296u64;
                let a2560 = 0u64;
                nat_rem(a2558, a2559, a2560)
            }))
        } else {
            Ok(f564(f565(v0.clone(), v1, v3), f565(v0.clone(), v1, {
                let a2561 = v3;
                let a2562 = 1u64;
                nat_add(a2561, a2562)?
            }), f565(v0.clone(), v1, {
                let a2563 = v3;
                let a2564 = 2u64;
                nat_add(a2563, a2564)?
            }), f565(v0, v1, {
                let a2565 = v3;
                let a2566 = 3u64;
                nat_add(a2565, a2566)?
            })))
        }
    }
}

pub fn f545(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2567 = v0;
    match m2567.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f545(v3, false))
            } else {
                f545(v3, true)
            }
        }
    }
}

pub fn f546(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2568, _) = v2;
        h2568
    };
    let a2569 = v3;
    let a2570 = 0u64;
    if nat_eq(a2569, a2570) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2571 = v3;
            let a2572 = v1;
            let a2573 = 0u64;
            nat_quot(a2571, a2572, a2573)
        }, {
            let a2578 = f515(v0, {
                let a2574 = v3;
                let a2575 = v1;
                let a2576 = 0u64;
                nat_rem(a2574, a2575, a2576)
            });
            let a2579 = {
                let (_, h2577) = v2;
                h2577
            };
            append_bytes(a2578, a2579)
        }))
    }
}

pub fn f547(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2624 = {
            let a2600 = {
                let a2588 = {
                    let a2582 = {
                        let a2580 = v1;
                        let a2581 = 128u8;
                        fixed_u8::bit_and(a2580, a2581)
                    };
                    let a2583 = 128u8;
                    if equal(a2582, a2583) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2589 = {
                    let a2586 = {
                        let a2584 = v1;
                        let a2585 = 64u8;
                        fixed_u8::bit_and(a2584, a2585)
                    };
                    let a2587 = 64u8;
                    if equal(a2586, a2587) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2588, a2589)?
            };
            let a2601 = {
                let a2598 = {
                    let a2592 = {
                        let a2590 = v1;
                        let a2591 = 32u8;
                        fixed_u8::bit_and(a2590, a2591)
                    };
                    let a2593 = 32u8;
                    if equal(a2592, a2593) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2599 = {
                    let a2596 = {
                        let a2594 = v1;
                        let a2595 = 16u8;
                        fixed_u8::bit_and(a2594, a2595)
                    };
                    let a2597 = 16u8;
                    if equal(a2596, a2597) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2598, a2599)?
            };
            nat_add(a2600, a2601)?
        };
        let a2625 = {
            let a2622 = {
                let a2610 = {
                    let a2604 = {
                        let a2602 = v1;
                        let a2603 = 8u8;
                        fixed_u8::bit_and(a2602, a2603)
                    };
                    let a2605 = 8u8;
                    if equal(a2604, a2605) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2611 = {
                    let a2608 = {
                        let a2606 = v1;
                        let a2607 = 4u8;
                        fixed_u8::bit_and(a2606, a2607)
                    };
                    let a2609 = 4u8;
                    if equal(a2608, a2609) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2610, a2611)?
            };
            let a2623 = {
                let a2620 = {
                    let a2614 = {
                        let a2612 = v1;
                        let a2613 = 2u8;
                        fixed_u8::bit_and(a2612, a2613)
                    };
                    let a2615 = 2u8;
                    if equal(a2614, a2615) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2621 = {
                    let a2618 = {
                        let a2616 = v1;
                        let a2617 = 1u8;
                        fixed_u8::bit_and(a2616, a2617)
                    };
                    let a2619 = 1u8;
                    if equal(a2618, a2619) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2620, a2621)?
            };
            nat_add(a2622, a2623)?
        };
        nat_add(a2624, a2625)?
    };
    let a2632 = f515(f566(), {
        let a2626 = v2;
        let a2627 = 16u64;
        let a2628 = 0u64;
        nat_quot(a2626, a2627, a2628)
    });
    let a2633 = f515(f566(), {
        let a2629 = v2;
        let a2630 = 16u64;
        let a2631 = 0u64;
        nat_rem(a2629, a2630, a2631)
    });
    Ok(append_bytes(a2632, a2633))
}

pub fn f548(v0: Fn19, p2637: List<List<(Bytes, Bytes)>>, p2638: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2636 = (p2637, p2638);
    {
        let mut m2636 = m2636;
        loop {
            let step = {
                let (v1, v2) = m2636;
                let m2634 = v2;
                match m2634.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2635: Fn19 = v0.clone();
                            c2635.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2636 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f550(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2640 = {
        let a2639 = v0.clone();
        length_list(a2639)
    };
    let a2641 = 1u64;
    if nat_le(a2640, a2641) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f567(v0)?))
    }
}

pub fn f551(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2642, _) = v1;
        h2642
    };
    let m2644 = {
        let (_, h2643) = v1;
        h2643
    };
    match m2644 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2647 = {
                let a2645 = v0.clone();
                let a2646 = v2;
                index_bytes(a2645, a2646)
            };
            match m2647 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2652 = {
                        let a2650 = v0;
                        let a2651 = {
                            let a2648 = v2;
                            let a2649 = 1u64;
                            nat_add(a2648, a2649)?
                        };
                        index_bytes(a2650, a2651)
                    };
                    match m2652 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f568(v4)? {
                                f568(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2653 = v2;
                                    let a2654 = 2u64;
                                    nat_add(a2653, a2654)?
                                }, Some({
                                    let a2659 = v3;
                                    let a2660 = f515(f569(), {
                                        let a2657 = {
                                            let a2655 = f570(v4)?;
                                            let a2656 = 16u64;
                                            nat_mul(a2655, a2656)?
                                        };
                                        let a2658 = f570(v5)?;
                                        nat_add(a2657, a2658)?
                                    });
                                    append_bytes(a2659, a2660)
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

pub fn f552(v0: Fn20, p2665: u64, p2666: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2664 = (p2665, p2666);
    {
        let mut m2664 = m2664;
        loop {
            let step = {
                let (v1, v2) = m2664;
                let m2661 = v1;
                if m2661 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2661 - 1;
                    let m2663 = {
                        let c2662: Fn20 = v0.clone();
                        c2662.apply(v2.clone())?
                    };
                    match m2663 {
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
                Ok(next) => { m2664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f571(v0)
}

pub fn f554(p2669: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2668 = p2669;
    {
        let mut m2668 = m2668;
        loop {
            let step = {
                let v0 = m2668;
                let m2667 = v0;
                match m2667.uncons() {
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
                Ok(next) => { m2668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2670 = v0;
    match m2670 {
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
        Adt2::C6(r2673) => {
            let v2 = (*r2673).clone();
            f555(v2, v1)
        }
        Adt2::C7(r2674) => {
            let v3 = (*r2674).clone();
            f555(v3, v1)
        }
        Adt2::C8 => {
            Ok(v1)
        }
        Adt2::C9(r2675, r2676) => {
            let v4 = (*r2675).clone();
            let v5 = (*r2676).clone();
            f555(v4, f555(v5, v1)?)
        }
        Adt2::C10(v6, r2677, r2678) => {
            let v7 = (*r2677).clone();
            let v8 = (*r2678).clone();
            let v9: List<Bytes> = f555(v7.clone(), f555(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2671 = f383(Some(v7));
                match m2671 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2672 = f572(v10)?;
                        match m2672 {
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

pub fn f556() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f557(v0: Bytes) -> R<Bytes> {
    let m2681 = {
        let a2679 = v0.clone();
        let a2680 = 0u64;
        index_bytes(a2679, a2680)
    };
    match m2681 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2687 = {
                let a2682 = f8(Str::lit("u"));
                let a2683 = f573(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2682, a2683)
            };
            let a2688 = f46(v0.clone(), 1u64, {
                let a2685 = {
                    let a2684 = v0;
                    length_bytes(a2684)
                };
                let a2686 = 1u64;
                nat_sub(a2685, a2686)
            });
            Ok(append_bytes(a2687, a2688))
        }
    }
}

pub fn f558(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    let m2689 = f280(v1.clone(), f8(Str::lit("enum")));
    match m2689 {
        None => {
            let m2691 = f280(v1.clone(), f8(Str::lit("oneOf")));
            match m2691 {
                None => {
                    if f578(v1.clone(), v0.clone())? {
                        let m2692 = f579(v0.clone(), f498(f280(v1.clone(), f8(Str::lit("properties")))), f284(f285(f280(v1, f8(Str::lit("required"))))), true)?;
                        match m2692 {
                            Ok(v7) => {
                                Ok(f576(f580(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f391(f581(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f582(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2693 = f498(f280(v1.clone(), f8(Str::lit("properties"))));
                        match m2693.uncons() {
                            None => {
                                if f583(v1.clone()) {
                                    Ok(f576(f584(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2696 = f585(v1, {
                                        let a2694 = v0.clone();
                                        let a2695 = f8(Str::lit("Value"));
                                        append_bytes(a2694, a2695)
                                    })?;
                                    match m2696 {
                                        Ok(v9) => {
                                            Ok(f576(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f539(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f538(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2697 = f579(v0.clone(), f498(f280(v1.clone(), f8(Str::lit("properties")))), f284(f285(f280(v1, f8(Str::lit("required"))))), false)?;
                                match m2697 {
                                    Ok(v13) => {
                                        Ok(f576(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2698 = f581(v13.clone());
                                            match m2698.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f391(f581(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f582(v13)))
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
                    f577(v0, f384(v5))
                }
            }
        }
        Some(v2) => {
            let m2690 = f574(f384(v2), List::<Bytes>::nil())?;
            match m2690 {
                Ok(v4) => {
                    Ok(f576(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f391(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f575(v3, v0))
                }
            }
        }
    }
}

pub fn f559(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2699 = f280(v0.clone(), f8(Str::lit("anyOf")));
        match m2699 {
            None => {
                v0
            }
            Some(v1) => {
                if f586(f384(v1.clone())) {
                    f587(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f588(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f589(f384(v1.clone()))
                    } {
                        v0
                    } else {
                        f590(f587(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2700 = f280(v2.clone(), f8(Str::lit("oneOf")));
        match m2700 {
            None => {
                v2
            }
            Some(v3) => {
                if f586(f384(v3)) {
                    f587(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2701 = f280(v4.clone(), f8(Str::lit("allOf")));
    match m2701 {
        None => {
            v4
        }
        Some(v5) => {
            if f591(f384(v5)) {
                f587(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f560(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f592(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2702) => {
            h2702
        }
    });
    let v2: u32 = f543(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2703, _, _) => {
            h2703
        }
    }, f543(f593(match v0.clone() {
        Adt27::C0(_, _, _, _, h2704, _, _, _, _, _) => {
            h2704
        }
    }), f543(f594(match v0.clone() {
        Adt27::C0(_, _, _, _, h2705, _, _, _, _, _) => {
            h2705
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2706, _, _, _, _) => {
            h2706
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2707, _, _, _) => {
            h2707
        }
    }), f543(v1, {
        let m2710 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2709, _) => {
                h2709
            }
        };
        match m2710 {
            Adt26::C0(h2708, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2708
            }
        }
    }))));
    let v3: u32 = f543(f595(match v0.clone() {
        Adt27::C0(h2711, _, _, _, _, _, _, _, _, _) => {
            h2711
        }
    }), f596(match v0.clone() {
        Adt27::C0(h2712, _, _, _, _, _, _, _, _, _) => {
            h2712
        }
    }, match v0.clone() {
        Adt27::C0(_, h2713, _, _, _, _, _, _, _, _) => {
            h2713
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2714, _, _, _, _, _, _, _) => {
            h2714
        }
    }));
    Ok(Adt27::C0(f543(v2, v3), match v0.clone() {
        Adt27::C0(h2715, _, _, _, _, _, _, _, _, _) => {
            h2715
        }
    }, match v0.clone() {
        Adt27::C0(_, h2716, _, _, _, _, _, _, _, _) => {
            h2716
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2717, _, _, _, _, _, _, _) => {
            h2717
        }
    }, f543(match v0.clone() {
        Adt27::C0(_, _, _, h2718, _, _, _, _, _, _) => {
            h2718
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2719, _, _, _, _, _) => {
            h2719
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2720, _, _, _, _) => {
            h2720
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2721, _, _, _) => {
            h2721
        }
    }, f597(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2722, _) => {
            h2722
        }
    }), {
        let a2724 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2723) => {
                h2723
            }
        };
        let a2725 = 1u64;
        nat_add(a2724, a2725)?
    }))
}

pub fn f561(v0: u32) -> u64 {
    let m2727 = {
        let a2726 = v0;
        fixed_u64::convert(i128::from(a2726))
    };
    match m2727 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f562(v0: u64) -> u32 {
    let m2731 = {
        let a2730 = {
            let a2728 = v0;
            let a2729 = 4294967295u64;
            fixed_u64::bit_and(a2728, a2729)
        };
        fixed_u32::convert(i128::from(a2730))
    };
    match m2731 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f563(v0: u64) -> u32 {
    let (_, h2732) = {
        let (_, h2733) = f598(Fn21::F599, 32u64, (v0, (1u32, 0u32)));
        h2733
    };
    h2732
}

pub fn f564(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2738 = f600(v0, 24u32);
    let a2739 = {
        let a2736 = f600(v1, 16u32);
        let a2737 = {
            let a2734 = f600(v2, 8u32);
            let a2735 = v3;
            fixed_u32::bit_or(a2734, a2735)
        };
        fixed_u32::bit_or(a2736, a2737)
    };
    fixed_u32::bit_or(a2738, a2739)
}

pub fn f565(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2740 = v2;
    let a2741 = v1;
    if nat_lt(a2740, a2741) {
        let m2744 = {
            let a2742 = v0;
            let a2743 = v2;
            index_bytes(a2742, a2743)
        };
        match m2744 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2746 = {
                    let a2745 = v3;
                    fixed_u32::convert(i128::from(a2745))
                };
                match m2746 {
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
        let a2747 = v2;
        let a2748 = v1;
        if nat_eq(a2747, a2748) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f566() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f567(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f518({
        let (_, h2749) = {
            let (h2750, _) = f601(Fn22::F602, {
                let a2752 = {
                    let a2751 = v0.clone();
                    length_list(a2751)
                };
                let a2753 = 1u64;
                nat_add(a2752, a2753)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2750
        };
        h2749
    }))
}

pub fn f568(v0: u8) -> R<bool> {
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
    let a2800 = 48u64;
    let a2801 = v2;
    if if nat_le(a2800, a2801) {
        let a2802 = v2;
        let a2803 = 57u64;
        nat_le(a2802, a2803)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2804 = 97u64;
        let a2805 = v2;
        if nat_le(a2804, a2805) {
            let a2806 = v2;
            let a2807 = 102u64;
            Ok(nat_le(a2806, a2807))
        } else {
            Ok(false)
        }
    }
}

pub fn f569() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f570(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2852 = {
            let a2828 = {
                let a2816 = {
                    let a2810 = {
                        let a2808 = v1;
                        let a2809 = 128u8;
                        fixed_u8::bit_and(a2808, a2809)
                    };
                    let a2811 = 128u8;
                    if equal(a2810, a2811) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2817 = {
                    let a2814 = {
                        let a2812 = v1;
                        let a2813 = 64u8;
                        fixed_u8::bit_and(a2812, a2813)
                    };
                    let a2815 = 64u8;
                    if equal(a2814, a2815) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2816, a2817)?
            };
            let a2829 = {
                let a2826 = {
                    let a2820 = {
                        let a2818 = v1;
                        let a2819 = 32u8;
                        fixed_u8::bit_and(a2818, a2819)
                    };
                    let a2821 = 32u8;
                    if equal(a2820, a2821) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2827 = {
                    let a2824 = {
                        let a2822 = v1;
                        let a2823 = 16u8;
                        fixed_u8::bit_and(a2822, a2823)
                    };
                    let a2825 = 16u8;
                    if equal(a2824, a2825) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2826, a2827)?
            };
            nat_add(a2828, a2829)?
        };
        let a2853 = {
            let a2850 = {
                let a2838 = {
                    let a2832 = {
                        let a2830 = v1;
                        let a2831 = 8u8;
                        fixed_u8::bit_and(a2830, a2831)
                    };
                    let a2833 = 8u8;
                    if equal(a2832, a2833) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2839 = {
                    let a2836 = {
                        let a2834 = v1;
                        let a2835 = 4u8;
                        fixed_u8::bit_and(a2834, a2835)
                    };
                    let a2837 = 4u8;
                    if equal(a2836, a2837) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2838, a2839)?
            };
            let a2851 = {
                let a2848 = {
                    let a2842 = {
                        let a2840 = v1;
                        let a2841 = 2u8;
                        fixed_u8::bit_and(a2840, a2841)
                    };
                    let a2843 = 2u8;
                    if equal(a2842, a2843) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2849 = {
                    let a2846 = {
                        let a2844 = v1;
                        let a2845 = 1u8;
                        fixed_u8::bit_and(a2844, a2845)
                    };
                    let a2847 = 1u8;
                    if equal(a2846, a2847) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2848, a2849)?
            };
            nat_add(a2850, a2851)?
        };
        nat_add(a2852, a2853)?
    };
    let a2854 = v2;
    let a2855 = 57u64;
    if nat_le(a2854, a2855) {
        let a2856 = v2;
        let a2857 = 48u64;
        Ok(nat_sub(a2856, a2857))
    } else {
        let a2858 = v2;
        let a2859 = 87u64;
        Ok(nat_sub(a2858, a2859))
    }
}

pub fn f571(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2860, _) = v0;
        h2860
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2861) = v0;
        h2861
    };
    let m2862 = v1;
    match m2862.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2863 = v4;
            match m2863.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f603(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f572(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f534(f46(v0.clone(), 8u64, {
            let a2865 = {
                let a2864 = v0;
                length_bytes(a2864)
            };
            let a2866 = 8u64;
            nat_sub(a2865, a2866)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f534(f46(v0.clone(), 14u64, {
                let a2868 = {
                    let a2867 = v0;
                    length_bytes(a2867)
                };
                let a2869 = 14u64;
                nat_sub(a2868, a2869)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f573(p2886: u64, p2887: u64, p2888: u64, p2889: Bytes) -> Bytes {
    let m2885 = (p2886, (p2887, (p2888, p2889)));
    {
        let mut m2885 = m2885;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2885;
                let m2870 = v0;
                if m2870 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2870 - 1;
                    if f118({
                        let a2871 = v1;
                        let a2872 = 0u64;
                        nat_eq(a2871, a2872)
                    }, {
                        let a2873 = v2;
                        let a2874 = 0u64;
                        nat_eq(a2873, a2874)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2875 = v1;
                            let a2876 = 16u64;
                            let a2877 = 0u64;
                            nat_quot(a2875, a2876, a2877)
                        }, ({
                            let a2878 = v2;
                            let a2879 = 1u64;
                            nat_sub(a2878, a2879)
                        }, {
                            let a2883 = f604({
                                let a2880 = v1;
                                let a2881 = 16u64;
                                let a2882 = 0u64;
                                nat_rem(a2880, a2881, a2882)
                            });
                            let a2884 = v3;
                            append_bytes(a2883, a2884)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f574(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f605(f606(f284(v0.clone()))?, v0, v1)
}

pub fn f575(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f576(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f577(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt28, Bytes>> {
    let m2890 = v1.clone();
    match m2890.uncons() {
        None => {
            Ok(f575(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2891 = f607(f498(f280(v2, f8(Str::lit("properties")))), v1.clone());
            match m2891.uncons() {
                None => {
                    let m2892 = f608(f606(f609(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2892 {
                        Ok(v5) => {
                            Ok(f576(f580(v0, f581(v5.clone())), f582(v5)))
                        }
                        Err(v4) => {
                            Ok(f575(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2893 = f610(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2893 {
                        Ok(v9) => {
                            Ok(f576(f580(v0, f581(v9.clone())), f582(v9)))
                        }
                        Err(v8) => {
                            Ok(f575(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f578(v0: Adt2, v1: Bytes) -> R<bool> {
    let m2894 = v0;
    match m2894 {
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
        Adt2::C6(r2895) => {
            let v2 = (*r2895).clone();
            f578(v2, v1)
        }
        Adt2::C7(r2896) => {
            let v3 = (*r2896).clone();
            f578(v3, v1)
        }
        Adt2::C8 => {
            Ok(false)
        }
        Adt2::C9(r2897, r2898) => {
            let v4 = (*r2897).clone();
            let v5 = (*r2898).clone();
            if f578(v4, v1.clone())? {
                Ok(true)
            } else {
                f578(v5, v1)
            }
        }
        Adt2::C10(v6, r2899, r2900) => {
            let v7 = (*r2899).clone();
            let v8 = (*r2900).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f611(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f578(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f578(v8, v1)
                }
            }
        }
    }
}

pub fn f579(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f612(f606(f613(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f580(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f391(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f581(v0: Adt29) -> List<Bytes> {
    let m2901 = v0;
    match m2901 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f582(v0: Adt29) -> List<Adt25> {
    let m2902 = v0;
    match m2902 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f583(v0: Adt2) -> bool {
    let m2903 = f498(f280(v0.clone(), f8(Str::lit("properties"))));
    match m2903.uncons() {
        None => {
            let m2904 = f614(v0);
            match m2904 {
                None => {
                    false
                }
                Some(v3) => {
                    f615(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f584(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f585(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2905 = f616(v0.clone());
    match m2905 {
        None => {
            if f32(f283(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2906 = f280(v0, f8(Str::lit("items")));
                match m2906 {
                    None => {
                        Ok(f617(f8(Str::lit("List")), f619()))
                    }
                    Some(v3) => {
                        Ok(f617(f8(Str::lit("List")), f618(v3, {
                            let a2907 = v1;
                            let a2908 = f8(Str::lit("Item"));
                            append_bytes(a2907, a2908)
                        })?))
                    }
                }
            } else {
                f618(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f617(f8(Str::lit("Option")), f618(v2, v1)?))
        }
    }
}

pub fn f586(p2911: List<Adt2>) -> bool {
    let m2910 = p2911;
    {
        let mut m2910 = m2910;
        loop {
            let step = {
                let v0 = m2910;
                let m2909 = v0;
                match m2909.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f620(f531(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2910 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(v0: Adt2, v1: Bytes) -> Adt2 {
    let m2912 = v0;
    match m2912 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r2913) => {
            let v7 = (*r2913).clone();
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
        Adt2::C6(r2914) => {
            let v15 = (*r2914).clone();
            Adt2::C6(std::rc::Rc::new(v15))
        }
        Adt2::C7(r2915) => {
            let v5 = (*r2915).clone();
            Adt2::C7(std::rc::Rc::new(f587(v5, v1)))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r2916, r2917) => {
            let v16 = (*r2916).clone();
            let v17 = (*r2917).clone();
            Adt2::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt2::C10(v2, r2918, r2919) => {
            let v3 = (*r2918).clone();
            let v4 = (*r2919).clone();
            if f32(v2.clone(), v1.clone()) {
                f587(v4, v1)
            } else {
                Adt2::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f587(v4, v1)))
            }
        }
    }
}

pub fn f588(v0: Adt2, v1: Bytes) -> bool {
    let m2920 = f280(v0, v1);
    match m2920 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f589(v0: List<Adt2>) -> bool {
    let m2921 = v0.clone();
    match m2921.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2922 = f607(f498(f280(v1, f8(Str::lit("properties")))), v0.clone());
            match m2922.uncons() {
                None => {
                    f622(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f621(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f590(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m2923 = v0;
    match m2923 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r2924) => {
            let v8 = (*r2924).clone();
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
        Adt2::C6(r2925) => {
            let v16 = (*r2925).clone();
            Adt2::C6(std::rc::Rc::new(v16))
        }
        Adt2::C7(r2926) => {
            let v3 = (*r2926).clone();
            Adt2::C7(std::rc::Rc::new(Adt2::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r2927, r2928) => {
            let v17 = (*r2927).clone();
            let v18 = (*r2928).clone();
            Adt2::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt2::C10(v4, r2929, r2930) => {
            let v5 = (*r2929).clone();
            let v6 = (*r2930).clone();
            Adt2::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f591(p2933: List<Adt2>) -> bool {
    let m2932 = p2933;
    {
        let mut m2932 = m2932;
        loop {
            let step = {
                let v0 = m2932;
                let m2931 = v0;
                match m2931.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f623(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
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

pub fn f592(v0: u64) -> u32 {
    let a2934 = v0;
    let a2935 = 32u64;
    if nat_lt(a2934, a2935) {
        let a2936 = v0;
        let a2937 = 16u64;
        if nat_lt(a2936, a2937) {
            let a2938 = v0;
            let a2939 = 8u64;
            if nat_lt(a2938, a2939) {
                let a2940 = v0;
                let a2941 = 4u64;
                if nat_lt(a2940, a2941) {
                    let a2942 = v0;
                    let a2943 = 2u64;
                    if nat_lt(a2942, a2943) {
                        let a2944 = v0;
                        let a2945 = 1u64;
                        if nat_lt(a2944, a2945) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2946 = v0;
                        let a2947 = 3u64;
                        if nat_lt(a2946, a2947) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2948 = v0;
                    let a2949 = 6u64;
                    if nat_lt(a2948, a2949) {
                        let a2950 = v0;
                        let a2951 = 5u64;
                        if nat_lt(a2950, a2951) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2952 = v0;
                        let a2953 = 7u64;
                        if nat_lt(a2952, a2953) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2954 = v0;
                let a2955 = 12u64;
                if nat_lt(a2954, a2955) {
                    let a2956 = v0;
                    let a2957 = 10u64;
                    if nat_lt(a2956, a2957) {
                        let a2958 = v0;
                        let a2959 = 9u64;
                        if nat_lt(a2958, a2959) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2960 = v0;
                        let a2961 = 11u64;
                        if nat_lt(a2960, a2961) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2962 = v0;
                    let a2963 = 14u64;
                    if nat_lt(a2962, a2963) {
                        let a2964 = v0;
                        let a2965 = 13u64;
                        if nat_lt(a2964, a2965) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2966 = v0;
                        let a2967 = 15u64;
                        if nat_lt(a2966, a2967) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2968 = v0;
            let a2969 = 24u64;
            if nat_lt(a2968, a2969) {
                let a2970 = v0;
                let a2971 = 20u64;
                if nat_lt(a2970, a2971) {
                    let a2972 = v0;
                    let a2973 = 18u64;
                    if nat_lt(a2972, a2973) {
                        let a2974 = v0;
                        let a2975 = 17u64;
                        if nat_lt(a2974, a2975) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2976 = v0;
                        let a2977 = 19u64;
                        if nat_lt(a2976, a2977) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2978 = v0;
                    let a2979 = 22u64;
                    if nat_lt(a2978, a2979) {
                        let a2980 = v0;
                        let a2981 = 21u64;
                        if nat_lt(a2980, a2981) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2982 = v0;
                        let a2983 = 23u64;
                        if nat_lt(a2982, a2983) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2984 = v0;
                let a2985 = 28u64;
                if nat_lt(a2984, a2985) {
                    let a2986 = v0;
                    let a2987 = 26u64;
                    if nat_lt(a2986, a2987) {
                        let a2988 = v0;
                        let a2989 = 25u64;
                        if nat_lt(a2988, a2989) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2990 = v0;
                        let a2991 = 27u64;
                        if nat_lt(a2990, a2991) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2992 = v0;
                    let a2993 = 30u64;
                    if nat_lt(a2992, a2993) {
                        let a2994 = v0;
                        let a2995 = 29u64;
                        if nat_lt(a2994, a2995) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2996 = v0;
                        let a2997 = 31u64;
                        if nat_lt(a2996, a2997) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2998 = v0;
        let a2999 = 48u64;
        if nat_lt(a2998, a2999) {
            let a3000 = v0;
            let a3001 = 40u64;
            if nat_lt(a3000, a3001) {
                let a3002 = v0;
                let a3003 = 36u64;
                if nat_lt(a3002, a3003) {
                    let a3004 = v0;
                    let a3005 = 34u64;
                    if nat_lt(a3004, a3005) {
                        let a3006 = v0;
                        let a3007 = 33u64;
                        if nat_lt(a3006, a3007) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3008 = v0;
                        let a3009 = 35u64;
                        if nat_lt(a3008, a3009) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3010 = v0;
                    let a3011 = 38u64;
                    if nat_lt(a3010, a3011) {
                        let a3012 = v0;
                        let a3013 = 37u64;
                        if nat_lt(a3012, a3013) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3014 = v0;
                        let a3015 = 39u64;
                        if nat_lt(a3014, a3015) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3016 = v0;
                let a3017 = 44u64;
                if nat_lt(a3016, a3017) {
                    let a3018 = v0;
                    let a3019 = 42u64;
                    if nat_lt(a3018, a3019) {
                        let a3020 = v0;
                        let a3021 = 41u64;
                        if nat_lt(a3020, a3021) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3022 = v0;
                        let a3023 = 43u64;
                        if nat_lt(a3022, a3023) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3024 = v0;
                    let a3025 = 46u64;
                    if nat_lt(a3024, a3025) {
                        let a3026 = v0;
                        let a3027 = 45u64;
                        if nat_lt(a3026, a3027) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3028 = v0;
                        let a3029 = 47u64;
                        if nat_lt(a3028, a3029) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3030 = v0;
            let a3031 = 56u64;
            if nat_lt(a3030, a3031) {
                let a3032 = v0;
                let a3033 = 52u64;
                if nat_lt(a3032, a3033) {
                    let a3034 = v0;
                    let a3035 = 50u64;
                    if nat_lt(a3034, a3035) {
                        let a3036 = v0;
                        let a3037 = 49u64;
                        if nat_lt(a3036, a3037) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3038 = v0;
                        let a3039 = 51u64;
                        if nat_lt(a3038, a3039) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3040 = v0;
                    let a3041 = 54u64;
                    if nat_lt(a3040, a3041) {
                        let a3042 = v0;
                        let a3043 = 53u64;
                        if nat_lt(a3042, a3043) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3044 = v0;
                        let a3045 = 55u64;
                        if nat_lt(a3044, a3045) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3046 = v0;
                let a3047 = 60u64;
                if nat_lt(a3046, a3047) {
                    let a3048 = v0;
                    let a3049 = 58u64;
                    if nat_lt(a3048, a3049) {
                        let a3050 = v0;
                        let a3051 = 57u64;
                        if nat_lt(a3050, a3051) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3052 = v0;
                        let a3053 = 59u64;
                        if nat_lt(a3052, a3053) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3054 = v0;
                    let a3055 = 62u64;
                    if nat_lt(a3054, a3055) {
                        let a3056 = v0;
                        let a3057 = 61u64;
                        if nat_lt(a3056, a3057) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3058 = v0;
                        let a3059 = 63u64;
                        if nat_lt(a3058, a3059) {
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

pub fn f593(v0: u32) -> u32 {
    let a3062 = {
        let a3060 = f624(v0, 6u32, 26u32);
        let a3061 = f624(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3060, a3061)
    };
    let a3063 = f624(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3062, a3063)
}

pub fn f594(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3069 = {
        let a3064 = v0;
        let a3065 = v1;
        fixed_u32::bit_and(a3064, a3065)
    };
    let a3070 = {
        let a3067 = {
            let a3066 = v0;
            fixed_u32::bit_not(a3066)
        };
        let a3068 = v2;
        fixed_u32::bit_and(a3067, a3068)
    };
    fixed_u32::bit_xor(a3069, a3070)
}

pub fn f595(v0: u32) -> u32 {
    let a3073 = {
        let a3071 = f624(v0, 2u32, 30u32);
        let a3072 = f624(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3071, a3072)
    };
    let a3074 = f624(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3073, a3074)
}

pub fn f596(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3083 = {
        let a3079 = {
            let a3075 = v0;
            let a3076 = v1;
            fixed_u32::bit_and(a3075, a3076)
        };
        let a3080 = {
            let a3077 = v0;
            let a3078 = v2;
            fixed_u32::bit_and(a3077, a3078)
        };
        fixed_u32::bit_xor(a3079, a3080)
    };
    let a3084 = {
        let a3081 = v1;
        let a3082 = v2;
        fixed_u32::bit_and(a3081, a3082)
    };
    fixed_u32::bit_xor(a3083, a3084)
}

pub fn f597(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3085, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3085
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3086, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3086
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3087, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3087
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3088, _, _, _, _, _, _, _, _, _, _, _) => {
            h3088
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3089, _, _, _, _, _, _, _, _, _, _) => {
            h3089
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3090, _, _, _, _, _, _, _, _, _) => {
            h3090
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3091, _, _, _, _, _, _, _, _) => {
            h3091
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3092, _, _, _, _, _, _, _) => {
            h3092
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3093, _, _, _, _, _, _) => {
            h3093
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3094, _, _, _, _, _) => {
            h3094
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3095, _, _, _, _) => {
            h3095
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3096, _, _, _) => {
            h3096
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3097, _, _) => {
            h3097
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3098, _) => {
            h3098
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3099) => {
            h3099
        }
    }, f543(f625(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3100, _) => {
            h3100
        }
    }), f543(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3101, _, _, _, _, _, _) => {
            h3101
        }
    }, f543(f626(match v0.clone() {
        Adt26::C0(_, h3102, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3102
        }
    }), match v0 {
        Adt26::C0(h3103, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3103
        }
    }))))
}

pub fn f598(v0: Fn21, p3107: u64, p3108: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3106 = (p3107, p3108);
    {
        let mut m3106 = m3106;
        loop {
            let step = {
                let (v1, v2) = m3106;
                let m3104 = v1;
                if m3104 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3104 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3105: Fn21 = v0.clone();
                        c3105.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f599(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3109, _) = v0;
        h3109
    };
    let v2: u32 = {
        let (h3110, _) = {
            let (_, h3111) = v0;
            h3111
        };
        h3110
    };
    let v3: u32 = {
        let (_, h3112) = {
            let (_, h3113) = v0;
            h3113
        };
        h3112
    };
    ({
        let a3114 = v1;
        let a3115 = 2u64;
        let a3116 = 0u64;
        nat_quot(a3114, a3115, a3116)
    }, (f600(v2, 1u32), {
        let a3120 = {
            let a3117 = v1;
            let a3118 = 2u64;
            let a3119 = 0u64;
            nat_rem(a3117, a3118, a3119)
        };
        let a3121 = 1u64;
        if nat_eq(a3120, a3121) {
            let a3122 = v3;
            let a3123 = v2;
            fixed_u32::bit_or(a3122, a3123)
        } else {
            v3
        }
    }))
}

pub fn f600(v0: u32, v1: u32) -> u32 {
    let m3126 = {
        let a3124 = v0;
        let a3125 = v1;
        fixed_u32::shift_left(a3124, a3125)
    };
    match m3126 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f601(v0: Fn22, p3131: u64, p3132: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3130 = (p3131, p3132);
    {
        let mut m3130 = m3130;
        loop {
            let step = {
                let (v1, v2) = m3130;
                let m3127 = v1;
                if m3127 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3127 - 1;
                    let m3129 = {
                        let c3128: Fn22 = v0.clone();
                        c3128.apply(v2.clone())?
                    };
                    match m3129 {
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
                Ok(next) => { m3130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f602(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f627(v0)
}

pub fn f603(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3137 = {
            let a3135 = {
                let a3133 = v0.clone();
                length_list(a3133)
            };
            let a3136 = {
                let a3134 = v1.clone();
                length_list(a3134)
            };
            nat_add(a3135, a3136)?
        };
        let a3138 = 1u64;
        nat_add(a3137, a3138)?
    };
    Ok(f442({
        let (_, h3139) = {
            let (_, h3140) = {
                let (h3141, _) = f628(Fn23::F629, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3141
            };
            h3140
        };
        h3139
    }))
}

pub fn f604(v0: u64) -> Bytes {
    f46({
        let a3142 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3142)
    }, v0, 1u64)
}

pub fn f605(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3143 = v1;
    match m3143.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3144 = f630(v0, v3.clone())?;
            match m3144 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f631(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3145 = f605(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3145 {
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

pub fn f606(p3149: List<Bytes>) -> R<bool> {
    let m3148 = p3149;
    {
        let mut m3148 = m3148;
        loop {
            let step = {
                let v0 = m3148;
                let m3146 = v0;
                match m3146.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3147 = f632(v1)?;
                        match m3147 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f633(false, v2.clone())?) {
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
                Ok(next) => { m3148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f607(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3150 = v0;
    match m3150.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f634(v1.clone(), {
                let (h3151, _) = v2.clone();
                h3151
            }) {
                List::cons({
                    let (h3152, _) = v2;
                    h3152
                }, f607(v3, v1))
            } else {
                f607(v3, v1)
            }
        }
    }
}

pub fn f608(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3153 = v2;
    match m3153.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3154 = f635(v4.clone())?;
            match m3154 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3155 = f636(v0, v6.clone())?;
                    match m3155 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3158 = f637(v4, {
                                    let a3156 = v1.clone();
                                    let a3157 = f377(v6)?;
                                    append_bytes(a3156, a3157)
                                })?;
                                match m3158 {
                                    Ok(v9) => {
                                        let m3159 = f608(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3159 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f539(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f581(v11.clone())), f502(f538(v9), f582(v11)))))
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

pub fn f609(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3160 = v0;
    match m3160.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3161 = f635(v1)?;
            match m3161 {
                None => {
                    f609(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f609(v2)?))
                }
            }
        }
    }
}

pub fn f610(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f638(f606(f639(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f611(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3162 = f383(Some(v0));
    match m3162 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3163 = f572(v2)?;
            match m3163 {
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

pub fn f612(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m3164 = v2;
    match m3164.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3166 = f640(v0, {
                let (h3165, _) = v5.clone();
                h3165
            })?;
            match m3166 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3167, _) = v5;
                        h3167
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f641(v0, f613(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3172 = f585({
                            let (_, h3168) = v5.clone();
                            h3168
                        }, {
                            let a3170 = v1.clone();
                            let a3171 = f377({
                                let (h3169, _) = v5.clone();
                                h3169
                            })?;
                            append_bytes(a3170, a3171)
                        })?;
                        match m3172 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3173, _) = v5;
                                    h3173
                                }, v3.clone()) {
                                    f539(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f539(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3174 = f612(v0, v1, v6, v3, v4)?;
                                match m3174 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f581(v12.clone())), f502(f538(v9), f582(v12)))))
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

pub fn f613(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m3175 = v0;
    match m3175.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3176, _) = v1;
                h3176
            }, f613(v2))
        }
    }
}

pub fn f614(v0: Adt2) -> Option<Adt2> {
    let m3177 = f280(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3177 {
        None => {
            f642(v0)
        }
        Some(v1) => {
            if f615(v1.clone()) {
                let m3178 = f642(v0);
                match m3178 {
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

pub fn f615(v0: Adt2) -> bool {
    let m3179 = v0;
    match m3179 {
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

pub fn f616(v0: Adt2) -> Option<Adt2> {
    let m3180 = f285(f280(v0, f8(Str::lit("oneOf"))));
    match m3180.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3181 = v2;
            match m3181.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3182 = v4;
                    match m3182.uncons() {
                        None => {
                            if f643(v3.clone()) {
                                Some(v1)
                            } else {
                                if f643(v1) {
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

pub fn f617(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3183 = v1;
    match m3183 {
        Ok(v3) => {
            f576(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f539(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f538(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f618(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3184 = f616(v0.clone());
    match m3184 {
        None => {
            if f32(f283(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3185 = f280(v0, f8(Str::lit("items")));
                match m3185 {
                    None => {
                        Ok(f617(f8(Str::lit("List")), f619()))
                    }
                    Some(v3) => {
                        Ok(f617(f8(Str::lit("List")), f637(v3, {
                            let a3186 = v1;
                            let a3187 = f8(Str::lit("Item"));
                            append_bytes(a3186, a3187)
                        })?))
                    }
                }
            } else {
                f637(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f617(f8(Str::lit("Option")), f637(v2, v1)?))
        }
    }
}

pub fn f619() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f535(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f620(p3191: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3190 = p3191;
    {
        let mut m3190 = m3190;
        loop {
            let step = {
                let v0 = m3190;
                let m3188 = v0;
                match m3188.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3189, _) = v2;
                            h3189
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(p3195: List<Adt2>, v1: Bytes, p3196: List<Bytes>) -> bool {
    let m3194 = (p3195, p3196);
    {
        let mut m3194 = m3194;
        loop {
            let step = {
                let (v0, v2) = m3194;
                let m3192 = v0;
                match m3192.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3193 = f644(v3, v1.clone());
                        match m3193 {
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
                Ok(next) => { m3194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(p3199: List<Adt2>, p3200: List<Bytes>) -> bool {
    let m3198 = (p3199, p3200);
    {
        let mut m3198 = m3198;
        loop {
            let step = {
                let (v0, v1) = m3198;
                let m3197 = v0;
                match m3197.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f283(v2, f8(Str::lit("type")));
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
                Ok(next) => { m3198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3201 = f280(v0.clone(), f8(Str::lit("anyOf")));
        match m3201 {
            None => {
                Adt2::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f620(f531(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f586(f384(v2))
    } else {
        false
    }
}

pub fn f624(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3202 = f509(v0, v1);
    let a3203 = f600(v0, v2);
    fixed_u32::bit_or(a3202, a3203)
}

pub fn f625(v0: u32) -> u32 {
    let a3206 = {
        let a3204 = f624(v0, 17u32, 15u32);
        let a3205 = f624(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3204, a3205)
    };
    let a3207 = f509(v0, 10u32);
    fixed_u32::bit_xor(a3206, a3207)
}

pub fn f626(v0: u32) -> u32 {
    let a3210 = {
        let a3208 = f624(v0, 7u32, 25u32);
        let a3209 = f624(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3208, a3209)
    };
    let a3211 = f509(v0, 3u32);
    fixed_u32::bit_xor(a3210, a3211)
}

pub fn f627(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3212, _) = v0;
        h3212
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3213) = v0;
        h3213
    };
    let m3214 = v1;
    match m3214.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3215 = v4;
            match m3215.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f645(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f628(v0: Fn23, p3220: u64, p3221: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3219 = (p3220, p3221);
    {
        let mut m3219 = m3219;
        loop {
            let step = {
                let (v1, v2) = m3219;
                let m3216 = v1;
                if m3216 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3216 - 1;
                    let m3218 = {
                        let c3217: Fn23 = v0.clone();
                        c3217.apply(v2.clone())
                    };
                    match m3218 {
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
                Ok(next) => { m3219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f629(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f646(v0)
}

pub fn f630(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3222 = v1;
    match m3222 {
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
                let a3233 = if f118(v7, {
                    let a3231 = 0u64;
                    let a3232 = v8;
                    nat_lt(a3231, a3232)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3234 = f305(false, v8)?;
                append_bytes(a3233, a3234)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3225 = f647({
                let a3223 = f8(Str::lit("n"));
                let a3224 = f456(false, v4, v5)?;
                append_bytes(a3223, a3224)
            })?;
            match m3225 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3229 = f8(Str::lit("nMinus"));
                        let a3230 = f46(v6.clone(), 1u64, {
                            let a3227 = {
                                let a3226 = v6;
                                length_bytes(a3226)
                            };
                            let a3228 = 1u64;
                            nat_sub(a3227, a3228)
                        });
                        append_bytes(a3229, a3230)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f636(v0, v2)
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

pub fn f631(v0: Adt2) -> R<Bytes> {
    f382(v0)
}

pub fn f632(v0: Bytes) -> R<Option<Bytes>> {
    let m3235 = f648(v0)?;
    match m3235 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f649(v1)?))
        }
    }
}

pub fn f633(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3236 = v1;
    match m3236.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3237 = f636(v0, v2)?;
            match m3237 {
                None => {
                    f633(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f633(v0, v3)?))
                }
            }
        }
    }
}

pub fn f634(p3241: List<Adt2>, v1: Bytes) -> bool {
    let m3240 = p3241;
    {
        let mut m3240 = m3240;
        loop {
            let step = {
                let v0 = m3240;
                let m3238 = v0;
                match m3238.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3239 = f644(v2, v1.clone());
                        if match m3239 {
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
                Ok(next) => { m3240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f635(v0: Adt2) -> R<Option<Bytes>> {
    let m3242 = f383(f280(v0.clone(), f8(Str::lit("$ref"))));
    match m3242 {
        None => {
            Ok(f383(f280(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f572(v1)
        }
    }
}

pub fn f636(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f647(v1)
    } else {
        f632(v1)
    }
}

pub fn f637(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f650(f559(v0), v1)
}

pub fn f638(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3243 = v2;
    match m3243.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3244 = f644(v5.clone(), v3.clone());
            match m3244 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3245 = f636(v0, v7.clone())?;
                    match m3245 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3248 = f579({
                                    let a3246 = v1.clone();
                                    let a3247 = f377(v7)?;
                                    append_bytes(a3246, a3247)
                                }, f533(f498(f280(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f284(f285(f280(v5, f8(Str::lit("required"))))), true)?;
                                match m3248 {
                                    Ok(v10) => {
                                        let m3249 = f638(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3249 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f391(f581(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f581(v12.clone())), f502(f582(v10), f582(v12)))))
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

pub fn f639(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3250 = v0;
    match m3250.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3251 = f644(v2, v1.clone());
            match m3251 {
                None => {
                    f639(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f639(v3, v1))
                }
            }
        }
    }
}

pub fn f640(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3252 = f636(v0, v1)?;
    match m3252 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f557(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f641(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3253 = v1;
    match m3253.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3254 = f640(v0, v2)?;
            match m3254 {
                None => {
                    f641(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f641(v0, v3)?))
                }
            }
        }
    }
}

pub fn f642(v0: Adt2) -> Option<Adt2> {
    let m3255 = f498(f280(v0, f8(Str::lit("patternProperties"))));
    match m3255.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3256 = v2;
            match m3256.uncons() {
                None => {
                    Some({
                        let (_, h3257) = v1;
                        h3257
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f643(v0: Adt2) -> bool {
    f32(f283(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f644(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f284(f285(f280(v0.clone(), f8(Str::lit("required")))))) {
        let m3258 = f280(v0, f8(Str::lit("properties")));
        match m3258 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3259 = f280(v2, v1);
                match m3259 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f383(f280(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f645(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3264 = {
            let a3262 = {
                let a3260 = v0.clone();
                length_list(a3260)
            };
            let a3263 = {
                let a3261 = v1.clone();
                length_list(a3261)
            };
            nat_add(a3262, a3263)?
        };
        let a3265 = 1u64;
        nat_add(a3264, a3265)?
    };
    Ok(f651({
        let (_, h3266) = {
            let (_, h3267) = {
                let (h3268, _) = f652(Fn24::F653, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3268
            };
            h3267
        };
        h3266
    }))
}

pub fn f646(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3269, _) = v0.clone();
        h3269
    };
    let v2: List<(Bytes, ())> = {
        let (h3270, _) = {
            let (_, h3271) = v0.clone();
            h3271
        };
        h3270
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3272) = {
            let (_, h3273) = v0;
            h3273
        };
        h3272
    };
    let m3274 = v1.clone();
    match m3274.uncons() {
        None => {
            let m3275 = v2;
            match m3275.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3276 = v2.clone();
            match m3276.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f654({
                        let (h3277, _) = v8.clone();
                        h3277
                    }, {
                        let (h3278, _) = v6.clone();
                        h3278
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

pub fn f647(v0: Bytes) -> R<Option<Bytes>> {
    let m3280 = f655({
        let a3279 = v0.clone();
        length_bytes(a3279)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3280 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3282 = {
                let a3281 = v1.clone();
                length_bytes(a3281)
            };
            let a3283 = 0u64;
            if nat_eq(a3282, a3283) {
                let a3285 = {
                    let a3284 = v0;
                    length_bytes(a3284)
                };
                let a3286 = 0u64;
                if nat_eq(a3285, a3286) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f649(v1)?))
            }
        }
    }
}

pub fn f648(v0: Bytes) -> R<Option<Bytes>> {
    if f656({
        let a3287 = v0.clone();
        length_bytes(a3287)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f657(v0)?;
        if f658(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3290 = {
                let a3288 = v1.clone();
                let a3289 = 0u64;
                index_bytes(a3288, a3289)
            };
            match m3290 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f496(f72(v2)?) {
                        Ok(Some({
                            let a3291 = f8(Str::lit("n"));
                            let a3292 = v1;
                            append_bytes(a3291, a3292)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f659({
            let a3293 = v0.clone();
            length_bytes(a3293)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f649(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f660()) {
        f557(v0)
    } else {
        Ok(v0)
    }
}

pub fn f650(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3294 = v0.clone();
    match m3294 {
        Adt2::C0 => {
            Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f619())
            } else {
                Ok(f575(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_) => {
            Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            let m3295 = f661(f531(v0.clone()));
            match m3295 {
                None => {
                    let m3296 = f383(f280(v0.clone(), f8(Str::lit("$ref"))));
                    match m3296 {
                        None => {
                            let m3298 = f280(v0.clone(), f8(Str::lit("allOf")));
                            match m3298 {
                                None => {
                                    let m3303 = f280(v0.clone(), f8(Str::lit("enum")));
                                    match m3303 {
                                        None => {
                                            let m3304 = f280(v0.clone(), f8(Str::lit("const")));
                                            match m3304 {
                                                None => {
                                                    let m3306 = f280(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3306 {
                                                        None => {
                                                            let v32: Bytes = f283(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v32.clone(), Bytes::lit(&[])) {
                                                                !f662(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f619())
                                                            } else {
                                                                if f32(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f576(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f32(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f576(f663(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f32(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f576(f664(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f32(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f576(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f32(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3307 = f498(f280(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3307.uncons() {
                                                                                        None => {
                                                                                            if f583(v0.clone()) {
                                                                                                Ok(f576(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3308 = f614(v0);
                                                                                                match m3308 {
                                                                                                    None => {
                                                                                                        Ok(f617(f8(Str::lit("List")), f665(f619())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f617(f8(Str::lit("List")), f665(f666(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f576(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f575(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f576(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3305 = f383(Some(v29));
                                                    match m3305 {
                                                        None => {
                                                            Ok(f575(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f576(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f576(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3299 = f384(v21);
                                    match m3299.uncons() {
                                        None => {
                                            Ok(f575(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3300 = v23;
                                            match m3300.uncons() {
                                                None => {
                                                    let m3301 = f383(f280(v22, f8(Str::lit("$ref"))));
                                                    match m3301 {
                                                        None => {
                                                            Ok(f575(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3302 = f572(v26)?;
                                                            match m3302 {
                                                                None => {
                                                                    Ok(f575(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f576(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f575(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3297 = f572(v19)?;
                            match m3297 {
                                None => {
                                    Ok(f575(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f576(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f575(if f32(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C8 => {
            Ok(f619())
        }
        Adt2::C9(_, _) => {
            Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C10(_, _, _) => {
            Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f651(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f403(Fn6::F667, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f652(v0: Fn24, p3313: u64, p3314: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3312 = (p3313, p3314);
    {
        let mut m3312 = m3312;
        loop {
            let step = {
                let (v1, v2) = m3312;
                let m3309 = v1;
                if m3309 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3309 - 1;
                    let m3311 = {
                        let c3310: Fn24 = v0.clone();
                        c3310.apply(v2.clone())
                    };
                    match m3311 {
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
                Ok(next) => { m3312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f653(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f668(v0)
}

pub fn f654(v0: Bytes, v1: Bytes) -> bool {
    let a3319 = {
        let a3315 = v0;
        let a3316 = v1;
        compare_bytes(a3315, a3316)
    };
    let a3320 = {
        let a3317 = Bytes::lit(&[]);
        let a3318 = Bytes::lit(&[0x01]);
        compare_bytes(a3317, a3318)
    };
    equal(a3319, a3320)
}

pub fn f655(p3333: u64, v1: Bytes, p3334: u64, p3335: Bytes) -> R<Option<Bytes>> {
    let m3332 = (p3333, (p3334, p3335));
    {
        let mut m3332 = m3332;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3332;
                let m3321 = v0;
                if m3321 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3321 - 1;
                    let a3323 = {
                        let a3322 = v1.clone();
                        length_bytes(a3322)
                    };
                    let a3324 = v2;
                    if nat_le(a3323, a3324) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3325 = f669(v1.clone(), v2)?;
                        match m3325 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3327 = v2;
                                    let a3328 = {
                                        let (_, h3326) = v5;
                                        h3326
                                    };
                                    nat_add(a3327, a3328)?
                                }, {
                                    let a3330 = v3;
                                    let a3331 = f670(v1.clone(), v2, {
                                        let (h3329, _) = v5;
                                        h3329
                                    });
                                    append_bytes(a3330, a3331)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f656(p3345: u64, v1: Bytes, p3346: u64) -> R<bool> {
    let m3344 = (p3345, p3346);
    {
        let mut m3344 = m3344;
        loop {
            let step = {
                let (v0, v2) = m3344;
                let m3336 = v0;
                if m3336 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3336 - 1;
                    let m3339 = {
                        let a3337 = v1.clone();
                        let a3338 = v2;
                        index_bytes(a3337, a3338)
                    };
                    match m3339 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3340 = f72(v4)?;
                            let a3341 = 128u64;
                            if nat_lt(a3340, a3341) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3342 = v2;
                                    let a3343 = 1u64;
                                    nat_add(a3342, a3343)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f657(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f454(v0.clone(), 0u64, {
        let a3347 = v0;
        length_bytes(a3347)
    }, false, Bytes::lit(&[]))?;
    let m3350 = {
        let a3348 = v1.clone();
        let a3349 = 0u64;
        index_bytes(a3348, a3349)
    };
    match m3350 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f495(v3) {
                let a3357 = f46({
                    let a3351 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3351)
                }, {
                    let a3352 = v3;
                    let a3353 = 65u64;
                    nat_sub(a3352, a3353)
                }, 1u64);
                let a3358 = f46(v1.clone(), 1u64, {
                    let a3355 = {
                        let a3354 = v1;
                        length_bytes(a3354)
                    };
                    let a3356 = 1u64;
                    nat_sub(a3355, a3356)
                });
                Ok(append_bytes(a3357, a3358))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f658(v0: Bytes) -> R<bool> {
    let m3361 = {
        let a3359 = v0;
        let a3360 = 0u64;
        index_bytes(a3359, a3360)
    };
    match m3361 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f208(f494(f72(v1)?), f495(f72(v1)?)))
        }
    }
}

pub fn f659(p3374: u64, v1: Bytes, p3375: u64, p3376: Bytes) -> R<Option<Bytes>> {
    let m3373 = (p3374, (p3375, p3376));
    {
        let mut m3373 = m3373;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3373;
                let m3362 = v0;
                if m3362 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3362 - 1;
                    let a3364 = {
                        let a3363 = v1.clone();
                        length_bytes(a3363)
                    };
                    let a3365 = v2;
                    if nat_le(a3364, a3365) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3366 = f669(v1.clone(), v2)?;
                        match m3366 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3368 = v2;
                                    let a3369 = {
                                        let (_, h3367) = v5;
                                        h3367
                                    };
                                    nat_add(a3368, a3369)?
                                }, {
                                    let a3371 = v3;
                                    let a3372 = f671({
                                        let (h3370, _) = v5;
                                        h3370
                                    });
                                    append_bytes(a3371, a3372)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f660() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f661(p3381: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3380 = p3381;
    {
        let mut m3380 = m3380;
        loop {
            let step = {
                let v0 = m3380;
                let m3377 = v0;
                match m3377.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h3378, _) = v1.clone();
                            h3378
                        }, f672()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3379, _) = v1;
                                h3379
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f662(v0: Adt2) -> bool {
    if f588(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f588(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f588(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f588(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f588(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f588(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f663(v0: Adt2) -> Bytes {
    let m3382 = f673(f280(v0, f8(Str::lit("minimum"))));
    match m3382 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f664() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f665(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3383 = v0;
    match m3383 {
        Ok(v2) => {
            f576(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f539(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f538(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f666(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f674(16u64, v0, v1)
}

pub fn f667(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f668(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3384, _) = v0.clone();
        h3384
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3385, _) = {
            let (_, h3386) = v0.clone();
            h3386
        };
        h3385
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3387) = {
            let (_, h3388) = v0;
            h3388
        };
        h3387
    };
    let m3389 = v1.clone();
    match m3389.uncons() {
        None => {
            let m3390 = v2;
            match m3390.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3391 = v2.clone();
            match m3391.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f654({
                        let (h3392, _) = v8.clone();
                        h3392
                    }, {
                        let (h3393, _) = v6.clone();
                        h3393
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

pub fn f669(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3396 = {
        let a3394 = v0.clone();
        let a3395 = v1;
        index_bytes(a3394, a3395)
    };
    match m3396 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3397 = v3;
            let a3398 = 128u64;
            if nat_lt(a3397, a3398) {
                Ok(Some((v3, 1u64)))
            } else {
                if f118({
                    let a3399 = 194u64;
                    let a3400 = v3;
                    nat_le(a3399, a3400)
                }, {
                    let a3401 = v3;
                    let a3402 = 223u64;
                    nat_le(a3401, a3402)
                }) {
                    let m3405 = f675(v0, {
                        let a3403 = v1;
                        let a3404 = 1u64;
                        nat_add(a3403, a3404)?
                    })?;
                    match m3405 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f676({
                                let a3410 = {
                                    let a3408 = {
                                        let a3406 = v3;
                                        let a3407 = 192u64;
                                        nat_sub(a3406, a3407)
                                    };
                                    let a3409 = 64u64;
                                    nat_mul(a3408, a3409)?
                                };
                                let a3411 = v4;
                                nat_add(a3410, a3411)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f118({
                        let a3412 = 224u64;
                        let a3413 = v3;
                        nat_le(a3412, a3413)
                    }, {
                        let a3414 = v3;
                        let a3415 = 239u64;
                        nat_le(a3414, a3415)
                    }) {
                        let m3418 = f675(v0.clone(), {
                            let a3416 = v1;
                            let a3417 = 1u64;
                            nat_add(a3416, a3417)?
                        })?;
                        match m3418 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3421 = f675(v0, {
                                    let a3419 = v1;
                                    let a3420 = 2u64;
                                    nat_add(a3419, a3420)?
                                })?;
                                match m3421 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f676({
                                            let a3430 = {
                                                let a3424 = {
                                                    let a3422 = v3;
                                                    let a3423 = 224u64;
                                                    nat_sub(a3422, a3423)
                                                };
                                                let a3425 = 4096u64;
                                                nat_mul(a3424, a3425)?
                                            };
                                            let a3431 = {
                                                let a3428 = {
                                                    let a3426 = v5;
                                                    let a3427 = 64u64;
                                                    nat_mul(a3426, a3427)?
                                                };
                                                let a3429 = v6;
                                                nat_add(a3428, a3429)?
                                            };
                                            nat_add(a3430, a3431)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f118({
                            let a3432 = 240u64;
                            let a3433 = v3;
                            nat_le(a3432, a3433)
                        }, {
                            let a3434 = v3;
                            let a3435 = 244u64;
                            nat_le(a3434, a3435)
                        }) {
                            let m3438 = f675(v0.clone(), {
                                let a3436 = v1;
                                let a3437 = 1u64;
                                nat_add(a3436, a3437)?
                            })?;
                            match m3438 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3441 = f675(v0.clone(), {
                                        let a3439 = v1;
                                        let a3440 = 2u64;
                                        nat_add(a3439, a3440)?
                                    })?;
                                    match m3441 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3444 = f675(v0, {
                                                let a3442 = v1;
                                                let a3443 = 3u64;
                                                nat_add(a3442, a3443)?
                                            })?;
                                            match m3444 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f676({
                                                        let a3457 = {
                                                            let a3447 = {
                                                                let a3445 = v3;
                                                                let a3446 = 240u64;
                                                                nat_sub(a3445, a3446)
                                                            };
                                                            let a3448 = 262144u64;
                                                            nat_mul(a3447, a3448)?
                                                        };
                                                        let a3458 = {
                                                            let a3455 = {
                                                                let a3449 = v7;
                                                                let a3450 = 4096u64;
                                                                nat_mul(a3449, a3450)?
                                                            };
                                                            let a3456 = {
                                                                let a3453 = {
                                                                    let a3451 = v8;
                                                                    let a3452 = 64u64;
                                                                    nat_mul(a3451, a3452)?
                                                                };
                                                                let a3454 = v9;
                                                                nat_add(a3453, a3454)?
                                                            };
                                                            nat_add(a3455, a3456)?
                                                        };
                                                        nat_add(a3457, a3458)?
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

pub fn f670(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f208(f118(f208(f494(v2), f495(v2)), f118({
        let a3459 = v2;
        let a3460 = 117u64;
        !nat_eq(a3459, a3460)
    }, {
        let a3461 = v2;
        let a3462 = 85u64;
        !nat_eq(a3461, a3462)
    })), f118(f496(v2), {
        let a3463 = 0u64;
        let a3464 = v1;
        nat_lt(a3463, a3464)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a3465 = v2;
        let a3466 = 65536u64;
        if nat_lt(a3465, a3466) {
            let a3467 = f8(Str::lit("u"));
            let a3468 = f573(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3467, a3468)
        } else {
            let a3469 = f8(Str::lit("U"));
            let a3470 = f573(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3469, a3470)
        }
    }
}

pub fn f671(v0: u64) -> Bytes {
    let a3471 = f8(Str::lit("u"));
    let a3472 = f573(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3471, a3472)
}

pub fn f672() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f673(v0: Option<Adt2>) -> Option<u64> {
    let m3473 = v0;
    match m3473 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m3474 = v1;
            match m3474 {
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

pub fn f674(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3475 = v0;
    if m3475 == 0 {
        Ok(f575(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3475 - 1;
        let m3476 = v1.clone();
        match m3476 {
            Adt2::C0 => {
                Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f619())
                } else {
                    Ok(f575(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_) => {
                Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                let m3477 = f383(f280(v1.clone(), f8(Str::lit("$ref"))));
                match m3477 {
                    None => {
                        if if f588(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f588(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f588(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f576(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f283(v1.clone(), f8(Str::lit("type")));
                            if if f32(v22.clone(), Bytes::lit(&[])) {
                                !f662(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f619())
                            } else {
                                if f32(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f576(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f32(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f576(f663(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f32(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f576(f664(), List::<Adt25>::nil()))
                                        } else {
                                            if f32(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f576(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f32(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3479 = f280(v1, f8(Str::lit("items")));
                                                    match m3479 {
                                                        None => {
                                                            Ok(f617(f8(Str::lit("List")), f619()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f617(f8(Str::lit("List")), f674(v3, v23, {
                                                                let a3480 = v2;
                                                                let a3481 = f8(Str::lit("Item"));
                                                                append_bytes(a3480, a3481)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v22.clone(), f8(Str::lit("object"))) {
                                                        if f583(v1.clone()) {
                                                            Ok(f576(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3482 = f614(v1);
                                                            match m3482 {
                                                                None => {
                                                                    Ok(f617(f8(Str::lit("List")), f665(f619())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f617(f8(Str::lit("List")), f665(f674(v3, v24, {
                                                                        let a3483 = v2;
                                                                        let a3484 = f8(Str::lit("Value"));
                                                                        append_bytes(a3483, a3484)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f575(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3478 = f572(v20)?;
                        match m3478 {
                            None => {
                                Ok(f575(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f576(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C8 => {
                Ok(f619())
            }
            Adt2::C9(_, _) => {
                Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C10(_, _, _) => {
                Ok(f575(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f675(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3487 = {
        let a3485 = v0;
        let a3486 = v1;
        index_bytes(a3485, a3486)
    };
    match m3487 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f118({
                let a3488 = 128u64;
                let a3489 = v3;
                nat_le(a3488, a3489)
            }, {
                let a3490 = v3;
                let a3491 = 191u64;
                nat_le(a3490, a3491)
            }) {
                Ok(Some({
                    let a3492 = v3;
                    let a3493 = 128u64;
                    nat_sub(a3492, a3493)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f676(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f118({
        let a3494 = v1;
        let a3495 = v0;
        nat_le(a3494, a3495)
    }, f118({
        let a3496 = v0;
        let a3497 = v2;
        nat_le(a3496, a3497)
    }, f208({
        let a3498 = v0;
        let a3499 = 55296u64;
        nat_lt(a3498, a3499)
    }, {
        let a3500 = 57343u64;
        let a3501 = v0;
        nat_lt(a3500, a3501)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
