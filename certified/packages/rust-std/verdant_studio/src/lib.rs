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
    F53(u8, Bytes),
    F174(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F53(k0, k1) => f53(*k0, k1.clone(), p0),
            Fn0::F174(k0) => f174(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F132,
    F149,
    F205,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F132 => f132(p0, p1),
            Fn1::F149 => f149(p0, p1),
            Fn1::F205 => f205(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F170(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F170(k0) => f170(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F194(Bytes, u64, Bytes, u64),
    F270(Bytes),
    F271(u64, Bytes),
    F333(Bytes),
    F377(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F194(k0, k1, k2, k3) => f194(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F270(k0) => Ok(f270(k0.clone(), p0)),
            Fn3::F271(k0, k1) => f271(*k0, k1.clone(), p0),
            Fn3::F333(k0) => f333(k0.clone(), p0),
            Fn3::F377(k0) => f377(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F224(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<(u64, Adt20)> {
        match self {
            Fn4::F224(k0, k1, k2) => f224(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F255,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F255 => f255(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F258,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F258 => f258(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F260,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F260 => f260(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F263,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F263 => f263(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F287(Bytes),
    F341(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F287(k0) => f287(k0.clone(), p0),
            Fn9::F341(k0, k1) => Ok(f341(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F289,
    F409,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F289 => f289(p0, p1),
            Fn10::F409 => f409(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F292,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F292 => f292(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F294,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F294 => f294(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F297,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F297 => f297(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F301,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn14::F301 => f301(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F306(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F306(k0) => f306(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F336(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F336(k0) => f336(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F345,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F345 => f345(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F347,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F347 => f347(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F349,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F349 => f349(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F357,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F357 => f357(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F369,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F369 => f369(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F391,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F391 => f391(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F393,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F393 => f393(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F395(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F395(k0) => f395(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F408,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F408 => f408(p0),
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
    f26(f27(v0, v1)?)
}

pub fn f21(v0: Bytes, v1: u64, v2: u64) -> R<List<Str>> {
    let m128 = v2;
    if m128 == 0 {
        Ok(List::<Str>::nil())
    } else {
        let v3 = m128 - 1;
        let m131 = {
            let a129 = v0.clone();
            let a130 = v1;
            index_bytes(a129, a130)
        };
        match m131 {
            None => {
                Ok(List::<Str>::nil())
            }
            Some(v4) => {
                Ok(List::cons(f28(v4)?, f21(v0, {
                    let a132 = v1;
                    let a133 = 1u64;
                    nat_add(a132, a133)?
                }, v3)?))
            }
        }
    }
}

pub fn f22(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h134, _, _, _, _, _, _, _, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt0::C0(_, h135, _, _, _, _, _, _, _, _) => {
            h135
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, h136, _, _, _, _, _) => {
            h136
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h137, _, _, _, _) => {
            h137
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h138, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h139, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h140, _) => {
            h140
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h141) => {
            h141
        }
    })
}

pub fn f23(v0: List<Bytes>) -> R<List<Str>> {
    let m142 = v0;
    match m142.uncons() {
        None => {
            Ok(List::<Str>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f4(v1)?, f23(v2)?))
        }
    }
}

pub fn f24(v0: List<(Bytes, Bytes)>) -> R<List<(Str, Str)>> {
    let m143 = v0;
    match m143.uncons() {
        None => {
            Ok(List::<(Str, Str)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f4({
                let (h144, _) = v1;
                h144
            })?, f4({
                let (_, h145) = v1;
                h145
            })?), f24(v2)?))
        }
    }
}

pub fn f25(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f29(v0.clone(), 47u8)?;
    let v2: Bytes = f30(v1);
    let a148 = {
        let a146 = v0.clone();
        length_bytes(a146)
    };
    let a149 = {
        let a147 = v2.clone();
        length_bytes(a147)
    };
    if nat_eq(a148, a149) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f31(v0.clone(), 0u64, {
            let a154 = {
                let a150 = v0;
                length_bytes(a150)
            };
            let a155 = {
                let a152 = {
                    let a151 = v2;
                    length_bytes(a151)
                };
                let a153 = 1u64;
                nat_add(a152, a153)?
            };
            nat_sub(a154, a155)
        }))
    }
}

pub fn f26(v0: Adt11) -> R<Adt11> {
    let m157 = {
        let m156 = v0.clone();
        match m156 {
            Adt11::C0(v1) => {
                f32(v1)?
            }
            Adt11::C1(v2) => {
                f32(v2)?
            }
            Adt11::C2(v3, v4, _) => {
                f32(List::cons(v3, v4))?
            }
            Adt11::C3(v6, _) => {
                f32(f33(v6))?
            }
            Adt11::C4(_) => {
                None::<Bytes>
            }
        }
    };
    match m157 {
        None => {
            Ok(v0)
        }
        Some(v9) => {
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ")), List::cons(v9, List::cons(f8(Str::lit(" is not\u{a}")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f27(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m158 = v0;
    match m158.uncons() {
        None => {
            Ok(Adt11::C4(f34()))
        }
        Some((v2, v3)) => {
            if f35(v2.clone(), f8(Str::lit("check"))) {
                f36(v1)
            } else {
                if f35(v2.clone(), f8(Str::lit("next"))) {
                    f37(v1, v3)
                } else {
                    if f35(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f38(v1))
                    } else {
                        if f35(v2.clone(), f8(Str::lit("publish"))) {
                            f39(v1)
                        } else {
                            if f35(v2.clone(), f8(Str::lit("agenda"))) {
                                f40(v1)
                            } else {
                                let m159 = v3;
                                match m159.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f34()))
                                    }
                                    Some((v4, v5)) => {
                                        if f35(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f41(v1, v4)
                                        } else {
                                            if f35(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                f42(v1, v4)
                                            } else {
                                                if f35(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                    f43(v1, v4, false)
                                                } else {
                                                    if f35(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                        f43(v1, v4, true)
                                                    } else {
                                                        if f35(v2.clone(), f8(Str::lit("admit"))) {
                                                            f44(v1, v4)
                                                        } else {
                                                            if f35(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                f45(v1, v4)
                                                            } else {
                                                                if f35(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                    f46(v1, v4)
                                                                } else {
                                                                    if f35(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                        f47(v1, v4)
                                                                    } else {
                                                                        if f35(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                            f48(v1, v4)
                                                                        } else {
                                                                            if f35(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                f49(v1, v4)
                                                                            } else {
                                                                                if f35(v2, f8(Str::lit("propose-encoding"))) {
                                                                                    let m160 = v5;
                                                                                    match m160.uncons() {
                                                                                        None => {
                                                                                            Ok(Adt11::C4(f34()))
                                                                                        }
                                                                                        Some((v6, v7)) => {
                                                                                            f50(v1, v4, v6, v7)
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(Adt11::C4(f34()))
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

pub fn f28(v0: u8) -> R<Str> {
    let m209 = {
        let a207 = f51();
        let a208 = {
            let v1: u8 = v0;
            let a205 = {
                let a181 = {
                    let a169 = {
                        let a163 = {
                            let a161 = v1;
                            let a162 = 128u8;
                            fixed_u8::bit_and(a161, a162)
                        };
                        let a164 = 128u8;
                        if equal(a163, a164) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a170 = {
                        let a167 = {
                            let a165 = v1;
                            let a166 = 64u8;
                            fixed_u8::bit_and(a165, a166)
                        };
                        let a168 = 64u8;
                        if equal(a167, a168) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a169, a170)?
                };
                let a182 = {
                    let a179 = {
                        let a173 = {
                            let a171 = v1;
                            let a172 = 32u8;
                            fixed_u8::bit_and(a171, a172)
                        };
                        let a174 = 32u8;
                        if equal(a173, a174) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a180 = {
                        let a177 = {
                            let a175 = v1;
                            let a176 = 16u8;
                            fixed_u8::bit_and(a175, a176)
                        };
                        let a178 = 16u8;
                        if equal(a177, a178) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a179, a180)?
                };
                nat_add(a181, a182)?
            };
            let a206 = {
                let a203 = {
                    let a191 = {
                        let a185 = {
                            let a183 = v1;
                            let a184 = 8u8;
                            fixed_u8::bit_and(a183, a184)
                        };
                        let a186 = 8u8;
                        if equal(a185, a186) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a192 = {
                        let a189 = {
                            let a187 = v1;
                            let a188 = 4u8;
                            fixed_u8::bit_and(a187, a188)
                        };
                        let a190 = 4u8;
                        if equal(a189, a190) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a191, a192)?
                };
                let a204 = {
                    let a201 = {
                        let a195 = {
                            let a193 = v1;
                            let a194 = 2u8;
                            fixed_u8::bit_and(a193, a194)
                        };
                        let a196 = 2u8;
                        if equal(a195, a196) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a202 = {
                        let a199 = {
                            let a197 = v1;
                            let a198 = 1u8;
                            fixed_u8::bit_and(a197, a198)
                        };
                        let a200 = 1u8;
                        if equal(a199, a200) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a201, a202)?
                };
                nat_add(a203, a204)?
            };
            nat_add(a205, a206)?
        };
        index_list(a207, a208)
    };
    match m209 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f29(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a210 = v0.clone();
        length_bytes(a210)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h211, _) = f52(Fn0::F53(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h211
    };
    let v4: u64 = {
        let (h212, _) = {
            let (_, h213) = v3.clone();
            h213
        };
        h212
    };
    Ok(f54(List::cons(f31(v0, v4, {
        let a214 = v2;
        let a215 = v4;
        nat_sub(a214, a215)
    }), {
        let (_, h216) = {
            let (_, h217) = v3;
            h217
        };
        h216
    })))
}

pub fn f30(p221: List<Bytes>) -> Bytes {
    let m220 = p221;
    {
        let mut m220 = m220;
        loop {
            let step = {
                let v0 = m220;
                let m218 = v0;
                match m218.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m219 = v2.clone();
                        match m219.uncons() {
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
                Ok(next) => { m220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m225 = {
        let a222 = v0;
        let a223 = v1;
        let a224 = v2;
        slice_bytes(a222, a223, a224)
    };
    match m225 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f32(p229: List<Bytes>) -> R<Option<Bytes>> {
    let m228 = p229;
    {
        let mut m228 = m228;
        loop {
            let step = {
                let v0 = m228;
                let m226 = v0;
                match m226.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f55(v1.clone(), 0u64, {
                            let a227 = v1.clone();
                            length_bytes(a227)
                        })? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m230 = v0;
    match m230.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h231, _) = v1;
                h231
            }, f33(v2))
        }
    }
}

pub fn f34() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f35(v0: Bytes, v1: Bytes) -> bool {
    let a232 = v0;
    let a233 = v1;
    equal(a232, a233)
}

pub fn f36(v0: Adt1) -> R<Adt11> {
    let m234 = f56(v0.clone(), f57());
    match m234.uncons() {
        None => {
            let m235 = f58(v0.clone(), f57())?;
            match m235.uncons() {
                None => {
                    let m238 = f59(match v0.clone() {
                        Adt1::C0(_, h236, _) => {
                            h236
                        }
                    }, f60(f61(), f62(match v0.clone() {
                        Adt1::C0(_, h237, _) => {
                            h237
                        }
                    }, false)));
                    match m238.uncons() {
                        None => {
                            let m240 = f56(v0.clone(), f62(match v0.clone() {
                                Adt1::C0(_, h239, _) => {
                                    h239
                                }
                            }, true));
                            match m240.uncons() {
                                None => {
                                    let m242 = f63(v0.clone(), f62(match v0.clone() {
                                        Adt1::C0(_, h241, _) => {
                                            h241
                                        }
                                    }, true))?;
                                    match m242 {
                                        None => {
                                            let v10: (bool, Bytes) = f64(Adt13::C0(Adt12::C0(f65(f66(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f67(v0.clone())?, f68(f69(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f65(f66(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f68(f69(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f69(v0.clone(), f8(Str::lit("model/admissions.json")))?, f69(v0.clone(), f8(Str::lit("model/strata.json")))?, f70(f69(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f62(match v0 {
                                                Adt1::C0(_, h243, _) => {
                                                    h243
                                                }
                                            }, true)))?;
                                            let (h244, _) = v10;
                                            if h244 {
                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                    let (_, h245) = v10;
                                                    h245
                                                }))
                                            } else {
                                                Ok(Adt11::C4({
                                                    let (_, h246) = v10;
                                                    h246
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

pub fn f37(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f71(f72(match v0.clone() {
        Adt1::C0(_, h247, _) => {
            h247
        }
    }, f73())) {
        Ok(Adt11::C1(List::cons(f73(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f74(f75(f73(), f76(match v0.clone() {
            Adt1::C0(_, h248, _) => {
                h248
            }
        }, f73())))?;
        let m249 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m249.uncons() {
            None => {
                let m250 = f77(v0.clone(), f78(v2, v1.clone()))?;
                match m250 {
                    None => {
                        f79(v0, v1)
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

pub fn f38(v0: Adt1) -> Adt11 {
    if f71(f80(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h251) => {
            h251
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f81(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f71(f80(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h252) => {
                h252
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f81(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f39(v0: Adt1) -> R<Adt11> {
    let m253 = f56(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f82(), List::<Bytes>::nil()))));
    match m253.uncons() {
        None => {
            Ok(Adt11::C3(f83(f84(v0.clone())?, List::cons((f85(), f86()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f87()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f88()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f89()), List::cons((f8(Str::lit("docs/model.md")), f90(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f91()), List::<(Bytes, Bytes)>::nil()))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md and docs/owner-decisions.md\u{a}"))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m254 = f56(v0.clone(), List::cons(f92(), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m254.uncons() {
        None => {
            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f93(f94(f95(f96(f66(v0.clone(), f92()))?)), f97(f69(v0, f8(Str::lit("model/admissions.json")))?)))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f41(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m255 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m255.uncons() {
        None => {
            let v4: Bytes = f66(v0.clone(), v1);
            let v5: List<List<Bytes>> = f96(v4.clone())?;
            if f71(f98(f99(v5.clone()), f100())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f101(Str::lit("id"), v6.clone())?, List::cons(f101(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f101(Str::lit("artifact"), f92())?, List::cons(f101(Str::lit("checksum"), f102(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f92(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f103(f66(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f104(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f105(f94(f95(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f42(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m256 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m256.uncons() {
        None => {
            let m257 = f106(f107(f66(v0, v1))?)?;
            match m257 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f105(f33(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f43(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m258 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m258.uncons() {
        None => {
            let v5: List<Bytes> = f107(f66(v0.clone(), v1.clone()))?;
            if f71(f108(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f109(v5)?
                } else {
                    f110(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f111(f112(), f113(v1.clone())?), f66(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f111(f112(), f113(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f114()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f44(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m259 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil())))));
    match m259.uncons() {
        None => {
            let v4: List<Bytes> = f107(f66(v0.clone(), v1))?;
            let m260 = f115(v4.clone(), f8(Str::lit("requirement")))?;
            match m260 {
                None => {
                    Ok(Adt11::C4(f114()))
                }
                Some(v5) => {
                    let m261 = f115(v4, f8(Str::lit("admission")))?;
                    match m261 {
                        None => {
                            Ok(Adt11::C4(f114()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f69(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f69(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f60(f68(v5.clone(), f8(Str::lit("id"))), f68(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f74(f60(f68(v7.clone(), f8(Str::lit("id"))), f68(v8.clone(), f8(Str::lit("construct")))))?;
                            let a263 = {
                                let a262 = v9.clone();
                                length_list(a262)
                            };
                            let a264 = 0u64;
                            if nat_eq(a263, a264) {
                                Ok(Adt11::C4(f114()))
                            } else {
                                if f71({
                                    let a267 = {
                                        let a265 = f74(f60(v9.clone(), v10.clone()))?;
                                        length_list(a265)
                                    };
                                    let a268 = {
                                        let a266 = f60(v9.clone(), v10);
                                        length_list(a266)
                                    };
                                    nat_eq(a267, a268)
                                }) {
                                    Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                } else {
                                    Ok(Adt11::C3(List::cons(f116(f8(Str::lit("model/requirements.json")), f117(v7, v5))?, List::cons(f116(f8(Str::lit("model/admissions.json")), f117(v8, v6))?, List::cons(f104(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f105(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f45(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m269 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m269.uncons() {
        None => {
            let m270 = f118(f107(f66(v0, v1))?)?;
            match m270 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f105(f33(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f46(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m271 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m271.uncons() {
        None => {
            let m272 = f119(v0.clone(), v1.clone())?;
            match m272 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m273 = f120(v4.clone(), f8(Str::lit("schema")));
                    match m273 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f121(v0, f122(v5.clone())?, v5, f123(v4.clone(), f8(Str::lit("name"))), f70(f124(f120(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f47(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m274 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f82(), List::<Bytes>::nil())));
    match m274.uncons() {
        None => {
            let m275 = f125(f107(f66(v0.clone(), v1))?)?;
            match m275 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f69(v0, f82())?;
                    let v6: List<Bytes> = f68(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f74(f68(v5.clone(), f8(Str::lit("at"))))?;
                    let a277 = {
                        let a276 = v6.clone();
                        length_list(a276)
                    };
                    let a278 = 0u64;
                    if if nat_eq(a277, a278) {
                        true
                    } else {
                        f71({
                            let a281 = {
                                let a279 = f74(f60(v6.clone(), v7.clone()))?;
                                length_list(a279)
                            };
                            let a282 = {
                                let a280 = f60(v6.clone(), v7);
                                length_list(a280)
                            };
                            nat_eq(a281, a282)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f116(f82(), f117(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f105(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m283 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m283.uncons() {
        None => {
            let m284 = f119(v0, v1)?;
            match m284 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f126(f124(Some(v4))) {
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

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m285 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m285.uncons() {
        None => {
            let m286 = f119(v0.clone(), v1.clone())?;
            match m286 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f71(f35(f123(v4.clone(), f8(Str::lit("schema"))), f127())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f123(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m287 = f120(v4.clone(), f8(Str::lit("source")));
                            match m287 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f128(f120(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f111(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f111(v9.clone(), f8(Str::lit("binding.json"))), f66(v0.clone(), v1));
                        let m288 = v8;
                        match m288 {
                            None => {
                                f129(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f71(f35(f123(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f129(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f111(v9, f113(f123(v7.clone(), f8(Str::lit("url"))))?);
                                    if f71(f80(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h289) => {
                                            h289
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f123(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m290 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m290.uncons() {
                                            None => {
                                                if f35(f102(f66(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f129(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f50(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m291 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m291.uncons() {
        None => {
            let m292 = f119(v0.clone(), v1.clone())?;
            match m292 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f121(v0.clone(), f66(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f51() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f52(v0: Fn0, p297: u64, p298: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v1, v2) = m296;
                let m293 = v1;
                if m293 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m293 - 1;
                    let m295 = {
                        let c294: Fn0 = v0.clone();
                        c294.apply(v2.clone())?
                    };
                    match m295 {
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
                Ok(next) => { m296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f53(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f130(v1, v0, v2)
}

pub fn f54(v0: List<Bytes>) -> List<Bytes> {
    f131(Fn1::F132, List::<Bytes>::nil(), v0)
}

pub fn f55(v0: Bytes, p354: u64, p355: u64) -> R<bool> {
    let m353 = (p354, p355);
    {
        let mut m353 = m353;
        loop {
            let step = {
                let (v1, v2) = m353;
                let m299 = v2;
                if m299 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m299 - 1;
                    let m302 = {
                        let a300 = v0.clone();
                        let a301 = v1;
                        index_bytes(a300, a301)
                    };
                    match m302 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a349 = {
                                let v5: u8 = v4;
                                let a347 = {
                                    let a323 = {
                                        let a311 = {
                                            let a305 = {
                                                let a303 = v5;
                                                let a304 = 128u8;
                                                fixed_u8::bit_and(a303, a304)
                                            };
                                            let a306 = 128u8;
                                            if equal(a305, a306) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a312 = {
                                            let a309 = {
                                                let a307 = v5;
                                                let a308 = 64u8;
                                                fixed_u8::bit_and(a307, a308)
                                            };
                                            let a310 = 64u8;
                                            if equal(a309, a310) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a311, a312)?
                                    };
                                    let a324 = {
                                        let a321 = {
                                            let a315 = {
                                                let a313 = v5;
                                                let a314 = 32u8;
                                                fixed_u8::bit_and(a313, a314)
                                            };
                                            let a316 = 32u8;
                                            if equal(a315, a316) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a322 = {
                                            let a319 = {
                                                let a317 = v5;
                                                let a318 = 16u8;
                                                fixed_u8::bit_and(a317, a318)
                                            };
                                            let a320 = 16u8;
                                            if equal(a319, a320) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a321, a322)?
                                    };
                                    nat_add(a323, a324)?
                                };
                                let a348 = {
                                    let a345 = {
                                        let a333 = {
                                            let a327 = {
                                                let a325 = v5;
                                                let a326 = 8u8;
                                                fixed_u8::bit_and(a325, a326)
                                            };
                                            let a328 = 8u8;
                                            if equal(a327, a328) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a334 = {
                                            let a331 = {
                                                let a329 = v5;
                                                let a330 = 4u8;
                                                fixed_u8::bit_and(a329, a330)
                                            };
                                            let a332 = 4u8;
                                            if equal(a331, a332) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a333, a334)?
                                    };
                                    let a346 = {
                                        let a343 = {
                                            let a337 = {
                                                let a335 = v5;
                                                let a336 = 2u8;
                                                fixed_u8::bit_and(a335, a336)
                                            };
                                            let a338 = 2u8;
                                            if equal(a337, a338) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a344 = {
                                            let a341 = {
                                                let a339 = v5;
                                                let a340 = 1u8;
                                                fixed_u8::bit_and(a339, a340)
                                            };
                                            let a342 = 1u8;
                                            if equal(a341, a342) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a343, a344)?
                                    };
                                    nat_add(a345, a346)?
                                };
                                nat_add(a347, a348)?
                            };
                            let a350 = 127u64;
                            if nat_le(a349, a350) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a351 = v1;
                                    let a352 = 1u64;
                                    nat_add(a351, a352)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m353 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m356 = v1;
    match m356.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m358 = f133(match v0.clone() {
                Adt1::C0(h357, _, _) => {
                    h357
                }
            }, v2.clone());
            match m358 {
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

pub fn f57() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f92(), List::<Bytes>::nil())))))))
}

pub fn f58(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m359 = v1;
    match m359.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f134(v0.clone(), v2.clone()) {
                f108(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m360 = f119(v0.clone(), v2.clone())?;
                match m360 {
                    None => {
                        Ok(List::cons(v2, f58(v0, v3)?))
                    }
                    Some(_) => {
                        f58(v0, v3)
                    }
                }
            } else {
                f58(v0, v3)
            }
        }
    }
}

pub fn f59(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m361 = v1;
    match m361.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f72(v0.clone(), v2.clone()) {
                f59(v0, v3)
            } else {
                List::cons(v2, f59(v0, v3))
            }
        }
    }
}

pub fn f60(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m362 = v0;
    match m362.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f60(v3, v1))
        }
    }
}

pub fn f61() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f62(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m363 = v0;
    match m363.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f60(f135({
                let (h364, _) = v2;
                h364
            }, {
                let (_, h365) = v2;
                h365
            }, v1), f62(v3, v1))
        }
    }
}

pub fn f63(v0: Adt1, p369: List<Bytes>) -> R<Option<Bytes>> {
    let m368 = p369;
    {
        let mut m368 = m368;
        loop {
            let step = {
                let v1 = m368;
                let m366 = v1;
                match m366.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m367 = f136(f66(v0.clone(), v2.clone()), f137())?;
                        match m367 {
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
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h370, _, _, _, _) => {
            h370
        }
    };
    let v2: Adt15 = f138(v1.clone())?;
    let v3: List<Bytes> = f139(v1.clone());
    let v4: List<Bytes> = f68(match v0.clone() {
        Adt13::C0(_, _, h371, _, _) => {
            h371
        }
    }, f8(Str::lit("path")));
    let m373 = f140(v2, List::<u64>::nil(), f141(v1.clone(), match v0.clone() {
        Adt13::C0(_, h372, _, _, _) => {
            h372
        }
    }, 0u64)?);
    match m373 {
        None => {
            if f143(f144(match v0.clone() {
                Adt13::C0(_, _, _, _, h375) => {
                    h375
                }
            }, f60(match v0.clone() {
                Adt13::C0(_, _, _, _, h376) => {
                    h376
                }
            }, v4.clone()))?, f144(v4.clone(), f60(match v0.clone() {
                Adt13::C0(_, _, _, _, h377) => {
                    h377
                }
            }, v4))?, f145(match v0.clone() {
                Adt13::C0(_, _, h378, _, _) => {
                    h378
                }
            })) {
                let m382 = f146(f144(match v0.clone() {
                    Adt13::C0(_, _, _, h379, _) => {
                        h379
                    }
                }, f60(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h380, _) => {
                        h380
                    }
                }))?, f144(v3.clone(), f60(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h381, _) => {
                        h381
                    }
                }))?);
                match m382 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f105(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f142(f60(v3, match v0 {
                            Adt13::C0(_, _, _, h383, _) => {
                                h383
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f142(match v1 {
                Adt12::C0(_, _, _, _, h374) => {
                    h374
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f65(v0: Bytes) -> R<List<Bytes>> {
    f147(f107(v0)?, f8(Str::lit("id = \"")))
}

pub fn f66(v0: Adt1, v1: Bytes) -> Bytes {
    let m385 = f133(match v0 {
        Adt1::C0(h384, _, _) => {
            h384
        }
    }, v1);
    match m385 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m386 = v2;
            match m386 {
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

pub fn f67(v0: Adt1) -> R<List<Bytes>> {
    let m387 = f96(f66(v0, f92()))?;
    match m387.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f94(v2))
        }
    }
}

pub fn f68(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m388 = v0;
    match m388.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f123(v2, v1.clone()), f68(v3, v1))
        }
    }
}

pub fn f69(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f124(f119(v0, v1)?))
}

pub fn f70(v0: List<Adt14>) -> List<Bytes> {
    let m389 = v0;
    match m389.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m390 = f128(Some(v1));
            match m390 {
                None => {
                    f70(v2)
                }
                Some(v3) => {
                    List::cons(v3, f70(v2))
                }
            }
        }
    }
}

pub fn f71(v0: bool) -> bool {
    !v0
}

pub fn f72(p394: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m393 = p394;
    {
        let mut m393 = m393;
        loop {
            let step = {
                let v0 = m393;
                let m391 = v0;
                match m391.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h392, _) = v2;
                            h392
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f74(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f148(f131(Fn1::F149, List::<Bytes>::nil(), f150(v0)?)))
}

pub fn f75(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m395 = v1;
    match m395.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f108(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f111(v0.clone(), v2), f75(v0, v3))
            } else {
                f75(v0, v3)
            }
        }
    }
}

pub fn f76(p400: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m399 = p400;
    {
        let mut m399 = m399;
        loop {
            let step = {
                let v0 = m399;
                let m396 = v0;
                match m396.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h397, _) = v2;
                            h397
                        }, v1.clone()) {
                            let (_, h398) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h398)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Adt1, p403: List<Bytes>) -> R<Option<Bytes>> {
    let m402 = p403;
    {
        let mut m402 = m402;
        loop {
            let step = {
                let v1 = m402;
                let m401 = v1;
                match m401.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        if f151(v0.clone(), v2.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v2)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m404 = v0;
    match m404.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f80(v2.clone(), v1.clone()) {
                f78(v3, v1)
            } else {
                List::cons(v2, f78(v3, v1))
            }
        }
    }
}

pub fn f79(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f71(f72(match v0.clone() {
        Adt1::C0(_, h405, _) => {
            h405
        }
    }, f152())) {
        Ok(Adt11::C1(List::cons(f152(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f78(f74(f75(f152(), f76(match v0.clone() {
            Adt1::C0(_, h406, _) => {
                h406
            }
        }, f152())))?, v1.clone());
        let m407 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m407.uncons() {
            None => {
                let m408 = f153(v0.clone(), v2, f68(f69(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m408 {
                    None => {
                        f154(v0, v1)
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

pub fn f80(v0: Bytes, p411: List<Bytes>) -> bool {
    let m410 = p411;
    {
        let mut m410 = m410;
        loop {
            let step = {
                let v1 = m410;
                let m409 = v1;
                match m409.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f35(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f82() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f83(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m412 = v0;
    match m412.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f83(v3, v1))
        }
    }
}

pub fn f84(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m413 = f69(v0.clone(), f82())?;
    match m413.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f155(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f85() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f86() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f87() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/encodings/**', 'model/queue/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          { echo \"proposed a change to the API's sources, exported again as $(basename studio/.lexlean/production/*/)\"; git diff --stat -- studio/src README.md; } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio propose-command\") | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          gh pr create --head \"$branch\" --base main --title \"verdant_studio $ARGS\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f88() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src and\u{a}# README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one.\u{a}name: refresh\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  refresh:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select(.title == \\\"verdant_studio propose-command\\\" and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            state=unknown\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              state=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\u{a}              if [ \"$state\" != unknown ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$state\" = dirty ]; then stale=\"$stale $pr\"; fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply each change to the new main, export it again, and push it to its own pull request\u{a}        if: steps.stale.outputs.stale != ''\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ steps.stale.outputs.stale }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /'\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}"))
}

pub fn f89() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f90(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f156(f69(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f157(f69(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f91() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f92() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f93(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m414 = v0;
    match m414.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f80(v2.clone(), v1.clone()) {
                f93(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f93(v3, v1))
            }
        }
    }
}

pub fn f94(v0: List<List<Bytes>>) -> List<Bytes> {
    let m415 = v0;
    match m415.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m416 = v1;
            match m416.uncons() {
                None => {
                    f94(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f94(v2))
                }
            }
        }
    }
}

pub fn f95(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m417 = v0;
    match m417.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f96(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f158(v0.clone(), 0u64, {
        let a418 = v0;
        length_bytes(a418)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a421 = {
            let a420 = match v1.clone() {
                Adt16::C0(_, _, h419, _, _) => {
                    h419
                }
            };
            length_bytes(a420)
        };
        let a422 = 0u64;
        if if nat_eq(a421, a422) {
            let m424 = match v1.clone() {
                Adt16::C0(_, _, _, h423, _) => {
                    h423
                }
            };
            match m424.uncons() {
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
            f159(v1)
        }
    };
    Ok(f160(match v4 {
        Adt16::C0(_, _, _, _, h425) => {
            h425
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f97(v0: List<Adt14>) -> List<Bytes> {
    let m426 = v0;
    match m426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m427 = f120(v1, f8(Str::lit("std")));
            match m427 {
                None => {
                    f97(v2)
                }
                Some(v3) => {
                    List::cons(f123(v3, f8(Str::lit("row"))), f97(v2))
                }
            }
        }
    }
}

pub fn f98(p432: List<Bytes>, p433: List<Bytes>) -> bool {
    let m431 = (p432, p433);
    {
        let mut m431 = m431;
        loop {
            let step = {
                let (v0, v1) = m431;
                let m428 = v0;
                match m428.uncons() {
                    None => {
                        let m429 = v1;
                        match m429.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m430 = v1;
                        match m430.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f35(v4, v6) {
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
                Ok(next) => { m431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: List<List<Bytes>>) -> List<Bytes> {
    let m434 = v0;
    match m434.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f100() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f101(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f161(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f102(v0: Bytes) -> R<Bytes> {
    Ok(f162(f163(v0)?))
}

pub fn f103(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f164(f165(f107(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f166(f167(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f104(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f122(Adt14::C3(std::rc::Rc::new(f168(f74(f60(v1, f70(f69(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f105(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m435 = f169(Fn2::F170(v1), None::<Bytes>, v0);
    match m435 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f106(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m436 = v0;
    match m436.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f171(v1)?;
            let a438 = {
                let a437 = v3.clone();
                length_bytes(a437)
            };
            let a439 = 0u64;
            if nat_eq(a438, a439) {
                f106(v2)
            } else {
                let m440 = f172(v3.clone())?;
                match m440 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f123(v4.clone(), f8(Str::lit("id")));
                        if if f35(f123(v4, f8(Str::lit("schema"))), f127()) {
                            f71({
                                let a442 = {
                                    let a441 = v5.clone();
                                    length_bytes(a441)
                                };
                                let a443 = 0u64;
                                nat_eq(a442, a443)
                            })
                        } else {
                            false
                        } {
                            let m444 = f106(v2)?;
                            match m444 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f173(v5), {
                                        let a445 = v3;
                                        let a446 = Bytes::lit(&[0x0a]);
                                        append_bytes(a445, a446)
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

pub fn f107(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a447 = v0.clone();
        length_bytes(a447)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h448, _) = f52(Fn0::F174(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h448
    };
    let v3: u64 = {
        let (h449, _) = {
            let (_, h450) = v2.clone();
            h450
        };
        h449
    };
    let v4: List<Bytes> = {
        let (_, h451) = {
            let (_, h452) = v2;
            h452
        };
        h451
    };
    let a453 = v3;
    let a454 = v1;
    if nat_lt(a453, a454) {
        Ok(f54(List::cons(f31(v0, v3, {
            let a455 = v1;
            let a456 = v3;
            nat_sub(a455, a456)
        }), v4)))
    } else {
        Ok(f54(v4))
    }
}

pub fn f108(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a457 = v0.clone();
        length_bytes(a457)
    };
    let v3: u64 = {
        let a458 = v1.clone();
        length_bytes(a458)
    };
    let a459 = v3;
    let a460 = v2;
    if nat_le(a459, a460) {
        let m466 = {
            let a463 = v0;
            let a464 = {
                let a461 = v2;
                let a462 = v3;
                nat_sub(a461, a462)
            };
            let a465 = v3;
            slice_bytes(a463, a464, a465)
        };
        match m466 {
            None => {
                false
            }
            Some(v4) => {
                let a467 = v4;
                let a468 = v1;
                equal(a467, a468)
            }
        }
    } else {
        false
    }
}

pub fn f109(v0: List<Bytes>) -> R<bool> {
    if f175(f125(v0.clone())?) {
        Ok(f71({
            let a470 = {
                let a469 = f176(f125(v0)?);
                length_list(a469)
            };
            let a471 = 0u64;
            nat_eq(a470, a471)
        }))
    } else {
        Ok(false)
    }
}

pub fn f110(v0: List<Bytes>) -> R<bool> {
    if f175(f115(v0.clone(), f8(Str::lit("requirement")))?) {
        if f175(f115(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f71({
                let a473 = {
                    let a472 = f117(f176(f115(v0.clone(), f8(Str::lit("requirement")))?), f176(f115(v0, f8(Str::lit("admission")))?));
                    length_list(a472)
                };
                let a474 = 0u64;
                nat_eq(a473, a474)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f111(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f112() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f113(v0: Bytes) -> R<Bytes> {
    Ok(f30(f29(v0, 47u8)?))
}

pub fn f114() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f115(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m475 = v0;
    match m475.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f171(v2)?;
            let a477 = {
                let a476 = v4.clone();
                length_bytes(a476)
            };
            let a478 = 0u64;
            if nat_eq(a477, a478) {
                f115(v3, v1)
            } else {
                let m479 = f172(v4)?;
                match m479 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f177(v5.clone());
                        let a481 = {
                            let a480 = v6.clone();
                            length_bytes(a480)
                        };
                        let a482 = 0u64;
                        if nat_eq(a481, a482) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m483 = f115(v3, v1.clone())?;
                            match m483 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f35(v6, v1) {
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

pub fn f116(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f122(Adt14::C3(std::rc::Rc::new(f178(v1))))?))
}

pub fn f117(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m484 = v0;
    match m484.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f117(v3, v1))
        }
    }
}

pub fn f118(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m485 = v0;
    match m485.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f171(v1)?;
            let a487 = {
                let a486 = v3.clone();
                length_bytes(a486)
            };
            let a488 = 0u64;
            if nat_eq(a487, a488) {
                f118(v2)
            } else {
                let m489 = f172(v3.clone())?;
                match m489 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f179(v4.clone()) {
                            let m490 = f118(v2)?;
                            match m490 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f152(), List::cons(f8(Str::lit("/")), List::cons(f180(f123(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a491 = v3;
                                        let a492 = Bytes::lit(&[0x0a]);
                                        append_bytes(a491, a492)
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

pub fn f119(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f172(f66(v0, v1))
}

pub fn f120(p498: Adt14, v1: Bytes) -> Option<Adt14> {
    let m497 = p498;
    {
        let mut m497 = m497;
        loop {
            let step = {
                let v0 = m497;
                let m493 = v0;
                match m493 {
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
                    Adt14::C4(r494) => {
                        let v5 = (*r494).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r495, r496) => {
                        let v3 = (*r495).clone();
                        let v4 = (*r496).clone();
                        if f35(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
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

pub fn f121(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m499 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m499.uncons() {
        None => {
            let m500 = f181(v4);
            match m500 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a501 = f8(Str::lit("Catalog."));
                        let a502 = f180(v3.clone())?;
                        append_bytes(a501, a502)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f180(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m503 = f182(v2, v9.clone(), f180(v3.clone())?)?;
                    match m503 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f180(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f122(Adt14::C3(std::rc::Rc::new(f178(f183(f69(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f122(Adt14::C3(std::rc::Rc::new(f178(f183(f69(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a507 = f184(f66(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f31(v10.clone(), 4u64, {
                                    let a505 = {
                                        let a504 = v10;
                                        length_bytes(a504)
                                    };
                                    let a506 = 4u64;
                                    nat_sub(a505, a506)
                                }))?;
                                let a508 = Bytes::lit(&[0x0a]);
                                append_bytes(a507, a508)
                            }), List::cons(f104(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f122(v0: Adt14) -> R<Bytes> {
    let a509 = f185(v0)?;
    let a510 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a509, a510))
}

pub fn f123(v0: Adt14, v1: Bytes) -> Bytes {
    let m511 = f128(f120(v0, v1));
    match m511 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124(v0: Option<Adt14>) -> List<Adt14> {
    let m512 = v0;
    match m512 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f186(v1)
        }
    }
}

pub fn f125(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m513 = v0;
    match m513.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f171(v1)?;
            let a515 = {
                let a514 = v3.clone();
                length_bytes(a514)
            };
            let a516 = 0u64;
            if nat_eq(a515, a516) {
                f125(v2)
            } else {
                let m517 = f172(v3)?;
                match m517 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f187(v4.clone(), f8(Str::lit("agent"))) {
                            if f187(v4.clone(), f8(Str::lit("at"))) {
                                if f187(v4.clone(), f8(Str::lit("quote"))) {
                                    f188(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m518 = f125(v2)?;
                            match m518 {
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

pub fn f126(p521: List<Adt14>) -> bool {
    let m520 = p521;
    {
        let mut m520 = m520;
        loop {
            let step = {
                let v0 = m520;
                let m519 = v0;
                match m519.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f35(f123(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f80(f8(Str::lit("studio")), f68(f189(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f128(v0: Option<Adt14>) -> Option<Bytes> {
    let m522 = v0;
    match m522 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m523 = v1;
            match m523 {
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

pub fn f129(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f103(f66(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f190(v1, v4)?)?), List::cons(f104(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f130(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h524, _) = v2.clone();
        h524
    };
    let v4: u64 = {
        let (h525, _) = {
            let (_, h526) = v2.clone();
            h526
        };
        h525
    };
    let v5: List<Bytes> = {
        let (_, h527) = {
            let (_, h528) = v2;
            h528
        };
        h527
    };
    let m529 = f191(v0.clone(), v3);
    match m529 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a530 = v6;
            let a531 = v1;
            if equal(a530, a531) {
                Ok(Some(({
                    let a532 = v3;
                    let a533 = 1u64;
                    nat_add(a532, a533)?
                }, ({
                    let a534 = v3;
                    let a535 = 1u64;
                    nat_add(a534, a535)?
                }, List::cons(f31(v0, v4, {
                    let a536 = v3;
                    let a537 = v4;
                    nat_sub(a536, a537)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a538 = v3;
                    let a539 = 1u64;
                    nat_add(a538, a539)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f131(v0: Fn1, p543: List<Bytes>, p544: List<Bytes>) -> List<Bytes> {
    let m542 = (p543, p544);
    {
        let mut m542 = m542;
        loop {
            let step = {
                let (v1, v2) = m542;
                let m540 = v2;
                match m540.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c541: Fn1 = v0.clone();
                            c541.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f132(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f133(p549: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m548 = p549;
    {
        let mut m548 = m548;
        loop {
            let step = {
                let v0 = m548;
                let m545 = v0;
                match m545.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h546, _) = v2;
                            h546
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h547) = v2;
                                h547
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
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

pub fn f134(v0: Adt1, v1: Bytes) -> bool {
    let m551 = f133(match v0 {
        Adt1::C0(h550, _, _) => {
            h550
        }
    }, v1);
    match m551 {
        None => {
            false
        }
        Some(v2) => {
            let m552 = v2;
            match m552 {
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

pub fn f135(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m553 = v1;
    match m553.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f192(v3.clone())
            } else {
                f71(f192(v3.clone()))
            } {
                List::cons(f111(v0.clone(), v3), f135(v0, v4, v2))
            } else {
                f135(v0, v4, v2)
            }
        }
    }
}

pub fn f136(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a554 = v0.clone();
        length_bytes(a554)
    };
    let v3: u64 = {
        let a555 = v1.clone();
        length_bytes(a555)
    };
    let v4: u64 = {
        let (h556, _) = f193(Fn3::F194(v1.clone(), v3, v0.clone(), v2), {
            let a557 = v2;
            let a558 = 1u64;
            nat_add(a557, a558)?
        }, 0u64)?;
        h556
    };
    if f195(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f137() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f138(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f196(match v0.clone() {
        Adt12::C0(h559, _, _, _, _) => {
            h559
        }
    }, 0u64)?, f196(match v0.clone() {
        Adt12::C0(_, h560, _, _, _) => {
            h560
        }
    }, 0u64)?, f196(match v0.clone() {
        Adt12::C0(_, _, h561, _, _) => {
            h561
        }
    }, 0u64)?, f196(match v0 {
        Adt12::C0(_, _, _, h562, _) => {
            h562
        }
    }, 0u64)?))
}

pub fn f139(v0: Adt12) -> List<Bytes> {
    f60(match v0.clone() {
        Adt12::C0(h563, _, _, _, _) => {
            h563
        }
    }, f60(match v0.clone() {
        Adt12::C0(_, _, h564, _, _) => {
            h564
        }
    }, f60(match v0.clone() {
        Adt12::C0(_, _, _, h565, _) => {
            h565
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h566) => {
            h566
        }
    })))
}

pub fn f140(v0: Adt15, p572: List<u64>, p573: List<Adt17>) -> Option<u64> {
    let m571 = (p572, p573);
    {
        let mut m571 = m571;
        loop {
            let step = {
                let (v1, v2) = m571;
                let m567 = v2;
                match m567.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt17>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f197(v0.clone(), v1.clone(), match v3.clone() {
                            Adt17::C0(_, h568) => {
                                h568
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt17>), Option<u64>>((List::cons(match v3 {
                                Adt17::C0(h569, _) => {
                                    h569
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt17>), Option<u64>>(Some(match v3 {
                                Adt17::C0(h570, _) => {
                                    h570
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt17>> {
    let m574 = v1;
    match m574.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt17::C0(v2, f198(v0.clone(), v3)?), f141(v0, v4, {
                let a575 = v2;
                let a576 = 1u64;
                nat_add(a575, a576)?
            })?))
        }
    }
}

pub fn f142(p580: List<Bytes>, p581: u64) -> Bytes {
    let m579 = (p580, p581);
    {
        let mut m579 = m579;
        loop {
            let step = {
                let (v0, v1) = m579;
                let m577 = v0;
                match m577.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m578 = v1;
                        if m578 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m578 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f199(v0.clone(), v1.clone()) {
        if f199(v1, v0) {
            f200(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f144(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m582 = v0;
    match m582.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f201(v2, v1.clone())?, f144(v3, v1)?))
        }
    }
}

pub fn f145(v0: List<Adt14>) -> List<u64> {
    let m583 = v0;
    match m583.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f202(v1), f145(v2))
        }
    }
}

pub fn f146(p586: List<u64>, v1: List<u64>) -> Option<u64> {
    let m585 = p586;
    {
        let mut m585 = m585;
        loop {
            let step = {
                let v0 = m585;
                let m584 = v0;
                match m584.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f203(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m585 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m587 = v0;
    match m587.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m588 = f204(v2, v1.clone())?;
            match m588 {
                None => {
                    f147(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f147(v3, v1)?))
                }
            }
        }
    }
}

pub fn f148(v0: List<Bytes>) -> List<Bytes> {
    f131(Fn1::F205, List::<Bytes>::nil(), v0)
}

pub fn f149(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f206(v0, v1)
}

pub fn f150(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f207(f208(f209(v0))?))
}

pub fn f151(v0: Adt1, v1: Bytes) -> R<bool> {
    let m589 = f119(v0.clone(), v1)?;
    match m589 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let v3: List<Bytes> = f210(f165(f107(f66(v0, f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f123(v2.clone(), f8(Str::lit("id"))))?;
            let m590 = v3.clone();
            match m590.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m592 = f128(f120({
                        let m591 = f120(v2.clone(), f8(Str::lit("source")));
                        match m591 {
                            None => {
                                Adt14::C5
                            }
                            Some(v6) => {
                                v6
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m592 {
                        None => {
                            Ok(false)
                        }
                        Some(v7) => {
                            Ok(f71(f80(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f161(f211(v2, v7))?, List::<Bytes>::nil()))), v3)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f152() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f153(v0: Adt1, p596: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m595 = p596;
    {
        let mut m595 = m595;
        loop {
            let step = {
                let v1 = m595;
                let m593 = v1;
                match m593.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f80(f212(f123({
                            let m594 = f119(v0.clone(), v3.clone())?;
                            match m594 {
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
                Ok(next) => { m595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f71(f72(match v0.clone() {
        Adt1::C0(_, h597, _) => {
            h597
        }
    }, f112())) {
        Ok(Adt11::C1(List::cons(f112(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f78(f74(f213(f76(match v0.clone() {
            Adt1::C0(_, h598, _) => {
                h598
            }
        }, f112())))?, v1);
        let m599 = f56(v0.clone(), List::cons(f82(), List::cons(f92(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m599.uncons() {
            None => {
                let v5: (bool, Bytes) = f214(v0.clone(), v2)?;
                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                    let (h600, _) = v5;
                    if h600 {
                        let (_, h601) = v5;
                        h601
                    } else {
                        let a603 = {
                            let (_, h602) = v5;
                            h602
                        };
                        let a604 = f215(v0)?;
                        append_bytes(a603, a604)
                    }
                }))
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f155(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f216(f69(v0, f82())?)?)))
}

pub fn f156(v0: List<Adt14>) -> List<Bytes> {
    let m605 = v0;
    match m605.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f123(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f123(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f123(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f156(v2))
        }
    }
}

pub fn f157(v0: List<Adt14>) -> List<Bytes> {
    let m606 = v0;
    match m606.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f123(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f217(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f157(v2))
        }
    }
}

pub fn f158(v0: Bytes, p664: u64, p665: u64, p666: Adt16) -> R<Adt16> {
    let m663 = (p664, (p665, p666));
    {
        let mut m663 = m663;
        loop {
            let step = {
                let (v1, (v2, v3)) = m663;
                let m607 = v2;
                if m607 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m607 - 1;
                    let v5: u64 = f218(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h608, _, _, _) => {
                            h608
                        }
                    } {
                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                            let a609 = v1;
                            let a610 = 1u64;
                            nat_add(a609, a610)?
                        }, (v4, Adt16::C0(true, false, match v3.clone() {
                            Adt16::C0(_, _, h611, _, _) => {
                                h611
                            }
                        }, match v3.clone() {
                            Adt16::C0(_, _, _, h612, _) => {
                                h612
                            }
                        }, match v3 {
                            Adt16::C0(_, _, _, _, h613) => {
                                h613
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt16::C0(h614, _, _, _, _) => {
                                h614
                            }
                        } {
                            let a615 = v5;
                            let a616 = 34u64;
                            if nat_eq(a615, a616) {
                                let a619 = f218(v0.clone(), {
                                    let a617 = v1;
                                    let a618 = 1u64;
                                    nat_add(a617, a618)?
                                })?;
                                let a620 = 34u64;
                                if nat_eq(a619, a620) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a621 = v1;
                                        let a622 = 1u64;
                                        nat_add(a621, a622)?
                                    }, (v4, Adt16::C0(true, true, f219(match v3.clone() {
                                        Adt16::C0(_, _, h623, _, _) => {
                                            h623
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h624, _) => {
                                            h624
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h625) => {
                                            h625
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a626 = v1;
                                        let a627 = 1u64;
                                        nat_add(a626, a627)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h628, _, _) => {
                                            h628
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h629, _) => {
                                            h629
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h630) => {
                                            h630
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a631 = v1;
                                    let a632 = 1u64;
                                    nat_add(a631, a632)?
                                }, (v4, Adt16::C0(true, false, f219(match v3.clone() {
                                    Adt16::C0(_, _, h633, _, _) => {
                                        h633
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h634, _) => {
                                        h634
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h635) => {
                                        h635
                                    }
                                }))))
                            }
                        } else {
                            let a636 = v5;
                            let a637 = 34u64;
                            if nat_eq(a636, a637) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a638 = v1;
                                    let a639 = 1u64;
                                    nat_add(a638, a639)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h640, _, _) => {
                                        h640
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h641, _) => {
                                        h641
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h642) => {
                                        h642
                                    }
                                }))))
                            } else {
                                let a643 = v5;
                                let a644 = 44u64;
                                if nat_eq(a643, a644) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a645 = v1;
                                        let a646 = 1u64;
                                        nat_add(a645, a646)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h647, _, _) => {
                                            h647
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h648, _) => {
                                            h648
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h649) => {
                                            h649
                                        }
                                    }))))
                                } else {
                                    let a650 = v5;
                                    let a651 = 13u64;
                                    if nat_eq(a650, a651) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a652 = v1;
                                            let a653 = 1u64;
                                            nat_add(a652, a653)?
                                        }, (v4, v3)))
                                    } else {
                                        let a654 = v5;
                                        let a655 = 10u64;
                                        if nat_eq(a654, a655) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a656 = v1;
                                                let a657 = 1u64;
                                                nat_add(a656, a657)?
                                            }, (v4, f159(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a658 = v1;
                                                let a659 = 1u64;
                                                nat_add(a658, a659)?
                                            }, (v4, Adt16::C0(false, false, f219(match v3.clone() {
                                                Adt16::C0(_, _, h660, _, _) => {
                                                    h660
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h661, _) => {
                                                    h661
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h662) => {
                                                    h662
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
                Ok(next) => { m663 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f220(List::cons(match v0.clone() {
        Adt16::C0(_, _, h667, _, _) => {
            h667
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h668, _) => {
            h668
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h669) => {
            h669
        }
    }))
}

pub fn f160(p672: List<List<Bytes>>, p673: List<List<Bytes>>) -> List<List<Bytes>> {
    let m671 = (p672, p673);
    {
        let mut m671 = m671;
        loop {
            let step = {
                let (v0, v1) = m671;
                let m670 = v0;
                match m670.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f221(v0.clone(), 0u64, {
        let a674 = v0;
        length_bytes(a674)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f162(v0: Adt20) -> Bytes {
    let a695 = f222(match v0.clone() {
        Adt20::C0(h675, _, _, _, _, _, _, _) => {
            h675
        }
    });
    let a696 = {
        let a693 = f222(match v0.clone() {
            Adt20::C0(_, h676, _, _, _, _, _, _) => {
                h676
            }
        });
        let a694 = {
            let a691 = f222(match v0.clone() {
                Adt20::C0(_, _, h677, _, _, _, _, _) => {
                    h677
                }
            });
            let a692 = {
                let a689 = f222(match v0.clone() {
                    Adt20::C0(_, _, _, h678, _, _, _, _) => {
                        h678
                    }
                });
                let a690 = {
                    let a687 = f222(match v0.clone() {
                        Adt20::C0(_, _, _, _, h679, _, _, _) => {
                            h679
                        }
                    });
                    let a688 = {
                        let a685 = f222(match v0.clone() {
                            Adt20::C0(_, _, _, _, _, h680, _, _) => {
                                h680
                            }
                        });
                        let a686 = {
                            let a683 = f222(match v0.clone() {
                                Adt20::C0(_, _, _, _, _, _, h681, _) => {
                                    h681
                                }
                            });
                            let a684 = f222(match v0 {
                                Adt20::C0(_, _, _, _, _, _, _, h682) => {
                                    h682
                                }
                            });
                            append_bytes(a683, a684)
                        };
                        append_bytes(a685, a686)
                    };
                    append_bytes(a687, a688)
                };
                append_bytes(a689, a690)
            };
            append_bytes(a691, a692)
        };
        append_bytes(a693, a694)
    };
    append_bytes(a695, a696)
}

pub fn f163(v0: Bytes) -> R<Adt20> {
    let v1: u64 = {
        let a697 = v0.clone();
        length_bytes(a697)
    };
    let v2: u64 = {
        let a703 = {
            let a700 = {
                let a698 = v1;
                let a699 = 72u64;
                nat_add(a698, a699)?
            };
            let a701 = 64u64;
            let a702 = 0u64;
            nat_quot(a700, a701, a702)
        };
        let a704 = 64u64;
        nat_mul(a703, a704)?
    };
    let (_, h705) = f223(Fn4::F224(v0, v1, v2), {
        let a706 = v2;
        let a707 = 64u64;
        let a708 = 0u64;
        nat_quot(a706, a707, a708)
    }, (0u64, f225()))?;
    Ok(h705)
}

pub fn f164(p711: List<List<Bytes>>, v1: Bytes, p712: List<Bytes>) -> R<List<Bytes>> {
    let m710 = (p711, p712);
    {
        let mut m710 = m710;
        loop {
            let step = {
                let (v0, v2) = m710;
                let m709 = v0;
                match m709.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f226(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f167(f227(v3), Bytes::lit(&[0x0a])), v2)))
                        }
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

pub fn f165(p715: List<Bytes>, p716: List<Bytes>, p717: List<List<Bytes>>) -> List<List<Bytes>> {
    let m714 = (p715, (p716, p717));
    {
        let mut m714 = m714;
        loop {
            let step = {
                let (v0, (v1, v2)) = m714;
                let m713 = v0;
                match m713.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f228(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f35(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f228(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f166(v0: Bytes) -> R<Bytes> {
    Ok(f31(v0.clone(), 0u64, f229(v0)?))
}

pub fn f167(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m718 = v0;
    match m718.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m719 = v3.clone();
            match m719.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a722 = v2;
                    let a723 = {
                        let a720 = v1.clone();
                        let a721 = f167(v3, v1);
                        append_bytes(a720, a721)
                    };
                    append_bytes(a722, a723)
                }
            }
        }
    }
}

pub fn f168(v0: List<Bytes>) -> Adt14 {
    let m724 = v0;
    match m724.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f168(v2)))
        }
    }
}

pub fn f169(v0: Fn2, p728: Option<Bytes>, p729: List<Bytes>) -> Option<Bytes> {
    let m727 = (p728, p729);
    {
        let mut m727 = m727;
        loop {
            let step = {
                let (v1, v2) = m727;
                let m725 = v2;
                match m725.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c726: Fn2 = v0.clone();
                            c726.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m727 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m730 = v1;
    match m730 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a733 = {
                    let a731 = v3;
                    let a732 = v0;
                    append_bytes(a731, a732)
                };
                let a734 = v2;
                append_bytes(a733, a734)
            })
        }
    }
}

pub fn f171(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f229(v0.clone())?;
    let v2: u64 = f230(v0.clone(), v1)?;
    Ok(f31(v0, v2, {
        let a735 = v1;
        let a736 = v2;
        nat_sub(a735, a736)
    }))
}

pub fn f172(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f231(v0)?;
    let m740 = f232(v1.clone(), 0u64, {
        let a738 = {
            let a737 = v1.clone();
            length_bytes(a737)
        };
        let a739 = 2u64;
        nat_add(a738, a739)?
    }, 0u64)?;
    match m740 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a743 = {
                let (_, h741) = v2;
                h741
            };
            let a744 = {
                let a742 = v1;
                length_bytes(a742)
            };
            if nat_eq(a743, a744) {
                Ok(Some({
                    let (h745, _) = v2;
                    h745
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f173(v0: Bytes) -> Bytes {
    f7(List::cons(f73(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f174(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f233(v0, v1)
}

pub fn f175(v0: Option<List<Adt14>>) -> bool {
    let m746 = v0;
    match m746 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f176(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m747 = v0;
    match m747 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f177(v0: Adt14) -> Bytes {
    if f187(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f187(v0.clone(), f8(Str::lit("id"))) {
            if f187(v0.clone(), f8(Str::lit("statement"))) {
                if f187(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f188(v0, f8(Str::lit("evidence")))
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

pub fn f178(v0: List<Adt14>) -> Adt14 {
    let m748 = v0;
    match m748.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f178(v2)))
        }
    }
}

pub fn f179(v0: Adt14) -> bool {
    if f187(v0.clone(), f8(Str::lit("name"))) {
        if f188(v0.clone(), f8(Str::lit("schema"))) {
            let m749 = f181(f70(f124(f120(v0, f8(Str::lit("citation"))))));
            match m749 {
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

pub fn f180(v0: Bytes) -> R<Bytes> {
    f234(v0.clone(), 0u64, {
        let a750 = v0;
        length_bytes(a750)
    }, true, Bytes::lit(&[]))
}

pub fn f181(v0: List<Bytes>) -> Option<Adt14> {
    let m751 = v0;
    match m751.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f35(v1.clone(), f8(Str::lit("req"))) {
                let m752 = v2;
                match m752.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f35(v1.clone(), f8(Str::lit("std"))) {
                    let m753 = v2;
                    match m753.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m754 = v6;
                                match m754.uncons() {
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
                    if f35(v1, f8(Str::lit("der"))) {
                        let m755 = v2;
                        match m755.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m756 = v10;
                                match m756.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f168(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f182(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m757 = f235(4096u64, f236(f237(f238(f120(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt21::C0(v2, v0), List::<Adt21>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m757 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f167(f228(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f183(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m758 = v0;
    match m758.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f183(v3, v1))
        }
    }
}

pub fn f184(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f167(f239(f107(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f185(v0: Adt14) -> R<Bytes> {
    let m759 = v0;
    match m759 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f240(v2, v3)
        }
        Adt14::C2(v4) => {
            f241(v4)
        }
        Adt14::C3(r760) => {
            let v5 = (*r760).clone();
            Ok(f242(f243(v5)?))
        }
        Adt14::C4(r761) => {
            let v6 = (*r761).clone();
            f244(f245(v6)?)
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

pub fn f186(v0: Adt14) -> List<Adt14> {
    let m762 = v0;
    match m762 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r763) => {
            let v3 = (*r763).clone();
            f186(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r764, r765) => {
            let v1 = (*r764).clone();
            let v2 = (*r765).clone();
            List::cons(v1, f186(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f187(v0: Adt14, v1: Bytes) -> bool {
    f71({
        let a767 = {
            let a766 = f123(v0, v1);
            length_bytes(a766)
        };
        let a768 = 0u64;
        nat_eq(a767, a768)
    })
}

pub fn f188(v0: Adt14, v1: Bytes) -> bool {
    let m769 = f120(v0, v1);
    match m769 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f189(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m770 = f120(v0, v1);
    match m770 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f124(f120(v3, v2))
        }
    }
}

pub fn f190(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f120(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f101(Str::lit("id"), f123(v0.clone(), f8(Str::lit("id"))))?, List::cons(f101(Str::lit("name"), f123(v0.clone(), f8(Str::lit("name"))))?, List::cons(f101(Str::lit("issuer"), f123(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f101(Str::lit("edition"), f123(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f101(Str::lit("canonical_id"), f123(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f101(Str::lit("source_role"), f123(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f101(Str::lit("retrieval_date"), f123(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f101(Str::lit("url"), f246(v2.clone(), f8(Str::lit("url"))))?, List::cons(f101(Str::lit("media_type"), f246(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f101(Str::lit("license"), f246(v2.clone(), f8(Str::lit("license"))))?, List::cons(f101(Str::lit("redistribution"), f246(v2, f8(Str::lit("redistribution"))))?, List::cons(f101(Str::lit("statement"), f123(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m771 = v1;
        match m771 {
            None => {
                f7(List::cons(f101(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f101(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f101(Str::lit("artifact"), {
                    let (h772, _) = v3;
                    h772
                })?, List::cons(f101(Str::lit("checksum"), {
                    let (_, h773) = v3;
                    h773
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f247(f70(f124(f120(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f191(v0: Bytes, v1: u64) -> Option<u8> {
    let a774 = v0;
    let a775 = v1;
    index_bytes(a774, a775)
}

pub fn f192(v0: Bytes) -> bool {
    f108(v0, f8(Str::lit(".lex.tex")))
}

pub fn f193(v0: Fn3, p780: u64, p781: u64) -> R<(u64, bool)> {
    let m779 = (p780, p781);
    {
        let mut m779 = m779;
        loop {
            let step = {
                let (v1, v2) = m779;
                let m776 = v1;
                if m776 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m776 - 1;
                    let m778 = {
                        let c777: Fn3 = v0.clone();
                        c777.apply(v2)?
                    };
                    match m778 {
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
                Ok(next) => { m779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f195(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a784 = {
            let a782 = v4;
            let a783 = v1;
            nat_add(a782, a783)?
        };
        let a785 = v3;
        if nat_lt(a784, a785) {
            Ok(Some({
                let a786 = v4;
                let a787 = 1u64;
                nat_add(a786, a787)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f195(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m792 = {
        let a789 = v0;
        let a790 = v2;
        let a791 = {
            let a788 = v1.clone();
            length_bytes(a788)
        };
        slice_bytes(a789, a790, a791)
    };
    match m792 {
        None => {
            false
        }
        Some(v3) => {
            let a793 = v3;
            let a794 = v1;
            equal(a793, a794)
        }
    }
}

pub fn f196(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m795 = v0;
    match m795.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f196(v3, {
                let a796 = v1;
                let a797 = 1u64;
                nat_add(a796, a797)?
            })?))
        }
    }
}

pub fn f197(v0: Adt15, v1: List<u64>, v2: Adt18) -> bool {
    let m798 = v2;
    match m798 {
        Adt18::C0(v3, v4) => {
            if f203(v3, match v0.clone() {
                Adt15::C0(h799, _, _, _) => {
                    h799
                }
            }) {
                f248(v0, v4)
            } else {
                false
            }
        }
        Adt18::C1(v5) => {
            f203(v5, match v0 {
                Adt15::C0(_, _, h800, _) => {
                    h800
                }
            })
        }
        Adt18::C2(_, v7, v8) => {
            if f199(v7, v1) {
                f203(v8, match v0 {
                    Adt15::C0(_, _, _, h801) => {
                        h801
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f198(v0: Adt12, v1: Adt14) -> R<Adt18> {
    let m802 = f120(v1.clone(), f8(Str::lit("std")));
    match m802 {
        None => {
            let m807 = f128(f120(v1.clone(), f8(Str::lit("req"))));
            match m807 {
                None => {
                    let m809 = f120(v1, f8(Str::lit("der")));
                    match m809 {
                        None => {
                            Ok(f249(v0))
                        }
                        Some(v5) => {
                            let m810 = f250(f123(v5.clone(), f8(Str::lit("rule"))));
                            match m810 {
                                None => {
                                    Ok(f249(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt18::C2(v6, f144(f70(f124(f120(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h811) => {
                                            h811
                                        }
                                    })?, f201(f123(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h812, _) => {
                                            h812
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt18::C1(f201(v4, match v0 {
                        Adt12::C0(_, _, h808, _, _) => {
                            h808
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m803 = f128(f120(v2.clone(), f8(Str::lit("row"))));
            match m803 {
                None => {
                    Ok(Adt18::C0(f201(f123(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h804, _, _, _, _) => {
                            h804
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt18::C0(f201(f123(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h805, _, _, _, _) => {
                            h805
                        }
                    })?, Some(f201(v3, match v0 {
                        Adt12::C0(_, h806, _, _, _) => {
                            h806
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f199(p815: List<u64>, v1: List<u64>) -> bool {
    let m814 = p815;
    {
        let mut m814 = m814;
        loop {
            let step = {
                let v0 = m814;
                let m813 = v0;
                match m813.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f203(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(p820: List<u64>) -> bool {
    let m819 = p820;
    {
        let mut m819 = m819;
        loop {
            let step = {
                let v0 = m819;
                let m816 = v0;
                match m816.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a817 = v1;
                        let a818 = 7u64;
                        if nat_le(a817, a818) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m819 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m821 = f251(v0, v1.clone(), 0u64)?;
    match m821 {
        None => {
            let a822 = v1;
            Ok(length_list(a822))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f202(v0: Adt14) -> u64 {
    let m823 = f252(f120(v0, f8(Str::lit("stratum"))));
    match m823 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f203(v0: u64, p828: List<u64>) -> bool {
    let m827 = p828;
    {
        let mut m827 = m827;
        loop {
            let step = {
                let v1 = m827;
                let m824 = v1;
                match m824.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a825 = v2;
                        let a826 = v0;
                        if nat_eq(a825, a826) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f253(v0.clone(), v1) {
        let m829 = f29(v0, 34u8)?;
        match m829.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m830 = v3;
                match m830.uncons() {
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

pub fn f205(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f206(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m831 = v0.clone();
    match m831.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a832 = v2;
            let a833 = v1.clone();
            if equal(a832, a833) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f207(v0: List<(Bytes, ())>) -> List<Bytes> {
    f148(f254(Fn5::F255, List::<Bytes>::nil(), v0))
}

pub fn f208(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f256(f257(Fn6::F258, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m838 = {
        let (h834, _) = f259(Fn7::F260, {
            let a836 = {
                let a835 = v0;
                length_list(a835)
            };
            let a837 = 1u64;
            nat_add(a836, a837)?
        }, v1)?;
        h834
    };
    match m838.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f209(v0: List<Bytes>) -> List<(Bytes, ())> {
    f261(f262(Fn8::F263, List::<(Bytes, ())>::nil(), v0))
}

pub fn f210(p841: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m840 = p841;
    {
        let mut m840 = m840;
        loop {
            let step = {
                let v0 = m840;
                let m839 = v0;
                match m839.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f226(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
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

pub fn f211(v0: Adt14, v1: Bytes) -> Bytes {
    if f35(f246(f120(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f212(v0: Bytes) -> R<Bytes> {
    let a842 = f8(Str::lit("Catalog."));
    let a843 = f180(v0)?;
    Ok(append_bytes(a842, a843))
}

pub fn f213(v0: List<Bytes>) -> List<Bytes> {
    let m844 = v0;
    match m844.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f108(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f111(f112(), v1), f213(v2))
            } else {
                f213(v2)
            }
        }
    }
}

pub fn f214(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m845 = v1;
    match m845.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f264(v0.clone(), v2)?;
            let (h846, _) = v4.clone();
            if h846 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f214(v0, v3)?;
                let (h847, _) = v5.clone();
                if h847 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a850 = {
                            let (_, h848) = v4;
                            h848
                        };
                        let a851 = {
                            let (_, h849) = v5;
                            h849
                        };
                        append_bytes(a850, a851)
                    }))
                }
            }
        }
    }
}

pub fn f215(v0: Adt1) -> R<Bytes> {
    let a853 = {
        let a852 = f93(f94(f95(f96(f66(v0.clone(), f92()))?)), f97(f69(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a852)
    };
    let a854 = 0u64;
    if nat_eq(a853, a854) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f216(v0: List<Adt14>) -> R<List<Bytes>> {
    let m855 = v0;
    match m855.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f105(f70(f124(f120(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f123(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f123(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f265(f107(f123(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f216(v2)?))
        }
    }
}

pub fn f217(v0: Adt14) -> Bytes {
    if f187(v0.clone(), f8(Str::lit("req"))) {
        let a856 = f8(Str::lit("req "));
        let a857 = f123(v0, f8(Str::lit("req")));
        append_bytes(a856, a857)
    } else {
        let m858 = f120(v0.clone(), f8(Str::lit("std")));
        match m858 {
            None => {
                let m861 = f120(v0, f8(Str::lit("der")));
                match m861 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f123(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f105(f70(f124(f120(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f123(v1.clone(), f8(Str::lit("authority"))), List::cons(if f187(v1.clone(), f8(Str::lit("row"))) {
                    let a859 = f8(Str::lit(" row "));
                    let a860 = f123(v1, f8(Str::lit("row")));
                    append_bytes(a859, a860)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f218(v0: Bytes, v1: u64) -> R<u64> {
    let m864 = {
        let a862 = v0;
        let a863 = v1;
        index_bytes(a862, a863)
    };
    match m864 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a909 = {
                let a885 = {
                    let a873 = {
                        let a867 = {
                            let a865 = v3;
                            let a866 = 128u8;
                            fixed_u8::bit_and(a865, a866)
                        };
                        let a868 = 128u8;
                        if equal(a867, a868) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a874 = {
                        let a871 = {
                            let a869 = v3;
                            let a870 = 64u8;
                            fixed_u8::bit_and(a869, a870)
                        };
                        let a872 = 64u8;
                        if equal(a871, a872) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a873, a874)?
                };
                let a886 = {
                    let a883 = {
                        let a877 = {
                            let a875 = v3;
                            let a876 = 32u8;
                            fixed_u8::bit_and(a875, a876)
                        };
                        let a878 = 32u8;
                        if equal(a877, a878) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a884 = {
                        let a881 = {
                            let a879 = v3;
                            let a880 = 16u8;
                            fixed_u8::bit_and(a879, a880)
                        };
                        let a882 = 16u8;
                        if equal(a881, a882) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a883, a884)?
                };
                nat_add(a885, a886)?
            };
            let a910 = {
                let a907 = {
                    let a895 = {
                        let a889 = {
                            let a887 = v3;
                            let a888 = 8u8;
                            fixed_u8::bit_and(a887, a888)
                        };
                        let a890 = 8u8;
                        if equal(a889, a890) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a896 = {
                        let a893 = {
                            let a891 = v3;
                            let a892 = 4u8;
                            fixed_u8::bit_and(a891, a892)
                        };
                        let a894 = 4u8;
                        if equal(a893, a894) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a895, a896)?
                };
                let a908 = {
                    let a905 = {
                        let a899 = {
                            let a897 = v3;
                            let a898 = 2u8;
                            fixed_u8::bit_and(a897, a898)
                        };
                        let a900 = 2u8;
                        if equal(a899, a900) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a906 = {
                        let a903 = {
                            let a901 = v3;
                            let a902 = 1u8;
                            fixed_u8::bit_and(a901, a902)
                        };
                        let a904 = 1u8;
                        if equal(a903, a904) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a905, a906)?
                };
                nat_add(a907, a908)?
            };
            nat_add(a909, a910)
        }
    }
}

pub fn f219(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m914 = {
        let a911 = v1;
        let a912 = v2;
        let a913 = 1u64;
        slice_bytes(a911, a912, a913)
    };
    match m914 {
        None => {
            v0
        }
        Some(v3) => {
            let a915 = v0;
            let a916 = v3;
            append_bytes(a915, a916)
        }
    }
}

pub fn f220(p919: List<Bytes>, p920: List<Bytes>) -> List<Bytes> {
    let m918 = (p919, p920);
    {
        let mut m918 = m918;
        loop {
            let step = {
                let (v0, v1) = m918;
                let m917 = v0;
                match m917.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: Bytes, p992: u64, p993: u64, p994: Bytes) -> R<Bytes> {
    let m991 = (p992, (p993, p994));
    {
        let mut m991 = m991;
        loop {
            let step = {
                let (v1, (v2, v3)) = m991;
                let m921 = v2;
                if m921 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m921 - 1;
                    let m924 = {
                        let a922 = v0.clone();
                        let a923 = v1;
                        index_bytes(a922, a923)
                    };
                    match m924 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a969 = {
                                    let a945 = {
                                        let a933 = {
                                            let a927 = {
                                                let a925 = v6;
                                                let a926 = 128u8;
                                                fixed_u8::bit_and(a925, a926)
                                            };
                                            let a928 = 128u8;
                                            if equal(a927, a928) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a934 = {
                                            let a931 = {
                                                let a929 = v6;
                                                let a930 = 64u8;
                                                fixed_u8::bit_and(a929, a930)
                                            };
                                            let a932 = 64u8;
                                            if equal(a931, a932) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a933, a934)?
                                    };
                                    let a946 = {
                                        let a943 = {
                                            let a937 = {
                                                let a935 = v6;
                                                let a936 = 32u8;
                                                fixed_u8::bit_and(a935, a936)
                                            };
                                            let a938 = 32u8;
                                            if equal(a937, a938) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a944 = {
                                            let a941 = {
                                                let a939 = v6;
                                                let a940 = 16u8;
                                                fixed_u8::bit_and(a939, a940)
                                            };
                                            let a942 = 16u8;
                                            if equal(a941, a942) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a943, a944)?
                                    };
                                    nat_add(a945, a946)?
                                };
                                let a970 = {
                                    let a967 = {
                                        let a955 = {
                                            let a949 = {
                                                let a947 = v6;
                                                let a948 = 8u8;
                                                fixed_u8::bit_and(a947, a948)
                                            };
                                            let a950 = 8u8;
                                            if equal(a949, a950) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a956 = {
                                            let a953 = {
                                                let a951 = v6;
                                                let a952 = 4u8;
                                                fixed_u8::bit_and(a951, a952)
                                            };
                                            let a954 = 4u8;
                                            if equal(a953, a954) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a955, a956)?
                                    };
                                    let a968 = {
                                        let a965 = {
                                            let a959 = {
                                                let a957 = v6;
                                                let a958 = 2u8;
                                                fixed_u8::bit_and(a957, a958)
                                            };
                                            let a960 = 2u8;
                                            if equal(a959, a960) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a966 = {
                                            let a963 = {
                                                let a961 = v6;
                                                let a962 = 1u8;
                                                fixed_u8::bit_and(a961, a962)
                                            };
                                            let a964 = 1u8;
                                            if equal(a963, a964) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a965, a966)?
                                    };
                                    nat_add(a967, a968)?
                                };
                                nat_add(a969, a970)?
                            };
                            let a971 = v7;
                            let a972 = 34u64;
                            if if nat_eq(a971, a972) {
                                true
                            } else {
                                let a973 = v7;
                                let a974 = 92u64;
                                nat_eq(a973, a974)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a975 = v1;
                                    let a976 = 1u64;
                                    nat_add(a975, a976)?
                                }, (v4, {
                                    let a979 = v3;
                                    let a980 = {
                                        let a977 = Bytes::lit(&[0x5c]);
                                        let a978 = f31(v0.clone(), v1, 1u64);
                                        append_bytes(a977, a978)
                                    };
                                    append_bytes(a979, a980)
                                })))
                            } else {
                                let a981 = v7;
                                let a982 = 10u64;
                                if nat_eq(a981, a982) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a983 = v1;
                                        let a984 = 1u64;
                                        nat_add(a983, a984)?
                                    }, (v4, {
                                        let a985 = v3;
                                        let a986 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a985, a986)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a987 = v1;
                                        let a988 = 1u64;
                                        nat_add(a987, a988)?
                                    }, (v4, {
                                        let a989 = v3;
                                        let a990 = f31(v0.clone(), v1, 1u64);
                                        append_bytes(a989, a990)
                                    })))
                                }
                            }
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

pub fn f222(v0: u32) -> Bytes {
    let a1023 = f266({
        let a995 = f267(v0, 28u32);
        let a996 = 15u32;
        fixed_u32::bit_and(a995, a996)
    });
    let a1024 = {
        let a1021 = f266({
            let a997 = f267(v0, 24u32);
            let a998 = 15u32;
            fixed_u32::bit_and(a997, a998)
        });
        let a1022 = {
            let a1019 = f266({
                let a999 = f267(v0, 20u32);
                let a1000 = 15u32;
                fixed_u32::bit_and(a999, a1000)
            });
            let a1020 = {
                let a1017 = f266({
                    let a1001 = f267(v0, 16u32);
                    let a1002 = 15u32;
                    fixed_u32::bit_and(a1001, a1002)
                });
                let a1018 = {
                    let a1015 = f266({
                        let a1003 = f267(v0, 12u32);
                        let a1004 = 15u32;
                        fixed_u32::bit_and(a1003, a1004)
                    });
                    let a1016 = {
                        let a1013 = f266({
                            let a1005 = f267(v0, 8u32);
                            let a1006 = 15u32;
                            fixed_u32::bit_and(a1005, a1006)
                        });
                        let a1014 = {
                            let a1011 = f266({
                                let a1007 = f267(v0, 4u32);
                                let a1008 = 15u32;
                                fixed_u32::bit_and(a1007, a1008)
                            });
                            let a1012 = f266({
                                let a1009 = v0;
                                let a1010 = 15u32;
                                fixed_u32::bit_and(a1009, a1010)
                            });
                            append_bytes(a1011, a1012)
                        };
                        append_bytes(a1013, a1014)
                    };
                    append_bytes(a1015, a1016)
                };
                append_bytes(a1017, a1018)
            };
            append_bytes(a1019, a1020)
        };
        append_bytes(a1021, a1022)
    };
    append_bytes(a1023, a1024)
}

pub fn f223(v0: Fn4, p1028: u64, p1029: (u64, Adt20)) -> R<(u64, Adt20)> {
    let m1027 = (p1028, p1029);
    {
        let mut m1027 = m1027;
        loop {
            let step = {
                let (v1, v2) = m1027;
                let m1025 = v1;
                if m1025 == 0 {
                    Err::<(u64, (u64, Adt20)), R<(u64, Adt20)>>(Ok(v2))
                } else {
                    let v3 = m1025 - 1;
                    Ok::<(u64, (u64, Adt20)), R<(u64, Adt20)>>((v3, {
                        let c1026: Fn4 = v0.clone();
                        c1026.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt20)) -> R<(u64, Adt20)> {
    Ok(({
        let a1031 = {
            let (h1030, _) = v3.clone();
            h1030
        };
        let a1032 = 64u64;
        nat_add(a1031, a1032)?
    }, f268({
        let (_, h1033) = v3;
        h1033
    }, f269(v0, v1, v2, {
        let (h1034, _) = v3;
        h1034
    })?)?))
}

pub fn f225() -> Adt20 {
    Adt20::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f226(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f80(v1, f65(f167(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f227(v0: List<Bytes>) -> List<Bytes> {
    let m1035 = v0;
    match m1035.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f35(v1.clone(), f8(Str::lit("authority = []"))) {
                f227(v2)
            } else {
                List::cons(v1, f227(v2))
            }
        }
    }
}

pub fn f228(p1038: List<Bytes>, p1039: List<Bytes>) -> List<Bytes> {
    let m1037 = (p1038, p1039);
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let (v0, v1) = m1037;
                let m1036 = v0;
                match m1036.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f229(v0: Bytes) -> R<u64> {
    let (h1040, _) = f193(Fn3::F270(v0.clone()), {
        let a1041 = v0.clone();
        length_bytes(a1041)
    }, {
        let a1042 = v0;
        length_bytes(a1042)
    })?;
    Ok(h1040)
}

pub fn f230(v0: Bytes, v1: u64) -> R<u64> {
    let (h1043, _) = f193(Fn3::F271(v1, v0), v1, 0u64)?;
    Ok(h1043)
}

pub fn f231(v0: Bytes) -> R<Bytes> {
    f272(v0.clone(), 0u64, {
        let a1044 = v0;
        length_bytes(a1044)
    }, false, false, Bytes::lit(&[]))
}

pub fn f232(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1045 = v2;
    if m1045 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1045 - 1;
        let v5: u64 = f273(v0.clone(), v1)?;
        let a1046 = v3;
        let a1047 = 1u64;
        if nat_eq(a1046, a1047) {
            let m1048 = f232(v0.clone(), v1, v4, 0u64)?;
            match m1048 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f273(v0.clone(), {
                        let (_, h1049) = v6.clone();
                        h1049
                    })?;
                    let a1050 = v7;
                    let a1051 = 44u64;
                    if nat_eq(a1050, a1051) {
                        let m1055 = f232(v0, {
                            let a1053 = {
                                let (_, h1052) = v6;
                                h1052
                            };
                            let a1054 = 1u64;
                            nat_add(a1053, a1054)?
                        }, v4, 1u64)?;
                        match m1055 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1056, _) = v6;
                                    h1056
                                }), std::rc::Rc::new({
                                    let (h1057, _) = v8;
                                    h1057
                                })), {
                                    let (_, h1058) = v8;
                                    h1058
                                })))
                            }
                        }
                    } else {
                        let a1059 = v7;
                        let a1060 = 93u64;
                        if nat_eq(a1059, a1060) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1061, _) = v6;
                                h1061
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1063 = {
                                    let (_, h1062) = v6;
                                    h1062
                                };
                                let a1064 = 1u64;
                                nat_add(a1063, a1064)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1065 = v3;
            let a1066 = 2u64;
            if nat_eq(a1065, a1066) {
                let m1069 = f274(v0.clone(), {
                    let a1067 = v1;
                    let a1068 = 1u64;
                    nat_add(a1067, a1068)?
                })?;
                match m1069 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1070 = v5;
                        let a1071 = 34u64;
                        if if nat_eq(a1070, a1071) {
                            let a1073 = f273(v0.clone(), {
                                let (h1072, _) = v9.clone();
                                h1072
                            })?;
                            let a1074 = 58u64;
                            nat_eq(a1073, a1074)
                        } else {
                            false
                        } {
                            let m1078 = f232(v0.clone(), {
                                let a1076 = {
                                    let (h1075, _) = v9;
                                    h1075
                                };
                                let a1077 = 1u64;
                                nat_add(a1076, a1077)?
                            }, v4, 0u64)?;
                            match m1078 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f273(v0.clone(), {
                                        let (_, h1079) = v10.clone();
                                        h1079
                                    })?;
                                    let a1080 = v11;
                                    let a1081 = 44u64;
                                    if nat_eq(a1080, a1081) {
                                        let m1085 = f232(v0, {
                                            let a1083 = {
                                                let (_, h1082) = v10;
                                                h1082
                                            };
                                            let a1084 = 1u64;
                                            nat_add(a1083, a1084)?
                                        }, v4, 2u64)?;
                                        match m1085 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1086) = v9;
                                                    h1086
                                                }, std::rc::Rc::new({
                                                    let (h1087, _) = v10;
                                                    h1087
                                                }), std::rc::Rc::new({
                                                    let (h1088, _) = v12;
                                                    h1088
                                                })), {
                                                    let (_, h1089) = v12;
                                                    h1089
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1090 = v11;
                                        let a1091 = 125u64;
                                        if nat_eq(a1090, a1091) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1092) = v9;
                                                h1092
                                            }, std::rc::Rc::new({
                                                let (h1093, _) = v10;
                                                h1093
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1095 = {
                                                    let (_, h1094) = v10;
                                                    h1094
                                                };
                                                let a1096 = 1u64;
                                                nat_add(a1095, a1096)?
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
                let a1097 = v5;
                let a1098 = 34u64;
                if nat_eq(a1097, a1098) {
                    let m1101 = f274(v0, {
                        let a1099 = v1;
                        let a1100 = 1u64;
                        nat_add(a1099, a1100)?
                    })?;
                    match m1101 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1102) = v13;
                                h1102
                            }), {
                                let (h1103, _) = v13;
                                h1103
                            })))
                        }
                    }
                } else {
                    if f35(f31(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1104 = v1;
                            let a1105 = 4u64;
                            nat_add(a1104, a1105)?
                        })))
                    } else {
                        if f35(f31(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1106 = v1;
                                let a1107 = 5u64;
                                nat_add(a1106, a1107)?
                            })))
                        } else {
                            let a1108 = v5;
                            let a1109 = 91u64;
                            if nat_eq(a1108, a1109) {
                                let a1112 = f273(v0.clone(), {
                                    let a1110 = v1;
                                    let a1111 = 1u64;
                                    nat_add(a1110, a1111)?
                                })?;
                                let a1113 = 93u64;
                                if nat_eq(a1112, a1113) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1114 = v1;
                                        let a1115 = 2u64;
                                        nat_add(a1114, a1115)?
                                    })))
                                } else {
                                    let m1118 = f232(v0, {
                                        let a1116 = v1;
                                        let a1117 = 1u64;
                                        nat_add(a1116, a1117)?
                                    }, v4, 1u64)?;
                                    match m1118 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1119, _) = v14;
                                                h1119
                                            })), {
                                                let (_, h1120) = v14;
                                                h1120
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1121 = v5;
                                let a1122 = 123u64;
                                if nat_eq(a1121, a1122) {
                                    let a1125 = f273(v0.clone(), {
                                        let a1123 = v1;
                                        let a1124 = 1u64;
                                        nat_add(a1123, a1124)?
                                    })?;
                                    let a1126 = 125u64;
                                    if nat_eq(a1125, a1126) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1127 = v1;
                                            let a1128 = 2u64;
                                            nat_add(a1127, a1128)?
                                        })))
                                    } else {
                                        let m1131 = f232(v0, {
                                            let a1129 = v1;
                                            let a1130 = 1u64;
                                            nat_add(a1129, a1130)?
                                        }, v4, 2u64)?;
                                        match m1131 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1132, _) = v15;
                                                    h1132
                                                })), {
                                                    let (_, h1133) = v15;
                                                    h1133
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1134 = f275(v0, v1)?;
                                    match m1134 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1135) = v16;
                                                h1135
                                            }, {
                                                let (h1136, _) = v16;
                                                h1136
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

pub fn f233(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1137, _) = v1.clone();
        h1137
    };
    let v3: u64 = {
        let (h1138, _) = {
            let (_, h1139) = v1.clone();
            h1139
        };
        h1138
    };
    let v4: List<Bytes> = {
        let (_, h1140) = {
            let (_, h1141) = v1;
            h1141
        };
        h1140
    };
    let m1142 = f191(v0.clone(), v2);
    match m1142 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1143 = v5;
            let a1144 = 10u8;
            if equal(a1143, a1144) {
                Ok(Some(({
                    let a1145 = v2;
                    let a1146 = 1u64;
                    nat_add(a1145, a1146)?
                }, ({
                    let a1147 = v2;
                    let a1148 = 1u64;
                    nat_add(a1147, a1148)?
                }, List::cons(f276(f31(v0, v3, {
                    let a1149 = v2;
                    let a1150 = v3;
                    nat_sub(a1149, a1150)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1151 = v2;
                    let a1152 = 1u64;
                    nat_add(a1151, a1152)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f234(v0: Bytes, p1219: u64, p1220: u64, p1221: bool, p1222: Bytes) -> R<Bytes> {
    let m1218 = (p1219, (p1220, (p1221, p1222)));
    {
        let mut m1218 = m1218;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1218;
                let m1153 = v2;
                if m1153 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1153 - 1;
                    let m1156 = {
                        let a1154 = v0.clone();
                        let a1155 = v1;
                        index_bytes(a1154, a1155)
                    };
                    match m1156 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1201 = {
                                    let a1177 = {
                                        let a1165 = {
                                            let a1159 = {
                                                let a1157 = v7;
                                                let a1158 = 128u8;
                                                fixed_u8::bit_and(a1157, a1158)
                                            };
                                            let a1160 = 128u8;
                                            if equal(a1159, a1160) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1166 = {
                                            let a1163 = {
                                                let a1161 = v7;
                                                let a1162 = 64u8;
                                                fixed_u8::bit_and(a1161, a1162)
                                            };
                                            let a1164 = 64u8;
                                            if equal(a1163, a1164) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1165, a1166)?
                                    };
                                    let a1178 = {
                                        let a1175 = {
                                            let a1169 = {
                                                let a1167 = v7;
                                                let a1168 = 32u8;
                                                fixed_u8::bit_and(a1167, a1168)
                                            };
                                            let a1170 = 32u8;
                                            if equal(a1169, a1170) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1176 = {
                                            let a1173 = {
                                                let a1171 = v7;
                                                let a1172 = 16u8;
                                                fixed_u8::bit_and(a1171, a1172)
                                            };
                                            let a1174 = 16u8;
                                            if equal(a1173, a1174) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1175, a1176)?
                                    };
                                    nat_add(a1177, a1178)?
                                };
                                let a1202 = {
                                    let a1199 = {
                                        let a1187 = {
                                            let a1181 = {
                                                let a1179 = v7;
                                                let a1180 = 8u8;
                                                fixed_u8::bit_and(a1179, a1180)
                                            };
                                            let a1182 = 8u8;
                                            if equal(a1181, a1182) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1188 = {
                                            let a1185 = {
                                                let a1183 = v7;
                                                let a1184 = 4u8;
                                                fixed_u8::bit_and(a1183, a1184)
                                            };
                                            let a1186 = 4u8;
                                            if equal(a1185, a1186) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1187, a1188)?
                                    };
                                    let a1200 = {
                                        let a1197 = {
                                            let a1191 = {
                                                let a1189 = v7;
                                                let a1190 = 2u8;
                                                fixed_u8::bit_and(a1189, a1190)
                                            };
                                            let a1192 = 2u8;
                                            if equal(a1191, a1192) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1198 = {
                                            let a1195 = {
                                                let a1193 = v7;
                                                let a1194 = 1u8;
                                                fixed_u8::bit_and(a1193, a1194)
                                            };
                                            let a1196 = 1u8;
                                            if equal(a1195, a1196) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1197, a1198)?
                                    };
                                    nat_add(a1199, a1200)?
                                };
                                nat_add(a1201, a1202)?
                            };
                            if f277(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1203 = v1;
                                    let a1204 = 1u64;
                                    nat_add(a1203, a1204)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f278(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1205 = v1;
                                        let a1206 = 1u64;
                                        nat_add(a1205, a1206)?
                                    }, (v5, (false, {
                                        let a1210 = v4;
                                        let a1211 = f31({
                                            let a1207 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1207)
                                        }, {
                                            let a1208 = v8;
                                            let a1209 = 97u64;
                                            nat_sub(a1208, a1209)
                                        }, 1u64);
                                        append_bytes(a1210, a1211)
                                    }))))
                                } else {
                                    if if f278(v8) {
                                        true
                                    } else {
                                        if f279(v8) {
                                            true
                                        } else {
                                            f280(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1212 = v1;
                                            let a1213 = 1u64;
                                            nat_add(a1212, a1213)?
                                        }, (v5, (false, {
                                            let a1214 = v4;
                                            let a1215 = f31(v0.clone(), v1, 1u64);
                                            append_bytes(a1214, a1215)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1216 = v1;
                                            let a1217 = 1u64;
                                            nat_add(a1216, a1217)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(p1230: u64, p1231: List<Adt21>, p1232: List<Bytes>, p1233: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1229 = (p1230, (p1231, (p1232, p1233)));
    {
        let mut m1229 = m1229;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1229;
                let m1223 = v0;
                if m1223 == 0 {
                    Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1223 - 1;
                    let m1224 = v1;
                    match m1224.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1225 = v5;
                            match m1225 {
                                Adt21::C0(v8, v9) => {
                                    if f80(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m1226 = f281(v8.clone(), v9)?;
                                        match m1226 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f236({
                                                    let (_, h1227) = v11;
                                                    h1227
                                                }, List::cons(Adt21::C1({
                                                    let (h1228, _) = v11;
                                                    h1228
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
                Ok(next) => { m1229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: List<Adt21>, v1: List<Adt21>) -> List<Adt21> {
    let m1234 = v0;
    match m1234.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f236(v3, v1))
        }
    }
}

pub fn f237(v0: List<(Bytes, Adt14)>) -> R<List<Adt21>> {
    let m1235 = v0;
    match m1235.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt21::C0(f180({
                let (h1236, _) = v1;
                h1236
            })?, {
                let (_, h1237) = v1;
                h1237
            }), f237(v2)?))
        }
    }
}

pub fn f238(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1238 = v0;
    match m1238 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f282(v1)
        }
    }
}

pub fn f239(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1239 = v0;
    match m1239.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f253(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f167(f283(f74(List::cons(v1, f284(f29(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f239(v3, v1)?))
            }
        }
    }
}

pub fn f240(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1240 = 0u64;
        let a1241 = v1;
        nat_lt(a1240, a1241)
    } else {
        false
    } {
        let a1242 = Bytes::lit(&[0x2d]);
        let a1243 = f285(v1)?;
        Ok(append_bytes(a1242, a1243))
    } else {
        f285(v1)
    }
}

pub fn f241(v0: Bytes) -> R<Bytes> {
    let a1249 = {
        let a1247 = Bytes::lit(&[0x22]);
        let a1248 = {
            let (_, h1244) = {
                let (h1245, _) = f286(Fn9::F287(v0.clone()), {
                    let a1246 = v0;
                    length_bytes(a1246)
                }, (0u64, Bytes::lit(&[])))?;
                h1245
            };
            h1244
        };
        append_bytes(a1247, a1248)
    };
    let a1250 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1249, a1250))
}

pub fn f242(v0: List<Bytes>) -> Bytes {
    let a1253 = {
        let a1251 = Bytes::lit(&[0x5b]);
        let a1252 = f105(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1251, a1252)
    };
    let a1254 = Bytes::lit(&[0x5d]);
    append_bytes(a1253, a1254)
}

pub fn f243(v0: Adt14) -> R<List<Bytes>> {
    let m1255 = v0;
    match m1255 {
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
        Adt14::C6(r1256, r1257) => {
            let v7 = (*r1256).clone();
            let v8 = (*r1257).clone();
            Ok(List::cons(f185(v7)?, f243(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f244(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f288(Fn10::F289, List::<(Bytes, Bytes)>::nil(), f290(v0)?);
    let v2: List<Bytes> = f291(Fn11::F292, List::<Bytes>::nil(), v1)?;
    let a1260 = {
        let a1258 = Bytes::lit(&[0x7b]);
        let a1259 = f105(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1258, a1259)
    };
    let a1261 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1260, a1261))
}

pub fn f245(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1262 = v0;
    match m1262 {
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
        Adt14::C7(v9, r1263, r1264) => {
            let v10 = (*r1263).clone();
            let v11 = (*r1264).clone();
            Ok(List::cons((v9, f185(v10)?), f245(v11)?))
        }
    }
}

pub fn f246(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1265 = v0;
    match m1265 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f123(v2, v1)
        }
    }
}

pub fn f247(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f167(f283(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f248(v0: Adt15, v1: Option<u64>) -> bool {
    let m1266 = v1;
    match m1266 {
        None => {
            true
        }
        Some(v2) => {
            f203(v2, match v0 {
                Adt15::C0(_, h1267, _, _) => {
                    h1267
                }
            })
        }
    }
}

pub fn f249(v0: Adt12) -> Adt18 {
    Adt18::C1({
        let a1269 = match v0 {
            Adt12::C0(_, _, h1268, _, _) => {
                h1268
            }
        };
        length_list(a1269)
    })
}

pub fn f250(v0: Bytes) -> Option<Adt19> {
    if f35(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt19::C0)
    } else {
        if f35(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt19::C1)
        } else {
            if f35(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt19::C2)
            } else {
                None::<Adt19>
            }
        }
    }
}

pub fn f251(v0: Bytes, p1274: List<Bytes>, p1275: u64) -> R<Option<u64>> {
    let m1273 = (p1274, p1275);
    {
        let mut m1273 = m1273;
        loop {
            let step = {
                let (v1, v2) = m1273;
                let m1270 = v1;
                match m1270.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f35(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1271 = v2;
                                let a1272 = 1u64;
                                nat_add(a1271, a1272)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(v0: Option<Adt14>) -> Option<u64> {
    let m1276 = v0;
    match m1276 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1277 = v1;
            match m1277 {
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

pub fn f253(v0: Bytes, v1: Bytes) -> bool {
    let m1282 = {
        let a1279 = v0;
        let a1280 = 0u64;
        let a1281 = {
            let a1278 = v1.clone();
            length_bytes(a1278)
        };
        slice_bytes(a1279, a1280, a1281)
    };
    match m1282 {
        None => {
            false
        }
        Some(v2) => {
            let a1283 = v2;
            let a1284 = v1;
            equal(a1283, a1284)
        }
    }
}

pub fn f254(v0: Fn5, p1288: List<Bytes>, p1289: List<(Bytes, ())>) -> List<Bytes> {
    let m1287 = (p1288, p1289);
    {
        let mut m1287 = m1287;
        loop {
            let step = {
                let (v1, v2) = m1287;
                let m1285 = v2;
                match m1285.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1286: Fn5 = v0.clone();
                            c1286.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1287 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1290, _) = v1;
        h1290
    }, v0)
}

pub fn f256(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f293(Fn12::F294, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f257(v0: Fn6, p1294: List<List<(Bytes, ())>>, p1295: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1293 = (p1294, p1295);
    {
        let mut m1293 = m1293;
        loop {
            let step = {
                let (v1, v2) = m1293;
                let m1291 = v2;
                match m1291.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1292: Fn6 = v0.clone();
                            c1292.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1293 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f258(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f259(v0: Fn7, p1300: u64, p1301: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1299 = (p1300, p1301);
    {
        let mut m1299 = m1299;
        loop {
            let step = {
                let (v1, v2) = m1299;
                let m1296 = v1;
                if m1296 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1296 - 1;
                    let m1298 = {
                        let c1297: Fn7 = v0.clone();
                        c1297.apply(v2.clone())?
                    };
                    match m1298 {
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
                Ok(next) => { m1299 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f295(v0)
}

pub fn f261(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f296(Fn13::F297, List::<(Bytes, ())>::nil(), v0)
}

pub fn f262(v0: Fn8, p1305: List<(Bytes, ())>, p1306: List<Bytes>) -> List<(Bytes, ())> {
    let m1304 = (p1305, p1306);
    {
        let mut m1304 = m1304;
        loop {
            let step = {
                let (v1, v2) = m1304;
                let m1302 = v2;
                match m1302.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1303: Fn8 = v0.clone();
                            c1303.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f264(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f107(f66(v0.clone(), v1.clone()))?;
    let m1307 = f125(v2.clone())?;
    match m1307 {
        None => {
            f299(v0, v1, v2)
        }
        Some(v3) => {
            f298(v0, v1, v3)
        }
    }
}

pub fn f265(v0: List<Bytes>) -> List<Bytes> {
    let m1308 = v0;
    match m1308.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1310 = {
                    let a1309 = v1.clone();
                    length_bytes(a1309)
                };
                let a1311 = 0u64;
                if nat_eq(a1310, a1311) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f265(v2))
        }
    }
}

pub fn f266(v0: u32) -> Bytes {
    let a1314 = {
        let a1312 = v0;
        let a1313 = 8u32;
        fixed_u32::bit_and(a1312, a1313)
    };
    let a1315 = 0u32;
    if equal(a1314, a1315) {
        let a1318 = {
            let a1316 = v0;
            let a1317 = 4u32;
            fixed_u32::bit_and(a1316, a1317)
        };
        let a1319 = 0u32;
        if equal(a1318, a1319) {
            let a1322 = {
                let a1320 = v0;
                let a1321 = 2u32;
                fixed_u32::bit_and(a1320, a1321)
            };
            let a1323 = 0u32;
            if equal(a1322, a1323) {
                let a1326 = {
                    let a1324 = v0;
                    let a1325 = 1u32;
                    fixed_u32::bit_and(a1324, a1325)
                };
                let a1327 = 0u32;
                if equal(a1326, a1327) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1330 = {
                    let a1328 = v0;
                    let a1329 = 1u32;
                    fixed_u32::bit_and(a1328, a1329)
                };
                let a1331 = 0u32;
                if equal(a1330, a1331) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1334 = {
                let a1332 = v0;
                let a1333 = 2u32;
                fixed_u32::bit_and(a1332, a1333)
            };
            let a1335 = 0u32;
            if equal(a1334, a1335) {
                let a1338 = {
                    let a1336 = v0;
                    let a1337 = 1u32;
                    fixed_u32::bit_and(a1336, a1337)
                };
                let a1339 = 0u32;
                if equal(a1338, a1339) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1342 = {
                    let a1340 = v0;
                    let a1341 = 1u32;
                    fixed_u32::bit_and(a1340, a1341)
                };
                let a1343 = 0u32;
                if equal(a1342, a1343) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1346 = {
            let a1344 = v0;
            let a1345 = 4u32;
            fixed_u32::bit_and(a1344, a1345)
        };
        let a1347 = 0u32;
        if equal(a1346, a1347) {
            let a1350 = {
                let a1348 = v0;
                let a1349 = 2u32;
                fixed_u32::bit_and(a1348, a1349)
            };
            let a1351 = 0u32;
            if equal(a1350, a1351) {
                let a1354 = {
                    let a1352 = v0;
                    let a1353 = 1u32;
                    fixed_u32::bit_and(a1352, a1353)
                };
                let a1355 = 0u32;
                if equal(a1354, a1355) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1358 = {
                    let a1356 = v0;
                    let a1357 = 1u32;
                    fixed_u32::bit_and(a1356, a1357)
                };
                let a1359 = 0u32;
                if equal(a1358, a1359) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1362 = {
                let a1360 = v0;
                let a1361 = 2u32;
                fixed_u32::bit_and(a1360, a1361)
            };
            let a1363 = 0u32;
            if equal(a1362, a1363) {
                let a1366 = {
                    let a1364 = v0;
                    let a1365 = 1u32;
                    fixed_u32::bit_and(a1364, a1365)
                };
                let a1367 = 0u32;
                if equal(a1366, a1367) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1370 = {
                    let a1368 = v0;
                    let a1369 = 1u32;
                    fixed_u32::bit_and(a1368, a1369)
                };
                let a1371 = 0u32;
                if equal(a1370, a1371) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f267(v0: u32, v1: u32) -> u32 {
    let m1374 = {
        let a1372 = v0;
        let a1373 = v1;
        fixed_u32::shift_right(a1372, a1373)
    };
    match m1374 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f268(v0: Adt20, v1: Adt22) -> R<Adt20> {
    let v2: Adt23 = f300(Fn14::F301, 64u64, Adt23::C0(match v0.clone() {
        Adt20::C0(h1375, _, _, _, _, _, _, _) => {
            h1375
        }
    }, match v0.clone() {
        Adt20::C0(_, h1376, _, _, _, _, _, _) => {
            h1376
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h1377, _, _, _, _, _) => {
            h1377
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1378, _, _, _, _) => {
            h1378
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, h1379, _, _, _) => {
            h1379
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1380, _, _) => {
            h1380
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1381, _) => {
            h1381
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, _, h1382) => {
            h1382
        }
    }, v1, 0u64))?;
    Ok(Adt20::C0(f302(match v0.clone() {
        Adt20::C0(h1383, _, _, _, _, _, _, _) => {
            h1383
        }
    }, match v2.clone() {
        Adt23::C0(h1384, _, _, _, _, _, _, _, _, _) => {
            h1384
        }
    }), f302(match v0.clone() {
        Adt20::C0(_, h1385, _, _, _, _, _, _) => {
            h1385
        }
    }, match v2.clone() {
        Adt23::C0(_, h1386, _, _, _, _, _, _, _, _) => {
            h1386
        }
    }), f302(match v0.clone() {
        Adt20::C0(_, _, h1387, _, _, _, _, _) => {
            h1387
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1388, _, _, _, _, _, _, _) => {
            h1388
        }
    }), f302(match v0.clone() {
        Adt20::C0(_, _, _, h1389, _, _, _, _) => {
            h1389
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1390, _, _, _, _, _, _) => {
            h1390
        }
    }), f302(match v0.clone() {
        Adt20::C0(_, _, _, _, h1391, _, _, _) => {
            h1391
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1392, _, _, _, _, _) => {
            h1392
        }
    }), f302(match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1393, _, _) => {
            h1393
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1394, _, _, _, _) => {
            h1394
        }
    }), f302(match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1395, _) => {
            h1395
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1396, _, _, _) => {
            h1396
        }
    }), f302(match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h1397) => {
            h1397
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1398, _, _) => {
            h1398
        }
    })))
}

pub fn f269(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f303(v0.clone(), v1, v2, v3)?, f303(v0.clone(), v1, v2, {
        let a1399 = v3;
        let a1400 = 4u64;
        nat_add(a1399, a1400)?
    })?, f303(v0.clone(), v1, v2, {
        let a1401 = v3;
        let a1402 = 8u64;
        nat_add(a1401, a1402)?
    })?, f303(v0.clone(), v1, v2, {
        let a1403 = v3;
        let a1404 = 12u64;
        nat_add(a1403, a1404)?
    })?, f303(v0.clone(), v1, v2, {
        let a1405 = v3;
        let a1406 = 16u64;
        nat_add(a1405, a1406)?
    })?, f303(v0.clone(), v1, v2, {
        let a1407 = v3;
        let a1408 = 20u64;
        nat_add(a1407, a1408)?
    })?, f303(v0.clone(), v1, v2, {
        let a1409 = v3;
        let a1410 = 24u64;
        nat_add(a1409, a1410)?
    })?, f303(v0.clone(), v1, v2, {
        let a1411 = v3;
        let a1412 = 28u64;
        nat_add(a1411, a1412)?
    })?, f303(v0.clone(), v1, v2, {
        let a1413 = v3;
        let a1414 = 32u64;
        nat_add(a1413, a1414)?
    })?, f303(v0.clone(), v1, v2, {
        let a1415 = v3;
        let a1416 = 36u64;
        nat_add(a1415, a1416)?
    })?, f303(v0.clone(), v1, v2, {
        let a1417 = v3;
        let a1418 = 40u64;
        nat_add(a1417, a1418)?
    })?, f303(v0.clone(), v1, v2, {
        let a1419 = v3;
        let a1420 = 44u64;
        nat_add(a1419, a1420)?
    })?, f303(v0.clone(), v1, v2, {
        let a1421 = v3;
        let a1422 = 48u64;
        nat_add(a1421, a1422)?
    })?, f303(v0.clone(), v1, v2, {
        let a1423 = v3;
        let a1424 = 52u64;
        nat_add(a1423, a1424)?
    })?, f303(v0.clone(), v1, v2, {
        let a1425 = v3;
        let a1426 = 56u64;
        nat_add(a1425, a1426)?
    })?, f303(v0, v1, v2, {
        let a1427 = v3;
        let a1428 = 60u64;
        nat_add(a1427, a1428)?
    })?))
}

pub fn f270(v0: Bytes, v1: u64) -> Option<u64> {
    let a1429 = 0u64;
    let a1430 = v1;
    if nat_lt(a1429, a1430) {
        if f304(v0, {
            let a1431 = v1;
            let a1432 = 1u64;
            nat_sub(a1431, a1432)
        }) {
            Some({
                let a1433 = v1;
                let a1434 = 1u64;
                nat_sub(a1433, a1434)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f271(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1435 = v2;
    let a1436 = v0;
    if nat_lt(a1435, a1436) {
        if f304(v1, v2) {
            Ok(Some({
                let a1437 = v2;
                let a1438 = 1u64;
                nat_add(a1437, a1438)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f272(v0: Bytes, p1518: u64, p1519: u64, p1520: bool, p1521: bool, p1522: Bytes) -> R<Bytes> {
    let m1517 = (p1518, (p1519, (p1520, (p1521, p1522))));
    {
        let mut m1517 = m1517;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1517;
                let m1439 = v2;
                if m1439 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1439 - 1;
                    let m1442 = {
                        let a1440 = v0.clone();
                        let a1441 = v1;
                        index_bytes(a1440, a1441)
                    };
                    match m1442 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1487 = {
                                    let a1463 = {
                                        let a1451 = {
                                            let a1445 = {
                                                let a1443 = v8;
                                                let a1444 = 128u8;
                                                fixed_u8::bit_and(a1443, a1444)
                                            };
                                            let a1446 = 128u8;
                                            if equal(a1445, a1446) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1452 = {
                                            let a1449 = {
                                                let a1447 = v8;
                                                let a1448 = 64u8;
                                                fixed_u8::bit_and(a1447, a1448)
                                            };
                                            let a1450 = 64u8;
                                            if equal(a1449, a1450) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1451, a1452)?
                                    };
                                    let a1464 = {
                                        let a1461 = {
                                            let a1455 = {
                                                let a1453 = v8;
                                                let a1454 = 32u8;
                                                fixed_u8::bit_and(a1453, a1454)
                                            };
                                            let a1456 = 32u8;
                                            if equal(a1455, a1456) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1462 = {
                                            let a1459 = {
                                                let a1457 = v8;
                                                let a1458 = 16u8;
                                                fixed_u8::bit_and(a1457, a1458)
                                            };
                                            let a1460 = 16u8;
                                            if equal(a1459, a1460) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1461, a1462)?
                                    };
                                    nat_add(a1463, a1464)?
                                };
                                let a1488 = {
                                    let a1485 = {
                                        let a1473 = {
                                            let a1467 = {
                                                let a1465 = v8;
                                                let a1466 = 8u8;
                                                fixed_u8::bit_and(a1465, a1466)
                                            };
                                            let a1468 = 8u8;
                                            if equal(a1467, a1468) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1474 = {
                                            let a1471 = {
                                                let a1469 = v8;
                                                let a1470 = 4u8;
                                                fixed_u8::bit_and(a1469, a1470)
                                            };
                                            let a1472 = 4u8;
                                            if equal(a1471, a1472) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1473, a1474)?
                                    };
                                    let a1486 = {
                                        let a1483 = {
                                            let a1477 = {
                                                let a1475 = v8;
                                                let a1476 = 2u8;
                                                fixed_u8::bit_and(a1475, a1476)
                                            };
                                            let a1478 = 2u8;
                                            if equal(a1477, a1478) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1484 = {
                                            let a1481 = {
                                                let a1479 = v8;
                                                let a1480 = 1u8;
                                                fixed_u8::bit_and(a1479, a1480)
                                            };
                                            let a1482 = 1u8;
                                            if equal(a1481, a1482) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1483, a1484)?
                                    };
                                    nat_add(a1485, a1486)?
                                };
                                nat_add(a1487, a1488)?
                            };
                            let v10: Bytes = f31(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1489 = v1;
                                    let a1490 = 1u64;
                                    nat_add(a1489, a1490)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1491 = v9;
                                    let a1492 = 34u64;
                                    !nat_eq(a1491, a1492)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1493 = v9;
                                    let a1494 = 92u64;
                                    nat_eq(a1493, a1494)
                                }, {
                                    let a1495 = v5;
                                    let a1496 = v10;
                                    append_bytes(a1495, a1496)
                                })))))
                            } else {
                                let a1497 = v9;
                                let a1498 = 32u64;
                                if if if if nat_eq(a1497, a1498) {
                                    true
                                } else {
                                    let a1499 = v9;
                                    let a1500 = 9u64;
                                    nat_eq(a1499, a1500)
                                } {
                                    true
                                } else {
                                    let a1501 = v9;
                                    let a1502 = 10u64;
                                    nat_eq(a1501, a1502)
                                } {
                                    true
                                } else {
                                    let a1503 = v9;
                                    let a1504 = 13u64;
                                    nat_eq(a1503, a1504)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1505 = v1;
                                        let a1506 = 1u64;
                                        nat_add(a1505, a1506)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f35(f31(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1507 = v1;
                                            let a1508 = 4u64;
                                            nat_add(a1507, a1508)?
                                        }, (v6, (false, (false, {
                                            let a1509 = v5;
                                            let a1510 = f8(Str::lit("[]"));
                                            append_bytes(a1509, a1510)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1511 = v1;
                                            let a1512 = 1u64;
                                            nat_add(a1511, a1512)?
                                        }, (v6, ({
                                            let a1513 = v9;
                                            let a1514 = 34u64;
                                            nat_eq(a1513, a1514)
                                        }, (false, {
                                            let a1515 = v5;
                                            let a1516 = v10;
                                            append_bytes(a1515, a1516)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Bytes, v1: u64) -> R<u64> {
    let m1525 = {
        let a1523 = v0;
        let a1524 = v1;
        index_bytes(a1523, a1524)
    };
    match m1525 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1570 = {
                let a1546 = {
                    let a1534 = {
                        let a1528 = {
                            let a1526 = v3;
                            let a1527 = 128u8;
                            fixed_u8::bit_and(a1526, a1527)
                        };
                        let a1529 = 128u8;
                        if equal(a1528, a1529) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1535 = {
                        let a1532 = {
                            let a1530 = v3;
                            let a1531 = 64u8;
                            fixed_u8::bit_and(a1530, a1531)
                        };
                        let a1533 = 64u8;
                        if equal(a1532, a1533) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1534, a1535)?
                };
                let a1547 = {
                    let a1544 = {
                        let a1538 = {
                            let a1536 = v3;
                            let a1537 = 32u8;
                            fixed_u8::bit_and(a1536, a1537)
                        };
                        let a1539 = 32u8;
                        if equal(a1538, a1539) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1545 = {
                        let a1542 = {
                            let a1540 = v3;
                            let a1541 = 16u8;
                            fixed_u8::bit_and(a1540, a1541)
                        };
                        let a1543 = 16u8;
                        if equal(a1542, a1543) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1544, a1545)?
                };
                nat_add(a1546, a1547)?
            };
            let a1571 = {
                let a1568 = {
                    let a1556 = {
                        let a1550 = {
                            let a1548 = v3;
                            let a1549 = 8u8;
                            fixed_u8::bit_and(a1548, a1549)
                        };
                        let a1551 = 8u8;
                        if equal(a1550, a1551) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1557 = {
                        let a1554 = {
                            let a1552 = v3;
                            let a1553 = 4u8;
                            fixed_u8::bit_and(a1552, a1553)
                        };
                        let a1555 = 4u8;
                        if equal(a1554, a1555) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1556, a1557)?
                };
                let a1569 = {
                    let a1566 = {
                        let a1560 = {
                            let a1558 = v3;
                            let a1559 = 2u8;
                            fixed_u8::bit_and(a1558, a1559)
                        };
                        let a1561 = 2u8;
                        if equal(a1560, a1561) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1567 = {
                        let a1564 = {
                            let a1562 = v3;
                            let a1563 = 1u8;
                            fixed_u8::bit_and(a1562, a1563)
                        };
                        let a1565 = 1u8;
                        if equal(a1564, a1565) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1566, a1567)?
                };
                nat_add(a1568, a1569)?
            };
            nat_add(a1570, a1571)
        }
    }
}

pub fn f274(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f305(Fn15::F306(v0.clone()), {
        let a1573 = {
            let a1572 = v0;
            length_bytes(a1572)
        };
        let a1574 = 1u64;
        nat_add(a1573, a1574)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1578 = {
        let (_, h1575) = {
            let (_, h1576) = {
                let (h1577, _) = v2.clone();
                h1577
            };
            h1576
        };
        h1575
    };
    let a1579 = 1u64;
    if nat_eq(a1578, a1579) {
        Ok(Some(({
            let (h1580, _) = {
                let (h1581, _) = v2.clone();
                h1581
            };
            h1580
        }, {
            let (h1582, _) = {
                let (_, h1583) = {
                    let (h1584, _) = v2;
                    h1584
                };
                h1583
            };
            h1582
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f275(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1585 = f273(v0.clone(), v1)?;
        let a1586 = 45u64;
        nat_eq(a1585, a1586)
    };
    let v3: u64 = if v2 {
        let a1587 = v1;
        let a1588 = 1u64;
        nat_add(a1587, a1588)?
    } else {
        v1
    };
    let v4: u64 = f307(v0.clone(), v3)?;
    let a1591 = 19u64;
    let a1592 = {
        let a1589 = v4;
        let a1590 = v3;
        nat_sub(a1589, a1590)
    };
    if nat_lt(a1591, a1592) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1595 = f308(f31(v0, v3, {
            let a1593 = v4;
            let a1594 = v3;
            nat_sub(a1593, a1594)
        }))?;
        match m1595 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f309(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f276(v0: Bytes) -> Bytes {
    if f108(v0.clone(), Bytes::lit(&[0x0d])) {
        f31(v0.clone(), 0u64, {
            let a1597 = {
                let a1596 = v0;
                length_bytes(a1596)
            };
            let a1598 = 1u64;
            nat_sub(a1597, a1598)
        })
    } else {
        v0
    }
}

pub fn f277(v0: u64) -> bool {
    let a1599 = v0;
    let a1600 = 95u64;
    if nat_eq(a1599, a1600) {
        true
    } else {
        let a1601 = v0;
        let a1602 = 45u64;
        if nat_eq(a1601, a1602) {
            true
        } else {
            let a1603 = v0;
            let a1604 = 32u64;
            if nat_eq(a1603, a1604) {
                true
            } else {
                let a1605 = v0;
                let a1606 = 46u64;
                if nat_eq(a1605, a1606) {
                    true
                } else {
                    let a1607 = v0;
                    let a1608 = 47u64;
                    nat_eq(a1607, a1608)
                }
            }
        }
    }
}

pub fn f278(v0: u64) -> bool {
    let a1609 = 97u64;
    let a1610 = v0;
    if nat_le(a1609, a1610) {
        let a1611 = v0;
        let a1612 = 122u64;
        nat_le(a1611, a1612)
    } else {
        false
    }
}

pub fn f279(v0: u64) -> bool {
    let a1613 = 65u64;
    let a1614 = v0;
    if nat_le(a1613, a1614) {
        let a1615 = v0;
        let a1616 = 90u64;
        nat_le(a1615, a1616)
    } else {
        false
    }
}

pub fn f280(v0: u64) -> bool {
    let a1617 = 48u64;
    let a1618 = v0;
    if nat_le(a1617, a1618) {
        let a1619 = v0;
        let a1620 = 57u64;
        nat_le(a1619, a1620)
    } else {
        false
    }
}

pub fn f281(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m1621 = f120(v1.clone(), f8(Str::lit("enum")));
    match m1621 {
        None => {
            let m1623 = f313(v0.clone(), f238(f120(v1.clone(), f8(Str::lit("properties")))), f70(f124(f120(v1, f8(Str::lit("required"))))))?;
            match m1623 {
                Ok(v6) => {
                    Ok(f312(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1625 = {
                            let (h1624, _) = v6.clone();
                            h1624
                        };
                        match m1625.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f167({
                                    let (h1626, _) = v6;
                                    h1626
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1627) = v6;
                        h1627
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt21>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1622 = f310(f70(f186(v2)))?;
            match m1622 {
                Ok(v4) => {
                    Ok(f312(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f167(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt21>::nil()))
                }
                Err(v3) => {
                    Ok(f311(v3, v0))
                }
            }
        }
    }
}

pub fn f282(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1628 = v0;
    match m1628 {
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
        Adt14::C4(r1629) => {
            let v4 = (*r1629).clone();
            f282(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1630, r1631) => {
            let v2 = (*r1630).clone();
            let v3 = (*r1631).clone();
            List::cons((v1, v2), f282(v3))
        }
    }
}

pub fn f283(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1632 = v0;
    match m1632.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f161(v1)?, f283(v2)?))
        }
    }
}

pub fn f284(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1633 = v0;
    match m1633.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f284(v3, false))
            } else {
                f284(v3, true)
            }
        }
    }
}

pub fn f285(v0: u64) -> R<Bytes> {
    f314(f315(), 10u64, v0)
}

pub fn f286(v0: Fn9, p1638: u64, p1639: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1637 = (p1638, p1639);
    {
        let mut m1637 = m1637;
        loop {
            let step = {
                let (v1, v2) = m1637;
                let m1634 = v1;
                if m1634 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1634 - 1;
                    let m1636 = {
                        let c1635: Fn9 = v0.clone();
                        c1635.apply(v2.clone())?
                    };
                    match m1636 {
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
                Ok(next) => { m1637 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f316(v0, v1)
}

pub fn f288(v0: Fn10, p1643: List<(Bytes, Bytes)>, p1644: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1642 = (p1643, p1644);
    {
        let mut m1642 = m1642;
        loop {
            let step = {
                let (v1, v2) = m1642;
                let m1640 = v2;
                match m1640.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1641: Fn10 = v0.clone();
                            c1641.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f317(v0, v1)
}

pub fn f290(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f318(v0)
}

pub fn f291(v0: Fn11, p1648: List<Bytes>, p1649: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1647 = (p1648, p1649);
    {
        let mut m1647 = m1647;
        loop {
            let step = {
                let (v1, v2) = m1647;
                let m1645 = v2;
                match m1645.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1646: Fn11 = v0.clone();
                            c1646.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f319(v1)?, v0))
}

pub fn f293(v0: Fn12, p1653: List<List<(Bytes, ())>>, p1654: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1652 = (p1653, p1654);
    {
        let mut m1652 = m1652;
        loop {
            let step = {
                let (v1, v2) = m1652;
                let m1650 = v2;
                match m1650.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1651: Fn12 = v0.clone();
                            c1651.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f295(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1656 = {
        let a1655 = v0.clone();
        length_list(a1655)
    };
    let a1657 = 1u64;
    if nat_le(a1656, a1657) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f320(v0)?))
    }
}

pub fn f296(v0: Fn13, p1661: List<(Bytes, ())>, p1662: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1660 = (p1661, p1662);
    {
        let mut m1660 = m1660;
        loop {
            let step = {
                let (v1, v2) = m1660;
                let m1658 = v2;
                match m1658.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1659: Fn13 = v0.clone();
                            c1659.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f298(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f68(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f321(v3.clone(), f68(f69(v0, f82())?, f8(Str::lit("at"))));
    let a1664 = {
        let a1663 = v4.clone();
        length_list(a1663)
    };
    let a1665 = 0u64;
    if nat_eq(a1664, a1665) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1668 = {
            let a1666 = v4;
            length_list(a1666)
        };
        let a1669 = {
            let a1667 = v3;
            length_list(a1667)
        };
        if nat_eq(a1668, a1669) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f322(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f299(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f176(f115(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f176(f115(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f60(f68(v3.clone(), f8(Str::lit("id"))), f68(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f68(f69(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f321(v5.clone(), f60(v6.clone(), f68(f69(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1671 = {
        let a1670 = v7.clone();
        length_list(a1670)
    };
    let a1672 = 0u64;
    if nat_eq(a1671, a1672) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f71({
            let a1675 = {
                let a1673 = v7;
                length_list(a1673)
            };
            let a1676 = {
                let a1674 = v5;
                length_list(a1674)
            };
            nat_eq(a1675, a1676)
        }) {
            Ok((false, f322(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f74(f60(f321(f323(v4.clone()), f65(f66(v0, f8(Str::lit("model/authorities.toml"))))?), f321(f324(v4), f60(v6, f68(v3, f8(Str::lit("id")))))))?;
            let a1678 = {
                let a1677 = v8.clone();
                length_list(a1677)
            };
            let a1679 = 0u64;
            if nat_eq(a1678, a1679) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f322(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f105(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f300(v0: Fn14, p1683: u64, p1684: Adt23) -> R<Adt23> {
    let m1682 = (p1683, p1684);
    {
        let mut m1682 = m1682;
        loop {
            let step = {
                let (v1, v2) = m1682;
                let m1680 = v1;
                if m1680 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1680 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1681: Fn14 = v0.clone();
                        c1681.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: Adt23) -> R<Adt23> {
    f325(v0)
}

pub fn f302(v0: u32, v1: u32) -> u32 {
    let m1687 = {
        let a1685 = f326(v0);
        let a1686 = f326(v1);
        fixed_u64::checked_add(a1685, a1686)
    };
    match m1687 {
        None => {
            0u32
        }
        Some(v2) => {
            f327(v2)
        }
    }
}

pub fn f303(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1690 = {
        let a1688 = v3;
        let a1689 = 8u64;
        nat_add(a1688, a1689)?
    };
    let a1691 = v2;
    if nat_eq(a1690, a1691) {
        Ok(f328({
            let a1694 = {
                let a1692 = v1;
                let a1693 = 8u64;
                nat_mul(a1692, a1693)?
            };
            let a1695 = 4294967296u64;
            let a1696 = 0u64;
            nat_quot(a1694, a1695, a1696)
        }))
    } else {
        let a1699 = {
            let a1697 = v3;
            let a1698 = 4u64;
            nat_add(a1697, a1698)?
        };
        let a1700 = v2;
        if nat_eq(a1699, a1700) {
            Ok(f328({
                let a1703 = {
                    let a1701 = v1;
                    let a1702 = 8u64;
                    nat_mul(a1701, a1702)?
                };
                let a1704 = 4294967296u64;
                let a1705 = 0u64;
                nat_rem(a1703, a1704, a1705)
            }))
        } else {
            Ok(f329(f330(v0.clone(), v1, v3), f330(v0.clone(), v1, {
                let a1706 = v3;
                let a1707 = 1u64;
                nat_add(a1706, a1707)?
            }), f330(v0.clone(), v1, {
                let a1708 = v3;
                let a1709 = 2u64;
                nat_add(a1708, a1709)?
            }), f330(v0, v1, {
                let a1710 = v3;
                let a1711 = 3u64;
                nat_add(a1710, a1711)?
            })))
        }
    }
}

pub fn f304(v0: Bytes, v1: u64) -> bool {
    let m1712 = f191(v0, v1);
    match m1712 {
        None => {
            false
        }
        Some(v2) => {
            f331(v2)
        }
    }
}

pub fn f305(v0: Fn15, p1717: u64, p1718: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1716 = (p1717, p1718);
    {
        let mut m1716 = m1716;
        loop {
            let step = {
                let (v1, v2) = m1716;
                let m1713 = v1;
                if m1713 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1713 - 1;
                    let m1715 = {
                        let c1714: Fn15 = v0.clone();
                        c1714.apply(v2.clone())?
                    };
                    match m1715 {
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
                Ok(next) => { m1716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f332(v0, v1)
}

pub fn f307(v0: Bytes, v1: u64) -> R<u64> {
    let (h1719, _) = f193(Fn3::F333(v0.clone()), {
        let a1721 = {
            let a1720 = v0;
            length_bytes(a1720)
        };
        let a1722 = 1u64;
        nat_add(a1721, a1722)?
    }, v1)?;
    Ok(h1719)
}

pub fn f308(v0: Bytes) -> R<Option<u64>> {
    if f334(v0.clone())? {
        Ok(Some({
            let (_, h1723) = {
                let (h1724, _) = f335(Fn16::F336(v0.clone()), {
                    let a1725 = v0;
                    length_bytes(a1725)
                }, (0u64, 0u64))?;
                h1724
            };
            h1723
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f309(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1726 = 0u64;
        let a1727 = v1;
        if nat_lt(a1726, a1727) {
            let a1728 = v1;
            let a1729 = 9223372036854775808u64;
            nat_le(a1728, a1729)
        } else {
            false
        }
    } else {
        let a1730 = v1;
        let a1731 = 9223372036854775807u64;
        nat_le(a1730, a1731)
    }
}

pub fn f310(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1732 = v0;
    match m1732.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f337(v1.clone())?;
            if f338(v3.clone())? {
                let m1733 = f310(v2)?;
                match m1733 {
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

pub fn f311(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt21>), Bytes> {
    Err::<(Bytes, List<Adt21>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f312(v0: Bytes, v1: List<Adt21>) -> Result<(Bytes, List<Adt21>), Bytes> {
    Ok::<(Bytes, List<Adt21>), Bytes>((v0, v1))
}

pub fn f313(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt21>), Bytes>> {
    let m1734 = v1;
    match m1734.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt21>), Bytes>((List::<Bytes>::nil(), List::<Adt21>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f337({
                let (h1735, _) = v3.clone();
                h1735
            })?;
            if f338(v5.clone())? {
                let m1740 = f339({
                    let (_, h1736) = v3.clone();
                    h1736
                }, {
                    let a1738 = v0.clone();
                    let a1739 = f180({
                        let (h1737, _) = v3.clone();
                        h1737
                    })?;
                    append_bytes(a1738, a1739)
                })?;
                match m1740 {
                    Ok(v7) => {
                        let v8: Bytes = if f80({
                            let (h1741, _) = v3;
                            h1741
                        }, v2.clone()) {
                            let (h1742, _) = v7;
                            h1742
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1743, _) = v7;
                                h1743
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1744 = f313(v0, v4, v2)?;
                        match m1744 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt21>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1745, _) = v10;
                                    h1745
                                }), f236({
                                    let (_, h1746) = v7;
                                    h1746
                                }, {
                                    let (_, h1747) = v10;
                                    h1747
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
                    let (h1748, _) = v3;
                    h1748
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f314(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1749 = v2;
    let a1750 = 0u64;
    if nat_eq(a1749, a1750) {
        Ok(f340(v0, 0u64))
    } else {
        let (_, h1751) = {
            let (h1752, _) = f286(Fn9::F341(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1752
        };
        Ok(h1751)
    }
}

pub fn f315() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f316(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1756 = {
        let a1754 = v0.clone();
        let a1755 = {
            let (h1753, _) = v1.clone();
            h1753
        };
        index_bytes(a1754, a1755)
    };
    match m1756 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1758 = {
                    let (h1757, _) = v1.clone();
                    h1757
                };
                let a1759 = 1u64;
                nat_add(a1758, a1759)?
            }, {
                let a1762 = {
                    let (_, h1760) = v1;
                    h1760
                };
                let a1763 = f342(v0, {
                    let (h1761, _) = v1;
                    h1761
                }, v2)?;
                append_bytes(a1762, a1763)
            })))
        }
    }
}

pub fn f317(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1764 = v0.clone();
    match m1764.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1767 = {
                let (h1765, _) = v2;
                h1765
            };
            let a1768 = {
                let (h1766, _) = v1.clone();
                h1766
            };
            if equal(a1767, a1768) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f318(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f343(f344(Fn17::F345, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1773 = {
        let (h1769, _) = f346(Fn18::F347, {
            let a1771 = {
                let a1770 = v0;
                length_list(a1770)
            };
            let a1772 = 1u64;
            nat_add(a1771, a1772)?
        }, v1)?;
        h1769
    };
    match m1773.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f319(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1778 = {
        let a1775 = f241({
            let (h1774, _) = v0;
            h1774
        })?;
        let a1776 = Bytes::lit(&[0x3a]);
        append_bytes(a1775, a1776)
    };
    let a1779 = {
        let (_, h1777) = v0;
        h1777
    };
    Ok(append_bytes(a1778, a1779))
}

pub fn f320(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f256({
        let (_, h1780) = {
            let (h1781, _) = f348(Fn19::F349, {
                let a1783 = {
                    let a1782 = v0.clone();
                    length_list(a1782)
                };
                let a1784 = 1u64;
                nat_add(a1783, a1784)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1781
        };
        h1780
    }))
}

pub fn f321(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1785 = v0;
    match m1785.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f80(v2.clone(), v1.clone()) {
                f321(v3, v1)
            } else {
                List::cons(v2, f321(v3, v1))
            }
        }
    }
}

pub fn f322(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f323(v0: List<Adt14>) -> List<Bytes> {
    let m1786 = v0;
    match m1786.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1787 = f120(v1, f8(Str::lit("std")));
            match m1787 {
                None => {
                    f323(v2)
                }
                Some(v3) => {
                    List::cons(f123(v3, f8(Str::lit("authority"))), f323(v2))
                }
            }
        }
    }
}

pub fn f324(v0: List<Adt14>) -> List<Bytes> {
    let m1788 = v0;
    match m1788.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1789 = f128(f120(v1, f8(Str::lit("req"))));
            match m1789 {
                None => {
                    f324(v2)
                }
                Some(v3) => {
                    List::cons(v3, f324(v2))
                }
            }
        }
    }
}

pub fn f325(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f350(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1790) => {
            h1790
        }
    });
    let v2: u32 = f302(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1791, _, _) => {
            h1791
        }
    }, f302(f351(match v0.clone() {
        Adt23::C0(_, _, _, _, h1792, _, _, _, _, _) => {
            h1792
        }
    }), f302(f352(match v0.clone() {
        Adt23::C0(_, _, _, _, h1793, _, _, _, _, _) => {
            h1793
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1794, _, _, _, _) => {
            h1794
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1795, _, _, _) => {
            h1795
        }
    }), f302(v1, {
        let m1798 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1797, _) => {
                h1797
            }
        };
        match m1798 {
            Adt22::C0(h1796, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1796
            }
        }
    }))));
    let v3: u32 = f302(f353(match v0.clone() {
        Adt23::C0(h1799, _, _, _, _, _, _, _, _, _) => {
            h1799
        }
    }), f354(match v0.clone() {
        Adt23::C0(h1800, _, _, _, _, _, _, _, _, _) => {
            h1800
        }
    }, match v0.clone() {
        Adt23::C0(_, h1801, _, _, _, _, _, _, _, _) => {
            h1801
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1802, _, _, _, _, _, _, _) => {
            h1802
        }
    }));
    Ok(Adt23::C0(f302(v2, v3), match v0.clone() {
        Adt23::C0(h1803, _, _, _, _, _, _, _, _, _) => {
            h1803
        }
    }, match v0.clone() {
        Adt23::C0(_, h1804, _, _, _, _, _, _, _, _) => {
            h1804
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1805, _, _, _, _, _, _, _) => {
            h1805
        }
    }, f302(match v0.clone() {
        Adt23::C0(_, _, _, h1806, _, _, _, _, _, _) => {
            h1806
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1807, _, _, _, _, _) => {
            h1807
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1808, _, _, _, _) => {
            h1808
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1809, _, _, _) => {
            h1809
        }
    }, f355(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1810, _) => {
            h1810
        }
    }), {
        let a1812 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1811) => {
                h1811
            }
        };
        let a1813 = 1u64;
        nat_add(a1812, a1813)?
    }))
}

pub fn f326(v0: u32) -> u64 {
    let m1815 = {
        let a1814 = v0;
        fixed_u64::convert(i128::from(a1814))
    };
    match m1815 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f327(v0: u64) -> u32 {
    let m1819 = {
        let a1818 = {
            let a1816 = v0;
            let a1817 = 4294967295u64;
            fixed_u64::bit_and(a1816, a1817)
        };
        fixed_u32::convert(i128::from(a1818))
    };
    match m1819 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f328(v0: u64) -> u32 {
    let (_, h1820) = {
        let (_, h1821) = f356(Fn20::F357, 32u64, (v0, (1u32, 0u32)));
        h1821
    };
    h1820
}

pub fn f329(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1826 = f358(v0, 24u32);
    let a1827 = {
        let a1824 = f358(v1, 16u32);
        let a1825 = {
            let a1822 = f358(v2, 8u32);
            let a1823 = v3;
            fixed_u32::bit_or(a1822, a1823)
        };
        fixed_u32::bit_or(a1824, a1825)
    };
    fixed_u32::bit_or(a1826, a1827)
}

pub fn f330(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1828 = v2;
    let a1829 = v1;
    if nat_lt(a1828, a1829) {
        let m1832 = {
            let a1830 = v0;
            let a1831 = v2;
            index_bytes(a1830, a1831)
        };
        match m1832 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1834 = {
                    let a1833 = v3;
                    fixed_u32::convert(i128::from(a1833))
                };
                match m1834 {
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
        let a1835 = v2;
        let a1836 = v1;
        if nat_eq(a1835, a1836) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f331(v0: u8) -> bool {
    let a1837 = v0;
    let a1838 = 32u8;
    if if if equal(a1837, a1838) {
        true
    } else {
        let a1839 = v0;
        let a1840 = 9u8;
        equal(a1839, a1840)
    } {
        true
    } else {
        let a1841 = v0;
        let a1842 = 10u8;
        equal(a1841, a1842)
    } {
        true
    } else {
        let a1843 = v0;
        let a1844 = 12u8;
        if equal(a1843, a1844) {
            true
        } else {
            let a1845 = v0;
            let a1846 = 13u8;
            equal(a1845, a1846)
        }
    }
}

pub fn f332(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1847, _) = v1.clone();
        h1847
    };
    let v3: Bytes = {
        let (h1848, _) = {
            let (_, h1849) = v1.clone();
            h1849
        };
        h1848
    };
    let a1852 = {
        let (_, h1850) = {
            let (_, h1851) = v1;
            h1851
        };
        h1850
    };
    let a1853 = 0u64;
    if nat_eq(a1852, a1853) {
        let v4: u64 = f273(v0.clone(), v2)?;
        let a1854 = v4;
        let a1855 = 34u64;
        if nat_eq(a1854, a1855) {
            Ok(Some(({
                let a1856 = v2;
                let a1857 = 1u64;
                nat_add(a1856, a1857)?
            }, (v3, 1u64))))
        } else {
            let a1858 = v4;
            let a1859 = 92u64;
            if nat_eq(a1858, a1859) {
                f359(v0, v2, v3)
            } else {
                let a1860 = v4;
                let a1861 = 32u64;
                if if nat_lt(a1860, a1861) {
                    true
                } else {
                    let a1862 = v4;
                    let a1863 = 256u64;
                    nat_eq(a1862, a1863)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1864 = v2;
                        let a1865 = 1u64;
                        nat_add(a1864, a1865)?
                    }, ({
                        let a1866 = v3;
                        let a1867 = f31(v0, v2, 1u64);
                        append_bytes(a1866, a1867)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f333(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f360(v0, v1)
}

pub fn f334(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1868 = v0.clone();
        length_bytes(a1868)
    };
    let a1869 = 0u64;
    let a1870 = v1;
    if nat_lt(a1869, a1870) {
        let a1871 = f361(v0.clone())?;
        let a1872 = v1;
        if nat_eq(a1871, a1872) {
            let a1873 = 1u64;
            let a1874 = v1;
            if nat_lt(a1873, a1874) {
                let m1877 = {
                    let a1875 = v0;
                    let a1876 = 0u64;
                    index_bytes(a1875, a1876)
                };
                match m1877 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1878 = v2;
                        let a1879 = 48u8;
                        Ok(!equal(a1878, a1879))
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

pub fn f335(v0: Fn16, p1884: u64, p1885: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1883 = (p1884, p1885);
    {
        let mut m1883 = m1883;
        loop {
            let step = {
                let (v1, v2) = m1883;
                let m1880 = v1;
                if m1880 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1880 - 1;
                    let m1882 = {
                        let c1881: Fn16 = v0.clone();
                        c1881.apply(v2)?
                    };
                    match m1882 {
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
                Ok(next) => { m1883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f336(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f362(v0, v1)
}

pub fn f337(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f234(v0.clone(), 0u64, {
        let a1886 = v0;
        length_bytes(a1886)
    }, false, Bytes::lit(&[]))?;
    let m1889 = {
        let a1887 = v1.clone();
        let a1888 = 0u64;
        index_bytes(a1887, a1888)
    };
    match m1889 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1934 = {
                    let a1910 = {
                        let a1898 = {
                            let a1892 = {
                                let a1890 = v3;
                                let a1891 = 128u8;
                                fixed_u8::bit_and(a1890, a1891)
                            };
                            let a1893 = 128u8;
                            if equal(a1892, a1893) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1899 = {
                            let a1896 = {
                                let a1894 = v3;
                                let a1895 = 64u8;
                                fixed_u8::bit_and(a1894, a1895)
                            };
                            let a1897 = 64u8;
                            if equal(a1896, a1897) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1898, a1899)?
                    };
                    let a1911 = {
                        let a1908 = {
                            let a1902 = {
                                let a1900 = v3;
                                let a1901 = 32u8;
                                fixed_u8::bit_and(a1900, a1901)
                            };
                            let a1903 = 32u8;
                            if equal(a1902, a1903) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1909 = {
                            let a1906 = {
                                let a1904 = v3;
                                let a1905 = 16u8;
                                fixed_u8::bit_and(a1904, a1905)
                            };
                            let a1907 = 16u8;
                            if equal(a1906, a1907) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1908, a1909)?
                    };
                    nat_add(a1910, a1911)?
                };
                let a1935 = {
                    let a1932 = {
                        let a1920 = {
                            let a1914 = {
                                let a1912 = v3;
                                let a1913 = 8u8;
                                fixed_u8::bit_and(a1912, a1913)
                            };
                            let a1915 = 8u8;
                            if equal(a1914, a1915) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1921 = {
                            let a1918 = {
                                let a1916 = v3;
                                let a1917 = 4u8;
                                fixed_u8::bit_and(a1916, a1917)
                            };
                            let a1919 = 4u8;
                            if equal(a1918, a1919) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1920, a1921)?
                    };
                    let a1933 = {
                        let a1930 = {
                            let a1924 = {
                                let a1922 = v3;
                                let a1923 = 2u8;
                                fixed_u8::bit_and(a1922, a1923)
                            };
                            let a1925 = 2u8;
                            if equal(a1924, a1925) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1931 = {
                            let a1928 = {
                                let a1926 = v3;
                                let a1927 = 1u8;
                                fixed_u8::bit_and(a1926, a1927)
                            };
                            let a1929 = 1u8;
                            if equal(a1928, a1929) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1930, a1931)?
                    };
                    nat_add(a1932, a1933)?
                };
                nat_add(a1934, a1935)?
            };
            if f279(v4) {
                let a1942 = f31({
                    let a1936 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1936)
                }, {
                    let a1937 = v4;
                    let a1938 = 65u64;
                    nat_sub(a1937, a1938)
                }, 1u64);
                let a1943 = f31(v1.clone(), 1u64, {
                    let a1940 = {
                        let a1939 = v1;
                        length_bytes(a1939)
                    };
                    let a1941 = 1u64;
                    nat_sub(a1940, a1941)
                });
                Ok(append_bytes(a1942, a1943))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f338(v0: Bytes) -> R<bool> {
    let m1946 = {
        let a1944 = v0;
        let a1945 = 0u64;
        index_bytes(a1944, a1945)
    };
    match m1946 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f278({
                let v2: u8 = v1;
                let a1991 = {
                    let a1967 = {
                        let a1955 = {
                            let a1949 = {
                                let a1947 = v2;
                                let a1948 = 128u8;
                                fixed_u8::bit_and(a1947, a1948)
                            };
                            let a1950 = 128u8;
                            if equal(a1949, a1950) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1956 = {
                            let a1953 = {
                                let a1951 = v2;
                                let a1952 = 64u8;
                                fixed_u8::bit_and(a1951, a1952)
                            };
                            let a1954 = 64u8;
                            if equal(a1953, a1954) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1955, a1956)?
                    };
                    let a1968 = {
                        let a1965 = {
                            let a1959 = {
                                let a1957 = v2;
                                let a1958 = 32u8;
                                fixed_u8::bit_and(a1957, a1958)
                            };
                            let a1960 = 32u8;
                            if equal(a1959, a1960) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1966 = {
                            let a1963 = {
                                let a1961 = v2;
                                let a1962 = 16u8;
                                fixed_u8::bit_and(a1961, a1962)
                            };
                            let a1964 = 16u8;
                            if equal(a1963, a1964) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1965, a1966)?
                    };
                    nat_add(a1967, a1968)?
                };
                let a1992 = {
                    let a1989 = {
                        let a1977 = {
                            let a1971 = {
                                let a1969 = v2;
                                let a1970 = 8u8;
                                fixed_u8::bit_and(a1969, a1970)
                            };
                            let a1972 = 8u8;
                            if equal(a1971, a1972) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1978 = {
                            let a1975 = {
                                let a1973 = v2;
                                let a1974 = 4u8;
                                fixed_u8::bit_and(a1973, a1974)
                            };
                            let a1976 = 4u8;
                            if equal(a1975, a1976) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1977, a1978)?
                    };
                    let a1990 = {
                        let a1987 = {
                            let a1981 = {
                                let a1979 = v2;
                                let a1980 = 2u8;
                                fixed_u8::bit_and(a1979, a1980)
                            };
                            let a1982 = 2u8;
                            if equal(a1981, a1982) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1988 = {
                            let a1985 = {
                                let a1983 = v2;
                                let a1984 = 1u8;
                                fixed_u8::bit_and(a1983, a1984)
                            };
                            let a1986 = 1u8;
                            if equal(a1985, a1986) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1987, a1988)?
                    };
                    nat_add(a1989, a1990)?
                };
                nat_add(a1991, a1992)?
            }) {
                Ok(true)
            } else {
                Ok(f279({
                    let v3: u8 = v1;
                    let a2037 = {
                        let a2013 = {
                            let a2001 = {
                                let a1995 = {
                                    let a1993 = v3;
                                    let a1994 = 128u8;
                                    fixed_u8::bit_and(a1993, a1994)
                                };
                                let a1996 = 128u8;
                                if equal(a1995, a1996) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a2002 = {
                                let a1999 = {
                                    let a1997 = v3;
                                    let a1998 = 64u8;
                                    fixed_u8::bit_and(a1997, a1998)
                                };
                                let a2000 = 64u8;
                                if equal(a1999, a2000) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2001, a2002)?
                        };
                        let a2014 = {
                            let a2011 = {
                                let a2005 = {
                                    let a2003 = v3;
                                    let a2004 = 32u8;
                                    fixed_u8::bit_and(a2003, a2004)
                                };
                                let a2006 = 32u8;
                                if equal(a2005, a2006) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a2012 = {
                                let a2009 = {
                                    let a2007 = v3;
                                    let a2008 = 16u8;
                                    fixed_u8::bit_and(a2007, a2008)
                                };
                                let a2010 = 16u8;
                                if equal(a2009, a2010) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2011, a2012)?
                        };
                        nat_add(a2013, a2014)?
                    };
                    let a2038 = {
                        let a2035 = {
                            let a2023 = {
                                let a2017 = {
                                    let a2015 = v3;
                                    let a2016 = 8u8;
                                    fixed_u8::bit_and(a2015, a2016)
                                };
                                let a2018 = 8u8;
                                if equal(a2017, a2018) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2024 = {
                                let a2021 = {
                                    let a2019 = v3;
                                    let a2020 = 4u8;
                                    fixed_u8::bit_and(a2019, a2020)
                                };
                                let a2022 = 4u8;
                                if equal(a2021, a2022) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2023, a2024)?
                        };
                        let a2036 = {
                            let a2033 = {
                                let a2027 = {
                                    let a2025 = v3;
                                    let a2026 = 2u8;
                                    fixed_u8::bit_and(a2025, a2026)
                                };
                                let a2028 = 2u8;
                                if equal(a2027, a2028) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2034 = {
                                let a2031 = {
                                    let a2029 = v3;
                                    let a2030 = 1u8;
                                    fixed_u8::bit_and(a2029, a2030)
                                };
                                let a2032 = 1u8;
                                if equal(a2031, a2032) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2033, a2034)?
                        };
                        nat_add(a2035, a2036)?
                    };
                    nat_add(a2037, a2038)?
                }))
            }
        }
    }
}

pub fn f339(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2039 = f363(v0.clone());
    match m2039 {
        None => {
            if f35(f123(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2040 = f120(v0, f8(Str::lit("items")));
                match m2040 {
                    None => {
                        Ok(f311(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f364(f8(Str::lit("List")), f365(v3, {
                            let a2041 = v1;
                            let a2042 = f8(Str::lit("Item"));
                            append_bytes(a2041, a2042)
                        })?))
                    }
                }
            } else {
                f365(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f364(f8(Str::lit("Option")), f365(v2, v1)?))
        }
    }
}

pub fn f340(v0: Bytes, v1: u64) -> Bytes {
    let m2046 = {
        let a2043 = v0;
        let a2044 = v1;
        let a2045 = 1u64;
        slice_bytes(a2043, a2044, a2045)
    };
    match m2046 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f341(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f366(v1, v0, v2)
}

pub fn f342(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2091 = {
            let a2067 = {
                let a2055 = {
                    let a2049 = {
                        let a2047 = v3;
                        let a2048 = 128u8;
                        fixed_u8::bit_and(a2047, a2048)
                    };
                    let a2050 = 128u8;
                    if equal(a2049, a2050) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2056 = {
                    let a2053 = {
                        let a2051 = v3;
                        let a2052 = 64u8;
                        fixed_u8::bit_and(a2051, a2052)
                    };
                    let a2054 = 64u8;
                    if equal(a2053, a2054) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2055, a2056)?
            };
            let a2068 = {
                let a2065 = {
                    let a2059 = {
                        let a2057 = v3;
                        let a2058 = 32u8;
                        fixed_u8::bit_and(a2057, a2058)
                    };
                    let a2060 = 32u8;
                    if equal(a2059, a2060) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2066 = {
                    let a2063 = {
                        let a2061 = v3;
                        let a2062 = 16u8;
                        fixed_u8::bit_and(a2061, a2062)
                    };
                    let a2064 = 16u8;
                    if equal(a2063, a2064) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2065, a2066)?
            };
            nat_add(a2067, a2068)?
        };
        let a2092 = {
            let a2089 = {
                let a2077 = {
                    let a2071 = {
                        let a2069 = v3;
                        let a2070 = 8u8;
                        fixed_u8::bit_and(a2069, a2070)
                    };
                    let a2072 = 8u8;
                    if equal(a2071, a2072) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2078 = {
                    let a2075 = {
                        let a2073 = v3;
                        let a2074 = 4u8;
                        fixed_u8::bit_and(a2073, a2074)
                    };
                    let a2076 = 4u8;
                    if equal(a2075, a2076) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2077, a2078)?
            };
            let a2090 = {
                let a2087 = {
                    let a2081 = {
                        let a2079 = v3;
                        let a2080 = 2u8;
                        fixed_u8::bit_and(a2079, a2080)
                    };
                    let a2082 = 2u8;
                    if equal(a2081, a2082) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2088 = {
                    let a2085 = {
                        let a2083 = v3;
                        let a2084 = 1u8;
                        fixed_u8::bit_and(a2083, a2084)
                    };
                    let a2086 = 1u8;
                    if equal(a2085, a2086) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2087, a2088)?
            };
            nat_add(a2089, a2090)?
        };
        nat_add(a2091, a2092)?
    };
    let a2093 = v4;
    let a2094 = 34u64;
    if nat_eq(a2093, a2094) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2095 = v4;
        let a2096 = 92u64;
        if nat_eq(a2095, a2096) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2097 = v4;
            let a2098 = 8u64;
            if nat_eq(a2097, a2098) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2099 = v4;
                let a2100 = 12u64;
                if nat_eq(a2099, a2100) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2101 = v4;
                    let a2102 = 10u64;
                    if nat_eq(a2101, a2102) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2103 = v4;
                        let a2104 = 13u64;
                        if nat_eq(a2103, a2104) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2105 = v4;
                            let a2106 = 9u64;
                            if nat_eq(a2105, a2106) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2107 = v4;
                                let a2108 = 32u64;
                                if nat_lt(a2107, a2108) {
                                    let a2109 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2110 = f367(v2)?;
                                    Ok(append_bytes(a2109, a2110))
                                } else {
                                    Ok(f31(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f343(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f368(Fn21::F369, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f344(v0: Fn17, p2114: List<List<(Bytes, Bytes)>>, p2115: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2113 = (p2114, p2115);
    {
        let mut m2113 = m2113;
        loop {
            let step = {
                let (v1, v2) = m2113;
                let m2111 = v2;
                match m2111.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2112: Fn17 = v0.clone();
                            c2112.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2113 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f346(v0: Fn18, p2120: u64, p2121: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2119 = (p2120, p2121);
    {
        let mut m2119 = m2119;
        loop {
            let step = {
                let (v1, v2) = m2119;
                let m2116 = v1;
                if m2116 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2116 - 1;
                    let m2118 = {
                        let c2117: Fn18 = v0.clone();
                        c2117.apply(v2.clone())?
                    };
                    match m2118 {
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
                Ok(next) => { m2119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f370(v0)
}

pub fn f348(v0: Fn19, p2126: u64, p2127: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2125 = (p2126, p2127);
    {
        let mut m2125 = m2125;
        loop {
            let step = {
                let (v1, v2) = m2125;
                let m2122 = v1;
                if m2122 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2122 - 1;
                    let m2124 = {
                        let c2123: Fn19 = v0.clone();
                        c2123.apply(v2.clone())?
                    };
                    match m2124 {
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
                Ok(next) => { m2125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f371(v0)
}

pub fn f350(v0: u64) -> u32 {
    let a2128 = v0;
    let a2129 = 32u64;
    if nat_lt(a2128, a2129) {
        let a2130 = v0;
        let a2131 = 16u64;
        if nat_lt(a2130, a2131) {
            let a2132 = v0;
            let a2133 = 8u64;
            if nat_lt(a2132, a2133) {
                let a2134 = v0;
                let a2135 = 4u64;
                if nat_lt(a2134, a2135) {
                    let a2136 = v0;
                    let a2137 = 2u64;
                    if nat_lt(a2136, a2137) {
                        let a2138 = v0;
                        let a2139 = 1u64;
                        if nat_lt(a2138, a2139) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2140 = v0;
                        let a2141 = 3u64;
                        if nat_lt(a2140, a2141) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2142 = v0;
                    let a2143 = 6u64;
                    if nat_lt(a2142, a2143) {
                        let a2144 = v0;
                        let a2145 = 5u64;
                        if nat_lt(a2144, a2145) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2146 = v0;
                        let a2147 = 7u64;
                        if nat_lt(a2146, a2147) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2148 = v0;
                let a2149 = 12u64;
                if nat_lt(a2148, a2149) {
                    let a2150 = v0;
                    let a2151 = 10u64;
                    if nat_lt(a2150, a2151) {
                        let a2152 = v0;
                        let a2153 = 9u64;
                        if nat_lt(a2152, a2153) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2154 = v0;
                        let a2155 = 11u64;
                        if nat_lt(a2154, a2155) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2156 = v0;
                    let a2157 = 14u64;
                    if nat_lt(a2156, a2157) {
                        let a2158 = v0;
                        let a2159 = 13u64;
                        if nat_lt(a2158, a2159) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2160 = v0;
                        let a2161 = 15u64;
                        if nat_lt(a2160, a2161) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2162 = v0;
            let a2163 = 24u64;
            if nat_lt(a2162, a2163) {
                let a2164 = v0;
                let a2165 = 20u64;
                if nat_lt(a2164, a2165) {
                    let a2166 = v0;
                    let a2167 = 18u64;
                    if nat_lt(a2166, a2167) {
                        let a2168 = v0;
                        let a2169 = 17u64;
                        if nat_lt(a2168, a2169) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2170 = v0;
                        let a2171 = 19u64;
                        if nat_lt(a2170, a2171) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2172 = v0;
                    let a2173 = 22u64;
                    if nat_lt(a2172, a2173) {
                        let a2174 = v0;
                        let a2175 = 21u64;
                        if nat_lt(a2174, a2175) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2176 = v0;
                        let a2177 = 23u64;
                        if nat_lt(a2176, a2177) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2178 = v0;
                let a2179 = 28u64;
                if nat_lt(a2178, a2179) {
                    let a2180 = v0;
                    let a2181 = 26u64;
                    if nat_lt(a2180, a2181) {
                        let a2182 = v0;
                        let a2183 = 25u64;
                        if nat_lt(a2182, a2183) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2184 = v0;
                        let a2185 = 27u64;
                        if nat_lt(a2184, a2185) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2186 = v0;
                    let a2187 = 30u64;
                    if nat_lt(a2186, a2187) {
                        let a2188 = v0;
                        let a2189 = 29u64;
                        if nat_lt(a2188, a2189) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2190 = v0;
                        let a2191 = 31u64;
                        if nat_lt(a2190, a2191) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2192 = v0;
        let a2193 = 48u64;
        if nat_lt(a2192, a2193) {
            let a2194 = v0;
            let a2195 = 40u64;
            if nat_lt(a2194, a2195) {
                let a2196 = v0;
                let a2197 = 36u64;
                if nat_lt(a2196, a2197) {
                    let a2198 = v0;
                    let a2199 = 34u64;
                    if nat_lt(a2198, a2199) {
                        let a2200 = v0;
                        let a2201 = 33u64;
                        if nat_lt(a2200, a2201) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2202 = v0;
                        let a2203 = 35u64;
                        if nat_lt(a2202, a2203) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2204 = v0;
                    let a2205 = 38u64;
                    if nat_lt(a2204, a2205) {
                        let a2206 = v0;
                        let a2207 = 37u64;
                        if nat_lt(a2206, a2207) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2208 = v0;
                        let a2209 = 39u64;
                        if nat_lt(a2208, a2209) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2210 = v0;
                let a2211 = 44u64;
                if nat_lt(a2210, a2211) {
                    let a2212 = v0;
                    let a2213 = 42u64;
                    if nat_lt(a2212, a2213) {
                        let a2214 = v0;
                        let a2215 = 41u64;
                        if nat_lt(a2214, a2215) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2216 = v0;
                        let a2217 = 43u64;
                        if nat_lt(a2216, a2217) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2218 = v0;
                    let a2219 = 46u64;
                    if nat_lt(a2218, a2219) {
                        let a2220 = v0;
                        let a2221 = 45u64;
                        if nat_lt(a2220, a2221) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2222 = v0;
                        let a2223 = 47u64;
                        if nat_lt(a2222, a2223) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2224 = v0;
            let a2225 = 56u64;
            if nat_lt(a2224, a2225) {
                let a2226 = v0;
                let a2227 = 52u64;
                if nat_lt(a2226, a2227) {
                    let a2228 = v0;
                    let a2229 = 50u64;
                    if nat_lt(a2228, a2229) {
                        let a2230 = v0;
                        let a2231 = 49u64;
                        if nat_lt(a2230, a2231) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2232 = v0;
                        let a2233 = 51u64;
                        if nat_lt(a2232, a2233) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2234 = v0;
                    let a2235 = 54u64;
                    if nat_lt(a2234, a2235) {
                        let a2236 = v0;
                        let a2237 = 53u64;
                        if nat_lt(a2236, a2237) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2238 = v0;
                        let a2239 = 55u64;
                        if nat_lt(a2238, a2239) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2240 = v0;
                let a2241 = 60u64;
                if nat_lt(a2240, a2241) {
                    let a2242 = v0;
                    let a2243 = 58u64;
                    if nat_lt(a2242, a2243) {
                        let a2244 = v0;
                        let a2245 = 57u64;
                        if nat_lt(a2244, a2245) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2246 = v0;
                        let a2247 = 59u64;
                        if nat_lt(a2246, a2247) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2248 = v0;
                    let a2249 = 62u64;
                    if nat_lt(a2248, a2249) {
                        let a2250 = v0;
                        let a2251 = 61u64;
                        if nat_lt(a2250, a2251) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2252 = v0;
                        let a2253 = 63u64;
                        if nat_lt(a2252, a2253) {
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

pub fn f351(v0: u32) -> u32 {
    let a2256 = {
        let a2254 = f372(v0, 6u32, 26u32);
        let a2255 = f372(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2254, a2255)
    };
    let a2257 = f372(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2256, a2257)
}

pub fn f352(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2263 = {
        let a2258 = v0;
        let a2259 = v1;
        fixed_u32::bit_and(a2258, a2259)
    };
    let a2264 = {
        let a2261 = {
            let a2260 = v0;
            fixed_u32::bit_not(a2260)
        };
        let a2262 = v2;
        fixed_u32::bit_and(a2261, a2262)
    };
    fixed_u32::bit_xor(a2263, a2264)
}

pub fn f353(v0: u32) -> u32 {
    let a2267 = {
        let a2265 = f372(v0, 2u32, 30u32);
        let a2266 = f372(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2265, a2266)
    };
    let a2268 = f372(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2267, a2268)
}

pub fn f354(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2277 = {
        let a2273 = {
            let a2269 = v0;
            let a2270 = v1;
            fixed_u32::bit_and(a2269, a2270)
        };
        let a2274 = {
            let a2271 = v0;
            let a2272 = v2;
            fixed_u32::bit_and(a2271, a2272)
        };
        fixed_u32::bit_xor(a2273, a2274)
    };
    let a2278 = {
        let a2275 = v1;
        let a2276 = v2;
        fixed_u32::bit_and(a2275, a2276)
    };
    fixed_u32::bit_xor(a2277, a2278)
}

pub fn f355(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2279, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2279
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2280, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2280
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2281, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2281
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2282, _, _, _, _, _, _, _, _, _, _, _) => {
            h2282
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2283, _, _, _, _, _, _, _, _, _, _) => {
            h2283
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2284, _, _, _, _, _, _, _, _, _) => {
            h2284
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2285, _, _, _, _, _, _, _, _) => {
            h2285
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2286, _, _, _, _, _, _, _) => {
            h2286
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2287, _, _, _, _, _, _) => {
            h2287
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2288, _, _, _, _, _) => {
            h2288
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2289, _, _, _, _) => {
            h2289
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2290, _, _, _) => {
            h2290
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2291, _, _) => {
            h2291
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2292, _) => {
            h2292
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2293) => {
            h2293
        }
    }, f302(f373(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2294, _) => {
            h2294
        }
    }), f302(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2295, _, _, _, _, _, _) => {
            h2295
        }
    }, f302(f374(match v0.clone() {
        Adt22::C0(_, h2296, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2296
        }
    }), match v0 {
        Adt22::C0(h2297, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2297
        }
    }))))
}

pub fn f356(v0: Fn20, p2301: u64, p2302: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2300 = (p2301, p2302);
    {
        let mut m2300 = m2300;
        loop {
            let step = {
                let (v1, v2) = m2300;
                let m2298 = v1;
                if m2298 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2298 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2299: Fn20 = v0.clone();
                        c2299.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2303, _) = v0;
        h2303
    };
    let v2: u32 = {
        let (h2304, _) = {
            let (_, h2305) = v0;
            h2305
        };
        h2304
    };
    let v3: u32 = {
        let (_, h2306) = {
            let (_, h2307) = v0;
            h2307
        };
        h2306
    };
    ({
        let a2308 = v1;
        let a2309 = 2u64;
        let a2310 = 0u64;
        nat_quot(a2308, a2309, a2310)
    }, (f358(v2, 1u32), {
        let a2314 = {
            let a2311 = v1;
            let a2312 = 2u64;
            let a2313 = 0u64;
            nat_rem(a2311, a2312, a2313)
        };
        let a2315 = 1u64;
        if nat_eq(a2314, a2315) {
            let a2316 = v3;
            let a2317 = v2;
            fixed_u32::bit_or(a2316, a2317)
        } else {
            v3
        }
    }))
}

pub fn f358(v0: u32, v1: u32) -> u32 {
    let m2320 = {
        let a2318 = v0;
        let a2319 = v1;
        fixed_u32::shift_left(a2318, a2319)
    };
    match m2320 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f359(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f273(v0.clone(), {
        let a2321 = v1;
        let a2322 = 1u64;
        nat_add(a2321, a2322)?
    })?;
    let v4: Bytes = {
        let a2323 = v3;
        let a2324 = 117u64;
        if nat_eq(a2323, a2324) {
            f375(v0, v1)?
        } else {
            f376(v3)
        }
    };
    let a2325 = v4.clone();
    let a2326 = Bytes::lit(&[]);
    if equal(a2325, a2326) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2327 = v3;
            let a2328 = 117u64;
            if nat_eq(a2327, a2328) {
                let a2329 = v1;
                let a2330 = 6u64;
                nat_add(a2329, a2330)?
            } else {
                let a2331 = v1;
                let a2332 = 2u64;
                nat_add(a2331, a2332)?
            }
        }, ({
            let a2333 = v2;
            let a2334 = v4;
            append_bytes(a2333, a2334)
        }, 0u64))))
    }
}

pub fn f360(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f273(v0, v1)?;
    let a2335 = 47u64;
    let a2336 = v2;
    if if nat_lt(a2335, a2336) {
        let a2337 = v2;
        let a2338 = 58u64;
        nat_lt(a2337, a2338)
    } else {
        false
    } {
        Ok(Some({
            let a2339 = v1;
            let a2340 = 1u64;
            nat_add(a2339, a2340)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f361(v0: Bytes) -> R<u64> {
    let (h2341, _) = f193(Fn3::F377(v0.clone()), {
        let a2342 = v0;
        length_bytes(a2342)
    }, 0u64)?;
    Ok(h2341)
}

pub fn f362(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2346 = {
        let a2344 = v0;
        let a2345 = {
            let (h2343, _) = v1;
            h2343
        };
        index_bytes(a2344, a2345)
    };
    match m2346 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2348 = {
                    let (h2347, _) = v1;
                    h2347
                };
                let a2349 = 1u64;
                nat_add(a2348, a2349)?
            }, {
                let a2353 = {
                    let a2351 = {
                        let (_, h2350) = v1;
                        h2350
                    };
                    let a2352 = 10u64;
                    nat_mul(a2351, a2352)?
                };
                let a2354 = f378(v2)?;
                nat_add(a2353, a2354)?
            })))
        }
    }
}

pub fn f363(v0: Adt14) -> Option<Adt14> {
    let m2355 = f124(f120(v0, f8(Str::lit("oneOf"))));
    match m2355.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2356 = v2;
            match m2356.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2357 = v4;
                    match m2357.uncons() {
                        None => {
                            if f379(v3.clone()) {
                                Some(v1)
                            } else {
                                if f379(v1) {
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

pub fn f364(v0: Bytes, v1: Result<(Bytes, List<Adt21>), Bytes>) -> Result<(Bytes, List<Adt21>), Bytes> {
    let m2358 = v1;
    match m2358 {
        Ok(v3) => {
            f312(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2359, _) = v3;
                h2359
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2360) = v3;
                h2360
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt21>), Bytes>(v2)
        }
    }
}

pub fn f365(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2361 = f363(v0.clone());
    match m2361 {
        None => {
            if f35(f123(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2362 = f120(v0, f8(Str::lit("items")));
                match m2362 {
                    None => {
                        Ok(f311(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f364(f8(Str::lit("List")), f380(v3, {
                            let a2363 = v1;
                            let a2364 = f8(Str::lit("Item"));
                            append_bytes(a2363, a2364)
                        })?))
                    }
                }
            } else {
                f380(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f364(f8(Str::lit("Option")), f380(v2, v1)?))
        }
    }
}

pub fn f366(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2365, _) = v2;
        h2365
    };
    let a2366 = v3;
    let a2367 = 0u64;
    if nat_eq(a2366, a2367) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2368 = v3;
            let a2369 = v1;
            let a2370 = 0u64;
            nat_quot(a2368, a2369, a2370)
        }, {
            let a2375 = f340(v0, {
                let a2371 = v3;
                let a2372 = v1;
                let a2373 = 0u64;
                nat_rem(a2371, a2372, a2373)
            });
            let a2376 = {
                let (_, h2374) = v2;
                h2374
            };
            append_bytes(a2375, a2376)
        }))
    }
}

pub fn f367(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2421 = {
            let a2397 = {
                let a2385 = {
                    let a2379 = {
                        let a2377 = v1;
                        let a2378 = 128u8;
                        fixed_u8::bit_and(a2377, a2378)
                    };
                    let a2380 = 128u8;
                    if equal(a2379, a2380) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2386 = {
                    let a2383 = {
                        let a2381 = v1;
                        let a2382 = 64u8;
                        fixed_u8::bit_and(a2381, a2382)
                    };
                    let a2384 = 64u8;
                    if equal(a2383, a2384) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2385, a2386)?
            };
            let a2398 = {
                let a2395 = {
                    let a2389 = {
                        let a2387 = v1;
                        let a2388 = 32u8;
                        fixed_u8::bit_and(a2387, a2388)
                    };
                    let a2390 = 32u8;
                    if equal(a2389, a2390) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2396 = {
                    let a2393 = {
                        let a2391 = v1;
                        let a2392 = 16u8;
                        fixed_u8::bit_and(a2391, a2392)
                    };
                    let a2394 = 16u8;
                    if equal(a2393, a2394) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2395, a2396)?
            };
            nat_add(a2397, a2398)?
        };
        let a2422 = {
            let a2419 = {
                let a2407 = {
                    let a2401 = {
                        let a2399 = v1;
                        let a2400 = 8u8;
                        fixed_u8::bit_and(a2399, a2400)
                    };
                    let a2402 = 8u8;
                    if equal(a2401, a2402) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2408 = {
                    let a2405 = {
                        let a2403 = v1;
                        let a2404 = 4u8;
                        fixed_u8::bit_and(a2403, a2404)
                    };
                    let a2406 = 4u8;
                    if equal(a2405, a2406) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2407, a2408)?
            };
            let a2420 = {
                let a2417 = {
                    let a2411 = {
                        let a2409 = v1;
                        let a2410 = 2u8;
                        fixed_u8::bit_and(a2409, a2410)
                    };
                    let a2412 = 2u8;
                    if equal(a2411, a2412) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2418 = {
                    let a2415 = {
                        let a2413 = v1;
                        let a2414 = 1u8;
                        fixed_u8::bit_and(a2413, a2414)
                    };
                    let a2416 = 1u8;
                    if equal(a2415, a2416) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2417, a2418)?
            };
            nat_add(a2419, a2420)?
        };
        nat_add(a2421, a2422)?
    };
    let a2429 = f340(f381(), {
        let a2423 = v2;
        let a2424 = 16u64;
        let a2425 = 0u64;
        nat_quot(a2423, a2424, a2425)
    });
    let a2430 = f340(f381(), {
        let a2426 = v2;
        let a2427 = 16u64;
        let a2428 = 0u64;
        nat_rem(a2426, a2427, a2428)
    });
    Ok(append_bytes(a2429, a2430))
}

pub fn f368(v0: Fn21, p2434: List<List<(Bytes, Bytes)>>, p2435: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2433 = (p2434, p2435);
    {
        let mut m2433 = m2433;
        loop {
            let step = {
                let (v1, v2) = m2433;
                let m2431 = v2;
                match m2431.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2432: Fn21 = v0.clone();
                            c2432.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f370(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2437 = {
        let a2436 = v0.clone();
        length_list(a2436)
    };
    let a2438 = 1u64;
    if nat_le(a2437, a2438) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f382(v0)?))
    }
}

pub fn f371(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2439, _) = v0;
        h2439
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2440) = v0;
        h2440
    };
    let m2441 = v1;
    match m2441.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2442 = v4;
            match m2442.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f383(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f372(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2443 = f267(v0, v1);
    let a2444 = f358(v0, v2);
    fixed_u32::bit_or(a2443, a2444)
}

pub fn f373(v0: u32) -> u32 {
    let a2447 = {
        let a2445 = f372(v0, 17u32, 15u32);
        let a2446 = f372(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2445, a2446)
    };
    let a2448 = f267(v0, 10u32);
    fixed_u32::bit_xor(a2447, a2448)
}

pub fn f374(v0: u32) -> u32 {
    let a2451 = {
        let a2449 = f372(v0, 7u32, 25u32);
        let a2450 = f372(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2449, a2450)
    };
    let a2452 = f267(v0, 3u32);
    fixed_u32::bit_xor(a2451, a2452)
}

pub fn f375(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2455 = f31(v0.clone(), {
        let a2453 = v1;
        let a2454 = 2u64;
        nat_add(a2453, a2454)?
    }, 2u64);
    let a2456 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2455, a2456) {
        let m2459 = f384(f31(v0, {
            let a2457 = v1;
            let a2458 = 4u64;
            nat_add(a2457, a2458)?
        }, 2u64))?;
        match m2459 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f273(v2.clone(), 0u64)?;
                let a2460 = v3;
                let a2461 = 32u64;
                if nat_lt(a2460, a2461) {
                    if f385(v3) {
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

pub fn f376(v0: u64) -> Bytes {
    let a2462 = v0;
    let a2463 = 34u64;
    if nat_eq(a2462, a2463) {
        Bytes::lit(&[0x22])
    } else {
        let a2464 = v0;
        let a2465 = 92u64;
        if nat_eq(a2464, a2465) {
            Bytes::lit(&[0x5c])
        } else {
            let a2466 = v0;
            let a2467 = 98u64;
            if nat_eq(a2466, a2467) {
                Bytes::lit(&[0x08])
            } else {
                let a2468 = v0;
                let a2469 = 102u64;
                if nat_eq(a2468, a2469) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2470 = v0;
                    let a2471 = 110u64;
                    if nat_eq(a2470, a2471) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2472 = v0;
                        let a2473 = 114u64;
                        if nat_eq(a2472, a2473) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2474 = v0;
                            let a2475 = 116u64;
                            if nat_eq(a2474, a2475) {
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

pub fn f377(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2478 = {
        let a2476 = v0;
        let a2477 = v1;
        index_bytes(a2476, a2477)
    };
    match m2478 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f386(v2)? {
                Ok(Some({
                    let a2479 = v1;
                    let a2480 = 1u64;
                    nat_add(a2479, a2480)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f378(v0: u8) -> R<u64> {
    let a2527 = {
        let v1: u8 = v0;
        let a2525 = {
            let a2501 = {
                let a2489 = {
                    let a2483 = {
                        let a2481 = v1;
                        let a2482 = 128u8;
                        fixed_u8::bit_and(a2481, a2482)
                    };
                    let a2484 = 128u8;
                    if equal(a2483, a2484) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2490 = {
                    let a2487 = {
                        let a2485 = v1;
                        let a2486 = 64u8;
                        fixed_u8::bit_and(a2485, a2486)
                    };
                    let a2488 = 64u8;
                    if equal(a2487, a2488) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2489, a2490)?
            };
            let a2502 = {
                let a2499 = {
                    let a2493 = {
                        let a2491 = v1;
                        let a2492 = 32u8;
                        fixed_u8::bit_and(a2491, a2492)
                    };
                    let a2494 = 32u8;
                    if equal(a2493, a2494) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2500 = {
                    let a2497 = {
                        let a2495 = v1;
                        let a2496 = 16u8;
                        fixed_u8::bit_and(a2495, a2496)
                    };
                    let a2498 = 16u8;
                    if equal(a2497, a2498) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2499, a2500)?
            };
            nat_add(a2501, a2502)?
        };
        let a2526 = {
            let a2523 = {
                let a2511 = {
                    let a2505 = {
                        let a2503 = v1;
                        let a2504 = 8u8;
                        fixed_u8::bit_and(a2503, a2504)
                    };
                    let a2506 = 8u8;
                    if equal(a2505, a2506) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2512 = {
                    let a2509 = {
                        let a2507 = v1;
                        let a2508 = 4u8;
                        fixed_u8::bit_and(a2507, a2508)
                    };
                    let a2510 = 4u8;
                    if equal(a2509, a2510) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2511, a2512)?
            };
            let a2524 = {
                let a2521 = {
                    let a2515 = {
                        let a2513 = v1;
                        let a2514 = 2u8;
                        fixed_u8::bit_and(a2513, a2514)
                    };
                    let a2516 = 2u8;
                    if equal(a2515, a2516) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2522 = {
                    let a2519 = {
                        let a2517 = v1;
                        let a2518 = 1u8;
                        fixed_u8::bit_and(a2517, a2518)
                    };
                    let a2520 = 1u8;
                    if equal(a2519, a2520) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2521, a2522)?
            };
            nat_add(a2523, a2524)?
        };
        nat_add(a2525, a2526)?
    };
    let a2528 = 48u64;
    Ok(nat_sub(a2527, a2528))
}

pub fn f379(v0: Adt14) -> bool {
    f35(f123(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f380(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2529 = f387(f282(v0.clone()));
    match m2529 {
        None => {
            let m2530 = f128(f120(v0.clone(), f8(Str::lit("$ref"))));
            match m2530 {
                None => {
                    let m2532 = f120(v0.clone(), f8(Str::lit("enum")));
                    match m2532 {
                        None => {
                            let m2533 = f120(v0.clone(), f8(Str::lit("const")));
                            match m2533 {
                                None => {
                                    let v8: Bytes = f123(v0.clone(), f8(Str::lit("type")));
                                    if f35(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f312(f8(Str::lit("Bytes")), List::<Adt21>::nil()))
                                    } else {
                                        if f35(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f312(f389(v0), List::<Adt21>::nil()))
                                        } else {
                                            if f35(v8.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f312(f8(Str::lit("Bool")), List::<Adt21>::nil()))
                                            } else {
                                                if f35(v8.clone(), f8(Str::lit("object"))) {
                                                    Ok(f312(v1.clone(), List::cons(Adt21::C0(v1, v0), List::<Adt21>::nil())))
                                                } else {
                                                    Ok(f311(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2534 = f128(Some(v6));
                                    match m2534 {
                                        None => {
                                            Ok(f311(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f312(f8(Str::lit("Bytes")), List::<Adt21>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f312(v1.clone(), List::cons(Adt21::C0(v1, v0), List::<Adt21>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2531 = f388(v3)?;
                    match m2531 {
                        None => {
                            Ok(f311(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f312(v4, List::<Adt21>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f311(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f381() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f382(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f343({
        let (_, h2535) = {
            let (h2536, _) = f390(Fn22::F391, {
                let a2538 = {
                    let a2537 = v0.clone();
                    length_list(a2537)
                };
                let a2539 = 1u64;
                nat_add(a2538, a2539)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2536
        };
        h2535
    }))
}

pub fn f383(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2544 = {
            let a2542 = {
                let a2540 = v0.clone();
                length_list(a2540)
            };
            let a2543 = {
                let a2541 = v1.clone();
                length_list(a2541)
            };
            nat_add(a2542, a2543)?
        };
        let a2545 = 1u64;
        nat_add(a2544, a2545)?
    };
    Ok(f261({
        let (_, h2546) = {
            let (_, h2547) = {
                let (h2548, _) = f392(Fn23::F393, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2548
            };
            h2547
        };
        h2546
    }))
}

pub fn f384(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2549 = v0.clone();
        length_bytes(a2549)
    };
    let a2553 = {
        let a2550 = v1;
        let a2551 = 2u64;
        let a2552 = 0u64;
        nat_rem(a2550, a2551, a2552)
    };
    let a2554 = 0u64;
    if nat_eq(a2553, a2554) {
        let (_, h2555) = {
            let (h2556, _) = f394(Fn24::F395(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2556
        };
        Ok(h2555)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f385(v0: u64) -> bool {
    let a2557 = v0;
    let a2558 = 8u64;
    if nat_eq(a2557, a2558) {
        true
    } else {
        let a2559 = v0;
        let a2560 = 9u64;
        if nat_eq(a2559, a2560) {
            true
        } else {
            let a2561 = v0;
            let a2562 = 10u64;
            if nat_eq(a2561, a2562) {
                true
            } else {
                let a2563 = v0;
                let a2564 = 12u64;
                if nat_eq(a2563, a2564) {
                    true
                } else {
                    let a2565 = v0;
                    let a2566 = 13u64;
                    nat_eq(a2565, a2566)
                }
            }
        }
    }
}

pub fn f386(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2611 = {
            let a2587 = {
                let a2575 = {
                    let a2569 = {
                        let a2567 = v1;
                        let a2568 = 128u8;
                        fixed_u8::bit_and(a2567, a2568)
                    };
                    let a2570 = 128u8;
                    if equal(a2569, a2570) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2576 = {
                    let a2573 = {
                        let a2571 = v1;
                        let a2572 = 64u8;
                        fixed_u8::bit_and(a2571, a2572)
                    };
                    let a2574 = 64u8;
                    if equal(a2573, a2574) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2575, a2576)?
            };
            let a2588 = {
                let a2585 = {
                    let a2579 = {
                        let a2577 = v1;
                        let a2578 = 32u8;
                        fixed_u8::bit_and(a2577, a2578)
                    };
                    let a2580 = 32u8;
                    if equal(a2579, a2580) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2586 = {
                    let a2583 = {
                        let a2581 = v1;
                        let a2582 = 16u8;
                        fixed_u8::bit_and(a2581, a2582)
                    };
                    let a2584 = 16u8;
                    if equal(a2583, a2584) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2585, a2586)?
            };
            nat_add(a2587, a2588)?
        };
        let a2612 = {
            let a2609 = {
                let a2597 = {
                    let a2591 = {
                        let a2589 = v1;
                        let a2590 = 8u8;
                        fixed_u8::bit_and(a2589, a2590)
                    };
                    let a2592 = 8u8;
                    if equal(a2591, a2592) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2598 = {
                    let a2595 = {
                        let a2593 = v1;
                        let a2594 = 4u8;
                        fixed_u8::bit_and(a2593, a2594)
                    };
                    let a2596 = 4u8;
                    if equal(a2595, a2596) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2597, a2598)?
            };
            let a2610 = {
                let a2607 = {
                    let a2601 = {
                        let a2599 = v1;
                        let a2600 = 2u8;
                        fixed_u8::bit_and(a2599, a2600)
                    };
                    let a2602 = 2u8;
                    if equal(a2601, a2602) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2608 = {
                    let a2605 = {
                        let a2603 = v1;
                        let a2604 = 1u8;
                        fixed_u8::bit_and(a2603, a2604)
                    };
                    let a2606 = 1u8;
                    if equal(a2605, a2606) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2607, a2608)?
            };
            nat_add(a2609, a2610)?
        };
        nat_add(a2611, a2612)?
    };
    let a2613 = 48u64;
    let a2614 = v2;
    if nat_le(a2613, a2614) {
        let a2615 = v2;
        let a2616 = 57u64;
        Ok(nat_le(a2615, a2616))
    } else {
        Ok(false)
    }
}

pub fn f387(p2622: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2621 = p2622;
    {
        let mut m2621 = m2621;
        loop {
            let step = {
                let v0 = m2621;
                let m2617 = v0;
                match m2617.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f80({
                            let (h2618, _) = v1.clone();
                            h2618
                        }, f396()) {
                            true
                        } else {
                            f80({
                                let (h2619, _) = v1.clone();
                                h2619
                            }, f397())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2620, _) = v1;
                                h2620
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: Bytes) -> R<Option<Bytes>> {
    if f253(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f180(f31(v0.clone(), 8u64, {
            let a2624 = {
                let a2623 = v0;
                length_bytes(a2623)
            };
            let a2625 = 8u64;
            nat_sub(a2624, a2625)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f389(v0: Adt14) -> Bytes {
    let m2626 = f252(f120(v0, f8(Str::lit("minimum"))));
    match m2626 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f390(v0: Fn22, p2631: u64, p2632: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2630 = (p2631, p2632);
    {
        let mut m2630 = m2630;
        loop {
            let step = {
                let (v1, v2) = m2630;
                let m2627 = v1;
                if m2627 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2627 - 1;
                    let m2629 = {
                        let c2628: Fn22 = v0.clone();
                        c2628.apply(v2.clone())?
                    };
                    match m2629 {
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
                Ok(next) => { m2630 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f398(v0)
}

pub fn f392(v0: Fn23, p2637: u64, p2638: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2636 = (p2637, p2638);
    {
        let mut m2636 = m2636;
        loop {
            let step = {
                let (v1, v2) = m2636;
                let m2633 = v1;
                if m2633 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2633 - 1;
                    let m2635 = {
                        let c2634: Fn23 = v0.clone();
                        c2634.apply(v2.clone())
                    };
                    match m2635 {
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
                Ok(next) => { m2636 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f399(v0)
}

pub fn f394(v0: Fn24, p2643: u64, p2644: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2642 = (p2643, p2644);
    {
        let mut m2642 = m2642;
        loop {
            let step = {
                let (v1, v2) = m2642;
                let m2639 = v1;
                if m2639 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2639 - 1;
                    let m2641 = {
                        let c2640: Fn24 = v0.clone();
                        c2640.apply(v2.clone())?
                    };
                    match m2641 {
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
                Ok(next) => { m2642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f400(v0, v1)
}

pub fn f396() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f397() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f398(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2645, _) = v0;
        h2645
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2646) = v0;
        h2646
    };
    let m2647 = v1;
    match m2647.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2648 = v4;
            match m2648.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f401(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f399(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2649, _) = v0.clone();
        h2649
    };
    let v2: List<(Bytes, ())> = {
        let (h2650, _) = {
            let (_, h2651) = v0.clone();
            h2651
        };
        h2650
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2652) = {
            let (_, h2653) = v0;
            h2653
        };
        h2652
    };
    let m2654 = v1.clone();
    match m2654.uncons() {
        None => {
            let m2655 = v2;
            match m2655.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2656 = v2.clone();
            match m2656.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f402({
                        let (h2657, _) = v8.clone();
                        h2657
                    }, {
                        let (h2658, _) = v6.clone();
                        h2658
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

pub fn f400(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2659, _) = v1;
        h2659
    };
    let m2661 = {
        let (_, h2660) = v1;
        h2660
    };
    match m2661 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2664 = {
                let a2662 = v0.clone();
                let a2663 = v2;
                index_bytes(a2662, a2663)
            };
            match m2664 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2669 = {
                        let a2667 = v0;
                        let a2668 = {
                            let a2665 = v2;
                            let a2666 = 1u64;
                            nat_add(a2665, a2666)?
                        };
                        index_bytes(a2667, a2668)
                    };
                    match m2669 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f403(v4)? {
                                f403(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2670 = v2;
                                    let a2671 = 2u64;
                                    nat_add(a2670, a2671)?
                                }, Some({
                                    let a2676 = v3;
                                    let a2677 = f340(f404(), {
                                        let a2674 = {
                                            let a2672 = f405(v4)?;
                                            let a2673 = 16u64;
                                            nat_mul(a2672, a2673)?
                                        };
                                        let a2675 = f405(v5)?;
                                        nat_add(a2674, a2675)?
                                    });
                                    append_bytes(a2676, a2677)
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

pub fn f401(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2682 = {
            let a2680 = {
                let a2678 = v0.clone();
                length_list(a2678)
            };
            let a2681 = {
                let a2679 = v1.clone();
                length_list(a2679)
            };
            nat_add(a2680, a2681)?
        };
        let a2683 = 1u64;
        nat_add(a2682, a2683)?
    };
    Ok(f406({
        let (_, h2684) = {
            let (_, h2685) = {
                let (h2686, _) = f407(Fn25::F408, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2686
            };
            h2685
        };
        h2684
    }))
}

pub fn f402(v0: Bytes, v1: Bytes) -> bool {
    let a2691 = {
        let a2687 = v0;
        let a2688 = v1;
        compare_bytes(a2687, a2688)
    };
    let a2692 = {
        let a2689 = Bytes::lit(&[]);
        let a2690 = Bytes::lit(&[0x01]);
        compare_bytes(a2689, a2690)
    };
    equal(a2691, a2692)
}

pub fn f403(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2737 = {
            let a2713 = {
                let a2701 = {
                    let a2695 = {
                        let a2693 = v1;
                        let a2694 = 128u8;
                        fixed_u8::bit_and(a2693, a2694)
                    };
                    let a2696 = 128u8;
                    if equal(a2695, a2696) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2702 = {
                    let a2699 = {
                        let a2697 = v1;
                        let a2698 = 64u8;
                        fixed_u8::bit_and(a2697, a2698)
                    };
                    let a2700 = 64u8;
                    if equal(a2699, a2700) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2701, a2702)?
            };
            let a2714 = {
                let a2711 = {
                    let a2705 = {
                        let a2703 = v1;
                        let a2704 = 32u8;
                        fixed_u8::bit_and(a2703, a2704)
                    };
                    let a2706 = 32u8;
                    if equal(a2705, a2706) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2712 = {
                    let a2709 = {
                        let a2707 = v1;
                        let a2708 = 16u8;
                        fixed_u8::bit_and(a2707, a2708)
                    };
                    let a2710 = 16u8;
                    if equal(a2709, a2710) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2711, a2712)?
            };
            nat_add(a2713, a2714)?
        };
        let a2738 = {
            let a2735 = {
                let a2723 = {
                    let a2717 = {
                        let a2715 = v1;
                        let a2716 = 8u8;
                        fixed_u8::bit_and(a2715, a2716)
                    };
                    let a2718 = 8u8;
                    if equal(a2717, a2718) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2724 = {
                    let a2721 = {
                        let a2719 = v1;
                        let a2720 = 4u8;
                        fixed_u8::bit_and(a2719, a2720)
                    };
                    let a2722 = 4u8;
                    if equal(a2721, a2722) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2723, a2724)?
            };
            let a2736 = {
                let a2733 = {
                    let a2727 = {
                        let a2725 = v1;
                        let a2726 = 2u8;
                        fixed_u8::bit_and(a2725, a2726)
                    };
                    let a2728 = 2u8;
                    if equal(a2727, a2728) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2734 = {
                    let a2731 = {
                        let a2729 = v1;
                        let a2730 = 1u8;
                        fixed_u8::bit_and(a2729, a2730)
                    };
                    let a2732 = 1u8;
                    if equal(a2731, a2732) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2733, a2734)?
            };
            nat_add(a2735, a2736)?
        };
        nat_add(a2737, a2738)?
    };
    let a2739 = 48u64;
    let a2740 = v2;
    if if nat_le(a2739, a2740) {
        let a2741 = v2;
        let a2742 = 57u64;
        nat_le(a2741, a2742)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2743 = 97u64;
        let a2744 = v2;
        if nat_le(a2743, a2744) {
            let a2745 = v2;
            let a2746 = 102u64;
            Ok(nat_le(a2745, a2746))
        } else {
            Ok(false)
        }
    }
}

pub fn f404() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f405(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2791 = {
            let a2767 = {
                let a2755 = {
                    let a2749 = {
                        let a2747 = v1;
                        let a2748 = 128u8;
                        fixed_u8::bit_and(a2747, a2748)
                    };
                    let a2750 = 128u8;
                    if equal(a2749, a2750) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2756 = {
                    let a2753 = {
                        let a2751 = v1;
                        let a2752 = 64u8;
                        fixed_u8::bit_and(a2751, a2752)
                    };
                    let a2754 = 64u8;
                    if equal(a2753, a2754) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2755, a2756)?
            };
            let a2768 = {
                let a2765 = {
                    let a2759 = {
                        let a2757 = v1;
                        let a2758 = 32u8;
                        fixed_u8::bit_and(a2757, a2758)
                    };
                    let a2760 = 32u8;
                    if equal(a2759, a2760) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2766 = {
                    let a2763 = {
                        let a2761 = v1;
                        let a2762 = 16u8;
                        fixed_u8::bit_and(a2761, a2762)
                    };
                    let a2764 = 16u8;
                    if equal(a2763, a2764) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2765, a2766)?
            };
            nat_add(a2767, a2768)?
        };
        let a2792 = {
            let a2789 = {
                let a2777 = {
                    let a2771 = {
                        let a2769 = v1;
                        let a2770 = 8u8;
                        fixed_u8::bit_and(a2769, a2770)
                    };
                    let a2772 = 8u8;
                    if equal(a2771, a2772) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2778 = {
                    let a2775 = {
                        let a2773 = v1;
                        let a2774 = 4u8;
                        fixed_u8::bit_and(a2773, a2774)
                    };
                    let a2776 = 4u8;
                    if equal(a2775, a2776) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2777, a2778)?
            };
            let a2790 = {
                let a2787 = {
                    let a2781 = {
                        let a2779 = v1;
                        let a2780 = 2u8;
                        fixed_u8::bit_and(a2779, a2780)
                    };
                    let a2782 = 2u8;
                    if equal(a2781, a2782) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2788 = {
                    let a2785 = {
                        let a2783 = v1;
                        let a2784 = 1u8;
                        fixed_u8::bit_and(a2783, a2784)
                    };
                    let a2786 = 1u8;
                    if equal(a2785, a2786) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2787, a2788)?
            };
            nat_add(a2789, a2790)?
        };
        nat_add(a2791, a2792)?
    };
    let a2793 = v2;
    let a2794 = 57u64;
    if nat_le(a2793, a2794) {
        let a2795 = v2;
        let a2796 = 48u64;
        Ok(nat_sub(a2795, a2796))
    } else {
        let a2797 = v2;
        let a2798 = 87u64;
        Ok(nat_sub(a2797, a2798))
    }
}

pub fn f406(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f288(Fn10::F409, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f407(v0: Fn25, p2803: u64, p2804: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2802 = (p2803, p2804);
    {
        let mut m2802 = m2802;
        loop {
            let step = {
                let (v1, v2) = m2802;
                let m2799 = v1;
                if m2799 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2799 - 1;
                    let m2801 = {
                        let c2800: Fn25 = v0.clone();
                        c2800.apply(v2.clone())
                    };
                    match m2801 {
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
                Ok(next) => { m2802 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f410(v0)
}

pub fn f409(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f410(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2805, _) = v0.clone();
        h2805
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2806, _) = {
            let (_, h2807) = v0.clone();
            h2807
        };
        h2806
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2808) = {
            let (_, h2809) = v0;
            h2809
        };
        h2808
    };
    let m2810 = v1.clone();
    match m2810.uncons() {
        None => {
            let m2811 = v2;
            match m2811.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2812 = v2.clone();
            match m2812.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f402({
                        let (h2813, _) = v8.clone();
                        h2813
                    }, {
                        let (h2814, _) = v6.clone();
                        h2814
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
