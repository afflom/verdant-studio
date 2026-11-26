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
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt13 {
    C0(bool),
    C1(bool, u64),
    C2(Bytes),
    C3(std::rc::Rc<Adt13>),
    C4(std::rc::Rc<Adt13>),
    C5,
    C6(std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C7(Bytes, std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
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
    C0(Adt15, List<Adt13>, List<Adt13>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(List<u64>, List<u64>, List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt18>),
    C5(std::rc::Rc<Adt18>),
    C6,
    C7(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C8(Bytes, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
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
    C0(Bytes, Adt18),
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
    F64(u8, Bytes),
    F161(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn0::F161(k0) => f161(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F94,
    F208,
    F296,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F94 => f94(p0, p1),
            Fn1::F208 => f208(p0, p1),
            Fn1::F296 => f296(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F175(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F175(k0) => f175(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F283(Bytes, u64, Bytes, u64),
    F360(Bytes),
    F361(u64, Bytes),
    F459(Bytes),
    F460(Bytes),
    F498(Bytes),
    F519(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F283(k0, k1, k2, k3) => f283(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F360(k0) => Ok(f360(k0.clone(), p0)),
            Fn3::F361(k0, k1) => f361(*k0, k1.clone(), p0),
            Fn3::F459(k0) => f459(k0.clone(), p0),
            Fn3::F460(k0) => f460(k0.clone(), p0),
            Fn3::F498(k0) => f498(k0.clone(), p0),
            Fn3::F519(k0) => f519(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F350(Bytes),
    F466(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F350(k0) => f350(k0.clone(), p0),
            Fn4::F466(k0, k1) => Ok(f466(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F352,
    F631,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F352 => f352(p0, p1),
            Fn5::F631 => f631(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F355,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F355 => f355(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F377,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F377 => f377(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F380,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F380 => f380(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F382,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F382 => f382(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F385,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F385 => f385(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F406(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn11::F406(k0, k1, k2) => f406(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F429,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F429 => f429(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F432,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F432 => f432(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F458(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn14::F458(k0) => f458(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F463(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn15::F463(k0) => f463(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F470,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F470 => f470(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F472,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F472 => f472(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F490,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F490 => f490(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F502,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F502 => f502(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F505,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F505 => f505(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F552,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F552 => f552(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F558,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F558 => f558(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F587,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F587 => f587(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F604(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F604(k0) => f604(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F614,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F614 => f614(p0),
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
    let m136 = v0.clone();
    match m136.uncons() {
        None => {
            f27(v0, v1)
        }
        Some((v2, _)) => {
            if f28(v2.clone(), f8(Str::lit("stage-engine"))) {
                f29(f30(v1)?)
            } else {
                if f28(v2, f8(Str::lit("publish"))) {
                    Ok(f31(f27(v0, v1)?))
                } else {
                    f27(v0, v1)
                }
            }
        }
    }
}

pub fn f21(v0: Adt0, v1: List<Bytes>) -> Adt0 {
    Adt0::C0(v1, match v0.clone() {
        Adt0::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h138, _, _, _, _, _, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h139, _, _, _, _, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h140, _, _, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h141, _, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h142, _, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h143, _, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h144, _, _) => {
            h144
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h145, _) => {
            h145
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h146) => {
            h146
        }
    })
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> R<List<Str>> {
    let m147 = v2;
    if m147 == 0 {
        Ok(List::<Str>::nil())
    } else {
        let v3 = m147 - 1;
        let m150 = {
            let a148 = v0.clone();
            let a149 = v1;
            index_bytes(a148, a149)
        };
        match m150 {
            None => {
                Ok(List::<Str>::nil())
            }
            Some(v4) => {
                Ok(List::cons(f32(v4)?, f22(v0, {
                    let a151 = v1;
                    let a152 = 1u64;
                    nat_add(a151, a152)?
                }, v3)?))
            }
        }
    }
}

pub fn f23(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h153, _, _, _, _, _, _, _, _, _, _) => {
            h153
        }
    }, match v0.clone() {
        Adt0::C0(_, h154, _, _, _, _, _, _, _, _, _) => {
            h154
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h155, _, _, _, _, _, _, _, _) => {
            h155
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h156, _, _, _, _, _) => {
            h156
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h157, _, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h158, _, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h159, _, _) => {
            h159
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h160, _) => {
            h160
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h161) => {
            h161
        }
    })
}

pub fn f24(v0: List<Bytes>) -> R<List<Str>> {
    let m162 = v0;
    match m162.uncons() {
        None => {
            Ok(List::<Str>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f4(v1)?, f24(v2)?))
        }
    }
}

pub fn f25(v0: List<(Bytes, Bytes)>) -> R<List<(Str, Str)>> {
    let m163 = v0;
    match m163.uncons() {
        None => {
            Ok(List::<(Str, Str)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f4({
                let (h164, _) = v1;
                h164
            })?, f4({
                let (_, h165) = v1;
                h165
            })?), f25(v2)?))
        }
    }
}

pub fn f26(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f33(v0.clone(), 47u8)?;
    let v2: Bytes = f34(v1);
    let a168 = {
        let a166 = v0.clone();
        length_bytes(a166)
    };
    let a169 = {
        let a167 = v2.clone();
        length_bytes(a167)
    };
    if nat_eq(a168, a169) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f35(v0.clone(), 0u64, {
            let a174 = {
                let a170 = v0;
                length_bytes(a170)
            };
            let a175 = {
                let a172 = {
                    let a171 = v2;
                    length_bytes(a171)
                };
                let a173 = 1u64;
                nat_add(a172, a173)?
            };
            nat_sub(a174, a175)
        }))
    }
}

pub fn f27(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    f29(f36(v0, v1)?)
}

pub fn f28(v0: Bytes, v1: Bytes) -> bool {
    let a176 = v0;
    let a177 = v1;
    equal(a176, a177)
}

pub fn f29(v0: Adt11) -> R<Adt11> {
    let m179 = {
        let m178 = v0.clone();
        match m178 {
            Adt11::C0(v1) => {
                f37(v1)?
            }
            Adt11::C1(v2) => {
                f37(v2)?
            }
            Adt11::C2(v3, v4, _) => {
                f37(List::cons(v3, v4))?
            }
            Adt11::C3(v6, _) => {
                f37(f38(v6))?
            }
            Adt11::C4(_) => {
                None::<Bytes>
            }
        }
    };
    match m179 {
        None => {
            Ok(v0)
        }
        Some(v9) => {
            Ok(Adt11::C4(f39(v9)))
        }
    }
}

pub fn f30(v0: Adt1) -> R<Adt11> {
    if f40(f41(f42(), match v0.clone() {
        Adt1::C0(_, _, h180) => {
            h180
        }
    })) {
        Ok(f43(f42(), f44(f8(Str::lit("releases/latest")))))
    } else {
        let m181 = f45(v0.clone(), List::cons(f42(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil()))));
        match m181.uncons() {
            None => {
                let v4: Bytes = {
                    let m182 = f46(v0.clone(), f42())?;
                    match m182 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v3) => {
                            f47(v3, f8(Str::lit("tag_name")))
                        }
                    }
                };
                let a184 = {
                    let a183 = v4.clone();
                    length_bytes(a183)
                };
                let a185 = 0u64;
                if nat_eq(a184, a185) {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("waiting engine: the latest verdant-engine release names no tag\u{a}"))))
                } else {
                    let v5: List<Bytes> = f48(f49(), f50(v4.clone()));
                    let a187 = {
                        let a186 = f51(v5.clone(), f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?);
                        length_list(a186)
                    };
                    let a188 = 0u64;
                    if nat_eq(a187, a188) {
                        f54(v0, v4)
                    } else {
                        let m189 = f45(v0.clone(), f55(v5));
                        match m189.uncons() {
                            None => {
                                if f56(v0.clone(), f57(f58(Adt12::C1, f50(v4.clone())))) {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting ingest: the verdant-engine ")), List::cons(v4, List::cons(f8(Str::lit(" bindings are staged, and advance ingests them\u{a}")), List::<Bytes>::nil()))))))
                                } else {
                                    f59(v0, v4)
                                }
                            }
                            Some((v6, v7)) => {
                                Ok(Adt11::C0(List::cons(v6, v7)))
                            }
                        }
                    }
                }
            }
            Some((v1, v2)) => {
                Ok(Adt11::C0(List::cons(v1, v2)))
            }
        }
    }
}

pub fn f31(v0: Adt11) -> Adt11 {
    let m190 = v0;
    match m190 {
        Adt11::C0(v3) => {
            Adt11::C0(v3)
        }
        Adt11::C1(v4) => {
            Adt11::C1(v4)
        }
        Adt11::C2(v5, v6, v7) => {
            Adt11::C2(v5, v6, v7)
        }
        Adt11::C3(v1, v2) => {
            Adt11::C3(List::cons((f8(Str::lit(".github/workflows/track.yml")), f60()), v1), v2)
        }
        Adt11::C4(v8) => {
            Adt11::C4(v8)
        }
    }
}

pub fn f32(v0: u8) -> R<Str> {
    let m193 = {
        let a191 = f61();
        let a192 = f62(v0)?;
        index_list(a191, a192)
    };
    match m193 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f33(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a194 = v0.clone();
        length_bytes(a194)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h195, _) = f63(Fn0::F64(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h195
    };
    let v4: u64 = {
        let (h196, _) = {
            let (_, h197) = v3.clone();
            h197
        };
        h196
    };
    Ok(f65(List::cons(f35(v0, v4, {
        let a198 = v2;
        let a199 = v4;
        nat_sub(a198, a199)
    }), {
        let (_, h200) = {
            let (_, h201) = v3;
            h201
        };
        h200
    })))
}

pub fn f34(p205: List<Bytes>) -> Bytes {
    let m204 = p205;
    {
        let mut m204 = m204;
        loop {
            let step = {
                let v0 = m204;
                let m202 = v0;
                match m202.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m203 = v2.clone();
                        match m203.uncons() {
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
                Ok(next) => { m204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m209 = {
        let a206 = v0;
        let a207 = v1;
        let a208 = v2;
        slice_bytes(a206, a207, a208)
    };
    match m209 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f36(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m210 = v0;
    match m210.uncons() {
        None => {
            Ok(Adt11::C4(f66()))
        }
        Some((v2, v3)) => {
            if f28(v2.clone(), f8(Str::lit("agenda"))) {
                f67(v1)
            } else {
                if f28(v2.clone(), f8(Str::lit("build"))) {
                    Ok(f68(v1))
                } else {
                    if f28(v2.clone(), f8(Str::lit("check"))) {
                        f69(v1)
                    } else {
                        if f28(v2.clone(), f8(Str::lit("next"))) {
                            f70(v1, v3)
                        } else {
                            if f28(v2.clone(), f8(Str::lit("publish"))) {
                                f71(v1)
                            } else {
                                let m211 = v3;
                                match m211.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f66()))
                                    }
                                    Some((v4, v5)) => {
                                        if f72(v2.clone(), f8(Str::lit("read-"))) {
                                            Ok(Adt11::C4(f8(Str::lit("read-webidl, read-xml, read-xsd and read-yaml run in verdant_readers, the command of the readers project\u{a}"))))
                                        } else {
                                            if f72(v2.clone(), f8(Str::lit("stage-"))) {
                                                f73(v2, v4, v5, v1)
                                            } else {
                                                f74(v2, v4, v5, v1)
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

pub fn f37(p214: List<Bytes>) -> R<Option<Bytes>> {
    let m213 = p214;
    {
        let mut m213 = m213;
        loop {
            let step = {
                let v0 = m213;
                let m212 = v0;
                match m212.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f75(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m215 = v0;
    match m215.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h216, _) = v1;
                h216
            }, f38(v2))
        }
    }
}

pub fn f39(v0: Bytes) -> Bytes {
    let a221 = {
        let a217 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a217)
    };
    let a222 = {
        let a219 = v0;
        let a220 = {
            let a218 = Str::lit(" is not\u{a}");
            utf8_encode(a218)
        };
        append_bytes(a219, a220)
    };
    append_bytes(a221, a222)
}

pub fn f40(v0: bool) -> bool {
    !v0
}

pub fn f41(v0: Bytes, p225: List<Bytes>) -> bool {
    let m224 = p225;
    {
        let mut m224 = m224;
        loop {
            let step = {
                let v1 = m224;
                let m223 = v1;
                match m223.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f42() -> Bytes {
    f8(Str::lit("target/engine/release.json"))
}

pub fn f43(v0: Bytes, v1: Bytes) -> Adt11 {
    Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v0.clone(), List::cons(v1, List::<Bytes>::nil()))))), v0)
}

pub fn f44(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://api.github.com/repos/afflom/verdant-engine/")), List::cons(v0, List::<Bytes>::nil())))
}

pub fn f45(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m226 = v1;
    match m226.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m228 = f76(match v0.clone() {
                Adt1::C0(h227, _, _) => {
                    h227
                }
            }, v2.clone());
            match m228 {
                None => {
                    List::cons(v2, f45(v0, v3))
                }
                Some(_) => {
                    f45(v0, v3)
                }
            }
        }
    }
}

pub fn f46(v0: Adt1, v1: Bytes) -> R<Option<Adt13>> {
    f77(f53(v0, v1))
}

pub fn f47(v0: Adt13, v1: Bytes) -> Bytes {
    let m229 = f78(f79(v0, v1));
    match m229 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f48(v0: List<Adt12>, v1: Bytes) -> List<Bytes> {
    let m230 = v0;
    match m230.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f48(v3, v1))
        }
    }
}

pub fn f49() -> List<Adt12> {
    List::cons(Adt12::C0, List::cons(Adt12::C1, List::cons(Adt12::C2, List::cons(Adt12::C3, List::<Adt12>::nil()))))
}

pub fn f50(v0: Bytes) -> Bytes {
    if f72(v0.clone(), f8(Str::lit("v"))) {
        f35(v0.clone(), 1u64, {
            let a232 = {
                let a231 = v0;
                length_bytes(a231)
            };
            let a233 = 1u64;
            nat_sub(a232, a233)
        })
    } else {
        v0
    }
}

pub fn f51(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m234 = v0;
    match m234.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f51(v3, v1)
            } else {
                List::cons(v2, f51(v3, v1))
            }
        }
    }
}

pub fn f52(v0: Bytes) -> R<List<Bytes>> {
    f80(f81(v0)?, f8(Str::lit("id = \"")))
}

pub fn f53(v0: Adt1, v1: Bytes) -> Bytes {
    let m236 = f76(match v0 {
        Adt1::C0(h235, _, _) => {
            h235
        }
    }, v1);
    match m236 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m237 = v2;
            match m237 {
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

pub fn f54(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let v2: List<Adt13> = f82(v0, f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f83(v2.clone(), f50(v1.clone()));
    if f84(f85(v2), f85(v3.clone())) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" is pinned, and every admission of its registers cites it\u{a}")), List::<Bytes>::nil()))))))
    } else {
        Ok(Adt11::C3(List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
    }
}

pub fn f55(v0: List<Bytes>) -> List<Bytes> {
    let m238 = v0;
    match m238.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f57(v1), f55(v2))
        }
    }
}

pub fn f56(v0: Adt1, v1: Bytes) -> bool {
    let m240 = f76(match v0 {
        Adt1::C0(h239, _, _) => {
            h239
        }
    }, v1);
    match m240 {
        None => {
            false
        }
        Some(v2) => {
            let m241 = v2;
            match m241 {
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

pub fn f57(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f58(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f88(v0), List::cons(v1, List::<Bytes>::nil())))
}

pub fn f59(v0: Adt1, v1: Bytes) -> R<Adt11> {
    if f40(f41(f89(), match v0.clone() {
        Adt1::C0(_, _, h242) => {
            h242
        }
    })) {
        Ok(f43(f89(), f44(f7(List::cons(f8(Str::lit("git/ref/tags/")), List::cons(v1, List::<Bytes>::nil()))))))
    } else {
        let m243 = f45(v0.clone(), List::cons(f89(), List::<Bytes>::nil()));
        match m243.uncons() {
            None => {
                let v6: Adt13 = {
                    let m245 = f79({
                        let m244 = f46(v0.clone(), f89())?;
                        match m244 {
                            None => {
                                Adt13::C5
                            }
                            Some(v4) => {
                                v4
                            }
                        }
                    }, f8(Str::lit("object")));
                    match m245 {
                        None => {
                            Adt13::C5
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                };
                if f28(f47(v6.clone(), f8(Str::lit("type"))), f8(Str::lit("tag"))) {
                    if f40(f41(f90(), match v0.clone() {
                        Adt1::C0(_, _, h246) => {
                            h246
                        }
                    })) {
                        Ok(f43(f90(), f44(f7(List::cons(f8(Str::lit("git/tags/")), List::cons(f47(v6, f8(Str::lit("sha"))), List::<Bytes>::nil()))))))
                    } else {
                        let m247 = f45(v0.clone(), List::cons(f90(), List::<Bytes>::nil()));
                        match m247.uncons() {
                            None => {
                                f91(v0.clone(), v1, {
                                    let m249 = f79({
                                        let m248 = f46(v0, f90())?;
                                        match m248 {
                                            None => {
                                                Adt13::C5
                                            }
                                            Some(v9) => {
                                                v9
                                            }
                                        }
                                    }, f8(Str::lit("object")));
                                    match m249 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v10) => {
                                            f47(v10, f8(Str::lit("sha")))
                                        }
                                    }
                                })
                            }
                            Some((v7, v8)) => {
                                Ok(Adt11::C0(List::cons(v7, v8)))
                            }
                        }
                    }
                } else {
                    f91(v0, v1, f47(v6, f8(Str::lit("sha"))))
                }
            }
            Some((v2, v3)) => {
                Ok(Adt11::C0(List::cons(v2, v3)))
            }
        }
    }
}

pub fn f60() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Engine.lex.tex. Do not edit.\u{a}# REQ-ENGINE-TRACKING: Verdant Studio tracks Verdant Engine releases. Every hour this runs\u{a}# operate stage-engine, which stages the latest release's four registers when they are new\u{a}# (advance then ingests them by sha256), and re-cites their admissions once they are ingested.\u{a}name: track\u{a}\u{a}on:\u{a}  schedule:\u{a}    - cron: '17 * * * *'\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}  actions: write\u{a}\u{a}jobs:\u{a}  track:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 5\u{a}    steps:\u{a}      - env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}        run: gh workflow run operate.yml --repo \"$GITHUB_REPOSITORY\" --ref main -f args=stage-engine\u{a}"))
}

pub fn f61() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f62(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a294 = {
        let a270 = {
            let a258 = {
                let a252 = {
                    let a250 = v1;
                    let a251 = 128u8;
                    fixed_u8::bit_and(a250, a251)
                };
                let a253 = 128u8;
                if equal(a252, a253) {
                    128u64
                } else {
                    0u64
                }
            };
            let a259 = {
                let a256 = {
                    let a254 = v1;
                    let a255 = 64u8;
                    fixed_u8::bit_and(a254, a255)
                };
                let a257 = 64u8;
                if equal(a256, a257) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a258, a259)?
        };
        let a271 = {
            let a268 = {
                let a262 = {
                    let a260 = v1;
                    let a261 = 32u8;
                    fixed_u8::bit_and(a260, a261)
                };
                let a263 = 32u8;
                if equal(a262, a263) {
                    32u64
                } else {
                    0u64
                }
            };
            let a269 = {
                let a266 = {
                    let a264 = v1;
                    let a265 = 16u8;
                    fixed_u8::bit_and(a264, a265)
                };
                let a267 = 16u8;
                if equal(a266, a267) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a268, a269)?
        };
        nat_add(a270, a271)?
    };
    let a295 = {
        let a292 = {
            let a280 = {
                let a274 = {
                    let a272 = v1;
                    let a273 = 8u8;
                    fixed_u8::bit_and(a272, a273)
                };
                let a275 = 8u8;
                if equal(a274, a275) {
                    8u64
                } else {
                    0u64
                }
            };
            let a281 = {
                let a278 = {
                    let a276 = v1;
                    let a277 = 4u8;
                    fixed_u8::bit_and(a276, a277)
                };
                let a279 = 4u8;
                if equal(a278, a279) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a280, a281)?
        };
        let a293 = {
            let a290 = {
                let a284 = {
                    let a282 = v1;
                    let a283 = 2u8;
                    fixed_u8::bit_and(a282, a283)
                };
                let a285 = 2u8;
                if equal(a284, a285) {
                    2u64
                } else {
                    0u64
                }
            };
            let a291 = {
                let a288 = {
                    let a286 = v1;
                    let a287 = 1u8;
                    fixed_u8::bit_and(a286, a287)
                };
                let a289 = 1u8;
                if equal(a288, a289) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a290, a291)?
        };
        nat_add(a292, a293)?
    };
    nat_add(a294, a295)
}

pub fn f63(v0: Fn0, p300: u64, p301: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m299 = (p300, p301);
    {
        let mut m299 = m299;
        loop {
            let step = {
                let (v1, v2) = m299;
                let m296 = v1;
                if m296 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m296 - 1;
                    let m298 = {
                        let c297: Fn0 = v0.clone();
                        c297.apply(v2.clone())?
                    };
                    match m298 {
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
                Ok(next) => { m299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f92(v1, v0, v2)
}

pub fn f65(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F94, List::<Bytes>::nil(), v0)
}

pub fn f66() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f95(f96(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f67(v0: Adt1) -> R<Adt11> {
    let m302 = f97(v0.clone());
    match m302.uncons() {
        None => {
            let m303 = f98(f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f99(v0.clone())?);
            match m303.uncons() {
                None => {
                    let v5: Adt14 = f100(v0.clone())?;
                    let v6: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f98(f101(f102(match v5.clone() {
                        Adt14::C0(h304, _, _) => {
                            h304
                        }
                    }, f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(v6.clone())), f105(f102(match v5.clone() {
                        Adt14::C0(h305, _, _) => {
                            h305
                        }
                    }, f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(v5.clone(), v6, f107(v5, f108(v0.clone(), f109(v0)?)?))?)))))
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

pub fn f68(v0: Adt1) -> Adt11 {
    if f40(f41(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h306) => {
            h306
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f110(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f40(f41(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h307) => {
                h307
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f110(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f69(v0: Adt1) -> R<Adt11> {
    let m308 = f45(v0.clone(), f111());
    match m308.uncons() {
        None => {
            let m309 = f112(v0.clone(), f111())?;
            match m309.uncons() {
                None => {
                    let m312 = f113(match v0.clone() {
                        Adt1::C0(_, h310, _) => {
                            h310
                        }
                    }, f98(f114(), f115(match v0.clone() {
                        Adt1::C0(_, h311, _) => {
                            h311
                        }
                    }, false)));
                    match m312.uncons() {
                        None => {
                            let m314 = f45(v0.clone(), f115(match v0.clone() {
                                Adt1::C0(_, h313, _) => {
                                    h313
                                }
                            }, true));
                            match m314.uncons() {
                                None => {
                                    let m315 = f45(v0.clone(), f116());
                                    match m315.uncons() {
                                        None => {
                                            let m316 = f117(v0.clone())?;
                                            match m316 {
                                                None => {
                                                    let m318 = f118(v0.clone(), f115(match v0.clone() {
                                                        Adt1::C0(_, h317, _) => {
                                                            h317
                                                        }
                                                    }, true))?;
                                                    match m318 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f119(Adt16::C0(Adt15::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f120(v0.clone())?, f121(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f122(f82(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f115(match v0 {
                                                                Adt1::C0(_, h319, _) => {
                                                                    h319
                                                                }
                                                            }, true)))?;
                                                            let (h320, _) = v13;
                                                            if h320 {
                                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h321) = v13;
                                                                    h321
                                                                }))
                                                            } else {
                                                                Ok(Adt11::C4({
                                                                    let (_, h322) = v13;
                                                                    h322
                                                                }))
                                                            }
                                                        }
                                                        Some(v12) => {
                                                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: classical choice is an assumption, and ")), List::cons(v12, List::cons(f8(Str::lit(" depends on it\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                    }
                                                }
                                                Some(v11) => {
                                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: a negated guard in a generated workflow is one command, because `if !` negates only the first command of a chain: ")), List::cons(v11, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                                }
                                            }
                                        }
                                        Some((v9, v10)) => {
                                            Ok(Adt11::C0(List::cons(v9, v10)))
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
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("register is not JSON: ")), List::cons(v3, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f70(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h323, _) => {
            h323
        }
    }, f124())) {
        Ok(Adt11::C1(List::cons(f124(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f125(f126(f124(), f127(match v0.clone() {
            Adt1::C0(_, h324, _) => {
                h324
            }
        }, f124())))?;
        let m325 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m325.uncons() {
            None => {
                let m326 = f128(v0.clone(), f129(f81(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f130(v2, v1.clone()))?;
                match m326 {
                    None => {
                        f131(v0, v1)
                    }
                    Some(v5) => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("ingest-authority ")), List::cons(v5, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f71(v0: Adt1) -> R<Adt11> {
    let m327 = f97(v0.clone());
    match m327.uncons() {
        None => {
            let m328 = f98(f45(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f132(), List::<Bytes>::nil())))), f99(v0.clone())?);
            match m328.uncons() {
                None => {
                    Ok(Adt11::C3(f133(f134(v0.clone())?, List::cons((f135(), f136()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f137()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f138()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f139()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f140()), List::cons((f8(Str::lit("docs/catalog.md")), f141(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f142(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f143(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f144()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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

pub fn f72(v0: Bytes, v1: Bytes) -> bool {
    let m333 = {
        let a330 = v0;
        let a331 = 0u64;
        let a332 = {
            let a329 = v1.clone();
            length_bytes(a329)
        };
        slice_bytes(a330, a331, a332)
    };
    match m333 {
        None => {
            false
        }
        Some(v2) => {
            let a334 = v2;
            let a335 = v1;
            equal(a334, a335)
        }
    }
}

pub fn f73(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f145(v3.clone(), v1.clone(), f146(v3, v1, false)?)
    } else {
        if f28(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f145(v3.clone(), v1.clone(), f147(v3, v1)?)
        } else {
            if f28(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f145(v3.clone(), v1.clone(), f146(v3, v1, true)?)
            } else {
                if f28(v0.clone(), f8(Str::lit("stage-encodings"))) {
                    f145(v3.clone(), v1.clone(), f148(v3, v1)?)
                } else {
                    if f28(v0.clone(), f8(Str::lit("stage-mappings"))) {
                        f145(v3.clone(), v1.clone(), f149(v3, v1, false)?)
                    } else {
                        if f28(v0, f8(Str::lit("stage-matrix"))) {
                            f145(v3.clone(), v1.clone(), f149(v3, v1, true)?)
                        } else {
                            Ok(Adt11::C4(f66()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("admit"))) {
        f150(v3, v1)
    } else {
        if f28(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f151(v3, v1)
        } else {
            if f28(v0.clone(), f8(Str::lit("ingest-authority"))) {
                f152(v3, v1)
            } else {
                if f28(v0.clone(), f8(Str::lit("ingest-index"))) {
                    f153(v3, v1)
                } else {
                    if f28(v0.clone(), f8(Str::lit("merge-gate"))) {
                        f154(v3, v1)
                    } else {
                        if f28(v0.clone(), f8(Str::lit("propose-encoding"))) {
                            let m336 = v2;
                            match m336.uncons() {
                                None => {
                                    Ok(Adt11::C4(f66()))
                                }
                                Some((v4, v5)) => {
                                    f155(v3, v1, v4, v5)
                                }
                            }
                        } else {
                            if f28(v0, f8(Str::lit("record-decision"))) {
                                f156(v3, v1)
                            } else {
                                Ok(Adt11::C4(f66()))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f75(v0: Bytes) -> R<bool> {
    f157(v0.clone(), 0u64, {
        let a337 = v0;
        length_bytes(a337)
    })
}

pub fn f76(p342: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m341 = p342;
    {
        let mut m341 = m341;
        loop {
            let step = {
                let v0 = m341;
                let m338 = v0;
                match m338.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h339, _) = v2;
                            h339
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h340) = v2;
                                h340
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m341 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Bytes) -> R<Option<Adt13>> {
    let m343 = f158(v0)?;
    match m343 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v1) => {
            Ok(f159(v1))
        }
    }
}

pub fn f78(v0: Option<Adt13>) -> Option<Bytes> {
    let m344 = v0;
    match m344 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m345 = v1;
            match m345 {
                Adt13::C0(_) => {
                    None::<Bytes>
                }
                Adt13::C1(_, _) => {
                    None::<Bytes>
                }
                Adt13::C2(v2) => {
                    Some(v2)
                }
                Adt13::C3(_) => {
                    None::<Bytes>
                }
                Adt13::C4(_) => {
                    None::<Bytes>
                }
                Adt13::C5 => {
                    None::<Bytes>
                }
                Adt13::C6(_, _) => {
                    None::<Bytes>
                }
                Adt13::C7(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f79(p351: Adt13, v1: Bytes) -> Option<Adt13> {
    let m350 = p351;
    {
        let mut m350 = m350;
        loop {
            let step = {
                let v0 = m350;
                let m346 = v0;
                match m346 {
                    Adt13::C0(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C1(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C3(_) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C4(r347) => {
                        let v5 = (*r347).clone();
                        Ok::<Adt13, Option<Adt13>>(v5)
                    }
                    Adt13::C5 => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C6(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C7(v2, r348, r349) => {
                        let v3 = (*r348).clone();
                        let v4 = (*r349).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt13, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Adt13>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m350 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m352 = v0;
    match m352.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m353 = f160(v2, v1.clone())?;
            match m353 {
                None => {
                    f80(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f80(v3, v1)?))
                }
            }
        }
    }
}

pub fn f81(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a354 = v0.clone();
        length_bytes(a354)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h355, _) = f63(Fn0::F161(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h355
    };
    let v3: u64 = {
        let (h356, _) = {
            let (_, h357) = v2.clone();
            h357
        };
        h356
    };
    let v4: List<Bytes> = {
        let (_, h358) = {
            let (_, h359) = v2;
            h359
        };
        h358
    };
    let a360 = v3;
    let a361 = v1;
    if nat_lt(a360, a361) {
        Ok(f65(List::cons(f35(v0, v3, {
            let a362 = v1;
            let a363 = v3;
            nat_sub(a362, a363)
        }), v4)))
    } else {
        Ok(f65(v4))
    }
}

pub fn f82(v0: Adt1, v1: Bytes) -> R<List<Adt13>> {
    Ok(f162(f46(v0, v1)?))
}

pub fn f83(v0: List<Adt13>, v1: Bytes) -> List<Adt13> {
    let m364 = v0;
    match m364.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f163(v2, v1.clone()), f83(v3, v1))
        }
    }
}

pub fn f84(p369: List<Bytes>, p370: List<Bytes>) -> bool {
    let m368 = (p369, p370);
    {
        let mut m368 = m368;
        loop {
            let step = {
                let (v0, v1) = m368;
                let m365 = v0;
                match m365.uncons() {
                    None => {
                        let m366 = v1;
                        match m366.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m367 = v1;
                        match m367.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f28(v4, v6) {
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
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt13>) -> List<Bytes> {
    let m371 = v0;
    match m371.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m372 = f79(v1, f8(Str::lit("std")));
            match m372 {
                None => {
                    f85(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f85(v2))
                }
            }
        }
    }
}

pub fn f86(v0: Adt13) -> R<Bytes> {
    let a373 = f164(v0)?;
    let a374 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a373, a374))
}

pub fn f87(v0: List<Adt13>) -> Adt13 {
    let m375 = v0;
    match m375.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f87(v2)))
        }
    }
}

pub fn f88(v0: Adt12) -> Bytes {
    let m376 = v0;
    match m376 {
        Adt12::C0 => {
            f8(Str::lit("VERDANT-ENGINE-AUTHORITIES-"))
        }
        Adt12::C1 => {
            f8(Str::lit("VERDANT-ENGINE-IDS-"))
        }
        Adt12::C2 => {
            f8(Str::lit("VERDANT-ENGINE-RELEASE-LEDGER-"))
        }
        Adt12::C3 => {
            f8(Str::lit("VERDANT-ENGINE-STANDARDS-LOCK-"))
        }
    }
}

pub fn f89() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f90() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f91(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt11> {
    let a378 = {
        let a377 = v2.clone();
        length_bytes(a377)
    };
    let a379 = 0u64;
    if nat_eq(a378, a379) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m380 = f165(v0.clone(), f49())?;
        match m380 {
            None => {
                if f40(f41(f168(), match v0.clone() {
                    Adt1::C0(_, _, h381) => {
                        h381
                    }
                })) {
                    Ok(Adt11::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f168()))
                } else {
                    let m382 = f45(v0.clone(), List::cons(f169(), f170(f49())?));
                    match m382.uncons() {
                        None => {
                            Ok(Adt11::C3(f171(v0.clone(), f49(), v1.clone(), v2.clone(), f172(f53(v0, f169()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt11::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f43(f166(v3.clone())?, f167(v3, v2)))
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h383, _) = v2.clone();
        h383
    };
    let v4: u64 = {
        let (h384, _) = {
            let (_, h385) = v2.clone();
            h385
        };
        h384
    };
    let v5: List<Bytes> = {
        let (_, h386) = {
            let (_, h387) = v2;
            h387
        };
        h386
    };
    let m388 = f173(v0.clone(), v3);
    match m388 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a389 = v6;
            let a390 = v1;
            if equal(a389, a390) {
                Ok(Some(({
                    let a391 = v3;
                    let a392 = 1u64;
                    nat_add(a391, a392)?
                }, ({
                    let a393 = v3;
                    let a394 = 1u64;
                    nat_add(a393, a394)?
                }, List::cons(f35(v0, v4, {
                    let a395 = v3;
                    let a396 = v4;
                    nat_sub(a395, a396)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a397 = v3;
                    let a398 = 1u64;
                    nat_add(a397, a398)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f93(v0: Fn1, p402: List<Bytes>, p403: List<Bytes>) -> List<Bytes> {
    let m401 = (p402, p403);
    {
        let mut m401 = m401;
        loop {
            let step = {
                let (v1, v2) = m401;
                let m399 = v2;
                match m399.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c400: Fn1 = v0.clone();
                            c400.apply(v1, v3)
                        }, v4))
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

pub fn f94(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m404 = f174(Fn2::F175(v1), None::<Bytes>, v0);
    match m404 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f96() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f97(v0: Adt1) -> List<Bytes> {
    if f123(match v0.clone() {
        Adt1::C0(_, h405, _) => {
            h405
        }
    }, f176()) {
        if f123(match v0 {
            Adt1::C0(_, h406, _) => {
                h406
            }
        }, f177()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f177(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f176(), List::<Bytes>::nil())
    }
}

pub fn f98(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m407 = v0;
    match m407.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f98(v3, v1))
        }
    }
}

pub fn f99(v0: Adt1) -> R<List<Bytes>> {
    Ok(f45(v0.clone(), List::cons(f178(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f109(v0)?)))))
}

pub fn f100(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f120(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f121(f82(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m408 = v0;
    match m408.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f101(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f101(v3, v1))
            }
        }
    }
}

pub fn f102(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m409 = v0;
    match m409.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f102(v3, v1)
            } else {
                List::cons(v2, f102(v3, v1))
            }
        }
    }
}

pub fn f103(v0: List<Adt13>) -> List<Bytes> {
    let m410 = v0;
    match m410.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f98(f122(f162(f79(v1, f8(Str::lit("defers"))))), f103(v2))
        }
    }
}

pub fn f104(v0: List<Adt13>) -> List<Bytes> {
    let m411 = v0;
    match m411.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m412 = f79(v1, f8(Str::lit("std")));
            match m412 {
                None => {
                    f104(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("row"))), f104(v2))
                }
            }
        }
    }
}

pub fn f105(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m413 = v0;
    match m413.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f105(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f105(v3, v1))
            }
        }
    }
}

pub fn f106(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f133(f179(v2.clone()), f180(f181(v0, v1.clone(), v2)?));
    f182(f125(f183(v1))?, v3.clone(), {
        let a415 = {
            let a414 = v3;
            length_list(a414)
        };
        let a416 = 1u64;
        nat_add(a415, a416)?
    })
}

pub fn f107(v0: Adt14, v1: List<Adt13>) -> List<Adt13> {
    let m417 = v1;
    match m417.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            if if f184(v2.clone(), f8(Str::lit("subject"))) {
                f185(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f107(v0, v3))
            } else {
                f107(v0, v3)
            }
        }
    }
}

pub fn f108(v0: Adt1, v1: List<Bytes>) -> R<List<Adt13>> {
    let m418 = v1;
    match m418.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v2, v3)) => {
            let m419 = f186(f81(f53(v0.clone(), v2.clone()))?, f72(v2, f177()))?;
            match m419 {
                None => {
                    f108(v0, v3)
                }
                Some(v4) => {
                    Ok(f187(v4, f108(v0, v3)?))
                }
            }
        }
    }
}

pub fn f109(v0: Adt1) -> R<List<Bytes>> {
    Ok(f98(f125(f188(f176(), f127(match v0.clone() {
        Adt1::C0(_, h420, _) => {
            h420
        }
    }, f176())))?, f125(f188(f177(), f127(match v0 {
        Adt1::C0(_, h421, _) => {
            h421
        }
    }, f177())))?))
}

pub fn f110(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f111() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f178(), List::<Bytes>::nil())))))))
}

pub fn f112(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m422 = v1;
    match m422.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f56(v0.clone(), v2.clone()) {
                f189(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m423 = f46(v0.clone(), v2.clone())?;
                match m423 {
                    None => {
                        Ok(List::cons(v2, f112(v0, v3)?))
                    }
                    Some(_) => {
                        f112(v0, v3)
                    }
                }
            } else {
                f112(v0, v3)
            }
        }
    }
}

pub fn f113(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m424 = v1;
    match m424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f123(v0.clone(), v2.clone()) {
                f113(v0, v3)
            } else {
                List::cons(v2, f113(v0, v3))
            }
        }
    }
}

pub fn f114() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f115(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m425 = v0;
    match m425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f98(f190({
                let (h426, _) = v2;
                h426
            }, {
                let (_, h427) = v2;
                h427
            }, v1), f115(v3, v1))
        }
    }
}

pub fn f116() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f117(v0: Adt1) -> R<Option<Bytes>> {
    f191(f192(v0, f116())?)
}

pub fn f118(v0: Adt1, p431: List<Bytes>) -> R<Option<Bytes>> {
    let m430 = p431;
    {
        let mut m430 = m430;
        loop {
            let step = {
                let v1 = m430;
                let m428 = v1;
                match m428.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m429 = f193(f53(v0.clone(), v2.clone()), f194())?;
                        match m429 {
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
                Ok(next) => { m430 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h432, _, _, _, _) => {
            h432
        }
    };
    let v2: Adt17 = f195(v1.clone())?;
    let v3: List<Bytes> = f196(v1.clone());
    let v4: List<Bytes> = f121(match v0.clone() {
        Adt16::C0(_, _, h433, _, _) => {
            h433
        }
    }, f8(Str::lit("path")));
    let m435 = f197(v2, List::<u64>::nil(), f198(v1.clone(), match v0.clone() {
        Adt16::C0(_, h434, _, _, _) => {
            h434
        }
    }, 0u64)?);
    match m435 {
        None => {
            if f200(f201(match v0.clone() {
                Adt16::C0(_, _, _, _, h437) => {
                    h437
                }
            }, f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h438) => {
                    h438
                }
            }, v4.clone()))?, f201(v4.clone(), f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h439) => {
                    h439
                }
            }, v4.clone()))?, f202(match v0.clone() {
                Adt16::C0(_, _, h440, _, _) => {
                    h440
                }
            })) {
                let m444 = f203(f201(match v0.clone() {
                    Adt16::C0(_, _, _, h441, _) => {
                        h441
                    }
                }, f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h442, _) => {
                        h442
                    }
                }))?, f201(v3.clone(), f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h443, _) => {
                        h443
                    }
                }))?);
                match m444 {
                    None => {
                        let m447 = f204(f121(match v0 {
                            Adt16::C0(_, h446, _, _, _) => {
                                h446
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m447 {
                            None => {
                                let m448 = f204(v4, List::<Bytes>::nil());
                                match m448 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f95(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f199(f98(v3, match v0 {
                            Adt16::C0(_, _, _, h445, _) => {
                                h445
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f199(match v1 {
                Adt15::C0(_, _, _, _, h436) => {
                    h436
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f120(v0: Adt1) -> R<List<Bytes>> {
    let m449 = f205(f53(v0, f178()))?;
    match m449.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f206(v2))
        }
    }
}

pub fn f121(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m450 = v0;
    match m450.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f47(v2, v1.clone()), f121(v3, v1))
        }
    }
}

pub fn f122(v0: List<Adt13>) -> List<Bytes> {
    let m451 = v0;
    match m451.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m452 = f78(Some(v1));
            match m452 {
                None => {
                    f122(v2)
                }
                Some(v3) => {
                    List::cons(v3, f122(v2))
                }
            }
        }
    }
}

pub fn f123(p456: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m455 = p456;
    {
        let mut m455 = m455;
        loop {
            let step = {
                let v0 = m455;
                let m453 = v0;
                match m453.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h454, _) = v2;
                            h454
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
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

pub fn f124() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f125(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f207(f93(Fn1::F208, List::<Bytes>::nil(), f209(v0)?)))
}

pub fn f126(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m457 = v1;
    match m457.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f189(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f210(v0.clone(), v2), f126(v0, v3))
            } else {
                f126(v0, v3)
            }
        }
    }
}

pub fn f127(p462: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m461 = p462;
    {
        let mut m461 = m461;
        loop {
            let step = {
                let v0 = m461;
                let m458 = v0;
                match m458.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h459, _) = v2;
                            h459
                        }, v1.clone()) {
                            let (_, h460) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h460)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: Adt1, v1: List<List<Bytes>>, p465: List<Bytes>) -> R<Option<Bytes>> {
    let m464 = p465;
    {
        let mut m464 = m464;
        loop {
            let step = {
                let v2 = m464;
                let m463 = v2;
                match m463.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f211(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(p468: List<Bytes>, p469: List<Bytes>, p470: List<List<Bytes>>) -> List<List<Bytes>> {
    let m467 = (p468, (p469, p470));
    {
        let mut m467 = m467;
        loop {
            let step = {
                let (v0, (v1, v2)) = m467;
                let m466 = v0;
                match m466.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f212(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f28(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f212(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m471 = v0;
    match m471.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f130(v3, v1)
            } else {
                List::cons(v2, f130(v3, v1))
            }
        }
    }
}

pub fn f131(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h472, _) => {
            h472
        }
    }, f213())) {
        Ok(Adt11::C1(List::cons(f213(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f130(f125(f126(f213(), f127(match v0.clone() {
            Adt1::C0(_, h473, _) => {
                h473
            }
        }, f213())))?, v1.clone());
        let m474 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m474.uncons() {
            None => {
                let v5: List<Bytes> = f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m475 = f45(v0.clone(), f214(v0.clone(), v2.clone(), v5.clone())?);
                match m475.uncons() {
                    None => {
                        let m476 = f215(v0.clone(), v2, v5)?;
                        match m476 {
                            None => {
                                f216(v0, v1)
                            }
                            Some(v8) => {
                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("admit-encoding ")), List::cons(v8, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f132() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f133(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m477 = v0;
    match m477.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f133(v3, v1))
        }
    }
}

pub fn f134(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m478 = f82(v0.clone(), f132())?;
    match m478.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f217(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f135() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f136() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f137() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f218(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f138() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f218(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f139() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f140() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f141(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f100(v0.clone())?;
    let v2: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f107(v1.clone(), f108(v0.clone(), f109(v0.clone())?)?);
    let v4: List<List<Bytes>> = f219(f205(f53(v0, f178()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f220(v4.clone(), f206(v4), f181(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f106(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f142(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f221(f100(v0.clone())?, f108(v0.clone(), f109(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f143(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f222(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f223(f82(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f144() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f145(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m479 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m479.uncons() {
        None => {
            if f224(f81(f53(v0, v1))?)? {
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

pub fn f146(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m480 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m480.uncons() {
        None => {
            let v5: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
            if f40(f189(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f225(v5)?
                } else {
                    f226(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f210(f227(), f228(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f210(f227(), f228(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f229()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f147(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m481 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m481.uncons() {
        None => {
            let m482 = f230(f81(f53(v0.clone(), v1.clone()))?)?;
            match m482 {
                None => {
                    Ok(Adt11::C4(f231()))
                }
                Some(v4) => {
                    let m483 = f232(f81(f53(v0, v1))?)?;
                    match m483 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f233(v5)))
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

pub fn f148(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m484 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m484.uncons() {
        None => {
            let m485 = f234(f81(f53(v0, v1))?)?;
            match m485 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f149(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m486 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m486.uncons() {
        None => {
            let v5: Bytes = f210(if v2 {
                f177()
            } else {
                f176()
            }, f228(v1.clone())?);
            if f40(f189(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m487 = f186(f81(f53(v0.clone(), v1.clone()))?, v2)?;
                match m487 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a489 = {
                            let a488 = v6;
                            length_list(a488)
                        };
                        let a490 = 0u64;
                        if nat_eq(a489, a490) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f53(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f150(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m491 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m491.uncons() {
        None => {
            let v4: List<Bytes> = f81(f53(v0.clone(), v1))?;
            let m492 = f235(v4.clone(), f8(Str::lit("requirement")))?;
            match m492 {
                None => {
                    Ok(Adt11::C4(f229()))
                }
                Some(v5) => {
                    let m493 = f235(v4, f8(Str::lit("admission")))?;
                    match m493 {
                        None => {
                            Ok(Adt11::C4(f229()))
                        }
                        Some(v6) => {
                            let v7: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f98(f121(v5.clone(), f8(Str::lit("id"))), f121(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f125(f98(f121(v7.clone(), f8(Str::lit("id"))), f121(v8.clone(), f8(Str::lit("construct")))))?;
                            let m494 = f236(f104(v6.clone()), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m494 {
                                None => {
                                    let a496 = {
                                        let a495 = v9.clone();
                                        length_list(a495)
                                    };
                                    let a497 = 0u64;
                                    if nat_eq(a496, a497) {
                                        Ok(Adt11::C4(f229()))
                                    } else {
                                        if f40({
                                            let a500 = {
                                                let a498 = f125(f98(v9.clone(), v10.clone()))?;
                                                length_list(a498)
                                            };
                                            let a501 = {
                                                let a499 = f98(v9.clone(), v10);
                                                length_list(a499)
                                            };
                                            nat_eq(a500, a501)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f238(f8(Str::lit("model/requirements.json")), f187(v7, v5))?, List::cons(f238(f8(Str::lit("model/admissions.json")), f187(v8, v6))?, List::cons(f239(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f95(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f237(v11))
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

pub fn f151(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m502 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m502.uncons() {
        None => {
            let m503 = f240(v0.clone(), v1.clone())?;
            match m503 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m504 = f241(v4.clone(), f8(Str::lit("schema")));
                    match m504 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f242(v0, f243(v5.clone())?, v5, f244(v4.clone(), f8(Str::lit("name"))), f245(f246(f241(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f152(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m505 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m505.uncons() {
        None => {
            let m506 = f46(v0.clone(), v1.clone())?;
            match m506 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f40(f28(f47(v4.clone(), f8(Str::lit("schema"))), f247())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt13 = {
                            let m507 = f79(v4.clone(), f8(Str::lit("source")));
                            match m507 {
                                None => {
                                    Adt13::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f78(f79(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f210(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f210(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m508 = v8;
                        match m508 {
                            None => {
                                f248(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f40(f28(f47(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f248(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f210(v9, f228(f47(v7.clone(), f8(Str::lit("url"))))?);
                                    if f40(f41(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h509) => {
                                            h509
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f47(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m510 = f45(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m510.uncons() {
                                            None => {
                                                if f28(f249(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f248(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f153(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m511 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m511.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f205(v4.clone())?;
            if f40(f84(f250(v5.clone()), f251())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f252(Str::lit("id"), v6.clone())?, List::cons(f252(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f252(Str::lit("artifact"), f178())?, List::cons(f252(Str::lit("checksum"), f249(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f178(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f253(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f239(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f95(f206(f219(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f154(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m512 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m512.uncons() {
        None => {
            let m513 = f46(v0, v1)?;
            match m513 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f254(f162(Some(v4))) {
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

pub fn f155(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m514 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m514.uncons() {
        None => {
            let m515 = f240(v0.clone(), v1.clone())?;
            match m515 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f242(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f156(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m516 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f132(), List::<Bytes>::nil())));
    match m516.uncons() {
        None => {
            let m517 = f255(f81(f53(v0.clone(), v1))?)?;
            match m517 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt13> = f82(v0, f132())?;
                    let v6: List<Bytes> = f121(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f125(f121(v5.clone(), f8(Str::lit("at"))))?;
                    let a519 = {
                        let a518 = v6.clone();
                        length_list(a518)
                    };
                    let a520 = 0u64;
                    if if nat_eq(a519, a520) {
                        true
                    } else {
                        f40({
                            let a523 = {
                                let a521 = f125(f98(v6.clone(), v7.clone()))?;
                                length_list(a521)
                            };
                            let a524 = {
                                let a522 = f98(v6.clone(), v7);
                                length_list(a522)
                            };
                            nat_eq(a523, a524)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f238(f132(), f187(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f95(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f157(v0: Bytes, p578: u64, p579: u64) -> R<bool> {
    let m577 = (p578, p579);
    {
        let mut m577 = m577;
        loop {
            let step = {
                let (v1, v2) = m577;
                let m525 = v2;
                if m525 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m525 - 1;
                    let m528 = {
                        let a526 = v0.clone();
                        let a527 = v1;
                        index_bytes(a526, a527)
                    };
                    match m528 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f256({
                                let v5: u8 = v4;
                                let a573 = {
                                    let a549 = {
                                        let a537 = {
                                            let a531 = {
                                                let a529 = v5;
                                                let a530 = 128u8;
                                                fixed_u8::bit_and(a529, a530)
                                            };
                                            let a532 = 128u8;
                                            if equal(a531, a532) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a538 = {
                                            let a535 = {
                                                let a533 = v5;
                                                let a534 = 64u8;
                                                fixed_u8::bit_and(a533, a534)
                                            };
                                            let a536 = 64u8;
                                            if equal(a535, a536) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a537, a538)?
                                    };
                                    let a550 = {
                                        let a547 = {
                                            let a541 = {
                                                let a539 = v5;
                                                let a540 = 32u8;
                                                fixed_u8::bit_and(a539, a540)
                                            };
                                            let a542 = 32u8;
                                            if equal(a541, a542) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a548 = {
                                            let a545 = {
                                                let a543 = v5;
                                                let a544 = 16u8;
                                                fixed_u8::bit_and(a543, a544)
                                            };
                                            let a546 = 16u8;
                                            if equal(a545, a546) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a547, a548)?
                                    };
                                    nat_add(a549, a550)?
                                };
                                let a574 = {
                                    let a571 = {
                                        let a559 = {
                                            let a553 = {
                                                let a551 = v5;
                                                let a552 = 8u8;
                                                fixed_u8::bit_and(a551, a552)
                                            };
                                            let a554 = 8u8;
                                            if equal(a553, a554) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a560 = {
                                            let a557 = {
                                                let a555 = v5;
                                                let a556 = 4u8;
                                                fixed_u8::bit_and(a555, a556)
                                            };
                                            let a558 = 4u8;
                                            if equal(a557, a558) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a559, a560)?
                                    };
                                    let a572 = {
                                        let a569 = {
                                            let a563 = {
                                                let a561 = v5;
                                                let a562 = 2u8;
                                                fixed_u8::bit_and(a561, a562)
                                            };
                                            let a564 = 2u8;
                                            if equal(a563, a564) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a570 = {
                                            let a567 = {
                                                let a565 = v5;
                                                let a566 = 1u8;
                                                fixed_u8::bit_and(a565, a566)
                                            };
                                            let a568 = 1u8;
                                            if equal(a567, a568) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a569, a570)?
                                    };
                                    nat_add(a571, a572)?
                                };
                                nat_add(a573, a574)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a575 = v1;
                                    let a576 = 1u64;
                                    nat_add(a575, a576)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
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

pub fn f158(v0: Bytes) -> R<Option<Adt18>> {
    let v1: Bytes = f257(v0)?;
    let m583 = f258(v1.clone(), 0u64, {
        let a581 = {
            let a580 = v1.clone();
            length_bytes(a580)
        };
        let a582 = 2u64;
        nat_add(a581, a582)?
    }, 0u64)?;
    match m583 {
        None => {
            Ok(None::<Adt18>)
        }
        Some(v2) => {
            let a586 = {
                let (_, h584) = v2;
                h584
            };
            let a587 = {
                let a585 = v1;
                length_bytes(a585)
            };
            if nat_eq(a586, a587) {
                Ok(Some({
                    let (h588, _) = v2;
                    h588
                }))
            } else {
                Ok(None::<Adt18>)
            }
        }
    }
}

pub fn f159(v0: Adt18) -> Option<Adt13> {
    let m589 = v0;
    match m589 {
        Adt18::C0(v1) => {
            Some(Adt13::C0(v1))
        }
        Adt18::C1(v2, v3) => {
            Some(Adt13::C1(v2, v3))
        }
        Adt18::C2(_, _, _) => {
            None::<Adt13>
        }
        Adt18::C3(v7) => {
            Some(Adt13::C2(v7))
        }
        Adt18::C4(r596) => {
            let v8 = (*r596).clone();
            let m590 = f159(v8);
            match m590 {
                None => {
                    None::<Adt13>
                }
                Some(v9) => {
                    Some(Adt13::C3(std::rc::Rc::new(v9)))
                }
            }
        }
        Adt18::C5(r597) => {
            let v10 = (*r597).clone();
            let m591 = f159(v10);
            match m591 {
                None => {
                    None::<Adt13>
                }
                Some(v11) => {
                    Some(Adt13::C4(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt18::C6 => {
            Some(Adt13::C5)
        }
        Adt18::C7(r598, r599) => {
            let v12 = (*r598).clone();
            let v13 = (*r599).clone();
            let m592 = f159(v12);
            match m592 {
                None => {
                    None::<Adt13>
                }
                Some(v14) => {
                    let m593 = f159(v13);
                    match m593 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v15) => {
                            Some(Adt13::C6(std::rc::Rc::new(v14), std::rc::Rc::new(v15)))
                        }
                    }
                }
            }
        }
        Adt18::C8(v16, r600, r601) => {
            let v17 = (*r600).clone();
            let v18 = (*r601).clone();
            let m594 = f159(v17);
            match m594 {
                None => {
                    None::<Adt13>
                }
                Some(v19) => {
                    let m595 = f159(v18);
                    match m595 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v20) => {
                            Some(Adt13::C7(v16, std::rc::Rc::new(v19), std::rc::Rc::new(v20)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f160(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), v1) {
        let m602 = f33(v0, 34u8)?;
        match m602.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m603 = v3;
                match m603.uncons() {
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

pub fn f161(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f259(v0, v1)
}

pub fn f162(v0: Option<Adt13>) -> List<Adt13> {
    let m604 = v0;
    match m604 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f260(v1)
        }
    }
}

pub fn f163(v0: Adt13, v1: Bytes) -> Adt13 {
    let m605 = v0;
    match m605 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r606) => {
            let v10 = (*r606).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r607) => {
            let v5 = (*r607).clone();
            Adt13::C4(std::rc::Rc::new(f163(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r608, r609) => {
            let v11 = (*r608).clone();
            let v12 = (*r609).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r610, r611) => {
            let v3 = (*r610).clone();
            let v4 = (*r611).clone();
            if f28(v2.clone(), f8(Str::lit("std"))) {
                Adt13::C7(v2, std::rc::Rc::new(f261(v3, v1.clone())), std::rc::Rc::new(f163(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f163(v4, v1)))
            }
        }
    }
}

pub fn f164(v0: Adt13) -> R<Bytes> {
    let m612 = v0;
    match m612 {
        Adt13::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt13::C1(v2, v3) => {
            f262(v2, v3)
        }
        Adt13::C2(v4) => {
            f263(v4)
        }
        Adt13::C3(r613) => {
            let v5 = (*r613).clone();
            Ok(f264(f265(v5)?))
        }
        Adt13::C4(r614) => {
            let v6 = (*r614).clone();
            f266(f267(v6)?)
        }
        Adt13::C5 => {
            Ok(Bytes::lit(&[]))
        }
        Adt13::C6(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt13::C7(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f165(v0: Adt1, p618: List<Adt12>) -> R<Option<Adt12>> {
    let m617 = p618;
    {
        let mut m617 = m617;
        loop {
            let step = {
                let v1 = m617;
                let m615 = v1;
                match m615.uncons() {
                    None => {
                        Err::<List<Adt12>, R<Option<Adt12>>>(Ok(None::<Adt12>))
                    }
                    Some((v2, v3)) => {
                        if f41(f166(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h616) => {
                                h616
                            }
                        }) {
                            Ok::<List<Adt12>, R<Option<Adt12>>>(v3)
                        } else {
                            Err::<List<Adt12>, R<Option<Adt12>>>(Ok(Some(v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Adt12) -> R<Bytes> {
    Ok(f210(f268(), f228(f269(v0))?))
}

pub fn f167(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f269(v0), List::<Bytes>::nil())))))
}

pub fn f168() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f169() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f170(v0: List<Adt12>) -> R<List<Bytes>> {
    let m619 = v0;
    match m619.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f166(v1)?, f170(v2)?))
        }
    }
}

pub fn f171(v0: Adt1, v1: List<Adt12>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m620 = v1;
    match m620.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f57(f58(v5.clone(), f50(v2.clone()))), f270(v5.clone(), v2.clone(), v3.clone(), f249(f53(v0.clone(), f166(v5)?))?, v4.clone())?), f171(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f172(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f271(v0.clone())?;
    let v2: u64 = f272(v0.clone(), v1)?;
    Ok(f35(v0, v2, {
        let a621 = v1;
        let a622 = v2;
        nat_sub(a621, a622)
    }))
}

pub fn f173(v0: Bytes, v1: u64) -> Option<u8> {
    let a623 = v0;
    let a624 = v1;
    index_bytes(a623, a624)
}

pub fn f174(v0: Fn2, p628: Option<Bytes>, p629: List<Bytes>) -> Option<Bytes> {
    let m627 = (p628, p629);
    {
        let mut m627 = m627;
        loop {
            let step = {
                let (v1, v2) = m627;
                let m625 = v2;
                match m625.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c626: Fn2 = v0.clone();
                            c626.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m627 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m630 = v1;
    match m630 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a633 = {
                    let a631 = v3;
                    let a632 = v0;
                    append_bytes(a631, a632)
                };
                let a634 = v2;
                append_bytes(a633, a634)
            })
        }
    }
}

pub fn f176() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f177() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f178() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f179(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m635 = v0;
    match m635.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f47(v1.clone(), f8(Str::lit("subject"))), f47(v1, f8(Str::lit("object")))), f179(v2))
        }
    }
}

pub fn f180(v0: List<Adt19>) -> List<(Bytes, Bytes)> {
    let m636 = v0;
    match m636.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt19::C0(h637, _, _) => {
                    h637
                }
            }, match v1 {
                Adt19::C0(_, _, h638) => {
                    h638
                }
            }), f180(v2))
        }
    }
}

pub fn f181(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Adt19>> {
    let v3: List<(Bytes, Bytes)> = f273(v1);
    let v4: List<(Bytes, Bytes)> = f274(v0, v2);
    f275(v3.clone(), f133(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f182(p645: List<Bytes>, v1: List<(Bytes, Bytes)>, p646: u64) -> R<List<Bytes>> {
    let m644 = (p645, p646);
    {
        let mut m644 = m644;
        loop {
            let step = {
                let (v0, v2) = m644;
                let m639 = v2;
                if m639 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m639 - 1;
                    let v4: List<Bytes> = f125(f98(v0.clone(), f276(v0.clone(), v1.clone())))?;
                    let a642 = {
                        let a640 = v4.clone();
                        length_list(a640)
                    };
                    let a643 = {
                        let a641 = v0.clone();
                        length_list(a641)
                    };
                    if nat_eq(a642, a643) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
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

pub fn f183(v0: List<Adt13>) -> List<Bytes> {
    let m647 = v0;
    match m647.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m648 = f79(v1, f8(Str::lit("std")));
            match m648 {
                None => {
                    f183(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3.clone(), f8(Str::lit("authority"))), if f184(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f47(v3, f8(Str::lit("row"))), f183(v2))
                    } else {
                        f183(v2)
                    })
                }
            }
        }
    }
}

pub fn f184(v0: Adt13, v1: Bytes) -> bool {
    f40({
        let a650 = {
            let a649 = f47(v0, v1);
            length_bytes(a649)
        };
        let a651 = 0u64;
        nat_eq(a650, a651)
    })
}

pub fn f185(v0: Adt14, v1: Adt13) -> bool {
    if f277(v0.clone(), v1.clone(), f47(v1.clone(), f8(Str::lit("subject")))) {
        f277(v0, v1.clone(), f47(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f186(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt13>>> {
    let m652 = v0;
    match m652.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f172(v2)?;
            let a654 = {
                let a653 = v4.clone();
                length_bytes(a653)
            };
            let a655 = 0u64;
            if nat_eq(a654, a655) {
                f186(v3, v1)
            } else {
                let m656 = f77(v4)?;
                match m656 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f278(v5.clone())
                        } else {
                            f279(v5.clone())
                        } {
                            let m657 = f186(v3, v1)?;
                            match m657 {
                                None => {
                                    Ok(None::<List<Adt13>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons(v5, v6)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt13>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f187(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m658 = v0;
    match m658.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f187(v3, v1))
        }
    }
}

pub fn f188(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m659 = v1;
    match m659.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f189(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f210(v0.clone(), v2), f188(v0, v3))
            } else {
                f188(v0, v3)
            }
        }
    }
}

pub fn f189(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a660 = v0.clone();
        length_bytes(a660)
    };
    let v3: u64 = {
        let a661 = v1.clone();
        length_bytes(a661)
    };
    let a662 = v3;
    let a663 = v2;
    if nat_le(a662, a663) {
        let m669 = {
            let a666 = v0;
            let a667 = {
                let a664 = v2;
                let a665 = v3;
                nat_sub(a664, a665)
            };
            let a668 = v3;
            slice_bytes(a666, a667, a668)
        };
        match m669 {
            None => {
                false
            }
            Some(v4) => {
                let a670 = v4;
                let a671 = v1;
                equal(a670, a671)
            }
        }
    } else {
        false
    }
}

pub fn f190(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m672 = v1;
    match m672.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f280(v3.clone())
            } else {
                f40(f280(v3.clone()))
            } {
                List::cons(f210(v0.clone(), v3), f190(v0, v4, v2))
            } else {
                f190(v0, v4, v2)
            }
        }
    }
}

pub fn f191(p675: List<Bytes>) -> R<Option<Bytes>> {
    let m674 = p675;
    {
        let mut m674 = m674;
        loop {
            let step = {
                let v0 = m674;
                let m673 = v0;
                match m673.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f281(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f172(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m676 = v1;
    match m676.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f81(f53(v0.clone(), v2))?, f192(v0, v3)?))
        }
    }
}

pub fn f193(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a677 = v0.clone();
        length_bytes(a677)
    };
    let v3: u64 = {
        let a678 = v1.clone();
        length_bytes(a678)
    };
    let v4: u64 = {
        let (h679, _) = f282(Fn3::F283(v1.clone(), v3, v0.clone(), v2), {
            let a680 = v2;
            let a681 = 1u64;
            nat_add(a680, a681)?
        }, 0u64)?;
        h679
    };
    if f284(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f194() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f195(v0: Adt15) -> R<Adt17> {
    Ok(Adt17::C0(f285(match v0.clone() {
        Adt15::C0(h682, _, _, _, _) => {
            h682
        }
    }, 0u64)?, f285(match v0.clone() {
        Adt15::C0(_, h683, _, _, _) => {
            h683
        }
    }, 0u64)?, f285(match v0.clone() {
        Adt15::C0(_, _, h684, _, _) => {
            h684
        }
    }, 0u64)?, f285(match v0 {
        Adt15::C0(_, _, _, h685, _) => {
            h685
        }
    }, 0u64)?))
}

pub fn f196(v0: Adt15) -> List<Bytes> {
    f98(match v0.clone() {
        Adt15::C0(h686, _, _, _, _) => {
            h686
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, h687, _, _) => {
            h687
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, _, h688, _) => {
            h688
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h689) => {
            h689
        }
    })))
}

pub fn f197(v0: Adt17, p695: List<u64>, p696: List<Adt20>) -> Option<u64> {
    let m694 = (p695, p696);
    {
        let mut m694 = m694;
        loop {
            let step = {
                let (v1, v2) = m694;
                let m690 = v2;
                match m690.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f286(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h691) => {
                                h691
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h692, _) => {
                                    h692
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h693, _) => {
                                    h693
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Adt15, v1: List<Adt13>, v2: u64) -> R<List<Adt20>> {
    let m697 = v1;
    match m697.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f287(v0.clone(), v3)?), f198(v0, v4, {
                let a698 = v2;
                let a699 = 1u64;
                nat_add(a698, a699)?
            })?))
        }
    }
}

pub fn f199(p703: List<Bytes>, p704: u64) -> Bytes {
    let m702 = (p703, p704);
    {
        let mut m702 = m702;
        loop {
            let step = {
                let (v0, v1) = m702;
                let m700 = v0;
                match m700.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m701 = v1;
                        if m701 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m701 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f288(v0.clone(), v1.clone()) {
        if f288(v1, v0) {
            f289(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f201(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m705 = v0;
    match m705.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f290(v2, v1.clone())?, f201(v3, v1)?))
        }
    }
}

pub fn f202(v0: List<Adt13>) -> List<u64> {
    let m706 = v0;
    match m706.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f291(v1), f202(v2))
        }
    }
}

pub fn f203(p709: List<u64>, v1: List<u64>) -> Option<u64> {
    let m708 = p709;
    {
        let mut m708 = m708;
        loop {
            let step = {
                let v0 = m708;
                let m707 = v0;
                match m707.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f292(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(p712: List<Bytes>, p713: List<Bytes>) -> Option<Bytes> {
    let m711 = (p712, p713);
    {
        let mut m711 = m711;
        loop {
            let step = {
                let (v0, v1) = m711;
                let m710 = v0;
                match m710.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f41(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt23 = f293(v0.clone(), 0u64, {
        let a714 = v0;
        length_bytes(a714)
    }, Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt23 = {
        let a717 = {
            let a716 = match v1.clone() {
                Adt23::C0(_, _, h715, _, _) => {
                    h715
                }
            };
            length_bytes(a716)
        };
        let a718 = 0u64;
        if if nat_eq(a717, a718) {
            let m720 = match v1.clone() {
                Adt23::C0(_, _, _, h719, _) => {
                    h719
                }
            };
            match m720.uncons() {
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
            f294(v1)
        }
    };
    Ok(f295(match v4 {
        Adt23::C0(_, _, _, _, h721) => {
            h721
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f206(v0: List<List<Bytes>>) -> List<Bytes> {
    let m722 = v0;
    match m722.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m723 = v1;
            match m723.uncons() {
                None => {
                    f206(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f206(v2))
                }
            }
        }
    }
}

pub fn f207(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F296, List::<Bytes>::nil(), v0)
}

pub fn f208(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f297(v0, v1)
}

pub fn f209(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f298(f299(f300(v0))?))
}

pub fn f210(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f211(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m724 = f46(v0, v2)?;
    match m724 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f301(v1, f47(v3.clone(), f8(Str::lit("id"))))?;
            let m725 = v4.clone();
            match m725.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m727 = f78(f79({
                        let m726 = f79(v3.clone(), f8(Str::lit("source")));
                        match m726 {
                            None => {
                                Adt13::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m727 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f40(f41(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f302(f303(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f212(p730: List<Bytes>, p731: List<Bytes>) -> List<Bytes> {
    let m729 = (p730, p731);
    {
        let mut m729 = m729;
        loop {
            let step = {
                let (v0, v1) = m729;
                let m728 = v0;
                match m728.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f214(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m732 = v1;
    match m732.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt18 = f304(v0.clone(), v3)?;
            if f41(f305(f244(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f306(v5)?, f214(v0, v4, v2)?))
            } else {
                f214(v0, v4, v2)
            }
        }
    }
}

pub fn f215(v0: Adt1, p736: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m735 = p736;
    {
        let mut m735 = m735;
        loop {
            let step = {
                let v1 = m735;
                let m733 = v1;
                match m733.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt18 = f304(v0.clone(), v3.clone())?;
                        if if f41(f305(f244(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f28(f243({
                                let m734 = f241(v5.clone(), f8(Str::lit("schema")));
                                match m734 {
                                    None => {
                                        Adt18::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f53(v0.clone(), f306(v5)?))
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
                Ok(next) => { m735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h737, _) => {
            h737
        }
    }, f227())) {
        Ok(Adt11::C1(List::cons(f227(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f130(f125(f307(f127(match v0.clone() {
            Adt1::C0(_, h738, _) => {
                h738
            }
        }, f227())))?, v1);
        let m739 = f45(v0.clone(), List::cons(f132(), List::cons(f178(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m739.uncons() {
            None => {
                let v5: (bool, Bytes) = f308(v0.clone(), v2)?;
                let (h740, _) = v5;
                if h740 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h741) = v5;
                        h741
                    }))
                } else {
                    let m742 = f97(v0.clone());
                    match m742.uncons() {
                        None => {
                            let m743 = f99(v0.clone())?;
                            match m743.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h744) = v5;
                                        h744
                                    }, List::cons(f7(f125(f309(f100(v0.clone())?, f108(v0.clone(), f109(v0.clone())?)?))?), List::cons(f310(v0)?, List::<Bytes>::nil()))))))
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

pub fn f217(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f311(f82(v0, f132())?)?)))
}

pub fn f218() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f219(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m745 = v0;
    match m745.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f220(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt19>, v3: List<Adt13>, v4: List<Bytes>) -> List<Bytes> {
    let m746 = v1;
    match m746.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f98(f312(v5.clone(), v2.clone()), f313(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f314(v0.clone(), v5.clone()), List::cons(if f41(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a748 = {
                    let a747 = v7.clone();
                    length_list(a747)
                };
                let a749 = 0u64;
                if nat_eq(a748, a749) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f95(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f220(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f221(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m750 = v1;
    match m750.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f184(v2.clone(), f8(Str::lit("domain"))) {
                f315(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f47(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m751 = f79(v2.clone(), f8(Str::lit("role")));
                    match m751 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f47(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f47(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f316(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m752 = f79(v2.clone(), f8(Str::lit("origin")));
                    match m752 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f47(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m753 = f79(v2, f8(Str::lit("evidence")));
                    match m753 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f47(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f221(v0, v3))
            } else {
                f221(v0, v3)
            }
        }
    }
}

pub fn f222(v0: List<Adt13>) -> List<Bytes> {
    let m754 = v0;
    match m754.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f47(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f47(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f47(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f222(v2))
        }
    }
}

pub fn f223(v0: List<Adt13>) -> List<Bytes> {
    let m755 = v0;
    match m755.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f47(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f317(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f223(v2))
        }
    }
}

pub fn f224(p761: List<Bytes>) -> R<bool> {
    let m760 = p761;
    {
        let mut m760 = m760;
        loop {
            let step = {
                let v0 = m760;
                let m756 = v0;
                match m756.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a758 = {
                            let a757 = f172(v1)?;
                            length_bytes(a757)
                        };
                        let a759 = 0u64;
                        if nat_eq(a758, a759) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m760 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: List<Bytes>) -> R<bool> {
    if f318(f255(v0.clone())?) {
        Ok(f40({
            let a763 = {
                let a762 = f319(f255(v0)?);
                length_list(a762)
            };
            let a764 = 0u64;
            nat_eq(a763, a764)
        }))
    } else {
        Ok(false)
    }
}

pub fn f226(v0: List<Bytes>) -> R<bool> {
    if f318(f235(v0.clone(), f8(Str::lit("requirement")))?) {
        if f318(f235(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f40({
                let a766 = {
                    let a765 = f187(f319(f235(v0.clone(), f8(Str::lit("requirement")))?), f319(f235(v0, f8(Str::lit("admission")))?));
                    length_list(a765)
                };
                let a767 = 0u64;
                nat_eq(a766, a767)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f227() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f228(v0: Bytes) -> R<Bytes> {
    Ok(f34(f33(v0, 47u8)?))
}

pub fn f229() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f230(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m768 = v0;
    match m768.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f172(v1)?;
            let a770 = {
                let a769 = v3.clone();
                length_bytes(a769)
            };
            let a771 = 0u64;
            if nat_eq(a770, a771) {
                f230(v2)
            } else {
                let m772 = f77(v3.clone())?;
                match m772 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        if f320(f47(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m773 = f230(v2)?;
                            match m773 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f321(v5), {
                                        let a774 = v3;
                                        let a775 = f8(Str::lit("\u{a}"));
                                        append_bytes(a774, a775)
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

pub fn f231() -> Bytes {
    let a776 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a776)
}

pub fn f232(p781: List<Bytes>) -> R<Option<Bytes>> {
    let m780 = p781;
    {
        let mut m780 = m780;
        loop {
            let step = {
                let v0 = m780;
                let m777 = v0;
                match m777.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m778 = f77(f172(v1)?)?;
                        match m778 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m779 = f79(v3.clone(), f8(Str::lit("source")));
                                match m779 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f322(f47(v4.clone(), f8(Str::lit("redistribution"))), f47(v4, f8(Str::lit("url"))))? {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f47(v3, f8(Str::lit("id"))))))
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
                Ok(next) => { m780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233(v0: Bytes) -> Bytes {
    let a786 = {
        let a782 = Str::lit("binding ");
        utf8_encode(a782)
    };
    let a787 = {
        let a784 = v0;
        let a785 = {
            let a783 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a783)
        };
        append_bytes(a784, a785)
    };
    append_bytes(a786, a787)
}

pub fn f234(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m788 = v0;
    match m788.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f172(v1)?;
            let a790 = {
                let a789 = v3.clone();
                length_bytes(a789)
            };
            let a791 = 0u64;
            if nat_eq(a790, a791) {
                f234(v2)
            } else {
                let m792 = f158(v3.clone())?;
                match m792 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f323(v4.clone()) {
                            let m793 = f234(v2)?;
                            match m793 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f213(), List::cons(f8(Str::lit("/")), List::cons(f324(f244(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a794 = v3;
                                        let a795 = f8(Str::lit("\u{a}"));
                                        append_bytes(a794, a795)
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

pub fn f235(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt13>>> {
    let m796 = v0;
    match m796.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f172(v2)?;
            let a798 = {
                let a797 = v4.clone();
                length_bytes(a797)
            };
            let a799 = 0u64;
            if nat_eq(a798, a799) {
                f235(v3, v1)
            } else {
                let m800 = f77(v4)?;
                match m800 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f325(v5.clone());
                        let a802 = {
                            let a801 = v6.clone();
                            length_bytes(a801)
                        };
                        let a803 = 0u64;
                        if nat_eq(a802, a803) {
                            Ok(None::<List<Adt13>>)
                        } else {
                            let m804 = f235(v3, v1.clone())?;
                            match m804 {
                                None => {
                                    Ok(None::<List<Adt13>>)
                                }
                                Some(v7) => {
                                    if f28(v6, v1) {
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

pub fn f236(p807: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m806 = p807;
    {
        let mut m806 = m806;
        loop {
            let step = {
                let v0 = m806;
                let m805 = v0;
                match m805.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f41(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f238(v0: Bytes, v1: List<Adt13>) -> R<(Bytes, Bytes)> {
    Ok((v0, f86(Adt13::C3(std::rc::Rc::new(f87(v1))))?))
}

pub fn f239(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f86(Adt13::C3(std::rc::Rc::new(f326(f125(f98(v1, f122(f82(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f240(v0: Adt1, v1: Bytes) -> R<Option<Adt18>> {
    f158(f53(v0, v1))
}

pub fn f241(p813: Adt18, v1: Bytes) -> Option<Adt18> {
    let m812 = p813;
    {
        let mut m812 = m812;
        loop {
            let step = {
                let v0 = m812;
                let m808 = v0;
                match m808 {
                    Adt18::C0(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C1(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C2(_, _, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C3(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C4(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C5(r809) => {
                        let v5 = (*r809).clone();
                        Ok::<Adt18, Option<Adt18>>(v5)
                    }
                    Adt18::C6 => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C7(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C8(v2, r810, r811) => {
                        let v3 = (*r810).clone();
                        let v4 = (*r811).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt18, Option<Adt18>>(Some(v3))
                        } else {
                            Ok::<Adt18, Option<Adt18>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt1, v1: Bytes, v2: Adt18, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m814 = f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m814.uncons() {
        None => {
            let m816 = f236(f104(List::cons(Adt13::C4(std::rc::Rc::new({
                let m815 = f327(v4.clone());
                match m815 {
                    None => {
                        Adt13::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m816 {
                None => {
                    f328(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f237(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f243(v0: Adt18) -> R<Bytes> {
    let a817 = f329(v0)?;
    let a818 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a817, a818))
}

pub fn f244(v0: Adt18, v1: Bytes) -> Bytes {
    let m819 = f330(f241(v0, v1));
    match m819 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f245(v0: List<Adt18>) -> List<Bytes> {
    let m820 = v0;
    match m820.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m821 = f330(Some(v1));
            match m821 {
                None => {
                    f245(v2)
                }
                Some(v3) => {
                    List::cons(v3, f245(v2))
                }
            }
        }
    }
}

pub fn f246(v0: Option<Adt18>) -> List<Adt18> {
    let m822 = v0;
    match m822 {
        None => {
            List::<Adt18>::nil()
        }
        Some(v1) => {
            f331(v1)
        }
    }
}

pub fn f247() -> Bytes {
    f332()
}

pub fn f248(v0: Adt1, v1: Adt13, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f253(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f333(v1, v4)?)?), List::cons(f239(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f249(v0: Bytes) -> R<Bytes> {
    Ok(f334(f335(v0)?))
}

pub fn f250(v0: List<List<Bytes>>) -> List<Bytes> {
    let m823 = v0;
    match m823.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f251() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f252(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f302(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f253(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f336(f129(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f337(f338(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f254(p826: List<Adt13>) -> bool {
    let m825 = p826;
    {
        let mut m825 = m825;
        loop {
            let step = {
                let v0 = m825;
                let m824 = v0;
                match m824.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f28(f47(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f41(f8(Str::lit("studio")), f121(f339(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
                        } else {
                            false
                        } {
                            Err::<List<Adt13>, bool>(true)
                        } else {
                            Ok::<List<Adt13>, bool>(v2)
                        }
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

pub fn f255(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m827 = v0;
    match m827.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f172(v1)?;
            let a829 = {
                let a828 = v3.clone();
                length_bytes(a828)
            };
            let a830 = 0u64;
            if nat_eq(a829, a830) {
                f255(v2)
            } else {
                let m831 = f77(v3)?;
                match m831 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if if f184(v4.clone(), f8(Str::lit("agent"))) {
                            if f184(v4.clone(), f8(Str::lit("at"))) {
                                if f184(v4.clone(), f8(Str::lit("quote"))) {
                                    f340(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m832 = f255(v2)?;
                            match m832 {
                                None => {
                                    Ok(None::<List<Adt13>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons(v4, v5)))
                                }
                            }
                        } else {
                            Ok(None::<List<Adt13>>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f256(v0: u64) -> bool {
    let a833 = v0;
    let a834 = 127u64;
    nat_le(a833, a834)
}

pub fn f257(v0: Bytes) -> R<Bytes> {
    f341(v0.clone(), 0u64, {
        let a835 = v0;
        length_bytes(a835)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f258(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt18, u64)>> {
    let m836 = v2;
    if m836 == 0 {
        Ok(None::<(Adt18, u64)>)
    } else {
        let v4 = m836 - 1;
        let v5: u64 = f342(v0.clone(), v1)?;
        let a837 = v3;
        let a838 = 1u64;
        if nat_eq(a837, a838) {
            let m839 = f258(v0.clone(), v1, v4, 0u64)?;
            match m839 {
                None => {
                    Ok(None::<(Adt18, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f342(v0.clone(), {
                        let (_, h840) = v6.clone();
                        h840
                    })?;
                    let a841 = v7;
                    let a842 = 44u64;
                    if nat_eq(a841, a842) {
                        let m846 = f258(v0, {
                            let a844 = {
                                let (_, h843) = v6;
                                h843
                            };
                            let a845 = 1u64;
                            nat_add(a844, a845)?
                        }, v4, 1u64)?;
                        match m846 {
                            None => {
                                Ok(None::<(Adt18, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt18::C7(std::rc::Rc::new({
                                    let (h847, _) = v6;
                                    h847
                                }), std::rc::Rc::new({
                                    let (h848, _) = v8;
                                    h848
                                })), {
                                    let (_, h849) = v8;
                                    h849
                                })))
                            }
                        }
                    } else {
                        let a850 = v7;
                        let a851 = 93u64;
                        if nat_eq(a850, a851) {
                            Ok(Some((Adt18::C7(std::rc::Rc::new({
                                let (h852, _) = v6;
                                h852
                            }), std::rc::Rc::new(Adt18::C6)), {
                                let a854 = {
                                    let (_, h853) = v6;
                                    h853
                                };
                                let a855 = 1u64;
                                nat_add(a854, a855)?
                            })))
                        } else {
                            Ok(None::<(Adt18, u64)>)
                        }
                    }
                }
            }
        } else {
            let a856 = v3;
            let a857 = 2u64;
            if nat_eq(a856, a857) {
                let m860 = f343(v0.clone(), {
                    let a858 = v1;
                    let a859 = 1u64;
                    nat_add(a858, a859)?
                })?;
                match m860 {
                    None => {
                        Ok(None::<(Adt18, u64)>)
                    }
                    Some(v9) => {
                        if f344({
                            let a861 = v5;
                            let a862 = 34u64;
                            nat_eq(a861, a862)
                        }, {
                            let a864 = f342(v0.clone(), {
                                let (h863, _) = v9.clone();
                                h863
                            })?;
                            let a865 = 58u64;
                            nat_eq(a864, a865)
                        }) {
                            let m869 = f258(v0.clone(), {
                                let a867 = {
                                    let (h866, _) = v9;
                                    h866
                                };
                                let a868 = 1u64;
                                nat_add(a867, a868)?
                            }, v4, 0u64)?;
                            match m869 {
                                None => {
                                    Ok(None::<(Adt18, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f342(v0.clone(), {
                                        let (_, h870) = v10.clone();
                                        h870
                                    })?;
                                    let a871 = v11;
                                    let a872 = 44u64;
                                    if nat_eq(a871, a872) {
                                        let m876 = f258(v0, {
                                            let a874 = {
                                                let (_, h873) = v10;
                                                h873
                                            };
                                            let a875 = 1u64;
                                            nat_add(a874, a875)?
                                        }, v4, 2u64)?;
                                        match m876 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt18::C8({
                                                    let (_, h877) = v9;
                                                    h877
                                                }, std::rc::Rc::new({
                                                    let (h878, _) = v10;
                                                    h878
                                                }), std::rc::Rc::new({
                                                    let (h879, _) = v12;
                                                    h879
                                                })), {
                                                    let (_, h880) = v12;
                                                    h880
                                                })))
                                            }
                                        }
                                    } else {
                                        let a881 = v11;
                                        let a882 = 125u64;
                                        if nat_eq(a881, a882) {
                                            Ok(Some((Adt18::C8({
                                                let (_, h883) = v9;
                                                h883
                                            }, std::rc::Rc::new({
                                                let (h884, _) = v10;
                                                h884
                                            }), std::rc::Rc::new(Adt18::C6)), {
                                                let a886 = {
                                                    let (_, h885) = v10;
                                                    h885
                                                };
                                                let a887 = 1u64;
                                                nat_add(a886, a887)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt18, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt18, u64)>)
                        }
                    }
                }
            } else {
                let a888 = v5;
                let a889 = 34u64;
                if nat_eq(a888, a889) {
                    let m892 = f343(v0, {
                        let a890 = v1;
                        let a891 = 1u64;
                        nat_add(a890, a891)?
                    })?;
                    match m892 {
                        None => {
                            Ok(None::<(Adt18, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt18::C3({
                                let (_, h893) = v13;
                                h893
                            }), {
                                let (h894, _) = v13;
                                h894
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt18::C0(true), {
                            let a895 = v1;
                            let a896 = 4u64;
                            nat_add(a895, a896)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt18::C0(false), {
                                let a897 = v1;
                                let a898 = 5u64;
                                nat_add(a897, a898)?
                            })))
                        } else {
                            let a899 = v5;
                            let a900 = 91u64;
                            if nat_eq(a899, a900) {
                                let a903 = f342(v0.clone(), {
                                    let a901 = v1;
                                    let a902 = 1u64;
                                    nat_add(a901, a902)?
                                })?;
                                let a904 = 93u64;
                                if nat_eq(a903, a904) {
                                    Ok(Some((Adt18::C4(std::rc::Rc::new(Adt18::C6)), {
                                        let a905 = v1;
                                        let a906 = 2u64;
                                        nat_add(a905, a906)?
                                    })))
                                } else {
                                    let m909 = f258(v0, {
                                        let a907 = v1;
                                        let a908 = 1u64;
                                        nat_add(a907, a908)?
                                    }, v4, 1u64)?;
                                    match m909 {
                                        None => {
                                            Ok(None::<(Adt18, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt18::C4(std::rc::Rc::new({
                                                let (h910, _) = v14;
                                                h910
                                            })), {
                                                let (_, h911) = v14;
                                                h911
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a912 = v5;
                                let a913 = 123u64;
                                if nat_eq(a912, a913) {
                                    let a916 = f342(v0.clone(), {
                                        let a914 = v1;
                                        let a915 = 1u64;
                                        nat_add(a914, a915)?
                                    })?;
                                    let a917 = 125u64;
                                    if nat_eq(a916, a917) {
                                        Ok(Some((Adt18::C5(std::rc::Rc::new(Adt18::C6)), {
                                            let a918 = v1;
                                            let a919 = 2u64;
                                            nat_add(a918, a919)?
                                        })))
                                    } else {
                                        let m922 = f258(v0, {
                                            let a920 = v1;
                                            let a921 = 1u64;
                                            nat_add(a920, a921)?
                                        }, v4, 2u64)?;
                                        match m922 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt18::C5(std::rc::Rc::new({
                                                    let (h923, _) = v15;
                                                    h923
                                                })), {
                                                    let (_, h924) = v15;
                                                    h924
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m925 = f345(v0, v1)?;
                                    match m925 {
                                        None => {
                                            Ok(None::<(Adt18, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h926) = v16;
                                                h926
                                            }, {
                                                let (h927, _) = v16;
                                                h927
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

pub fn f259(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h928, _) = v1.clone();
        h928
    };
    let v3: u64 = {
        let (h929, _) = {
            let (_, h930) = v1.clone();
            h930
        };
        h929
    };
    let v4: List<Bytes> = {
        let (_, h931) = {
            let (_, h932) = v1;
            h932
        };
        h931
    };
    let m933 = f173(v0.clone(), v2);
    match m933 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a934 = v5;
            let a935 = 10u8;
            if equal(a934, a935) {
                Ok(Some(({
                    let a936 = v2;
                    let a937 = 1u64;
                    nat_add(a936, a937)?
                }, ({
                    let a938 = v2;
                    let a939 = 1u64;
                    nat_add(a938, a939)?
                }, List::cons(f346(f35(v0, v3, {
                    let a940 = v2;
                    let a941 = v3;
                    nat_sub(a940, a941)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a942 = v2;
                    let a943 = 1u64;
                    nat_add(a942, a943)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f260(v0: Adt13) -> List<Adt13> {
    let m944 = v0;
    match m944 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r945) => {
            let v3 = (*r945).clone();
            f260(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r946, r947) => {
            let v1 = (*r946).clone();
            let v2 = (*r947).clone();
            List::cons(v1, f260(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f261(v0: Adt13, v1: Bytes) -> Adt13 {
    let m948 = v0;
    match m948 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r949) => {
            let v10 = (*r949).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r950) => {
            let v5 = (*r950).clone();
            Adt13::C4(std::rc::Rc::new(f261(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r951, r952) => {
            let v11 = (*r951).clone();
            let v12 = (*r952).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r953, r954) => {
            let v3 = (*r953).clone();
            let v4 = (*r954).clone();
            if f28(v2.clone(), f8(Str::lit("authority"))) {
                Adt13::C7(v2, std::rc::Rc::new(f347(v3, v1.clone())), std::rc::Rc::new(f261(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f261(v4, v1)))
            }
        }
    }
}

pub fn f262(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a955 = 0u64;
        let a956 = v1;
        nat_lt(a955, a956)
    } else {
        false
    } {
        let a957 = Bytes::lit(&[0x2d]);
        let a958 = f348(v1)?;
        Ok(append_bytes(a957, a958))
    } else {
        f348(v1)
    }
}

pub fn f263(v0: Bytes) -> R<Bytes> {
    let a964 = {
        let a962 = Bytes::lit(&[0x22]);
        let a963 = {
            let (_, h959) = {
                let (h960, _) = f349(Fn4::F350(v0.clone()), {
                    let a961 = v0;
                    length_bytes(a961)
                }, (0u64, Bytes::lit(&[])))?;
                h960
            };
            h959
        };
        append_bytes(a962, a963)
    };
    let a965 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a964, a965))
}

pub fn f264(v0: List<Bytes>) -> Bytes {
    let a968 = {
        let a966 = Bytes::lit(&[0x5b]);
        let a967 = f95(v0, Bytes::lit(&[0x2c]));
        append_bytes(a966, a967)
    };
    let a969 = Bytes::lit(&[0x5d]);
    append_bytes(a968, a969)
}

pub fn f265(v0: Adt13) -> R<List<Bytes>> {
    let m970 = v0;
    match m970 {
        Adt13::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C5 => {
            Ok(List::<Bytes>::nil())
        }
        Adt13::C6(r971, r972) => {
            let v7 = (*r971).clone();
            let v8 = (*r972).clone();
            Ok(List::cons(f164(v7)?, f265(v8)?))
        }
        Adt13::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f266(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f351(Fn5::F352, List::<(Bytes, Bytes)>::nil(), f353(v0)?);
    let v2: List<Bytes> = f354(Fn6::F355, List::<Bytes>::nil(), v1)?;
    let a975 = {
        let a973 = Bytes::lit(&[0x7b]);
        let a974 = f95(v2, Bytes::lit(&[0x2c]));
        append_bytes(a973, a974)
    };
    let a976 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a975, a976))
}

pub fn f267(v0: Adt13) -> R<List<(Bytes, Bytes)>> {
    let m977 = v0;
    match m977 {
        Adt13::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C5 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C6(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt13::C7(v9, r978, r979) => {
            let v10 = (*r978).clone();
            let v11 = (*r979).clone();
            Ok(List::cons((v9, f164(v10)?), f267(v11)?))
        }
    }
}

pub fn f268() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f269(v0: Adt12) -> Bytes {
    let m980 = v0;
    match m980 {
        Adt12::C0 => {
            f8(Str::lit("model/authorities.toml"))
        }
        Adt12::C1 => {
            f8(Str::lit("model/ids.toml"))
        }
        Adt12::C2 => {
            f8(Str::lit("site/generated/release-ledger.json"))
        }
        Adt12::C3 => {
            f8(Str::lit("standards.lock"))
        }
    }
}

pub fn f270(v0: Adt12, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f356(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f356(f8(Str::lit("id")), f58(v0.clone(), f50(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f356(f8(Str::lit("name")), f357(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f356(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f356(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f356(f8(Str::lit("statement")), f358(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f356(f8(Str::lit("media_type")), f359(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f356(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f356(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f356(f8(Str::lit("url")), f167(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f271(v0: Bytes) -> R<u64> {
    let (h981, _) = f282(Fn3::F360(v0.clone()), {
        let a982 = v0.clone();
        length_bytes(a982)
    }, {
        let a983 = v0;
        length_bytes(a983)
    })?;
    Ok(h981)
}

pub fn f272(v0: Bytes, v1: u64) -> R<u64> {
    let (h984, _) = f282(Fn3::F361(v1, v0), v1, 0u64)?;
    Ok(h984)
}

pub fn f273(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m985 = v0;
    match m985.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m986 = f79(v1, f8(Str::lit("std")));
            match m986 {
                None => {
                    f273(v2)
                }
                Some(v3) => {
                    if f184(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f47(v3.clone(), f8(Str::lit("row"))), f47(v3, f8(Str::lit("authority")))), f273(v2))
                    } else {
                        f273(v2)
                    }
                }
            }
        }
    }
}

pub fn f274(v0: Adt14, v1: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m987 = v1;
    match m987.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f47(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f47(v2, f8(Str::lit("object")));
            if if f41(v4.clone(), match v0.clone() {
                Adt14::C0(h988, _, _) => {
                    h988
                }
            }) {
                f41(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h989, _) => {
                        h989
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f274(v0, v3))
            } else {
                if if f41(v5.clone(), match v0.clone() {
                    Adt14::C0(h990, _, _) => {
                        h990
                    }
                }) {
                    f41(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h991, _) => {
                            h991
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f274(v0, v3))
                } else {
                    f274(v0, v3)
                }
            }
        }
    }
}

pub fn f275(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt19>> {
    let m992 = v0;
    match m992.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v4, v5)) => {
            if if f362(v4.clone(), v2.clone()) {
                true
            } else {
                f362(v4.clone(), v3.clone())
            } {
                f275(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt19::C0({
                    let (h993, _) = v4.clone();
                    h993
                }, {
                    let a996 = {
                        let a995 = f125(f363({
                            let (h994, _) = v4.clone();
                            h994
                        }, v1.clone()))?;
                        length_list(a995)
                    };
                    let a997 = 1u64;
                    if nat_eq(a996, a997) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h998) = v4.clone();
                    h998
                }), f275(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f276(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m999 = v1;
    match m999.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41({
                let (h1000, _) = v2.clone();
                h1000
            }, v0.clone()) {
                List::cons({
                    let (_, h1001) = v2;
                    h1001
                }, f276(v0, v3))
            } else {
                if f41({
                    let (_, h1002) = v2;
                    h1002
                }, v0.clone()) {
                    List::cons({
                        let (h1003, _) = v2;
                        h1003
                    }, f276(v0, v3))
                } else {
                    f276(v0, v3)
                }
            }
        }
    }
}

pub fn f277(v0: Adt14, v1: Adt13, v2: Bytes) -> bool {
    if f41(v2.clone(), match v0.clone() {
        Adt14::C0(h1004, _, _) => {
            h1004
        }
    }) {
        true
    } else {
        if f41(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1005, _) => {
                h1005
            }
        }) {
            true
        } else {
            if f41(v2, match v0.clone() {
                Adt14::C0(_, _, h1006) => {
                    h1006
                }
            }) {
                true
            } else {
                if f184(v1.clone(), f8(Str::lit("scheme"))) {
                    f41(f47(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1007, _) => {
                            h1007
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f278(v0: Adt13) -> bool {
    if f184(v0.clone(), f8(Str::lit("domain"))) {
        if f184(v0.clone(), f8(Str::lit("row"))) {
            if f41(f47(v0.clone(), f8(Str::lit("relation"))), f364()) {
                if f365(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f365(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f365(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f365(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f279(v0: Adt13) -> bool {
    if f184(v0.clone(), f8(Str::lit("subject"))) {
        if f184(v0.clone(), f8(Str::lit("object"))) {
            if f184(v0.clone(), f8(Str::lit("evidence"))) {
                f41(f47(v0, f8(Str::lit("relation"))), f366())
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

pub fn f280(v0: Bytes) -> bool {
    f189(v0, f8(Str::lit(".lex.tex")))
}

pub fn f281(v0: Bytes) -> R<bool> {
    let v1: Bytes = f172(v0)?;
    let v2: u64 = if f72(v1.clone(), f8(Str::lit("if ! "))) {
        5u64
    } else {
        if f72(v1.clone(), f8(Str::lit("elif ! "))) {
            7u64
        } else {
            0u64
        }
    };
    let v3: Bytes = f35(v1.clone(), v2, {
        let a1009 = {
            let a1008 = v1;
            length_bytes(a1008)
        };
        let a1010 = v2;
        nat_sub(a1009, a1010)
    });
    let v5: Bytes = if f72(v3.clone(), f8(Str::lit("("))) {
        let m1011 = f193(v3.clone(), f8(Str::lit(")")))?;
        match m1011 {
            None => {
                v3
            }
            Some(v4) => {
                f35(v3.clone(), {
                    let a1012 = v4;
                    let a1013 = 1u64;
                    nat_add(a1012, a1013)?
                }, {
                    let a1017 = {
                        let a1014 = v3;
                        length_bytes(a1014)
                    };
                    let a1018 = {
                        let a1015 = v4;
                        let a1016 = 1u64;
                        nat_add(a1015, a1016)?
                    };
                    nat_sub(a1017, a1018)
                })
            }
        }
    } else {
        v3
    };
    let a1019 = 0u64;
    let a1020 = v2;
    if nat_lt(a1019, a1020) {
        f367(v5)
    } else {
        Ok(false)
    }
}

pub fn f282(v0: Fn3, p1025: u64, p1026: u64) -> R<(u64, bool)> {
    let m1024 = (p1025, p1026);
    {
        let mut m1024 = m1024;
        loop {
            let step = {
                let (v1, v2) = m1024;
                let m1021 = v1;
                if m1021 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1021 - 1;
                    let m1023 = {
                        let c1022: Fn3 = v0.clone();
                        c1022.apply(v2)?
                    };
                    match m1023 {
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
                Ok(next) => { m1024 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f284(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1029 = {
            let a1027 = v4;
            let a1028 = v1;
            nat_add(a1027, a1028)?
        };
        let a1030 = v3;
        if nat_lt(a1029, a1030) {
            Ok(Some({
                let a1031 = v4;
                let a1032 = 1u64;
                nat_add(a1031, a1032)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f284(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1037 = {
        let a1034 = v0;
        let a1035 = v2;
        let a1036 = {
            let a1033 = v1.clone();
            length_bytes(a1033)
        };
        slice_bytes(a1034, a1035, a1036)
    };
    match m1037 {
        None => {
            false
        }
        Some(v3) => {
            let a1038 = v3;
            let a1039 = v1;
            equal(a1038, a1039)
        }
    }
}

pub fn f285(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1040 = v0;
    match m1040.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f285(v3, {
                let a1041 = v1;
                let a1042 = 1u64;
                nat_add(a1041, a1042)?
            })?))
        }
    }
}

pub fn f286(v0: Adt17, v1: List<u64>, v2: Adt21) -> bool {
    let m1043 = v2;
    match m1043 {
        Adt21::C0(v3, v4) => {
            if f292(v3, match v0.clone() {
                Adt17::C0(h1044, _, _, _) => {
                    h1044
                }
            }) {
                f368(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f292(v5, match v0 {
                Adt17::C0(_, _, h1045, _) => {
                    h1045
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f288(v7, v1) {
                f292(v8, match v0 {
                    Adt17::C0(_, _, _, h1046) => {
                        h1046
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f287(v0: Adt15, v1: Adt13) -> R<Adt21> {
    let m1047 = f79(v1.clone(), f8(Str::lit("std")));
    match m1047 {
        None => {
            let m1052 = f78(f79(v1.clone(), f8(Str::lit("req"))));
            match m1052 {
                None => {
                    let m1054 = f79(v1, f8(Str::lit("der")));
                    match m1054 {
                        None => {
                            Ok(f369(v0))
                        }
                        Some(v5) => {
                            let m1055 = f370(f47(v5.clone(), f8(Str::lit("rule"))));
                            match m1055 {
                                None => {
                                    Ok(f369(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f201(f122(f162(f79(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1056) => {
                                            h1056
                                        }
                                    })?, f290(f47(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1057, _) => {
                                            h1057
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f290(v4, match v0 {
                        Adt15::C0(_, _, h1053, _, _) => {
                            h1053
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1048 = f78(f79(v2.clone(), f8(Str::lit("row"))));
            match m1048 {
                None => {
                    Ok(Adt21::C0(f290(f47(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1049, _, _, _, _) => {
                            h1049
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f290(f47(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1050, _, _, _, _) => {
                            h1050
                        }
                    })?, Some(f290(v3, match v0 {
                        Adt15::C0(_, h1051, _, _, _) => {
                            h1051
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f288(p1060: List<u64>, v1: List<u64>) -> bool {
    let m1059 = p1060;
    {
        let mut m1059 = m1059;
        loop {
            let step = {
                let v0 = m1059;
                let m1058 = v0;
                match m1058.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f292(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f289(p1065: List<u64>) -> bool {
    let m1064 = p1065;
    {
        let mut m1064 = m1064;
        loop {
            let step = {
                let v0 = m1064;
                let m1061 = v0;
                match m1061.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1062 = v1;
                        let a1063 = 7u64;
                        if nat_le(a1062, a1063) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1064 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1066 = f371(v0, v1.clone(), 0u64)?;
    match m1066 {
        None => {
            let a1067 = v1;
            Ok(length_list(a1067))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f291(v0: Adt13) -> u64 {
    let m1068 = f372(f79(v0, f8(Str::lit("stratum"))));
    match m1068 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f292(v0: u64, p1073: List<u64>) -> bool {
    let m1072 = p1073;
    {
        let mut m1072 = m1072;
        loop {
            let step = {
                let v1 = m1072;
                let m1069 = v1;
                match m1069.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1070 = v2;
                        let a1071 = v0;
                        if nat_eq(a1070, a1071) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293(v0: Bytes, p1131: u64, p1132: u64, p1133: Adt23) -> R<Adt23> {
    let m1130 = (p1131, (p1132, p1133));
    {
        let mut m1130 = m1130;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1130;
                let m1074 = v2;
                if m1074 == 0 {
                    Err::<(u64, (u64, Adt23)), R<Adt23>>(Ok(v3))
                } else {
                    let v4 = m1074 - 1;
                    let v5: u64 = f373(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt23::C0(_, h1075, _, _, _) => {
                            h1075
                        }
                    } {
                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                            let a1076 = v1;
                            let a1077 = 1u64;
                            nat_add(a1076, a1077)?
                        }, (v4, Adt23::C0(true, false, match v3.clone() {
                            Adt23::C0(_, _, h1078, _, _) => {
                                h1078
                            }
                        }, match v3.clone() {
                            Adt23::C0(_, _, _, h1079, _) => {
                                h1079
                            }
                        }, match v3 {
                            Adt23::C0(_, _, _, _, h1080) => {
                                h1080
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt23::C0(h1081, _, _, _, _) => {
                                h1081
                            }
                        } {
                            let a1082 = v5;
                            let a1083 = 34u64;
                            if nat_eq(a1082, a1083) {
                                let a1086 = f373(v0.clone(), {
                                    let a1084 = v1;
                                    let a1085 = 1u64;
                                    nat_add(a1084, a1085)?
                                })?;
                                let a1087 = 34u64;
                                if nat_eq(a1086, a1087) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1088 = v1;
                                        let a1089 = 1u64;
                                        nat_add(a1088, a1089)?
                                    }, (v4, Adt23::C0(true, true, f374(match v3.clone() {
                                        Adt23::C0(_, _, h1090, _, _) => {
                                            h1090
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt23::C0(_, _, _, h1091, _) => {
                                            h1091
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1092) => {
                                            h1092
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1093 = v1;
                                        let a1094 = 1u64;
                                        nat_add(a1093, a1094)?
                                    }, (v4, Adt23::C0(false, false, match v3.clone() {
                                        Adt23::C0(_, _, h1095, _, _) => {
                                            h1095
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1096, _) => {
                                            h1096
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1097) => {
                                            h1097
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1098 = v1;
                                    let a1099 = 1u64;
                                    nat_add(a1098, a1099)?
                                }, (v4, Adt23::C0(true, false, f374(match v3.clone() {
                                    Adt23::C0(_, _, h1100, _, _) => {
                                        h1100
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt23::C0(_, _, _, h1101, _) => {
                                        h1101
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1102) => {
                                        h1102
                                    }
                                }))))
                            }
                        } else {
                            let a1103 = v5;
                            let a1104 = 34u64;
                            if nat_eq(a1103, a1104) {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1105 = v1;
                                    let a1106 = 1u64;
                                    nat_add(a1105, a1106)?
                                }, (v4, Adt23::C0(true, false, match v3.clone() {
                                    Adt23::C0(_, _, h1107, _, _) => {
                                        h1107
                                    }
                                }, match v3.clone() {
                                    Adt23::C0(_, _, _, h1108, _) => {
                                        h1108
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1109) => {
                                        h1109
                                    }
                                }))))
                            } else {
                                let a1110 = v5;
                                let a1111 = 44u64;
                                if nat_eq(a1110, a1111) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1112 = v1;
                                        let a1113 = 1u64;
                                        nat_add(a1112, a1113)?
                                    }, (v4, Adt23::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt23::C0(_, _, h1114, _, _) => {
                                            h1114
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1115, _) => {
                                            h1115
                                        }
                                    }), match v3 {
                                        Adt23::C0(_, _, _, _, h1116) => {
                                            h1116
                                        }
                                    }))))
                                } else {
                                    let a1117 = v5;
                                    let a1118 = 13u64;
                                    if nat_eq(a1117, a1118) {
                                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                            let a1119 = v1;
                                            let a1120 = 1u64;
                                            nat_add(a1119, a1120)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1121 = v5;
                                        let a1122 = 10u64;
                                        if nat_eq(a1121, a1122) {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1123 = v1;
                                                let a1124 = 1u64;
                                                nat_add(a1123, a1124)?
                                            }, (v4, f294(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1125 = v1;
                                                let a1126 = 1u64;
                                                nat_add(a1125, a1126)?
                                            }, (v4, Adt23::C0(false, false, f374(match v3.clone() {
                                                Adt23::C0(_, _, h1127, _, _) => {
                                                    h1127
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt23::C0(_, _, _, h1128, _) => {
                                                    h1128
                                                }
                                            }, match v3 {
                                                Adt23::C0(_, _, _, _, h1129) => {
                                                    h1129
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
                Ok(next) => { m1130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Adt23) -> Adt23 {
    Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f375(List::cons(match v0.clone() {
        Adt23::C0(_, _, h1134, _, _) => {
            h1134
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1135, _) => {
            h1135
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt23::C0(_, _, _, _, h1136) => {
            h1136
        }
    }))
}

pub fn f295(p1139: List<List<Bytes>>, p1140: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1138 = (p1139, p1140);
    {
        let mut m1138 = m1138;
        loop {
            let step = {
                let (v0, v1) = m1138;
                let m1137 = v0;
                match m1137.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f297(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1141 = v0.clone();
    match m1141.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1142 = v2;
            let a1143 = v1.clone();
            if equal(a1142, a1143) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f298(v0: List<(Bytes, ())>) -> List<Bytes> {
    f207(f376(Fn7::F377, List::<Bytes>::nil(), v0))
}

pub fn f299(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f378(f379(Fn8::F380, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1148 = {
        let (h1144, _) = f381(Fn9::F382, {
            let a1146 = {
                let a1145 = v0;
                length_list(a1145)
            };
            let a1147 = 1u64;
            nat_add(a1146, a1147)?
        }, v1)?;
        h1144
    };
    match m1148.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f300(v0: List<Bytes>) -> List<(Bytes, ())> {
    f383(f384(Fn10::F385, List::<(Bytes, ())>::nil(), v0))
}

pub fn f301(v0: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1149 = f33(v1, 34u8)?;
    match m1149.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1150 = v3;
            match m1150.uncons() {
                None => {
                    f386(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f302(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f387(v0.clone(), 0u64, {
        let a1151 = v0;
        length_bytes(a1151)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f303(v0: Adt13, v1: Bytes) -> Bytes {
    if f28(f388(f79(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f304(v0: Adt1, v1: Bytes) -> R<Adt18> {
    let m1152 = f240(v0, v1)?;
    match m1152 {
        None => {
            Ok(Adt18::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f305(v0: Bytes) -> R<Bytes> {
    let a1153 = f8(Str::lit("Catalog."));
    let a1154 = f324(v0)?;
    Ok(append_bytes(a1153, a1154))
}

pub fn f306(v0: Adt18) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f324(f244(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f307(v0: List<Bytes>) -> List<Bytes> {
    let m1155 = v0;
    match m1155.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f189(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f210(f227(), v1), f307(v2))
            } else {
                f307(v2)
            }
        }
    }
}

pub fn f308(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1156 = v1;
    match m1156.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f389(v0.clone(), v2)?;
            let (h1157, _) = v4.clone();
            if h1157 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f308(v0, v3)?;
                let (h1158, _) = v5.clone();
                if h1158 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1161 = {
                            let (_, h1159) = v4;
                            h1159
                        };
                        let a1162 = {
                            let (_, h1160) = v5;
                            h1160
                        };
                        append_bytes(a1161, a1162)
                    }))
                }
            }
        }
    }
}

pub fn f309(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m1163 = v1;
    match m1163.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f184(v2.clone(), f8(Str::lit("subject"))) {
                if f185(v0.clone(), v2.clone()) {
                    f309(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f47(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f309(v0, v3))
                }
            } else {
                if f315(v0.clone(), v2.clone()) {
                    f309(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f316(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f309(v0, v3))
                }
            }
        }
    }
}

pub fn f310(v0: Adt1) -> R<Bytes> {
    let a1165 = {
        let a1164 = f101(f102(f206(f219(f205(f53(v0.clone(), f178()))?)), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(f82(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1164)
    };
    let a1166 = 0u64;
    if nat_eq(a1165, a1166) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f311(v0: List<Adt13>) -> R<List<Bytes>> {
    let m1167 = v0;
    match m1167.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f95(f122(f162(f79(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f47(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f47(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f390(f81(f47(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f311(v2)?))
        }
    }
}

pub fn f312(v0: Bytes, v1: List<Adt19>) -> List<Bytes> {
    let m1168 = v1;
    match m1168.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(match v2.clone() {
                Adt19::C0(h1169, _, _) => {
                    h1169
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt19::C0(_, h1170, _) => {
                        h1170
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt19::C0(_, _, h1171) => {
                        h1171
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f312(v0, v3))
            } else {
                f312(v0, v3)
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: List<Adt13>) -> List<Bytes> {
    let m1172 = v1;
    match m1172.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(f47(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f313(v0, v3))
            } else {
                if f28(f47(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f47(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f313(v0, v3))
                } else {
                    f313(v0, v3)
                }
            }
        }
    }
}

pub fn f314(p1177: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1176 = p1177;
    {
        let mut m1176 = m1176;
        loop {
            let step = {
                let v0 = m1176;
                let m1173 = v0;
                match m1173.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1174 = v2;
                        match m1174.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f28(v4, v1.clone()) {
                                    let m1175 = v5;
                                    match m1175.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f391(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: Adt14, v1: Adt13) -> bool {
    if f41(f47(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1178, _, _) => {
            h1178
        }
    }) {
        f41(f316(v1), match v0 {
            Adt14::C0(_, h1179, _) => {
                h1179
            }
        })
    } else {
        false
    }
}

pub fn f316(v0: Adt13) -> Bytes {
    let m1180 = f79(v0, f8(Str::lit("evidence")));
    match m1180 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f47(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f317(v0: Adt13) -> Bytes {
    if f184(v0.clone(), f8(Str::lit("req"))) {
        let a1181 = f8(Str::lit("req "));
        let a1182 = f47(v0, f8(Str::lit("req")));
        append_bytes(a1181, a1182)
    } else {
        let m1183 = f79(v0.clone(), f8(Str::lit("std")));
        match m1183 {
            None => {
                let m1186 = f79(v0, f8(Str::lit("der")));
                match m1186 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f47(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f95(f122(f162(f79(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f47(v1.clone(), f8(Str::lit("authority"))), List::cons(if f184(v1.clone(), f8(Str::lit("row"))) {
                    let a1184 = f8(Str::lit(" row "));
                    let a1185 = f47(v1, f8(Str::lit("row")));
                    append_bytes(a1184, a1185)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f318(v0: Option<List<Adt13>>) -> bool {
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

pub fn f319(v0: Option<List<Adt13>>) -> List<Adt13> {
    let m1188 = v0;
    match m1188 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f320(v0: Bytes, v1: Bytes) -> bool {
    let a1189 = v0;
    let a1190 = f332();
    if equal(a1189, a1190) {
        let a1194 = {
            let a1192 = {
                let a1191 = v1;
                length_bytes(a1191)
            };
            let a1193 = 0u64;
            nat_eq(a1192, a1193)
        };
        bool_not(a1194)
    } else {
        false
    }
}

pub fn f321(v0: Bytes) -> Bytes {
    f7(List::cons(f124(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f322(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1196 = v0;
    let a1197 = {
        let a1195 = Str::lit("redistributable");
        utf8_encode(a1195)
    };
    if equal(a1196, a1197) {
        f392(v1)
    } else {
        Ok(false)
    }
}

pub fn f323(v0: Adt18) -> bool {
    if f393(v0.clone(), f8(Str::lit("name"))) {
        if f394(v0.clone(), f8(Str::lit("schema"))) {
            let m1198 = f327(f245(f246(f241(v0, f8(Str::lit("citation"))))));
            match m1198 {
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

pub fn f324(v0: Bytes) -> R<Bytes> {
    f395(v0.clone(), 0u64, {
        let a1199 = v0;
        length_bytes(a1199)
    }, true, Bytes::lit(&[]))
}

pub fn f325(v0: Adt13) -> Bytes {
    if f184(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f184(v0.clone(), f8(Str::lit("id"))) {
            if f184(v0.clone(), f8(Str::lit("statement"))) {
                if f184(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f340(v0, f8(Str::lit("evidence")))
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

pub fn f326(v0: List<Bytes>) -> Adt13 {
    let m1200 = v0;
    match m1200.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(f326(v2)))
        }
    }
}

pub fn f327(v0: List<Bytes>) -> Option<Adt13> {
    let m1201 = v0;
    match m1201.uncons() {
        None => {
            None::<Adt13>
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("req"))) {
                let m1202 = v2;
                match m1202.uncons() {
                    None => {
                        None::<Adt13>
                    }
                    Some((v3, _)) => {
                        Some(Adt13::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5)))
                    }
                }
            } else {
                if f28(v1.clone(), f8(Str::lit("std"))) {
                    let m1203 = v2;
                    match m1203.uncons() {
                        None => {
                            None::<Adt13>
                        }
                        Some((v5, v6)) => {
                            Some(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v5)), std::rc::Rc::new({
                                let m1204 = v6;
                                match m1204.uncons() {
                                    None => {
                                        Adt13::C5
                                    }
                                    Some((v7, _)) => {
                                        Adt13::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt13::C2(v7)), std::rc::Rc::new(Adt13::C5))
                                    }
                                }
                            }))))), std::rc::Rc::new(Adt13::C5)))
                        }
                    }
                } else {
                    if f28(v1, f8(Str::lit("der"))) {
                        let m1205 = v2;
                        match m1205.uncons() {
                            None => {
                                None::<Adt13>
                            }
                            Some((v9, v10)) => {
                                let m1206 = v10;
                                match m1206.uncons() {
                                    None => {
                                        None::<Adt13>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt13::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt13::C2(v11)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f326(v12)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt13::C2(v9)), std::rc::Rc::new(Adt13::C5))))))))), std::rc::Rc::new(Adt13::C5)))
                                    }
                                }
                            }
                        }
                    } else {
                        None::<Adt13>
                    }
                }
            }
        }
    }
}

pub fn f328(v0: Adt1, v1: Bytes, v2: Adt18, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1207 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1207.uncons() {
        None => {
            let m1208 = f327(v4);
            match m1208 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1209 = f8(Str::lit("Catalog."));
                        let a1210 = f324(v3.clone())?;
                        append_bytes(a1209, a1210)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f324(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1211 = f396(v2, v9.clone(), f324(v3.clone())?)?;
                    match m1211 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f324(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f397(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f397(f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt13::C2(v10.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt13::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt13::C1(false, 0u64)), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1215 = f398(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f35(v10.clone(), 4u64, {
                                    let a1213 = {
                                        let a1212 = v10;
                                        length_bytes(a1212)
                                    };
                                    let a1214 = 4u64;
                                    nat_sub(a1213, a1214)
                                }))?;
                                let a1216 = f8(Str::lit("\u{a}"));
                                append_bytes(a1215, a1216)
                            }), List::cons(f239(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Err(v11) => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v11, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f329(v0: Adt18) -> R<Bytes> {
    let m1217 = v0;
    match m1217 {
        Adt18::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt18::C1(v2, v3) => {
            f262(v2, v3)
        }
        Adt18::C2(v4, v5, v6) => {
            f399(v4, v5, v6)
        }
        Adt18::C3(v7) => {
            f400(v7)
        }
        Adt18::C4(r1218) => {
            let v8 = (*r1218).clone();
            Ok(f264(f401(v8)?))
        }
        Adt18::C5(r1219) => {
            let v9 = (*r1219).clone();
            f266(f402(v9)?)
        }
        Adt18::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f330(v0: Option<Adt18>) -> Option<Bytes> {
    let m1220 = v0;
    match m1220 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1221 = v1;
            match m1221 {
                Adt18::C0(_) => {
                    None::<Bytes>
                }
                Adt18::C1(_, _) => {
                    None::<Bytes>
                }
                Adt18::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt18::C3(v2) => {
                    Some(v2)
                }
                Adt18::C4(_) => {
                    None::<Bytes>
                }
                Adt18::C5(_) => {
                    None::<Bytes>
                }
                Adt18::C6 => {
                    None::<Bytes>
                }
                Adt18::C7(_, _) => {
                    None::<Bytes>
                }
                Adt18::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f331(v0: Adt18) -> List<Adt18> {
    let m1222 = v0;
    match m1222 {
        Adt18::C0(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C1(_, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C2(_, _, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C3(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C4(r1223) => {
            let v3 = (*r1223).clone();
            f331(v3)
        }
        Adt18::C5(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C6 => {
            List::<Adt18>::nil()
        }
        Adt18::C7(r1224, r1225) => {
            let v1 = (*r1224).clone();
            let v2 = (*r1225).clone();
            List::cons(v1, f331(v2))
        }
        Adt18::C8(_, _, _) => {
            List::<Adt18>::nil()
        }
    }
}

pub fn f332() -> Bytes {
    let a1226 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1226)
}

pub fn f333(v0: Adt13, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt13> = f79(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f252(Str::lit("id"), f47(v0.clone(), f8(Str::lit("id"))))?, List::cons(f252(Str::lit("name"), f47(v0.clone(), f8(Str::lit("name"))))?, List::cons(f252(Str::lit("issuer"), f47(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f252(Str::lit("edition"), f47(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f252(Str::lit("canonical_id"), f47(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f252(Str::lit("source_role"), f47(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f252(Str::lit("retrieval_date"), f47(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f252(Str::lit("url"), f388(v2.clone(), f8(Str::lit("url"))))?, List::cons(f252(Str::lit("media_type"), f388(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f252(Str::lit("license"), f388(v2.clone(), f8(Str::lit("license"))))?, List::cons(f252(Str::lit("redistribution"), f388(v2, f8(Str::lit("redistribution"))))?, List::cons(f252(Str::lit("statement"), f47(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1227 = v1;
        match m1227 {
            None => {
                f7(List::cons(f252(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f252(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f252(Str::lit("artifact"), {
                    let (h1228, _) = v3;
                    h1228
                })?, List::cons(f252(Str::lit("checksum"), {
                    let (_, h1229) = v3;
                    h1229
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f403(f122(f162(f79(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f334(v0: Adt24) -> Bytes {
    let a1250 = f404(match v0.clone() {
        Adt24::C0(h1230, _, _, _, _, _, _, _) => {
            h1230
        }
    });
    let a1251 = {
        let a1248 = f404(match v0.clone() {
            Adt24::C0(_, h1231, _, _, _, _, _, _) => {
                h1231
            }
        });
        let a1249 = {
            let a1246 = f404(match v0.clone() {
                Adt24::C0(_, _, h1232, _, _, _, _, _) => {
                    h1232
                }
            });
            let a1247 = {
                let a1244 = f404(match v0.clone() {
                    Adt24::C0(_, _, _, h1233, _, _, _, _) => {
                        h1233
                    }
                });
                let a1245 = {
                    let a1242 = f404(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1234, _, _, _) => {
                            h1234
                        }
                    });
                    let a1243 = {
                        let a1240 = f404(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1235, _, _) => {
                                h1235
                            }
                        });
                        let a1241 = {
                            let a1238 = f404(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1236, _) => {
                                    h1236
                                }
                            });
                            let a1239 = f404(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1237) => {
                                    h1237
                                }
                            });
                            append_bytes(a1238, a1239)
                        };
                        append_bytes(a1240, a1241)
                    };
                    append_bytes(a1242, a1243)
                };
                append_bytes(a1244, a1245)
            };
            append_bytes(a1246, a1247)
        };
        append_bytes(a1248, a1249)
    };
    append_bytes(a1250, a1251)
}

pub fn f335(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1252 = v0.clone();
        length_bytes(a1252)
    };
    let v2: u64 = {
        let a1258 = {
            let a1255 = {
                let a1253 = v1;
                let a1254 = 72u64;
                nat_add(a1253, a1254)?
            };
            let a1256 = 64u64;
            let a1257 = 0u64;
            nat_quot(a1255, a1256, a1257)
        };
        let a1259 = 64u64;
        nat_mul(a1258, a1259)?
    };
    let (_, h1260) = f405(Fn11::F406(v0, v1, v2), {
        let a1261 = v2;
        let a1262 = 64u64;
        let a1263 = 0u64;
        nat_quot(a1261, a1262, a1263)
    }, (0u64, f407()))?;
    Ok(h1260)
}

pub fn f336(p1266: List<List<Bytes>>, v1: Bytes, p1267: List<Bytes>) -> R<List<Bytes>> {
    let m1265 = (p1266, p1267);
    {
        let mut m1265 = m1265;
        loop {
            let step = {
                let (v0, v2) = m1265;
                let m1264 = v0;
                match m1264.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f408(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f338(f409(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: Bytes) -> R<Bytes> {
    Ok(f35(v0.clone(), 0u64, f271(v0)?))
}

pub fn f338(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1268 = v0;
    match m1268.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1269 = v3.clone();
            match m1269.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1272 = v2;
                    let a1273 = {
                        let a1270 = v1.clone();
                        let a1271 = f338(v3, v1);
                        append_bytes(a1270, a1271)
                    };
                    append_bytes(a1272, a1273)
                }
            }
        }
    }
}

pub fn f339(v0: Adt13, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1274 = f79(v0, v1);
    match m1274 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v3) => {
            f162(f79(v3, v2))
        }
    }
}

pub fn f340(v0: Adt13, v1: Bytes) -> bool {
    let m1275 = f79(v0, v1);
    match m1275 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f341(v0: Bytes, p1311: u64, p1312: u64, p1313: bool, p1314: bool, p1315: u64, p1316: Bytes) -> R<Bytes> {
    let m1310 = (p1311, (p1312, (p1313, (p1314, (p1315, p1316)))));
    {
        let mut m1310 = m1310;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m1310;
                let m1276 = v2;
                if m1276 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f410(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m1276 - 1;
                    let m1279 = {
                        let a1277 = v0.clone();
                        let a1278 = v1;
                        index_bytes(a1277, a1278)
                    };
                    match m1279 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f410(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a1280 = v1;
                                    let a1281 = 1u64;
                                    nat_add(a1280, a1281)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a1282 = v9;
                                    let a1283 = 34u64;
                                    !nat_eq(a1282, a1283)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1284 = v9;
                                    let a1285 = 92u64;
                                    nat_eq(a1284, a1285)
                                }, (v5, v6))))))
                            } else {
                                let a1286 = v9;
                                let a1287 = 32u64;
                                if if if if nat_eq(a1286, a1287) {
                                    true
                                } else {
                                    let a1288 = v9;
                                    let a1289 = 9u64;
                                    nat_eq(a1288, a1289)
                                } {
                                    true
                                } else {
                                    let a1290 = v9;
                                    let a1291 = 10u64;
                                    nat_eq(a1290, a1291)
                                } {
                                    true
                                } else {
                                    let a1292 = v9;
                                    let a1293 = 13u64;
                                    nat_eq(a1292, a1293)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1294 = v1;
                                        let a1295 = 1u64;
                                        nat_add(a1294, a1295)?
                                    }, (v7, (false, (false, ({
                                        let a1296 = v1;
                                        let a1297 = 1u64;
                                        nat_add(a1296, a1297)?
                                    }, f410(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    let a1298 = v9;
                                    let a1299 = 110u64;
                                    if if nat_eq(a1298, a1299) {
                                        f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("null")))
                                    } else {
                                        false
                                    } {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a1300 = v1;
                                            let a1301 = 4u64;
                                            nat_add(a1300, a1301)?
                                        }, (v7, (false, (false, ({
                                            let a1302 = v1;
                                            let a1303 = 4u64;
                                            nat_add(a1302, a1303)?
                                        }, {
                                            let a1304 = f410(v0.clone(), v5, v1, v6);
                                            let a1305 = f8(Str::lit("[]"));
                                            append_bytes(a1304, a1305)
                                        }))))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a1306 = v1;
                                            let a1307 = 1u64;
                                            nat_add(a1306, a1307)?
                                        }, (v7, ({
                                            let a1308 = v9;
                                            let a1309 = 34u64;
                                            nat_eq(a1308, a1309)
                                        }, (false, (v5, v6))))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f342(v0: Bytes, v1: u64) -> R<u64> {
    let m1319 = {
        let a1317 = v0;
        let a1318 = v1;
        index_bytes(a1317, a1318)
    };
    match m1319 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1364 = {
                let a1340 = {
                    let a1328 = {
                        let a1322 = {
                            let a1320 = v3;
                            let a1321 = 128u8;
                            fixed_u8::bit_and(a1320, a1321)
                        };
                        let a1323 = 128u8;
                        if equal(a1322, a1323) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1329 = {
                        let a1326 = {
                            let a1324 = v3;
                            let a1325 = 64u8;
                            fixed_u8::bit_and(a1324, a1325)
                        };
                        let a1327 = 64u8;
                        if equal(a1326, a1327) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1328, a1329)?
                };
                let a1341 = {
                    let a1338 = {
                        let a1332 = {
                            let a1330 = v3;
                            let a1331 = 32u8;
                            fixed_u8::bit_and(a1330, a1331)
                        };
                        let a1333 = 32u8;
                        if equal(a1332, a1333) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1339 = {
                        let a1336 = {
                            let a1334 = v3;
                            let a1335 = 16u8;
                            fixed_u8::bit_and(a1334, a1335)
                        };
                        let a1337 = 16u8;
                        if equal(a1336, a1337) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1338, a1339)?
                };
                nat_add(a1340, a1341)?
            };
            let a1365 = {
                let a1362 = {
                    let a1350 = {
                        let a1344 = {
                            let a1342 = v3;
                            let a1343 = 8u8;
                            fixed_u8::bit_and(a1342, a1343)
                        };
                        let a1345 = 8u8;
                        if equal(a1344, a1345) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1351 = {
                        let a1348 = {
                            let a1346 = v3;
                            let a1347 = 4u8;
                            fixed_u8::bit_and(a1346, a1347)
                        };
                        let a1349 = 4u8;
                        if equal(a1348, a1349) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1350, a1351)?
                };
                let a1363 = {
                    let a1360 = {
                        let a1354 = {
                            let a1352 = v3;
                            let a1353 = 2u8;
                            fixed_u8::bit_and(a1352, a1353)
                        };
                        let a1355 = 2u8;
                        if equal(a1354, a1355) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1361 = {
                        let a1358 = {
                            let a1356 = v3;
                            let a1357 = 1u8;
                            fixed_u8::bit_and(a1356, a1357)
                        };
                        let a1359 = 1u8;
                        if equal(a1358, a1359) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1360, a1361)?
                };
                nat_add(a1362, a1363)?
            };
            nat_add(a1364, a1365)
        }
    }
}

pub fn f343(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f411(v0.clone(), v1, {
        let a1367 = {
            let a1366 = v0.clone();
            length_bytes(a1366)
        };
        let a1368 = 1u64;
        nat_add(a1367, a1368)?
    })?;
    let a1369 = f342(v0.clone(), v2)?;
    let a1370 = 34u64;
    if nat_eq(a1369, a1370) {
        Ok(Some(({
            let a1371 = v2;
            let a1372 = 1u64;
            nat_add(a1371, a1372)?
        }, f35(v0, v1, {
            let a1373 = v2;
            let a1374 = v1;
            nat_sub(a1373, a1374)
        }))))
    } else {
        f412(v0, v1)
    }
}

pub fn f344(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f345(v0: Bytes, v1: u64) -> R<Option<(u64, Adt18)>> {
    let v2: bool = {
        let a1375 = f342(v0.clone(), v1)?;
        let a1376 = 45u64;
        nat_eq(a1375, a1376)
    };
    let v3: u64 = if v2 {
        let a1377 = v1;
        let a1378 = 1u64;
        nat_add(a1377, a1378)?
    } else {
        v1
    };
    let v4: u64 = f413(v0.clone(), v3)?;
    let v5: u64 = {
        let a1379 = v4;
        let a1380 = v3;
        nat_sub(a1379, a1380)
    };
    let v6: bool = {
        let a1381 = f342(v0.clone(), v4)?;
        let a1382 = 46u64;
        nat_eq(a1381, a1382)
    };
    let v7: u64 = if v6 {
        let a1383 = v4;
        let a1384 = 1u64;
        nat_add(a1383, a1384)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f413(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f342(v0.clone(), v8)?;
    let v10: bool = {
        let a1385 = v9;
        let a1386 = 101u64;
        if nat_eq(a1385, a1386) {
            true
        } else {
            let a1387 = v9;
            let a1388 = 69u64;
            nat_eq(a1387, a1388)
        }
    };
    let v11: u64 = f342(v0.clone(), {
        let a1389 = v8;
        let a1390 = 1u64;
        nat_add(a1389, a1390)?
    })?;
    let v12: bool = f344(v10, f414({
        let a1391 = v11;
        let a1392 = 43u64;
        nat_eq(a1391, a1392)
    }, {
        let a1393 = v11;
        let a1394 = 45u64;
        nat_eq(a1393, a1394)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a1395 = v8;
            let a1396 = 2u64;
            nat_add(a1395, a1396)?
        } else {
            let a1397 = v8;
            let a1398 = 1u64;
            nat_add(a1397, a1398)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f413(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a1399 = v14;
        let a1400 = v13;
        nat_sub(a1399, a1400)
    });
    let v16: u64 = f415(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a1402 = {
            let a1401 = v15;
            length_bytes(a1401)
        };
        let a1403 = v16;
        nat_sub(a1402, a1403)
    });
    let a1404 = v5;
    let a1405 = 0u64;
    if if nat_eq(a1404, a1405) {
        true
    } else {
        if f344({
            let a1406 = 1u64;
            let a1407 = v5;
            nat_lt(a1406, a1407)
        }, {
            let a1408 = f342(v0.clone(), v3)?;
            let a1409 = 48u64;
            nat_eq(a1408, a1409)
        }) {
            true
        } else {
            if f344(v6, {
                let a1410 = v8;
                let a1411 = v7;
                nat_eq(a1410, a1411)
            }) {
                true
            } else {
                f414(if v10 {
                    let a1412 = v14;
                    let a1413 = v13;
                    nat_eq(a1412, a1413)
                } else {
                    false
                }, {
                    let a1415 = 18u64;
                    let a1416 = {
                        let a1414 = v17.clone();
                        length_bytes(a1414)
                    };
                    nat_lt(a1415, a1416)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt18)>)
    } else {
        let m1420 = {
            let a1418 = {
                let a1417 = v17.clone();
                length_bytes(a1417)
            };
            let a1419 = 0u64;
            if nat_eq(a1418, a1419) {
                Some(0u64)
            } else {
                f416(v17)?
            }
        };
        match m1420 {
            None => {
                Ok(None::<(u64, Adt18)>)
            }
            Some(v18) => {
                let m1425 = f417(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a1421 = v8;
                    let a1422 = v7;
                    nat_sub(a1421, a1422)
                }), f344(v12, {
                    let a1423 = v11;
                    let a1424 = 45u64;
                    nat_eq(a1423, a1424)
                }), v18)?;
                match m1425 {
                    None => {
                        Ok(None::<(u64, Adt18)>)
                    }
                    Some(v19) => {
                        let a1428 = 19u64;
                        let a1429 = {
                            let a1427 = {
                                let (h1426, _) = v19.clone();
                                h1426
                            };
                            length_bytes(a1427)
                        };
                        if nat_lt(a1428, a1429) {
                            Ok(None::<(u64, Adt18)>)
                        } else {
                            let m1431 = f416({
                                let (h1430, _) = v19.clone();
                                h1430
                            })?;
                            match m1431 {
                                None => {
                                    Ok(None::<(u64, Adt18)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f344(v2, {
                                        let a1432 = 0u64;
                                        let a1433 = v20;
                                        nat_lt(a1432, a1433)
                                    });
                                    if f418(v21, v20) {
                                        Ok(Some((v14, {
                                            let a1435 = {
                                                let (_, h1434) = v19.clone();
                                                h1434
                                            };
                                            let a1436 = 0u64;
                                            if nat_eq(a1435, a1436) {
                                                Adt18::C1(v21, v20)
                                            } else {
                                                Adt18::C2(v21, v20, {
                                                    let (_, h1437) = v19;
                                                    h1437
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt18)>)
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

pub fn f346(v0: Bytes) -> Bytes {
    if f189(v0.clone(), Bytes::lit(&[0x0d])) {
        f35(v0.clone(), 0u64, {
            let a1439 = {
                let a1438 = v0;
                length_bytes(a1438)
            };
            let a1440 = 1u64;
            nat_sub(a1439, a1440)
        })
    } else {
        v0
    }
}

pub fn f347(v0: Adt13, v1: Bytes) -> Adt13 {
    let m1441 = v0;
    match m1441 {
        Adt13::C0(v3) => {
            Adt13::C0(v3)
        }
        Adt13::C1(v4, v5) => {
            Adt13::C1(v4, v5)
        }
        Adt13::C2(v2) => {
            Adt13::C2(f419(f49(), v2, v1))
        }
        Adt13::C3(r1442) => {
            let v6 = (*r1442).clone();
            Adt13::C3(std::rc::Rc::new(v6))
        }
        Adt13::C4(r1443) => {
            let v7 = (*r1443).clone();
            Adt13::C4(std::rc::Rc::new(v7))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r1444, r1445) => {
            let v8 = (*r1444).clone();
            let v9 = (*r1445).clone();
            Adt13::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt13::C7(v10, r1446, r1447) => {
            let v11 = (*r1446).clone();
            let v12 = (*r1447).clone();
            Adt13::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f348(v0: u64) -> R<Bytes> {
    f420(f421(), 10u64, v0)
}

pub fn f349(v0: Fn4, p1452: u64, p1453: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1451 = (p1452, p1453);
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let (v1, v2) = m1451;
                let m1448 = v1;
                if m1448 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1448 - 1;
                    let m1450 = {
                        let c1449: Fn4 = v0.clone();
                        c1449.apply(v2.clone())?
                    };
                    match m1450 {
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
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f422(v0, v1)
}

pub fn f351(v0: Fn5, p1457: List<(Bytes, Bytes)>, p1458: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1456 = (p1457, p1458);
    {
        let mut m1456 = m1456;
        loop {
            let step = {
                let (v1, v2) = m1456;
                let m1454 = v2;
                match m1454.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1455: Fn5 = v0.clone();
                            c1455.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f423(v0, v1)
}

pub fn f353(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f424(v0)
}

pub fn f354(v0: Fn6, p1462: List<Bytes>, p1463: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1461 = (p1462, p1463);
    {
        let mut m1461 = m1461;
        loop {
            let step = {
                let (v1, v2) = m1461;
                let m1459 = v2;
                match m1459.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1460: Fn6 = v0.clone();
                            c1460.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1461 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f425(v1)?, v0))
}

pub fn f356(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f263(v0)?, List::cons(f8(Str::lit(":")), List::cons(f263(v1)?, List::<Bytes>::nil())))))
}

pub fn f357(v0: Adt12) -> Bytes {
    let m1464 = v0;
    match m1464 {
        Adt12::C0 => {
            f8(Str::lit("Verdant Engine authority register"))
        }
        Adt12::C1 => {
            f8(Str::lit("Verdant Engine conformance ID register"))
        }
        Adt12::C2 => {
            f8(Str::lit("Verdant Engine release ledger"))
        }
        Adt12::C3 => {
            f8(Str::lit("Verdant Engine standards lock"))
        }
    }
}

pub fn f358(v0: Adt12) -> Bytes {
    let m1465 = v0;
    match m1465 {
        Adt12::C0 => {
            f8(Str::lit("The engine's imported authorities, each catalog pinned by sha256 and mapped to the IDs that realize it (realized_by)."))
        }
        Adt12::C1 => {
            f8(Str::lit("The engine's conformance IDs, each with level, suite, refusal and statement. Every engine capability Verdant Studio uses is cited by one of these IDs."))
        }
        Adt12::C2 => {
            f8(Str::lit("The engine's verdant/release-ledger/1: its current SemVer version, release epoch and change history."))
        }
        Adt12::C3 => {
            f8(Str::lit("The engine's prismpm/standards-lock/1: the standards it binds and the executable oracles, corpora and failure mappings it runs against them."))
        }
    }
}

pub fn f359(v0: Adt12) -> Bytes {
    let m1466 = v0;
    match m1466 {
        Adt12::C0 => {
            f8(Str::lit("application/toml"))
        }
        Adt12::C1 => {
            f8(Str::lit("application/toml"))
        }
        Adt12::C2 => {
            f8(Str::lit("application/json"))
        }
        Adt12::C3 => {
            f8(Str::lit("application/json"))
        }
    }
}

pub fn f360(v0: Bytes, v1: u64) -> Option<u64> {
    let a1467 = 0u64;
    let a1468 = v1;
    if nat_lt(a1467, a1468) {
        if f426(v0, {
            let a1469 = v1;
            let a1470 = 1u64;
            nat_sub(a1469, a1470)
        }) {
            Some({
                let a1471 = v1;
                let a1472 = 1u64;
                nat_sub(a1471, a1472)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f361(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1473 = v2;
    let a1474 = v0;
    if nat_lt(a1473, a1474) {
        if f426(v1, v2) {
            Ok(Some({
                let a1475 = v2;
                let a1476 = 1u64;
                nat_add(a1475, a1476)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f362(v0: (Bytes, Bytes), p1479: List<(Bytes, Bytes)>) -> bool {
    let m1478 = p1479;
    {
        let mut m1478 = m1478;
        loop {
            let step = {
                let v1 = m1478;
                let m1477 = v1;
                match m1477.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f427(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
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

pub fn f363(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1480 = v1;
    match m1480.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h1481, _) = v2;
                h1481
            }, v0.clone()) {
                List::cons({
                    let (_, h1482) = v2;
                    h1482
                }, f363(v0, v3))
            } else {
                f363(v0, v3)
            }
        }
    }
}

pub fn f364() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f365(v0: Adt13, v1: Bytes, v2: Bytes) -> bool {
    let m1483 = f79(v0, v1);
    match m1483 {
        None => {
            false
        }
        Some(v3) => {
            f184(v3, v2)
        }
    }
}

pub fn f366() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f367(v0: Bytes) -> R<bool> {
    let m1484 = f193(v0.clone(), f8(Str::lit("&&")))?;
    if match m1484 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1485 = f193(v0, f8(Str::lit("||")))?;
        match m1485 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f368(v0: Adt17, v1: Option<u64>) -> bool {
    let m1486 = v1;
    match m1486 {
        None => {
            true
        }
        Some(v2) => {
            f292(v2, match v0 {
                Adt17::C0(_, h1487, _, _) => {
                    h1487
                }
            })
        }
    }
}

pub fn f369(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a1489 = match v0 {
            Adt15::C0(_, _, h1488, _, _) => {
                h1488
            }
        };
        length_list(a1489)
    })
}

pub fn f370(v0: Bytes) -> Option<Adt22> {
    if f28(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f28(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f28(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f371(v0: Bytes, p1494: List<Bytes>, p1495: u64) -> R<Option<u64>> {
    let m1493 = (p1494, p1495);
    {
        let mut m1493 = m1493;
        loop {
            let step = {
                let (v1, v2) = m1493;
                let m1490 = v1;
                match m1490.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f28(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1491 = v2;
                                let a1492 = 1u64;
                                nat_add(a1491, a1492)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: Option<Adt13>) -> Option<u64> {
    let m1496 = v0;
    match m1496 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1497 = v1;
            match m1497 {
                Adt13::C0(_) => {
                    None::<u64>
                }
                Adt13::C1(v2, v3) => {
                    if v2 {
                        None::<u64>
                    } else {
                        Some(v3)
                    }
                }
                Adt13::C2(_) => {
                    None::<u64>
                }
                Adt13::C3(_) => {
                    None::<u64>
                }
                Adt13::C4(_) => {
                    None::<u64>
                }
                Adt13::C5 => {
                    None::<u64>
                }
                Adt13::C6(_, _) => {
                    None::<u64>
                }
                Adt13::C7(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f373(v0: Bytes, v1: u64) -> R<u64> {
    let m1500 = {
        let a1498 = v0;
        let a1499 = v1;
        index_bytes(a1498, a1499)
    };
    match m1500 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f62(v2)
        }
    }
}

pub fn f374(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1504 = {
        let a1501 = v1;
        let a1502 = v2;
        let a1503 = 1u64;
        slice_bytes(a1501, a1502, a1503)
    };
    match m1504 {
        None => {
            v0
        }
        Some(v3) => {
            let a1505 = v0;
            let a1506 = v3;
            append_bytes(a1505, a1506)
        }
    }
}

pub fn f375(p1509: List<Bytes>, p1510: List<Bytes>) -> List<Bytes> {
    let m1508 = (p1509, p1510);
    {
        let mut m1508 = m1508;
        loop {
            let step = {
                let (v0, v1) = m1508;
                let m1507 = v0;
                match m1507.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Fn7, p1514: List<Bytes>, p1515: List<(Bytes, ())>) -> List<Bytes> {
    let m1513 = (p1514, p1515);
    {
        let mut m1513 = m1513;
        loop {
            let step = {
                let (v1, v2) = m1513;
                let m1511 = v2;
                match m1511.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1512: Fn7 = v0.clone();
                            c1512.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1516, _) = v1;
        h1516
    }, v0)
}

pub fn f378(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f428(Fn12::F429, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f379(v0: Fn8, p1520: List<List<(Bytes, ())>>, p1521: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1519 = (p1520, p1521);
    {
        let mut m1519 = m1519;
        loop {
            let step = {
                let (v1, v2) = m1519;
                let m1517 = v2;
                match m1517.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1518: Fn8 = v0.clone();
                            c1518.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f381(v0: Fn9, p1526: u64, p1527: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1525 = (p1526, p1527);
    {
        let mut m1525 = m1525;
        loop {
            let step = {
                let (v1, v2) = m1525;
                let m1522 = v1;
                if m1522 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1522 - 1;
                    let m1524 = {
                        let c1523: Fn9 = v0.clone();
                        c1523.apply(v2.clone())?
                    };
                    match m1524 {
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
                Ok(next) => { m1525 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f430(v0)
}

pub fn f383(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f431(Fn13::F432, List::<(Bytes, ())>::nil(), v0)
}

pub fn f384(v0: Fn10, p1531: List<(Bytes, ())>, p1532: List<Bytes>) -> List<(Bytes, ())> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v1, v2) = m1530;
                let m1528 = v2;
                match m1528.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1529: Fn10 = v0.clone();
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

pub fn f385(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f386(p1535: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1534 = p1535;
    {
        let mut m1534 = m1534;
        loop {
            let step = {
                let v0 = m1534;
                let m1533 = v0;
                match m1533.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f433(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1534 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: Bytes, p1561: u64, p1562: u64, p1563: Bytes) -> R<Bytes> {
    let m1560 = (p1561, (p1562, p1563));
    {
        let mut m1560 = m1560;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1560;
                let m1536 = v2;
                if m1536 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1536 - 1;
                    let m1539 = {
                        let a1537 = v0.clone();
                        let a1538 = v1;
                        index_bytes(a1537, a1538)
                    };
                    match m1539 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f414({
                                let a1540 = v6;
                                let a1541 = 34u64;
                                nat_eq(a1540, a1541)
                            }, {
                                let a1542 = v6;
                                let a1543 = 92u64;
                                nat_eq(a1542, a1543)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1544 = v1;
                                    let a1545 = 1u64;
                                    nat_add(a1544, a1545)?
                                }, (v4, {
                                    let a1548 = v3;
                                    let a1549 = {
                                        let a1546 = f8(Str::lit("\\"));
                                        let a1547 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1546, a1547)
                                    };
                                    append_bytes(a1548, a1549)
                                })))
                            } else {
                                let a1550 = v6;
                                let a1551 = 10u64;
                                if nat_eq(a1550, a1551) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1552 = v1;
                                        let a1553 = 1u64;
                                        nat_add(a1552, a1553)?
                                    }, (v4, {
                                        let a1554 = v3;
                                        let a1555 = f8(Str::lit("\\n"));
                                        append_bytes(a1554, a1555)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1556 = v1;
                                        let a1557 = 1u64;
                                        nat_add(a1556, a1557)?
                                    }, (v4, {
                                        let a1558 = v3;
                                        let a1559 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1558, a1559)
                                    })))
                                }
                            }
                        }
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

pub fn f388(v0: Option<Adt13>, v1: Bytes) -> Bytes {
    let m1564 = v0;
    match m1564 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f47(v2, v1)
        }
    }
}

pub fn f389(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
    let m1565 = f255(v2.clone())?;
    match m1565 {
        None => {
            f435(v0, v1, v2)
        }
        Some(v3) => {
            f434(v0, v1, v3)
        }
    }
}

pub fn f390(v0: List<Bytes>) -> List<Bytes> {
    let m1566 = v0;
    match m1566.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1568 = {
                    let a1567 = v1.clone();
                    length_bytes(a1567)
                };
                let a1569 = 0u64;
                if nat_eq(a1568, a1569) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f390(v2))
        }
    }
}

pub fn f391(v0: List<Bytes>) -> Bytes {
    let m1570 = v0;
    match m1570.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f392(v0: Bytes) -> R<bool> {
    let m1572 = {
        let a1571 = v0.clone();
        length_bytes(a1571)
    };
    if m1572 == 0 {
        Ok(true)
    } else {
        let v1 = m1572 - 1;
        let m1575 = {
            let a1573 = v0;
            let a1574 = v1;
            index_bytes(a1573, a1574)
        };
        match m1575 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f436({
                    let v3: u8 = v2;
                    let a1620 = {
                        let a1596 = {
                            let a1584 = {
                                let a1578 = {
                                    let a1576 = v3;
                                    let a1577 = 128u8;
                                    fixed_u8::bit_and(a1576, a1577)
                                };
                                let a1579 = 128u8;
                                if equal(a1578, a1579) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1585 = {
                                let a1582 = {
                                    let a1580 = v3;
                                    let a1581 = 64u8;
                                    fixed_u8::bit_and(a1580, a1581)
                                };
                                let a1583 = 64u8;
                                if equal(a1582, a1583) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1584, a1585)?
                        };
                        let a1597 = {
                            let a1594 = {
                                let a1588 = {
                                    let a1586 = v3;
                                    let a1587 = 32u8;
                                    fixed_u8::bit_and(a1586, a1587)
                                };
                                let a1589 = 32u8;
                                if equal(a1588, a1589) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1595 = {
                                let a1592 = {
                                    let a1590 = v3;
                                    let a1591 = 16u8;
                                    fixed_u8::bit_and(a1590, a1591)
                                };
                                let a1593 = 16u8;
                                if equal(a1592, a1593) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1594, a1595)?
                        };
                        nat_add(a1596, a1597)?
                    };
                    let a1621 = {
                        let a1618 = {
                            let a1606 = {
                                let a1600 = {
                                    let a1598 = v3;
                                    let a1599 = 8u8;
                                    fixed_u8::bit_and(a1598, a1599)
                                };
                                let a1601 = 8u8;
                                if equal(a1600, a1601) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1607 = {
                                let a1604 = {
                                    let a1602 = v3;
                                    let a1603 = 4u8;
                                    fixed_u8::bit_and(a1602, a1603)
                                };
                                let a1605 = 4u8;
                                if equal(a1604, a1605) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1606, a1607)?
                        };
                        let a1619 = {
                            let a1616 = {
                                let a1610 = {
                                    let a1608 = v3;
                                    let a1609 = 2u8;
                                    fixed_u8::bit_and(a1608, a1609)
                                };
                                let a1611 = 2u8;
                                if equal(a1610, a1611) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1617 = {
                                let a1614 = {
                                    let a1612 = v3;
                                    let a1613 = 1u8;
                                    fixed_u8::bit_and(a1612, a1613)
                                };
                                let a1615 = 1u8;
                                if equal(a1614, a1615) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1616, a1617)?
                        };
                        nat_add(a1618, a1619)?
                    };
                    nat_add(a1620, a1621)?
                }))
            }
        }
    }
}

pub fn f393(v0: Adt18, v1: Bytes) -> bool {
    let a1623 = {
        let a1622 = f244(v0, v1);
        length_bytes(a1622)
    };
    let a1624 = 0u64;
    !nat_eq(a1623, a1624)
}

pub fn f394(v0: Adt18, v1: Bytes) -> bool {
    let m1625 = f241(v0, v1);
    match m1625 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f395(v0: Bytes, p1646: u64, p1647: u64, p1648: bool, p1649: Bytes) -> R<Bytes> {
    let m1645 = (p1646, (p1647, (p1648, p1649)));
    {
        let mut m1645 = m1645;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1645;
                let m1626 = v2;
                if m1626 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1626 - 1;
                    let m1629 = {
                        let a1627 = v0.clone();
                        let a1628 = v1;
                        index_bytes(a1627, a1628)
                    };
                    match m1629 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f437(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1630 = v1;
                                    let a1631 = 1u64;
                                    nat_add(a1630, a1631)?
                                }, (v5, (true, v4))))
                            } else {
                                if f344(f438(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1632 = v1;
                                        let a1633 = 1u64;
                                        nat_add(a1632, a1633)?
                                    }, (v5, (false, {
                                        let a1637 = v4;
                                        let a1638 = f35({
                                            let a1634 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1634)
                                        }, {
                                            let a1635 = v7;
                                            let a1636 = 97u64;
                                            nat_sub(a1635, a1636)
                                        }, 1u64);
                                        append_bytes(a1637, a1638)
                                    }))))
                                } else {
                                    if f414(f438(v7), f414(f439(v7), f440(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1639 = v1;
                                            let a1640 = 1u64;
                                            nat_add(a1639, a1640)?
                                        }, (v5, (false, {
                                            let a1641 = v4;
                                            let a1642 = f35(v0.clone(), v1, 1u64);
                                            append_bytes(a1641, a1642)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1643 = v1;
                                            let a1644 = 1u64;
                                            nat_add(a1643, a1644)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1645 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: Adt18, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt18)> = f441(f442(f241(v0.clone(), f8(Str::lit("$defs")))), f442(f241(v0.clone(), f8(Str::lit("definitions")))));
    let m1650 = f443(4096u64, v3.clone(), List::<Bytes>::nil(), f444(v3)?)?;
    match m1650 {
        Ok(v5) => {
            let m1651 = f445(4096u64, f446(f447(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1651 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f338(f212(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f397(v0: List<Adt13>, v1: Bytes, v2: Bytes, v3: Adt13) -> List<Adt13> {
    let m1652 = v0;
    match m1652.uncons() {
        None => {
            List::cons(v3, List::<Adt13>::nil())
        }
        Some((v4, v5)) => {
            if f28(f47(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f448(v5, v1, v2))
            } else {
                List::cons(v4, f397(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f398(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f338(f449(f81(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f399(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f348(v1)?;
    let v4: u64 = {
        let a1653 = v3.clone();
        length_bytes(a1653)
    };
    let v5: Bytes = {
        let a1654 = v2;
        let a1655 = v4;
        if nat_lt(a1654, a1655) {
            f7(List::cons(f35(v3.clone(), 0u64, {
                let a1656 = v4;
                let a1657 = v2;
                nat_sub(a1656, a1657)
            }), List::cons(f8(Str::lit(".")), List::cons(f35(v3, {
                let a1658 = v4;
                let a1659 = v2;
                nat_sub(a1658, a1659)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f450({
                let a1660 = v2;
                let a1661 = v4;
                nat_sub(a1660, a1661)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1662 = f8(Str::lit("-"));
        let a1663 = v5;
        Ok(append_bytes(a1662, a1663))
    } else {
        Ok(v5)
    }
}

pub fn f400(v0: Bytes) -> R<Bytes> {
    let a1668 = f411(v0.clone(), 0u64, {
        let a1665 = {
            let a1664 = v0.clone();
            length_bytes(a1664)
        };
        let a1666 = 1u64;
        nat_add(a1665, a1666)?
    })?;
    let a1669 = {
        let a1667 = v0.clone();
        length_bytes(a1667)
    };
    if nat_eq(a1668, a1669) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f263(v0)
    }
}

pub fn f401(v0: Adt18) -> R<List<Bytes>> {
    let m1670 = v0;
    match m1670 {
        Adt18::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C7(r1671, r1672) => {
            let v10 = (*r1671).clone();
            let v11 = (*r1672).clone();
            Ok(List::cons(f329(v10)?, f401(v11)?))
        }
        Adt18::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f402(v0: Adt18) -> R<List<(Bytes, Bytes)>> {
    let m1673 = v0;
    match m1673 {
        Adt18::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C8(v12, r1674, r1675) => {
            let v13 = (*r1674).clone();
            let v14 = (*r1675).clone();
            Ok(List::cons((v12, f329(v13)?), f402(v14)?))
        }
    }
}

pub fn f403(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f338(f451(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f404(v0: u32) -> Bytes {
    let a1704 = f452({
        let a1676 = f453(v0, 28u32);
        let a1677 = 15u32;
        fixed_u32::bit_and(a1676, a1677)
    });
    let a1705 = {
        let a1702 = f452({
            let a1678 = f453(v0, 24u32);
            let a1679 = 15u32;
            fixed_u32::bit_and(a1678, a1679)
        });
        let a1703 = {
            let a1700 = f452({
                let a1680 = f453(v0, 20u32);
                let a1681 = 15u32;
                fixed_u32::bit_and(a1680, a1681)
            });
            let a1701 = {
                let a1698 = f452({
                    let a1682 = f453(v0, 16u32);
                    let a1683 = 15u32;
                    fixed_u32::bit_and(a1682, a1683)
                });
                let a1699 = {
                    let a1696 = f452({
                        let a1684 = f453(v0, 12u32);
                        let a1685 = 15u32;
                        fixed_u32::bit_and(a1684, a1685)
                    });
                    let a1697 = {
                        let a1694 = f452({
                            let a1686 = f453(v0, 8u32);
                            let a1687 = 15u32;
                            fixed_u32::bit_and(a1686, a1687)
                        });
                        let a1695 = {
                            let a1692 = f452({
                                let a1688 = f453(v0, 4u32);
                                let a1689 = 15u32;
                                fixed_u32::bit_and(a1688, a1689)
                            });
                            let a1693 = f452({
                                let a1690 = v0;
                                let a1691 = 15u32;
                                fixed_u32::bit_and(a1690, a1691)
                            });
                            append_bytes(a1692, a1693)
                        };
                        append_bytes(a1694, a1695)
                    };
                    append_bytes(a1696, a1697)
                };
                append_bytes(a1698, a1699)
            };
            append_bytes(a1700, a1701)
        };
        append_bytes(a1702, a1703)
    };
    append_bytes(a1704, a1705)
}

pub fn f405(v0: Fn11, p1709: u64, p1710: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1708 = (p1709, p1710);
    {
        let mut m1708 = m1708;
        loop {
            let step = {
                let (v1, v2) = m1708;
                let m1706 = v1;
                if m1706 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1706 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1707: Fn11 = v0.clone();
                        c1707.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1712 = {
            let (h1711, _) = v3.clone();
            h1711
        };
        let a1713 = 64u64;
        nat_add(a1712, a1713)?
    }, f454({
        let (_, h1714) = v3;
        h1714
    }, f455(v0, v1, v2, {
        let (h1715, _) = v3;
        h1715
    })?)?))
}

pub fn f407() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f408(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1716 = f33(v1, 34u8)?;
    match m1716.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1717 = v3;
            match m1717.uncons() {
                None => {
                    Ok(f456(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f409(v0: List<Bytes>) -> List<Bytes> {
    let m1718 = v0;
    match m1718.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("authority = []"))) {
                f409(v2)
            } else {
                List::cons(v1, f409(v2))
            }
        }
    }
}

pub fn f410(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1721 = v3;
    let a1722 = f35(v0, v1, {
        let a1719 = v2;
        let a1720 = v1;
        nat_sub(a1719, a1720)
    });
    append_bytes(a1721, a1722)
}

pub fn f411(v0: Bytes, p1735: u64, p1736: u64) -> R<u64> {
    let m1734 = (p1735, p1736);
    {
        let mut m1734 = m1734;
        loop {
            let step = {
                let (v1, v2) = m1734;
                let m1723 = v2;
                if m1723 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1723 - 1;
                    let v4: u64 = f342(v0.clone(), v1)?;
                    let a1724 = v4;
                    let a1725 = 34u64;
                    if nat_eq(a1724, a1725) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1726 = v4;
                        let a1727 = 92u64;
                        if nat_eq(a1726, a1727) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1728 = v4;
                            let a1729 = 32u64;
                            if nat_lt(a1728, a1729) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1730 = v4;
                                let a1731 = 256u64;
                                if nat_eq(a1730, a1731) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1732 = v1;
                                        let a1733 = 1u64;
                                        nat_add(a1732, a1733)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f457(Fn14::F458(v0.clone()), {
        let a1738 = {
            let a1737 = v0;
            length_bytes(a1737)
        };
        let a1739 = 1u64;
        nat_add(a1738, a1739)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1743 = {
        let (_, h1740) = {
            let (_, h1741) = {
                let (h1742, _) = v2.clone();
                h1742
            };
            h1741
        };
        h1740
    };
    let a1744 = 1u64;
    if nat_eq(a1743, a1744) {
        Ok(Some(({
            let (h1745, _) = {
                let (h1746, _) = v2.clone();
                h1746
            };
            h1745
        }, {
            let (h1747, _) = {
                let (_, h1748) = {
                    let (h1749, _) = v2;
                    h1749
                };
                h1748
            };
            h1747
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f413(v0: Bytes, v1: u64) -> R<u64> {
    let (h1750, _) = f282(Fn3::F459(v0.clone()), {
        let a1752 = {
            let a1751 = v0;
            length_bytes(a1751)
        };
        let a1753 = 1u64;
        nat_add(a1752, a1753)?
    }, v1)?;
    Ok(h1750)
}

pub fn f414(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f415(v0: Bytes) -> R<u64> {
    let (h1754, _) = f282(Fn3::F460(v0.clone()), {
        let a1756 = {
            let a1755 = v0;
            length_bytes(a1755)
        };
        let a1757 = 1u64;
        nat_add(a1756, a1757)?
    }, 0u64)?;
    Ok(h1754)
}

pub fn f416(v0: Bytes) -> R<Option<u64>> {
    if f461(v0.clone())? {
        Ok(Some({
            let (_, h1758) = {
                let (h1759, _) = f462(Fn15::F463(v0.clone()), {
                    let a1760 = v0;
                    length_bytes(a1760)
                }, (0u64, 0u64))?;
                h1759
            };
            h1758
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f417(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a1762 = {
            let a1761 = v1.clone();
            length_bytes(a1761)
        };
        let a1763 = f464(v1)?;
        nat_sub(a1762, a1763)
    });
    let v5: Bytes = {
        let a1764 = v0;
        let a1765 = v4.clone();
        append_bytes(a1764, a1765)
    };
    let v6: u64 = f415(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a1767 = {
            let a1766 = v5;
            length_bytes(a1766)
        };
        let a1768 = v6;
        nat_sub(a1767, a1768)
    });
    let v8: u64 = {
        let a1769 = v4;
        length_bytes(a1769)
    };
    let v9: u64 = {
        let a1770 = v7.clone();
        length_bytes(a1770)
    };
    let a1771 = v9;
    let a1772 = 0u64;
    if nat_eq(a1771, a1772) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f414(v2, {
            let a1773 = v3;
            let a1774 = v8;
            nat_lt(a1773, a1774)
        }) {
            let v10: u64 = if v2 {
                let a1775 = v3;
                let a1776 = v8;
                nat_add(a1775, a1776)?
            } else {
                let a1777 = v8;
                let a1778 = v3;
                nat_sub(a1777, a1778)
            };
            let v11: u64 = f464(v7.clone())?;
            let a1779 = v10;
            let a1780 = v11;
            if nat_le(a1779, a1780) {
                Ok(Some((f35(v7, 0u64, {
                    let a1781 = v9;
                    let a1782 = v10;
                    nat_sub(a1781, a1782)
                }), 0u64)))
            } else {
                let a1785 = {
                    let a1783 = v10;
                    let a1784 = v11;
                    nat_sub(a1783, a1784)
                };
                let a1786 = 19u64;
                if nat_le(a1785, a1786) {
                    Ok(Some((f35(v7, 0u64, {
                        let a1787 = v9;
                        let a1788 = v11;
                        nat_sub(a1787, a1788)
                    }), {
                        let a1789 = v10;
                        let a1790 = v11;
                        nat_sub(a1789, a1790)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1791 = v3;
                let a1792 = v8;
                nat_sub(a1791, a1792)
            };
            let a1795 = {
                let a1793 = v9;
                let a1794 = v12;
                nat_add(a1793, a1794)?
            };
            let a1796 = 19u64;
            if nat_le(a1795, a1796) {
                Ok(Some(({
                    let a1797 = v7;
                    let a1798 = f450(v12);
                    append_bytes(a1797, a1798)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f418(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1799 = 0u64;
        let a1800 = v1;
        if nat_lt(a1799, a1800) {
            let a1801 = v1;
            let a1802 = 9223372036854775808u64;
            nat_le(a1801, a1802)
        } else {
            false
        }
    } else {
        let a1803 = v1;
        let a1804 = 9223372036854775807u64;
        nat_le(a1803, a1804)
    }
}

pub fn f419(p1807: List<Adt12>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1806 = p1807;
    {
        let mut m1806 = m1806;
        loop {
            let step = {
                let v0 = m1806;
                let m1805 = v0;
                match m1805.uncons() {
                    None => {
                        Err::<List<Adt12>, Bytes>(v1.clone())
                    }
                    Some((v3, v4)) => {
                        if f72(v1.clone(), f88(v3.clone())) {
                            Err::<List<Adt12>, Bytes>(f58(v3, v2.clone()))
                        } else {
                            Ok::<List<Adt12>, Bytes>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f420(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1808 = v2;
    let a1809 = 0u64;
    if nat_eq(a1808, a1809) {
        Ok(f465(v0, 0u64))
    } else {
        let (_, h1810) = {
            let (h1811, _) = f349(Fn4::F466(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1811
        };
        Ok(h1810)
    }
}

pub fn f421() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f422(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1815 = {
        let a1813 = v0.clone();
        let a1814 = {
            let (h1812, _) = v1.clone();
            h1812
        };
        index_bytes(a1813, a1814)
    };
    match m1815 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1817 = {
                    let (h1816, _) = v1.clone();
                    h1816
                };
                let a1818 = 1u64;
                nat_add(a1817, a1818)?
            }, {
                let a1821 = {
                    let (_, h1819) = v1;
                    h1819
                };
                let a1822 = f467(v0, {
                    let (h1820, _) = v1;
                    h1820
                }, v2)?;
                append_bytes(a1821, a1822)
            })))
        }
    }
}

pub fn f423(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1823 = v0.clone();
    match m1823.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1826 = {
                let (h1824, _) = v2;
                h1824
            };
            let a1827 = {
                let (h1825, _) = v1.clone();
                h1825
            };
            if equal(a1826, a1827) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f424(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f468(f469(Fn16::F470, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1832 = {
        let (h1828, _) = f471(Fn17::F472, {
            let a1830 = {
                let a1829 = v0;
                length_list(a1829)
            };
            let a1831 = 1u64;
            nat_add(a1830, a1831)?
        }, v1)?;
        h1828
    };
    match m1832.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f425(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1837 = {
        let a1834 = f263({
            let (h1833, _) = v0;
            h1833
        })?;
        let a1835 = Bytes::lit(&[0x3a]);
        append_bytes(a1834, a1835)
    };
    let a1838 = {
        let (_, h1836) = v0;
        h1836
    };
    Ok(append_bytes(a1837, a1838))
}

pub fn f426(v0: Bytes, v1: u64) -> bool {
    let m1839 = f173(v0, v1);
    match m1839 {
        None => {
            false
        }
        Some(v2) => {
            f473(v2)
        }
    }
}

pub fn f427(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f344(f28({
        let (h1840, _) = v0;
        h1840
    }, {
        let (h1841, _) = v1;
        h1841
    }), f28({
        let (_, h1842) = v0;
        h1842
    }, {
        let (_, h1843) = v1;
        h1843
    }))
}

pub fn f428(v0: Fn12, p1847: List<List<(Bytes, ())>>, p1848: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1846 = (p1847, p1848);
    {
        let mut m1846 = m1846;
        loop {
            let step = {
                let (v1, v2) = m1846;
                let m1844 = v2;
                match m1844.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1845: Fn12 = v0.clone();
                            c1845.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f430(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1850 = {
        let a1849 = v0.clone();
        length_list(a1849)
    };
    let a1851 = 1u64;
    if nat_le(a1850, a1851) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f474(v0)?))
    }
}

pub fn f431(v0: Fn13, p1855: List<(Bytes, ())>, p1856: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1854 = (p1855, p1856);
    {
        let mut m1854 = m1854;
        loop {
            let step = {
                let (v1, v2) = m1854;
                let m1852 = v2;
                match m1852.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1853: Fn13 = v0.clone();
                            c1853.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f433(p1860: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1859 = p1860;
    {
        let mut m1859 = m1859;
        loop {
            let step = {
                let v0 = m1859;
                let m1857 = v0;
                match m1857.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f475(v2, v1.clone(), 0u64, {
                            let a1858 = v1.clone();
                            length_bytes(a1858)
                        })? {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Adt1, v1: Bytes, v2: List<Adt13>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f121(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f51(v3.clone(), f121(f82(v0, f132())?, f8(Str::lit("at"))));
    let a1862 = {
        let a1861 = v4.clone();
        length_list(a1861)
    };
    let a1863 = 0u64;
    if nat_eq(a1862, a1863) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1866 = {
            let a1864 = v4;
            length_list(a1864)
        };
        let a1867 = {
            let a1865 = v3;
            length_list(a1865)
        };
        if nat_eq(a1866, a1867) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f476(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f435(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt13> = f319(f235(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt13> = f319(f235(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f98(f121(v3.clone(), f8(Str::lit("id"))), f121(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f121(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f51(v5.clone(), f98(v6.clone(), f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1869 = {
        let a1868 = v7.clone();
        length_list(a1868)
    };
    let a1870 = 0u64;
    if nat_eq(a1869, a1870) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f40({
            let a1873 = {
                let a1871 = v7;
                length_list(a1871)
            };
            let a1874 = {
                let a1872 = v5;
                length_list(a1872)
            };
            nat_eq(a1873, a1874)
        }) {
            Ok((false, f476(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f125(f98(f51(f477(v4.clone()), f52(f53(v0, f8(Str::lit("model/authorities.toml"))))?), f51(f478(v4), f98(v6, f121(v3, f8(Str::lit("id")))))))?;
            let a1876 = {
                let a1875 = v8.clone();
                length_list(a1875)
            };
            let a1877 = 0u64;
            if nat_eq(a1876, a1877) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f476(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f95(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f436(v0: u64) -> bool {
    let a1878 = v0;
    let a1879 = 47u64;
    nat_eq(a1878, a1879)
}

pub fn f437(v0: u64) -> bool {
    f414({
        let a1880 = v0;
        let a1881 = 95u64;
        nat_eq(a1880, a1881)
    }, f414({
        let a1882 = v0;
        let a1883 = 45u64;
        nat_eq(a1882, a1883)
    }, f414({
        let a1884 = v0;
        let a1885 = 32u64;
        nat_eq(a1884, a1885)
    }, f414({
        let a1886 = v0;
        let a1887 = 46u64;
        nat_eq(a1886, a1887)
    }, {
        let a1888 = v0;
        let a1889 = 47u64;
        nat_eq(a1888, a1889)
    }))))
}

pub fn f438(v0: u64) -> bool {
    f344({
        let a1890 = 97u64;
        let a1891 = v0;
        nat_le(a1890, a1891)
    }, {
        let a1892 = v0;
        let a1893 = 122u64;
        nat_le(a1892, a1893)
    })
}

pub fn f439(v0: u64) -> bool {
    f344({
        let a1894 = 65u64;
        let a1895 = v0;
        nat_le(a1894, a1895)
    }, {
        let a1896 = v0;
        let a1897 = 90u64;
        nat_le(a1896, a1897)
    })
}

pub fn f440(v0: u64) -> bool {
    f344({
        let a1898 = 48u64;
        let a1899 = v0;
        nat_le(a1898, a1899)
    }, {
        let a1900 = v0;
        let a1901 = 57u64;
        nat_le(a1900, a1901)
    })
}

pub fn f441(v0: List<(Bytes, Adt18)>, v1: List<(Bytes, Adt18)>) -> List<(Bytes, Adt18)> {
    let m1902 = v0;
    match m1902.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f441(v3, v1))
        }
    }
}

pub fn f442(v0: Option<Adt18>) -> List<(Bytes, Adt18)> {
    let m1903 = v0;
    match m1903 {
        None => {
            List::<(Bytes, Adt18)>::nil()
        }
        Some(v1) => {
            f479(v1)
        }
    }
}

pub fn f443(v0: u64, v1: List<(Bytes, Adt18)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt18)>, Bytes>> {
    let m1904 = v0;
    if m1904 == 0 {
        let m1905 = v1;
        match m1905.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::<(Bytes, Adt18)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt18)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1904 - 1;
        let m1906 = v1.clone();
        match m1906.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::<(Bytes, Adt18)>::nil()))
            }
            Some((_, _)) => {
                let m1907 = f480(v1.clone(), v2.clone(), v3.clone())?;
                match m1907 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt18)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f338(f444(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1910 = f443(v6, f481(v1, {
                            let (h1908, _) = v9.clone();
                            h1908
                        }), List::cons(f482({
                            let (h1909, _) = v9.clone();
                            h1909
                        })?, v2), v3)?;
                        match m1910 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt18)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f444(v0: List<(Bytes, Adt18)>) -> R<List<Bytes>> {
    let m1911 = v0;
    match m1911.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f482({
                let (h1912, _) = v1;
                h1912
            })?, f444(v2)?))
        }
    }
}

pub fn f445(p1918: u64, p1919: List<Adt25>, p1920: List<Bytes>, p1921: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1917 = (p1918, (p1919, (p1920, p1921)));
    {
        let mut m1917 = m1917;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1917;
                let m1913 = v0;
                if m1913 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1913 - 1;
                    let m1914 = v1;
                    match m1914.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1915 = v5;
                            match m1915 {
                                Adt25::C0(v8, v9) => {
                                    if f28(v8.clone(), f483()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f41(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1916 = f485(v8.clone(), v9)?;
                                            match m1916 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f446(f486(v11.clone()), List::cons(Adt25::C1(f487(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f41(f483(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f483(), v2), List::cons(f484(), v3)))))
                                    }
                                }
                            }
                        }
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

pub fn f446(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m1922 = v0;
    match m1922.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f446(v3, v1))
        }
    }
}

pub fn f447(v0: List<(Bytes, Adt18)>) -> R<List<Adt25>> {
    let m1923 = v0;
    match m1923.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f482({
                let (h1924, _) = v1;
                h1924
            })?, {
                let (_, h1925) = v1;
                h1925
            }), f447(v2)?))
        }
    }
}

pub fn f448(v0: List<Adt13>, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1926 = v0;
    match m1926.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            if f28(f47(v3.clone(), v1.clone()), v2.clone()) {
                f448(v4, v1, v2)
            } else {
                List::cons(v3, f448(v4, v1, v2))
            }
        }
    }
}

pub fn f449(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1927 = v0;
    match m1927.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f72(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f338(f451(f125(List::cons(v1, f488(f33(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f449(v3, v1)?))
            }
        }
    }
}

pub fn f450(v0: u64) -> Bytes {
    let m1928 = v0;
    if m1928 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1928 - 1;
        let a1929 = f8(Str::lit("0"));
        let a1930 = f450(v1);
        append_bytes(a1929, a1930)
    }
}

pub fn f451(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1931 = v0;
    match m1931.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f302(v1)?, f451(v2)?))
        }
    }
}

pub fn f452(v0: u32) -> Bytes {
    let a1934 = {
        let a1932 = v0;
        let a1933 = 8u32;
        fixed_u32::bit_and(a1932, a1933)
    };
    let a1935 = 0u32;
    if equal(a1934, a1935) {
        let a1938 = {
            let a1936 = v0;
            let a1937 = 4u32;
            fixed_u32::bit_and(a1936, a1937)
        };
        let a1939 = 0u32;
        if equal(a1938, a1939) {
            let a1942 = {
                let a1940 = v0;
                let a1941 = 2u32;
                fixed_u32::bit_and(a1940, a1941)
            };
            let a1943 = 0u32;
            if equal(a1942, a1943) {
                let a1946 = {
                    let a1944 = v0;
                    let a1945 = 1u32;
                    fixed_u32::bit_and(a1944, a1945)
                };
                let a1947 = 0u32;
                if equal(a1946, a1947) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1950 = {
                    let a1948 = v0;
                    let a1949 = 1u32;
                    fixed_u32::bit_and(a1948, a1949)
                };
                let a1951 = 0u32;
                if equal(a1950, a1951) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
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
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1962 = {
                    let a1960 = v0;
                    let a1961 = 1u32;
                    fixed_u32::bit_and(a1960, a1961)
                };
                let a1963 = 0u32;
                if equal(a1962, a1963) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1966 = {
            let a1964 = v0;
            let a1965 = 4u32;
            fixed_u32::bit_and(a1964, a1965)
        };
        let a1967 = 0u32;
        if equal(a1966, a1967) {
            let a1970 = {
                let a1968 = v0;
                let a1969 = 2u32;
                fixed_u32::bit_and(a1968, a1969)
            };
            let a1971 = 0u32;
            if equal(a1970, a1971) {
                let a1974 = {
                    let a1972 = v0;
                    let a1973 = 1u32;
                    fixed_u32::bit_and(a1972, a1973)
                };
                let a1975 = 0u32;
                if equal(a1974, a1975) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1978 = {
                    let a1976 = v0;
                    let a1977 = 1u32;
                    fixed_u32::bit_and(a1976, a1977)
                };
                let a1979 = 0u32;
                if equal(a1978, a1979) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
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
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1990 = {
                    let a1988 = v0;
                    let a1989 = 1u32;
                    fixed_u32::bit_and(a1988, a1989)
                };
                let a1991 = 0u32;
                if equal(a1990, a1991) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f453(v0: u32, v1: u32) -> u32 {
    let m1994 = {
        let a1992 = v0;
        let a1993 = v1;
        fixed_u32::shift_right(a1992, a1993)
    };
    match m1994 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f454(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f489(Fn18::F490, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h1995, _, _, _, _, _, _, _) => {
            h1995
        }
    }, match v0.clone() {
        Adt24::C0(_, h1996, _, _, _, _, _, _) => {
            h1996
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h1997, _, _, _, _, _) => {
            h1997
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h1998, _, _, _, _) => {
            h1998
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h1999, _, _, _) => {
            h1999
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2000, _, _) => {
            h2000
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2001, _) => {
            h2001
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2002) => {
            h2002
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f491(match v0.clone() {
        Adt24::C0(h2003, _, _, _, _, _, _, _) => {
            h2003
        }
    }, match v2.clone() {
        Adt27::C0(h2004, _, _, _, _, _, _, _, _, _) => {
            h2004
        }
    }), f491(match v0.clone() {
        Adt24::C0(_, h2005, _, _, _, _, _, _) => {
            h2005
        }
    }, match v2.clone() {
        Adt27::C0(_, h2006, _, _, _, _, _, _, _, _) => {
            h2006
        }
    }), f491(match v0.clone() {
        Adt24::C0(_, _, h2007, _, _, _, _, _) => {
            h2007
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2008, _, _, _, _, _, _, _) => {
            h2008
        }
    }), f491(match v0.clone() {
        Adt24::C0(_, _, _, h2009, _, _, _, _) => {
            h2009
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2010, _, _, _, _, _, _) => {
            h2010
        }
    }), f491(match v0.clone() {
        Adt24::C0(_, _, _, _, h2011, _, _, _) => {
            h2011
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2012, _, _, _, _, _) => {
            h2012
        }
    }), f491(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2013, _, _) => {
            h2013
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2014, _, _, _, _) => {
            h2014
        }
    }), f491(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2015, _) => {
            h2015
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2016, _, _, _) => {
            h2016
        }
    }), f491(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2017) => {
            h2017
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2018, _, _) => {
            h2018
        }
    })))
}

pub fn f455(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f492(v0.clone(), v1, v2, v3)?, f492(v0.clone(), v1, v2, {
        let a2019 = v3;
        let a2020 = 4u64;
        nat_add(a2019, a2020)?
    })?, f492(v0.clone(), v1, v2, {
        let a2021 = v3;
        let a2022 = 8u64;
        nat_add(a2021, a2022)?
    })?, f492(v0.clone(), v1, v2, {
        let a2023 = v3;
        let a2024 = 12u64;
        nat_add(a2023, a2024)?
    })?, f492(v0.clone(), v1, v2, {
        let a2025 = v3;
        let a2026 = 16u64;
        nat_add(a2025, a2026)?
    })?, f492(v0.clone(), v1, v2, {
        let a2027 = v3;
        let a2028 = 20u64;
        nat_add(a2027, a2028)?
    })?, f492(v0.clone(), v1, v2, {
        let a2029 = v3;
        let a2030 = 24u64;
        nat_add(a2029, a2030)?
    })?, f492(v0.clone(), v1, v2, {
        let a2031 = v3;
        let a2032 = 28u64;
        nat_add(a2031, a2032)?
    })?, f492(v0.clone(), v1, v2, {
        let a2033 = v3;
        let a2034 = 32u64;
        nat_add(a2033, a2034)?
    })?, f492(v0.clone(), v1, v2, {
        let a2035 = v3;
        let a2036 = 36u64;
        nat_add(a2035, a2036)?
    })?, f492(v0.clone(), v1, v2, {
        let a2037 = v3;
        let a2038 = 40u64;
        nat_add(a2037, a2038)?
    })?, f492(v0.clone(), v1, v2, {
        let a2039 = v3;
        let a2040 = 44u64;
        nat_add(a2039, a2040)?
    })?, f492(v0.clone(), v1, v2, {
        let a2041 = v3;
        let a2042 = 48u64;
        nat_add(a2041, a2042)?
    })?, f492(v0.clone(), v1, v2, {
        let a2043 = v3;
        let a2044 = 52u64;
        nat_add(a2043, a2044)?
    })?, f492(v0.clone(), v1, v2, {
        let a2045 = v3;
        let a2046 = 56u64;
        nat_add(a2045, a2046)?
    })?, f492(v0, v1, v2, {
        let a2047 = v3;
        let a2048 = 60u64;
        nat_add(a2047, a2048)?
    })?))
}

pub fn f456(p2051: List<Bytes>, v1: Bytes) -> bool {
    let m2050 = p2051;
    {
        let mut m2050 = m2050;
        loop {
            let step = {
                let v0 = m2050;
                let m2049 = v0;
                match m2049.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f72(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: Fn14, p2056: u64, p2057: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2055 = (p2056, p2057);
    {
        let mut m2055 = m2055;
        loop {
            let step = {
                let (v1, v2) = m2055;
                let m2052 = v1;
                if m2052 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2052 - 1;
                    let m2054 = {
                        let c2053: Fn14 = v0.clone();
                        c2053.apply(v2.clone())?
                    };
                    match m2054 {
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
                Ok(next) => { m2055 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f493(v0, v1)
}

pub fn f459(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f494(v0, v1)
}

pub fn f460(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f495(v0, v1)
}

pub fn f461(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2058 = v0.clone();
        length_bytes(a2058)
    };
    let a2059 = 0u64;
    let a2060 = v1;
    if nat_lt(a2059, a2060) {
        let a2061 = f496(v0.clone())?;
        let a2062 = v1;
        if nat_eq(a2061, a2062) {
            let a2063 = 1u64;
            let a2064 = v1;
            if nat_lt(a2063, a2064) {
                let m2067 = {
                    let a2065 = v0;
                    let a2066 = 0u64;
                    index_bytes(a2065, a2066)
                };
                match m2067 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2068 = v2;
                        let a2069 = 48u8;
                        Ok(!equal(a2068, a2069))
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

pub fn f462(v0: Fn15, p2074: u64, p2075: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2073 = (p2074, p2075);
    {
        let mut m2073 = m2073;
        loop {
            let step = {
                let (v1, v2) = m2073;
                let m2070 = v1;
                if m2070 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2070 - 1;
                    let m2072 = {
                        let c2071: Fn15 = v0.clone();
                        c2071.apply(v2)?
                    };
                    match m2072 {
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
                Ok(next) => { m2073 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f497(v0, v1)
}

pub fn f464(v0: Bytes) -> R<u64> {
    let (h2076, _) = f282(Fn3::F498(v0.clone()), {
        let a2078 = {
            let a2077 = v0;
            length_bytes(a2077)
        };
        let a2079 = 1u64;
        nat_add(a2078, a2079)?
    }, 0u64)?;
    Ok(h2076)
}

pub fn f465(v0: Bytes, v1: u64) -> Bytes {
    let m2083 = {
        let a2080 = v0;
        let a2081 = v1;
        let a2082 = 1u64;
        slice_bytes(a2080, a2081, a2082)
    };
    match m2083 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f466(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f499(v1, v0, v2)
}

pub fn f467(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2128 = {
            let a2104 = {
                let a2092 = {
                    let a2086 = {
                        let a2084 = v3;
                        let a2085 = 128u8;
                        fixed_u8::bit_and(a2084, a2085)
                    };
                    let a2087 = 128u8;
                    if equal(a2086, a2087) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2093 = {
                    let a2090 = {
                        let a2088 = v3;
                        let a2089 = 64u8;
                        fixed_u8::bit_and(a2088, a2089)
                    };
                    let a2091 = 64u8;
                    if equal(a2090, a2091) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2092, a2093)?
            };
            let a2105 = {
                let a2102 = {
                    let a2096 = {
                        let a2094 = v3;
                        let a2095 = 32u8;
                        fixed_u8::bit_and(a2094, a2095)
                    };
                    let a2097 = 32u8;
                    if equal(a2096, a2097) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2103 = {
                    let a2100 = {
                        let a2098 = v3;
                        let a2099 = 16u8;
                        fixed_u8::bit_and(a2098, a2099)
                    };
                    let a2101 = 16u8;
                    if equal(a2100, a2101) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2102, a2103)?
            };
            nat_add(a2104, a2105)?
        };
        let a2129 = {
            let a2126 = {
                let a2114 = {
                    let a2108 = {
                        let a2106 = v3;
                        let a2107 = 8u8;
                        fixed_u8::bit_and(a2106, a2107)
                    };
                    let a2109 = 8u8;
                    if equal(a2108, a2109) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2115 = {
                    let a2112 = {
                        let a2110 = v3;
                        let a2111 = 4u8;
                        fixed_u8::bit_and(a2110, a2111)
                    };
                    let a2113 = 4u8;
                    if equal(a2112, a2113) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2114, a2115)?
            };
            let a2127 = {
                let a2124 = {
                    let a2118 = {
                        let a2116 = v3;
                        let a2117 = 2u8;
                        fixed_u8::bit_and(a2116, a2117)
                    };
                    let a2119 = 2u8;
                    if equal(a2118, a2119) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2125 = {
                    let a2122 = {
                        let a2120 = v3;
                        let a2121 = 1u8;
                        fixed_u8::bit_and(a2120, a2121)
                    };
                    let a2123 = 1u8;
                    if equal(a2122, a2123) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2124, a2125)?
            };
            nat_add(a2126, a2127)?
        };
        nat_add(a2128, a2129)?
    };
    let a2130 = v4;
    let a2131 = 34u64;
    if nat_eq(a2130, a2131) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2132 = v4;
        let a2133 = 92u64;
        if nat_eq(a2132, a2133) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2134 = v4;
            let a2135 = 8u64;
            if nat_eq(a2134, a2135) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2136 = v4;
                let a2137 = 12u64;
                if nat_eq(a2136, a2137) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2138 = v4;
                    let a2139 = 10u64;
                    if nat_eq(a2138, a2139) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2140 = v4;
                        let a2141 = 13u64;
                        if nat_eq(a2140, a2141) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2142 = v4;
                            let a2143 = 9u64;
                            if nat_eq(a2142, a2143) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2144 = v4;
                                let a2145 = 32u64;
                                if nat_lt(a2144, a2145) {
                                    let a2146 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2147 = f500(v2)?;
                                    Ok(append_bytes(a2146, a2147))
                                } else {
                                    Ok(f35(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f468(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f501(Fn19::F502, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f469(v0: Fn16, p2151: List<List<(Bytes, Bytes)>>, p2152: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2150 = (p2151, p2152);
    {
        let mut m2150 = m2150;
        loop {
            let step = {
                let (v1, v2) = m2150;
                let m2148 = v2;
                match m2148.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2149: Fn16 = v0.clone();
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

pub fn f470(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f471(v0: Fn17, p2157: u64, p2158: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2156 = (p2157, p2158);
    {
        let mut m2156 = m2156;
        loop {
            let step = {
                let (v1, v2) = m2156;
                let m2153 = v1;
                if m2153 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2153 - 1;
                    let m2155 = {
                        let c2154: Fn17 = v0.clone();
                        c2154.apply(v2.clone())?
                    };
                    match m2155 {
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
                Ok(next) => { m2156 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f503(v0)
}

pub fn f473(v0: u8) -> bool {
    let a2159 = v0;
    let a2160 = 32u8;
    if if if equal(a2159, a2160) {
        true
    } else {
        let a2161 = v0;
        let a2162 = 9u8;
        equal(a2161, a2162)
    } {
        true
    } else {
        let a2163 = v0;
        let a2164 = 10u8;
        equal(a2163, a2164)
    } {
        true
    } else {
        let a2165 = v0;
        let a2166 = 12u8;
        if equal(a2165, a2166) {
            true
        } else {
            let a2167 = v0;
            let a2168 = 13u8;
            equal(a2167, a2168)
        }
    }
}

pub fn f474(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f378({
        let (_, h2169) = {
            let (h2170, _) = f504(Fn20::F505, {
                let a2172 = {
                    let a2171 = v0.clone();
                    length_list(a2171)
                };
                let a2173 = 1u64;
                nat_add(a2172, a2173)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2170
        };
        h2169
    }))
}

pub fn f475(v0: Bytes, v1: Bytes, p2186: u64, p2187: u64) -> R<bool> {
    let m2185 = (p2186, p2187);
    {
        let mut m2185 = m2185;
        loop {
            let step = {
                let (v2, v3) = m2185;
                let m2174 = v3;
                if m2174 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m2174 - 1;
                    let m2177 = {
                        let a2175 = v1.clone();
                        let a2176 = v2;
                        index_bytes(a2175, a2176)
                    };
                    match m2177 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m2180 = {
                                let a2178 = v0.clone();
                                let a2179 = v2;
                                index_bytes(a2178, a2179)
                            };
                            match m2180 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a2181 = v5;
                                    let a2182 = v6;
                                    if equal(a2181, a2182) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a2183 = v2;
                                            let a2184 = 1u64;
                                            nat_add(a2183, a2184)?
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
                Ok(next) => { m2185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f477(v0: List<Adt13>) -> List<Bytes> {
    let m2188 = v0;
    match m2188.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2189 = f79(v1, f8(Str::lit("std")));
            match m2189 {
                None => {
                    f477(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f477(v2))
                }
            }
        }
    }
}

pub fn f478(v0: List<Adt13>) -> List<Bytes> {
    let m2190 = v0;
    match m2190.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2191 = f78(f79(v1, f8(Str::lit("req"))));
            match m2191 {
                None => {
                    f478(v2)
                }
                Some(v3) => {
                    List::cons(v3, f478(v2))
                }
            }
        }
    }
}

pub fn f479(v0: Adt18) -> List<(Bytes, Adt18)> {
    let m2192 = v0;
    match m2192 {
        Adt18::C0(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C1(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C2(_, _, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C3(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C4(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C5(r2193) => {
            let v4 = (*r2193).clone();
            f479(v4)
        }
        Adt18::C6 => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C7(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C8(v1, r2194, r2195) => {
            let v2 = (*r2194).clone();
            let v3 = (*r2195).clone();
            List::cons((v1, v2), f479(v3))
        }
    }
}

pub fn f480(p2200: List<(Bytes, Adt18)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt18)>> {
    let m2199 = p2200;
    {
        let mut m2199 = m2199;
        loop {
            let step = {
                let v0 = m2199;
                let m2196 = v0;
                match m2196.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(Ok(None::<(Bytes, Adt18)>))
                    }
                    Some((v3, v4)) => {
                        if f506(f507({
                            let (_, h2197) = v3.clone();
                            h2197
                        }, List::<Bytes>::nil())?, f482({
                            let (h2198, _) = v3.clone();
                            h2198
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: List<(Bytes, Adt18)>, v1: Bytes) -> List<(Bytes, Adt18)> {
    let m2201 = v0;
    match m2201.uncons() {
        None => {
            List::<(Bytes, Adt18)>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h2202, _) = v2.clone();
                h2202
            }, v1.clone()) {
                f481(v3, v1)
            } else {
                List::cons(v2, f481(v3, v1))
            }
        }
    }
}

pub fn f482(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f324(v0)?;
    if f41(v1.clone(), f508()) {
        f509(v1)
    } else {
        Ok(v1)
    }
}

pub fn f483() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f484() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f485(v0: Bytes, v1: Adt18) -> R<Result<Adt28, Bytes>> {
    f510(v0, f511(v1))
}

pub fn f486(v0: Adt28) -> List<Adt25> {
    let m2203 = v0;
    match m2203 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f487(v0: Adt28) -> Bytes {
    let m2204 = v0;
    match m2204 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f488(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2205 = v0;
    match m2205.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f488(v3, false))
            } else {
                f488(v3, true)
            }
        }
    }
}

pub fn f489(v0: Fn18, p2209: u64, p2210: Adt27) -> R<Adt27> {
    let m2208 = (p2209, p2210);
    {
        let mut m2208 = m2208;
        loop {
            let step = {
                let (v1, v2) = m2208;
                let m2206 = v1;
                if m2206 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2206 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2207: Fn18 = v0.clone();
                        c2207.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: Adt27) -> R<Adt27> {
    f512(v0)
}

pub fn f491(v0: u32, v1: u32) -> u32 {
    let m2213 = {
        let a2211 = f513(v0);
        let a2212 = f513(v1);
        fixed_u64::checked_add(a2211, a2212)
    };
    match m2213 {
        None => {
            0u32
        }
        Some(v2) => {
            f514(v2)
        }
    }
}

pub fn f492(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2216 = {
        let a2214 = v3;
        let a2215 = 8u64;
        nat_add(a2214, a2215)?
    };
    let a2217 = v2;
    if nat_eq(a2216, a2217) {
        Ok(f515({
            let a2220 = {
                let a2218 = v1;
                let a2219 = 8u64;
                nat_mul(a2218, a2219)?
            };
            let a2221 = 4294967296u64;
            let a2222 = 0u64;
            nat_quot(a2220, a2221, a2222)
        }))
    } else {
        let a2225 = {
            let a2223 = v3;
            let a2224 = 4u64;
            nat_add(a2223, a2224)?
        };
        let a2226 = v2;
        if nat_eq(a2225, a2226) {
            Ok(f515({
                let a2229 = {
                    let a2227 = v1;
                    let a2228 = 8u64;
                    nat_mul(a2227, a2228)?
                };
                let a2230 = 4294967296u64;
                let a2231 = 0u64;
                nat_rem(a2229, a2230, a2231)
            }))
        } else {
            Ok(f516(f517(v0.clone(), v1, v3), f517(v0.clone(), v1, {
                let a2232 = v3;
                let a2233 = 1u64;
                nat_add(a2232, a2233)?
            }), f517(v0.clone(), v1, {
                let a2234 = v3;
                let a2235 = 2u64;
                nat_add(a2234, a2235)?
            }), f517(v0, v1, {
                let a2236 = v3;
                let a2237 = 3u64;
                nat_add(a2236, a2237)?
            })))
        }
    }
}

pub fn f493(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2238, _) = v1.clone();
        h2238
    };
    let v3: Bytes = {
        let (h2239, _) = {
            let (_, h2240) = v1.clone();
            h2240
        };
        h2239
    };
    let a2243 = {
        let (_, h2241) = {
            let (_, h2242) = v1;
            h2242
        };
        h2241
    };
    let a2244 = 0u64;
    if nat_eq(a2243, a2244) {
        let v4: u64 = f342(v0.clone(), v2)?;
        let a2245 = v4;
        let a2246 = 34u64;
        if nat_eq(a2245, a2246) {
            Ok(Some(({
                let a2247 = v2;
                let a2248 = 1u64;
                nat_add(a2247, a2248)?
            }, (v3, 1u64))))
        } else {
            let a2249 = v4;
            let a2250 = 92u64;
            if nat_eq(a2249, a2250) {
                f518(v0, v2, v3)
            } else {
                let a2251 = v4;
                let a2252 = 32u64;
                if if nat_lt(a2251, a2252) {
                    true
                } else {
                    let a2253 = v4;
                    let a2254 = 256u64;
                    nat_eq(a2253, a2254)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2255 = v2;
                        let a2256 = 1u64;
                        nat_add(a2255, a2256)?
                    }, ({
                        let a2257 = v3;
                        let a2258 = f35(v0, v2, 1u64);
                        append_bytes(a2257, a2258)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f494(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f342(v0, v1)?;
    let a2259 = 47u64;
    let a2260 = v2;
    if if nat_lt(a2259, a2260) {
        let a2261 = v2;
        let a2262 = 58u64;
        nat_lt(a2261, a2262)
    } else {
        false
    } {
        Ok(Some({
            let a2263 = v1;
            let a2264 = 1u64;
            nat_add(a2263, a2264)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f495(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2265 = f342(v0, v1)?;
    let a2266 = 48u64;
    if nat_eq(a2265, a2266) {
        Ok(Some({
            let a2267 = v1;
            let a2268 = 1u64;
            nat_add(a2267, a2268)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f496(v0: Bytes) -> R<u64> {
    let (h2269, _) = f282(Fn3::F519(v0.clone()), {
        let a2270 = v0;
        length_bytes(a2270)
    }, 0u64)?;
    Ok(h2269)
}

pub fn f497(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2274 = {
        let a2272 = v0;
        let a2273 = {
            let (h2271, _) = v1;
            h2271
        };
        index_bytes(a2272, a2273)
    };
    match m2274 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2276 = {
                    let (h2275, _) = v1;
                    h2275
                };
                let a2277 = 1u64;
                nat_add(a2276, a2277)?
            }, {
                let a2281 = {
                    let a2279 = {
                        let (_, h2278) = v1;
                        h2278
                    };
                    let a2280 = 10u64;
                    nat_mul(a2279, a2280)?
                };
                let a2282 = f520(v2)?;
                nat_add(a2281, a2282)?
            })))
        }
    }
}

pub fn f498(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f521(v0, v1)
}

pub fn f499(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2283, _) = v2;
        h2283
    };
    let a2284 = v3;
    let a2285 = 0u64;
    if nat_eq(a2284, a2285) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2286 = v3;
            let a2287 = v1;
            let a2288 = 0u64;
            nat_quot(a2286, a2287, a2288)
        }, {
            let a2293 = f465(v0, {
                let a2289 = v3;
                let a2290 = v1;
                let a2291 = 0u64;
                nat_rem(a2289, a2290, a2291)
            });
            let a2294 = {
                let (_, h2292) = v2;
                h2292
            };
            append_bytes(a2293, a2294)
        }))
    }
}

pub fn f500(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2339 = {
            let a2315 = {
                let a2303 = {
                    let a2297 = {
                        let a2295 = v1;
                        let a2296 = 128u8;
                        fixed_u8::bit_and(a2295, a2296)
                    };
                    let a2298 = 128u8;
                    if equal(a2297, a2298) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2304 = {
                    let a2301 = {
                        let a2299 = v1;
                        let a2300 = 64u8;
                        fixed_u8::bit_and(a2299, a2300)
                    };
                    let a2302 = 64u8;
                    if equal(a2301, a2302) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2303, a2304)?
            };
            let a2316 = {
                let a2313 = {
                    let a2307 = {
                        let a2305 = v1;
                        let a2306 = 32u8;
                        fixed_u8::bit_and(a2305, a2306)
                    };
                    let a2308 = 32u8;
                    if equal(a2307, a2308) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2314 = {
                    let a2311 = {
                        let a2309 = v1;
                        let a2310 = 16u8;
                        fixed_u8::bit_and(a2309, a2310)
                    };
                    let a2312 = 16u8;
                    if equal(a2311, a2312) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2313, a2314)?
            };
            nat_add(a2315, a2316)?
        };
        let a2340 = {
            let a2337 = {
                let a2325 = {
                    let a2319 = {
                        let a2317 = v1;
                        let a2318 = 8u8;
                        fixed_u8::bit_and(a2317, a2318)
                    };
                    let a2320 = 8u8;
                    if equal(a2319, a2320) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2326 = {
                    let a2323 = {
                        let a2321 = v1;
                        let a2322 = 4u8;
                        fixed_u8::bit_and(a2321, a2322)
                    };
                    let a2324 = 4u8;
                    if equal(a2323, a2324) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2325, a2326)?
            };
            let a2338 = {
                let a2335 = {
                    let a2329 = {
                        let a2327 = v1;
                        let a2328 = 2u8;
                        fixed_u8::bit_and(a2327, a2328)
                    };
                    let a2330 = 2u8;
                    if equal(a2329, a2330) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2336 = {
                    let a2333 = {
                        let a2331 = v1;
                        let a2332 = 1u8;
                        fixed_u8::bit_and(a2331, a2332)
                    };
                    let a2334 = 1u8;
                    if equal(a2333, a2334) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2335, a2336)?
            };
            nat_add(a2337, a2338)?
        };
        nat_add(a2339, a2340)?
    };
    let a2347 = f465(f522(), {
        let a2341 = v2;
        let a2342 = 16u64;
        let a2343 = 0u64;
        nat_quot(a2341, a2342, a2343)
    });
    let a2348 = f465(f522(), {
        let a2344 = v2;
        let a2345 = 16u64;
        let a2346 = 0u64;
        nat_rem(a2344, a2345, a2346)
    });
    Ok(append_bytes(a2347, a2348))
}

pub fn f501(v0: Fn19, p2352: List<List<(Bytes, Bytes)>>, p2353: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2351 = (p2352, p2353);
    {
        let mut m2351 = m2351;
        loop {
            let step = {
                let (v1, v2) = m2351;
                let m2349 = v2;
                match m2349.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2350: Fn19 = v0.clone();
                            c2350.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2351 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f503(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2355 = {
        let a2354 = v0.clone();
        length_list(a2354)
    };
    let a2356 = 1u64;
    if nat_le(a2355, a2356) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f523(v0)?))
    }
}

pub fn f504(v0: Fn20, p2361: u64, p2362: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2360 = (p2361, p2362);
    {
        let mut m2360 = m2360;
        loop {
            let step = {
                let (v1, v2) = m2360;
                let m2357 = v1;
                if m2357 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2357 - 1;
                    let m2359 = {
                        let c2358: Fn20 = v0.clone();
                        c2358.apply(v2.clone())?
                    };
                    match m2359 {
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
                Ok(next) => { m2360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f524(v0)
}

pub fn f506(p2365: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2364 = p2365;
    {
        let mut m2364 = m2364;
        loop {
            let step = {
                let v0 = m2364;
                let m2363 = v0;
                match m2363.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f28(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f41(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f41(v4, v3.clone())
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
                Ok(next) => { m2364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: Adt18, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2366 = v0;
    match m2366 {
        Adt18::C0(_) => {
            Ok(v1)
        }
        Adt18::C1(_, _) => {
            Ok(v1)
        }
        Adt18::C2(_, _, _) => {
            Ok(v1)
        }
        Adt18::C3(_) => {
            Ok(v1)
        }
        Adt18::C4(r2369) => {
            let v2 = (*r2369).clone();
            f507(v2, v1)
        }
        Adt18::C5(r2370) => {
            let v3 = (*r2370).clone();
            f507(v3, v1)
        }
        Adt18::C6 => {
            Ok(v1)
        }
        Adt18::C7(r2371, r2372) => {
            let v4 = (*r2371).clone();
            let v5 = (*r2372).clone();
            f507(v4, f507(v5, v1)?)
        }
        Adt18::C8(v6, r2373, r2374) => {
            let v7 = (*r2373).clone();
            let v8 = (*r2374).clone();
            let v9: List<Bytes> = f507(v7.clone(), f507(v8, v1)?)?;
            if f28(v6, f8(Str::lit("$ref"))) {
                let m2367 = f330(Some(v7));
                match m2367 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2368 = f525(v10)?;
                        match m2368 {
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

pub fn f508() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f509(v0: Bytes) -> R<Bytes> {
    let m2377 = {
        let a2375 = v0.clone();
        let a2376 = 0u64;
        index_bytes(a2375, a2376)
    };
    match m2377 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2383 = {
                let a2378 = f8(Str::lit("u"));
                let a2379 = f526(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2378, a2379)
            };
            let a2384 = f35(v0.clone(), 1u64, {
                let a2381 = {
                    let a2380 = v0;
                    length_bytes(a2380)
                };
                let a2382 = 1u64;
                nat_sub(a2381, a2382)
            });
            Ok(append_bytes(a2383, a2384))
        }
    }
}

pub fn f510(v0: Bytes, v1: Adt18) -> R<Result<Adt28, Bytes>> {
    let m2385 = f241(v1.clone(), f8(Str::lit("enum")));
    match m2385 {
        None => {
            let m2387 = f241(v1.clone(), f8(Str::lit("oneOf")));
            match m2387 {
                None => {
                    if f531(v1.clone(), v0.clone())? {
                        let m2388 = f532(v0.clone(), f442(f241(v1.clone(), f8(Str::lit("properties")))), f245(f246(f241(v1, f8(Str::lit("required"))))), true)?;
                        match m2388 {
                            Ok(v7) => {
                                Ok(f529(f533(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f338(f534(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f535(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2389 = f442(f241(v1.clone(), f8(Str::lit("properties"))));
                        match m2389.uncons() {
                            None => {
                                if f536(v1.clone()) {
                                    Ok(f529(f537(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2392 = f538(v1, {
                                        let a2390 = v0.clone();
                                        let a2391 = f8(Str::lit("Value"));
                                        append_bytes(a2390, a2391)
                                    })?;
                                    match m2392 {
                                        Ok(v9) => {
                                            Ok(f529(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f487(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f486(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2393 = f532(v0.clone(), f442(f241(v1.clone(), f8(Str::lit("properties")))), f245(f246(f241(v1, f8(Str::lit("required"))))), false)?;
                                match m2393 {
                                    Ok(v13) => {
                                        Ok(f529(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2394 = f534(v13.clone());
                                            match m2394.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f338(f534(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f535(v13)))
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
                    f530(v0, f331(v5))
                }
            }
        }
        Some(v2) => {
            let m2386 = f527(f331(v2), List::<Bytes>::nil())?;
            match m2386 {
                Ok(v4) => {
                    Ok(f529(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f338(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f528(v3, v0))
                }
            }
        }
    }
}

pub fn f511(v0: Adt18) -> Adt18 {
    let v2: Adt18 = {
        let m2395 = f241(v0.clone(), f8(Str::lit("anyOf")));
        match m2395 {
            None => {
                v0
            }
            Some(v1) => {
                if f539(f331(v1.clone())) {
                    f540(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f541(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f542(f331(v1.clone()))
                    } {
                        v0
                    } else {
                        f543(f540(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt18 = {
        let m2396 = f241(v2.clone(), f8(Str::lit("oneOf")));
        match m2396 {
            None => {
                v2
            }
            Some(v3) => {
                if f539(f331(v3)) {
                    f540(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2397 = f241(v4.clone(), f8(Str::lit("allOf")));
    match m2397 {
        None => {
            v4
        }
        Some(v5) => {
            if f544(f331(v5)) {
                f540(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f512(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f545(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2398) => {
            h2398
        }
    });
    let v2: u32 = f491(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2399, _, _) => {
            h2399
        }
    }, f491(f546(match v0.clone() {
        Adt27::C0(_, _, _, _, h2400, _, _, _, _, _) => {
            h2400
        }
    }), f491(f547(match v0.clone() {
        Adt27::C0(_, _, _, _, h2401, _, _, _, _, _) => {
            h2401
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2402, _, _, _, _) => {
            h2402
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2403, _, _, _) => {
            h2403
        }
    }), f491(v1, {
        let m2406 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2405, _) => {
                h2405
            }
        };
        match m2406 {
            Adt26::C0(h2404, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2404
            }
        }
    }))));
    let v3: u32 = f491(f548(match v0.clone() {
        Adt27::C0(h2407, _, _, _, _, _, _, _, _, _) => {
            h2407
        }
    }), f549(match v0.clone() {
        Adt27::C0(h2408, _, _, _, _, _, _, _, _, _) => {
            h2408
        }
    }, match v0.clone() {
        Adt27::C0(_, h2409, _, _, _, _, _, _, _, _) => {
            h2409
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2410, _, _, _, _, _, _, _) => {
            h2410
        }
    }));
    Ok(Adt27::C0(f491(v2, v3), match v0.clone() {
        Adt27::C0(h2411, _, _, _, _, _, _, _, _, _) => {
            h2411
        }
    }, match v0.clone() {
        Adt27::C0(_, h2412, _, _, _, _, _, _, _, _) => {
            h2412
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2413, _, _, _, _, _, _, _) => {
            h2413
        }
    }, f491(match v0.clone() {
        Adt27::C0(_, _, _, h2414, _, _, _, _, _, _) => {
            h2414
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2415, _, _, _, _, _) => {
            h2415
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2416, _, _, _, _) => {
            h2416
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2417, _, _, _) => {
            h2417
        }
    }, f550(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2418, _) => {
            h2418
        }
    }), {
        let a2420 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2419) => {
                h2419
            }
        };
        let a2421 = 1u64;
        nat_add(a2420, a2421)?
    }))
}

pub fn f513(v0: u32) -> u64 {
    let m2423 = {
        let a2422 = v0;
        fixed_u64::convert(i128::from(a2422))
    };
    match m2423 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f514(v0: u64) -> u32 {
    let m2427 = {
        let a2426 = {
            let a2424 = v0;
            let a2425 = 4294967295u64;
            fixed_u64::bit_and(a2424, a2425)
        };
        fixed_u32::convert(i128::from(a2426))
    };
    match m2427 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f515(v0: u64) -> u32 {
    let (_, h2428) = {
        let (_, h2429) = f551(Fn21::F552, 32u64, (v0, (1u32, 0u32)));
        h2429
    };
    h2428
}

pub fn f516(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2434 = f553(v0, 24u32);
    let a2435 = {
        let a2432 = f553(v1, 16u32);
        let a2433 = {
            let a2430 = f553(v2, 8u32);
            let a2431 = v3;
            fixed_u32::bit_or(a2430, a2431)
        };
        fixed_u32::bit_or(a2432, a2433)
    };
    fixed_u32::bit_or(a2434, a2435)
}

pub fn f517(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2436 = v2;
    let a2437 = v1;
    if nat_lt(a2436, a2437) {
        let m2440 = {
            let a2438 = v0;
            let a2439 = v2;
            index_bytes(a2438, a2439)
        };
        match m2440 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2442 = {
                    let a2441 = v3;
                    fixed_u32::convert(i128::from(a2441))
                };
                match m2442 {
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
        let a2443 = v2;
        let a2444 = v1;
        if nat_eq(a2443, a2444) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f518(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f342(v0.clone(), {
        let a2445 = v1;
        let a2446 = 1u64;
        nat_add(a2445, a2446)?
    })?;
    let v4: Bytes = {
        let a2447 = v3;
        let a2448 = 117u64;
        if nat_eq(a2447, a2448) {
            f554(v0, v1)?
        } else {
            f555(v3)
        }
    };
    let a2449 = v4.clone();
    let a2450 = Bytes::lit(&[]);
    if equal(a2449, a2450) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2451 = v3;
            let a2452 = 117u64;
            if nat_eq(a2451, a2452) {
                let a2453 = v1;
                let a2454 = 6u64;
                nat_add(a2453, a2454)?
            } else {
                let a2455 = v1;
                let a2456 = 2u64;
                nat_add(a2455, a2456)?
            }
        }, ({
            let a2457 = v2;
            let a2458 = v4;
            append_bytes(a2457, a2458)
        }, 0u64))))
    }
}

pub fn f519(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2461 = {
        let a2459 = v0;
        let a2460 = v1;
        index_bytes(a2459, a2460)
    };
    match m2461 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f556(v2)? {
                Ok(Some({
                    let a2462 = v1;
                    let a2463 = 1u64;
                    nat_add(a2462, a2463)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f520(v0: u8) -> R<u64> {
    let a2510 = {
        let v1: u8 = v0;
        let a2508 = {
            let a2484 = {
                let a2472 = {
                    let a2466 = {
                        let a2464 = v1;
                        let a2465 = 128u8;
                        fixed_u8::bit_and(a2464, a2465)
                    };
                    let a2467 = 128u8;
                    if equal(a2466, a2467) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2473 = {
                    let a2470 = {
                        let a2468 = v1;
                        let a2469 = 64u8;
                        fixed_u8::bit_and(a2468, a2469)
                    };
                    let a2471 = 64u8;
                    if equal(a2470, a2471) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2472, a2473)?
            };
            let a2485 = {
                let a2482 = {
                    let a2476 = {
                        let a2474 = v1;
                        let a2475 = 32u8;
                        fixed_u8::bit_and(a2474, a2475)
                    };
                    let a2477 = 32u8;
                    if equal(a2476, a2477) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2483 = {
                    let a2480 = {
                        let a2478 = v1;
                        let a2479 = 16u8;
                        fixed_u8::bit_and(a2478, a2479)
                    };
                    let a2481 = 16u8;
                    if equal(a2480, a2481) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2482, a2483)?
            };
            nat_add(a2484, a2485)?
        };
        let a2509 = {
            let a2506 = {
                let a2494 = {
                    let a2488 = {
                        let a2486 = v1;
                        let a2487 = 8u8;
                        fixed_u8::bit_and(a2486, a2487)
                    };
                    let a2489 = 8u8;
                    if equal(a2488, a2489) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2495 = {
                    let a2492 = {
                        let a2490 = v1;
                        let a2491 = 4u8;
                        fixed_u8::bit_and(a2490, a2491)
                    };
                    let a2493 = 4u8;
                    if equal(a2492, a2493) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2494, a2495)?
            };
            let a2507 = {
                let a2504 = {
                    let a2498 = {
                        let a2496 = v1;
                        let a2497 = 2u8;
                        fixed_u8::bit_and(a2496, a2497)
                    };
                    let a2499 = 2u8;
                    if equal(a2498, a2499) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2505 = {
                    let a2502 = {
                        let a2500 = v1;
                        let a2501 = 1u8;
                        fixed_u8::bit_and(a2500, a2501)
                    };
                    let a2503 = 1u8;
                    if equal(a2502, a2503) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2504, a2505)?
            };
            nat_add(a2506, a2507)?
        };
        nat_add(a2508, a2509)?
    };
    let a2511 = 48u64;
    Ok(nat_sub(a2510, a2511))
}

pub fn f521(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2512 = v0.clone();
        length_bytes(a2512)
    };
    let a2513 = v1;
    let a2514 = v2;
    if if nat_lt(a2513, a2514) {
        let a2519 = f342(v0, {
            let a2517 = v2;
            let a2518 = {
                let a2515 = v1;
                let a2516 = 1u64;
                nat_add(a2515, a2516)?
            };
            nat_sub(a2517, a2518)
        })?;
        let a2520 = 48u64;
        nat_eq(a2519, a2520)
    } else {
        false
    } {
        Ok(Some({
            let a2521 = v1;
            let a2522 = 1u64;
            nat_add(a2521, a2522)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f522() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f523(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f468({
        let (_, h2523) = {
            let (h2524, _) = f557(Fn22::F558, {
                let a2526 = {
                    let a2525 = v0.clone();
                    length_list(a2525)
                };
                let a2527 = 1u64;
                nat_add(a2526, a2527)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2524
        };
        h2523
    }))
}

pub fn f524(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2528, _) = v0;
        h2528
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2529) = v0;
        h2529
    };
    let m2530 = v1;
    match m2530.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2531 = v4;
            match m2531.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f559(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f525(v0: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f482(f35(v0.clone(), 8u64, {
            let a2533 = {
                let a2532 = v0;
                length_bytes(a2532)
            };
            let a2534 = 8u64;
            nat_sub(a2533, a2534)
        }))?))
    } else {
        if f72(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f482(f35(v0.clone(), 14u64, {
                let a2536 = {
                    let a2535 = v0;
                    length_bytes(a2535)
                };
                let a2537 = 14u64;
                nat_sub(a2536, a2537)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f526(p2554: u64, p2555: u64, p2556: u64, p2557: Bytes) -> Bytes {
    let m2553 = (p2554, (p2555, (p2556, p2557)));
    {
        let mut m2553 = m2553;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2553;
                let m2538 = v0;
                if m2538 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2538 - 1;
                    if f344({
                        let a2539 = v1;
                        let a2540 = 0u64;
                        nat_eq(a2539, a2540)
                    }, {
                        let a2541 = v2;
                        let a2542 = 0u64;
                        nat_eq(a2541, a2542)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2543 = v1;
                            let a2544 = 16u64;
                            let a2545 = 0u64;
                            nat_quot(a2543, a2544, a2545)
                        }, ({
                            let a2546 = v2;
                            let a2547 = 1u64;
                            nat_sub(a2546, a2547)
                        }, {
                            let a2551 = f560({
                                let a2548 = v1;
                                let a2549 = 16u64;
                                let a2550 = 0u64;
                                nat_rem(a2548, a2549, a2550)
                            });
                            let a2552 = v3;
                            append_bytes(a2551, a2552)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: List<Adt18>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f561(f562(f245(v0.clone()))?, v0, v1)
}

pub fn f528(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f529(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f530(v0: Bytes, v1: List<Adt18>) -> R<Result<Adt28, Bytes>> {
    let m2558 = v1.clone();
    match m2558.uncons() {
        None => {
            Ok(f528(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2559 = f563(f442(f241(v2, f8(Str::lit("properties")))), v1.clone());
            match m2559.uncons() {
                None => {
                    let m2560 = f564(f562(f565(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2560 {
                        Ok(v5) => {
                            Ok(f529(f533(v0, f534(v5.clone())), f535(v5)))
                        }
                        Err(v4) => {
                            Ok(f528(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2561 = f566(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2561 {
                        Ok(v9) => {
                            Ok(f529(f533(v0, f534(v9.clone())), f535(v9)))
                        }
                        Err(v8) => {
                            Ok(f528(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f531(v0: Adt18, v1: Bytes) -> R<bool> {
    let m2562 = v0;
    match m2562 {
        Adt18::C0(_) => {
            Ok(false)
        }
        Adt18::C1(_, _) => {
            Ok(false)
        }
        Adt18::C2(_, _, _) => {
            Ok(false)
        }
        Adt18::C3(_) => {
            Ok(false)
        }
        Adt18::C4(r2563) => {
            let v2 = (*r2563).clone();
            f531(v2, v1)
        }
        Adt18::C5(r2564) => {
            let v3 = (*r2564).clone();
            f531(v3, v1)
        }
        Adt18::C6 => {
            Ok(false)
        }
        Adt18::C7(r2565, r2566) => {
            let v4 = (*r2565).clone();
            let v5 = (*r2566).clone();
            if f531(v4, v1.clone())? {
                Ok(true)
            } else {
                f531(v5, v1)
            }
        }
        Adt18::C8(v6, r2567, r2568) => {
            let v7 = (*r2567).clone();
            let v8 = (*r2568).clone();
            if if f28(v6, f8(Str::lit("$ref"))) {
                f567(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f531(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f531(v8, v1)
                }
            }
        }
    }
}

pub fn f532(v0: Bytes, v1: List<(Bytes, Adt18)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f568(f562(f569(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f533(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f338(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f534(v0: Adt29) -> List<Bytes> {
    let m2569 = v0;
    match m2569 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f535(v0: Adt29) -> List<Adt25> {
    let m2570 = v0;
    match m2570 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f536(v0: Adt18) -> bool {
    let m2571 = f442(f241(v0.clone(), f8(Str::lit("properties"))));
    match m2571.uncons() {
        None => {
            let m2572 = f570(v0);
            match m2572 {
                None => {
                    false
                }
                Some(v3) => {
                    f571(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f537(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f538(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2573 = f572(v0.clone());
    match m2573 {
        None => {
            if f28(f244(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2574 = f241(v0, f8(Str::lit("items")));
                match m2574 {
                    None => {
                        Ok(f573(f8(Str::lit("List")), f575()))
                    }
                    Some(v3) => {
                        Ok(f573(f8(Str::lit("List")), f574(v3, {
                            let a2575 = v1;
                            let a2576 = f8(Str::lit("Item"));
                            append_bytes(a2575, a2576)
                        })?))
                    }
                }
            } else {
                f574(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f573(f8(Str::lit("Option")), f574(v2, v1)?))
        }
    }
}

pub fn f539(p2579: List<Adt18>) -> bool {
    let m2578 = p2579;
    {
        let mut m2578 = m2578;
        loop {
            let step = {
                let v0 = m2578;
                let m2577 = v0;
                match m2577.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f576(f479(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt18>, bool>(v2)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2578 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: Adt18, v1: Bytes) -> Adt18 {
    let m2580 = v0;
    match m2580 {
        Adt18::C0(v6) => {
            Adt18::C0(v6)
        }
        Adt18::C1(v10, v11) => {
            Adt18::C1(v10, v11)
        }
        Adt18::C2(v7, v8, v9) => {
            Adt18::C2(v7, v8, v9)
        }
        Adt18::C3(v12) => {
            Adt18::C3(v12)
        }
        Adt18::C4(r2581) => {
            let v13 = (*r2581).clone();
            Adt18::C4(std::rc::Rc::new(v13))
        }
        Adt18::C5(r2582) => {
            let v5 = (*r2582).clone();
            Adt18::C5(std::rc::Rc::new(f540(v5, v1)))
        }
        Adt18::C6 => {
            Adt18::C6
        }
        Adt18::C7(r2583, r2584) => {
            let v14 = (*r2583).clone();
            let v15 = (*r2584).clone();
            Adt18::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt18::C8(v2, r2585, r2586) => {
            let v3 = (*r2585).clone();
            let v4 = (*r2586).clone();
            if f28(v2.clone(), v1.clone()) {
                f540(v4, v1)
            } else {
                Adt18::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f540(v4, v1)))
            }
        }
    }
}

pub fn f541(v0: Adt18, v1: Bytes) -> bool {
    let m2587 = f241(v0, v1);
    match m2587 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f542(v0: List<Adt18>) -> bool {
    let m2588 = v0.clone();
    match m2588.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2589 = f563(f442(f241(v1, f8(Str::lit("properties")))), v0.clone());
            match m2589.uncons() {
                None => {
                    f578(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f577(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f543(v0: Adt18, v1: Bytes, v2: Adt18) -> Adt18 {
    let m2590 = v0;
    match m2590 {
        Adt18::C0(v7) => {
            Adt18::C0(v7)
        }
        Adt18::C1(v11, v12) => {
            Adt18::C1(v11, v12)
        }
        Adt18::C2(v8, v9, v10) => {
            Adt18::C2(v8, v9, v10)
        }
        Adt18::C3(v13) => {
            Adt18::C3(v13)
        }
        Adt18::C4(r2591) => {
            let v14 = (*r2591).clone();
            Adt18::C4(std::rc::Rc::new(v14))
        }
        Adt18::C5(r2592) => {
            let v3 = (*r2592).clone();
            Adt18::C5(std::rc::Rc::new(Adt18::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt18::C6 => {
            Adt18::C6
        }
        Adt18::C7(r2593, r2594) => {
            let v15 = (*r2593).clone();
            let v16 = (*r2594).clone();
            Adt18::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt18::C8(v4, r2595, r2596) => {
            let v5 = (*r2595).clone();
            let v6 = (*r2596).clone();
            Adt18::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f544(p2599: List<Adt18>) -> bool {
    let m2598 = p2599;
    {
        let mut m2598 = m2598;
        loop {
            let step = {
                let v0 = m2598;
                let m2597 = v0;
                match m2597.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f579(v1) {
                            Ok::<List<Adt18>, bool>(v2)
                        } else {
                            Err::<List<Adt18>, bool>(false)
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

pub fn f545(v0: u64) -> u32 {
    let a2600 = v0;
    let a2601 = 32u64;
    if nat_lt(a2600, a2601) {
        let a2602 = v0;
        let a2603 = 16u64;
        if nat_lt(a2602, a2603) {
            let a2604 = v0;
            let a2605 = 8u64;
            if nat_lt(a2604, a2605) {
                let a2606 = v0;
                let a2607 = 4u64;
                if nat_lt(a2606, a2607) {
                    let a2608 = v0;
                    let a2609 = 2u64;
                    if nat_lt(a2608, a2609) {
                        let a2610 = v0;
                        let a2611 = 1u64;
                        if nat_lt(a2610, a2611) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2612 = v0;
                        let a2613 = 3u64;
                        if nat_lt(a2612, a2613) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2614 = v0;
                    let a2615 = 6u64;
                    if nat_lt(a2614, a2615) {
                        let a2616 = v0;
                        let a2617 = 5u64;
                        if nat_lt(a2616, a2617) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2618 = v0;
                        let a2619 = 7u64;
                        if nat_lt(a2618, a2619) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2620 = v0;
                let a2621 = 12u64;
                if nat_lt(a2620, a2621) {
                    let a2622 = v0;
                    let a2623 = 10u64;
                    if nat_lt(a2622, a2623) {
                        let a2624 = v0;
                        let a2625 = 9u64;
                        if nat_lt(a2624, a2625) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2626 = v0;
                        let a2627 = 11u64;
                        if nat_lt(a2626, a2627) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2628 = v0;
                    let a2629 = 14u64;
                    if nat_lt(a2628, a2629) {
                        let a2630 = v0;
                        let a2631 = 13u64;
                        if nat_lt(a2630, a2631) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2632 = v0;
                        let a2633 = 15u64;
                        if nat_lt(a2632, a2633) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2634 = v0;
            let a2635 = 24u64;
            if nat_lt(a2634, a2635) {
                let a2636 = v0;
                let a2637 = 20u64;
                if nat_lt(a2636, a2637) {
                    let a2638 = v0;
                    let a2639 = 18u64;
                    if nat_lt(a2638, a2639) {
                        let a2640 = v0;
                        let a2641 = 17u64;
                        if nat_lt(a2640, a2641) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2642 = v0;
                        let a2643 = 19u64;
                        if nat_lt(a2642, a2643) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2644 = v0;
                    let a2645 = 22u64;
                    if nat_lt(a2644, a2645) {
                        let a2646 = v0;
                        let a2647 = 21u64;
                        if nat_lt(a2646, a2647) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2648 = v0;
                        let a2649 = 23u64;
                        if nat_lt(a2648, a2649) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2650 = v0;
                let a2651 = 28u64;
                if nat_lt(a2650, a2651) {
                    let a2652 = v0;
                    let a2653 = 26u64;
                    if nat_lt(a2652, a2653) {
                        let a2654 = v0;
                        let a2655 = 25u64;
                        if nat_lt(a2654, a2655) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2656 = v0;
                        let a2657 = 27u64;
                        if nat_lt(a2656, a2657) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2658 = v0;
                    let a2659 = 30u64;
                    if nat_lt(a2658, a2659) {
                        let a2660 = v0;
                        let a2661 = 29u64;
                        if nat_lt(a2660, a2661) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2662 = v0;
                        let a2663 = 31u64;
                        if nat_lt(a2662, a2663) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2664 = v0;
        let a2665 = 48u64;
        if nat_lt(a2664, a2665) {
            let a2666 = v0;
            let a2667 = 40u64;
            if nat_lt(a2666, a2667) {
                let a2668 = v0;
                let a2669 = 36u64;
                if nat_lt(a2668, a2669) {
                    let a2670 = v0;
                    let a2671 = 34u64;
                    if nat_lt(a2670, a2671) {
                        let a2672 = v0;
                        let a2673 = 33u64;
                        if nat_lt(a2672, a2673) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2674 = v0;
                        let a2675 = 35u64;
                        if nat_lt(a2674, a2675) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2676 = v0;
                    let a2677 = 38u64;
                    if nat_lt(a2676, a2677) {
                        let a2678 = v0;
                        let a2679 = 37u64;
                        if nat_lt(a2678, a2679) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2680 = v0;
                        let a2681 = 39u64;
                        if nat_lt(a2680, a2681) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2682 = v0;
                let a2683 = 44u64;
                if nat_lt(a2682, a2683) {
                    let a2684 = v0;
                    let a2685 = 42u64;
                    if nat_lt(a2684, a2685) {
                        let a2686 = v0;
                        let a2687 = 41u64;
                        if nat_lt(a2686, a2687) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2688 = v0;
                        let a2689 = 43u64;
                        if nat_lt(a2688, a2689) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2690 = v0;
                    let a2691 = 46u64;
                    if nat_lt(a2690, a2691) {
                        let a2692 = v0;
                        let a2693 = 45u64;
                        if nat_lt(a2692, a2693) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2694 = v0;
                        let a2695 = 47u64;
                        if nat_lt(a2694, a2695) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2696 = v0;
            let a2697 = 56u64;
            if nat_lt(a2696, a2697) {
                let a2698 = v0;
                let a2699 = 52u64;
                if nat_lt(a2698, a2699) {
                    let a2700 = v0;
                    let a2701 = 50u64;
                    if nat_lt(a2700, a2701) {
                        let a2702 = v0;
                        let a2703 = 49u64;
                        if nat_lt(a2702, a2703) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2704 = v0;
                        let a2705 = 51u64;
                        if nat_lt(a2704, a2705) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2706 = v0;
                    let a2707 = 54u64;
                    if nat_lt(a2706, a2707) {
                        let a2708 = v0;
                        let a2709 = 53u64;
                        if nat_lt(a2708, a2709) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2710 = v0;
                        let a2711 = 55u64;
                        if nat_lt(a2710, a2711) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2712 = v0;
                let a2713 = 60u64;
                if nat_lt(a2712, a2713) {
                    let a2714 = v0;
                    let a2715 = 58u64;
                    if nat_lt(a2714, a2715) {
                        let a2716 = v0;
                        let a2717 = 57u64;
                        if nat_lt(a2716, a2717) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2718 = v0;
                        let a2719 = 59u64;
                        if nat_lt(a2718, a2719) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2720 = v0;
                    let a2721 = 62u64;
                    if nat_lt(a2720, a2721) {
                        let a2722 = v0;
                        let a2723 = 61u64;
                        if nat_lt(a2722, a2723) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2724 = v0;
                        let a2725 = 63u64;
                        if nat_lt(a2724, a2725) {
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

pub fn f546(v0: u32) -> u32 {
    let a2728 = {
        let a2726 = f580(v0, 6u32, 26u32);
        let a2727 = f580(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2726, a2727)
    };
    let a2729 = f580(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2728, a2729)
}

pub fn f547(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2735 = {
        let a2730 = v0;
        let a2731 = v1;
        fixed_u32::bit_and(a2730, a2731)
    };
    let a2736 = {
        let a2733 = {
            let a2732 = v0;
            fixed_u32::bit_not(a2732)
        };
        let a2734 = v2;
        fixed_u32::bit_and(a2733, a2734)
    };
    fixed_u32::bit_xor(a2735, a2736)
}

pub fn f548(v0: u32) -> u32 {
    let a2739 = {
        let a2737 = f580(v0, 2u32, 30u32);
        let a2738 = f580(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2737, a2738)
    };
    let a2740 = f580(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2739, a2740)
}

pub fn f549(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2749 = {
        let a2745 = {
            let a2741 = v0;
            let a2742 = v1;
            fixed_u32::bit_and(a2741, a2742)
        };
        let a2746 = {
            let a2743 = v0;
            let a2744 = v2;
            fixed_u32::bit_and(a2743, a2744)
        };
        fixed_u32::bit_xor(a2745, a2746)
    };
    let a2750 = {
        let a2747 = v1;
        let a2748 = v2;
        fixed_u32::bit_and(a2747, a2748)
    };
    fixed_u32::bit_xor(a2749, a2750)
}

pub fn f550(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h2751, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2751
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2752, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2752
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h2753, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2753
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h2754, _, _, _, _, _, _, _, _, _, _, _) => {
            h2754
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2755, _, _, _, _, _, _, _, _, _, _) => {
            h2755
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2756, _, _, _, _, _, _, _, _, _) => {
            h2756
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h2757, _, _, _, _, _, _, _, _) => {
            h2757
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h2758, _, _, _, _, _, _, _) => {
            h2758
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2759, _, _, _, _, _, _) => {
            h2759
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h2760, _, _, _, _, _) => {
            h2760
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h2761, _, _, _, _) => {
            h2761
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2762, _, _, _) => {
            h2762
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2763, _, _) => {
            h2763
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2764, _) => {
            h2764
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2765) => {
            h2765
        }
    }, f491(f581(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2766, _) => {
            h2766
        }
    }), f491(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2767, _, _, _, _, _, _) => {
            h2767
        }
    }, f491(f582(match v0.clone() {
        Adt26::C0(_, h2768, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2768
        }
    }), match v0 {
        Adt26::C0(h2769, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2769
        }
    }))))
}

pub fn f551(v0: Fn21, p2773: u64, p2774: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2772 = (p2773, p2774);
    {
        let mut m2772 = m2772;
        loop {
            let step = {
                let (v1, v2) = m2772;
                let m2770 = v1;
                if m2770 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2770 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2771: Fn21 = v0.clone();
                        c2771.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2772 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2775, _) = v0;
        h2775
    };
    let v2: u32 = {
        let (h2776, _) = {
            let (_, h2777) = v0;
            h2777
        };
        h2776
    };
    let v3: u32 = {
        let (_, h2778) = {
            let (_, h2779) = v0;
            h2779
        };
        h2778
    };
    ({
        let a2780 = v1;
        let a2781 = 2u64;
        let a2782 = 0u64;
        nat_quot(a2780, a2781, a2782)
    }, (f553(v2, 1u32), {
        let a2786 = {
            let a2783 = v1;
            let a2784 = 2u64;
            let a2785 = 0u64;
            nat_rem(a2783, a2784, a2785)
        };
        let a2787 = 1u64;
        if nat_eq(a2786, a2787) {
            let a2788 = v3;
            let a2789 = v2;
            fixed_u32::bit_or(a2788, a2789)
        } else {
            v3
        }
    }))
}

pub fn f553(v0: u32, v1: u32) -> u32 {
    let m2792 = {
        let a2790 = v0;
        let a2791 = v1;
        fixed_u32::shift_left(a2790, a2791)
    };
    match m2792 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f554(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2795 = f35(v0.clone(), {
        let a2793 = v1;
        let a2794 = 2u64;
        nat_add(a2793, a2794)?
    }, 2u64);
    let a2796 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2795, a2796) {
        let m2799 = f583(f35(v0, {
            let a2797 = v1;
            let a2798 = 4u64;
            nat_add(a2797, a2798)?
        }, 2u64))?;
        match m2799 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f342(v2.clone(), 0u64)?;
                let a2800 = v3;
                let a2801 = 32u64;
                if nat_lt(a2800, a2801) {
                    if f584(v3) {
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

pub fn f555(v0: u64) -> Bytes {
    let a2802 = v0;
    let a2803 = 34u64;
    if nat_eq(a2802, a2803) {
        Bytes::lit(&[0x22])
    } else {
        let a2804 = v0;
        let a2805 = 92u64;
        if nat_eq(a2804, a2805) {
            Bytes::lit(&[0x5c])
        } else {
            let a2806 = v0;
            let a2807 = 98u64;
            if nat_eq(a2806, a2807) {
                Bytes::lit(&[0x08])
            } else {
                let a2808 = v0;
                let a2809 = 102u64;
                if nat_eq(a2808, a2809) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2810 = v0;
                    let a2811 = 110u64;
                    if nat_eq(a2810, a2811) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2812 = v0;
                        let a2813 = 114u64;
                        if nat_eq(a2812, a2813) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2814 = v0;
                            let a2815 = 116u64;
                            if nat_eq(a2814, a2815) {
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

pub fn f556(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2860 = {
            let a2836 = {
                let a2824 = {
                    let a2818 = {
                        let a2816 = v1;
                        let a2817 = 128u8;
                        fixed_u8::bit_and(a2816, a2817)
                    };
                    let a2819 = 128u8;
                    if equal(a2818, a2819) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2825 = {
                    let a2822 = {
                        let a2820 = v1;
                        let a2821 = 64u8;
                        fixed_u8::bit_and(a2820, a2821)
                    };
                    let a2823 = 64u8;
                    if equal(a2822, a2823) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2824, a2825)?
            };
            let a2837 = {
                let a2834 = {
                    let a2828 = {
                        let a2826 = v1;
                        let a2827 = 32u8;
                        fixed_u8::bit_and(a2826, a2827)
                    };
                    let a2829 = 32u8;
                    if equal(a2828, a2829) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2835 = {
                    let a2832 = {
                        let a2830 = v1;
                        let a2831 = 16u8;
                        fixed_u8::bit_and(a2830, a2831)
                    };
                    let a2833 = 16u8;
                    if equal(a2832, a2833) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2834, a2835)?
            };
            nat_add(a2836, a2837)?
        };
        let a2861 = {
            let a2858 = {
                let a2846 = {
                    let a2840 = {
                        let a2838 = v1;
                        let a2839 = 8u8;
                        fixed_u8::bit_and(a2838, a2839)
                    };
                    let a2841 = 8u8;
                    if equal(a2840, a2841) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2847 = {
                    let a2844 = {
                        let a2842 = v1;
                        let a2843 = 4u8;
                        fixed_u8::bit_and(a2842, a2843)
                    };
                    let a2845 = 4u8;
                    if equal(a2844, a2845) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2846, a2847)?
            };
            let a2859 = {
                let a2856 = {
                    let a2850 = {
                        let a2848 = v1;
                        let a2849 = 2u8;
                        fixed_u8::bit_and(a2848, a2849)
                    };
                    let a2851 = 2u8;
                    if equal(a2850, a2851) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2857 = {
                    let a2854 = {
                        let a2852 = v1;
                        let a2853 = 1u8;
                        fixed_u8::bit_and(a2852, a2853)
                    };
                    let a2855 = 1u8;
                    if equal(a2854, a2855) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2856, a2857)?
            };
            nat_add(a2858, a2859)?
        };
        nat_add(a2860, a2861)?
    };
    let a2862 = 48u64;
    let a2863 = v2;
    if nat_le(a2862, a2863) {
        let a2864 = v2;
        let a2865 = 57u64;
        Ok(nat_le(a2864, a2865))
    } else {
        Ok(false)
    }
}

pub fn f557(v0: Fn22, p2870: u64, p2871: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2869 = (p2870, p2871);
    {
        let mut m2869 = m2869;
        loop {
            let step = {
                let (v1, v2) = m2869;
                let m2866 = v1;
                if m2866 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2866 - 1;
                    let m2868 = {
                        let c2867: Fn22 = v0.clone();
                        c2867.apply(v2.clone())?
                    };
                    match m2868 {
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
                Ok(next) => { m2869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f558(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f585(v0)
}

pub fn f559(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2876 = {
            let a2874 = {
                let a2872 = v0.clone();
                length_list(a2872)
            };
            let a2875 = {
                let a2873 = v1.clone();
                length_list(a2873)
            };
            nat_add(a2874, a2875)?
        };
        let a2877 = 1u64;
        nat_add(a2876, a2877)?
    };
    Ok(f383({
        let (_, h2878) = {
            let (_, h2879) = {
                let (h2880, _) = f586(Fn23::F587, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2880
            };
            h2879
        };
        h2878
    }))
}

pub fn f560(v0: u64) -> Bytes {
    f35({
        let a2881 = Str::lit("0123456789ABCDEF");
        utf8_encode(a2881)
    }, v0, 1u64)
}

pub fn f561(v0: bool, v1: List<Adt18>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2882 = v1;
    match m2882.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2883 = f588(v0, v3.clone())?;
            match m2883 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f589(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2884 = f561(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2884 {
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

pub fn f562(p2888: List<Bytes>) -> R<bool> {
    let m2887 = p2888;
    {
        let mut m2887 = m2887;
        loop {
            let step = {
                let v0 = m2887;
                let m2885 = v0;
                match m2885.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2886 = f590(v1)?;
                        match m2886 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f41(v3, f591(false, v2.clone())?) {
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
                Ok(next) => { m2887 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563(v0: List<(Bytes, Adt18)>, v1: List<Adt18>) -> List<Bytes> {
    let m2889 = v0;
    match m2889.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f592(v1.clone(), {
                let (h2890, _) = v2.clone();
                h2890
            }) {
                List::cons({
                    let (h2891, _) = v2;
                    h2891
                }, f563(v3, v1))
            } else {
                f563(v3, v1)
            }
        }
    }
}

pub fn f564(v0: bool, v1: Bytes, v2: List<Adt18>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m2892 = v2;
    match m2892.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m2893 = f593(v4.clone())?;
            match m2893 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m2894 = f594(v0, v6.clone())?;
                    match m2894 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f41(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2897 = f595(v4, {
                                    let a2895 = v1.clone();
                                    let a2896 = f324(v6)?;
                                    append_bytes(a2895, a2896)
                                })?;
                                match m2897 {
                                    Ok(v9) => {
                                        let m2898 = f564(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m2898 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f487(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f534(v11.clone())), f446(f486(v9), f535(v11)))))
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

pub fn f565(v0: List<Adt18>) -> R<List<Bytes>> {
    let m2899 = v0;
    match m2899.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m2900 = f593(v1)?;
            match m2900 {
                None => {
                    f565(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f565(v2)?))
                }
            }
        }
    }
}

pub fn f566(v0: Bytes, v1: List<Adt18>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f596(f562(f597(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f567(v0: Adt18, v1: Bytes) -> R<bool> {
    let m2901 = f330(Some(v0));
    match m2901 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2902 = f525(v2)?;
            match m2902 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f28(v3, v1))
                }
            }
        }
    }
}

pub fn f568(v0: bool, v1: Bytes, v2: List<(Bytes, Adt18)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m2903 = v2;
    match m2903.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m2905 = f598(v0, {
                let (h2904, _) = v5.clone();
                h2904
            })?;
            match m2905 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h2906, _) = v5;
                        h2906
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f41(v7.clone(), f599(v0, f569(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2911 = f538({
                            let (_, h2907) = v5.clone();
                            h2907
                        }, {
                            let a2909 = v1.clone();
                            let a2910 = f324({
                                let (h2908, _) = v5.clone();
                                h2908
                            })?;
                            append_bytes(a2909, a2910)
                        })?;
                        match m2911 {
                            Ok(v9) => {
                                let v10: Bytes = if f41({
                                    let (h2912, _) = v5;
                                    h2912
                                }, v3.clone()) {
                                    f487(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f487(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m2913 = f568(v0, v1, v6, v3, v4)?;
                                match m2913 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f534(v12.clone())), f446(f486(v9), f535(v12)))))
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

pub fn f569(v0: List<(Bytes, Adt18)>) -> List<Bytes> {
    let m2914 = v0;
    match m2914.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2915, _) = v1;
                h2915
            }, f569(v2))
        }
    }
}

pub fn f570(v0: Adt18) -> Option<Adt18> {
    let m2916 = f241(v0.clone(), f8(Str::lit("additionalProperties")));
    match m2916 {
        None => {
            f600(v0)
        }
        Some(v1) => {
            if f571(v1.clone()) {
                let m2917 = f600(v0);
                match m2917 {
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

pub fn f571(v0: Adt18) -> bool {
    let m2918 = v0;
    match m2918 {
        Adt18::C0(v1) => {
            !v1
        }
        Adt18::C1(_, _) => {
            false
        }
        Adt18::C2(_, _, _) => {
            false
        }
        Adt18::C3(_) => {
            false
        }
        Adt18::C4(_) => {
            false
        }
        Adt18::C5(_) => {
            false
        }
        Adt18::C6 => {
            false
        }
        Adt18::C7(_, _) => {
            false
        }
        Adt18::C8(_, _, _) => {
            false
        }
    }
}

pub fn f572(v0: Adt18) -> Option<Adt18> {
    let m2919 = f246(f241(v0, f8(Str::lit("oneOf"))));
    match m2919.uncons() {
        None => {
            None::<Adt18>
        }
        Some((v1, v2)) => {
            let m2920 = v2;
            match m2920.uncons() {
                None => {
                    None::<Adt18>
                }
                Some((v3, v4)) => {
                    let m2921 = v4;
                    match m2921.uncons() {
                        None => {
                            if f601(v3.clone()) {
                                Some(v1)
                            } else {
                                if f601(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt18>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt18>
                        }
                    }
                }
            }
        }
    }
}

pub fn f573(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m2922 = v1;
    match m2922 {
        Ok(v3) => {
            f529(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f487(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f486(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f574(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2923 = f572(v0.clone());
    match m2923 {
        None => {
            if f28(f244(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2924 = f241(v0, f8(Str::lit("items")));
                match m2924 {
                    None => {
                        Ok(f573(f8(Str::lit("List")), f575()))
                    }
                    Some(v3) => {
                        Ok(f573(f8(Str::lit("List")), f595(v3, {
                            let a2925 = v1;
                            let a2926 = f8(Str::lit("Item"));
                            append_bytes(a2925, a2926)
                        })?))
                    }
                }
            } else {
                f595(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f573(f8(Str::lit("Option")), f595(v2, v1)?))
        }
    }
}

pub fn f575() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f483(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f576(p2930: List<(Bytes, Adt18)>, v1: List<Bytes>) -> bool {
    let m2929 = p2930;
    {
        let mut m2929 = m2929;
        loop {
            let step = {
                let v0 = m2929;
                let m2927 = v0;
                match m2927.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h2928, _) = v2;
                            h2928
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt18)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt18)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f577(p2934: List<Adt18>, v1: Bytes, p2935: List<Bytes>) -> bool {
    let m2933 = (p2934, p2935);
    {
        let mut m2933 = m2933;
        loop {
            let step = {
                let (v0, v2) = m2933;
                let m2931 = v0;
                match m2931.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m2932 = f602(v3, v1.clone());
                        match m2932 {
                            None => {
                                Err::<(List<Adt18>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f41(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt18>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt18>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2933 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(p2938: List<Adt18>, p2939: List<Bytes>) -> bool {
    let m2937 = (p2938, p2939);
    {
        let mut m2937 = m2937;
        loop {
            let step = {
                let (v0, v1) = m2937;
                let m2936 = v0;
                match m2936.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f244(v2, f8(Str::lit("type")));
                        if !f28(v4.clone(), Bytes::lit(&[])) {
                            if !f41(v4.clone(), v1.clone()) {
                                if !if if f28(v4.clone(), f8(Str::lit("integer"))) {
                                    f41(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f28(v4.clone(), f8(Str::lit("number"))) {
                                        f41(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt18>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt18>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt18>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt18>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2937 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: Adt18) -> bool {
    let v2: Adt18 = {
        let m2940 = f241(v0.clone(), f8(Str::lit("anyOf")));
        match m2940 {
            None => {
                Adt18::C6
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f576(f479(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f539(f331(v2))
    } else {
        false
    }
}

pub fn f580(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2941 = f453(v0, v1);
    let a2942 = f553(v0, v2);
    fixed_u32::bit_or(a2941, a2942)
}

pub fn f581(v0: u32) -> u32 {
    let a2945 = {
        let a2943 = f580(v0, 17u32, 15u32);
        let a2944 = f580(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2943, a2944)
    };
    let a2946 = f453(v0, 10u32);
    fixed_u32::bit_xor(a2945, a2946)
}

pub fn f582(v0: u32) -> u32 {
    let a2949 = {
        let a2947 = f580(v0, 7u32, 25u32);
        let a2948 = f580(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2947, a2948)
    };
    let a2950 = f453(v0, 3u32);
    fixed_u32::bit_xor(a2949, a2950)
}

pub fn f583(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2951 = v0.clone();
        length_bytes(a2951)
    };
    let a2955 = {
        let a2952 = v1;
        let a2953 = 2u64;
        let a2954 = 0u64;
        nat_rem(a2952, a2953, a2954)
    };
    let a2956 = 0u64;
    if nat_eq(a2955, a2956) {
        let (_, h2957) = {
            let (h2958, _) = f603(Fn24::F604(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2958
        };
        Ok(h2957)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f584(v0: u64) -> bool {
    let a2959 = v0;
    let a2960 = 8u64;
    if nat_eq(a2959, a2960) {
        true
    } else {
        let a2961 = v0;
        let a2962 = 9u64;
        if nat_eq(a2961, a2962) {
            true
        } else {
            let a2963 = v0;
            let a2964 = 10u64;
            if nat_eq(a2963, a2964) {
                true
            } else {
                let a2965 = v0;
                let a2966 = 12u64;
                if nat_eq(a2965, a2966) {
                    true
                } else {
                    let a2967 = v0;
                    let a2968 = 13u64;
                    nat_eq(a2967, a2968)
                }
            }
        }
    }
}

pub fn f585(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2969, _) = v0;
        h2969
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2970) = v0;
        h2970
    };
    let m2971 = v1;
    match m2971.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2972 = v4;
            match m2972.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f605(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f586(v0: Fn23, p2977: u64, p2978: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2976 = (p2977, p2978);
    {
        let mut m2976 = m2976;
        loop {
            let step = {
                let (v1, v2) = m2976;
                let m2973 = v1;
                if m2973 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2973 - 1;
                    let m2975 = {
                        let c2974: Fn23 = v0.clone();
                        c2974.apply(v2.clone())
                    };
                    match m2975 {
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
                Ok(next) => { m2976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f606(v0)
}

pub fn f588(v0: bool, v1: Adt18) -> R<Option<Bytes>> {
    let m2979 = v1;
    match m2979 {
        Adt18::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C1(v7, v8) => {
            Ok(Some({
                let a2990 = if f344(v7, {
                    let a2988 = 0u64;
                    let a2989 = v8;
                    nat_lt(a2988, a2989)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2991 = f262(false, v8)?;
                append_bytes(a2990, a2991)
            }))
        }
        Adt18::C2(v3, v4, v5) => {
            let m2982 = f607({
                let a2980 = f8(Str::lit("n"));
                let a2981 = f399(false, v4, v5)?;
                append_bytes(a2980, a2981)
            })?;
            match m2982 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2986 = f8(Str::lit("nMinus"));
                        let a2987 = f35(v6.clone(), 1u64, {
                            let a2984 = {
                                let a2983 = v6;
                                length_bytes(a2983)
                            };
                            let a2985 = 1u64;
                            nat_sub(a2984, a2985)
                        });
                        append_bytes(a2986, a2987)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt18::C3(v2) => {
            f594(v0, v2)
        }
        Adt18::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C6 => {
            Ok(None::<Bytes>)
        }
        Adt18::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt18::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f589(v0: Adt18) -> R<Bytes> {
    f329(v0)
}

pub fn f590(v0: Bytes) -> R<Option<Bytes>> {
    let m2992 = f608(v0)?;
    match m2992 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f609(v1)?))
        }
    }
}

pub fn f591(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2993 = v1;
    match m2993.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2994 = f594(v0, v2)?;
            match m2994 {
                None => {
                    f591(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f591(v0, v3)?))
                }
            }
        }
    }
}

pub fn f592(p2998: List<Adt18>, v1: Bytes) -> bool {
    let m2997 = p2998;
    {
        let mut m2997 = m2997;
        loop {
            let step = {
                let v0 = m2997;
                let m2995 = v0;
                match m2995.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m2996 = f602(v2, v1.clone());
                        if match m2996 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt18>, bool>(v3)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: Adt18) -> R<Option<Bytes>> {
    let m2999 = f330(f241(v0.clone(), f8(Str::lit("$ref"))));
    match m2999 {
        None => {
            Ok(f330(f241(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f525(v1)
        }
    }
}

pub fn f594(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f607(v1)
    } else {
        f590(v1)
    }
}

pub fn f595(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f610(f511(v0), v1)
}

pub fn f596(v0: bool, v1: Bytes, v2: List<Adt18>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3000 = v2;
    match m3000.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3001 = f602(v5.clone(), v3.clone());
            match m3001 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3002 = f594(v0, v7.clone())?;
                    match m3002 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f41(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3005 = f532({
                                    let a3003 = v1.clone();
                                    let a3004 = f324(v7)?;
                                    append_bytes(a3003, a3004)
                                }, f481(f442(f241(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f245(f246(f241(v5, f8(Str::lit("required"))))), true)?;
                                match m3005 {
                                    Ok(v10) => {
                                        let m3006 = f596(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3006 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f338(f534(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f534(v12.clone())), f446(f535(v10), f535(v12)))))
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

pub fn f597(v0: List<Adt18>, v1: Bytes) -> List<Bytes> {
    let m3007 = v0;
    match m3007.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3008 = f602(v2, v1.clone());
            match m3008 {
                None => {
                    f597(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f597(v3, v1))
                }
            }
        }
    }
}

pub fn f598(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3009 = f594(v0, v1)?;
    match m3009 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f28(v2.clone(), f8(Str::lit("mk"))) {
                f509(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f599(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3010 = v1;
    match m3010.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3011 = f598(v0, v2)?;
            match m3011 {
                None => {
                    f599(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f599(v0, v3)?))
                }
            }
        }
    }
}

pub fn f600(v0: Adt18) -> Option<Adt18> {
    let m3012 = f442(f241(v0, f8(Str::lit("patternProperties"))));
    match m3012.uncons() {
        None => {
            None::<Adt18>
        }
        Some((v1, v2)) => {
            let m3013 = v2;
            match m3013.uncons() {
                None => {
                    Some({
                        let (_, h3014) = v1;
                        h3014
                    })
                }
                Some((_, _)) => {
                    None::<Adt18>
                }
            }
        }
    }
}

pub fn f601(v0: Adt18) -> bool {
    f28(f244(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f602(v0: Adt18, v1: Bytes) -> Option<Bytes> {
    if f41(v1.clone(), f245(f246(f241(v0.clone(), f8(Str::lit("required")))))) {
        let m3015 = f241(v0, f8(Str::lit("properties")));
        match m3015 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3016 = f241(v2, v1);
                match m3016 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f330(f241(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f603(v0: Fn24, p3021: u64, p3022: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3020 = (p3021, p3022);
    {
        let mut m3020 = m3020;
        loop {
            let step = {
                let (v1, v2) = m3020;
                let m3017 = v1;
                if m3017 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3017 - 1;
                    let m3019 = {
                        let c3018: Fn24 = v0.clone();
                        c3018.apply(v2.clone())?
                    };
                    match m3019 {
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
                Ok(next) => { m3020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f611(v0, v1)
}

pub fn f605(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3027 = {
            let a3025 = {
                let a3023 = v0.clone();
                length_list(a3023)
            };
            let a3026 = {
                let a3024 = v1.clone();
                length_list(a3024)
            };
            nat_add(a3025, a3026)?
        };
        let a3028 = 1u64;
        nat_add(a3027, a3028)?
    };
    Ok(f612({
        let (_, h3029) = {
            let (_, h3030) = {
                let (h3031, _) = f613(Fn25::F614, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3031
            };
            h3030
        };
        h3029
    }))
}

pub fn f606(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3032, _) = v0.clone();
        h3032
    };
    let v2: List<(Bytes, ())> = {
        let (h3033, _) = {
            let (_, h3034) = v0.clone();
            h3034
        };
        h3033
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3035) = {
            let (_, h3036) = v0;
            h3036
        };
        h3035
    };
    let m3037 = v1.clone();
    match m3037.uncons() {
        None => {
            let m3038 = v2;
            match m3038.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3039 = v2.clone();
            match m3039.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f615({
                        let (h3040, _) = v8.clone();
                        h3040
                    }, {
                        let (h3041, _) = v6.clone();
                        h3041
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

pub fn f607(v0: Bytes) -> R<Option<Bytes>> {
    let m3043 = f616({
        let a3042 = v0.clone();
        length_bytes(a3042)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3043 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3045 = {
                let a3044 = v1.clone();
                length_bytes(a3044)
            };
            let a3046 = 0u64;
            if nat_eq(a3045, a3046) {
                let a3048 = {
                    let a3047 = v0;
                    length_bytes(a3047)
                };
                let a3049 = 0u64;
                if nat_eq(a3048, a3049) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f609(v1)?))
            }
        }
    }
}

pub fn f608(v0: Bytes) -> R<Option<Bytes>> {
    if f617({
        let a3050 = v0.clone();
        length_bytes(a3050)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f618(v0)?;
        if f619(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3053 = {
                let a3051 = v1.clone();
                let a3052 = 0u64;
                index_bytes(a3051, a3052)
            };
            match m3053 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f440(f62(v2)?) {
                        Ok(Some({
                            let a3054 = f8(Str::lit("n"));
                            let a3055 = v1;
                            append_bytes(a3054, a3055)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f620({
            let a3056 = v0.clone();
            length_bytes(a3056)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f609(v0: Bytes) -> R<Bytes> {
    if f41(v0.clone(), f621()) {
        f509(v0)
    } else {
        Ok(v0)
    }
}

pub fn f610(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3057 = v0.clone();
    match m3057 {
        Adt18::C0(v2) => {
            if v2 {
                Ok(f575())
            } else {
                Ok(f528(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt18::C1(_, _) => {
            Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C2(_, _, _) => {
            Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C3(_) => {
            Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C4(_) => {
            Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C5(_) => {
            let m3058 = f622(f479(v0.clone()));
            match m3058 {
                None => {
                    let m3059 = f330(f241(v0.clone(), f8(Str::lit("$ref"))));
                    match m3059 {
                        None => {
                            let m3061 = f241(v0.clone(), f8(Str::lit("allOf")));
                            match m3061 {
                                None => {
                                    let m3066 = f241(v0.clone(), f8(Str::lit("enum")));
                                    match m3066 {
                                        None => {
                                            let m3067 = f241(v0.clone(), f8(Str::lit("const")));
                                            match m3067 {
                                                None => {
                                                    let m3069 = f241(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3069 {
                                                        None => {
                                                            let v30: Bytes = f244(v0.clone(), f8(Str::lit("type")));
                                                            if if f28(v30.clone(), Bytes::lit(&[])) {
                                                                !f623(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f575())
                                                            } else {
                                                                if f28(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f529(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f28(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f529(f624(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f28(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f529(f625(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f28(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f529(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f28(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m3070 = f442(f241(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3070.uncons() {
                                                                                        None => {
                                                                                            if f536(v0.clone()) {
                                                                                                Ok(f529(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3071 = f570(v0);
                                                                                                match m3071 {
                                                                                                    None => {
                                                                                                        Ok(f573(f8(Str::lit("List")), f626(f575())))
                                                                                                    }
                                                                                                    Some(v33) => {
                                                                                                        Ok(f573(f8(Str::lit("List")), f626(f627(v33, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f529(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f528(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f529(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m3068 = f330(Some(v27));
                                                    match m3068 {
                                                        None => {
                                                            Ok(f528(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f529(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f529(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m3062 = f331(v19);
                                    match m3062.uncons() {
                                        None => {
                                            Ok(f528(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m3063 = v21;
                                            match m3063.uncons() {
                                                None => {
                                                    let m3064 = f330(f241(v20, f8(Str::lit("$ref"))));
                                                    match m3064 {
                                                        None => {
                                                            Ok(f528(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m3065 = f525(v24)?;
                                                            match m3065 {
                                                                None => {
                                                                    Ok(f528(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f529(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f528(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m3060 = f525(v17)?;
                            match m3060 {
                                None => {
                                    Ok(f528(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f529(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f528(if f28(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt18::C6 => {
            Ok(f575())
        }
        Adt18::C7(_, _) => {
            Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C8(_, _, _) => {
            Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f611(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3072, _) = v1;
        h3072
    };
    let m3074 = {
        let (_, h3073) = v1;
        h3073
    };
    match m3074 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3077 = {
                let a3075 = v0.clone();
                let a3076 = v2;
                index_bytes(a3075, a3076)
            };
            match m3077 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3082 = {
                        let a3080 = v0;
                        let a3081 = {
                            let a3078 = v2;
                            let a3079 = 1u64;
                            nat_add(a3078, a3079)?
                        };
                        index_bytes(a3080, a3081)
                    };
                    match m3082 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f628(v4)? {
                                f628(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3083 = v2;
                                    let a3084 = 2u64;
                                    nat_add(a3083, a3084)?
                                }, Some({
                                    let a3089 = v3;
                                    let a3090 = f465(f629(), {
                                        let a3087 = {
                                            let a3085 = f630(v4)?;
                                            let a3086 = 16u64;
                                            nat_mul(a3085, a3086)?
                                        };
                                        let a3088 = f630(v5)?;
                                        nat_add(a3087, a3088)?
                                    });
                                    append_bytes(a3089, a3090)
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

pub fn f612(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f351(Fn5::F631, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f613(v0: Fn25, p3095: u64, p3096: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3094 = (p3095, p3096);
    {
        let mut m3094 = m3094;
        loop {
            let step = {
                let (v1, v2) = m3094;
                let m3091 = v1;
                if m3091 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3091 - 1;
                    let m3093 = {
                        let c3092: Fn25 = v0.clone();
                        c3092.apply(v2.clone())
                    };
                    match m3093 {
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
                Ok(next) => { m3094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f632(v0)
}

pub fn f615(v0: Bytes, v1: Bytes) -> bool {
    let a3101 = {
        let a3097 = v0;
        let a3098 = v1;
        compare_bytes(a3097, a3098)
    };
    let a3102 = {
        let a3099 = Bytes::lit(&[]);
        let a3100 = Bytes::lit(&[0x01]);
        compare_bytes(a3099, a3100)
    };
    equal(a3101, a3102)
}

pub fn f616(p3115: u64, v1: Bytes, p3116: u64, p3117: Bytes) -> R<Option<Bytes>> {
    let m3114 = (p3115, (p3116, p3117));
    {
        let mut m3114 = m3114;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3114;
                let m3103 = v0;
                if m3103 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3103 - 1;
                    let a3105 = {
                        let a3104 = v1.clone();
                        length_bytes(a3104)
                    };
                    let a3106 = v2;
                    if nat_le(a3105, a3106) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3107 = f633(v1.clone(), v2)?;
                        match m3107 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3109 = v2;
                                    let a3110 = {
                                        let (_, h3108) = v5;
                                        h3108
                                    };
                                    nat_add(a3109, a3110)?
                                }, {
                                    let a3112 = v3;
                                    let a3113 = f634(v1.clone(), v2, {
                                        let (h3111, _) = v5;
                                        h3111
                                    });
                                    append_bytes(a3112, a3113)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f617(p3127: u64, v1: Bytes, p3128: u64) -> R<bool> {
    let m3126 = (p3127, p3128);
    {
        let mut m3126 = m3126;
        loop {
            let step = {
                let (v0, v2) = m3126;
                let m3118 = v0;
                if m3118 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3118 - 1;
                    let m3121 = {
                        let a3119 = v1.clone();
                        let a3120 = v2;
                        index_bytes(a3119, a3120)
                    };
                    match m3121 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3122 = f62(v4)?;
                            let a3123 = 128u64;
                            if nat_lt(a3122, a3123) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3124 = v2;
                                    let a3125 = 1u64;
                                    nat_add(a3124, a3125)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f395(v0.clone(), 0u64, {
        let a3129 = v0;
        length_bytes(a3129)
    }, false, Bytes::lit(&[]))?;
    let m3132 = {
        let a3130 = v1.clone();
        let a3131 = 0u64;
        index_bytes(a3130, a3131)
    };
    match m3132 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f439(v3) {
                let a3139 = f35({
                    let a3133 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3133)
                }, {
                    let a3134 = v3;
                    let a3135 = 65u64;
                    nat_sub(a3134, a3135)
                }, 1u64);
                let a3140 = f35(v1.clone(), 1u64, {
                    let a3137 = {
                        let a3136 = v1;
                        length_bytes(a3136)
                    };
                    let a3138 = 1u64;
                    nat_sub(a3137, a3138)
                });
                Ok(append_bytes(a3139, a3140))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f619(v0: Bytes) -> R<bool> {
    let m3143 = {
        let a3141 = v0;
        let a3142 = 0u64;
        index_bytes(a3141, a3142)
    };
    match m3143 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f414(f438(f62(v1)?), f439(f62(v1)?)))
        }
    }
}

pub fn f620(p3156: u64, v1: Bytes, p3157: u64, p3158: Bytes) -> R<Option<Bytes>> {
    let m3155 = (p3156, (p3157, p3158));
    {
        let mut m3155 = m3155;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3155;
                let m3144 = v0;
                if m3144 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3144 - 1;
                    let a3146 = {
                        let a3145 = v1.clone();
                        length_bytes(a3145)
                    };
                    let a3147 = v2;
                    if nat_le(a3146, a3147) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3148 = f633(v1.clone(), v2)?;
                        match m3148 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3150 = v2;
                                    let a3151 = {
                                        let (_, h3149) = v5;
                                        h3149
                                    };
                                    nat_add(a3150, a3151)?
                                }, {
                                    let a3153 = v3;
                                    let a3154 = f635({
                                        let (h3152, _) = v5;
                                        h3152
                                    });
                                    append_bytes(a3153, a3154)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f622(p3163: List<(Bytes, Adt18)>) -> Option<Bytes> {
    let m3162 = p3163;
    {
        let mut m3162 = m3162;
        loop {
            let step = {
                let v0 = m3162;
                let m3159 = v0;
                match m3159.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h3160, _) = v1.clone();
                            h3160
                        }, f636()) {
                            Err::<List<(Bytes, Adt18)>, Option<Bytes>>(Some({
                                let (h3161, _) = v1;
                                h3161
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt18)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623(v0: Adt18) -> bool {
    if f541(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f541(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f541(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f541(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f541(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f541(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f624(v0: Adt18) -> Bytes {
    let m3164 = f637(f241(v0, f8(Str::lit("minimum"))));
    match m3164 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f625() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f626(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3165 = v0;
    match m3165 {
        Ok(v2) => {
            f529(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f487(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f486(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f627(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f638(16u64, v0, v1)
}

pub fn f628(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3210 = {
            let a3186 = {
                let a3174 = {
                    let a3168 = {
                        let a3166 = v1;
                        let a3167 = 128u8;
                        fixed_u8::bit_and(a3166, a3167)
                    };
                    let a3169 = 128u8;
                    if equal(a3168, a3169) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3175 = {
                    let a3172 = {
                        let a3170 = v1;
                        let a3171 = 64u8;
                        fixed_u8::bit_and(a3170, a3171)
                    };
                    let a3173 = 64u8;
                    if equal(a3172, a3173) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3174, a3175)?
            };
            let a3187 = {
                let a3184 = {
                    let a3178 = {
                        let a3176 = v1;
                        let a3177 = 32u8;
                        fixed_u8::bit_and(a3176, a3177)
                    };
                    let a3179 = 32u8;
                    if equal(a3178, a3179) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3185 = {
                    let a3182 = {
                        let a3180 = v1;
                        let a3181 = 16u8;
                        fixed_u8::bit_and(a3180, a3181)
                    };
                    let a3183 = 16u8;
                    if equal(a3182, a3183) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3184, a3185)?
            };
            nat_add(a3186, a3187)?
        };
        let a3211 = {
            let a3208 = {
                let a3196 = {
                    let a3190 = {
                        let a3188 = v1;
                        let a3189 = 8u8;
                        fixed_u8::bit_and(a3188, a3189)
                    };
                    let a3191 = 8u8;
                    if equal(a3190, a3191) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3197 = {
                    let a3194 = {
                        let a3192 = v1;
                        let a3193 = 4u8;
                        fixed_u8::bit_and(a3192, a3193)
                    };
                    let a3195 = 4u8;
                    if equal(a3194, a3195) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3196, a3197)?
            };
            let a3209 = {
                let a3206 = {
                    let a3200 = {
                        let a3198 = v1;
                        let a3199 = 2u8;
                        fixed_u8::bit_and(a3198, a3199)
                    };
                    let a3201 = 2u8;
                    if equal(a3200, a3201) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3207 = {
                    let a3204 = {
                        let a3202 = v1;
                        let a3203 = 1u8;
                        fixed_u8::bit_and(a3202, a3203)
                    };
                    let a3205 = 1u8;
                    if equal(a3204, a3205) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3206, a3207)?
            };
            nat_add(a3208, a3209)?
        };
        nat_add(a3210, a3211)?
    };
    let a3212 = 48u64;
    let a3213 = v2;
    if if nat_le(a3212, a3213) {
        let a3214 = v2;
        let a3215 = 57u64;
        nat_le(a3214, a3215)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3216 = 97u64;
        let a3217 = v2;
        if nat_le(a3216, a3217) {
            let a3218 = v2;
            let a3219 = 102u64;
            Ok(nat_le(a3218, a3219))
        } else {
            Ok(false)
        }
    }
}

pub fn f629() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f630(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3264 = {
            let a3240 = {
                let a3228 = {
                    let a3222 = {
                        let a3220 = v1;
                        let a3221 = 128u8;
                        fixed_u8::bit_and(a3220, a3221)
                    };
                    let a3223 = 128u8;
                    if equal(a3222, a3223) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3229 = {
                    let a3226 = {
                        let a3224 = v1;
                        let a3225 = 64u8;
                        fixed_u8::bit_and(a3224, a3225)
                    };
                    let a3227 = 64u8;
                    if equal(a3226, a3227) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3228, a3229)?
            };
            let a3241 = {
                let a3238 = {
                    let a3232 = {
                        let a3230 = v1;
                        let a3231 = 32u8;
                        fixed_u8::bit_and(a3230, a3231)
                    };
                    let a3233 = 32u8;
                    if equal(a3232, a3233) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3239 = {
                    let a3236 = {
                        let a3234 = v1;
                        let a3235 = 16u8;
                        fixed_u8::bit_and(a3234, a3235)
                    };
                    let a3237 = 16u8;
                    if equal(a3236, a3237) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3238, a3239)?
            };
            nat_add(a3240, a3241)?
        };
        let a3265 = {
            let a3262 = {
                let a3250 = {
                    let a3244 = {
                        let a3242 = v1;
                        let a3243 = 8u8;
                        fixed_u8::bit_and(a3242, a3243)
                    };
                    let a3245 = 8u8;
                    if equal(a3244, a3245) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3251 = {
                    let a3248 = {
                        let a3246 = v1;
                        let a3247 = 4u8;
                        fixed_u8::bit_and(a3246, a3247)
                    };
                    let a3249 = 4u8;
                    if equal(a3248, a3249) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3250, a3251)?
            };
            let a3263 = {
                let a3260 = {
                    let a3254 = {
                        let a3252 = v1;
                        let a3253 = 2u8;
                        fixed_u8::bit_and(a3252, a3253)
                    };
                    let a3255 = 2u8;
                    if equal(a3254, a3255) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3261 = {
                    let a3258 = {
                        let a3256 = v1;
                        let a3257 = 1u8;
                        fixed_u8::bit_and(a3256, a3257)
                    };
                    let a3259 = 1u8;
                    if equal(a3258, a3259) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3260, a3261)?
            };
            nat_add(a3262, a3263)?
        };
        nat_add(a3264, a3265)?
    };
    let a3266 = v2;
    let a3267 = 57u64;
    if nat_le(a3266, a3267) {
        let a3268 = v2;
        let a3269 = 48u64;
        Ok(nat_sub(a3268, a3269))
    } else {
        let a3270 = v2;
        let a3271 = 87u64;
        Ok(nat_sub(a3270, a3271))
    }
}

pub fn f631(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f632(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3272, _) = v0.clone();
        h3272
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3273, _) = {
            let (_, h3274) = v0.clone();
            h3274
        };
        h3273
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3275) = {
            let (_, h3276) = v0;
            h3276
        };
        h3275
    };
    let m3277 = v1.clone();
    match m3277.uncons() {
        None => {
            let m3278 = v2;
            match m3278.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3279 = v2.clone();
            match m3279.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f615({
                        let (h3280, _) = v8.clone();
                        h3280
                    }, {
                        let (h3281, _) = v6.clone();
                        h3281
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

pub fn f633(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3284 = {
        let a3282 = v0.clone();
        let a3283 = v1;
        index_bytes(a3282, a3283)
    };
    match m3284 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a3285 = v3;
            let a3286 = 128u64;
            if nat_lt(a3285, a3286) {
                Ok(Some((v3, 1u64)))
            } else {
                if f344({
                    let a3287 = 194u64;
                    let a3288 = v3;
                    nat_le(a3287, a3288)
                }, {
                    let a3289 = v3;
                    let a3290 = 223u64;
                    nat_le(a3289, a3290)
                }) {
                    let m3293 = f639(v0, {
                        let a3291 = v1;
                        let a3292 = 1u64;
                        nat_add(a3291, a3292)?
                    })?;
                    match m3293 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f640({
                                let a3298 = {
                                    let a3296 = {
                                        let a3294 = v3;
                                        let a3295 = 192u64;
                                        nat_sub(a3294, a3295)
                                    };
                                    let a3297 = 64u64;
                                    nat_mul(a3296, a3297)?
                                };
                                let a3299 = v4;
                                nat_add(a3298, a3299)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f344({
                        let a3300 = 224u64;
                        let a3301 = v3;
                        nat_le(a3300, a3301)
                    }, {
                        let a3302 = v3;
                        let a3303 = 239u64;
                        nat_le(a3302, a3303)
                    }) {
                        let m3306 = f639(v0.clone(), {
                            let a3304 = v1;
                            let a3305 = 1u64;
                            nat_add(a3304, a3305)?
                        })?;
                        match m3306 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3309 = f639(v0, {
                                    let a3307 = v1;
                                    let a3308 = 2u64;
                                    nat_add(a3307, a3308)?
                                })?;
                                match m3309 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f640({
                                            let a3318 = {
                                                let a3312 = {
                                                    let a3310 = v3;
                                                    let a3311 = 224u64;
                                                    nat_sub(a3310, a3311)
                                                };
                                                let a3313 = 4096u64;
                                                nat_mul(a3312, a3313)?
                                            };
                                            let a3319 = {
                                                let a3316 = {
                                                    let a3314 = v5;
                                                    let a3315 = 64u64;
                                                    nat_mul(a3314, a3315)?
                                                };
                                                let a3317 = v6;
                                                nat_add(a3316, a3317)?
                                            };
                                            nat_add(a3318, a3319)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f344({
                            let a3320 = 240u64;
                            let a3321 = v3;
                            nat_le(a3320, a3321)
                        }, {
                            let a3322 = v3;
                            let a3323 = 244u64;
                            nat_le(a3322, a3323)
                        }) {
                            let m3326 = f639(v0.clone(), {
                                let a3324 = v1;
                                let a3325 = 1u64;
                                nat_add(a3324, a3325)?
                            })?;
                            match m3326 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3329 = f639(v0.clone(), {
                                        let a3327 = v1;
                                        let a3328 = 2u64;
                                        nat_add(a3327, a3328)?
                                    })?;
                                    match m3329 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3332 = f639(v0, {
                                                let a3330 = v1;
                                                let a3331 = 3u64;
                                                nat_add(a3330, a3331)?
                                            })?;
                                            match m3332 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f640({
                                                        let a3345 = {
                                                            let a3335 = {
                                                                let a3333 = v3;
                                                                let a3334 = 240u64;
                                                                nat_sub(a3333, a3334)
                                                            };
                                                            let a3336 = 262144u64;
                                                            nat_mul(a3335, a3336)?
                                                        };
                                                        let a3346 = {
                                                            let a3343 = {
                                                                let a3337 = v7;
                                                                let a3338 = 4096u64;
                                                                nat_mul(a3337, a3338)?
                                                            };
                                                            let a3344 = {
                                                                let a3341 = {
                                                                    let a3339 = v8;
                                                                    let a3340 = 64u64;
                                                                    nat_mul(a3339, a3340)?
                                                                };
                                                                let a3342 = v9;
                                                                nat_add(a3341, a3342)?
                                                            };
                                                            nat_add(a3343, a3344)?
                                                        };
                                                        nat_add(a3345, a3346)?
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

pub fn f634(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f414(f344(f414(f438(v2), f439(v2)), f344({
        let a3347 = v2;
        let a3348 = 117u64;
        !nat_eq(a3347, a3348)
    }, {
        let a3349 = v2;
        let a3350 = 85u64;
        !nat_eq(a3349, a3350)
    })), f344(f440(v2), {
        let a3351 = 0u64;
        let a3352 = v1;
        nat_lt(a3351, a3352)
    })) {
        f35(v0, v1, 1u64)
    } else {
        let a3353 = v2;
        let a3354 = 65536u64;
        if nat_lt(a3353, a3354) {
            let a3355 = f8(Str::lit("u"));
            let a3356 = f526(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3355, a3356)
        } else {
            let a3357 = f8(Str::lit("U"));
            let a3358 = f526(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3357, a3358)
        }
    }
}

pub fn f635(v0: u64) -> Bytes {
    let a3359 = f8(Str::lit("u"));
    let a3360 = f526(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3359, a3360)
}

pub fn f636() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f637(v0: Option<Adt18>) -> Option<u64> {
    let m3361 = v0;
    match m3361 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m3362 = v1;
            match m3362 {
                Adt18::C0(_) => {
                    None::<u64>
                }
                Adt18::C1(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt18::C2(_, _, _) => {
                    None::<u64>
                }
                Adt18::C3(_) => {
                    None::<u64>
                }
                Adt18::C4(_) => {
                    None::<u64>
                }
                Adt18::C5(_) => {
                    None::<u64>
                }
                Adt18::C6 => {
                    None::<u64>
                }
                Adt18::C7(_, _) => {
                    None::<u64>
                }
                Adt18::C8(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f638(v0: u64, v1: Adt18, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3363 = v0;
    if m3363 == 0 {
        Ok(f528(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3363 - 1;
        let m3364 = v1.clone();
        match m3364 {
            Adt18::C0(v4) => {
                if v4 {
                    Ok(f575())
                } else {
                    Ok(f528(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt18::C1(_, _) => {
                Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C2(_, _, _) => {
                Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C3(_) => {
                Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C4(_) => {
                Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C5(_) => {
                let m3365 = f330(f241(v1.clone(), f8(Str::lit("$ref"))));
                match m3365 {
                    None => {
                        if if f541(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f541(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f541(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f529(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f244(v1.clone(), f8(Str::lit("type")));
                            if if f28(v20.clone(), Bytes::lit(&[])) {
                                !f623(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f575())
                            } else {
                                if f28(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f529(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f28(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f529(f624(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f28(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f529(f625(), List::<Adt25>::nil()))
                                        } else {
                                            if f28(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f529(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f28(v20.clone(), f8(Str::lit("array"))) {
                                                    let m3367 = f241(v1, f8(Str::lit("items")));
                                                    match m3367 {
                                                        None => {
                                                            Ok(f573(f8(Str::lit("List")), f575()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f573(f8(Str::lit("List")), f638(v3, v21, {
                                                                let a3368 = v2;
                                                                let a3369 = f8(Str::lit("Item"));
                                                                append_bytes(a3368, a3369)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f28(v20.clone(), f8(Str::lit("object"))) {
                                                        if f536(v1.clone()) {
                                                            Ok(f529(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3370 = f570(v1);
                                                            match m3370 {
                                                                None => {
                                                                    Ok(f573(f8(Str::lit("List")), f626(f575())))
                                                                }
                                                                Some(v22) => {
                                                                    Ok(f573(f8(Str::lit("List")), f626(f638(v3, v22, {
                                                                        let a3371 = v2;
                                                                        let a3372 = f8(Str::lit("Value"));
                                                                        append_bytes(a3371, a3372)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f528(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m3366 = f525(v18)?;
                        match m3366 {
                            None => {
                                Ok(f528(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f529(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt18::C6 => {
                Ok(f575())
            }
            Adt18::C7(_, _) => {
                Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C8(_, _, _) => {
                Ok(f528(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f639(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3375 = {
        let a3373 = v0;
        let a3374 = v1;
        index_bytes(a3373, a3374)
    };
    match m3375 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f344({
                let a3376 = 128u64;
                let a3377 = v3;
                nat_le(a3376, a3377)
            }, {
                let a3378 = v3;
                let a3379 = 191u64;
                nat_le(a3378, a3379)
            }) {
                Ok(Some({
                    let a3380 = v3;
                    let a3381 = 128u64;
                    nat_sub(a3380, a3381)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f640(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f344({
        let a3382 = v1;
        let a3383 = v0;
        nat_le(a3382, a3383)
    }, f344({
        let a3384 = v0;
        let a3385 = v2;
        nat_le(a3384, a3385)
    }, f414({
        let a3386 = v0;
        let a3387 = 55296u64;
        nat_lt(a3386, a3387)
    }, {
        let a3388 = 57343u64;
        let a3389 = v0;
        nat_lt(a3388, a3389)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
