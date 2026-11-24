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
    F118(u8, Bytes),
    F162(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F118(k0, k1) => f118(*k0, k1.clone(), p0),
            Fn0::F162(k0) => f162(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F138,
    F178,
    F192,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F138 => f138(p0, p1),
            Fn1::F178 => f178(p0, p1),
            Fn1::F192 => f192(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F158(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F158(k0) => f158(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F181(Bytes, u64, Bytes, u64),
    F254(Bytes),
    F255(u64, Bytes),
    F311(Bytes),
    F355(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F181(k0, k1, k2, k3) => f181(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F254(k0) => Ok(f254(k0.clone(), p0)),
            Fn3::F255(k0, k1) => f255(*k0, k1.clone(), p0),
            Fn3::F311(k0) => f311(k0.clone(), p0),
            Fn3::F355(k0) => f355(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F208(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<(u64, Adt20)> {
        match self {
            Fn4::F208(k0, k1, k2) => f208(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F240,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F240 => f240(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F243,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F243 => f243(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F245,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F245 => f245(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F248,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F248 => f248(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F271(Bytes),
    F319(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F271(k0) => f271(k0.clone(), p0),
            Fn9::F319(k0, k1) => Ok(f319(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F273,
    F387,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F273 => f273(p0, p1),
            Fn10::F387 => f387(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F276,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F276 => f276(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F278,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F278 => f278(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F281,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F281 => f281(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F283,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn14::F283 => f283(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F288(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F288(k0) => f288(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F314(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F314(k0) => f314(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F323,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F323 => f323(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F325,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F325 => f325(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F327,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F327 => f327(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F335,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F335 => f335(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F347,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F347 => f347(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F369,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F369 => f369(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F371,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F371 => f371(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F373(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F373(k0) => f373(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F386,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F386 => f386(p0),
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
                                                    if f27(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                        f36(v1, v4)
                                                    } else {
                                                        if f27(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                            f37(v1, v4)
                                                        } else {
                                                            if f27(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                f38(v1, v4)
                                                            } else {
                                                                if f27(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                    f39(v1, v4)
                                                                } else {
                                                                    if f27(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                        f40(v1, v4)
                                                                    } else {
                                                                        if f27(v2, f8(Str::lit("propose-encoding"))) {
                                                                            let m130 = v5;
                                                                            match m130.uncons() {
                                                                                None => {
                                                                                    Ok(Adt11::C4(f26()))
                                                                                }
                                                                                Some((v6, v7)) => {
                                                                                    f41(v1, v4, v6, v7)
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
                Ok(List::cons(f42(v4)?, f21(v0, {
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
    let v1: List<Bytes> = f43(v0.clone(), 47u8)?;
    let v2: Bytes = f44(v1);
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
        Ok(f45(v0.clone(), 0u64, {
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
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f27(v0: Bytes, v1: Bytes) -> bool {
    let a159 = v0;
    let a160 = v1;
    equal(a159, a160)
}

pub fn f28(v0: Adt1) -> R<Adt11> {
    let m161 = f46(v0.clone(), f47());
    match m161.uncons() {
        None => {
            let m162 = f48(v0.clone(), f47())?;
            match m162.uncons() {
                None => {
                    let m165 = f49(match v0.clone() {
                        Adt1::C0(_, h163, _) => {
                            h163
                        }
                    }, f50(f51(), f52(match v0.clone() {
                        Adt1::C0(_, h164, _) => {
                            h164
                        }
                    }, false)));
                    match m165.uncons() {
                        None => {
                            let m167 = f46(v0.clone(), f52(match v0.clone() {
                                Adt1::C0(_, h166, _) => {
                                    h166
                                }
                            }, true));
                            match m167.uncons() {
                                None => {
                                    let m169 = f53(v0.clone(), f52(match v0.clone() {
                                        Adt1::C0(_, h168, _) => {
                                            h168
                                        }
                                    }, true))?;
                                    match m169 {
                                        None => {
                                            let v10: (bool, Bytes) = f54(Adt13::C0(Adt12::C0(f55(f56(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f57(v0.clone())?, f58(f59(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f55(f56(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f58(f59(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f59(v0.clone(), f8(Str::lit("model/admissions.json")))?, f59(v0.clone(), f8(Str::lit("model/strata.json")))?, f60(f59(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f52(match v0 {
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
    if f61(f62(match v0.clone() {
        Adt1::C0(_, h174, _) => {
            h174
        }
    }, f63())) {
        Ok(Adt11::C1(List::cons(f63(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f64(f65(f63(), f66(match v0.clone() {
            Adt1::C0(_, h175, _) => {
                h175
            }
        }, f63())))?;
        let m176 = f46(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m176.uncons() {
            None => {
                let m177 = f67(v0.clone(), f68(v2, v1.clone()))?;
                match m177 {
                    None => {
                        f69(v0, v1)
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
    if f61(f70(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h178) => {
            h178
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f71(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f61(f70(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h179) => {
                h179
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f71(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f31(v0: Adt1) -> R<Adt11> {
    let m180 = f46(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f72(), List::<Bytes>::nil()))));
    match m180.uncons() {
        None => {
            Ok(Adt11::C3(f73(f74(v0.clone())?, List::cons((f75(), f76()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f77()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f78()), List::cons((f8(Str::lit("docs/model.md")), f79(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f80()), List::<(Bytes, Bytes)>::nil())))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md and docs/owner-decisions.md\u{a}"))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f32(v0: Adt1) -> R<Adt11> {
    let m181 = f46(v0.clone(), List::cons(f81(), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m181.uncons() {
        None => {
            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f82(f83(f84(f85(f56(v0.clone(), f81()))?)), f86(f59(v0, f8(Str::lit("model/admissions.json")))?)))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f33(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m182 = f46(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m182.uncons() {
        None => {
            let v4: Bytes = f56(v0.clone(), v1);
            let v5: List<List<Bytes>> = f85(v4.clone())?;
            if f61(f87(f88(v5.clone()), f89())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f90(Str::lit("id"), v6.clone())?, List::cons(f90(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f90(Str::lit("artifact"), f81())?, List::cons(f90(Str::lit("checksum"), f91(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f81(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f92(f56(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f93(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f94(f83(f84(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f34(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m183 = f46(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m183.uncons() {
        None => {
            let m184 = f95(f96(f56(v0, v1))?)?;
            match m184 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f94(f97(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f35(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m185 = f46(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil())))));
    match m185.uncons() {
        None => {
            let v4: List<Bytes> = f96(f56(v0.clone(), v1))?;
            let m186 = f98(v4.clone(), f8(Str::lit("requirement")))?;
            match m186 {
                None => {
                    Ok(Adt11::C4(f99()))
                }
                Some(v5) => {
                    let m187 = f98(v4, f8(Str::lit("admission")))?;
                    match m187 {
                        None => {
                            Ok(Adt11::C4(f99()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f59(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f59(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f50(f58(v5.clone(), f8(Str::lit("id"))), f58(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f64(f50(f58(v7.clone(), f8(Str::lit("id"))), f58(v8.clone(), f8(Str::lit("construct")))))?;
                            let a189 = {
                                let a188 = v9.clone();
                                length_list(a188)
                            };
                            let a190 = 0u64;
                            if nat_eq(a189, a190) {
                                Ok(Adt11::C4(f99()))
                            } else {
                                if f61({
                                    let a193 = {
                                        let a191 = f64(f50(v9.clone(), v10.clone()))?;
                                        length_list(a191)
                                    };
                                    let a194 = {
                                        let a192 = f50(v9.clone(), v10);
                                        length_list(a192)
                                    };
                                    nat_eq(a193, a194)
                                }) {
                                    Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                } else {
                                    Ok(Adt11::C3(List::cons(f100(f8(Str::lit("model/requirements.json")), f101(v7, v5))?, List::cons(f100(f8(Str::lit("model/admissions.json")), f101(v8, v6))?, List::cons(f93(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f94(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m195 = f46(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m195.uncons() {
        None => {
            let m196 = f102(f96(f56(v0, v1))?)?;
            match m196 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f94(f97(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f37(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m197 = f46(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m197.uncons() {
        None => {
            let m198 = f103(v0.clone(), v1.clone())?;
            match m198 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m199 = f104(v4.clone(), f8(Str::lit("schema")));
                    match m199 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f105(v0, f106(v5.clone())?, v5, f107(v4.clone(), f8(Str::lit("name"))), f60(f108(f104(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f38(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m200 = f46(v0.clone(), List::cons(v1.clone(), List::cons(f72(), List::<Bytes>::nil())));
    match m200.uncons() {
        None => {
            let m201 = f109(f96(f56(v0.clone(), v1))?)?;
            match m201 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f59(v0, f72())?;
                    let v6: List<Bytes> = f58(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f64(f58(v5.clone(), f8(Str::lit("at"))))?;
                    let a203 = {
                        let a202 = v6.clone();
                        length_list(a202)
                    };
                    let a204 = 0u64;
                    if if nat_eq(a203, a204) {
                        true
                    } else {
                        f61({
                            let a207 = {
                                let a205 = f64(f50(v6.clone(), v7.clone()))?;
                                length_list(a205)
                            };
                            let a208 = {
                                let a206 = f50(v6.clone(), v7);
                                length_list(a206)
                            };
                            nat_eq(a207, a208)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f100(f72(), f101(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f94(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m209 = f46(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m209.uncons() {
        None => {
            let m210 = f103(v0, v1)?;
            match m210 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f110(f108(Some(v4))) {
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

pub fn f40(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m211 = f46(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m211.uncons() {
        None => {
            let m212 = f103(v0.clone(), v1.clone())?;
            match m212 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f61(f27(f107(v4.clone(), f8(Str::lit("schema"))), f111())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f107(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m213 = f104(v4.clone(), f8(Str::lit("source")));
                            match m213 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f112(f104(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f113(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f113(v9.clone(), f8(Str::lit("binding.json"))), f56(v0.clone(), v1));
                        let m214 = v8;
                        match m214 {
                            None => {
                                f114(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f61(f27(f107(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f114(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f113(v9, f115(f107(v7.clone(), f8(Str::lit("url"))))?);
                                    if f61(f70(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h215) => {
                                            h215
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f107(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m216 = f46(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m216.uncons() {
                                            None => {
                                                if f27(f91(f56(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f114(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f41(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m217 = f46(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m217.uncons() {
        None => {
            let m218 = f103(v0.clone(), v1.clone())?;
            match m218 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f105(v0.clone(), f56(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f42(v0: u8) -> R<Str> {
    let m267 = {
        let a265 = f116();
        let a266 = {
            let v1: u8 = v0;
            let a263 = {
                let a239 = {
                    let a227 = {
                        let a221 = {
                            let a219 = v1;
                            let a220 = 128u8;
                            fixed_u8::bit_and(a219, a220)
                        };
                        let a222 = 128u8;
                        if equal(a221, a222) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a228 = {
                        let a225 = {
                            let a223 = v1;
                            let a224 = 64u8;
                            fixed_u8::bit_and(a223, a224)
                        };
                        let a226 = 64u8;
                        if equal(a225, a226) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a227, a228)?
                };
                let a240 = {
                    let a237 = {
                        let a231 = {
                            let a229 = v1;
                            let a230 = 32u8;
                            fixed_u8::bit_and(a229, a230)
                        };
                        let a232 = 32u8;
                        if equal(a231, a232) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a238 = {
                        let a235 = {
                            let a233 = v1;
                            let a234 = 16u8;
                            fixed_u8::bit_and(a233, a234)
                        };
                        let a236 = 16u8;
                        if equal(a235, a236) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a237, a238)?
                };
                nat_add(a239, a240)?
            };
            let a264 = {
                let a261 = {
                    let a249 = {
                        let a243 = {
                            let a241 = v1;
                            let a242 = 8u8;
                            fixed_u8::bit_and(a241, a242)
                        };
                        let a244 = 8u8;
                        if equal(a243, a244) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a250 = {
                        let a247 = {
                            let a245 = v1;
                            let a246 = 4u8;
                            fixed_u8::bit_and(a245, a246)
                        };
                        let a248 = 4u8;
                        if equal(a247, a248) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a249, a250)?
                };
                let a262 = {
                    let a259 = {
                        let a253 = {
                            let a251 = v1;
                            let a252 = 2u8;
                            fixed_u8::bit_and(a251, a252)
                        };
                        let a254 = 2u8;
                        if equal(a253, a254) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a260 = {
                        let a257 = {
                            let a255 = v1;
                            let a256 = 1u8;
                            fixed_u8::bit_and(a255, a256)
                        };
                        let a258 = 1u8;
                        if equal(a257, a258) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a259, a260)?
                };
                nat_add(a261, a262)?
            };
            nat_add(a263, a264)?
        };
        index_list(a265, a266)
    };
    match m267 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f43(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a268 = v0.clone();
        length_bytes(a268)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h269, _) = f117(Fn0::F118(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h269
    };
    let v4: u64 = {
        let (h270, _) = {
            let (_, h271) = v3.clone();
            h271
        };
        h270
    };
    Ok(f119(List::cons(f45(v0, v4, {
        let a272 = v2;
        let a273 = v4;
        nat_sub(a272, a273)
    }), {
        let (_, h274) = {
            let (_, h275) = v3;
            h275
        };
        h274
    })))
}

pub fn f44(p279: List<Bytes>) -> Bytes {
    let m278 = p279;
    {
        let mut m278 = m278;
        loop {
            let step = {
                let v0 = m278;
                let m276 = v0;
                match m276.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m277 = v2.clone();
                        match m277.uncons() {
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
                Ok(next) => { m278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m283 = {
        let a280 = v0;
        let a281 = v1;
        let a282 = v2;
        slice_bytes(a280, a281, a282)
    };
    match m283 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f46(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m284 = v1;
    match m284.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m286 = f120(match v0.clone() {
                Adt1::C0(h285, _, _) => {
                    h285
                }
            }, v2.clone());
            match m286 {
                None => {
                    List::cons(v2, f46(v0, v3))
                }
                Some(_) => {
                    f46(v0, v3)
                }
            }
        }
    }
}

pub fn f47() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f81(), List::<Bytes>::nil())))))))
}

pub fn f48(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m287 = v1;
    match m287.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f121(v0.clone(), v2.clone()) {
                f122(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m288 = f103(v0.clone(), v2.clone())?;
                match m288 {
                    None => {
                        Ok(List::cons(v2, f48(v0, v3)?))
                    }
                    Some(_) => {
                        f48(v0, v3)
                    }
                }
            } else {
                f48(v0, v3)
            }
        }
    }
}

pub fn f49(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m289 = v1;
    match m289.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f62(v0.clone(), v2.clone()) {
                f49(v0, v3)
            } else {
                List::cons(v2, f49(v0, v3))
            }
        }
    }
}

pub fn f50(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m290 = v0;
    match m290.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f50(v3, v1))
        }
    }
}

pub fn f51() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f52(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m291 = v0;
    match m291.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f50(f123({
                let (h292, _) = v2;
                h292
            }, {
                let (_, h293) = v2;
                h293
            }, v1), f52(v3, v1))
        }
    }
}

pub fn f53(v0: Adt1, p297: List<Bytes>) -> R<Option<Bytes>> {
    let m296 = p297;
    {
        let mut m296 = m296;
        loop {
            let step = {
                let v1 = m296;
                let m294 = v1;
                match m294.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m295 = f124(f56(v0.clone(), v2.clone()), f125())?;
                        match m295 {
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
                Ok(next) => { m296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h298, _, _, _, _) => {
            h298
        }
    };
    let v2: Adt15 = f126(v1.clone())?;
    let v3: List<Bytes> = f127(v1.clone());
    let v4: List<Bytes> = f58(match v0.clone() {
        Adt13::C0(_, _, h299, _, _) => {
            h299
        }
    }, f8(Str::lit("path")));
    let m301 = f128(v2, List::<u64>::nil(), f129(v1.clone(), match v0.clone() {
        Adt13::C0(_, h300, _, _, _) => {
            h300
        }
    }, 0u64)?);
    match m301 {
        None => {
            if f131(f132(match v0.clone() {
                Adt13::C0(_, _, _, _, h303) => {
                    h303
                }
            }, f50(match v0.clone() {
                Adt13::C0(_, _, _, _, h304) => {
                    h304
                }
            }, v4.clone()))?, f132(v4.clone(), f50(match v0.clone() {
                Adt13::C0(_, _, _, _, h305) => {
                    h305
                }
            }, v4))?, f133(match v0.clone() {
                Adt13::C0(_, _, h306, _, _) => {
                    h306
                }
            })) {
                let m310 = f134(f132(match v0.clone() {
                    Adt13::C0(_, _, _, h307, _) => {
                        h307
                    }
                }, f50(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h308, _) => {
                        h308
                    }
                }))?, f132(v3.clone(), f50(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h309, _) => {
                        h309
                    }
                }))?);
                match m310 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f94(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f130(f50(v3, match v0 {
                            Adt13::C0(_, _, _, h311, _) => {
                                h311
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f130(match v1 {
                Adt12::C0(_, _, _, _, h302) => {
                    h302
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f55(v0: Bytes) -> R<List<Bytes>> {
    f135(f96(v0)?, f8(Str::lit("id = \"")))
}

pub fn f56(v0: Adt1, v1: Bytes) -> Bytes {
    let m313 = f120(match v0 {
        Adt1::C0(h312, _, _) => {
            h312
        }
    }, v1);
    match m313 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m314 = v2;
            match m314 {
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

pub fn f57(v0: Adt1) -> R<List<Bytes>> {
    let m315 = f85(f56(v0, f81()))?;
    match m315.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f83(v2))
        }
    }
}

pub fn f58(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m316 = v0;
    match m316.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f107(v2, v1.clone()), f58(v3, v1))
        }
    }
}

pub fn f59(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f108(f103(v0, v1)?))
}

pub fn f60(v0: List<Adt14>) -> List<Bytes> {
    let m317 = v0;
    match m317.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m318 = f112(Some(v1));
            match m318 {
                None => {
                    f60(v2)
                }
                Some(v3) => {
                    List::cons(v3, f60(v2))
                }
            }
        }
    }
}

pub fn f61(v0: bool) -> bool {
    !v0
}

pub fn f62(p322: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m321 = p322;
    {
        let mut m321 = m321;
        loop {
            let step = {
                let v0 = m321;
                let m319 = v0;
                match m319.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f27({
                            let (h320, _) = v2;
                            h320
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f63() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f64(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f136(f137(Fn1::F138, List::<Bytes>::nil(), f139(v0)?)))
}

pub fn f65(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m323 = v1;
    match m323.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f122(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f113(v0.clone(), v2), f65(v0, v3))
            } else {
                f65(v0, v3)
            }
        }
    }
}

pub fn f66(p328: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m327 = p328;
    {
        let mut m327 = m327;
        loop {
            let step = {
                let v0 = m327;
                let m324 = v0;
                match m324.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f27({
                            let (h325, _) = v2;
                            h325
                        }, v1.clone()) {
                            let (_, h326) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h326)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt1, p331: List<Bytes>) -> R<Option<Bytes>> {
    let m330 = p331;
    {
        let mut m330 = m330;
        loop {
            let step = {
                let v1 = m330;
                let m329 = v1;
                match m329.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        if f140(v0.clone(), v2.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v2)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m332 = v0;
    match m332.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f70(v2.clone(), v1.clone()) {
                f68(v3, v1)
            } else {
                List::cons(v2, f68(v3, v1))
            }
        }
    }
}

pub fn f69(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f61(f62(match v0.clone() {
        Adt1::C0(_, h333, _) => {
            h333
        }
    }, f141())) {
        Ok(Adt11::C1(List::cons(f141(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f68(f64(f65(f141(), f66(match v0.clone() {
            Adt1::C0(_, h334, _) => {
                h334
            }
        }, f141())))?, v1);
        let m335 = f46(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m335.uncons() {
            None => {
                let m336 = f142(v0.clone(), v2, f58(f59(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m336 {
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

pub fn f70(v0: Bytes, p339: List<Bytes>) -> bool {
    let m338 = p339;
    {
        let mut m338 = m338;
        loop {
            let step = {
                let v1 = m338;
                let m337 = v1;
                match m337.uncons() {
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
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f72() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f73(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m340 = v0;
    match m340.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f73(v3, v1))
        }
    }
}

pub fn f74(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m341 = f59(v0.clone(), f72())?;
    match m341.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f143(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f75() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f76() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f77() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/encodings/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          { echo \"proposed a change to the API's sources, exported again as $(basename studio/.lexlean/production/*/)\"; git diff --stat -- studio/src README.md; } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          gh pr create --head \"$branch\" --base main --title \"verdant_studio $ARGS\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f78() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f79(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f144(f59(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f145(f59(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f80() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f81() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f82(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m342 = v0;
    match m342.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f70(v2.clone(), v1.clone()) {
                f82(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f82(v3, v1))
            }
        }
    }
}

pub fn f83(v0: List<List<Bytes>>) -> List<Bytes> {
    let m343 = v0;
    match m343.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m344 = v1;
            match m344.uncons() {
                None => {
                    f83(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f83(v2))
                }
            }
        }
    }
}

pub fn f84(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m345 = v0;
    match m345.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f85(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f146(v0.clone(), 0u64, {
        let a346 = v0;
        length_bytes(a346)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a349 = {
            let a348 = match v1.clone() {
                Adt16::C0(_, _, h347, _, _) => {
                    h347
                }
            };
            length_bytes(a348)
        };
        let a350 = 0u64;
        if if nat_eq(a349, a350) {
            let m352 = match v1.clone() {
                Adt16::C0(_, _, _, h351, _) => {
                    h351
                }
            };
            match m352.uncons() {
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
            f147(v1)
        }
    };
    Ok(f148(match v4 {
        Adt16::C0(_, _, _, _, h353) => {
            h353
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f86(v0: List<Adt14>) -> List<Bytes> {
    let m354 = v0;
    match m354.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m355 = f104(v1, f8(Str::lit("std")));
            match m355 {
                None => {
                    f86(v2)
                }
                Some(v3) => {
                    List::cons(f107(v3, f8(Str::lit("row"))), f86(v2))
                }
            }
        }
    }
}

pub fn f87(p360: List<Bytes>, p361: List<Bytes>) -> bool {
    let m359 = (p360, p361);
    {
        let mut m359 = m359;
        loop {
            let step = {
                let (v0, v1) = m359;
                let m356 = v0;
                match m356.uncons() {
                    None => {
                        let m357 = v1;
                        match m357.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m358 = v1;
                        match m358.uncons() {
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
                Ok(next) => { m359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: List<List<Bytes>>) -> List<Bytes> {
    let m362 = v0;
    match m362.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f89() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f90(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f149(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f91(v0: Bytes) -> R<Bytes> {
    Ok(f150(f151(v0)?))
}

pub fn f92(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f152(f153(f96(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f154(f155(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f93(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f106(Adt14::C3(std::rc::Rc::new(f156(f64(f50(v1, f60(f59(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f94(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m363 = f157(Fn2::F158(v1), None::<Bytes>, v0);
    match m363 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f95(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m364 = v0;
    match m364.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f159(v1)?;
            let a366 = {
                let a365 = v3.clone();
                length_bytes(a365)
            };
            let a367 = 0u64;
            if nat_eq(a366, a367) {
                f95(v2)
            } else {
                let m368 = f160(v3.clone())?;
                match m368 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f107(v4.clone(), f8(Str::lit("id")));
                        if if f27(f107(v4, f8(Str::lit("schema"))), f111()) {
                            f61({
                                let a370 = {
                                    let a369 = v5.clone();
                                    length_bytes(a369)
                                };
                                let a371 = 0u64;
                                nat_eq(a370, a371)
                            })
                        } else {
                            false
                        } {
                            let m372 = f95(v2)?;
                            match m372 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f161(v5), {
                                        let a373 = v3;
                                        let a374 = Bytes::lit(&[0x0a]);
                                        append_bytes(a373, a374)
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

pub fn f96(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a375 = v0.clone();
        length_bytes(a375)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h376, _) = f117(Fn0::F162(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h376
    };
    let v3: u64 = {
        let (h377, _) = {
            let (_, h378) = v2.clone();
            h378
        };
        h377
    };
    let v4: List<Bytes> = {
        let (_, h379) = {
            let (_, h380) = v2;
            h380
        };
        h379
    };
    let a381 = v3;
    let a382 = v1;
    if nat_lt(a381, a382) {
        Ok(f119(List::cons(f45(v0, v3, {
            let a383 = v1;
            let a384 = v3;
            nat_sub(a383, a384)
        }), v4)))
    } else {
        Ok(f119(v4))
    }
}

pub fn f97(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m385 = v0;
    match m385.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h386, _) = v1;
                h386
            }, f97(v2))
        }
    }
}

pub fn f98(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m387 = v0;
    match m387.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f159(v2)?;
            let a389 = {
                let a388 = v4.clone();
                length_bytes(a388)
            };
            let a390 = 0u64;
            if nat_eq(a389, a390) {
                f98(v3, v1)
            } else {
                let m391 = f160(v4)?;
                match m391 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f163(v5.clone());
                        let a393 = {
                            let a392 = v6.clone();
                            length_bytes(a392)
                        };
                        let a394 = 0u64;
                        if nat_eq(a393, a394) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m395 = f98(v3, v1.clone())?;
                            match m395 {
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

pub fn f99() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f100(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f106(Adt14::C3(std::rc::Rc::new(f164(v1))))?))
}

pub fn f101(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m396 = v0;
    match m396.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f101(v3, v1))
        }
    }
}

pub fn f102(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m397 = v0;
    match m397.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f159(v1)?;
            let a399 = {
                let a398 = v3.clone();
                length_bytes(a398)
            };
            let a400 = 0u64;
            if nat_eq(a399, a400) {
                f102(v2)
            } else {
                let m401 = f160(v3.clone())?;
                match m401 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f165(v4.clone()) {
                            let m402 = f102(v2)?;
                            match m402 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f141(), List::cons(f8(Str::lit("/")), List::cons(f166(f107(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a403 = v3;
                                        let a404 = Bytes::lit(&[0x0a]);
                                        append_bytes(a403, a404)
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

pub fn f103(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f160(f56(v0, v1))
}

pub fn f104(p410: Adt14, v1: Bytes) -> Option<Adt14> {
    let m409 = p410;
    {
        let mut m409 = m409;
        loop {
            let step = {
                let v0 = m409;
                let m405 = v0;
                match m405 {
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
                    Adt14::C4(r406) => {
                        let v5 = (*r406).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r407, r408) => {
                        let v3 = (*r407).clone();
                        let v4 = (*r408).clone();
                        if f27(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m411 = f46(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m411.uncons() {
        None => {
            let m412 = f167(v4);
            match m412 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a413 = f8(Str::lit("Catalog."));
                        let a414 = f166(v3.clone())?;
                        append_bytes(a413, a414)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f166(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m415 = f168(v2, v9.clone(), f166(v3.clone())?)?;
                    match m415 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f166(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f106(Adt14::C3(std::rc::Rc::new(f164(f169(f59(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f106(Adt14::C3(std::rc::Rc::new(f164(f169(f59(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a419 = f170(f56(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f45(v10.clone(), 4u64, {
                                    let a417 = {
                                        let a416 = v10;
                                        length_bytes(a416)
                                    };
                                    let a418 = 4u64;
                                    nat_sub(a417, a418)
                                }))?;
                                let a420 = Bytes::lit(&[0x0a]);
                                append_bytes(a419, a420)
                            }), List::cons(f93(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f106(v0: Adt14) -> R<Bytes> {
    let a421 = f171(v0)?;
    let a422 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a421, a422))
}

pub fn f107(v0: Adt14, v1: Bytes) -> Bytes {
    let m423 = f112(f104(v0, v1));
    match m423 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f108(v0: Option<Adt14>) -> List<Adt14> {
    let m424 = v0;
    match m424 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f172(v1)
        }
    }
}

pub fn f109(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m425 = v0;
    match m425.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f159(v1)?;
            let a427 = {
                let a426 = v3.clone();
                length_bytes(a426)
            };
            let a428 = 0u64;
            if nat_eq(a427, a428) {
                f109(v2)
            } else {
                let m429 = f160(v3)?;
                match m429 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f173(v4.clone(), f8(Str::lit("agent"))) {
                            if f173(v4.clone(), f8(Str::lit("at"))) {
                                if f173(v4.clone(), f8(Str::lit("quote"))) {
                                    f174(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m430 = f109(v2)?;
                            match m430 {
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

pub fn f110(p433: List<Adt14>) -> bool {
    let m432 = p433;
    {
        let mut m432 = m432;
        loop {
            let step = {
                let v0 = m432;
                let m431 = v0;
                match m431.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f27(f107(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f70(f8(Str::lit("studio")), f58(f175(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f112(v0: Option<Adt14>) -> Option<Bytes> {
    let m434 = v0;
    match m434 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m435 = v1;
            match m435 {
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

pub fn f113(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f114(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f92(f56(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f176(v1, v4)?)?), List::cons(f93(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f115(v0: Bytes) -> R<Bytes> {
    Ok(f44(f43(v0, 47u8)?))
}

pub fn f116() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f117(v0: Fn0, p440: u64, p441: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m439 = (p440, p441);
    {
        let mut m439 = m439;
        loop {
            let step = {
                let (v1, v2) = m439;
                let m436 = v1;
                if m436 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m436 - 1;
                    let m438 = {
                        let c437: Fn0 = v0.clone();
                        c437.apply(v2.clone())?
                    };
                    match m438 {
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
                Ok(next) => { m439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f177(v1, v0, v2)
}

pub fn f119(v0: List<Bytes>) -> List<Bytes> {
    f137(Fn1::F178, List::<Bytes>::nil(), v0)
}

pub fn f120(p446: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m445 = p446;
    {
        let mut m445 = m445;
        loop {
            let step = {
                let v0 = m445;
                let m442 = v0;
                match m442.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f27({
                            let (h443, _) = v2;
                            h443
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h444) = v2;
                                h444
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m445 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt1, v1: Bytes) -> bool {
    let m448 = f120(match v0 {
        Adt1::C0(h447, _, _) => {
            h447
        }
    }, v1);
    match m448 {
        None => {
            false
        }
        Some(v2) => {
            let m449 = v2;
            match m449 {
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

pub fn f122(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a450 = v0.clone();
        length_bytes(a450)
    };
    let v3: u64 = {
        let a451 = v1.clone();
        length_bytes(a451)
    };
    let a452 = v3;
    let a453 = v2;
    if nat_le(a452, a453) {
        let m459 = {
            let a456 = v0;
            let a457 = {
                let a454 = v2;
                let a455 = v3;
                nat_sub(a454, a455)
            };
            let a458 = v3;
            slice_bytes(a456, a457, a458)
        };
        match m459 {
            None => {
                false
            }
            Some(v4) => {
                let a460 = v4;
                let a461 = v1;
                equal(a460, a461)
            }
        }
    } else {
        false
    }
}

pub fn f123(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m462 = v1;
    match m462.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f179(v3.clone())
            } else {
                f61(f179(v3.clone()))
            } {
                List::cons(f113(v0.clone(), v3), f123(v0, v4, v2))
            } else {
                f123(v0, v4, v2)
            }
        }
    }
}

pub fn f124(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a463 = v0.clone();
        length_bytes(a463)
    };
    let v3: u64 = {
        let a464 = v1.clone();
        length_bytes(a464)
    };
    let v4: u64 = {
        let (h465, _) = f180(Fn3::F181(v1.clone(), v3, v0.clone(), v2), {
            let a466 = v2;
            let a467 = 1u64;
            nat_add(a466, a467)?
        }, 0u64)?;
        h465
    };
    if f182(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f125() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f126(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f183(match v0.clone() {
        Adt12::C0(h468, _, _, _, _) => {
            h468
        }
    }, 0u64)?, f183(match v0.clone() {
        Adt12::C0(_, h469, _, _, _) => {
            h469
        }
    }, 0u64)?, f183(match v0.clone() {
        Adt12::C0(_, _, h470, _, _) => {
            h470
        }
    }, 0u64)?, f183(match v0 {
        Adt12::C0(_, _, _, h471, _) => {
            h471
        }
    }, 0u64)?))
}

pub fn f127(v0: Adt12) -> List<Bytes> {
    f50(match v0.clone() {
        Adt12::C0(h472, _, _, _, _) => {
            h472
        }
    }, f50(match v0.clone() {
        Adt12::C0(_, _, h473, _, _) => {
            h473
        }
    }, f50(match v0.clone() {
        Adt12::C0(_, _, _, h474, _) => {
            h474
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h475) => {
            h475
        }
    })))
}

pub fn f128(v0: Adt15, p481: List<u64>, p482: List<Adt17>) -> Option<u64> {
    let m480 = (p481, p482);
    {
        let mut m480 = m480;
        loop {
            let step = {
                let (v1, v2) = m480;
                let m476 = v2;
                match m476.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt17>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f184(v0.clone(), v1.clone(), match v3.clone() {
                            Adt17::C0(_, h477) => {
                                h477
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt17>), Option<u64>>((List::cons(match v3 {
                                Adt17::C0(h478, _) => {
                                    h478
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt17>), Option<u64>>(Some(match v3 {
                                Adt17::C0(h479, _) => {
                                    h479
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt17>> {
    let m483 = v1;
    match m483.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt17::C0(v2, f185(v0.clone(), v3)?), f129(v0, v4, {
                let a484 = v2;
                let a485 = 1u64;
                nat_add(a484, a485)?
            })?))
        }
    }
}

pub fn f130(p489: List<Bytes>, p490: u64) -> Bytes {
    let m488 = (p489, p490);
    {
        let mut m488 = m488;
        loop {
            let step = {
                let (v0, v1) = m488;
                let m486 = v0;
                match m486.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m487 = v1;
                        if m487 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m487 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f186(v0.clone(), v1.clone()) {
        if f186(v1, v0) {
            f187(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f132(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m491 = v0;
    match m491.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f188(v2, v1.clone())?, f132(v3, v1)?))
        }
    }
}

pub fn f133(v0: List<Adt14>) -> List<u64> {
    let m492 = v0;
    match m492.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f189(v1), f133(v2))
        }
    }
}

pub fn f134(p495: List<u64>, v1: List<u64>) -> Option<u64> {
    let m494 = p495;
    {
        let mut m494 = m494;
        loop {
            let step = {
                let v0 = m494;
                let m493 = v0;
                match m493.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f190(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m496 = v0;
    match m496.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m497 = f191(v2, v1.clone())?;
            match m497 {
                None => {
                    f135(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f135(v3, v1)?))
                }
            }
        }
    }
}

pub fn f136(v0: List<Bytes>) -> List<Bytes> {
    f137(Fn1::F192, List::<Bytes>::nil(), v0)
}

pub fn f137(v0: Fn1, p501: List<Bytes>, p502: List<Bytes>) -> List<Bytes> {
    let m500 = (p501, p502);
    {
        let mut m500 = m500;
        loop {
            let step = {
                let (v1, v2) = m500;
                let m498 = v2;
                match m498.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c499: Fn1 = v0.clone();
                            c499.apply(v1, v3)
                        }, v4))
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

pub fn f138(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f193(v0, v1)
}

pub fn f139(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f194(f195(f196(v0))?))
}

pub fn f140(v0: Adt1, v1: Bytes) -> R<bool> {
    let m503 = f103(v0.clone(), v1)?;
    match m503 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let v3: List<Bytes> = f197(f153(f96(f56(v0, f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f107(v2.clone(), f8(Str::lit("id"))))?;
            let m504 = v3.clone();
            match m504.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m506 = f112(f104({
                        let m505 = f104(v2.clone(), f8(Str::lit("source")));
                        match m505 {
                            None => {
                                Adt14::C5
                            }
                            Some(v6) => {
                                v6
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m506 {
                        None => {
                            Ok(false)
                        }
                        Some(v7) => {
                            Ok(f61(f70(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f149(f198(v2, v7))?, List::<Bytes>::nil()))), v3)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f141() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f142(v0: Adt1, p510: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m509 = p510;
    {
        let mut m509 = m509;
        loop {
            let step = {
                let v1 = m509;
                let m507 = v1;
                match m507.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f70(f199(f107({
                            let m508 = f103(v0.clone(), v3.clone())?;
                            match m508 {
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
                Ok(next) => { m509 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f200(f59(v0, f72())?)?)))
}

pub fn f144(v0: List<Adt14>) -> List<Bytes> {
    let m511 = v0;
    match m511.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f107(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f107(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f107(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f144(v2))
        }
    }
}

pub fn f145(v0: List<Adt14>) -> List<Bytes> {
    let m512 = v0;
    match m512.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f107(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f201(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f145(v2))
        }
    }
}

pub fn f146(v0: Bytes, p570: u64, p571: u64, p572: Adt16) -> R<Adt16> {
    let m569 = (p570, (p571, p572));
    {
        let mut m569 = m569;
        loop {
            let step = {
                let (v1, (v2, v3)) = m569;
                let m513 = v2;
                if m513 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m513 - 1;
                    let v5: u64 = f202(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h514, _, _, _) => {
                            h514
                        }
                    } {
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
                        if match v3.clone() {
                            Adt16::C0(h520, _, _, _, _) => {
                                h520
                            }
                        } {
                            let a521 = v5;
                            let a522 = 34u64;
                            if nat_eq(a521, a522) {
                                let a525 = f202(v0.clone(), {
                                    let a523 = v1;
                                    let a524 = 1u64;
                                    nat_add(a523, a524)?
                                })?;
                                let a526 = 34u64;
                                if nat_eq(a525, a526) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a527 = v1;
                                        let a528 = 1u64;
                                        nat_add(a527, a528)?
                                    }, (v4, Adt16::C0(true, true, f203(match v3.clone() {
                                        Adt16::C0(_, _, h529, _, _) => {
                                            h529
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h530, _) => {
                                            h530
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h531) => {
                                            h531
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a532 = v1;
                                        let a533 = 1u64;
                                        nat_add(a532, a533)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h534, _, _) => {
                                            h534
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h535, _) => {
                                            h535
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h536) => {
                                            h536
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a537 = v1;
                                    let a538 = 1u64;
                                    nat_add(a537, a538)?
                                }, (v4, Adt16::C0(true, false, f203(match v3.clone() {
                                    Adt16::C0(_, _, h539, _, _) => {
                                        h539
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h540, _) => {
                                        h540
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h541) => {
                                        h541
                                    }
                                }))))
                            }
                        } else {
                            let a542 = v5;
                            let a543 = 34u64;
                            if nat_eq(a542, a543) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a544 = v1;
                                    let a545 = 1u64;
                                    nat_add(a544, a545)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h546, _, _) => {
                                        h546
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h547, _) => {
                                        h547
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h548) => {
                                        h548
                                    }
                                }))))
                            } else {
                                let a549 = v5;
                                let a550 = 44u64;
                                if nat_eq(a549, a550) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a551 = v1;
                                        let a552 = 1u64;
                                        nat_add(a551, a552)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h553, _, _) => {
                                            h553
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h554, _) => {
                                            h554
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h555) => {
                                            h555
                                        }
                                    }))))
                                } else {
                                    let a556 = v5;
                                    let a557 = 13u64;
                                    if nat_eq(a556, a557) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a558 = v1;
                                            let a559 = 1u64;
                                            nat_add(a558, a559)?
                                        }, (v4, v3)))
                                    } else {
                                        let a560 = v5;
                                        let a561 = 10u64;
                                        if nat_eq(a560, a561) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a562 = v1;
                                                let a563 = 1u64;
                                                nat_add(a562, a563)?
                                            }, (v4, f147(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a564 = v1;
                                                let a565 = 1u64;
                                                nat_add(a564, a565)?
                                            }, (v4, Adt16::C0(false, false, f203(match v3.clone() {
                                                Adt16::C0(_, _, h566, _, _) => {
                                                    h566
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h567, _) => {
                                                    h567
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h568) => {
                                                    h568
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
                Ok(next) => { m569 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f204(List::cons(match v0.clone() {
        Adt16::C0(_, _, h573, _, _) => {
            h573
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h574, _) => {
            h574
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h575) => {
            h575
        }
    }))
}

pub fn f148(p578: List<List<Bytes>>, p579: List<List<Bytes>>) -> List<List<Bytes>> {
    let m577 = (p578, p579);
    {
        let mut m577 = m577;
        loop {
            let step = {
                let (v0, v1) = m577;
                let m576 = v0;
                match m576.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f205(v0.clone(), 0u64, {
        let a580 = v0;
        length_bytes(a580)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f150(v0: Adt20) -> Bytes {
    let a601 = f206(match v0.clone() {
        Adt20::C0(h581, _, _, _, _, _, _, _) => {
            h581
        }
    });
    let a602 = {
        let a599 = f206(match v0.clone() {
            Adt20::C0(_, h582, _, _, _, _, _, _) => {
                h582
            }
        });
        let a600 = {
            let a597 = f206(match v0.clone() {
                Adt20::C0(_, _, h583, _, _, _, _, _) => {
                    h583
                }
            });
            let a598 = {
                let a595 = f206(match v0.clone() {
                    Adt20::C0(_, _, _, h584, _, _, _, _) => {
                        h584
                    }
                });
                let a596 = {
                    let a593 = f206(match v0.clone() {
                        Adt20::C0(_, _, _, _, h585, _, _, _) => {
                            h585
                        }
                    });
                    let a594 = {
                        let a591 = f206(match v0.clone() {
                            Adt20::C0(_, _, _, _, _, h586, _, _) => {
                                h586
                            }
                        });
                        let a592 = {
                            let a589 = f206(match v0.clone() {
                                Adt20::C0(_, _, _, _, _, _, h587, _) => {
                                    h587
                                }
                            });
                            let a590 = f206(match v0 {
                                Adt20::C0(_, _, _, _, _, _, _, h588) => {
                                    h588
                                }
                            });
                            append_bytes(a589, a590)
                        };
                        append_bytes(a591, a592)
                    };
                    append_bytes(a593, a594)
                };
                append_bytes(a595, a596)
            };
            append_bytes(a597, a598)
        };
        append_bytes(a599, a600)
    };
    append_bytes(a601, a602)
}

pub fn f151(v0: Bytes) -> R<Adt20> {
    let v1: u64 = {
        let a603 = v0.clone();
        length_bytes(a603)
    };
    let v2: u64 = {
        let a609 = {
            let a606 = {
                let a604 = v1;
                let a605 = 72u64;
                nat_add(a604, a605)?
            };
            let a607 = 64u64;
            let a608 = 0u64;
            nat_quot(a606, a607, a608)
        };
        let a610 = 64u64;
        nat_mul(a609, a610)?
    };
    let (_, h611) = f207(Fn4::F208(v0, v1, v2), {
        let a612 = v2;
        let a613 = 64u64;
        let a614 = 0u64;
        nat_quot(a612, a613, a614)
    }, (0u64, f209()))?;
    Ok(h611)
}

pub fn f152(p617: List<List<Bytes>>, v1: Bytes, p618: List<Bytes>) -> R<List<Bytes>> {
    let m616 = (p617, p618);
    {
        let mut m616 = m616;
        loop {
            let step = {
                let (v0, v2) = m616;
                let m615 = v0;
                match m615.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f210(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f155(f211(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(p621: List<Bytes>, p622: List<Bytes>, p623: List<List<Bytes>>) -> List<List<Bytes>> {
    let m620 = (p621, (p622, p623));
    {
        let mut m620 = m620;
        loop {
            let step = {
                let (v0, (v1, v2)) = m620;
                let m619 = v0;
                match m619.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f212(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f27(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f212(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Bytes) -> R<Bytes> {
    Ok(f45(v0.clone(), 0u64, f213(v0)?))
}

pub fn f155(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m624 = v0;
    match m624.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m625 = v3.clone();
            match m625.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a628 = v2;
                    let a629 = {
                        let a626 = v1.clone();
                        let a627 = f155(v3, v1);
                        append_bytes(a626, a627)
                    };
                    append_bytes(a628, a629)
                }
            }
        }
    }
}

pub fn f156(v0: List<Bytes>) -> Adt14 {
    let m630 = v0;
    match m630.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f156(v2)))
        }
    }
}

pub fn f157(v0: Fn2, p634: Option<Bytes>, p635: List<Bytes>) -> Option<Bytes> {
    let m633 = (p634, p635);
    {
        let mut m633 = m633;
        loop {
            let step = {
                let (v1, v2) = m633;
                let m631 = v2;
                match m631.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c632: Fn2 = v0.clone();
                            c632.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m633 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m636 = v1;
    match m636 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a639 = {
                    let a637 = v3;
                    let a638 = v0;
                    append_bytes(a637, a638)
                };
                let a640 = v2;
                append_bytes(a639, a640)
            })
        }
    }
}

pub fn f159(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f213(v0.clone())?;
    let v2: u64 = f214(v0.clone(), v1)?;
    Ok(f45(v0, v2, {
        let a641 = v1;
        let a642 = v2;
        nat_sub(a641, a642)
    }))
}

pub fn f160(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f215(v0)?;
    let m646 = f216(v1.clone(), 0u64, {
        let a644 = {
            let a643 = v1.clone();
            length_bytes(a643)
        };
        let a645 = 2u64;
        nat_add(a644, a645)?
    }, 0u64)?;
    match m646 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a649 = {
                let (_, h647) = v2;
                h647
            };
            let a650 = {
                let a648 = v1;
                length_bytes(a648)
            };
            if nat_eq(a649, a650) {
                Ok(Some({
                    let (h651, _) = v2;
                    h651
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f161(v0: Bytes) -> Bytes {
    f7(List::cons(f63(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f162(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f217(v0, v1)
}

pub fn f163(v0: Adt14) -> Bytes {
    if f173(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f173(v0.clone(), f8(Str::lit("id"))) {
            if f173(v0.clone(), f8(Str::lit("statement"))) {
                if f173(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f174(v0, f8(Str::lit("evidence")))
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

pub fn f164(v0: List<Adt14>) -> Adt14 {
    let m652 = v0;
    match m652.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f164(v2)))
        }
    }
}

pub fn f165(v0: Adt14) -> bool {
    if f173(v0.clone(), f8(Str::lit("name"))) {
        if f174(v0.clone(), f8(Str::lit("schema"))) {
            let m653 = f167(f60(f108(f104(v0, f8(Str::lit("citation"))))));
            match m653 {
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

pub fn f166(v0: Bytes) -> R<Bytes> {
    f218(v0.clone(), 0u64, {
        let a654 = v0;
        length_bytes(a654)
    }, true, Bytes::lit(&[]))
}

pub fn f167(v0: List<Bytes>) -> Option<Adt14> {
    let m655 = v0;
    match m655.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f27(v1.clone(), f8(Str::lit("req"))) {
                let m656 = v2;
                match m656.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f27(v1.clone(), f8(Str::lit("std"))) {
                    let m657 = v2;
                    match m657.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m658 = v6;
                                match m658.uncons() {
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
                        let m659 = v2;
                        match m659.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m660 = v10;
                                match m660.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f156(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f168(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m661 = f219(4096u64, f220(f221(f222(f104(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt21::C0(v2, v0), List::<Adt21>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m661 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f155(f212(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f169(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m662 = v0;
    match m662.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f169(v3, v1))
        }
    }
}

pub fn f170(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f155(f223(f96(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f171(v0: Adt14) -> R<Bytes> {
    let m663 = v0;
    match m663 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f224(v2, v3)
        }
        Adt14::C2(v4) => {
            f225(v4)
        }
        Adt14::C3(r664) => {
            let v5 = (*r664).clone();
            Ok(f226(f227(v5)?))
        }
        Adt14::C4(r665) => {
            let v6 = (*r665).clone();
            f228(f229(v6)?)
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

pub fn f172(v0: Adt14) -> List<Adt14> {
    let m666 = v0;
    match m666 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r667) => {
            let v3 = (*r667).clone();
            f172(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r668, r669) => {
            let v1 = (*r668).clone();
            let v2 = (*r669).clone();
            List::cons(v1, f172(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f173(v0: Adt14, v1: Bytes) -> bool {
    f61({
        let a671 = {
            let a670 = f107(v0, v1);
            length_bytes(a670)
        };
        let a672 = 0u64;
        nat_eq(a671, a672)
    })
}

pub fn f174(v0: Adt14, v1: Bytes) -> bool {
    let m673 = f104(v0, v1);
    match m673 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f175(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m674 = f104(v0, v1);
    match m674 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f108(f104(v3, v2))
        }
    }
}

pub fn f176(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f104(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f90(Str::lit("id"), f107(v0.clone(), f8(Str::lit("id"))))?, List::cons(f90(Str::lit("name"), f107(v0.clone(), f8(Str::lit("name"))))?, List::cons(f90(Str::lit("issuer"), f107(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f90(Str::lit("edition"), f107(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f90(Str::lit("canonical_id"), f107(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f90(Str::lit("source_role"), f107(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f90(Str::lit("retrieval_date"), f107(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f90(Str::lit("url"), f230(v2.clone(), f8(Str::lit("url"))))?, List::cons(f90(Str::lit("media_type"), f230(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f90(Str::lit("license"), f230(v2.clone(), f8(Str::lit("license"))))?, List::cons(f90(Str::lit("redistribution"), f230(v2, f8(Str::lit("redistribution"))))?, List::cons(f90(Str::lit("statement"), f107(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m675 = v1;
        match m675 {
            None => {
                f7(List::cons(f90(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f90(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f90(Str::lit("artifact"), {
                    let (h676, _) = v3;
                    h676
                })?, List::cons(f90(Str::lit("checksum"), {
                    let (_, h677) = v3;
                    h677
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f231(f60(f108(f104(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f177(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h678, _) = v2.clone();
        h678
    };
    let v4: u64 = {
        let (h679, _) = {
            let (_, h680) = v2.clone();
            h680
        };
        h679
    };
    let v5: List<Bytes> = {
        let (_, h681) = {
            let (_, h682) = v2;
            h682
        };
        h681
    };
    let m683 = f232(v0.clone(), v3);
    match m683 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a684 = v6;
            let a685 = v1;
            if equal(a684, a685) {
                Ok(Some(({
                    let a686 = v3;
                    let a687 = 1u64;
                    nat_add(a686, a687)?
                }, ({
                    let a688 = v3;
                    let a689 = 1u64;
                    nat_add(a688, a689)?
                }, List::cons(f45(v0, v4, {
                    let a690 = v3;
                    let a691 = v4;
                    nat_sub(a690, a691)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a692 = v3;
                    let a693 = 1u64;
                    nat_add(a692, a693)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f178(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f179(v0: Bytes) -> bool {
    f122(v0, f8(Str::lit(".lex.tex")))
}

pub fn f180(v0: Fn3, p698: u64, p699: u64) -> R<(u64, bool)> {
    let m697 = (p698, p699);
    {
        let mut m697 = m697;
        loop {
            let step = {
                let (v1, v2) = m697;
                let m694 = v1;
                if m694 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m694 - 1;
                    let m696 = {
                        let c695: Fn3 = v0.clone();
                        c695.apply(v2)?
                    };
                    match m696 {
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
                Ok(next) => { m697 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f182(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a702 = {
            let a700 = v4;
            let a701 = v1;
            nat_add(a700, a701)?
        };
        let a703 = v3;
        if nat_lt(a702, a703) {
            Ok(Some({
                let a704 = v4;
                let a705 = 1u64;
                nat_add(a704, a705)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f182(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m710 = {
        let a707 = v0;
        let a708 = v2;
        let a709 = {
            let a706 = v1.clone();
            length_bytes(a706)
        };
        slice_bytes(a707, a708, a709)
    };
    match m710 {
        None => {
            false
        }
        Some(v3) => {
            let a711 = v3;
            let a712 = v1;
            equal(a711, a712)
        }
    }
}

pub fn f183(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m713 = v0;
    match m713.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f183(v3, {
                let a714 = v1;
                let a715 = 1u64;
                nat_add(a714, a715)?
            })?))
        }
    }
}

pub fn f184(v0: Adt15, v1: List<u64>, v2: Adt18) -> bool {
    let m716 = v2;
    match m716 {
        Adt18::C0(v3, v4) => {
            if f190(v3, match v0.clone() {
                Adt15::C0(h717, _, _, _) => {
                    h717
                }
            }) {
                f233(v0, v4)
            } else {
                false
            }
        }
        Adt18::C1(v5) => {
            f190(v5, match v0 {
                Adt15::C0(_, _, h718, _) => {
                    h718
                }
            })
        }
        Adt18::C2(_, v7, v8) => {
            if f186(v7, v1) {
                f190(v8, match v0 {
                    Adt15::C0(_, _, _, h719) => {
                        h719
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f185(v0: Adt12, v1: Adt14) -> R<Adt18> {
    let m720 = f104(v1.clone(), f8(Str::lit("std")));
    match m720 {
        None => {
            let m725 = f112(f104(v1.clone(), f8(Str::lit("req"))));
            match m725 {
                None => {
                    let m727 = f104(v1, f8(Str::lit("der")));
                    match m727 {
                        None => {
                            Ok(f234(v0))
                        }
                        Some(v5) => {
                            let m728 = f235(f107(v5.clone(), f8(Str::lit("rule"))));
                            match m728 {
                                None => {
                                    Ok(f234(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt18::C2(v6, f132(f60(f108(f104(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h729) => {
                                            h729
                                        }
                                    })?, f188(f107(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h730, _) => {
                                            h730
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt18::C1(f188(v4, match v0 {
                        Adt12::C0(_, _, h726, _, _) => {
                            h726
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m721 = f112(f104(v2.clone(), f8(Str::lit("row"))));
            match m721 {
                None => {
                    Ok(Adt18::C0(f188(f107(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h722, _, _, _, _) => {
                            h722
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt18::C0(f188(f107(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h723, _, _, _, _) => {
                            h723
                        }
                    })?, Some(f188(v3, match v0 {
                        Adt12::C0(_, h724, _, _, _) => {
                            h724
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f186(p733: List<u64>, v1: List<u64>) -> bool {
    let m732 = p733;
    {
        let mut m732 = m732;
        loop {
            let step = {
                let v0 = m732;
                let m731 = v0;
                match m731.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f190(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(p738: List<u64>) -> bool {
    let m737 = p738;
    {
        let mut m737 = m737;
        loop {
            let step = {
                let v0 = m737;
                let m734 = v0;
                match m734.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a735 = v1;
                        let a736 = 7u64;
                        if nat_le(a735, a736) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f188(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m739 = f236(v0, v1.clone(), 0u64)?;
    match m739 {
        None => {
            let a740 = v1;
            Ok(length_list(a740))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f189(v0: Adt14) -> u64 {
    let m741 = f237(f104(v0, f8(Str::lit("stratum"))));
    match m741 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f190(v0: u64, p746: List<u64>) -> bool {
    let m745 = p746;
    {
        let mut m745 = m745;
        loop {
            let step = {
                let v1 = m745;
                let m742 = v1;
                match m742.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a743 = v2;
                        let a744 = v0;
                        if nat_eq(a743, a744) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
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

pub fn f191(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f238(v0.clone(), v1) {
        let m747 = f43(v0, 34u8)?;
        match m747.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m748 = v3;
                match m748.uncons() {
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

pub fn f192(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f193(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m749 = v0.clone();
    match m749.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a750 = v2;
            let a751 = v1.clone();
            if equal(a750, a751) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f194(v0: List<(Bytes, ())>) -> List<Bytes> {
    f136(f239(Fn5::F240, List::<Bytes>::nil(), v0))
}

pub fn f195(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f241(f242(Fn6::F243, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m756 = {
        let (h752, _) = f244(Fn7::F245, {
            let a754 = {
                let a753 = v0;
                length_list(a753)
            };
            let a755 = 1u64;
            nat_add(a754, a755)?
        }, v1)?;
        h752
    };
    match m756.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f196(v0: List<Bytes>) -> List<(Bytes, ())> {
    f246(f247(Fn8::F248, List::<(Bytes, ())>::nil(), v0))
}

pub fn f197(p759: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m758 = p759;
    {
        let mut m758 = m758;
        loop {
            let step = {
                let v0 = m758;
                let m757 = v0;
                match m757.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f210(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Adt14, v1: Bytes) -> Bytes {
    if f27(f230(f104(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f199(v0: Bytes) -> R<Bytes> {
    let a760 = f8(Str::lit("Catalog."));
    let a761 = f166(v0)?;
    Ok(append_bytes(a760, a761))
}

pub fn f200(v0: List<Adt14>) -> R<List<Bytes>> {
    let m762 = v0;
    match m762.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f94(f60(f108(f104(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f107(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f107(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f249(f96(f107(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f200(v2)?))
        }
    }
}

pub fn f201(v0: Adt14) -> Bytes {
    if f173(v0.clone(), f8(Str::lit("req"))) {
        let a763 = f8(Str::lit("req "));
        let a764 = f107(v0, f8(Str::lit("req")));
        append_bytes(a763, a764)
    } else {
        let m765 = f104(v0.clone(), f8(Str::lit("std")));
        match m765 {
            None => {
                let m768 = f104(v0, f8(Str::lit("der")));
                match m768 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f107(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f94(f60(f108(f104(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f107(v1.clone(), f8(Str::lit("authority"))), List::cons(if f173(v1.clone(), f8(Str::lit("row"))) {
                    let a766 = f8(Str::lit(" row "));
                    let a767 = f107(v1, f8(Str::lit("row")));
                    append_bytes(a766, a767)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f202(v0: Bytes, v1: u64) -> R<u64> {
    let m771 = {
        let a769 = v0;
        let a770 = v1;
        index_bytes(a769, a770)
    };
    match m771 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a816 = {
                let a792 = {
                    let a780 = {
                        let a774 = {
                            let a772 = v3;
                            let a773 = 128u8;
                            fixed_u8::bit_and(a772, a773)
                        };
                        let a775 = 128u8;
                        if equal(a774, a775) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a781 = {
                        let a778 = {
                            let a776 = v3;
                            let a777 = 64u8;
                            fixed_u8::bit_and(a776, a777)
                        };
                        let a779 = 64u8;
                        if equal(a778, a779) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a780, a781)?
                };
                let a793 = {
                    let a790 = {
                        let a784 = {
                            let a782 = v3;
                            let a783 = 32u8;
                            fixed_u8::bit_and(a782, a783)
                        };
                        let a785 = 32u8;
                        if equal(a784, a785) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a791 = {
                        let a788 = {
                            let a786 = v3;
                            let a787 = 16u8;
                            fixed_u8::bit_and(a786, a787)
                        };
                        let a789 = 16u8;
                        if equal(a788, a789) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a790, a791)?
                };
                nat_add(a792, a793)?
            };
            let a817 = {
                let a814 = {
                    let a802 = {
                        let a796 = {
                            let a794 = v3;
                            let a795 = 8u8;
                            fixed_u8::bit_and(a794, a795)
                        };
                        let a797 = 8u8;
                        if equal(a796, a797) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a803 = {
                        let a800 = {
                            let a798 = v3;
                            let a799 = 4u8;
                            fixed_u8::bit_and(a798, a799)
                        };
                        let a801 = 4u8;
                        if equal(a800, a801) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a802, a803)?
                };
                let a815 = {
                    let a812 = {
                        let a806 = {
                            let a804 = v3;
                            let a805 = 2u8;
                            fixed_u8::bit_and(a804, a805)
                        };
                        let a807 = 2u8;
                        if equal(a806, a807) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a813 = {
                        let a810 = {
                            let a808 = v3;
                            let a809 = 1u8;
                            fixed_u8::bit_and(a808, a809)
                        };
                        let a811 = 1u8;
                        if equal(a810, a811) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a812, a813)?
                };
                nat_add(a814, a815)?
            };
            nat_add(a816, a817)
        }
    }
}

pub fn f203(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m821 = {
        let a818 = v1;
        let a819 = v2;
        let a820 = 1u64;
        slice_bytes(a818, a819, a820)
    };
    match m821 {
        None => {
            v0
        }
        Some(v3) => {
            let a822 = v0;
            let a823 = v3;
            append_bytes(a822, a823)
        }
    }
}

pub fn f204(p826: List<Bytes>, p827: List<Bytes>) -> List<Bytes> {
    let m825 = (p826, p827);
    {
        let mut m825 = m825;
        loop {
            let step = {
                let (v0, v1) = m825;
                let m824 = v0;
                match m824.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f205(v0: Bytes, p899: u64, p900: u64, p901: Bytes) -> R<Bytes> {
    let m898 = (p899, (p900, p901));
    {
        let mut m898 = m898;
        loop {
            let step = {
                let (v1, (v2, v3)) = m898;
                let m828 = v2;
                if m828 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m828 - 1;
                    let m831 = {
                        let a829 = v0.clone();
                        let a830 = v1;
                        index_bytes(a829, a830)
                    };
                    match m831 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a876 = {
                                    let a852 = {
                                        let a840 = {
                                            let a834 = {
                                                let a832 = v6;
                                                let a833 = 128u8;
                                                fixed_u8::bit_and(a832, a833)
                                            };
                                            let a835 = 128u8;
                                            if equal(a834, a835) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a841 = {
                                            let a838 = {
                                                let a836 = v6;
                                                let a837 = 64u8;
                                                fixed_u8::bit_and(a836, a837)
                                            };
                                            let a839 = 64u8;
                                            if equal(a838, a839) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a840, a841)?
                                    };
                                    let a853 = {
                                        let a850 = {
                                            let a844 = {
                                                let a842 = v6;
                                                let a843 = 32u8;
                                                fixed_u8::bit_and(a842, a843)
                                            };
                                            let a845 = 32u8;
                                            if equal(a844, a845) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a851 = {
                                            let a848 = {
                                                let a846 = v6;
                                                let a847 = 16u8;
                                                fixed_u8::bit_and(a846, a847)
                                            };
                                            let a849 = 16u8;
                                            if equal(a848, a849) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a850, a851)?
                                    };
                                    nat_add(a852, a853)?
                                };
                                let a877 = {
                                    let a874 = {
                                        let a862 = {
                                            let a856 = {
                                                let a854 = v6;
                                                let a855 = 8u8;
                                                fixed_u8::bit_and(a854, a855)
                                            };
                                            let a857 = 8u8;
                                            if equal(a856, a857) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a863 = {
                                            let a860 = {
                                                let a858 = v6;
                                                let a859 = 4u8;
                                                fixed_u8::bit_and(a858, a859)
                                            };
                                            let a861 = 4u8;
                                            if equal(a860, a861) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a862, a863)?
                                    };
                                    let a875 = {
                                        let a872 = {
                                            let a866 = {
                                                let a864 = v6;
                                                let a865 = 2u8;
                                                fixed_u8::bit_and(a864, a865)
                                            };
                                            let a867 = 2u8;
                                            if equal(a866, a867) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a873 = {
                                            let a870 = {
                                                let a868 = v6;
                                                let a869 = 1u8;
                                                fixed_u8::bit_and(a868, a869)
                                            };
                                            let a871 = 1u8;
                                            if equal(a870, a871) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a872, a873)?
                                    };
                                    nat_add(a874, a875)?
                                };
                                nat_add(a876, a877)?
                            };
                            let a878 = v7;
                            let a879 = 34u64;
                            if if nat_eq(a878, a879) {
                                true
                            } else {
                                let a880 = v7;
                                let a881 = 92u64;
                                nat_eq(a880, a881)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a882 = v1;
                                    let a883 = 1u64;
                                    nat_add(a882, a883)?
                                }, (v4, {
                                    let a886 = v3;
                                    let a887 = {
                                        let a884 = Bytes::lit(&[0x5c]);
                                        let a885 = f45(v0.clone(), v1, 1u64);
                                        append_bytes(a884, a885)
                                    };
                                    append_bytes(a886, a887)
                                })))
                            } else {
                                let a888 = v7;
                                let a889 = 10u64;
                                if nat_eq(a888, a889) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a890 = v1;
                                        let a891 = 1u64;
                                        nat_add(a890, a891)?
                                    }, (v4, {
                                        let a892 = v3;
                                        let a893 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a892, a893)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a894 = v1;
                                        let a895 = 1u64;
                                        nat_add(a894, a895)?
                                    }, (v4, {
                                        let a896 = v3;
                                        let a897 = f45(v0.clone(), v1, 1u64);
                                        append_bytes(a896, a897)
                                    })))
                                }
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

pub fn f206(v0: u32) -> Bytes {
    let a930 = f250({
        let a902 = f251(v0, 28u32);
        let a903 = 15u32;
        fixed_u32::bit_and(a902, a903)
    });
    let a931 = {
        let a928 = f250({
            let a904 = f251(v0, 24u32);
            let a905 = 15u32;
            fixed_u32::bit_and(a904, a905)
        });
        let a929 = {
            let a926 = f250({
                let a906 = f251(v0, 20u32);
                let a907 = 15u32;
                fixed_u32::bit_and(a906, a907)
            });
            let a927 = {
                let a924 = f250({
                    let a908 = f251(v0, 16u32);
                    let a909 = 15u32;
                    fixed_u32::bit_and(a908, a909)
                });
                let a925 = {
                    let a922 = f250({
                        let a910 = f251(v0, 12u32);
                        let a911 = 15u32;
                        fixed_u32::bit_and(a910, a911)
                    });
                    let a923 = {
                        let a920 = f250({
                            let a912 = f251(v0, 8u32);
                            let a913 = 15u32;
                            fixed_u32::bit_and(a912, a913)
                        });
                        let a921 = {
                            let a918 = f250({
                                let a914 = f251(v0, 4u32);
                                let a915 = 15u32;
                                fixed_u32::bit_and(a914, a915)
                            });
                            let a919 = f250({
                                let a916 = v0;
                                let a917 = 15u32;
                                fixed_u32::bit_and(a916, a917)
                            });
                            append_bytes(a918, a919)
                        };
                        append_bytes(a920, a921)
                    };
                    append_bytes(a922, a923)
                };
                append_bytes(a924, a925)
            };
            append_bytes(a926, a927)
        };
        append_bytes(a928, a929)
    };
    append_bytes(a930, a931)
}

pub fn f207(v0: Fn4, p935: u64, p936: (u64, Adt20)) -> R<(u64, Adt20)> {
    let m934 = (p935, p936);
    {
        let mut m934 = m934;
        loop {
            let step = {
                let (v1, v2) = m934;
                let m932 = v1;
                if m932 == 0 {
                    Err::<(u64, (u64, Adt20)), R<(u64, Adt20)>>(Ok(v2))
                } else {
                    let v3 = m932 - 1;
                    Ok::<(u64, (u64, Adt20)), R<(u64, Adt20)>>((v3, {
                        let c933: Fn4 = v0.clone();
                        c933.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m934 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt20)) -> R<(u64, Adt20)> {
    Ok(({
        let a938 = {
            let (h937, _) = v3.clone();
            h937
        };
        let a939 = 64u64;
        nat_add(a938, a939)?
    }, f252({
        let (_, h940) = v3;
        h940
    }, f253(v0, v1, v2, {
        let (h941, _) = v3;
        h941
    })?)?))
}

pub fn f209() -> Adt20 {
    Adt20::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f210(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f70(v1, f55(f155(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f211(v0: List<Bytes>) -> List<Bytes> {
    let m942 = v0;
    match m942.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f27(v1.clone(), f8(Str::lit("authority = []"))) {
                f211(v2)
            } else {
                List::cons(v1, f211(v2))
            }
        }
    }
}

pub fn f212(p945: List<Bytes>, p946: List<Bytes>) -> List<Bytes> {
    let m944 = (p945, p946);
    {
        let mut m944 = m944;
        loop {
            let step = {
                let (v0, v1) = m944;
                let m943 = v0;
                match m943.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m944 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Bytes) -> R<u64> {
    let (h947, _) = f180(Fn3::F254(v0.clone()), {
        let a948 = v0.clone();
        length_bytes(a948)
    }, {
        let a949 = v0;
        length_bytes(a949)
    })?;
    Ok(h947)
}

pub fn f214(v0: Bytes, v1: u64) -> R<u64> {
    let (h950, _) = f180(Fn3::F255(v1, v0), v1, 0u64)?;
    Ok(h950)
}

pub fn f215(v0: Bytes) -> R<Bytes> {
    f256(v0.clone(), 0u64, {
        let a951 = v0;
        length_bytes(a951)
    }, false, false, Bytes::lit(&[]))
}

pub fn f216(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m952 = v2;
    if m952 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m952 - 1;
        let v5: u64 = f257(v0.clone(), v1)?;
        let a953 = v3;
        let a954 = 1u64;
        if nat_eq(a953, a954) {
            let m955 = f216(v0.clone(), v1, v4, 0u64)?;
            match m955 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f257(v0.clone(), {
                        let (_, h956) = v6.clone();
                        h956
                    })?;
                    let a957 = v7;
                    let a958 = 44u64;
                    if nat_eq(a957, a958) {
                        let m962 = f216(v0, {
                            let a960 = {
                                let (_, h959) = v6;
                                h959
                            };
                            let a961 = 1u64;
                            nat_add(a960, a961)?
                        }, v4, 1u64)?;
                        match m962 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h963, _) = v6;
                                    h963
                                }), std::rc::Rc::new({
                                    let (h964, _) = v8;
                                    h964
                                })), {
                                    let (_, h965) = v8;
                                    h965
                                })))
                            }
                        }
                    } else {
                        let a966 = v7;
                        let a967 = 93u64;
                        if nat_eq(a966, a967) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h968, _) = v6;
                                h968
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a970 = {
                                    let (_, h969) = v6;
                                    h969
                                };
                                let a971 = 1u64;
                                nat_add(a970, a971)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a972 = v3;
            let a973 = 2u64;
            if nat_eq(a972, a973) {
                let m976 = f258(v0.clone(), {
                    let a974 = v1;
                    let a975 = 1u64;
                    nat_add(a974, a975)?
                })?;
                match m976 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a977 = v5;
                        let a978 = 34u64;
                        if if nat_eq(a977, a978) {
                            let a980 = f257(v0.clone(), {
                                let (h979, _) = v9.clone();
                                h979
                            })?;
                            let a981 = 58u64;
                            nat_eq(a980, a981)
                        } else {
                            false
                        } {
                            let m985 = f216(v0.clone(), {
                                let a983 = {
                                    let (h982, _) = v9;
                                    h982
                                };
                                let a984 = 1u64;
                                nat_add(a983, a984)?
                            }, v4, 0u64)?;
                            match m985 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f257(v0.clone(), {
                                        let (_, h986) = v10.clone();
                                        h986
                                    })?;
                                    let a987 = v11;
                                    let a988 = 44u64;
                                    if nat_eq(a987, a988) {
                                        let m992 = f216(v0, {
                                            let a990 = {
                                                let (_, h989) = v10;
                                                h989
                                            };
                                            let a991 = 1u64;
                                            nat_add(a990, a991)?
                                        }, v4, 2u64)?;
                                        match m992 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h993) = v9;
                                                    h993
                                                }, std::rc::Rc::new({
                                                    let (h994, _) = v10;
                                                    h994
                                                }), std::rc::Rc::new({
                                                    let (h995, _) = v12;
                                                    h995
                                                })), {
                                                    let (_, h996) = v12;
                                                    h996
                                                })))
                                            }
                                        }
                                    } else {
                                        let a997 = v11;
                                        let a998 = 125u64;
                                        if nat_eq(a997, a998) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h999) = v9;
                                                h999
                                            }, std::rc::Rc::new({
                                                let (h1000, _) = v10;
                                                h1000
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1002 = {
                                                    let (_, h1001) = v10;
                                                    h1001
                                                };
                                                let a1003 = 1u64;
                                                nat_add(a1002, a1003)?
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
                let a1004 = v5;
                let a1005 = 34u64;
                if nat_eq(a1004, a1005) {
                    let m1008 = f258(v0, {
                        let a1006 = v1;
                        let a1007 = 1u64;
                        nat_add(a1006, a1007)?
                    })?;
                    match m1008 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1009) = v13;
                                h1009
                            }), {
                                let (h1010, _) = v13;
                                h1010
                            })))
                        }
                    }
                } else {
                    if f27(f45(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1011 = v1;
                            let a1012 = 4u64;
                            nat_add(a1011, a1012)?
                        })))
                    } else {
                        if f27(f45(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1013 = v1;
                                let a1014 = 5u64;
                                nat_add(a1013, a1014)?
                            })))
                        } else {
                            let a1015 = v5;
                            let a1016 = 91u64;
                            if nat_eq(a1015, a1016) {
                                let a1019 = f257(v0.clone(), {
                                    let a1017 = v1;
                                    let a1018 = 1u64;
                                    nat_add(a1017, a1018)?
                                })?;
                                let a1020 = 93u64;
                                if nat_eq(a1019, a1020) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1021 = v1;
                                        let a1022 = 2u64;
                                        nat_add(a1021, a1022)?
                                    })))
                                } else {
                                    let m1025 = f216(v0, {
                                        let a1023 = v1;
                                        let a1024 = 1u64;
                                        nat_add(a1023, a1024)?
                                    }, v4, 1u64)?;
                                    match m1025 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1026, _) = v14;
                                                h1026
                                            })), {
                                                let (_, h1027) = v14;
                                                h1027
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1028 = v5;
                                let a1029 = 123u64;
                                if nat_eq(a1028, a1029) {
                                    let a1032 = f257(v0.clone(), {
                                        let a1030 = v1;
                                        let a1031 = 1u64;
                                        nat_add(a1030, a1031)?
                                    })?;
                                    let a1033 = 125u64;
                                    if nat_eq(a1032, a1033) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1034 = v1;
                                            let a1035 = 2u64;
                                            nat_add(a1034, a1035)?
                                        })))
                                    } else {
                                        let m1038 = f216(v0, {
                                            let a1036 = v1;
                                            let a1037 = 1u64;
                                            nat_add(a1036, a1037)?
                                        }, v4, 2u64)?;
                                        match m1038 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1039, _) = v15;
                                                    h1039
                                                })), {
                                                    let (_, h1040) = v15;
                                                    h1040
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1041 = f259(v0, v1)?;
                                    match m1041 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1042) = v16;
                                                h1042
                                            }, {
                                                let (h1043, _) = v16;
                                                h1043
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

pub fn f217(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1044, _) = v1.clone();
        h1044
    };
    let v3: u64 = {
        let (h1045, _) = {
            let (_, h1046) = v1.clone();
            h1046
        };
        h1045
    };
    let v4: List<Bytes> = {
        let (_, h1047) = {
            let (_, h1048) = v1;
            h1048
        };
        h1047
    };
    let m1049 = f232(v0.clone(), v2);
    match m1049 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1050 = v5;
            let a1051 = 10u8;
            if equal(a1050, a1051) {
                Ok(Some(({
                    let a1052 = v2;
                    let a1053 = 1u64;
                    nat_add(a1052, a1053)?
                }, ({
                    let a1054 = v2;
                    let a1055 = 1u64;
                    nat_add(a1054, a1055)?
                }, List::cons(f260(f45(v0, v3, {
                    let a1056 = v2;
                    let a1057 = v3;
                    nat_sub(a1056, a1057)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1058 = v2;
                    let a1059 = 1u64;
                    nat_add(a1058, a1059)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f218(v0: Bytes, p1126: u64, p1127: u64, p1128: bool, p1129: Bytes) -> R<Bytes> {
    let m1125 = (p1126, (p1127, (p1128, p1129)));
    {
        let mut m1125 = m1125;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1125;
                let m1060 = v2;
                if m1060 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1060 - 1;
                    let m1063 = {
                        let a1061 = v0.clone();
                        let a1062 = v1;
                        index_bytes(a1061, a1062)
                    };
                    match m1063 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1108 = {
                                    let a1084 = {
                                        let a1072 = {
                                            let a1066 = {
                                                let a1064 = v7;
                                                let a1065 = 128u8;
                                                fixed_u8::bit_and(a1064, a1065)
                                            };
                                            let a1067 = 128u8;
                                            if equal(a1066, a1067) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1073 = {
                                            let a1070 = {
                                                let a1068 = v7;
                                                let a1069 = 64u8;
                                                fixed_u8::bit_and(a1068, a1069)
                                            };
                                            let a1071 = 64u8;
                                            if equal(a1070, a1071) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1072, a1073)?
                                    };
                                    let a1085 = {
                                        let a1082 = {
                                            let a1076 = {
                                                let a1074 = v7;
                                                let a1075 = 32u8;
                                                fixed_u8::bit_and(a1074, a1075)
                                            };
                                            let a1077 = 32u8;
                                            if equal(a1076, a1077) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1083 = {
                                            let a1080 = {
                                                let a1078 = v7;
                                                let a1079 = 16u8;
                                                fixed_u8::bit_and(a1078, a1079)
                                            };
                                            let a1081 = 16u8;
                                            if equal(a1080, a1081) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1082, a1083)?
                                    };
                                    nat_add(a1084, a1085)?
                                };
                                let a1109 = {
                                    let a1106 = {
                                        let a1094 = {
                                            let a1088 = {
                                                let a1086 = v7;
                                                let a1087 = 8u8;
                                                fixed_u8::bit_and(a1086, a1087)
                                            };
                                            let a1089 = 8u8;
                                            if equal(a1088, a1089) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1095 = {
                                            let a1092 = {
                                                let a1090 = v7;
                                                let a1091 = 4u8;
                                                fixed_u8::bit_and(a1090, a1091)
                                            };
                                            let a1093 = 4u8;
                                            if equal(a1092, a1093) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1094, a1095)?
                                    };
                                    let a1107 = {
                                        let a1104 = {
                                            let a1098 = {
                                                let a1096 = v7;
                                                let a1097 = 2u8;
                                                fixed_u8::bit_and(a1096, a1097)
                                            };
                                            let a1099 = 2u8;
                                            if equal(a1098, a1099) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1105 = {
                                            let a1102 = {
                                                let a1100 = v7;
                                                let a1101 = 1u8;
                                                fixed_u8::bit_and(a1100, a1101)
                                            };
                                            let a1103 = 1u8;
                                            if equal(a1102, a1103) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1104, a1105)?
                                    };
                                    nat_add(a1106, a1107)?
                                };
                                nat_add(a1108, a1109)?
                            };
                            if f261(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1110 = v1;
                                    let a1111 = 1u64;
                                    nat_add(a1110, a1111)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f262(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1112 = v1;
                                        let a1113 = 1u64;
                                        nat_add(a1112, a1113)?
                                    }, (v5, (false, {
                                        let a1117 = v4;
                                        let a1118 = f45({
                                            let a1114 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1114)
                                        }, {
                                            let a1115 = v8;
                                            let a1116 = 97u64;
                                            nat_sub(a1115, a1116)
                                        }, 1u64);
                                        append_bytes(a1117, a1118)
                                    }))))
                                } else {
                                    if if f262(v8) {
                                        true
                                    } else {
                                        if f263(v8) {
                                            true
                                        } else {
                                            f264(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1119 = v1;
                                            let a1120 = 1u64;
                                            nat_add(a1119, a1120)?
                                        }, (v5, (false, {
                                            let a1121 = v4;
                                            let a1122 = f45(v0.clone(), v1, 1u64);
                                            append_bytes(a1121, a1122)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1123 = v1;
                                            let a1124 = 1u64;
                                            nat_add(a1123, a1124)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
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

pub fn f219(p1137: u64, p1138: List<Adt21>, p1139: List<Bytes>, p1140: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1136 = (p1137, (p1138, (p1139, p1140)));
    {
        let mut m1136 = m1136;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1136;
                let m1130 = v0;
                if m1130 == 0 {
                    Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1130 - 1;
                    let m1131 = v1;
                    match m1131.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1132 = v5;
                            match m1132 {
                                Adt21::C0(v8, v9) => {
                                    if f70(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m1133 = f265(v8.clone(), v9)?;
                                        match m1133 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f220({
                                                    let (_, h1134) = v11;
                                                    h1134
                                                }, List::cons(Adt21::C1({
                                                    let (h1135, _) = v11;
                                                    h1135
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
                Ok(next) => { m1136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: List<Adt21>, v1: List<Adt21>) -> List<Adt21> {
    let m1141 = v0;
    match m1141.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f220(v3, v1))
        }
    }
}

pub fn f221(v0: List<(Bytes, Adt14)>) -> R<List<Adt21>> {
    let m1142 = v0;
    match m1142.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt21::C0(f166({
                let (h1143, _) = v1;
                h1143
            })?, {
                let (_, h1144) = v1;
                h1144
            }), f221(v2)?))
        }
    }
}

pub fn f222(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1145 = v0;
    match m1145 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f266(v1)
        }
    }
}

pub fn f223(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1146 = v0;
    match m1146.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f238(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f155(f267(f64(List::cons(v1, f268(f43(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f223(v3, v1)?))
            }
        }
    }
}

pub fn f224(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1147 = 0u64;
        let a1148 = v1;
        nat_lt(a1147, a1148)
    } else {
        false
    } {
        let a1149 = Bytes::lit(&[0x2d]);
        let a1150 = f269(v1)?;
        Ok(append_bytes(a1149, a1150))
    } else {
        f269(v1)
    }
}

pub fn f225(v0: Bytes) -> R<Bytes> {
    let a1156 = {
        let a1154 = Bytes::lit(&[0x22]);
        let a1155 = {
            let (_, h1151) = {
                let (h1152, _) = f270(Fn9::F271(v0.clone()), {
                    let a1153 = v0;
                    length_bytes(a1153)
                }, (0u64, Bytes::lit(&[])))?;
                h1152
            };
            h1151
        };
        append_bytes(a1154, a1155)
    };
    let a1157 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1156, a1157))
}

pub fn f226(v0: List<Bytes>) -> Bytes {
    let a1160 = {
        let a1158 = Bytes::lit(&[0x5b]);
        let a1159 = f94(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1158, a1159)
    };
    let a1161 = Bytes::lit(&[0x5d]);
    append_bytes(a1160, a1161)
}

pub fn f227(v0: Adt14) -> R<List<Bytes>> {
    let m1162 = v0;
    match m1162 {
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
        Adt14::C6(r1163, r1164) => {
            let v7 = (*r1163).clone();
            let v8 = (*r1164).clone();
            Ok(List::cons(f171(v7)?, f227(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f228(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f272(Fn10::F273, List::<(Bytes, Bytes)>::nil(), f274(v0)?);
    let v2: List<Bytes> = f275(Fn11::F276, List::<Bytes>::nil(), v1)?;
    let a1167 = {
        let a1165 = Bytes::lit(&[0x7b]);
        let a1166 = f94(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1165, a1166)
    };
    let a1168 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1167, a1168))
}

pub fn f229(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1169 = v0;
    match m1169 {
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
        Adt14::C7(v9, r1170, r1171) => {
            let v10 = (*r1170).clone();
            let v11 = (*r1171).clone();
            Ok(List::cons((v9, f171(v10)?), f229(v11)?))
        }
    }
}

pub fn f230(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1172 = v0;
    match m1172 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f107(v2, v1)
        }
    }
}

pub fn f231(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f155(f267(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f232(v0: Bytes, v1: u64) -> Option<u8> {
    let a1173 = v0;
    let a1174 = v1;
    index_bytes(a1173, a1174)
}

pub fn f233(v0: Adt15, v1: Option<u64>) -> bool {
    let m1175 = v1;
    match m1175 {
        None => {
            true
        }
        Some(v2) => {
            f190(v2, match v0 {
                Adt15::C0(_, h1176, _, _) => {
                    h1176
                }
            })
        }
    }
}

pub fn f234(v0: Adt12) -> Adt18 {
    Adt18::C1({
        let a1178 = match v0 {
            Adt12::C0(_, _, h1177, _, _) => {
                h1177
            }
        };
        length_list(a1178)
    })
}

pub fn f235(v0: Bytes) -> Option<Adt19> {
    if f27(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt19::C0)
    } else {
        if f27(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt19::C1)
        } else {
            if f27(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt19::C2)
            } else {
                None::<Adt19>
            }
        }
    }
}

pub fn f236(v0: Bytes, p1183: List<Bytes>, p1184: u64) -> R<Option<u64>> {
    let m1182 = (p1183, p1184);
    {
        let mut m1182 = m1182;
        loop {
            let step = {
                let (v1, v2) = m1182;
                let m1179 = v1;
                match m1179.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f27(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1180 = v2;
                                let a1181 = 1u64;
                                nat_add(a1180, a1181)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Option<Adt14>) -> Option<u64> {
    let m1185 = v0;
    match m1185 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1186 = v1;
            match m1186 {
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

pub fn f238(v0: Bytes, v1: Bytes) -> bool {
    let m1191 = {
        let a1188 = v0;
        let a1189 = 0u64;
        let a1190 = {
            let a1187 = v1.clone();
            length_bytes(a1187)
        };
        slice_bytes(a1188, a1189, a1190)
    };
    match m1191 {
        None => {
            false
        }
        Some(v2) => {
            let a1192 = v2;
            let a1193 = v1;
            equal(a1192, a1193)
        }
    }
}

pub fn f239(v0: Fn5, p1197: List<Bytes>, p1198: List<(Bytes, ())>) -> List<Bytes> {
    let m1196 = (p1197, p1198);
    {
        let mut m1196 = m1196;
        loop {
            let step = {
                let (v1, v2) = m1196;
                let m1194 = v2;
                match m1194.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1195: Fn5 = v0.clone();
                            c1195.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1196 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1199, _) = v1;
        h1199
    }, v0)
}

pub fn f241(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f277(Fn12::F278, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f242(v0: Fn6, p1203: List<List<(Bytes, ())>>, p1204: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1202 = (p1203, p1204);
    {
        let mut m1202 = m1202;
        loop {
            let step = {
                let (v1, v2) = m1202;
                let m1200 = v2;
                match m1200.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1201: Fn6 = v0.clone();
                            c1201.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f244(v0: Fn7, p1209: u64, p1210: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1208 = (p1209, p1210);
    {
        let mut m1208 = m1208;
        loop {
            let step = {
                let (v1, v2) = m1208;
                let m1205 = v1;
                if m1205 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1205 - 1;
                    let m1207 = {
                        let c1206: Fn7 = v0.clone();
                        c1206.apply(v2.clone())?
                    };
                    match m1207 {
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
                Ok(next) => { m1208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f279(v0)
}

pub fn f246(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f280(Fn13::F281, List::<(Bytes, ())>::nil(), v0)
}

pub fn f247(v0: Fn8, p1214: List<(Bytes, ())>, p1215: List<Bytes>) -> List<(Bytes, ())> {
    let m1213 = (p1214, p1215);
    {
        let mut m1213 = m1213;
        loop {
            let step = {
                let (v1, v2) = m1213;
                let m1211 = v2;
                match m1211.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1212: Fn8 = v0.clone();
                            c1212.apply(v1, v3)
                        }, v4))
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

pub fn f248(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f249(v0: List<Bytes>) -> List<Bytes> {
    let m1216 = v0;
    match m1216.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1218 = {
                    let a1217 = v1.clone();
                    length_bytes(a1217)
                };
                let a1219 = 0u64;
                if nat_eq(a1218, a1219) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f249(v2))
        }
    }
}

pub fn f250(v0: u32) -> Bytes {
    let a1222 = {
        let a1220 = v0;
        let a1221 = 8u32;
        fixed_u32::bit_and(a1220, a1221)
    };
    let a1223 = 0u32;
    if equal(a1222, a1223) {
        let a1226 = {
            let a1224 = v0;
            let a1225 = 4u32;
            fixed_u32::bit_and(a1224, a1225)
        };
        let a1227 = 0u32;
        if equal(a1226, a1227) {
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
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1238 = {
                    let a1236 = v0;
                    let a1237 = 1u32;
                    fixed_u32::bit_and(a1236, a1237)
                };
                let a1239 = 0u32;
                if equal(a1238, a1239) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1242 = {
                let a1240 = v0;
                let a1241 = 2u32;
                fixed_u32::bit_and(a1240, a1241)
            };
            let a1243 = 0u32;
            if equal(a1242, a1243) {
                let a1246 = {
                    let a1244 = v0;
                    let a1245 = 1u32;
                    fixed_u32::bit_and(a1244, a1245)
                };
                let a1247 = 0u32;
                if equal(a1246, a1247) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1250 = {
                    let a1248 = v0;
                    let a1249 = 1u32;
                    fixed_u32::bit_and(a1248, a1249)
                };
                let a1251 = 0u32;
                if equal(a1250, a1251) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1254 = {
            let a1252 = v0;
            let a1253 = 4u32;
            fixed_u32::bit_and(a1252, a1253)
        };
        let a1255 = 0u32;
        if equal(a1254, a1255) {
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
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1266 = {
                    let a1264 = v0;
                    let a1265 = 1u32;
                    fixed_u32::bit_and(a1264, a1265)
                };
                let a1267 = 0u32;
                if equal(a1266, a1267) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1270 = {
                let a1268 = v0;
                let a1269 = 2u32;
                fixed_u32::bit_and(a1268, a1269)
            };
            let a1271 = 0u32;
            if equal(a1270, a1271) {
                let a1274 = {
                    let a1272 = v0;
                    let a1273 = 1u32;
                    fixed_u32::bit_and(a1272, a1273)
                };
                let a1275 = 0u32;
                if equal(a1274, a1275) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1278 = {
                    let a1276 = v0;
                    let a1277 = 1u32;
                    fixed_u32::bit_and(a1276, a1277)
                };
                let a1279 = 0u32;
                if equal(a1278, a1279) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f251(v0: u32, v1: u32) -> u32 {
    let m1282 = {
        let a1280 = v0;
        let a1281 = v1;
        fixed_u32::shift_right(a1280, a1281)
    };
    match m1282 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f252(v0: Adt20, v1: Adt22) -> R<Adt20> {
    let v2: Adt23 = f282(Fn14::F283, 64u64, Adt23::C0(match v0.clone() {
        Adt20::C0(h1283, _, _, _, _, _, _, _) => {
            h1283
        }
    }, match v0.clone() {
        Adt20::C0(_, h1284, _, _, _, _, _, _) => {
            h1284
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h1285, _, _, _, _, _) => {
            h1285
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1286, _, _, _, _) => {
            h1286
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, h1287, _, _, _) => {
            h1287
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1288, _, _) => {
            h1288
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1289, _) => {
            h1289
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, _, h1290) => {
            h1290
        }
    }, v1, 0u64))?;
    Ok(Adt20::C0(f284(match v0.clone() {
        Adt20::C0(h1291, _, _, _, _, _, _, _) => {
            h1291
        }
    }, match v2.clone() {
        Adt23::C0(h1292, _, _, _, _, _, _, _, _, _) => {
            h1292
        }
    }), f284(match v0.clone() {
        Adt20::C0(_, h1293, _, _, _, _, _, _) => {
            h1293
        }
    }, match v2.clone() {
        Adt23::C0(_, h1294, _, _, _, _, _, _, _, _) => {
            h1294
        }
    }), f284(match v0.clone() {
        Adt20::C0(_, _, h1295, _, _, _, _, _) => {
            h1295
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1296, _, _, _, _, _, _, _) => {
            h1296
        }
    }), f284(match v0.clone() {
        Adt20::C0(_, _, _, h1297, _, _, _, _) => {
            h1297
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1298, _, _, _, _, _, _) => {
            h1298
        }
    }), f284(match v0.clone() {
        Adt20::C0(_, _, _, _, h1299, _, _, _) => {
            h1299
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1300, _, _, _, _, _) => {
            h1300
        }
    }), f284(match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1301, _, _) => {
            h1301
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1302, _, _, _, _) => {
            h1302
        }
    }), f284(match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1303, _) => {
            h1303
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1304, _, _, _) => {
            h1304
        }
    }), f284(match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h1305) => {
            h1305
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1306, _, _) => {
            h1306
        }
    })))
}

pub fn f253(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f285(v0.clone(), v1, v2, v3)?, f285(v0.clone(), v1, v2, {
        let a1307 = v3;
        let a1308 = 4u64;
        nat_add(a1307, a1308)?
    })?, f285(v0.clone(), v1, v2, {
        let a1309 = v3;
        let a1310 = 8u64;
        nat_add(a1309, a1310)?
    })?, f285(v0.clone(), v1, v2, {
        let a1311 = v3;
        let a1312 = 12u64;
        nat_add(a1311, a1312)?
    })?, f285(v0.clone(), v1, v2, {
        let a1313 = v3;
        let a1314 = 16u64;
        nat_add(a1313, a1314)?
    })?, f285(v0.clone(), v1, v2, {
        let a1315 = v3;
        let a1316 = 20u64;
        nat_add(a1315, a1316)?
    })?, f285(v0.clone(), v1, v2, {
        let a1317 = v3;
        let a1318 = 24u64;
        nat_add(a1317, a1318)?
    })?, f285(v0.clone(), v1, v2, {
        let a1319 = v3;
        let a1320 = 28u64;
        nat_add(a1319, a1320)?
    })?, f285(v0.clone(), v1, v2, {
        let a1321 = v3;
        let a1322 = 32u64;
        nat_add(a1321, a1322)?
    })?, f285(v0.clone(), v1, v2, {
        let a1323 = v3;
        let a1324 = 36u64;
        nat_add(a1323, a1324)?
    })?, f285(v0.clone(), v1, v2, {
        let a1325 = v3;
        let a1326 = 40u64;
        nat_add(a1325, a1326)?
    })?, f285(v0.clone(), v1, v2, {
        let a1327 = v3;
        let a1328 = 44u64;
        nat_add(a1327, a1328)?
    })?, f285(v0.clone(), v1, v2, {
        let a1329 = v3;
        let a1330 = 48u64;
        nat_add(a1329, a1330)?
    })?, f285(v0.clone(), v1, v2, {
        let a1331 = v3;
        let a1332 = 52u64;
        nat_add(a1331, a1332)?
    })?, f285(v0.clone(), v1, v2, {
        let a1333 = v3;
        let a1334 = 56u64;
        nat_add(a1333, a1334)?
    })?, f285(v0, v1, v2, {
        let a1335 = v3;
        let a1336 = 60u64;
        nat_add(a1335, a1336)?
    })?))
}

pub fn f254(v0: Bytes, v1: u64) -> Option<u64> {
    let a1337 = 0u64;
    let a1338 = v1;
    if nat_lt(a1337, a1338) {
        if f286(v0, {
            let a1339 = v1;
            let a1340 = 1u64;
            nat_sub(a1339, a1340)
        }) {
            Some({
                let a1341 = v1;
                let a1342 = 1u64;
                nat_sub(a1341, a1342)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f255(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1343 = v2;
    let a1344 = v0;
    if nat_lt(a1343, a1344) {
        if f286(v1, v2) {
            Ok(Some({
                let a1345 = v2;
                let a1346 = 1u64;
                nat_add(a1345, a1346)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f256(v0: Bytes, p1426: u64, p1427: u64, p1428: bool, p1429: bool, p1430: Bytes) -> R<Bytes> {
    let m1425 = (p1426, (p1427, (p1428, (p1429, p1430))));
    {
        let mut m1425 = m1425;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1425;
                let m1347 = v2;
                if m1347 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1347 - 1;
                    let m1350 = {
                        let a1348 = v0.clone();
                        let a1349 = v1;
                        index_bytes(a1348, a1349)
                    };
                    match m1350 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1395 = {
                                    let a1371 = {
                                        let a1359 = {
                                            let a1353 = {
                                                let a1351 = v8;
                                                let a1352 = 128u8;
                                                fixed_u8::bit_and(a1351, a1352)
                                            };
                                            let a1354 = 128u8;
                                            if equal(a1353, a1354) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1360 = {
                                            let a1357 = {
                                                let a1355 = v8;
                                                let a1356 = 64u8;
                                                fixed_u8::bit_and(a1355, a1356)
                                            };
                                            let a1358 = 64u8;
                                            if equal(a1357, a1358) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1359, a1360)?
                                    };
                                    let a1372 = {
                                        let a1369 = {
                                            let a1363 = {
                                                let a1361 = v8;
                                                let a1362 = 32u8;
                                                fixed_u8::bit_and(a1361, a1362)
                                            };
                                            let a1364 = 32u8;
                                            if equal(a1363, a1364) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1370 = {
                                            let a1367 = {
                                                let a1365 = v8;
                                                let a1366 = 16u8;
                                                fixed_u8::bit_and(a1365, a1366)
                                            };
                                            let a1368 = 16u8;
                                            if equal(a1367, a1368) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1369, a1370)?
                                    };
                                    nat_add(a1371, a1372)?
                                };
                                let a1396 = {
                                    let a1393 = {
                                        let a1381 = {
                                            let a1375 = {
                                                let a1373 = v8;
                                                let a1374 = 8u8;
                                                fixed_u8::bit_and(a1373, a1374)
                                            };
                                            let a1376 = 8u8;
                                            if equal(a1375, a1376) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1382 = {
                                            let a1379 = {
                                                let a1377 = v8;
                                                let a1378 = 4u8;
                                                fixed_u8::bit_and(a1377, a1378)
                                            };
                                            let a1380 = 4u8;
                                            if equal(a1379, a1380) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1381, a1382)?
                                    };
                                    let a1394 = {
                                        let a1391 = {
                                            let a1385 = {
                                                let a1383 = v8;
                                                let a1384 = 2u8;
                                                fixed_u8::bit_and(a1383, a1384)
                                            };
                                            let a1386 = 2u8;
                                            if equal(a1385, a1386) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1392 = {
                                            let a1389 = {
                                                let a1387 = v8;
                                                let a1388 = 1u8;
                                                fixed_u8::bit_and(a1387, a1388)
                                            };
                                            let a1390 = 1u8;
                                            if equal(a1389, a1390) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1391, a1392)?
                                    };
                                    nat_add(a1393, a1394)?
                                };
                                nat_add(a1395, a1396)?
                            };
                            let v10: Bytes = f45(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1397 = v1;
                                    let a1398 = 1u64;
                                    nat_add(a1397, a1398)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1399 = v9;
                                    let a1400 = 34u64;
                                    !nat_eq(a1399, a1400)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1401 = v9;
                                    let a1402 = 92u64;
                                    nat_eq(a1401, a1402)
                                }, {
                                    let a1403 = v5;
                                    let a1404 = v10;
                                    append_bytes(a1403, a1404)
                                })))))
                            } else {
                                let a1405 = v9;
                                let a1406 = 32u64;
                                if if if if nat_eq(a1405, a1406) {
                                    true
                                } else {
                                    let a1407 = v9;
                                    let a1408 = 9u64;
                                    nat_eq(a1407, a1408)
                                } {
                                    true
                                } else {
                                    let a1409 = v9;
                                    let a1410 = 10u64;
                                    nat_eq(a1409, a1410)
                                } {
                                    true
                                } else {
                                    let a1411 = v9;
                                    let a1412 = 13u64;
                                    nat_eq(a1411, a1412)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1413 = v1;
                                        let a1414 = 1u64;
                                        nat_add(a1413, a1414)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f27(f45(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1415 = v1;
                                            let a1416 = 4u64;
                                            nat_add(a1415, a1416)?
                                        }, (v6, (false, (false, {
                                            let a1417 = v5;
                                            let a1418 = f8(Str::lit("[]"));
                                            append_bytes(a1417, a1418)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1419 = v1;
                                            let a1420 = 1u64;
                                            nat_add(a1419, a1420)?
                                        }, (v6, ({
                                            let a1421 = v9;
                                            let a1422 = 34u64;
                                            nat_eq(a1421, a1422)
                                        }, (false, {
                                            let a1423 = v5;
                                            let a1424 = v10;
                                            append_bytes(a1423, a1424)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f257(v0: Bytes, v1: u64) -> R<u64> {
    let m1433 = {
        let a1431 = v0;
        let a1432 = v1;
        index_bytes(a1431, a1432)
    };
    match m1433 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1478 = {
                let a1454 = {
                    let a1442 = {
                        let a1436 = {
                            let a1434 = v3;
                            let a1435 = 128u8;
                            fixed_u8::bit_and(a1434, a1435)
                        };
                        let a1437 = 128u8;
                        if equal(a1436, a1437) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1443 = {
                        let a1440 = {
                            let a1438 = v3;
                            let a1439 = 64u8;
                            fixed_u8::bit_and(a1438, a1439)
                        };
                        let a1441 = 64u8;
                        if equal(a1440, a1441) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1442, a1443)?
                };
                let a1455 = {
                    let a1452 = {
                        let a1446 = {
                            let a1444 = v3;
                            let a1445 = 32u8;
                            fixed_u8::bit_and(a1444, a1445)
                        };
                        let a1447 = 32u8;
                        if equal(a1446, a1447) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1453 = {
                        let a1450 = {
                            let a1448 = v3;
                            let a1449 = 16u8;
                            fixed_u8::bit_and(a1448, a1449)
                        };
                        let a1451 = 16u8;
                        if equal(a1450, a1451) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1452, a1453)?
                };
                nat_add(a1454, a1455)?
            };
            let a1479 = {
                let a1476 = {
                    let a1464 = {
                        let a1458 = {
                            let a1456 = v3;
                            let a1457 = 8u8;
                            fixed_u8::bit_and(a1456, a1457)
                        };
                        let a1459 = 8u8;
                        if equal(a1458, a1459) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1465 = {
                        let a1462 = {
                            let a1460 = v3;
                            let a1461 = 4u8;
                            fixed_u8::bit_and(a1460, a1461)
                        };
                        let a1463 = 4u8;
                        if equal(a1462, a1463) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1464, a1465)?
                };
                let a1477 = {
                    let a1474 = {
                        let a1468 = {
                            let a1466 = v3;
                            let a1467 = 2u8;
                            fixed_u8::bit_and(a1466, a1467)
                        };
                        let a1469 = 2u8;
                        if equal(a1468, a1469) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1475 = {
                        let a1472 = {
                            let a1470 = v3;
                            let a1471 = 1u8;
                            fixed_u8::bit_and(a1470, a1471)
                        };
                        let a1473 = 1u8;
                        if equal(a1472, a1473) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1474, a1475)?
                };
                nat_add(a1476, a1477)?
            };
            nat_add(a1478, a1479)
        }
    }
}

pub fn f258(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f287(Fn15::F288(v0.clone()), {
        let a1481 = {
            let a1480 = v0;
            length_bytes(a1480)
        };
        let a1482 = 1u64;
        nat_add(a1481, a1482)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1486 = {
        let (_, h1483) = {
            let (_, h1484) = {
                let (h1485, _) = v2.clone();
                h1485
            };
            h1484
        };
        h1483
    };
    let a1487 = 1u64;
    if nat_eq(a1486, a1487) {
        Ok(Some(({
            let (h1488, _) = {
                let (h1489, _) = v2.clone();
                h1489
            };
            h1488
        }, {
            let (h1490, _) = {
                let (_, h1491) = {
                    let (h1492, _) = v2;
                    h1492
                };
                h1491
            };
            h1490
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f259(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1493 = f257(v0.clone(), v1)?;
        let a1494 = 45u64;
        nat_eq(a1493, a1494)
    };
    let v3: u64 = if v2 {
        let a1495 = v1;
        let a1496 = 1u64;
        nat_add(a1495, a1496)?
    } else {
        v1
    };
    let v4: u64 = f289(v0.clone(), v3)?;
    let a1499 = 19u64;
    let a1500 = {
        let a1497 = v4;
        let a1498 = v3;
        nat_sub(a1497, a1498)
    };
    if nat_lt(a1499, a1500) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1503 = f290(f45(v0, v3, {
            let a1501 = v4;
            let a1502 = v3;
            nat_sub(a1501, a1502)
        }))?;
        match m1503 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f291(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f260(v0: Bytes) -> Bytes {
    if f122(v0.clone(), Bytes::lit(&[0x0d])) {
        f45(v0.clone(), 0u64, {
            let a1505 = {
                let a1504 = v0;
                length_bytes(a1504)
            };
            let a1506 = 1u64;
            nat_sub(a1505, a1506)
        })
    } else {
        v0
    }
}

pub fn f261(v0: u64) -> bool {
    let a1507 = v0;
    let a1508 = 95u64;
    if nat_eq(a1507, a1508) {
        true
    } else {
        let a1509 = v0;
        let a1510 = 45u64;
        if nat_eq(a1509, a1510) {
            true
        } else {
            let a1511 = v0;
            let a1512 = 32u64;
            if nat_eq(a1511, a1512) {
                true
            } else {
                let a1513 = v0;
                let a1514 = 46u64;
                if nat_eq(a1513, a1514) {
                    true
                } else {
                    let a1515 = v0;
                    let a1516 = 47u64;
                    nat_eq(a1515, a1516)
                }
            }
        }
    }
}

pub fn f262(v0: u64) -> bool {
    let a1517 = 97u64;
    let a1518 = v0;
    if nat_le(a1517, a1518) {
        let a1519 = v0;
        let a1520 = 122u64;
        nat_le(a1519, a1520)
    } else {
        false
    }
}

pub fn f263(v0: u64) -> bool {
    let a1521 = 65u64;
    let a1522 = v0;
    if nat_le(a1521, a1522) {
        let a1523 = v0;
        let a1524 = 90u64;
        nat_le(a1523, a1524)
    } else {
        false
    }
}

pub fn f264(v0: u64) -> bool {
    let a1525 = 48u64;
    let a1526 = v0;
    if nat_le(a1525, a1526) {
        let a1527 = v0;
        let a1528 = 57u64;
        nat_le(a1527, a1528)
    } else {
        false
    }
}

pub fn f265(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m1529 = f104(v1.clone(), f8(Str::lit("enum")));
    match m1529 {
        None => {
            let m1531 = f295(v0.clone(), f222(f104(v1.clone(), f8(Str::lit("properties")))), f60(f108(f104(v1, f8(Str::lit("required"))))))?;
            match m1531 {
                Ok(v6) => {
                    Ok(f294(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1533 = {
                            let (h1532, _) = v6.clone();
                            h1532
                        };
                        match m1533.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f155({
                                    let (h1534, _) = v6;
                                    h1534
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1535) = v6;
                        h1535
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt21>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1530 = f292(f60(f172(v2)))?;
            match m1530 {
                Ok(v4) => {
                    Ok(f294(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f155(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt21>::nil()))
                }
                Err(v3) => {
                    Ok(f293(v3, v0))
                }
            }
        }
    }
}

pub fn f266(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1536 = v0;
    match m1536 {
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
        Adt14::C4(r1537) => {
            let v4 = (*r1537).clone();
            f266(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1538, r1539) => {
            let v2 = (*r1538).clone();
            let v3 = (*r1539).clone();
            List::cons((v1, v2), f266(v3))
        }
    }
}

pub fn f267(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1540 = v0;
    match m1540.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f149(v1)?, f267(v2)?))
        }
    }
}

pub fn f268(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1541 = v0;
    match m1541.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f268(v3, false))
            } else {
                f268(v3, true)
            }
        }
    }
}

pub fn f269(v0: u64) -> R<Bytes> {
    f296(f297(), 10u64, v0)
}

pub fn f270(v0: Fn9, p1546: u64, p1547: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1545 = (p1546, p1547);
    {
        let mut m1545 = m1545;
        loop {
            let step = {
                let (v1, v2) = m1545;
                let m1542 = v1;
                if m1542 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1542 - 1;
                    let m1544 = {
                        let c1543: Fn9 = v0.clone();
                        c1543.apply(v2.clone())?
                    };
                    match m1544 {
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
                Ok(next) => { m1545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f271(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f298(v0, v1)
}

pub fn f272(v0: Fn10, p1551: List<(Bytes, Bytes)>, p1552: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1550 = (p1551, p1552);
    {
        let mut m1550 = m1550;
        loop {
            let step = {
                let (v1, v2) = m1550;
                let m1548 = v2;
                match m1548.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1549: Fn10 = v0.clone();
                            c1549.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f299(v0, v1)
}

pub fn f274(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f300(v0)
}

pub fn f275(v0: Fn11, p1556: List<Bytes>, p1557: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1555 = (p1556, p1557);
    {
        let mut m1555 = m1555;
        loop {
            let step = {
                let (v1, v2) = m1555;
                let m1553 = v2;
                match m1553.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1554: Fn11 = v0.clone();
                            c1554.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f301(v1)?, v0))
}

pub fn f277(v0: Fn12, p1561: List<List<(Bytes, ())>>, p1562: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1560 = (p1561, p1562);
    {
        let mut m1560 = m1560;
        loop {
            let step = {
                let (v1, v2) = m1560;
                let m1558 = v2;
                match m1558.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1559: Fn12 = v0.clone();
                            c1559.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f279(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1564 = {
        let a1563 = v0.clone();
        length_list(a1563)
    };
    let a1565 = 1u64;
    if nat_le(a1564, a1565) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f302(v0)?))
    }
}

pub fn f280(v0: Fn13, p1569: List<(Bytes, ())>, p1570: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1568 = (p1569, p1570);
    {
        let mut m1568 = m1568;
        loop {
            let step = {
                let (v1, v2) = m1568;
                let m1566 = v2;
                match m1566.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1567: Fn13 = v0.clone();
                            c1567.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f282(v0: Fn14, p1574: u64, p1575: Adt23) -> R<Adt23> {
    let m1573 = (p1574, p1575);
    {
        let mut m1573 = m1573;
        loop {
            let step = {
                let (v1, v2) = m1573;
                let m1571 = v1;
                if m1571 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1571 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1572: Fn14 = v0.clone();
                        c1572.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: Adt23) -> R<Adt23> {
    f303(v0)
}

pub fn f284(v0: u32, v1: u32) -> u32 {
    let m1578 = {
        let a1576 = f304(v0);
        let a1577 = f304(v1);
        fixed_u64::checked_add(a1576, a1577)
    };
    match m1578 {
        None => {
            0u32
        }
        Some(v2) => {
            f305(v2)
        }
    }
}

pub fn f285(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1581 = {
        let a1579 = v3;
        let a1580 = 8u64;
        nat_add(a1579, a1580)?
    };
    let a1582 = v2;
    if nat_eq(a1581, a1582) {
        Ok(f306({
            let a1585 = {
                let a1583 = v1;
                let a1584 = 8u64;
                nat_mul(a1583, a1584)?
            };
            let a1586 = 4294967296u64;
            let a1587 = 0u64;
            nat_quot(a1585, a1586, a1587)
        }))
    } else {
        let a1590 = {
            let a1588 = v3;
            let a1589 = 4u64;
            nat_add(a1588, a1589)?
        };
        let a1591 = v2;
        if nat_eq(a1590, a1591) {
            Ok(f306({
                let a1594 = {
                    let a1592 = v1;
                    let a1593 = 8u64;
                    nat_mul(a1592, a1593)?
                };
                let a1595 = 4294967296u64;
                let a1596 = 0u64;
                nat_rem(a1594, a1595, a1596)
            }))
        } else {
            Ok(f307(f308(v0.clone(), v1, v3), f308(v0.clone(), v1, {
                let a1597 = v3;
                let a1598 = 1u64;
                nat_add(a1597, a1598)?
            }), f308(v0.clone(), v1, {
                let a1599 = v3;
                let a1600 = 2u64;
                nat_add(a1599, a1600)?
            }), f308(v0, v1, {
                let a1601 = v3;
                let a1602 = 3u64;
                nat_add(a1601, a1602)?
            })))
        }
    }
}

pub fn f286(v0: Bytes, v1: u64) -> bool {
    let m1603 = f232(v0, v1);
    match m1603 {
        None => {
            false
        }
        Some(v2) => {
            f309(v2)
        }
    }
}

pub fn f287(v0: Fn15, p1608: u64, p1609: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1607 = (p1608, p1609);
    {
        let mut m1607 = m1607;
        loop {
            let step = {
                let (v1, v2) = m1607;
                let m1604 = v1;
                if m1604 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1604 - 1;
                    let m1606 = {
                        let c1605: Fn15 = v0.clone();
                        c1605.apply(v2.clone())?
                    };
                    match m1606 {
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
                Ok(next) => { m1607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f310(v0, v1)
}

pub fn f289(v0: Bytes, v1: u64) -> R<u64> {
    let (h1610, _) = f180(Fn3::F311(v0.clone()), {
        let a1612 = {
            let a1611 = v0;
            length_bytes(a1611)
        };
        let a1613 = 1u64;
        nat_add(a1612, a1613)?
    }, v1)?;
    Ok(h1610)
}

pub fn f290(v0: Bytes) -> R<Option<u64>> {
    if f312(v0.clone())? {
        Ok(Some({
            let (_, h1614) = {
                let (h1615, _) = f313(Fn16::F314(v0.clone()), {
                    let a1616 = v0;
                    length_bytes(a1616)
                }, (0u64, 0u64))?;
                h1615
            };
            h1614
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f291(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1617 = 0u64;
        let a1618 = v1;
        if nat_lt(a1617, a1618) {
            let a1619 = v1;
            let a1620 = 9223372036854775808u64;
            nat_le(a1619, a1620)
        } else {
            false
        }
    } else {
        let a1621 = v1;
        let a1622 = 9223372036854775807u64;
        nat_le(a1621, a1622)
    }
}

pub fn f292(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1623 = v0;
    match m1623.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f315(v1.clone())?;
            if f316(v3.clone())? {
                let m1624 = f292(v2)?;
                match m1624 {
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

pub fn f293(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt21>), Bytes> {
    Err::<(Bytes, List<Adt21>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f294(v0: Bytes, v1: List<Adt21>) -> Result<(Bytes, List<Adt21>), Bytes> {
    Ok::<(Bytes, List<Adt21>), Bytes>((v0, v1))
}

pub fn f295(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt21>), Bytes>> {
    let m1625 = v1;
    match m1625.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt21>), Bytes>((List::<Bytes>::nil(), List::<Adt21>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f315({
                let (h1626, _) = v3.clone();
                h1626
            })?;
            if f316(v5.clone())? {
                let m1631 = f317({
                    let (_, h1627) = v3.clone();
                    h1627
                }, {
                    let a1629 = v0.clone();
                    let a1630 = f166({
                        let (h1628, _) = v3.clone();
                        h1628
                    })?;
                    append_bytes(a1629, a1630)
                })?;
                match m1631 {
                    Ok(v7) => {
                        let v8: Bytes = if f70({
                            let (h1632, _) = v3;
                            h1632
                        }, v2.clone()) {
                            let (h1633, _) = v7;
                            h1633
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1634, _) = v7;
                                h1634
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1635 = f295(v0, v4, v2)?;
                        match m1635 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt21>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1636, _) = v10;
                                    h1636
                                }), f220({
                                    let (_, h1637) = v7;
                                    h1637
                                }, {
                                    let (_, h1638) = v10;
                                    h1638
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
                    let (h1639, _) = v3;
                    h1639
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f296(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1640 = v2;
    let a1641 = 0u64;
    if nat_eq(a1640, a1641) {
        Ok(f318(v0, 0u64))
    } else {
        let (_, h1642) = {
            let (h1643, _) = f270(Fn9::F319(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1643
        };
        Ok(h1642)
    }
}

pub fn f297() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f298(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1647 = {
        let a1645 = v0.clone();
        let a1646 = {
            let (h1644, _) = v1.clone();
            h1644
        };
        index_bytes(a1645, a1646)
    };
    match m1647 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1649 = {
                    let (h1648, _) = v1.clone();
                    h1648
                };
                let a1650 = 1u64;
                nat_add(a1649, a1650)?
            }, {
                let a1653 = {
                    let (_, h1651) = v1;
                    h1651
                };
                let a1654 = f320(v0, {
                    let (h1652, _) = v1;
                    h1652
                }, v2)?;
                append_bytes(a1653, a1654)
            })))
        }
    }
}

pub fn f299(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1655 = v0.clone();
    match m1655.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1658 = {
                let (h1656, _) = v2;
                h1656
            };
            let a1659 = {
                let (h1657, _) = v1.clone();
                h1657
            };
            if equal(a1658, a1659) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f300(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f321(f322(Fn17::F323, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1664 = {
        let (h1660, _) = f324(Fn18::F325, {
            let a1662 = {
                let a1661 = v0;
                length_list(a1661)
            };
            let a1663 = 1u64;
            nat_add(a1662, a1663)?
        }, v1)?;
        h1660
    };
    match m1664.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f301(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1669 = {
        let a1666 = f225({
            let (h1665, _) = v0;
            h1665
        })?;
        let a1667 = Bytes::lit(&[0x3a]);
        append_bytes(a1666, a1667)
    };
    let a1670 = {
        let (_, h1668) = v0;
        h1668
    };
    Ok(append_bytes(a1669, a1670))
}

pub fn f302(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f241({
        let (_, h1671) = {
            let (h1672, _) = f326(Fn19::F327, {
                let a1674 = {
                    let a1673 = v0.clone();
                    length_list(a1673)
                };
                let a1675 = 1u64;
                nat_add(a1674, a1675)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1672
        };
        h1671
    }))
}

pub fn f303(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f328(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1676) => {
            h1676
        }
    });
    let v2: u32 = f284(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1677, _, _) => {
            h1677
        }
    }, f284(f329(match v0.clone() {
        Adt23::C0(_, _, _, _, h1678, _, _, _, _, _) => {
            h1678
        }
    }), f284(f330(match v0.clone() {
        Adt23::C0(_, _, _, _, h1679, _, _, _, _, _) => {
            h1679
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1680, _, _, _, _) => {
            h1680
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1681, _, _, _) => {
            h1681
        }
    }), f284(v1, {
        let m1684 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1683, _) => {
                h1683
            }
        };
        match m1684 {
            Adt22::C0(h1682, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1682
            }
        }
    }))));
    let v3: u32 = f284(f331(match v0.clone() {
        Adt23::C0(h1685, _, _, _, _, _, _, _, _, _) => {
            h1685
        }
    }), f332(match v0.clone() {
        Adt23::C0(h1686, _, _, _, _, _, _, _, _, _) => {
            h1686
        }
    }, match v0.clone() {
        Adt23::C0(_, h1687, _, _, _, _, _, _, _, _) => {
            h1687
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1688, _, _, _, _, _, _, _) => {
            h1688
        }
    }));
    Ok(Adt23::C0(f284(v2, v3), match v0.clone() {
        Adt23::C0(h1689, _, _, _, _, _, _, _, _, _) => {
            h1689
        }
    }, match v0.clone() {
        Adt23::C0(_, h1690, _, _, _, _, _, _, _, _) => {
            h1690
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1691, _, _, _, _, _, _, _) => {
            h1691
        }
    }, f284(match v0.clone() {
        Adt23::C0(_, _, _, h1692, _, _, _, _, _, _) => {
            h1692
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1693, _, _, _, _, _) => {
            h1693
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1694, _, _, _, _) => {
            h1694
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1695, _, _, _) => {
            h1695
        }
    }, f333(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1696, _) => {
            h1696
        }
    }), {
        let a1698 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1697) => {
                h1697
            }
        };
        let a1699 = 1u64;
        nat_add(a1698, a1699)?
    }))
}

pub fn f304(v0: u32) -> u64 {
    let m1701 = {
        let a1700 = v0;
        fixed_u64::convert(i128::from(a1700))
    };
    match m1701 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f305(v0: u64) -> u32 {
    let m1705 = {
        let a1704 = {
            let a1702 = v0;
            let a1703 = 4294967295u64;
            fixed_u64::bit_and(a1702, a1703)
        };
        fixed_u32::convert(i128::from(a1704))
    };
    match m1705 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f306(v0: u64) -> u32 {
    let (_, h1706) = {
        let (_, h1707) = f334(Fn20::F335, 32u64, (v0, (1u32, 0u32)));
        h1707
    };
    h1706
}

pub fn f307(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1712 = f336(v0, 24u32);
    let a1713 = {
        let a1710 = f336(v1, 16u32);
        let a1711 = {
            let a1708 = f336(v2, 8u32);
            let a1709 = v3;
            fixed_u32::bit_or(a1708, a1709)
        };
        fixed_u32::bit_or(a1710, a1711)
    };
    fixed_u32::bit_or(a1712, a1713)
}

pub fn f308(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1714 = v2;
    let a1715 = v1;
    if nat_lt(a1714, a1715) {
        let m1718 = {
            let a1716 = v0;
            let a1717 = v2;
            index_bytes(a1716, a1717)
        };
        match m1718 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1720 = {
                    let a1719 = v3;
                    fixed_u32::convert(i128::from(a1719))
                };
                match m1720 {
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
        let a1721 = v2;
        let a1722 = v1;
        if nat_eq(a1721, a1722) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f309(v0: u8) -> bool {
    let a1723 = v0;
    let a1724 = 32u8;
    if if if equal(a1723, a1724) {
        true
    } else {
        let a1725 = v0;
        let a1726 = 9u8;
        equal(a1725, a1726)
    } {
        true
    } else {
        let a1727 = v0;
        let a1728 = 10u8;
        equal(a1727, a1728)
    } {
        true
    } else {
        let a1729 = v0;
        let a1730 = 12u8;
        if equal(a1729, a1730) {
            true
        } else {
            let a1731 = v0;
            let a1732 = 13u8;
            equal(a1731, a1732)
        }
    }
}

pub fn f310(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1733, _) = v1.clone();
        h1733
    };
    let v3: Bytes = {
        let (h1734, _) = {
            let (_, h1735) = v1.clone();
            h1735
        };
        h1734
    };
    let a1738 = {
        let (_, h1736) = {
            let (_, h1737) = v1;
            h1737
        };
        h1736
    };
    let a1739 = 0u64;
    if nat_eq(a1738, a1739) {
        let v4: u64 = f257(v0.clone(), v2)?;
        let a1740 = v4;
        let a1741 = 34u64;
        if nat_eq(a1740, a1741) {
            Ok(Some(({
                let a1742 = v2;
                let a1743 = 1u64;
                nat_add(a1742, a1743)?
            }, (v3, 1u64))))
        } else {
            let a1744 = v4;
            let a1745 = 92u64;
            if nat_eq(a1744, a1745) {
                f337(v0, v2, v3)
            } else {
                let a1746 = v4;
                let a1747 = 32u64;
                if if nat_lt(a1746, a1747) {
                    true
                } else {
                    let a1748 = v4;
                    let a1749 = 256u64;
                    nat_eq(a1748, a1749)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1750 = v2;
                        let a1751 = 1u64;
                        nat_add(a1750, a1751)?
                    }, ({
                        let a1752 = v3;
                        let a1753 = f45(v0, v2, 1u64);
                        append_bytes(a1752, a1753)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f338(v0, v1)
}

pub fn f312(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1754 = v0.clone();
        length_bytes(a1754)
    };
    let a1755 = 0u64;
    let a1756 = v1;
    if nat_lt(a1755, a1756) {
        let a1757 = f339(v0.clone())?;
        let a1758 = v1;
        if nat_eq(a1757, a1758) {
            let a1759 = 1u64;
            let a1760 = v1;
            if nat_lt(a1759, a1760) {
                let m1763 = {
                    let a1761 = v0;
                    let a1762 = 0u64;
                    index_bytes(a1761, a1762)
                };
                match m1763 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1764 = v2;
                        let a1765 = 48u8;
                        Ok(!equal(a1764, a1765))
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

pub fn f313(v0: Fn16, p1770: u64, p1771: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1769 = (p1770, p1771);
    {
        let mut m1769 = m1769;
        loop {
            let step = {
                let (v1, v2) = m1769;
                let m1766 = v1;
                if m1766 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1766 - 1;
                    let m1768 = {
                        let c1767: Fn16 = v0.clone();
                        c1767.apply(v2)?
                    };
                    match m1768 {
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
                Ok(next) => { m1769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f314(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f340(v0, v1)
}

pub fn f315(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f218(v0.clone(), 0u64, {
        let a1772 = v0;
        length_bytes(a1772)
    }, false, Bytes::lit(&[]))?;
    let m1775 = {
        let a1773 = v1.clone();
        let a1774 = 0u64;
        index_bytes(a1773, a1774)
    };
    match m1775 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1820 = {
                    let a1796 = {
                        let a1784 = {
                            let a1778 = {
                                let a1776 = v3;
                                let a1777 = 128u8;
                                fixed_u8::bit_and(a1776, a1777)
                            };
                            let a1779 = 128u8;
                            if equal(a1778, a1779) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1785 = {
                            let a1782 = {
                                let a1780 = v3;
                                let a1781 = 64u8;
                                fixed_u8::bit_and(a1780, a1781)
                            };
                            let a1783 = 64u8;
                            if equal(a1782, a1783) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1784, a1785)?
                    };
                    let a1797 = {
                        let a1794 = {
                            let a1788 = {
                                let a1786 = v3;
                                let a1787 = 32u8;
                                fixed_u8::bit_and(a1786, a1787)
                            };
                            let a1789 = 32u8;
                            if equal(a1788, a1789) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1795 = {
                            let a1792 = {
                                let a1790 = v3;
                                let a1791 = 16u8;
                                fixed_u8::bit_and(a1790, a1791)
                            };
                            let a1793 = 16u8;
                            if equal(a1792, a1793) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1794, a1795)?
                    };
                    nat_add(a1796, a1797)?
                };
                let a1821 = {
                    let a1818 = {
                        let a1806 = {
                            let a1800 = {
                                let a1798 = v3;
                                let a1799 = 8u8;
                                fixed_u8::bit_and(a1798, a1799)
                            };
                            let a1801 = 8u8;
                            if equal(a1800, a1801) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1807 = {
                            let a1804 = {
                                let a1802 = v3;
                                let a1803 = 4u8;
                                fixed_u8::bit_and(a1802, a1803)
                            };
                            let a1805 = 4u8;
                            if equal(a1804, a1805) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1806, a1807)?
                    };
                    let a1819 = {
                        let a1816 = {
                            let a1810 = {
                                let a1808 = v3;
                                let a1809 = 2u8;
                                fixed_u8::bit_and(a1808, a1809)
                            };
                            let a1811 = 2u8;
                            if equal(a1810, a1811) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1817 = {
                            let a1814 = {
                                let a1812 = v3;
                                let a1813 = 1u8;
                                fixed_u8::bit_and(a1812, a1813)
                            };
                            let a1815 = 1u8;
                            if equal(a1814, a1815) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1816, a1817)?
                    };
                    nat_add(a1818, a1819)?
                };
                nat_add(a1820, a1821)?
            };
            if f263(v4) {
                let a1828 = f45({
                    let a1822 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1822)
                }, {
                    let a1823 = v4;
                    let a1824 = 65u64;
                    nat_sub(a1823, a1824)
                }, 1u64);
                let a1829 = f45(v1.clone(), 1u64, {
                    let a1826 = {
                        let a1825 = v1;
                        length_bytes(a1825)
                    };
                    let a1827 = 1u64;
                    nat_sub(a1826, a1827)
                });
                Ok(append_bytes(a1828, a1829))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f316(v0: Bytes) -> R<bool> {
    let m1832 = {
        let a1830 = v0;
        let a1831 = 0u64;
        index_bytes(a1830, a1831)
    };
    match m1832 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f262({
                let v2: u8 = v1;
                let a1877 = {
                    let a1853 = {
                        let a1841 = {
                            let a1835 = {
                                let a1833 = v2;
                                let a1834 = 128u8;
                                fixed_u8::bit_and(a1833, a1834)
                            };
                            let a1836 = 128u8;
                            if equal(a1835, a1836) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1842 = {
                            let a1839 = {
                                let a1837 = v2;
                                let a1838 = 64u8;
                                fixed_u8::bit_and(a1837, a1838)
                            };
                            let a1840 = 64u8;
                            if equal(a1839, a1840) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1841, a1842)?
                    };
                    let a1854 = {
                        let a1851 = {
                            let a1845 = {
                                let a1843 = v2;
                                let a1844 = 32u8;
                                fixed_u8::bit_and(a1843, a1844)
                            };
                            let a1846 = 32u8;
                            if equal(a1845, a1846) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1852 = {
                            let a1849 = {
                                let a1847 = v2;
                                let a1848 = 16u8;
                                fixed_u8::bit_and(a1847, a1848)
                            };
                            let a1850 = 16u8;
                            if equal(a1849, a1850) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1851, a1852)?
                    };
                    nat_add(a1853, a1854)?
                };
                let a1878 = {
                    let a1875 = {
                        let a1863 = {
                            let a1857 = {
                                let a1855 = v2;
                                let a1856 = 8u8;
                                fixed_u8::bit_and(a1855, a1856)
                            };
                            let a1858 = 8u8;
                            if equal(a1857, a1858) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1864 = {
                            let a1861 = {
                                let a1859 = v2;
                                let a1860 = 4u8;
                                fixed_u8::bit_and(a1859, a1860)
                            };
                            let a1862 = 4u8;
                            if equal(a1861, a1862) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1863, a1864)?
                    };
                    let a1876 = {
                        let a1873 = {
                            let a1867 = {
                                let a1865 = v2;
                                let a1866 = 2u8;
                                fixed_u8::bit_and(a1865, a1866)
                            };
                            let a1868 = 2u8;
                            if equal(a1867, a1868) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1874 = {
                            let a1871 = {
                                let a1869 = v2;
                                let a1870 = 1u8;
                                fixed_u8::bit_and(a1869, a1870)
                            };
                            let a1872 = 1u8;
                            if equal(a1871, a1872) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1873, a1874)?
                    };
                    nat_add(a1875, a1876)?
                };
                nat_add(a1877, a1878)?
            }) {
                Ok(true)
            } else {
                Ok(f263({
                    let v3: u8 = v1;
                    let a1923 = {
                        let a1899 = {
                            let a1887 = {
                                let a1881 = {
                                    let a1879 = v3;
                                    let a1880 = 128u8;
                                    fixed_u8::bit_and(a1879, a1880)
                                };
                                let a1882 = 128u8;
                                if equal(a1881, a1882) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1888 = {
                                let a1885 = {
                                    let a1883 = v3;
                                    let a1884 = 64u8;
                                    fixed_u8::bit_and(a1883, a1884)
                                };
                                let a1886 = 64u8;
                                if equal(a1885, a1886) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1887, a1888)?
                        };
                        let a1900 = {
                            let a1897 = {
                                let a1891 = {
                                    let a1889 = v3;
                                    let a1890 = 32u8;
                                    fixed_u8::bit_and(a1889, a1890)
                                };
                                let a1892 = 32u8;
                                if equal(a1891, a1892) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1898 = {
                                let a1895 = {
                                    let a1893 = v3;
                                    let a1894 = 16u8;
                                    fixed_u8::bit_and(a1893, a1894)
                                };
                                let a1896 = 16u8;
                                if equal(a1895, a1896) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1897, a1898)?
                        };
                        nat_add(a1899, a1900)?
                    };
                    let a1924 = {
                        let a1921 = {
                            let a1909 = {
                                let a1903 = {
                                    let a1901 = v3;
                                    let a1902 = 8u8;
                                    fixed_u8::bit_and(a1901, a1902)
                                };
                                let a1904 = 8u8;
                                if equal(a1903, a1904) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1910 = {
                                let a1907 = {
                                    let a1905 = v3;
                                    let a1906 = 4u8;
                                    fixed_u8::bit_and(a1905, a1906)
                                };
                                let a1908 = 4u8;
                                if equal(a1907, a1908) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1909, a1910)?
                        };
                        let a1922 = {
                            let a1919 = {
                                let a1913 = {
                                    let a1911 = v3;
                                    let a1912 = 2u8;
                                    fixed_u8::bit_and(a1911, a1912)
                                };
                                let a1914 = 2u8;
                                if equal(a1913, a1914) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1920 = {
                                let a1917 = {
                                    let a1915 = v3;
                                    let a1916 = 1u8;
                                    fixed_u8::bit_and(a1915, a1916)
                                };
                                let a1918 = 1u8;
                                if equal(a1917, a1918) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1919, a1920)?
                        };
                        nat_add(a1921, a1922)?
                    };
                    nat_add(a1923, a1924)?
                }))
            }
        }
    }
}

pub fn f317(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m1925 = f341(v0.clone());
    match m1925 {
        None => {
            if f27(f107(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1926 = f104(v0, f8(Str::lit("items")));
                match m1926 {
                    None => {
                        Ok(f293(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f342(f8(Str::lit("List")), f343(v3, {
                            let a1927 = v1;
                            let a1928 = f8(Str::lit("Item"));
                            append_bytes(a1927, a1928)
                        })?))
                    }
                }
            } else {
                f343(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f342(f8(Str::lit("Option")), f343(v2, v1)?))
        }
    }
}

pub fn f318(v0: Bytes, v1: u64) -> Bytes {
    let m1932 = {
        let a1929 = v0;
        let a1930 = v1;
        let a1931 = 1u64;
        slice_bytes(a1929, a1930, a1931)
    };
    match m1932 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f319(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f344(v1, v0, v2)
}

pub fn f320(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1977 = {
            let a1953 = {
                let a1941 = {
                    let a1935 = {
                        let a1933 = v3;
                        let a1934 = 128u8;
                        fixed_u8::bit_and(a1933, a1934)
                    };
                    let a1936 = 128u8;
                    if equal(a1935, a1936) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1942 = {
                    let a1939 = {
                        let a1937 = v3;
                        let a1938 = 64u8;
                        fixed_u8::bit_and(a1937, a1938)
                    };
                    let a1940 = 64u8;
                    if equal(a1939, a1940) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1941, a1942)?
            };
            let a1954 = {
                let a1951 = {
                    let a1945 = {
                        let a1943 = v3;
                        let a1944 = 32u8;
                        fixed_u8::bit_and(a1943, a1944)
                    };
                    let a1946 = 32u8;
                    if equal(a1945, a1946) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1952 = {
                    let a1949 = {
                        let a1947 = v3;
                        let a1948 = 16u8;
                        fixed_u8::bit_and(a1947, a1948)
                    };
                    let a1950 = 16u8;
                    if equal(a1949, a1950) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1951, a1952)?
            };
            nat_add(a1953, a1954)?
        };
        let a1978 = {
            let a1975 = {
                let a1963 = {
                    let a1957 = {
                        let a1955 = v3;
                        let a1956 = 8u8;
                        fixed_u8::bit_and(a1955, a1956)
                    };
                    let a1958 = 8u8;
                    if equal(a1957, a1958) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1964 = {
                    let a1961 = {
                        let a1959 = v3;
                        let a1960 = 4u8;
                        fixed_u8::bit_and(a1959, a1960)
                    };
                    let a1962 = 4u8;
                    if equal(a1961, a1962) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1963, a1964)?
            };
            let a1976 = {
                let a1973 = {
                    let a1967 = {
                        let a1965 = v3;
                        let a1966 = 2u8;
                        fixed_u8::bit_and(a1965, a1966)
                    };
                    let a1968 = 2u8;
                    if equal(a1967, a1968) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1974 = {
                    let a1971 = {
                        let a1969 = v3;
                        let a1970 = 1u8;
                        fixed_u8::bit_and(a1969, a1970)
                    };
                    let a1972 = 1u8;
                    if equal(a1971, a1972) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1973, a1974)?
            };
            nat_add(a1975, a1976)?
        };
        nat_add(a1977, a1978)?
    };
    let a1979 = v4;
    let a1980 = 34u64;
    if nat_eq(a1979, a1980) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1981 = v4;
        let a1982 = 92u64;
        if nat_eq(a1981, a1982) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1983 = v4;
            let a1984 = 8u64;
            if nat_eq(a1983, a1984) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1985 = v4;
                let a1986 = 12u64;
                if nat_eq(a1985, a1986) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1987 = v4;
                    let a1988 = 10u64;
                    if nat_eq(a1987, a1988) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1989 = v4;
                        let a1990 = 13u64;
                        if nat_eq(a1989, a1990) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1991 = v4;
                            let a1992 = 9u64;
                            if nat_eq(a1991, a1992) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1993 = v4;
                                let a1994 = 32u64;
                                if nat_lt(a1993, a1994) {
                                    let a1995 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1996 = f345(v2)?;
                                    Ok(append_bytes(a1995, a1996))
                                } else {
                                    Ok(f45(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f321(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f346(Fn21::F347, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f322(v0: Fn17, p2000: List<List<(Bytes, Bytes)>>, p2001: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m1999 = (p2000, p2001);
    {
        let mut m1999 = m1999;
        loop {
            let step = {
                let (v1, v2) = m1999;
                let m1997 = v2;
                match m1997.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c1998: Fn17 = v0.clone();
                            c1998.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1999 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f323(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f324(v0: Fn18, p2006: u64, p2007: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2005 = (p2006, p2007);
    {
        let mut m2005 = m2005;
        loop {
            let step = {
                let (v1, v2) = m2005;
                let m2002 = v1;
                if m2002 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2002 - 1;
                    let m2004 = {
                        let c2003: Fn18 = v0.clone();
                        c2003.apply(v2.clone())?
                    };
                    match m2004 {
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
                Ok(next) => { m2005 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f325(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f348(v0)
}

pub fn f326(v0: Fn19, p2012: u64, p2013: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2011 = (p2012, p2013);
    {
        let mut m2011 = m2011;
        loop {
            let step = {
                let (v1, v2) = m2011;
                let m2008 = v1;
                if m2008 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2008 - 1;
                    let m2010 = {
                        let c2009: Fn19 = v0.clone();
                        c2009.apply(v2.clone())?
                    };
                    match m2010 {
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
                Ok(next) => { m2011 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f349(v0)
}

pub fn f328(v0: u64) -> u32 {
    let a2014 = v0;
    let a2015 = 32u64;
    if nat_lt(a2014, a2015) {
        let a2016 = v0;
        let a2017 = 16u64;
        if nat_lt(a2016, a2017) {
            let a2018 = v0;
            let a2019 = 8u64;
            if nat_lt(a2018, a2019) {
                let a2020 = v0;
                let a2021 = 4u64;
                if nat_lt(a2020, a2021) {
                    let a2022 = v0;
                    let a2023 = 2u64;
                    if nat_lt(a2022, a2023) {
                        let a2024 = v0;
                        let a2025 = 1u64;
                        if nat_lt(a2024, a2025) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2026 = v0;
                        let a2027 = 3u64;
                        if nat_lt(a2026, a2027) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2028 = v0;
                    let a2029 = 6u64;
                    if nat_lt(a2028, a2029) {
                        let a2030 = v0;
                        let a2031 = 5u64;
                        if nat_lt(a2030, a2031) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2032 = v0;
                        let a2033 = 7u64;
                        if nat_lt(a2032, a2033) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2034 = v0;
                let a2035 = 12u64;
                if nat_lt(a2034, a2035) {
                    let a2036 = v0;
                    let a2037 = 10u64;
                    if nat_lt(a2036, a2037) {
                        let a2038 = v0;
                        let a2039 = 9u64;
                        if nat_lt(a2038, a2039) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2040 = v0;
                        let a2041 = 11u64;
                        if nat_lt(a2040, a2041) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2042 = v0;
                    let a2043 = 14u64;
                    if nat_lt(a2042, a2043) {
                        let a2044 = v0;
                        let a2045 = 13u64;
                        if nat_lt(a2044, a2045) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2046 = v0;
                        let a2047 = 15u64;
                        if nat_lt(a2046, a2047) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2048 = v0;
            let a2049 = 24u64;
            if nat_lt(a2048, a2049) {
                let a2050 = v0;
                let a2051 = 20u64;
                if nat_lt(a2050, a2051) {
                    let a2052 = v0;
                    let a2053 = 18u64;
                    if nat_lt(a2052, a2053) {
                        let a2054 = v0;
                        let a2055 = 17u64;
                        if nat_lt(a2054, a2055) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2056 = v0;
                        let a2057 = 19u64;
                        if nat_lt(a2056, a2057) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2058 = v0;
                    let a2059 = 22u64;
                    if nat_lt(a2058, a2059) {
                        let a2060 = v0;
                        let a2061 = 21u64;
                        if nat_lt(a2060, a2061) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2062 = v0;
                        let a2063 = 23u64;
                        if nat_lt(a2062, a2063) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2064 = v0;
                let a2065 = 28u64;
                if nat_lt(a2064, a2065) {
                    let a2066 = v0;
                    let a2067 = 26u64;
                    if nat_lt(a2066, a2067) {
                        let a2068 = v0;
                        let a2069 = 25u64;
                        if nat_lt(a2068, a2069) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2070 = v0;
                        let a2071 = 27u64;
                        if nat_lt(a2070, a2071) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2072 = v0;
                    let a2073 = 30u64;
                    if nat_lt(a2072, a2073) {
                        let a2074 = v0;
                        let a2075 = 29u64;
                        if nat_lt(a2074, a2075) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2076 = v0;
                        let a2077 = 31u64;
                        if nat_lt(a2076, a2077) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2078 = v0;
        let a2079 = 48u64;
        if nat_lt(a2078, a2079) {
            let a2080 = v0;
            let a2081 = 40u64;
            if nat_lt(a2080, a2081) {
                let a2082 = v0;
                let a2083 = 36u64;
                if nat_lt(a2082, a2083) {
                    let a2084 = v0;
                    let a2085 = 34u64;
                    if nat_lt(a2084, a2085) {
                        let a2086 = v0;
                        let a2087 = 33u64;
                        if nat_lt(a2086, a2087) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2088 = v0;
                        let a2089 = 35u64;
                        if nat_lt(a2088, a2089) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2090 = v0;
                    let a2091 = 38u64;
                    if nat_lt(a2090, a2091) {
                        let a2092 = v0;
                        let a2093 = 37u64;
                        if nat_lt(a2092, a2093) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2094 = v0;
                        let a2095 = 39u64;
                        if nat_lt(a2094, a2095) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2096 = v0;
                let a2097 = 44u64;
                if nat_lt(a2096, a2097) {
                    let a2098 = v0;
                    let a2099 = 42u64;
                    if nat_lt(a2098, a2099) {
                        let a2100 = v0;
                        let a2101 = 41u64;
                        if nat_lt(a2100, a2101) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2102 = v0;
                        let a2103 = 43u64;
                        if nat_lt(a2102, a2103) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2104 = v0;
                    let a2105 = 46u64;
                    if nat_lt(a2104, a2105) {
                        let a2106 = v0;
                        let a2107 = 45u64;
                        if nat_lt(a2106, a2107) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2108 = v0;
                        let a2109 = 47u64;
                        if nat_lt(a2108, a2109) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2110 = v0;
            let a2111 = 56u64;
            if nat_lt(a2110, a2111) {
                let a2112 = v0;
                let a2113 = 52u64;
                if nat_lt(a2112, a2113) {
                    let a2114 = v0;
                    let a2115 = 50u64;
                    if nat_lt(a2114, a2115) {
                        let a2116 = v0;
                        let a2117 = 49u64;
                        if nat_lt(a2116, a2117) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2118 = v0;
                        let a2119 = 51u64;
                        if nat_lt(a2118, a2119) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2120 = v0;
                    let a2121 = 54u64;
                    if nat_lt(a2120, a2121) {
                        let a2122 = v0;
                        let a2123 = 53u64;
                        if nat_lt(a2122, a2123) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2124 = v0;
                        let a2125 = 55u64;
                        if nat_lt(a2124, a2125) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2126 = v0;
                let a2127 = 60u64;
                if nat_lt(a2126, a2127) {
                    let a2128 = v0;
                    let a2129 = 58u64;
                    if nat_lt(a2128, a2129) {
                        let a2130 = v0;
                        let a2131 = 57u64;
                        if nat_lt(a2130, a2131) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2132 = v0;
                        let a2133 = 59u64;
                        if nat_lt(a2132, a2133) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2134 = v0;
                    let a2135 = 62u64;
                    if nat_lt(a2134, a2135) {
                        let a2136 = v0;
                        let a2137 = 61u64;
                        if nat_lt(a2136, a2137) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2138 = v0;
                        let a2139 = 63u64;
                        if nat_lt(a2138, a2139) {
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

pub fn f329(v0: u32) -> u32 {
    let a2142 = {
        let a2140 = f350(v0, 6u32, 26u32);
        let a2141 = f350(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2140, a2141)
    };
    let a2143 = f350(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2142, a2143)
}

pub fn f330(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2149 = {
        let a2144 = v0;
        let a2145 = v1;
        fixed_u32::bit_and(a2144, a2145)
    };
    let a2150 = {
        let a2147 = {
            let a2146 = v0;
            fixed_u32::bit_not(a2146)
        };
        let a2148 = v2;
        fixed_u32::bit_and(a2147, a2148)
    };
    fixed_u32::bit_xor(a2149, a2150)
}

pub fn f331(v0: u32) -> u32 {
    let a2153 = {
        let a2151 = f350(v0, 2u32, 30u32);
        let a2152 = f350(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2151, a2152)
    };
    let a2154 = f350(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2153, a2154)
}

pub fn f332(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2163 = {
        let a2159 = {
            let a2155 = v0;
            let a2156 = v1;
            fixed_u32::bit_and(a2155, a2156)
        };
        let a2160 = {
            let a2157 = v0;
            let a2158 = v2;
            fixed_u32::bit_and(a2157, a2158)
        };
        fixed_u32::bit_xor(a2159, a2160)
    };
    let a2164 = {
        let a2161 = v1;
        let a2162 = v2;
        fixed_u32::bit_and(a2161, a2162)
    };
    fixed_u32::bit_xor(a2163, a2164)
}

pub fn f333(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2165, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2165
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2166, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2166
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2167, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2167
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2168, _, _, _, _, _, _, _, _, _, _, _) => {
            h2168
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2169, _, _, _, _, _, _, _, _, _, _) => {
            h2169
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2170, _, _, _, _, _, _, _, _, _) => {
            h2170
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2171, _, _, _, _, _, _, _, _) => {
            h2171
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2172, _, _, _, _, _, _, _) => {
            h2172
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2173, _, _, _, _, _, _) => {
            h2173
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2174, _, _, _, _, _) => {
            h2174
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2175, _, _, _, _) => {
            h2175
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2176, _, _, _) => {
            h2176
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2177, _, _) => {
            h2177
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2178, _) => {
            h2178
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2179) => {
            h2179
        }
    }, f284(f351(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2180, _) => {
            h2180
        }
    }), f284(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2181, _, _, _, _, _, _) => {
            h2181
        }
    }, f284(f352(match v0.clone() {
        Adt22::C0(_, h2182, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2182
        }
    }), match v0 {
        Adt22::C0(h2183, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2183
        }
    }))))
}

pub fn f334(v0: Fn20, p2187: u64, p2188: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2186 = (p2187, p2188);
    {
        let mut m2186 = m2186;
        loop {
            let step = {
                let (v1, v2) = m2186;
                let m2184 = v1;
                if m2184 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2184 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2185: Fn20 = v0.clone();
                        c2185.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2189, _) = v0;
        h2189
    };
    let v2: u32 = {
        let (h2190, _) = {
            let (_, h2191) = v0;
            h2191
        };
        h2190
    };
    let v3: u32 = {
        let (_, h2192) = {
            let (_, h2193) = v0;
            h2193
        };
        h2192
    };
    ({
        let a2194 = v1;
        let a2195 = 2u64;
        let a2196 = 0u64;
        nat_quot(a2194, a2195, a2196)
    }, (f336(v2, 1u32), {
        let a2200 = {
            let a2197 = v1;
            let a2198 = 2u64;
            let a2199 = 0u64;
            nat_rem(a2197, a2198, a2199)
        };
        let a2201 = 1u64;
        if nat_eq(a2200, a2201) {
            let a2202 = v3;
            let a2203 = v2;
            fixed_u32::bit_or(a2202, a2203)
        } else {
            v3
        }
    }))
}

pub fn f336(v0: u32, v1: u32) -> u32 {
    let m2206 = {
        let a2204 = v0;
        let a2205 = v1;
        fixed_u32::shift_left(a2204, a2205)
    };
    match m2206 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f337(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f257(v0.clone(), {
        let a2207 = v1;
        let a2208 = 1u64;
        nat_add(a2207, a2208)?
    })?;
    let v4: Bytes = {
        let a2209 = v3;
        let a2210 = 117u64;
        if nat_eq(a2209, a2210) {
            f353(v0, v1)?
        } else {
            f354(v3)
        }
    };
    let a2211 = v4.clone();
    let a2212 = Bytes::lit(&[]);
    if equal(a2211, a2212) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2213 = v3;
            let a2214 = 117u64;
            if nat_eq(a2213, a2214) {
                let a2215 = v1;
                let a2216 = 6u64;
                nat_add(a2215, a2216)?
            } else {
                let a2217 = v1;
                let a2218 = 2u64;
                nat_add(a2217, a2218)?
            }
        }, ({
            let a2219 = v2;
            let a2220 = v4;
            append_bytes(a2219, a2220)
        }, 0u64))))
    }
}

pub fn f338(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f257(v0, v1)?;
    let a2221 = 47u64;
    let a2222 = v2;
    if if nat_lt(a2221, a2222) {
        let a2223 = v2;
        let a2224 = 58u64;
        nat_lt(a2223, a2224)
    } else {
        false
    } {
        Ok(Some({
            let a2225 = v1;
            let a2226 = 1u64;
            nat_add(a2225, a2226)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f339(v0: Bytes) -> R<u64> {
    let (h2227, _) = f180(Fn3::F355(v0.clone()), {
        let a2228 = v0;
        length_bytes(a2228)
    }, 0u64)?;
    Ok(h2227)
}

pub fn f340(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2232 = {
        let a2230 = v0;
        let a2231 = {
            let (h2229, _) = v1;
            h2229
        };
        index_bytes(a2230, a2231)
    };
    match m2232 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2234 = {
                    let (h2233, _) = v1;
                    h2233
                };
                let a2235 = 1u64;
                nat_add(a2234, a2235)?
            }, {
                let a2239 = {
                    let a2237 = {
                        let (_, h2236) = v1;
                        h2236
                    };
                    let a2238 = 10u64;
                    nat_mul(a2237, a2238)?
                };
                let a2240 = f356(v2)?;
                nat_add(a2239, a2240)?
            })))
        }
    }
}

pub fn f341(v0: Adt14) -> Option<Adt14> {
    let m2241 = f108(f104(v0, f8(Str::lit("oneOf"))));
    match m2241.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2242 = v2;
            match m2242.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2243 = v4;
                    match m2243.uncons() {
                        None => {
                            if f357(v3.clone()) {
                                Some(v1)
                            } else {
                                if f357(v1) {
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

pub fn f342(v0: Bytes, v1: Result<(Bytes, List<Adt21>), Bytes>) -> Result<(Bytes, List<Adt21>), Bytes> {
    let m2244 = v1;
    match m2244 {
        Ok(v3) => {
            f294(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2245, _) = v3;
                h2245
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2246) = v3;
                h2246
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt21>), Bytes>(v2)
        }
    }
}

pub fn f343(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2247 = f341(v0.clone());
    match m2247 {
        None => {
            if f27(f107(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2248 = f104(v0, f8(Str::lit("items")));
                match m2248 {
                    None => {
                        Ok(f293(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f342(f8(Str::lit("List")), f358(v3, {
                            let a2249 = v1;
                            let a2250 = f8(Str::lit("Item"));
                            append_bytes(a2249, a2250)
                        })?))
                    }
                }
            } else {
                f358(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f342(f8(Str::lit("Option")), f358(v2, v1)?))
        }
    }
}

pub fn f344(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2251, _) = v2;
        h2251
    };
    let a2252 = v3;
    let a2253 = 0u64;
    if nat_eq(a2252, a2253) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2254 = v3;
            let a2255 = v1;
            let a2256 = 0u64;
            nat_quot(a2254, a2255, a2256)
        }, {
            let a2261 = f318(v0, {
                let a2257 = v3;
                let a2258 = v1;
                let a2259 = 0u64;
                nat_rem(a2257, a2258, a2259)
            });
            let a2262 = {
                let (_, h2260) = v2;
                h2260
            };
            append_bytes(a2261, a2262)
        }))
    }
}

pub fn f345(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2307 = {
            let a2283 = {
                let a2271 = {
                    let a2265 = {
                        let a2263 = v1;
                        let a2264 = 128u8;
                        fixed_u8::bit_and(a2263, a2264)
                    };
                    let a2266 = 128u8;
                    if equal(a2265, a2266) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2272 = {
                    let a2269 = {
                        let a2267 = v1;
                        let a2268 = 64u8;
                        fixed_u8::bit_and(a2267, a2268)
                    };
                    let a2270 = 64u8;
                    if equal(a2269, a2270) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2271, a2272)?
            };
            let a2284 = {
                let a2281 = {
                    let a2275 = {
                        let a2273 = v1;
                        let a2274 = 32u8;
                        fixed_u8::bit_and(a2273, a2274)
                    };
                    let a2276 = 32u8;
                    if equal(a2275, a2276) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2282 = {
                    let a2279 = {
                        let a2277 = v1;
                        let a2278 = 16u8;
                        fixed_u8::bit_and(a2277, a2278)
                    };
                    let a2280 = 16u8;
                    if equal(a2279, a2280) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2281, a2282)?
            };
            nat_add(a2283, a2284)?
        };
        let a2308 = {
            let a2305 = {
                let a2293 = {
                    let a2287 = {
                        let a2285 = v1;
                        let a2286 = 8u8;
                        fixed_u8::bit_and(a2285, a2286)
                    };
                    let a2288 = 8u8;
                    if equal(a2287, a2288) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2294 = {
                    let a2291 = {
                        let a2289 = v1;
                        let a2290 = 4u8;
                        fixed_u8::bit_and(a2289, a2290)
                    };
                    let a2292 = 4u8;
                    if equal(a2291, a2292) {
                        4u64
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
                        let a2296 = 2u8;
                        fixed_u8::bit_and(a2295, a2296)
                    };
                    let a2298 = 2u8;
                    if equal(a2297, a2298) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2304 = {
                    let a2301 = {
                        let a2299 = v1;
                        let a2300 = 1u8;
                        fixed_u8::bit_and(a2299, a2300)
                    };
                    let a2302 = 1u8;
                    if equal(a2301, a2302) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2303, a2304)?
            };
            nat_add(a2305, a2306)?
        };
        nat_add(a2307, a2308)?
    };
    let a2315 = f318(f359(), {
        let a2309 = v2;
        let a2310 = 16u64;
        let a2311 = 0u64;
        nat_quot(a2309, a2310, a2311)
    });
    let a2316 = f318(f359(), {
        let a2312 = v2;
        let a2313 = 16u64;
        let a2314 = 0u64;
        nat_rem(a2312, a2313, a2314)
    });
    Ok(append_bytes(a2315, a2316))
}

pub fn f346(v0: Fn21, p2320: List<List<(Bytes, Bytes)>>, p2321: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2319 = (p2320, p2321);
    {
        let mut m2319 = m2319;
        loop {
            let step = {
                let (v1, v2) = m2319;
                let m2317 = v2;
                match m2317.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2318: Fn21 = v0.clone();
                            c2318.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f348(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2323 = {
        let a2322 = v0.clone();
        length_list(a2322)
    };
    let a2324 = 1u64;
    if nat_le(a2323, a2324) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f360(v0)?))
    }
}

pub fn f349(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2325, _) = v0;
        h2325
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2326) = v0;
        h2326
    };
    let m2327 = v1;
    match m2327.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2328 = v4;
            match m2328.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f361(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f350(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2329 = f251(v0, v1);
    let a2330 = f336(v0, v2);
    fixed_u32::bit_or(a2329, a2330)
}

pub fn f351(v0: u32) -> u32 {
    let a2333 = {
        let a2331 = f350(v0, 17u32, 15u32);
        let a2332 = f350(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2331, a2332)
    };
    let a2334 = f251(v0, 10u32);
    fixed_u32::bit_xor(a2333, a2334)
}

pub fn f352(v0: u32) -> u32 {
    let a2337 = {
        let a2335 = f350(v0, 7u32, 25u32);
        let a2336 = f350(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2335, a2336)
    };
    let a2338 = f251(v0, 3u32);
    fixed_u32::bit_xor(a2337, a2338)
}

pub fn f353(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2341 = f45(v0.clone(), {
        let a2339 = v1;
        let a2340 = 2u64;
        nat_add(a2339, a2340)?
    }, 2u64);
    let a2342 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2341, a2342) {
        let m2345 = f362(f45(v0, {
            let a2343 = v1;
            let a2344 = 4u64;
            nat_add(a2343, a2344)?
        }, 2u64))?;
        match m2345 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f257(v2.clone(), 0u64)?;
                let a2346 = v3;
                let a2347 = 32u64;
                if nat_lt(a2346, a2347) {
                    if f363(v3) {
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

pub fn f354(v0: u64) -> Bytes {
    let a2348 = v0;
    let a2349 = 34u64;
    if nat_eq(a2348, a2349) {
        Bytes::lit(&[0x22])
    } else {
        let a2350 = v0;
        let a2351 = 92u64;
        if nat_eq(a2350, a2351) {
            Bytes::lit(&[0x5c])
        } else {
            let a2352 = v0;
            let a2353 = 98u64;
            if nat_eq(a2352, a2353) {
                Bytes::lit(&[0x08])
            } else {
                let a2354 = v0;
                let a2355 = 102u64;
                if nat_eq(a2354, a2355) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2356 = v0;
                    let a2357 = 110u64;
                    if nat_eq(a2356, a2357) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2358 = v0;
                        let a2359 = 114u64;
                        if nat_eq(a2358, a2359) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2360 = v0;
                            let a2361 = 116u64;
                            if nat_eq(a2360, a2361) {
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

pub fn f355(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2364 = {
        let a2362 = v0;
        let a2363 = v1;
        index_bytes(a2362, a2363)
    };
    match m2364 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f364(v2)? {
                Ok(Some({
                    let a2365 = v1;
                    let a2366 = 1u64;
                    nat_add(a2365, a2366)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f356(v0: u8) -> R<u64> {
    let a2413 = {
        let v1: u8 = v0;
        let a2411 = {
            let a2387 = {
                let a2375 = {
                    let a2369 = {
                        let a2367 = v1;
                        let a2368 = 128u8;
                        fixed_u8::bit_and(a2367, a2368)
                    };
                    let a2370 = 128u8;
                    if equal(a2369, a2370) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2376 = {
                    let a2373 = {
                        let a2371 = v1;
                        let a2372 = 64u8;
                        fixed_u8::bit_and(a2371, a2372)
                    };
                    let a2374 = 64u8;
                    if equal(a2373, a2374) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2375, a2376)?
            };
            let a2388 = {
                let a2385 = {
                    let a2379 = {
                        let a2377 = v1;
                        let a2378 = 32u8;
                        fixed_u8::bit_and(a2377, a2378)
                    };
                    let a2380 = 32u8;
                    if equal(a2379, a2380) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2386 = {
                    let a2383 = {
                        let a2381 = v1;
                        let a2382 = 16u8;
                        fixed_u8::bit_and(a2381, a2382)
                    };
                    let a2384 = 16u8;
                    if equal(a2383, a2384) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2385, a2386)?
            };
            nat_add(a2387, a2388)?
        };
        let a2412 = {
            let a2409 = {
                let a2397 = {
                    let a2391 = {
                        let a2389 = v1;
                        let a2390 = 8u8;
                        fixed_u8::bit_and(a2389, a2390)
                    };
                    let a2392 = 8u8;
                    if equal(a2391, a2392) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2398 = {
                    let a2395 = {
                        let a2393 = v1;
                        let a2394 = 4u8;
                        fixed_u8::bit_and(a2393, a2394)
                    };
                    let a2396 = 4u8;
                    if equal(a2395, a2396) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2397, a2398)?
            };
            let a2410 = {
                let a2407 = {
                    let a2401 = {
                        let a2399 = v1;
                        let a2400 = 2u8;
                        fixed_u8::bit_and(a2399, a2400)
                    };
                    let a2402 = 2u8;
                    if equal(a2401, a2402) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2408 = {
                    let a2405 = {
                        let a2403 = v1;
                        let a2404 = 1u8;
                        fixed_u8::bit_and(a2403, a2404)
                    };
                    let a2406 = 1u8;
                    if equal(a2405, a2406) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2407, a2408)?
            };
            nat_add(a2409, a2410)?
        };
        nat_add(a2411, a2412)?
    };
    let a2414 = 48u64;
    Ok(nat_sub(a2413, a2414))
}

pub fn f357(v0: Adt14) -> bool {
    f27(f107(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f358(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2415 = f365(f266(v0.clone()));
    match m2415 {
        None => {
            let m2416 = f112(f104(v0.clone(), f8(Str::lit("$ref"))));
            match m2416 {
                None => {
                    let m2418 = f104(v0.clone(), f8(Str::lit("enum")));
                    match m2418 {
                        None => {
                            let m2419 = f104(v0.clone(), f8(Str::lit("const")));
                            match m2419 {
                                None => {
                                    let v8: Bytes = f107(v0.clone(), f8(Str::lit("type")));
                                    if f27(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f294(f8(Str::lit("Bytes")), List::<Adt21>::nil()))
                                    } else {
                                        if f27(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f294(f367(v0), List::<Adt21>::nil()))
                                        } else {
                                            if f27(v8.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f294(f8(Str::lit("Bool")), List::<Adt21>::nil()))
                                            } else {
                                                if f27(v8.clone(), f8(Str::lit("object"))) {
                                                    Ok(f294(v1.clone(), List::cons(Adt21::C0(v1, v0), List::<Adt21>::nil())))
                                                } else {
                                                    Ok(f293(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2420 = f112(Some(v6));
                                    match m2420 {
                                        None => {
                                            Ok(f293(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f294(f8(Str::lit("Bytes")), List::<Adt21>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f294(v1.clone(), List::cons(Adt21::C0(v1, v0), List::<Adt21>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2417 = f366(v3)?;
                    match m2417 {
                        None => {
                            Ok(f293(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f294(v4, List::<Adt21>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f293(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f359() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f360(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f321({
        let (_, h2421) = {
            let (h2422, _) = f368(Fn22::F369, {
                let a2424 = {
                    let a2423 = v0.clone();
                    length_list(a2423)
                };
                let a2425 = 1u64;
                nat_add(a2424, a2425)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2422
        };
        h2421
    }))
}

pub fn f361(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2430 = {
            let a2428 = {
                let a2426 = v0.clone();
                length_list(a2426)
            };
            let a2429 = {
                let a2427 = v1.clone();
                length_list(a2427)
            };
            nat_add(a2428, a2429)?
        };
        let a2431 = 1u64;
        nat_add(a2430, a2431)?
    };
    Ok(f246({
        let (_, h2432) = {
            let (_, h2433) = {
                let (h2434, _) = f370(Fn23::F371, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2434
            };
            h2433
        };
        h2432
    }))
}

pub fn f362(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2435 = v0.clone();
        length_bytes(a2435)
    };
    let a2439 = {
        let a2436 = v1;
        let a2437 = 2u64;
        let a2438 = 0u64;
        nat_rem(a2436, a2437, a2438)
    };
    let a2440 = 0u64;
    if nat_eq(a2439, a2440) {
        let (_, h2441) = {
            let (h2442, _) = f372(Fn24::F373(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2442
        };
        Ok(h2441)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f363(v0: u64) -> bool {
    let a2443 = v0;
    let a2444 = 8u64;
    if nat_eq(a2443, a2444) {
        true
    } else {
        let a2445 = v0;
        let a2446 = 9u64;
        if nat_eq(a2445, a2446) {
            true
        } else {
            let a2447 = v0;
            let a2448 = 10u64;
            if nat_eq(a2447, a2448) {
                true
            } else {
                let a2449 = v0;
                let a2450 = 12u64;
                if nat_eq(a2449, a2450) {
                    true
                } else {
                    let a2451 = v0;
                    let a2452 = 13u64;
                    nat_eq(a2451, a2452)
                }
            }
        }
    }
}

pub fn f364(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2497 = {
            let a2473 = {
                let a2461 = {
                    let a2455 = {
                        let a2453 = v1;
                        let a2454 = 128u8;
                        fixed_u8::bit_and(a2453, a2454)
                    };
                    let a2456 = 128u8;
                    if equal(a2455, a2456) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2462 = {
                    let a2459 = {
                        let a2457 = v1;
                        let a2458 = 64u8;
                        fixed_u8::bit_and(a2457, a2458)
                    };
                    let a2460 = 64u8;
                    if equal(a2459, a2460) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2461, a2462)?
            };
            let a2474 = {
                let a2471 = {
                    let a2465 = {
                        let a2463 = v1;
                        let a2464 = 32u8;
                        fixed_u8::bit_and(a2463, a2464)
                    };
                    let a2466 = 32u8;
                    if equal(a2465, a2466) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2472 = {
                    let a2469 = {
                        let a2467 = v1;
                        let a2468 = 16u8;
                        fixed_u8::bit_and(a2467, a2468)
                    };
                    let a2470 = 16u8;
                    if equal(a2469, a2470) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2471, a2472)?
            };
            nat_add(a2473, a2474)?
        };
        let a2498 = {
            let a2495 = {
                let a2483 = {
                    let a2477 = {
                        let a2475 = v1;
                        let a2476 = 8u8;
                        fixed_u8::bit_and(a2475, a2476)
                    };
                    let a2478 = 8u8;
                    if equal(a2477, a2478) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2484 = {
                    let a2481 = {
                        let a2479 = v1;
                        let a2480 = 4u8;
                        fixed_u8::bit_and(a2479, a2480)
                    };
                    let a2482 = 4u8;
                    if equal(a2481, a2482) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2483, a2484)?
            };
            let a2496 = {
                let a2493 = {
                    let a2487 = {
                        let a2485 = v1;
                        let a2486 = 2u8;
                        fixed_u8::bit_and(a2485, a2486)
                    };
                    let a2488 = 2u8;
                    if equal(a2487, a2488) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2494 = {
                    let a2491 = {
                        let a2489 = v1;
                        let a2490 = 1u8;
                        fixed_u8::bit_and(a2489, a2490)
                    };
                    let a2492 = 1u8;
                    if equal(a2491, a2492) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2493, a2494)?
            };
            nat_add(a2495, a2496)?
        };
        nat_add(a2497, a2498)?
    };
    let a2499 = 48u64;
    let a2500 = v2;
    if nat_le(a2499, a2500) {
        let a2501 = v2;
        let a2502 = 57u64;
        Ok(nat_le(a2501, a2502))
    } else {
        Ok(false)
    }
}

pub fn f365(p2508: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2507 = p2508;
    {
        let mut m2507 = m2507;
        loop {
            let step = {
                let v0 = m2507;
                let m2503 = v0;
                match m2503.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f70({
                            let (h2504, _) = v1.clone();
                            h2504
                        }, f374()) {
                            true
                        } else {
                            f70({
                                let (h2505, _) = v1.clone();
                                h2505
                            }, f375())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2506, _) = v1;
                                h2506
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: Bytes) -> R<Option<Bytes>> {
    if f238(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f166(f45(v0.clone(), 8u64, {
            let a2510 = {
                let a2509 = v0;
                length_bytes(a2509)
            };
            let a2511 = 8u64;
            nat_sub(a2510, a2511)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f367(v0: Adt14) -> Bytes {
    let m2512 = f237(f104(v0, f8(Str::lit("minimum"))));
    match m2512 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f368(v0: Fn22, p2517: u64, p2518: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2516 = (p2517, p2518);
    {
        let mut m2516 = m2516;
        loop {
            let step = {
                let (v1, v2) = m2516;
                let m2513 = v1;
                if m2513 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2513 - 1;
                    let m2515 = {
                        let c2514: Fn22 = v0.clone();
                        c2514.apply(v2.clone())?
                    };
                    match m2515 {
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
                Ok(next) => { m2516 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f376(v0)
}

pub fn f370(v0: Fn23, p2523: u64, p2524: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2522 = (p2523, p2524);
    {
        let mut m2522 = m2522;
        loop {
            let step = {
                let (v1, v2) = m2522;
                let m2519 = v1;
                if m2519 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2519 - 1;
                    let m2521 = {
                        let c2520: Fn23 = v0.clone();
                        c2520.apply(v2.clone())
                    };
                    match m2521 {
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
                Ok(next) => { m2522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f371(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f377(v0)
}

pub fn f372(v0: Fn24, p2529: u64, p2530: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2528 = (p2529, p2530);
    {
        let mut m2528 = m2528;
        loop {
            let step = {
                let (v1, v2) = m2528;
                let m2525 = v1;
                if m2525 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2525 - 1;
                    let m2527 = {
                        let c2526: Fn24 = v0.clone();
                        c2526.apply(v2.clone())?
                    };
                    match m2527 {
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
                Ok(next) => { m2528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f378(v0, v1)
}

pub fn f374() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f375() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f376(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2531, _) = v0;
        h2531
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2532) = v0;
        h2532
    };
    let m2533 = v1;
    match m2533.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2534 = v4;
            match m2534.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f379(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f377(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2535, _) = v0.clone();
        h2535
    };
    let v2: List<(Bytes, ())> = {
        let (h2536, _) = {
            let (_, h2537) = v0.clone();
            h2537
        };
        h2536
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2538) = {
            let (_, h2539) = v0;
            h2539
        };
        h2538
    };
    let m2540 = v1.clone();
    match m2540.uncons() {
        None => {
            let m2541 = v2;
            match m2541.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2542 = v2.clone();
            match m2542.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f380({
                        let (h2543, _) = v8.clone();
                        h2543
                    }, {
                        let (h2544, _) = v6.clone();
                        h2544
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

pub fn f378(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2545, _) = v1;
        h2545
    };
    let m2547 = {
        let (_, h2546) = v1;
        h2546
    };
    match m2547 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2550 = {
                let a2548 = v0.clone();
                let a2549 = v2;
                index_bytes(a2548, a2549)
            };
            match m2550 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2555 = {
                        let a2553 = v0;
                        let a2554 = {
                            let a2551 = v2;
                            let a2552 = 1u64;
                            nat_add(a2551, a2552)?
                        };
                        index_bytes(a2553, a2554)
                    };
                    match m2555 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f381(v4)? {
                                f381(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2556 = v2;
                                    let a2557 = 2u64;
                                    nat_add(a2556, a2557)?
                                }, Some({
                                    let a2562 = v3;
                                    let a2563 = f318(f382(), {
                                        let a2560 = {
                                            let a2558 = f383(v4)?;
                                            let a2559 = 16u64;
                                            nat_mul(a2558, a2559)?
                                        };
                                        let a2561 = f383(v5)?;
                                        nat_add(a2560, a2561)?
                                    });
                                    append_bytes(a2562, a2563)
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

pub fn f379(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2568 = {
            let a2566 = {
                let a2564 = v0.clone();
                length_list(a2564)
            };
            let a2567 = {
                let a2565 = v1.clone();
                length_list(a2565)
            };
            nat_add(a2566, a2567)?
        };
        let a2569 = 1u64;
        nat_add(a2568, a2569)?
    };
    Ok(f384({
        let (_, h2570) = {
            let (_, h2571) = {
                let (h2572, _) = f385(Fn25::F386, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2572
            };
            h2571
        };
        h2570
    }))
}

pub fn f380(v0: Bytes, v1: Bytes) -> bool {
    let a2577 = {
        let a2573 = v0;
        let a2574 = v1;
        compare_bytes(a2573, a2574)
    };
    let a2578 = {
        let a2575 = Bytes::lit(&[]);
        let a2576 = Bytes::lit(&[0x01]);
        compare_bytes(a2575, a2576)
    };
    equal(a2577, a2578)
}

pub fn f381(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2623 = {
            let a2599 = {
                let a2587 = {
                    let a2581 = {
                        let a2579 = v1;
                        let a2580 = 128u8;
                        fixed_u8::bit_and(a2579, a2580)
                    };
                    let a2582 = 128u8;
                    if equal(a2581, a2582) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2588 = {
                    let a2585 = {
                        let a2583 = v1;
                        let a2584 = 64u8;
                        fixed_u8::bit_and(a2583, a2584)
                    };
                    let a2586 = 64u8;
                    if equal(a2585, a2586) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2587, a2588)?
            };
            let a2600 = {
                let a2597 = {
                    let a2591 = {
                        let a2589 = v1;
                        let a2590 = 32u8;
                        fixed_u8::bit_and(a2589, a2590)
                    };
                    let a2592 = 32u8;
                    if equal(a2591, a2592) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2598 = {
                    let a2595 = {
                        let a2593 = v1;
                        let a2594 = 16u8;
                        fixed_u8::bit_and(a2593, a2594)
                    };
                    let a2596 = 16u8;
                    if equal(a2595, a2596) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2597, a2598)?
            };
            nat_add(a2599, a2600)?
        };
        let a2624 = {
            let a2621 = {
                let a2609 = {
                    let a2603 = {
                        let a2601 = v1;
                        let a2602 = 8u8;
                        fixed_u8::bit_and(a2601, a2602)
                    };
                    let a2604 = 8u8;
                    if equal(a2603, a2604) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2610 = {
                    let a2607 = {
                        let a2605 = v1;
                        let a2606 = 4u8;
                        fixed_u8::bit_and(a2605, a2606)
                    };
                    let a2608 = 4u8;
                    if equal(a2607, a2608) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2609, a2610)?
            };
            let a2622 = {
                let a2619 = {
                    let a2613 = {
                        let a2611 = v1;
                        let a2612 = 2u8;
                        fixed_u8::bit_and(a2611, a2612)
                    };
                    let a2614 = 2u8;
                    if equal(a2613, a2614) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2620 = {
                    let a2617 = {
                        let a2615 = v1;
                        let a2616 = 1u8;
                        fixed_u8::bit_and(a2615, a2616)
                    };
                    let a2618 = 1u8;
                    if equal(a2617, a2618) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2619, a2620)?
            };
            nat_add(a2621, a2622)?
        };
        nat_add(a2623, a2624)?
    };
    let a2625 = 48u64;
    let a2626 = v2;
    if if nat_le(a2625, a2626) {
        let a2627 = v2;
        let a2628 = 57u64;
        nat_le(a2627, a2628)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2629 = 97u64;
        let a2630 = v2;
        if nat_le(a2629, a2630) {
            let a2631 = v2;
            let a2632 = 102u64;
            Ok(nat_le(a2631, a2632))
        } else {
            Ok(false)
        }
    }
}

pub fn f382() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f383(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2677 = {
            let a2653 = {
                let a2641 = {
                    let a2635 = {
                        let a2633 = v1;
                        let a2634 = 128u8;
                        fixed_u8::bit_and(a2633, a2634)
                    };
                    let a2636 = 128u8;
                    if equal(a2635, a2636) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2642 = {
                    let a2639 = {
                        let a2637 = v1;
                        let a2638 = 64u8;
                        fixed_u8::bit_and(a2637, a2638)
                    };
                    let a2640 = 64u8;
                    if equal(a2639, a2640) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2641, a2642)?
            };
            let a2654 = {
                let a2651 = {
                    let a2645 = {
                        let a2643 = v1;
                        let a2644 = 32u8;
                        fixed_u8::bit_and(a2643, a2644)
                    };
                    let a2646 = 32u8;
                    if equal(a2645, a2646) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2652 = {
                    let a2649 = {
                        let a2647 = v1;
                        let a2648 = 16u8;
                        fixed_u8::bit_and(a2647, a2648)
                    };
                    let a2650 = 16u8;
                    if equal(a2649, a2650) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2651, a2652)?
            };
            nat_add(a2653, a2654)?
        };
        let a2678 = {
            let a2675 = {
                let a2663 = {
                    let a2657 = {
                        let a2655 = v1;
                        let a2656 = 8u8;
                        fixed_u8::bit_and(a2655, a2656)
                    };
                    let a2658 = 8u8;
                    if equal(a2657, a2658) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2664 = {
                    let a2661 = {
                        let a2659 = v1;
                        let a2660 = 4u8;
                        fixed_u8::bit_and(a2659, a2660)
                    };
                    let a2662 = 4u8;
                    if equal(a2661, a2662) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2663, a2664)?
            };
            let a2676 = {
                let a2673 = {
                    let a2667 = {
                        let a2665 = v1;
                        let a2666 = 2u8;
                        fixed_u8::bit_and(a2665, a2666)
                    };
                    let a2668 = 2u8;
                    if equal(a2667, a2668) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2674 = {
                    let a2671 = {
                        let a2669 = v1;
                        let a2670 = 1u8;
                        fixed_u8::bit_and(a2669, a2670)
                    };
                    let a2672 = 1u8;
                    if equal(a2671, a2672) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2673, a2674)?
            };
            nat_add(a2675, a2676)?
        };
        nat_add(a2677, a2678)?
    };
    let a2679 = v2;
    let a2680 = 57u64;
    if nat_le(a2679, a2680) {
        let a2681 = v2;
        let a2682 = 48u64;
        Ok(nat_sub(a2681, a2682))
    } else {
        let a2683 = v2;
        let a2684 = 87u64;
        Ok(nat_sub(a2683, a2684))
    }
}

pub fn f384(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f272(Fn10::F387, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f385(v0: Fn25, p2689: u64, p2690: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2688 = (p2689, p2690);
    {
        let mut m2688 = m2688;
        loop {
            let step = {
                let (v1, v2) = m2688;
                let m2685 = v1;
                if m2685 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2685 - 1;
                    let m2687 = {
                        let c2686: Fn25 = v0.clone();
                        c2686.apply(v2.clone())
                    };
                    match m2687 {
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
                Ok(next) => { m2688 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f388(v0)
}

pub fn f387(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f388(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2691, _) = v0.clone();
        h2691
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2692, _) = {
            let (_, h2693) = v0.clone();
            h2693
        };
        h2692
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2694) = {
            let (_, h2695) = v0;
            h2695
        };
        h2694
    };
    let m2696 = v1.clone();
    match m2696.uncons() {
        None => {
            let m2697 = v2;
            match m2697.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2698 = v2.clone();
            match m2698.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f380({
                        let (h2699, _) = v8.clone();
                        h2699
                    }, {
                        let (h2700, _) = v6.clone();
                        h2700
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
