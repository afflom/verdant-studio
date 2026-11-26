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
    C0,
    C1(Bytes, std::rc::Rc<Adt17>),
    C2(bool),
    C3(bool, u64),
    C4(bool, u64, u64),
    C5(Bytes),
    C6(std::rc::Rc<Adt17>),
    C7(std::rc::Rc<Adt17>),
    C8,
    C9(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C10(Bytes, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
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
    C0(Bytes, Adt17),
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
    C0(bool, List<(u64, u64)>, u64),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(List<Bytes>, List<Adt25>),
}

#[derive(Clone)]
pub enum Fn0 {
    F64(u8, Bytes),
    F155(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn0::F155(k0) => f155(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F94,
    F197,
    F290,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F94 => f94(p0, p1),
            Fn1::F197 => f197(p0, p1),
            Fn1::F290 => f290(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F169(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F169(k0) => f169(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F350(Bytes),
    F497(u64, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F350(k0) => f350(k0.clone(), p0),
            Fn3::F497(k0, k1) => Ok(f497(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F352,
    F694,
}

impl Fn4 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn4::F352 => f352(p0, p1),
            Fn4::F694 => f694(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F355,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn5::F355 => f355(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F361(Bytes),
    F362(u64, Bytes),
    F370(Bytes, u64, Bytes, u64),
    F490(Bytes),
    F491(Bytes),
    F545(Bytes),
    F574(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn6::F361(k0) => Ok(f361(k0.clone(), p0)),
            Fn6::F362(k0, k1) => f362(*k0, k1.clone(), p0),
            Fn6::F370(k0, k1, k2, k3) => f370(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn6::F490(k0) => f490(k0.clone(), p0),
            Fn6::F491(k0) => f491(k0.clone(), p0),
            Fn6::F545(k0) => f545(k0.clone(), p0),
            Fn6::F574(k0) => f574(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F381,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F381 => f381(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F384,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F384 => f384(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F386,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F386 => f386(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F389,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F389 => f389(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F412(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn11::F412(k0, k1, k2) => f412(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F447,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F447 => f447(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F450,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F450 => f450(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F489(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn14::F489(k0) => f489(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F494(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn15::F494(k0) => f494(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F501,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F501 => f501(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F503,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F503 => f503(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F520,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F520 => f520(p0),
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
    F552,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F552 => f552(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F607,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F607 => f607(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F618,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F618 => f618(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F651,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F651 => f651(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F667(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F667(k0) => f667(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F677,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F677 => f677(p0),
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
                                            Ok(Adt11::C4(f8(Str::lit("read-webidl, read-xml, read-xsd, read-xsd-components and read-yaml run in verdant_readers, the command of the readers project\u{a}"))))
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
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h308, _) => {
            h308
        }
    }, f112())) {
        Ok(Adt11::C1(List::cons(f112(), List::<Bytes>::nil())))
    } else {
        let m309 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f113(v0.clone())?)));
        match m309.uncons() {
            None => {
                let m310 = f114(v0.clone(), f113(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m310.uncons() {
                    None => {
                        f116(v0)
                    }
                    Some((v3, v4)) => {
                        Ok(Adt11::C4(f7(List::cons(v3, v4))))
                    }
                }
            }
            Some((v1, v2)) => {
                Ok(Adt11::C0(List::cons(v1, v2)))
            }
        }
    }
}

pub fn f70(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h311, _) => {
            h311
        }
    }, f117())) {
        Ok(Adt11::C1(List::cons(f117(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f118(f119(f117(), f120(match v0.clone() {
            Adt1::C0(_, h312, _) => {
                h312
            }
        }, f117())))?;
        let m313 = f45(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m313.uncons() {
            None => {
                let m314 = f121(v0.clone(), f122(f81(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f123(v2, v1.clone()))?;
                match m314 {
                    None => {
                        f124(v0, v1)
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
    let m315 = f97(v0.clone());
    match m315.uncons() {
        None => {
            let m316 = f98(f45(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f125(), List::<Bytes>::nil())))), f99(v0.clone())?);
            match m316.uncons() {
                None => {
                    Ok(Adt11::C3(f126(f127(v0.clone())?, List::cons((f128(), f129()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f130()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f131()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f132()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f133()), List::cons((f8(Str::lit("docs/catalog.md")), f134(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f135(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f136(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f137()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m321 = {
        let a318 = v0;
        let a319 = 0u64;
        let a320 = {
            let a317 = v1.clone();
            length_bytes(a317)
        };
        slice_bytes(a318, a319, a320)
    };
    match m321 {
        None => {
            false
        }
        Some(v2) => {
            let a322 = v2;
            let a323 = v1;
            equal(a322, a323)
        }
    }
}

pub fn f73(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f28(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f138(v3.clone(), v1.clone(), f139(v3, v1, false)?)
    } else {
        if f28(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f138(v3.clone(), v1.clone(), f140(v3, v1)?)
        } else {
            if f28(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f138(v3.clone(), v1.clone(), f139(v3, v1, true)?)
            } else {
                if f28(v0.clone(), f8(Str::lit("stage-encodings"))) {
                    f138(v3.clone(), v1.clone(), f141(v3, v1)?)
                } else {
                    if f28(v0.clone(), f8(Str::lit("stage-mappings"))) {
                        f138(v3.clone(), v1.clone(), f142(v3, v1, false)?)
                    } else {
                        if f28(v0, f8(Str::lit("stage-matrix"))) {
                            f138(v3.clone(), v1.clone(), f142(v3, v1, true)?)
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
        f143(v3, v1)
    } else {
        if f28(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f144(v3, v1)
        } else {
            if f28(v0.clone(), f8(Str::lit("admit-registry"))) {
                f145(v3, v1, v2)
            } else {
                if f28(v0.clone(), f8(Str::lit("ingest-authority"))) {
                    f146(v3, v1)
                } else {
                    if f28(v0.clone(), f8(Str::lit("ingest-index"))) {
                        f147(v3, v1)
                    } else {
                        if f28(v0.clone(), f8(Str::lit("merge-gate"))) {
                            f148(v3, v1)
                        } else {
                            if f28(v0.clone(), f8(Str::lit("propose-encoding"))) {
                                let m324 = v2;
                                match m324.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f66()))
                                    }
                                    Some((v4, v5)) => {
                                        f149(v3, v1, v4, v5)
                                    }
                                }
                            } else {
                                if f28(v0, f8(Str::lit("record-decision"))) {
                                    f150(v3, v1)
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
}

pub fn f75(v0: Bytes) -> R<bool> {
    f151(v0.clone(), 0u64, {
        let a325 = v0;
        length_bytes(a325)
    })
}

pub fn f76(p330: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m329 = p330;
    {
        let mut m329 = m329;
        loop {
            let step = {
                let v0 = m329;
                let m326 = v0;
                match m326.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h327, _) = v2;
                            h327
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h328) = v2;
                                h328
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f77(v0: Bytes) -> R<Option<Adt13>> {
    let m331 = f152(v0)?;
    match m331 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v1) => {
            Ok(f153(v1))
        }
    }
}

pub fn f78(v0: Option<Adt13>) -> Option<Bytes> {
    let m332 = v0;
    match m332 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m333 = v1;
            match m333 {
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

pub fn f79(p339: Adt13, v1: Bytes) -> Option<Adt13> {
    let m338 = p339;
    {
        let mut m338 = m338;
        loop {
            let step = {
                let v0 = m338;
                let m334 = v0;
                match m334 {
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
                    Adt13::C4(r335) => {
                        let v5 = (*r335).clone();
                        Ok::<Adt13, Option<Adt13>>(v5)
                    }
                    Adt13::C5 => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C6(_, _) => {
                        Err::<Adt13, Option<Adt13>>(None::<Adt13>)
                    }
                    Adt13::C7(v2, r336, r337) => {
                        let v3 = (*r336).clone();
                        let v4 = (*r337).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt13, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Adt13>>(v4)
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

pub fn f80(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m340 = v0;
    match m340.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m341 = f154(v2, v1.clone())?;
            match m341 {
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
        let a342 = v0.clone();
        length_bytes(a342)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h343, _) = f63(Fn0::F155(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h343
    };
    let v3: u64 = {
        let (h344, _) = {
            let (_, h345) = v2.clone();
            h345
        };
        h344
    };
    let v4: List<Bytes> = {
        let (_, h346) = {
            let (_, h347) = v2;
            h347
        };
        h346
    };
    let a348 = v3;
    let a349 = v1;
    if nat_lt(a348, a349) {
        Ok(f65(List::cons(f35(v0, v3, {
            let a350 = v1;
            let a351 = v3;
            nat_sub(a350, a351)
        }), v4)))
    } else {
        Ok(f65(v4))
    }
}

pub fn f82(v0: Adt1, v1: Bytes) -> R<List<Adt13>> {
    Ok(f156(f46(v0, v1)?))
}

pub fn f83(v0: List<Adt13>, v1: Bytes) -> List<Adt13> {
    let m352 = v0;
    match m352.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f157(v2, v1.clone()), f83(v3, v1))
        }
    }
}

pub fn f84(p357: List<Bytes>, p358: List<Bytes>) -> bool {
    let m356 = (p357, p358);
    {
        let mut m356 = m356;
        loop {
            let step = {
                let (v0, v1) = m356;
                let m353 = v0;
                match m353.uncons() {
                    None => {
                        let m354 = v1;
                        match m354.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m355 = v1;
                        match m355.uncons() {
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
                Ok(next) => { m356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<Adt13>) -> List<Bytes> {
    let m359 = v0;
    match m359.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m360 = f79(v1, f8(Str::lit("std")));
            match m360 {
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
    let a361 = f158(v0)?;
    let a362 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a361, a362))
}

pub fn f87(v0: List<Adt13>) -> Adt13 {
    let m363 = v0;
    match m363.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f87(v2)))
        }
    }
}

pub fn f88(v0: Adt12) -> Bytes {
    let m364 = v0;
    match m364 {
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
    let a366 = {
        let a365 = v2.clone();
        length_bytes(a365)
    };
    let a367 = 0u64;
    if nat_eq(a366, a367) {
        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m368 = f159(v0.clone(), f49())?;
        match m368 {
            None => {
                if f40(f41(f162(), match v0.clone() {
                    Adt1::C0(_, _, h369) => {
                        h369
                    }
                })) {
                    Ok(Adt11::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f162()))
                } else {
                    let m370 = f45(v0.clone(), List::cons(f163(), f164(f49())?));
                    match m370.uncons() {
                        None => {
                            Ok(Adt11::C3(f165(v0.clone(), f49(), v1.clone(), v2.clone(), f166(f53(v0, f163()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt11::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f43(f160(v3.clone())?, f161(v3, v2)))
            }
        }
    }
}

pub fn f92(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h371, _) = v2.clone();
        h371
    };
    let v4: u64 = {
        let (h372, _) = {
            let (_, h373) = v2.clone();
            h373
        };
        h372
    };
    let v5: List<Bytes> = {
        let (_, h374) = {
            let (_, h375) = v2;
            h375
        };
        h374
    };
    let m376 = f167(v0.clone(), v3);
    match m376 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a377 = v6;
            let a378 = v1;
            if equal(a377, a378) {
                Ok(Some(({
                    let a379 = v3;
                    let a380 = 1u64;
                    nat_add(a379, a380)?
                }, ({
                    let a381 = v3;
                    let a382 = 1u64;
                    nat_add(a381, a382)?
                }, List::cons(f35(v0, v4, {
                    let a383 = v3;
                    let a384 = v4;
                    nat_sub(a383, a384)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a385 = v3;
                    let a386 = 1u64;
                    nat_add(a385, a386)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f93(v0: Fn1, p390: List<Bytes>, p391: List<Bytes>) -> List<Bytes> {
    let m389 = (p390, p391);
    {
        let mut m389 = m389;
        loop {
            let step = {
                let (v1, v2) = m389;
                let m387 = v2;
                match m387.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c388: Fn1 = v0.clone();
                            c388.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m392 = f168(Fn2::F169(v1), None::<Bytes>, v0);
    match m392 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f96() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY JSONFILE ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))
}

pub fn f97(v0: Adt1) -> List<Bytes> {
    if f111(match v0.clone() {
        Adt1::C0(_, h393, _) => {
            h393
        }
    }, f170()) {
        if f111(match v0 {
            Adt1::C0(_, h394, _) => {
                h394
            }
        }, f171()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f171(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f170(), List::<Bytes>::nil())
    }
}

pub fn f98(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m395 = v0;
    match m395.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f98(v3, v1))
        }
    }
}

pub fn f99(v0: Adt1) -> R<List<Bytes>> {
    Ok(f45(v0.clone(), List::cons(f172(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f109(v0)?)))))
}

pub fn f100(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f173(v0.clone())?, f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f115(f82(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m396 = v0;
    match m396.uncons() {
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
    let m397 = v0;
    match m397.uncons() {
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
    let m398 = v0;
    match m398.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f98(f174(f156(f79(v1, f8(Str::lit("defers"))))), f103(v2))
        }
    }
}

pub fn f104(v0: List<Adt13>) -> List<Bytes> {
    let m399 = v0;
    match m399.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m400 = f79(v1, f8(Str::lit("std")));
            match m400 {
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
    let m401 = v0;
    match m401.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f126(f175(v2.clone()), f176(f177(v0, v1.clone(), v2)?));
    f178(f118(f179(v1))?, v3.clone(), {
        let a403 = {
            let a402 = v3;
            length_list(a402)
        };
        let a404 = 1u64;
        nat_add(a403, a404)?
    })
}

pub fn f107(v0: Adt14, v1: List<Adt13>) -> List<Adt13> {
    let m405 = v1;
    match m405.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            if if f180(v2.clone(), f8(Str::lit("subject"))) {
                f181(v0.clone(), v2.clone())
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
    let m406 = v1;
    match m406.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v2, v3)) => {
            let m407 = f182(f81(f53(v0.clone(), v2.clone()))?, f72(v2, f171()))?;
            match m407 {
                None => {
                    f108(v0, v3)
                }
                Some(v4) => {
                    Ok(f183(v4, f108(v0, v3)?))
                }
            }
        }
    }
}

pub fn f109(v0: Adt1) -> R<List<Bytes>> {
    Ok(f98(f118(f184(f170(), f120(match v0.clone() {
        Adt1::C0(_, h408, _) => {
            h408
        }
    }, f170())))?, f118(f184(f171(), f120(match v0 {
        Adt1::C0(_, h409, _) => {
            h409
        }
    }, f171())))?))
}

pub fn f110(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f111(p413: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m412 = p413;
    {
        let mut m412 = m412;
        loop {
            let step = {
                let v0 = m412;
                let m410 = v0;
                match m410.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h411, _) = v2;
                            h411
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m412 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f113(v0: Adt1) -> R<List<Bytes>> {
    f118(f184(f112(), f120(match v0 {
        Adt1::C0(_, h414, _) => {
            h414
        }
    }, f112())))
}

pub fn f114(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m415 = v1;
    match m415.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f98(if f41(v4.clone(), v3.clone()) {
                f185(v4.clone(), f81(f53(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f186(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f114(v0, v5, v2, v3)?))
        }
    }
}

pub fn f115(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m416 = v0;
    match m416.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f47(v2, v1.clone()), f115(v3, v1))
        }
    }
}

pub fn f116(v0: Adt1) -> R<Adt11> {
    let m417 = f45(v0.clone(), f187());
    match m417.uncons() {
        None => {
            let m418 = f188(v0.clone(), f187())?;
            match m418.uncons() {
                None => {
                    let m421 = f189(match v0.clone() {
                        Adt1::C0(_, h419, _) => {
                            h419
                        }
                    }, f98(f190(), f191(match v0.clone() {
                        Adt1::C0(_, h420, _) => {
                            h420
                        }
                    }, false)));
                    match m421.uncons() {
                        None => {
                            let m423 = f45(v0.clone(), f191(match v0.clone() {
                                Adt1::C0(_, h422, _) => {
                                    h422
                                }
                            }, true));
                            match m423.uncons() {
                                None => {
                                    let m424 = f45(v0.clone(), f192());
                                    match m424.uncons() {
                                        None => {
                                            let m425 = f193(v0.clone())?;
                                            match m425 {
                                                None => {
                                                    let m427 = f194(v0.clone(), f191(match v0.clone() {
                                                        Adt1::C0(_, h426, _) => {
                                                            h426
                                                        }
                                                    }, true))?;
                                                    match m427 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f195(Adt16::C0(Adt15::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f173(v0.clone())?, f115(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f174(f82(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f191(match v0 {
                                                                Adt1::C0(_, h428, _) => {
                                                                    h428
                                                                }
                                                            }, true)))?;
                                                            let (h429, _) = v13;
                                                            if h429 {
                                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h430) = v13;
                                                                    h430
                                                                }))
                                                            } else {
                                                                Ok(Adt11::C4({
                                                                    let (_, h431) = v13;
                                                                    h431
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

pub fn f117() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f118(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f196(f93(Fn1::F197, List::<Bytes>::nil(), f198(v0)?)))
}

pub fn f119(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m432 = v1;
    match m432.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f199(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f200(v0.clone(), v2), f119(v0, v3))
            } else {
                f119(v0, v3)
            }
        }
    }
}

pub fn f120(p437: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m436 = p437;
    {
        let mut m436 = m436;
        loop {
            let step = {
                let v0 = m436;
                let m433 = v0;
                match m433.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f28({
                            let (h434, _) = v2;
                            h434
                        }, v1.clone()) {
                            let (_, h435) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h435)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt1, v1: List<List<Bytes>>, p440: List<Bytes>) -> R<Option<Bytes>> {
    let m439 = p440;
    {
        let mut m439 = m439;
        loop {
            let step = {
                let v2 = m439;
                let m438 = v2;
                match m438.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f201(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f122(p443: List<Bytes>, p444: List<Bytes>, p445: List<List<Bytes>>) -> List<List<Bytes>> {
    let m442 = (p443, (p444, p445));
    {
        let mut m442 = m442;
        loop {
            let step = {
                let (v0, (v1, v2)) = m442;
                let m441 = v0;
                match m441.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f202(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f28(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f202(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m446 = v0;
    match m446.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f123(v3, v1)
            } else {
                List::cons(v2, f123(v3, v1))
            }
        }
    }
}

pub fn f124(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h447, _) => {
            h447
        }
    }, f203())) {
        Ok(Adt11::C1(List::cons(f203(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f123(f118(f119(f203(), f120(match v0.clone() {
            Adt1::C0(_, h448, _) => {
                h448
            }
        }, f203())))?, v1.clone());
        let m449 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m449.uncons() {
            None => {
                let v5: List<Bytes> = f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m450 = f45(v0.clone(), f204(v0.clone(), v2.clone(), v5.clone())?);
                match m450.uncons() {
                    None => {
                        let m451 = f205(v0.clone(), v2, v5)?;
                        match m451 {
                            None => {
                                f206(v0, v1)
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

pub fn f125() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f126(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m452 = v0;
    match m452.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m453 = f82(v0.clone(), f125())?;
    match m453.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f207(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f128() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f129() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f130() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f208(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f131() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f208(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f132() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f133() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f134(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f100(v0.clone())?;
    let v2: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt13> = f107(v1.clone(), f108(v0.clone(), f109(v0.clone())?)?);
    let v4: List<List<Bytes>> = f209(f210(f53(v0, f172()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f211(v4.clone(), f212(v4), f177(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f106(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f135(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f213(f100(v0.clone())?, f108(v0.clone(), f109(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f136(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f214(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f215(f82(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f137() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f138(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m454 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m454.uncons() {
        None => {
            if f216(f81(f53(v0, v1))?)? {
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

pub fn f139(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m455 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m455.uncons() {
        None => {
            let v5: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
            if f40(f199(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f217(v5)?
                } else {
                    f218(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f200(f219(), f220(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f200(f219(), f220(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f221()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f140(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m456 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m456.uncons() {
        None => {
            let m457 = f222(f81(f53(v0.clone(), v1.clone()))?)?;
            match m457 {
                None => {
                    Ok(Adt11::C4(f223()))
                }
                Some(v4) => {
                    let m458 = f224(f81(f53(v0, v1))?)?;
                    match m458 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f225(v5)))
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

pub fn f141(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m459 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m459.uncons() {
        None => {
            let m460 = f226(f81(f53(v0, v1))?)?;
            match m460 {
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

pub fn f142(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m461 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m461.uncons() {
        None => {
            let v5: Bytes = f200(if v2 {
                f171()
            } else {
                f170()
            }, f220(v1.clone())?);
            if f40(f199(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m462 = f182(f81(f53(v0.clone(), v1.clone()))?, v2)?;
                match m462 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a464 = {
                            let a463 = v6;
                            length_list(a463)
                        };
                        let a465 = 0u64;
                        if nat_eq(a464, a465) {
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

pub fn f143(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m466 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m466.uncons() {
        None => {
            let v4: List<Bytes> = f81(f53(v0.clone(), v1))?;
            let m467 = f227(v4.clone(), f8(Str::lit("requirement")))?;
            match m467 {
                None => {
                    Ok(Adt11::C4(f221()))
                }
                Some(v5) => {
                    let m468 = f227(v4, f8(Str::lit("admission")))?;
                    match m468 {
                        None => {
                            Ok(Adt11::C4(f221()))
                        }
                        Some(v6) => {
                            let v7: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f98(f115(v5.clone(), f8(Str::lit("id"))), f115(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f118(f98(f115(v7.clone(), f8(Str::lit("id"))), f115(v8.clone(), f8(Str::lit("construct")))))?;
                            let m469 = f228(f104(v6.clone()), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m469 {
                                None => {
                                    let a471 = {
                                        let a470 = v9.clone();
                                        length_list(a470)
                                    };
                                    let a472 = 0u64;
                                    if nat_eq(a471, a472) {
                                        Ok(Adt11::C4(f221()))
                                    } else {
                                        if f40({
                                            let a475 = {
                                                let a473 = f118(f98(v9.clone(), v10.clone()))?;
                                                length_list(a473)
                                            };
                                            let a476 = {
                                                let a474 = f98(v9.clone(), v10);
                                                length_list(a474)
                                            };
                                            nat_eq(a475, a476)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f230(f8(Str::lit("model/requirements.json")), f183(v7, v5))?, List::cons(f230(f8(Str::lit("model/admissions.json")), f183(v8, v6))?, List::cons(f231(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f95(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f229(v11))
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

pub fn f144(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m477 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m477.uncons() {
        None => {
            let m478 = f232(v0.clone(), v1.clone())?;
            match m478 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m479 = f233(v4.clone(), f8(Str::lit("schema")));
                    match m479 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f234(v0, f235(v5.clone())?, v5, f236(v4.clone(), f8(Str::lit("name"))), f237(f238(f233(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f145(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt11> {
    let m480 = v2;
    match m480.uncons() {
        None => {
            Ok(Adt11::C4(f66()))
        }
        Some((v3, v4)) => {
            let m481 = v4;
            match m481.uncons() {
                None => {
                    Ok(Adt11::C4(f66()))
                }
                Some((v5, v6)) => {
                    let m482 = v6;
                    match m482.uncons() {
                        None => {
                            Ok(Adt11::C4(f66()))
                        }
                        Some((v7, v8)) => {
                            let m483 = v8.clone();
                            match m483.uncons() {
                                None => {
                                    Ok(Adt11::C4(f66()))
                                }
                                Some((v9, v10)) => {
                                    let m484 = v10;
                                    match m484.uncons() {
                                        None => {
                                            Ok(Adt11::C4(f66()))
                                        }
                                        Some((v11, _)) => {
                                            if f40(f72(v9, f8(Str::lit("read-")))) {
                                                Ok(Adt11::C4(f8(Str::lit("name the verdant_readers command that printed JSONFILE: admit-registry AUTHORITY JSONFILE ROW CONSTRUCT|none read-OP ARTIFACT\u{a}"))))
                                            } else {
                                                let m485 = f45(v0.clone(), List::cons(f239(v1.clone()), List::cons(v3.clone(), List::cons(v11.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f28(v7.clone(), f8(Str::lit("none"))) {
                                                    List::<Bytes>::nil()
                                                } else {
                                                    List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v7.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                                })))));
                                                match m485.uncons() {
                                                    None => {
                                                        if f40(f56(v0.clone(), f239(v1.clone()))) {
                                                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f239(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                        } else {
                                                            let m486 = f46(v0.clone(), f239(v1.clone()))?;
                                                            match m486 {
                                                                None => {
                                                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f239(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                                                                }
                                                                Some(v15) => {
                                                                    let v17: Bytes = {
                                                                        let m487 = f79(v15, f8(Str::lit("source")));
                                                                        match m487 {
                                                                            None => {
                                                                                Bytes::lit(&[])
                                                                            }
                                                                            Some(v16) => {
                                                                                f47(v16, f8(Str::lit("sha256")))
                                                                            }
                                                                        }
                                                                    };
                                                                    let v18: Bytes = f240(f53(v0.clone(), v11.clone()))?;
                                                                    if f40(f28(v18.clone(), v17.clone())) {
                                                                        Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v11, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v18, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v17, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                                    } else {
                                                                        f241(v0, v1, v3, v5, v7, v8)
                                                                    }
                                                                }
                                                            }
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
            }
        }
    }
}

pub fn f146(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m488 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m488.uncons() {
        None => {
            let m489 = f46(v0.clone(), v1.clone())?;
            match m489 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f40(f28(f47(v4.clone(), f8(Str::lit("schema"))), f242())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt13 = {
                            let m490 = f79(v4.clone(), f8(Str::lit("source")));
                            match m490 {
                                None => {
                                    Adt13::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f78(f79(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f200(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f200(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m491 = v8;
                        match m491 {
                            None => {
                                f243(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f40(f28(f47(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f243(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f200(v9, f220(f47(v7.clone(), f8(Str::lit("url"))))?);
                                    if f40(f41(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h492) => {
                                            h492
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f47(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m493 = f45(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m493.uncons() {
                                            None => {
                                                if f28(f240(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f243(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f147(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m494 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m494.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f210(v4.clone())?;
            if f40(f84(f244(v5.clone()), f245())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f246(Str::lit("id"), v6.clone())?, List::cons(f246(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f246(Str::lit("artifact"), f172())?, List::cons(f246(Str::lit("checksum"), f240(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f172(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f247(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f231(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f95(f212(f209(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f148(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m495 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m495.uncons() {
        None => {
            let m496 = f46(v0, v1)?;
            match m496 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f248(f156(Some(v4))) {
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

pub fn f149(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m497 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m497.uncons() {
        None => {
            let m498 = f232(v0.clone(), v1.clone())?;
            match m498 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f234(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f150(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m499 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f125(), List::<Bytes>::nil())));
    match m499.uncons() {
        None => {
            let m500 = f249(f81(f53(v0.clone(), v1))?)?;
            match m500 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt13> = f82(v0, f125())?;
                    let v6: List<Bytes> = f115(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f118(f115(v5.clone(), f8(Str::lit("at"))))?;
                    let a502 = {
                        let a501 = v6.clone();
                        length_list(a501)
                    };
                    let a503 = 0u64;
                    if if nat_eq(a502, a503) {
                        true
                    } else {
                        f40({
                            let a506 = {
                                let a504 = f118(f98(v6.clone(), v7.clone()))?;
                                length_list(a504)
                            };
                            let a507 = {
                                let a505 = f98(v6.clone(), v7);
                                length_list(a505)
                            };
                            nat_eq(a506, a507)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f230(f125(), f183(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f95(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f151(v0: Bytes, p561: u64, p562: u64) -> R<bool> {
    let m560 = (p561, p562);
    {
        let mut m560 = m560;
        loop {
            let step = {
                let (v1, v2) = m560;
                let m508 = v2;
                if m508 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m508 - 1;
                    let m511 = {
                        let a509 = v0.clone();
                        let a510 = v1;
                        index_bytes(a509, a510)
                    };
                    match m511 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f250({
                                let v5: u8 = v4;
                                let a556 = {
                                    let a532 = {
                                        let a520 = {
                                            let a514 = {
                                                let a512 = v5;
                                                let a513 = 128u8;
                                                fixed_u8::bit_and(a512, a513)
                                            };
                                            let a515 = 128u8;
                                            if equal(a514, a515) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a521 = {
                                            let a518 = {
                                                let a516 = v5;
                                                let a517 = 64u8;
                                                fixed_u8::bit_and(a516, a517)
                                            };
                                            let a519 = 64u8;
                                            if equal(a518, a519) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a520, a521)?
                                    };
                                    let a533 = {
                                        let a530 = {
                                            let a524 = {
                                                let a522 = v5;
                                                let a523 = 32u8;
                                                fixed_u8::bit_and(a522, a523)
                                            };
                                            let a525 = 32u8;
                                            if equal(a524, a525) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a531 = {
                                            let a528 = {
                                                let a526 = v5;
                                                let a527 = 16u8;
                                                fixed_u8::bit_and(a526, a527)
                                            };
                                            let a529 = 16u8;
                                            if equal(a528, a529) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a530, a531)?
                                    };
                                    nat_add(a532, a533)?
                                };
                                let a557 = {
                                    let a554 = {
                                        let a542 = {
                                            let a536 = {
                                                let a534 = v5;
                                                let a535 = 8u8;
                                                fixed_u8::bit_and(a534, a535)
                                            };
                                            let a537 = 8u8;
                                            if equal(a536, a537) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a543 = {
                                            let a540 = {
                                                let a538 = v5;
                                                let a539 = 4u8;
                                                fixed_u8::bit_and(a538, a539)
                                            };
                                            let a541 = 4u8;
                                            if equal(a540, a541) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a542, a543)?
                                    };
                                    let a555 = {
                                        let a552 = {
                                            let a546 = {
                                                let a544 = v5;
                                                let a545 = 2u8;
                                                fixed_u8::bit_and(a544, a545)
                                            };
                                            let a547 = 2u8;
                                            if equal(a546, a547) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a553 = {
                                            let a550 = {
                                                let a548 = v5;
                                                let a549 = 1u8;
                                                fixed_u8::bit_and(a548, a549)
                                            };
                                            let a551 = 1u8;
                                            if equal(a550, a551) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a552, a553)?
                                    };
                                    nat_add(a554, a555)?
                                };
                                nat_add(a556, a557)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a558 = v1;
                                    let a559 = 1u64;
                                    nat_add(a558, a559)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
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

pub fn f152(v0: Bytes) -> R<Option<Adt17>> {
    let v1: Bytes = f251(v0)?;
    let m566 = f252(v1.clone(), 0u64, {
        let a564 = {
            let a563 = v1.clone();
            length_bytes(a563)
        };
        let a565 = 2u64;
        nat_add(a564, a565)?
    }, 0u64)?;
    match m566 {
        None => {
            Ok(None::<Adt17>)
        }
        Some(v2) => {
            let a569 = {
                let (_, h567) = v2;
                h567
            };
            let a570 = {
                let a568 = v1;
                length_bytes(a568)
            };
            if nat_eq(a569, a570) {
                Ok(Some({
                    let (h571, _) = v2;
                    h571
                }))
            } else {
                Ok(None::<Adt17>)
            }
        }
    }
}

pub fn f153(v0: Adt17) -> Option<Adt13> {
    let m572 = v0;
    match m572 {
        Adt17::C0 => {
            Some(Adt13::C3(std::rc::Rc::new(Adt13::C5)))
        }
        Adt17::C1(_, _) => {
            None::<Adt13>
        }
        Adt17::C2(v3) => {
            Some(Adt13::C0(v3))
        }
        Adt17::C3(v4, v5) => {
            Some(Adt13::C1(v4, v5))
        }
        Adt17::C4(_, _, _) => {
            None::<Adt13>
        }
        Adt17::C5(v9) => {
            Some(Adt13::C2(v9))
        }
        Adt17::C6(r579) => {
            let v10 = (*r579).clone();
            let m573 = f153(v10);
            match m573 {
                None => {
                    None::<Adt13>
                }
                Some(v11) => {
                    Some(Adt13::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt17::C7(r580) => {
            let v12 = (*r580).clone();
            let m574 = f153(v12);
            match m574 {
                None => {
                    None::<Adt13>
                }
                Some(v13) => {
                    Some(Adt13::C4(std::rc::Rc::new(v13)))
                }
            }
        }
        Adt17::C8 => {
            Some(Adt13::C5)
        }
        Adt17::C9(r581, r582) => {
            let v14 = (*r581).clone();
            let v15 = (*r582).clone();
            let m575 = f153(v14);
            match m575 {
                None => {
                    None::<Adt13>
                }
                Some(v16) => {
                    let m576 = f153(v15);
                    match m576 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v17) => {
                            Some(Adt13::C6(std::rc::Rc::new(v16), std::rc::Rc::new(v17)))
                        }
                    }
                }
            }
        }
        Adt17::C10(v18, r583, r584) => {
            let v19 = (*r583).clone();
            let v20 = (*r584).clone();
            let m577 = f153(v19);
            match m577 {
                None => {
                    None::<Adt13>
                }
                Some(v21) => {
                    let m578 = f153(v20);
                    match m578 {
                        None => {
                            None::<Adt13>
                        }
                        Some(v22) => {
                            Some(Adt13::C7(v18, std::rc::Rc::new(v21), std::rc::Rc::new(v22)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f154(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), v1) {
        let m585 = f33(v0, 34u8)?;
        match m585.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m586 = v3;
                match m586.uncons() {
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

pub fn f155(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f253(v0, v1)
}

pub fn f156(v0: Option<Adt13>) -> List<Adt13> {
    let m587 = v0;
    match m587 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f254(v1)
        }
    }
}

pub fn f157(v0: Adt13, v1: Bytes) -> Adt13 {
    let m588 = v0;
    match m588 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r589) => {
            let v10 = (*r589).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r590) => {
            let v5 = (*r590).clone();
            Adt13::C4(std::rc::Rc::new(f157(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r591, r592) => {
            let v11 = (*r591).clone();
            let v12 = (*r592).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r593, r594) => {
            let v3 = (*r593).clone();
            let v4 = (*r594).clone();
            if f28(v2.clone(), f8(Str::lit("std"))) {
                Adt13::C7(v2, std::rc::Rc::new(f255(v3, v1.clone())), std::rc::Rc::new(f157(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f157(v4, v1)))
            }
        }
    }
}

pub fn f158(v0: Adt13) -> R<Bytes> {
    let m595 = v0;
    match m595 {
        Adt13::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt13::C1(v2, v3) => {
            f256(v2, v3)
        }
        Adt13::C2(v4) => {
            f257(v4)
        }
        Adt13::C3(r596) => {
            let v5 = (*r596).clone();
            Ok(f258(f259(v5)?))
        }
        Adt13::C4(r597) => {
            let v6 = (*r597).clone();
            f260(f261(v6)?)
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

pub fn f159(v0: Adt1, p601: List<Adt12>) -> R<Option<Adt12>> {
    let m600 = p601;
    {
        let mut m600 = m600;
        loop {
            let step = {
                let v1 = m600;
                let m598 = v1;
                match m598.uncons() {
                    None => {
                        Err::<List<Adt12>, R<Option<Adt12>>>(Ok(None::<Adt12>))
                    }
                    Some((v2, v3)) => {
                        if f41(f160(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h599) => {
                                h599
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
                Ok(next) => { m600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: Adt12) -> R<Bytes> {
    Ok(f200(f262(), f220(f263(v0))?))
}

pub fn f161(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f263(v0), List::<Bytes>::nil())))))
}

pub fn f162() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f163() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f164(v0: List<Adt12>) -> R<List<Bytes>> {
    let m602 = v0;
    match m602.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f160(v1)?, f164(v2)?))
        }
    }
}

pub fn f165(v0: Adt1, v1: List<Adt12>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m603 = v1;
    match m603.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f57(f58(v5.clone(), f50(v2.clone()))), f264(v5.clone(), v2.clone(), v3.clone(), f240(f53(v0.clone(), f160(v5)?))?, v4.clone())?), f165(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f166(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f265(v0.clone())?;
    let v2: u64 = f266(v0.clone(), v1)?;
    Ok(f35(v0, v2, {
        let a604 = v1;
        let a605 = v2;
        nat_sub(a604, a605)
    }))
}

pub fn f167(v0: Bytes, v1: u64) -> Option<u8> {
    let a606 = v0;
    let a607 = v1;
    index_bytes(a606, a607)
}

pub fn f168(v0: Fn2, p611: Option<Bytes>, p612: List<Bytes>) -> Option<Bytes> {
    let m610 = (p611, p612);
    {
        let mut m610 = m610;
        loop {
            let step = {
                let (v1, v2) = m610;
                let m608 = v2;
                match m608.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c609: Fn2 = v0.clone();
                            c609.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m613 = v1;
    match m613 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a616 = {
                    let a614 = v3;
                    let a615 = v0;
                    append_bytes(a614, a615)
                };
                let a617 = v2;
                append_bytes(a616, a617)
            })
        }
    }
}

pub fn f170() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f171() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f172() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f173(v0: Adt1) -> R<List<Bytes>> {
    let m618 = f210(f53(v0, f172()))?;
    match m618.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f212(v2))
        }
    }
}

pub fn f174(v0: List<Adt13>) -> List<Bytes> {
    let m619 = v0;
    match m619.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m620 = f78(Some(v1));
            match m620 {
                None => {
                    f174(v2)
                }
                Some(v3) => {
                    List::cons(v3, f174(v2))
                }
            }
        }
    }
}

pub fn f175(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m621 = v0;
    match m621.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f47(v1.clone(), f8(Str::lit("subject"))), f47(v1, f8(Str::lit("object")))), f175(v2))
        }
    }
}

pub fn f176(v0: List<Adt18>) -> List<(Bytes, Bytes)> {
    let m622 = v0;
    match m622.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt18::C0(h623, _, _) => {
                    h623
                }
            }, match v1 {
                Adt18::C0(_, _, h624) => {
                    h624
                }
            }), f176(v2))
        }
    }
}

pub fn f177(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Adt18>> {
    let v3: List<(Bytes, Bytes)> = f267(v1);
    let v4: List<(Bytes, Bytes)> = f268(v0, v2);
    f269(v3.clone(), f126(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f178(p631: List<Bytes>, v1: List<(Bytes, Bytes)>, p632: u64) -> R<List<Bytes>> {
    let m630 = (p631, p632);
    {
        let mut m630 = m630;
        loop {
            let step = {
                let (v0, v2) = m630;
                let m625 = v2;
                if m625 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m625 - 1;
                    let v4: List<Bytes> = f118(f98(v0.clone(), f270(v0.clone(), v1.clone())))?;
                    let a628 = {
                        let a626 = v4.clone();
                        length_list(a626)
                    };
                    let a629 = {
                        let a627 = v0.clone();
                        length_list(a627)
                    };
                    if nat_eq(a628, a629) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m630 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: List<Adt13>) -> List<Bytes> {
    let m633 = v0;
    match m633.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m634 = f79(v1, f8(Str::lit("std")));
            match m634 {
                None => {
                    f179(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3.clone(), f8(Str::lit("authority"))), if f180(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f47(v3, f8(Str::lit("row"))), f179(v2))
                    } else {
                        f179(v2)
                    })
                }
            }
        }
    }
}

pub fn f180(v0: Adt13, v1: Bytes) -> bool {
    f40({
        let a636 = {
            let a635 = f47(v0, v1);
            length_bytes(a635)
        };
        let a637 = 0u64;
        nat_eq(a636, a637)
    })
}

pub fn f181(v0: Adt14, v1: Adt13) -> bool {
    if f271(v0.clone(), v1.clone(), f47(v1.clone(), f8(Str::lit("subject")))) {
        f271(v0, v1.clone(), f47(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f182(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt13>>> {
    let m638 = v0;
    match m638.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f166(v2)?;
            let a640 = {
                let a639 = v4.clone();
                length_bytes(a639)
            };
            let a641 = 0u64;
            if nat_eq(a640, a641) {
                f182(v3, v1)
            } else {
                let m642 = f77(v4)?;
                match m642 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f272(v5.clone())
                        } else {
                            f273(v5.clone())
                        } {
                            let m643 = f182(v3, v1)?;
                            match m643 {
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

pub fn f183(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m644 = v0;
    match m644.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f183(v3, v1))
        }
    }
}

pub fn f184(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m645 = v1;
    match m645.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f199(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f200(v0.clone(), v2), f184(v0, v3))
            } else {
                f184(v0, v3)
            }
        }
    }
}

pub fn f185(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m646 = v1;
    match m646.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f98({
                let a648 = {
                    let a647 = f166(v4.clone())?;
                    length_bytes(a647)
                };
                let a649 = 0u64;
                if nat_eq(a648, a649) {
                    List::<Bytes>::nil()
                } else {
                    f274(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f185(v0, v5, v2, v3)?))
        }
    }
}

pub fn f186(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f187() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f172(), List::<Bytes>::nil())))))))
}

pub fn f188(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m650 = v1;
    match m650.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f56(v0.clone(), v2.clone()) {
                f199(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m651 = f46(v0.clone(), v2.clone())?;
                match m651 {
                    None => {
                        Ok(List::cons(v2, f188(v0, v3)?))
                    }
                    Some(_) => {
                        f188(v0, v3)
                    }
                }
            } else {
                f188(v0, v3)
            }
        }
    }
}

pub fn f189(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m652 = v1;
    match m652.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f111(v0.clone(), v2.clone()) {
                f189(v0, v3)
            } else {
                List::cons(v2, f189(v0, v3))
            }
        }
    }
}

pub fn f190() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f191(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m653 = v0;
    match m653.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f98(f275({
                let (h654, _) = v2;
                h654
            }, {
                let (_, h655) = v2;
                h655
            }, v1), f191(v3, v1))
        }
    }
}

pub fn f192() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f193(v0: Adt1) -> R<Option<Bytes>> {
    f276(f277(v0, f192())?)
}

pub fn f194(v0: Adt1, p659: List<Bytes>) -> R<Option<Bytes>> {
    let m658 = p659;
    {
        let mut m658 = m658;
        loop {
            let step = {
                let v1 = m658;
                let m656 = v1;
                match m656.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m657 = f278(f53(v0.clone(), v2.clone()), f279())?;
                        match m657 {
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
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h660, _, _, _, _) => {
            h660
        }
    };
    let v2: Adt19 = f280(v1.clone())?;
    let v3: List<Bytes> = f281(v1.clone());
    let v4: List<Bytes> = f115(match v0.clone() {
        Adt16::C0(_, _, h661, _, _) => {
            h661
        }
    }, f8(Str::lit("path")));
    let m663 = f282(v2, List::<u64>::nil(), f283(v1.clone(), match v0.clone() {
        Adt16::C0(_, h662, _, _, _) => {
            h662
        }
    }, 0u64)?);
    match m663 {
        None => {
            if f285(f286(match v0.clone() {
                Adt16::C0(_, _, _, _, h665) => {
                    h665
                }
            }, f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h666) => {
                    h666
                }
            }, v4.clone()))?, f286(v4.clone(), f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h667) => {
                    h667
                }
            }, v4.clone()))?, f287(match v0.clone() {
                Adt16::C0(_, _, h668, _, _) => {
                    h668
                }
            })) {
                let m672 = f288(f286(match v0.clone() {
                    Adt16::C0(_, _, _, h669, _) => {
                        h669
                    }
                }, f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h670, _) => {
                        h670
                    }
                }))?, f286(v3.clone(), f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h671, _) => {
                        h671
                    }
                }))?);
                match m672 {
                    None => {
                        let m675 = f289(f115(match v0 {
                            Adt16::C0(_, h674, _, _, _) => {
                                h674
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m675 {
                            None => {
                                let m676 = f289(v4, List::<Bytes>::nil());
                                match m676 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f284(f98(v3, match v0 {
                            Adt16::C0(_, _, _, h673, _) => {
                                h673
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f284(match v1 {
                Adt15::C0(_, _, _, _, h664) => {
                    h664
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f196(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F290, List::<Bytes>::nil(), v0)
}

pub fn f197(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f291(v0, v1)
}

pub fn f198(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f292(f293(f294(v0))?))
}

pub fn f199(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a677 = v0.clone();
        length_bytes(a677)
    };
    let v3: u64 = {
        let a678 = v1.clone();
        length_bytes(a678)
    };
    let a679 = v3;
    let a680 = v2;
    if nat_le(a679, a680) {
        let m686 = {
            let a683 = v0;
            let a684 = {
                let a681 = v2;
                let a682 = v3;
                nat_sub(a681, a682)
            };
            let a685 = v3;
            slice_bytes(a683, a684, a685)
        };
        match m686 {
            None => {
                false
            }
            Some(v4) => {
                let a687 = v4;
                let a688 = v1;
                equal(a687, a688)
            }
        }
    } else {
        false
    }
}

pub fn f200(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f201(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m689 = f46(v0, v2)?;
    match m689 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f295(v1, f47(v3.clone(), f8(Str::lit("id"))))?;
            let m690 = v4.clone();
            match m690.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m692 = f78(f79({
                        let m691 = f79(v3.clone(), f8(Str::lit("source")));
                        match m691 {
                            None => {
                                Adt13::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m692 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f40(f41(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f296(f297(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f202(p695: List<Bytes>, p696: List<Bytes>) -> List<Bytes> {
    let m694 = (p695, p696);
    {
        let mut m694 = m694;
        loop {
            let step = {
                let (v0, v1) = m694;
                let m693 = v0;
                match m693.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f203() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f204(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m697 = v1;
    match m697.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt17 = f298(v0.clone(), v3)?;
            if f41(f299(f236(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f300(v5)?, f204(v0, v4, v2)?))
            } else {
                f204(v0, v4, v2)
            }
        }
    }
}

pub fn f205(v0: Adt1, p701: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m700 = p701;
    {
        let mut m700 = m700;
        loop {
            let step = {
                let v1 = m700;
                let m698 = v1;
                match m698.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt17 = f298(v0.clone(), v3.clone())?;
                        if if f41(f299(f236(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f28(f235({
                                let m699 = f233(v5.clone(), f8(Str::lit("schema")));
                                match m699 {
                                    None => {
                                        Adt17::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f53(v0.clone(), f300(v5)?))
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
                Ok(next) => { m700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f111(match v0.clone() {
        Adt1::C0(_, h702, _) => {
            h702
        }
    }, f219())) {
        Ok(Adt11::C1(List::cons(f219(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f123(f118(f301(f120(match v0.clone() {
            Adt1::C0(_, h703, _) => {
                h703
            }
        }, f219())))?, v1);
        let m704 = f45(v0.clone(), List::cons(f125(), List::cons(f172(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m704.uncons() {
            None => {
                let v5: (bool, Bytes) = f302(v0.clone(), v2)?;
                let (h705, _) = v5;
                if h705 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h706) = v5;
                        h706
                    }))
                } else {
                    let m707 = f97(v0.clone());
                    match m707.uncons() {
                        None => {
                            let m708 = f99(v0.clone())?;
                            match m708.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h709) = v5;
                                        h709
                                    }, List::cons(f7(f118(f303(f100(v0.clone())?, f108(v0.clone(), f109(v0.clone())?)?))?), List::cons(f304(v0)?, List::<Bytes>::nil()))))))
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

pub fn f207(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f305(f82(v0, f125())?)?)))
}

pub fn f208() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f209(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m710 = v0;
    match m710.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f210(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt20 = f306(v0.clone(), 0u64, {
        let a711 = v0;
        length_bytes(a711)
    }, Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt20 = {
        let a714 = {
            let a713 = match v1.clone() {
                Adt20::C0(_, _, h712, _, _) => {
                    h712
                }
            };
            length_bytes(a713)
        };
        let a715 = 0u64;
        if if nat_eq(a714, a715) {
            let m717 = match v1.clone() {
                Adt20::C0(_, _, _, h716, _) => {
                    h716
                }
            };
            match m717.uncons() {
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
            f307(v1)
        }
    };
    Ok(f308(match v4 {
        Adt20::C0(_, _, _, _, h718) => {
            h718
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f211(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt18>, v3: List<Adt13>, v4: List<Bytes>) -> List<Bytes> {
    let m719 = v1;
    match m719.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f98(f309(v5.clone(), v2.clone()), f310(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f311(v0.clone(), v5.clone()), List::cons(if f41(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a721 = {
                    let a720 = v7.clone();
                    length_list(a720)
                };
                let a722 = 0u64;
                if nat_eq(a721, a722) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f95(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f211(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f212(v0: List<List<Bytes>>) -> List<Bytes> {
    let m723 = v0;
    match m723.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m724 = v1;
            match m724.uncons() {
                None => {
                    f212(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f212(v2))
                }
            }
        }
    }
}

pub fn f213(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m725 = v1;
    match m725.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f180(v2.clone(), f8(Str::lit("domain"))) {
                f312(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f47(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m726 = f79(v2.clone(), f8(Str::lit("role")));
                    match m726 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f47(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f47(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f313(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m727 = f79(v2.clone(), f8(Str::lit("origin")));
                    match m727 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f47(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m728 = f79(v2, f8(Str::lit("evidence")));
                    match m728 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f47(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f213(v0, v3))
            } else {
                f213(v0, v3)
            }
        }
    }
}

pub fn f214(v0: List<Adt13>) -> List<Bytes> {
    let m729 = v0;
    match m729.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f47(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f47(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f47(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f214(v2))
        }
    }
}

pub fn f215(v0: List<Adt13>) -> List<Bytes> {
    let m730 = v0;
    match m730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f47(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f314(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f215(v2))
        }
    }
}

pub fn f216(p736: List<Bytes>) -> R<bool> {
    let m735 = p736;
    {
        let mut m735 = m735;
        loop {
            let step = {
                let v0 = m735;
                let m731 = v0;
                match m731.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a733 = {
                            let a732 = f166(v1)?;
                            length_bytes(a732)
                        };
                        let a734 = 0u64;
                        if nat_eq(a733, a734) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
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

pub fn f217(v0: List<Bytes>) -> R<bool> {
    if f315(f249(v0.clone())?) {
        Ok(f40({
            let a738 = {
                let a737 = f316(f249(v0)?);
                length_list(a737)
            };
            let a739 = 0u64;
            nat_eq(a738, a739)
        }))
    } else {
        Ok(false)
    }
}

pub fn f218(v0: List<Bytes>) -> R<bool> {
    if f315(f227(v0.clone(), f8(Str::lit("requirement")))?) {
        if f315(f227(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f40({
                let a741 = {
                    let a740 = f183(f316(f227(v0.clone(), f8(Str::lit("requirement")))?), f316(f227(v0, f8(Str::lit("admission")))?));
                    length_list(a740)
                };
                let a742 = 0u64;
                nat_eq(a741, a742)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f219() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f220(v0: Bytes) -> R<Bytes> {
    Ok(f34(f33(v0, 47u8)?))
}

pub fn f221() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f222(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m743 = v0;
    match m743.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f166(v1)?;
            let a745 = {
                let a744 = v3.clone();
                length_bytes(a744)
            };
            let a746 = 0u64;
            if nat_eq(a745, a746) {
                f222(v2)
            } else {
                let m747 = f77(v3.clone())?;
                match m747 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        if f317(f47(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m748 = f222(v2)?;
                            match m748 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f318(v5), {
                                        let a749 = v3;
                                        let a750 = f8(Str::lit("\u{a}"));
                                        append_bytes(a749, a750)
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

pub fn f223() -> Bytes {
    let a751 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a751)
}

pub fn f224(p756: List<Bytes>) -> R<Option<Bytes>> {
    let m755 = p756;
    {
        let mut m755 = m755;
        loop {
            let step = {
                let v0 = m755;
                let m752 = v0;
                match m752.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m753 = f77(f166(v1)?)?;
                        match m753 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m754 = f79(v3.clone(), f8(Str::lit("source")));
                                match m754 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f319(f47(v4.clone(), f8(Str::lit("redistribution"))), f47(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: Bytes) -> Bytes {
    let a761 = {
        let a757 = Str::lit("binding ");
        utf8_encode(a757)
    };
    let a762 = {
        let a759 = v0;
        let a760 = {
            let a758 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a758)
        };
        append_bytes(a759, a760)
    };
    append_bytes(a761, a762)
}

pub fn f226(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m763 = v0;
    match m763.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f166(v1)?;
            let a765 = {
                let a764 = v3.clone();
                length_bytes(a764)
            };
            let a766 = 0u64;
            if nat_eq(a765, a766) {
                f226(v2)
            } else {
                let m767 = f152(v3.clone())?;
                match m767 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f320(v4.clone()) {
                            let m768 = f226(v2)?;
                            match m768 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f203(), List::cons(f8(Str::lit("/")), List::cons(f321(f236(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a769 = v3;
                                        let a770 = f8(Str::lit("\u{a}"));
                                        append_bytes(a769, a770)
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

pub fn f227(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt13>>> {
    let m771 = v0;
    match m771.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f166(v2)?;
            let a773 = {
                let a772 = v4.clone();
                length_bytes(a772)
            };
            let a774 = 0u64;
            if nat_eq(a773, a774) {
                f227(v3, v1)
            } else {
                let m775 = f77(v4)?;
                match m775 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f322(v5.clone());
                        let a777 = {
                            let a776 = v6.clone();
                            length_bytes(a776)
                        };
                        let a778 = 0u64;
                        if nat_eq(a777, a778) {
                            Ok(None::<List<Adt13>>)
                        } else {
                            let m779 = f227(v3, v1.clone())?;
                            match m779 {
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

pub fn f228(p782: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m781 = p782;
    {
        let mut m781 = m781;
        loop {
            let step = {
                let v0 = m781;
                let m780 = v0;
                match m780.uncons() {
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
                Ok(next) => { m781 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f230(v0: Bytes, v1: List<Adt13>) -> R<(Bytes, Bytes)> {
    Ok((v0, f86(Adt13::C3(std::rc::Rc::new(f87(v1))))?))
}

pub fn f231(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f86(Adt13::C3(std::rc::Rc::new(f323(f118(f98(v1, f174(f82(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f232(v0: Adt1, v1: Bytes) -> R<Option<Adt17>> {
    f152(f53(v0, v1))
}

pub fn f233(p788: Adt17, v1: Bytes) -> Option<Adt17> {
    let m787 = p788;
    {
        let mut m787 = m787;
        loop {
            let step = {
                let v0 = m787;
                let m783 = v0;
                match m783 {
                    Adt17::C0 => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C1(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C3(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C4(_, _, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C5(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C6(_) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C7(r784) => {
                        let v5 = (*r784).clone();
                        Ok::<Adt17, Option<Adt17>>(v5)
                    }
                    Adt17::C8 => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C9(_, _) => {
                        Err::<Adt17, Option<Adt17>>(None::<Adt17>)
                    }
                    Adt17::C10(v2, r785, r786) => {
                        let v3 = (*r785).clone();
                        let v4 = (*r786).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt17, Option<Adt17>>(Some(v3))
                        } else {
                            Ok::<Adt17, Option<Adt17>>(v4)
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

pub fn f234(v0: Adt1, v1: Bytes, v2: Adt17, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m789 = f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m789.uncons() {
        None => {
            let m791 = f228(f104(List::cons(Adt13::C4(std::rc::Rc::new({
                let m790 = f324(v4.clone());
                match m790 {
                    None => {
                        Adt13::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m791 {
                None => {
                    f325(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f229(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f235(v0: Adt17) -> R<Bytes> {
    let a792 = f326(v0)?;
    let a793 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a792, a793))
}

pub fn f236(v0: Adt17, v1: Bytes) -> Bytes {
    let m794 = f327(f233(v0, v1));
    match m794 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f237(v0: List<Adt17>) -> List<Bytes> {
    let m795 = v0;
    match m795.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m796 = f327(Some(v1));
            match m796 {
                None => {
                    f237(v2)
                }
                Some(v3) => {
                    List::cons(v3, f237(v2))
                }
            }
        }
    }
}

pub fn f238(v0: Option<Adt17>) -> List<Adt17> {
    let m797 = v0;
    match m797 {
        None => {
            List::<Adt17>::nil()
        }
        Some(v1) => {
            f328(v1)
        }
    }
}

pub fn f239(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f240(v0: Bytes) -> R<Bytes> {
    Ok(f329(f330(v0)?))
}

pub fn f241(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>) -> R<Adt11> {
    let m798 = f232(v0.clone(), v2.clone())?;
    match m798 {
        None => {
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
        }
        Some(v6) => {
            if f28(v4.clone(), f8(Str::lit("none"))) {
                f331(v0, v1, v2, v3, v4, v5, v6)
            } else {
                let v7: Bytes = {
                    let a799 = f8(Str::lit("Catalog."));
                    let a800 = v4.clone();
                    append_bytes(a799, a800)
                };
                if f40(f332(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, v7.clone(), v3.clone())) {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m801 = f232(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m801 {
                        None => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v8) => {
                            let m802 = f333(v8, v6.clone())?;
                            match m802 {
                                None => {
                                    f331(v0, v1, v2, v3, {
                                        let a803 = f8(Str::lit("Catalog."));
                                        let a804 = v4;
                                        append_bytes(a803, a804)
                                    }, v5, v6)
                                }
                                Some(v9) => {
                                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" does not conform to ")), List::cons(v7, List::cons(f8(Str::lit(": ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f242() -> Bytes {
    f334()
}

pub fn f243(v0: Adt1, v1: Adt13, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f247(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f335(v1, v4)?)?), List::cons(f231(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f244(v0: List<List<Bytes>>) -> List<Bytes> {
    let m805 = v0;
    match m805.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f245() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f246(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f296(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f247(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f336(f122(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f337(f338(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f248(p808: List<Adt13>) -> bool {
    let m807 = p808;
    {
        let mut m807 = m807;
        loop {
            let step = {
                let v0 = m807;
                let m806 = v0;
                match m806.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f28(f47(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f41(f8(Str::lit("studio")), f115(f339(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m809 = v0;
    match m809.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f166(v1)?;
            let a811 = {
                let a810 = v3.clone();
                length_bytes(a810)
            };
            let a812 = 0u64;
            if nat_eq(a811, a812) {
                f249(v2)
            } else {
                let m813 = f77(v3)?;
                match m813 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if if f180(v4.clone(), f8(Str::lit("agent"))) {
                            if f180(v4.clone(), f8(Str::lit("at"))) {
                                if f180(v4.clone(), f8(Str::lit("quote"))) {
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
                            let m814 = f249(v2)?;
                            match m814 {
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

pub fn f250(v0: u64) -> bool {
    let a815 = v0;
    let a816 = 127u64;
    nat_le(a815, a816)
}

pub fn f251(v0: Bytes) -> R<Bytes> {
    f341(v0.clone(), 0u64, {
        let a817 = v0;
        length_bytes(a817)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f252(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt17, u64)>> {
    let m818 = v2;
    if m818 == 0 {
        Ok(None::<(Adt17, u64)>)
    } else {
        let v4 = m818 - 1;
        let v5: u64 = f342(v0.clone(), v1)?;
        let a819 = v3;
        let a820 = 1u64;
        if nat_eq(a819, a820) {
            let m821 = f252(v0.clone(), v1, v4, 0u64)?;
            match m821 {
                None => {
                    Ok(None::<(Adt17, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f342(v0.clone(), {
                        let (_, h822) = v6.clone();
                        h822
                    })?;
                    let a823 = v7;
                    let a824 = 44u64;
                    if nat_eq(a823, a824) {
                        let m828 = f252(v0, {
                            let a826 = {
                                let (_, h825) = v6;
                                h825
                            };
                            let a827 = 1u64;
                            nat_add(a826, a827)?
                        }, v4, 1u64)?;
                        match m828 {
                            None => {
                                Ok(None::<(Adt17, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt17::C9(std::rc::Rc::new({
                                    let (h829, _) = v6;
                                    h829
                                }), std::rc::Rc::new({
                                    let (h830, _) = v8;
                                    h830
                                })), {
                                    let (_, h831) = v8;
                                    h831
                                })))
                            }
                        }
                    } else {
                        let a832 = v7;
                        let a833 = 93u64;
                        if nat_eq(a832, a833) {
                            Ok(Some((Adt17::C9(std::rc::Rc::new({
                                let (h834, _) = v6;
                                h834
                            }), std::rc::Rc::new(Adt17::C8)), {
                                let a836 = {
                                    let (_, h835) = v6;
                                    h835
                                };
                                let a837 = 1u64;
                                nat_add(a836, a837)?
                            })))
                        } else {
                            Ok(None::<(Adt17, u64)>)
                        }
                    }
                }
            }
        } else {
            let a838 = v3;
            let a839 = 2u64;
            if nat_eq(a838, a839) {
                let m842 = f343(v0.clone(), {
                    let a840 = v1;
                    let a841 = 1u64;
                    nat_add(a840, a841)?
                })?;
                match m842 {
                    None => {
                        Ok(None::<(Adt17, u64)>)
                    }
                    Some(v9) => {
                        if f344({
                            let a843 = v5;
                            let a844 = 34u64;
                            nat_eq(a843, a844)
                        }, {
                            let a846 = f342(v0.clone(), {
                                let (h845, _) = v9.clone();
                                h845
                            })?;
                            let a847 = 58u64;
                            nat_eq(a846, a847)
                        }) {
                            let m851 = f252(v0.clone(), {
                                let a849 = {
                                    let (h848, _) = v9;
                                    h848
                                };
                                let a850 = 1u64;
                                nat_add(a849, a850)?
                            }, v4, 0u64)?;
                            match m851 {
                                None => {
                                    Ok(None::<(Adt17, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f342(v0.clone(), {
                                        let (_, h852) = v10.clone();
                                        h852
                                    })?;
                                    let a853 = v11;
                                    let a854 = 44u64;
                                    if nat_eq(a853, a854) {
                                        let m858 = f252(v0, {
                                            let a856 = {
                                                let (_, h855) = v10;
                                                h855
                                            };
                                            let a857 = 1u64;
                                            nat_add(a856, a857)?
                                        }, v4, 2u64)?;
                                        match m858 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt17::C10({
                                                    let (_, h859) = v9;
                                                    h859
                                                }, std::rc::Rc::new({
                                                    let (h860, _) = v10;
                                                    h860
                                                }), std::rc::Rc::new({
                                                    let (h861, _) = v12;
                                                    h861
                                                })), {
                                                    let (_, h862) = v12;
                                                    h862
                                                })))
                                            }
                                        }
                                    } else {
                                        let a863 = v11;
                                        let a864 = 125u64;
                                        if nat_eq(a863, a864) {
                                            Ok(Some((Adt17::C10({
                                                let (_, h865) = v9;
                                                h865
                                            }, std::rc::Rc::new({
                                                let (h866, _) = v10;
                                                h866
                                            }), std::rc::Rc::new(Adt17::C8)), {
                                                let a868 = {
                                                    let (_, h867) = v10;
                                                    h867
                                                };
                                                let a869 = 1u64;
                                                nat_add(a868, a869)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt17, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt17, u64)>)
                        }
                    }
                }
            } else {
                let a870 = v5;
                let a871 = 34u64;
                if nat_eq(a870, a871) {
                    let m874 = f343(v0, {
                        let a872 = v1;
                        let a873 = 1u64;
                        nat_add(a872, a873)?
                    })?;
                    match m874 {
                        None => {
                            Ok(None::<(Adt17, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt17::C5({
                                let (_, h875) = v13;
                                h875
                            }), {
                                let (h876, _) = v13;
                                h876
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt17::C0, {
                            let a877 = v1;
                            let a878 = 4u64;
                            nat_add(a877, a878)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt17::C2(true), {
                                let a879 = v1;
                                let a880 = 4u64;
                                nat_add(a879, a880)?
                            })))
                        } else {
                            if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt17::C2(false), {
                                    let a881 = v1;
                                    let a882 = 5u64;
                                    nat_add(a881, a882)?
                                })))
                            } else {
                                let a883 = v5;
                                let a884 = 91u64;
                                if nat_eq(a883, a884) {
                                    let a887 = f342(v0.clone(), {
                                        let a885 = v1;
                                        let a886 = 1u64;
                                        nat_add(a885, a886)?
                                    })?;
                                    let a888 = 93u64;
                                    if nat_eq(a887, a888) {
                                        Ok(Some((Adt17::C6(std::rc::Rc::new(Adt17::C8)), {
                                            let a889 = v1;
                                            let a890 = 2u64;
                                            nat_add(a889, a890)?
                                        })))
                                    } else {
                                        let m893 = f252(v0, {
                                            let a891 = v1;
                                            let a892 = 1u64;
                                            nat_add(a891, a892)?
                                        }, v4, 1u64)?;
                                        match m893 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt17::C6(std::rc::Rc::new({
                                                    let (h894, _) = v14;
                                                    h894
                                                })), {
                                                    let (_, h895) = v14;
                                                    h895
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a896 = v5;
                                    let a897 = 123u64;
                                    if nat_eq(a896, a897) {
                                        let a900 = f342(v0.clone(), {
                                            let a898 = v1;
                                            let a899 = 1u64;
                                            nat_add(a898, a899)?
                                        })?;
                                        let a901 = 125u64;
                                        if nat_eq(a900, a901) {
                                            Ok(Some((Adt17::C7(std::rc::Rc::new(Adt17::C8)), {
                                                let a902 = v1;
                                                let a903 = 2u64;
                                                nat_add(a902, a903)?
                                            })))
                                        } else {
                                            let m906 = f252(v0, {
                                                let a904 = v1;
                                                let a905 = 1u64;
                                                nat_add(a904, a905)?
                                            }, v4, 2u64)?;
                                            match m906 {
                                                None => {
                                                    Ok(None::<(Adt17, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt17::C7(std::rc::Rc::new({
                                                        let (h907, _) = v15;
                                                        h907
                                                    })), {
                                                        let (_, h908) = v15;
                                                        h908
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m909 = f345(v0, v1)?;
                                        match m909 {
                                            None => {
                                                Ok(None::<(Adt17, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h910) = v16;
                                                    h910
                                                }, {
                                                    let (h911, _) = v16;
                                                    h911
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

pub fn f253(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h912, _) = v1.clone();
        h912
    };
    let v3: u64 = {
        let (h913, _) = {
            let (_, h914) = v1.clone();
            h914
        };
        h913
    };
    let v4: List<Bytes> = {
        let (_, h915) = {
            let (_, h916) = v1;
            h916
        };
        h915
    };
    let m917 = f167(v0.clone(), v2);
    match m917 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a918 = v5;
            let a919 = 10u8;
            if equal(a918, a919) {
                Ok(Some(({
                    let a920 = v2;
                    let a921 = 1u64;
                    nat_add(a920, a921)?
                }, ({
                    let a922 = v2;
                    let a923 = 1u64;
                    nat_add(a922, a923)?
                }, List::cons(f346(f35(v0, v3, {
                    let a924 = v2;
                    let a925 = v3;
                    nat_sub(a924, a925)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a926 = v2;
                    let a927 = 1u64;
                    nat_add(a926, a927)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f254(v0: Adt13) -> List<Adt13> {
    let m928 = v0;
    match m928 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r929) => {
            let v3 = (*r929).clone();
            f254(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r930, r931) => {
            let v1 = (*r930).clone();
            let v2 = (*r931).clone();
            List::cons(v1, f254(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f255(v0: Adt13, v1: Bytes) -> Adt13 {
    let m932 = v0;
    match m932 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r933) => {
            let v10 = (*r933).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r934) => {
            let v5 = (*r934).clone();
            Adt13::C4(std::rc::Rc::new(f255(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r935, r936) => {
            let v11 = (*r935).clone();
            let v12 = (*r936).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r937, r938) => {
            let v3 = (*r937).clone();
            let v4 = (*r938).clone();
            if f28(v2.clone(), f8(Str::lit("authority"))) {
                Adt13::C7(v2, std::rc::Rc::new(f347(v3, v1.clone())), std::rc::Rc::new(f255(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f255(v4, v1)))
            }
        }
    }
}

pub fn f256(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a939 = 0u64;
        let a940 = v1;
        nat_lt(a939, a940)
    } else {
        false
    } {
        let a941 = Bytes::lit(&[0x2d]);
        let a942 = f348(v1)?;
        Ok(append_bytes(a941, a942))
    } else {
        f348(v1)
    }
}

pub fn f257(v0: Bytes) -> R<Bytes> {
    let a948 = {
        let a946 = Bytes::lit(&[0x22]);
        let a947 = {
            let (_, h943) = {
                let (h944, _) = f349(Fn3::F350(v0.clone()), {
                    let a945 = v0;
                    length_bytes(a945)
                }, (0u64, Bytes::lit(&[])))?;
                h944
            };
            h943
        };
        append_bytes(a946, a947)
    };
    let a949 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a948, a949))
}

pub fn f258(v0: List<Bytes>) -> Bytes {
    let a952 = {
        let a950 = Bytes::lit(&[0x5b]);
        let a951 = f95(v0, Bytes::lit(&[0x2c]));
        append_bytes(a950, a951)
    };
    let a953 = Bytes::lit(&[0x5d]);
    append_bytes(a952, a953)
}

pub fn f259(v0: Adt13) -> R<List<Bytes>> {
    let m954 = v0;
    match m954 {
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
        Adt13::C6(r955, r956) => {
            let v7 = (*r955).clone();
            let v8 = (*r956).clone();
            Ok(List::cons(f158(v7)?, f259(v8)?))
        }
        Adt13::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f260(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f351(Fn4::F352, List::<(Bytes, Bytes)>::nil(), f353(v0)?);
    let v2: List<Bytes> = f354(Fn5::F355, List::<Bytes>::nil(), v1)?;
    let a959 = {
        let a957 = Bytes::lit(&[0x7b]);
        let a958 = f95(v2, Bytes::lit(&[0x2c]));
        append_bytes(a957, a958)
    };
    let a960 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a959, a960))
}

pub fn f261(v0: Adt13) -> R<List<(Bytes, Bytes)>> {
    let m961 = v0;
    match m961 {
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
        Adt13::C7(v9, r962, r963) => {
            let v10 = (*r962).clone();
            let v11 = (*r963).clone();
            Ok(List::cons((v9, f158(v10)?), f261(v11)?))
        }
    }
}

pub fn f262() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f263(v0: Adt12) -> Bytes {
    let m964 = v0;
    match m964 {
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

pub fn f264(v0: Adt12, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f356(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f356(f8(Str::lit("id")), f58(v0.clone(), f50(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f356(f8(Str::lit("name")), f357(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f356(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f356(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f356(f8(Str::lit("statement")), f358(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f356(f8(Str::lit("media_type")), f359(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f356(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f356(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f356(f8(Str::lit("url")), f161(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f265(v0: Bytes) -> R<u64> {
    let (h965, _) = f360(Fn6::F361(v0.clone()), {
        let a966 = v0.clone();
        length_bytes(a966)
    }, {
        let a967 = v0;
        length_bytes(a967)
    })?;
    Ok(h965)
}

pub fn f266(v0: Bytes, v1: u64) -> R<u64> {
    let (h968, _) = f360(Fn6::F362(v1, v0), v1, 0u64)?;
    Ok(h968)
}

pub fn f267(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m969 = v0;
    match m969.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m970 = f79(v1, f8(Str::lit("std")));
            match m970 {
                None => {
                    f267(v2)
                }
                Some(v3) => {
                    if f180(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f47(v3.clone(), f8(Str::lit("row"))), f47(v3, f8(Str::lit("authority")))), f267(v2))
                    } else {
                        f267(v2)
                    }
                }
            }
        }
    }
}

pub fn f268(v0: Adt14, v1: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m971 = v1;
    match m971.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f47(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f47(v2, f8(Str::lit("object")));
            if if f41(v4.clone(), match v0.clone() {
                Adt14::C0(h972, _, _) => {
                    h972
                }
            }) {
                f41(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h973, _) => {
                        h973
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f268(v0, v3))
            } else {
                if if f41(v5.clone(), match v0.clone() {
                    Adt14::C0(h974, _, _) => {
                        h974
                    }
                }) {
                    f41(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h975, _) => {
                            h975
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f268(v0, v3))
                } else {
                    f268(v0, v3)
                }
            }
        }
    }
}

pub fn f269(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt18>> {
    let m976 = v0;
    match m976.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v4, v5)) => {
            if if f363(v4.clone(), v2.clone()) {
                true
            } else {
                f363(v4.clone(), v3.clone())
            } {
                f269(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt18::C0({
                    let (h977, _) = v4.clone();
                    h977
                }, {
                    let a980 = {
                        let a979 = f118(f364({
                            let (h978, _) = v4.clone();
                            h978
                        }, v1.clone()))?;
                        length_list(a979)
                    };
                    let a981 = 1u64;
                    if nat_eq(a980, a981) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h982) = v4.clone();
                    h982
                }), f269(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f270(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m983 = v1;
    match m983.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41({
                let (h984, _) = v2.clone();
                h984
            }, v0.clone()) {
                List::cons({
                    let (_, h985) = v2;
                    h985
                }, f270(v0, v3))
            } else {
                if f41({
                    let (_, h986) = v2;
                    h986
                }, v0.clone()) {
                    List::cons({
                        let (h987, _) = v2;
                        h987
                    }, f270(v0, v3))
                } else {
                    f270(v0, v3)
                }
            }
        }
    }
}

pub fn f271(v0: Adt14, v1: Adt13, v2: Bytes) -> bool {
    if f41(v2.clone(), match v0.clone() {
        Adt14::C0(h988, _, _) => {
            h988
        }
    }) {
        true
    } else {
        if f41(v2.clone(), match v0.clone() {
            Adt14::C0(_, h989, _) => {
                h989
            }
        }) {
            true
        } else {
            if f41(v2, match v0.clone() {
                Adt14::C0(_, _, h990) => {
                    h990
                }
            }) {
                true
            } else {
                if f180(v1.clone(), f8(Str::lit("scheme"))) {
                    f41(f47(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h991, _) => {
                            h991
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f272(v0: Adt13) -> bool {
    if f180(v0.clone(), f8(Str::lit("domain"))) {
        if f180(v0.clone(), f8(Str::lit("row"))) {
            if f41(f47(v0.clone(), f8(Str::lit("relation"))), f365()) {
                if f366(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f366(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f366(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f366(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f273(v0: Adt13) -> bool {
    if f180(v0.clone(), f8(Str::lit("subject"))) {
        if f180(v0.clone(), f8(Str::lit("object"))) {
            if f180(v0.clone(), f8(Str::lit("evidence"))) {
                f41(f47(v0, f8(Str::lit("relation"))), f367())
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

pub fn f274(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m992 = f152(v1)?;
    match m992 {
        None => {
            Ok(f186(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f237(f238(f233(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m993 = f327(f233(v4.clone(), f8(Str::lit("construct"))));
                match m993 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m994 = v5;
            match m994.uncons() {
                None => {
                    Ok(f186(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m995 = v9;
                    match m995.uncons() {
                        None => {
                            Ok(f186(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f40(f28(v8, f8(Str::lit("std")))) {
                                Ok(f186(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f40(f41(v10.clone(), v2)) {
                                    Ok(f186(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f344(f40(f28(v7.clone(), f8(Str::lit("none")))), f40(f41(v7.clone(), v3))) {
                                        Ok(f186(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m996 = f327(f233(v4.clone(), f8(Str::lit("pointer"))));
                                        match m996 {
                                            None => {
                                                Ok(f186(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m997 = f233(v4, f8(Str::lit("value")));
                                                match m997 {
                                                    None => {
                                                        Ok(f186(v0, f8(Str::lit("a row has no value"))))
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

pub fn f275(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m998 = v1;
    match m998.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f368(v3.clone())
            } else {
                f40(f368(v3.clone()))
            } {
                List::cons(f200(v0.clone(), v3), f275(v0, v4, v2))
            } else {
                f275(v0, v4, v2)
            }
        }
    }
}

pub fn f276(p1001: List<Bytes>) -> R<Option<Bytes>> {
    let m1000 = p1001;
    {
        let mut m1000 = m1000;
        loop {
            let step = {
                let v0 = m1000;
                let m999 = v0;
                match m999.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f369(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f166(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1002 = v1;
    match m1002.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f81(f53(v0.clone(), v2))?, f277(v0, v3)?))
        }
    }
}

pub fn f278(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1003 = v0.clone();
        length_bytes(a1003)
    };
    let v3: u64 = {
        let a1004 = v1.clone();
        length_bytes(a1004)
    };
    let v4: u64 = {
        let (h1005, _) = f360(Fn6::F370(v1.clone(), v3, v0.clone(), v2), {
            let a1006 = v2;
            let a1007 = 1u64;
            nat_add(a1006, a1007)?
        }, 0u64)?;
        h1005
    };
    if f371(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f279() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f280(v0: Adt15) -> R<Adt19> {
    Ok(Adt19::C0(f372(match v0.clone() {
        Adt15::C0(h1008, _, _, _, _) => {
            h1008
        }
    }, 0u64)?, f372(match v0.clone() {
        Adt15::C0(_, h1009, _, _, _) => {
            h1009
        }
    }, 0u64)?, f372(match v0.clone() {
        Adt15::C0(_, _, h1010, _, _) => {
            h1010
        }
    }, 0u64)?, f372(match v0 {
        Adt15::C0(_, _, _, h1011, _) => {
            h1011
        }
    }, 0u64)?))
}

pub fn f281(v0: Adt15) -> List<Bytes> {
    f98(match v0.clone() {
        Adt15::C0(h1012, _, _, _, _) => {
            h1012
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, h1013, _, _) => {
            h1013
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, _, h1014, _) => {
            h1014
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1015) => {
            h1015
        }
    })))
}

pub fn f282(v0: Adt19, p1021: List<u64>, p1022: List<Adt21>) -> Option<u64> {
    let m1020 = (p1021, p1022);
    {
        let mut m1020 = m1020;
        loop {
            let step = {
                let (v1, v2) = m1020;
                let m1016 = v2;
                match m1016.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt21>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f373(v0.clone(), v1.clone(), match v3.clone() {
                            Adt21::C0(_, h1017) => {
                                h1017
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt21>), Option<u64>>((List::cons(match v3 {
                                Adt21::C0(h1018, _) => {
                                    h1018
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt21>), Option<u64>>(Some(match v3 {
                                Adt21::C0(h1019, _) => {
                                    h1019
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: Adt15, v1: List<Adt13>, v2: u64) -> R<List<Adt21>> {
    let m1023 = v1;
    match m1023.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt21::C0(v2, f374(v0.clone(), v3)?), f283(v0, v4, {
                let a1024 = v2;
                let a1025 = 1u64;
                nat_add(a1024, a1025)?
            })?))
        }
    }
}

pub fn f284(p1029: List<Bytes>, p1030: u64) -> Bytes {
    let m1028 = (p1029, p1030);
    {
        let mut m1028 = m1028;
        loop {
            let step = {
                let (v0, v1) = m1028;
                let m1026 = v0;
                match m1026.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1027 = v1;
                        if m1027 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1027 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f375(v0.clone(), v1.clone()) {
        if f375(v1, v0) {
            f376(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f286(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1031 = v0;
    match m1031.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f377(v2, v1.clone())?, f286(v3, v1)?))
        }
    }
}

pub fn f287(v0: List<Adt13>) -> List<u64> {
    let m1032 = v0;
    match m1032.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f378(v1), f287(v2))
        }
    }
}

pub fn f288(p1035: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1034 = p1035;
    {
        let mut m1034 = m1034;
        loop {
            let step = {
                let v0 = m1034;
                let m1033 = v0;
                match m1033.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f379(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289(p1038: List<Bytes>, p1039: List<Bytes>) -> Option<Bytes> {
    let m1037 = (p1038, p1039);
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let (v0, v1) = m1037;
                let m1036 = v0;
                match m1036.uncons() {
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
                Ok(next) => { m1037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f291(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1040 = v0.clone();
    match m1040.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1041 = v2;
            let a1042 = v1.clone();
            if equal(a1041, a1042) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f292(v0: List<(Bytes, ())>) -> List<Bytes> {
    f196(f380(Fn7::F381, List::<Bytes>::nil(), v0))
}

pub fn f293(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f382(f383(Fn8::F384, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1047 = {
        let (h1043, _) = f385(Fn9::F386, {
            let a1045 = {
                let a1044 = v0;
                length_list(a1044)
            };
            let a1046 = 1u64;
            nat_add(a1045, a1046)?
        }, v1)?;
        h1043
    };
    match m1047.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f294(v0: List<Bytes>) -> List<(Bytes, ())> {
    f387(f388(Fn10::F389, List::<(Bytes, ())>::nil(), v0))
}

pub fn f295(v0: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1048 = f33(v1, 34u8)?;
    match m1048.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1049 = v3;
            match m1049.uncons() {
                None => {
                    f390(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f296(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f391(v0.clone(), 0u64, {
        let a1050 = v0;
        length_bytes(a1050)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f297(v0: Adt13, v1: Bytes) -> Bytes {
    if f28(f392(f79(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f298(v0: Adt1, v1: Bytes) -> R<Adt17> {
    let m1051 = f232(v0, v1)?;
    match m1051 {
        None => {
            Ok(Adt17::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f299(v0: Bytes) -> R<Bytes> {
    let a1052 = f8(Str::lit("Catalog."));
    let a1053 = f321(v0)?;
    Ok(append_bytes(a1052, a1053))
}

pub fn f300(v0: Adt17) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f321(f236(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f301(v0: List<Bytes>) -> List<Bytes> {
    let m1054 = v0;
    match m1054.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f199(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f200(f219(), v1), f301(v2))
            } else {
                f301(v2)
            }
        }
    }
}

pub fn f302(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1055 = v1;
    match m1055.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f393(v0.clone(), v2)?;
            let (h1056, _) = v4.clone();
            if h1056 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f302(v0, v3)?;
                let (h1057, _) = v5.clone();
                if h1057 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1060 = {
                            let (_, h1058) = v4;
                            h1058
                        };
                        let a1061 = {
                            let (_, h1059) = v5;
                            h1059
                        };
                        append_bytes(a1060, a1061)
                    }))
                }
            }
        }
    }
}

pub fn f303(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m1062 = v1;
    match m1062.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f180(v2.clone(), f8(Str::lit("subject"))) {
                if f181(v0.clone(), v2.clone()) {
                    f303(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f47(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f303(v0, v3))
                }
            } else {
                if f312(v0.clone(), v2.clone()) {
                    f303(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f313(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f303(v0, v3))
                }
            }
        }
    }
}

pub fn f304(v0: Adt1) -> R<Bytes> {
    let a1064 = {
        let a1063 = f101(f102(f212(f209(f210(f53(v0.clone(), f172()))?)), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(f82(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1063)
    };
    let a1065 = 0u64;
    if nat_eq(a1064, a1065) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f305(v0: List<Adt13>) -> R<List<Bytes>> {
    let m1066 = v0;
    match m1066.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f95(f174(f156(f79(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f47(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f47(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f394(f81(f47(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f305(v2)?))
        }
    }
}

pub fn f306(v0: Bytes, p1124: u64, p1125: u64, p1126: Adt20) -> R<Adt20> {
    let m1123 = (p1124, (p1125, p1126));
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1123;
                let m1067 = v2;
                if m1067 == 0 {
                    Err::<(u64, (u64, Adt20)), R<Adt20>>(Ok(v3))
                } else {
                    let v4 = m1067 - 1;
                    let v5: u64 = f395(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt20::C0(_, h1068, _, _, _) => {
                            h1068
                        }
                    } {
                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                            let a1069 = v1;
                            let a1070 = 1u64;
                            nat_add(a1069, a1070)?
                        }, (v4, Adt20::C0(true, false, match v3.clone() {
                            Adt20::C0(_, _, h1071, _, _) => {
                                h1071
                            }
                        }, match v3.clone() {
                            Adt20::C0(_, _, _, h1072, _) => {
                                h1072
                            }
                        }, match v3 {
                            Adt20::C0(_, _, _, _, h1073) => {
                                h1073
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt20::C0(h1074, _, _, _, _) => {
                                h1074
                            }
                        } {
                            let a1075 = v5;
                            let a1076 = 34u64;
                            if nat_eq(a1075, a1076) {
                                let a1079 = f395(v0.clone(), {
                                    let a1077 = v1;
                                    let a1078 = 1u64;
                                    nat_add(a1077, a1078)?
                                })?;
                                let a1080 = 34u64;
                                if nat_eq(a1079, a1080) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1081 = v1;
                                        let a1082 = 1u64;
                                        nat_add(a1081, a1082)?
                                    }, (v4, Adt20::C0(true, true, f396(match v3.clone() {
                                        Adt20::C0(_, _, h1083, _, _) => {
                                            h1083
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt20::C0(_, _, _, h1084, _) => {
                                            h1084
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1085) => {
                                            h1085
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1086 = v1;
                                        let a1087 = 1u64;
                                        nat_add(a1086, a1087)?
                                    }, (v4, Adt20::C0(false, false, match v3.clone() {
                                        Adt20::C0(_, _, h1088, _, _) => {
                                            h1088
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1089, _) => {
                                            h1089
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1090) => {
                                            h1090
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1091 = v1;
                                    let a1092 = 1u64;
                                    nat_add(a1091, a1092)?
                                }, (v4, Adt20::C0(true, false, f396(match v3.clone() {
                                    Adt20::C0(_, _, h1093, _, _) => {
                                        h1093
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt20::C0(_, _, _, h1094, _) => {
                                        h1094
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1095) => {
                                        h1095
                                    }
                                }))))
                            }
                        } else {
                            let a1096 = v5;
                            let a1097 = 34u64;
                            if nat_eq(a1096, a1097) {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1098 = v1;
                                    let a1099 = 1u64;
                                    nat_add(a1098, a1099)?
                                }, (v4, Adt20::C0(true, false, match v3.clone() {
                                    Adt20::C0(_, _, h1100, _, _) => {
                                        h1100
                                    }
                                }, match v3.clone() {
                                    Adt20::C0(_, _, _, h1101, _) => {
                                        h1101
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1102) => {
                                        h1102
                                    }
                                }))))
                            } else {
                                let a1103 = v5;
                                let a1104 = 44u64;
                                if nat_eq(a1103, a1104) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1105 = v1;
                                        let a1106 = 1u64;
                                        nat_add(a1105, a1106)?
                                    }, (v4, Adt20::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt20::C0(_, _, h1107, _, _) => {
                                            h1107
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1108, _) => {
                                            h1108
                                        }
                                    }), match v3 {
                                        Adt20::C0(_, _, _, _, h1109) => {
                                            h1109
                                        }
                                    }))))
                                } else {
                                    let a1110 = v5;
                                    let a1111 = 13u64;
                                    if nat_eq(a1110, a1111) {
                                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                            let a1112 = v1;
                                            let a1113 = 1u64;
                                            nat_add(a1112, a1113)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1114 = v5;
                                        let a1115 = 10u64;
                                        if nat_eq(a1114, a1115) {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1116 = v1;
                                                let a1117 = 1u64;
                                                nat_add(a1116, a1117)?
                                            }, (v4, f307(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1118 = v1;
                                                let a1119 = 1u64;
                                                nat_add(a1118, a1119)?
                                            }, (v4, Adt20::C0(false, false, f396(match v3.clone() {
                                                Adt20::C0(_, _, h1120, _, _) => {
                                                    h1120
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt20::C0(_, _, _, h1121, _) => {
                                                    h1121
                                                }
                                            }, match v3 {
                                                Adt20::C0(_, _, _, _, h1122) => {
                                                    h1122
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
                Ok(next) => { m1123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307(v0: Adt20) -> Adt20 {
    Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f397(List::cons(match v0.clone() {
        Adt20::C0(_, _, h1127, _, _) => {
            h1127
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1128, _) => {
            h1128
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt20::C0(_, _, _, _, h1129) => {
            h1129
        }
    }))
}

pub fn f308(p1132: List<List<Bytes>>, p1133: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1131 = (p1132, p1133);
    {
        let mut m1131 = m1131;
        loop {
            let step = {
                let (v0, v1) = m1131;
                let m1130 = v0;
                match m1130.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(v0: Bytes, v1: List<Adt18>) -> List<Bytes> {
    let m1134 = v1;
    match m1134.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(match v2.clone() {
                Adt18::C0(h1135, _, _) => {
                    h1135
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt18::C0(_, h1136, _) => {
                        h1136
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt18::C0(_, _, h1137) => {
                        h1137
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f309(v0, v3))
            } else {
                f309(v0, v3)
            }
        }
    }
}

pub fn f310(v0: Bytes, v1: List<Adt13>) -> List<Bytes> {
    let m1138 = v1;
    match m1138.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(f47(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f310(v0, v3))
            } else {
                if f28(f47(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f47(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f310(v0, v3))
                } else {
                    f310(v0, v3)
                }
            }
        }
    }
}

pub fn f311(p1143: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1142 = p1143;
    {
        let mut m1142 = m1142;
        loop {
            let step = {
                let v0 = m1142;
                let m1139 = v0;
                match m1139.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1140 = v2;
                        match m1140.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f28(v4, v1.clone()) {
                                    let m1141 = v5;
                                    match m1141.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f398(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Adt14, v1: Adt13) -> bool {
    if f41(f47(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1144, _, _) => {
            h1144
        }
    }) {
        f41(f313(v1), match v0 {
            Adt14::C0(_, h1145, _) => {
                h1145
            }
        })
    } else {
        false
    }
}

pub fn f313(v0: Adt13) -> Bytes {
    let m1146 = f79(v0, f8(Str::lit("evidence")));
    match m1146 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f47(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f314(v0: Adt13) -> Bytes {
    if f180(v0.clone(), f8(Str::lit("req"))) {
        let a1147 = f8(Str::lit("req "));
        let a1148 = f47(v0, f8(Str::lit("req")));
        append_bytes(a1147, a1148)
    } else {
        let m1149 = f79(v0.clone(), f8(Str::lit("std")));
        match m1149 {
            None => {
                let m1152 = f79(v0, f8(Str::lit("der")));
                match m1152 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f47(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f95(f174(f156(f79(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f47(v1.clone(), f8(Str::lit("authority"))), List::cons(if f180(v1.clone(), f8(Str::lit("row"))) {
                    let a1150 = f8(Str::lit(" row "));
                    let a1151 = f47(v1, f8(Str::lit("row")));
                    append_bytes(a1150, a1151)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f315(v0: Option<List<Adt13>>) -> bool {
    let m1153 = v0;
    match m1153 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f316(v0: Option<List<Adt13>>) -> List<Adt13> {
    let m1154 = v0;
    match m1154 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f317(v0: Bytes, v1: Bytes) -> bool {
    let a1155 = v0;
    let a1156 = f334();
    if equal(a1155, a1156) {
        let a1160 = {
            let a1158 = {
                let a1157 = v1;
                length_bytes(a1157)
            };
            let a1159 = 0u64;
            nat_eq(a1158, a1159)
        };
        bool_not(a1160)
    } else {
        false
    }
}

pub fn f318(v0: Bytes) -> Bytes {
    f7(List::cons(f117(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f319(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1162 = v0;
    let a1163 = {
        let a1161 = Str::lit("redistributable");
        utf8_encode(a1161)
    };
    if equal(a1162, a1163) {
        f399(v1)
    } else {
        Ok(false)
    }
}

pub fn f320(v0: Adt17) -> bool {
    if f400(v0.clone(), f8(Str::lit("name"))) {
        if f401(v0.clone(), f8(Str::lit("schema"))) {
            let m1164 = f324(f237(f238(f233(v0, f8(Str::lit("citation"))))));
            match m1164 {
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

pub fn f321(v0: Bytes) -> R<Bytes> {
    f402(v0.clone(), 0u64, {
        let a1165 = v0;
        length_bytes(a1165)
    }, true, Bytes::lit(&[]))
}

pub fn f322(v0: Adt13) -> Bytes {
    if f180(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f180(v0.clone(), f8(Str::lit("id"))) {
            if f180(v0.clone(), f8(Str::lit("statement"))) {
                if f180(v0.clone(), f8(Str::lit("stakeholder"))) {
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

pub fn f323(v0: List<Bytes>) -> Adt13 {
    let m1166 = v0;
    match m1166.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(f323(v2)))
        }
    }
}

pub fn f324(v0: List<Bytes>) -> Option<Adt13> {
    let m1167 = v0;
    match m1167.uncons() {
        None => {
            None::<Adt13>
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("req"))) {
                let m1168 = v2;
                match m1168.uncons() {
                    None => {
                        None::<Adt13>
                    }
                    Some((v3, _)) => {
                        Some(Adt13::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5)))
                    }
                }
            } else {
                if f28(v1.clone(), f8(Str::lit("std"))) {
                    let m1169 = v2;
                    match m1169.uncons() {
                        None => {
                            None::<Adt13>
                        }
                        Some((v5, v6)) => {
                            Some(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v5)), std::rc::Rc::new({
                                let m1170 = v6;
                                match m1170.uncons() {
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
                        let m1171 = v2;
                        match m1171.uncons() {
                            None => {
                                None::<Adt13>
                            }
                            Some((v9, v10)) => {
                                let m1172 = v10;
                                match m1172.uncons() {
                                    None => {
                                        None::<Adt13>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt13::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt13::C2(v11)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f323(v12)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt13::C2(v9)), std::rc::Rc::new(Adt13::C5))))))))), std::rc::Rc::new(Adt13::C5)))
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

pub fn f325(v0: Adt1, v1: Bytes, v2: Adt17, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1173 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1173.uncons() {
        None => {
            let m1174 = f324(v4);
            match m1174 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1175 = f8(Str::lit("Catalog."));
                        let a1176 = f321(v3.clone())?;
                        append_bytes(a1175, a1176)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f321(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1177 = f403(v2, v9.clone(), f321(v3.clone())?)?;
                    match m1177 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f321(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f404(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f404(f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt13::C2(v10.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt13::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt13::C1(false, 0u64)), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1181 = f405(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f35(v10.clone(), 4u64, {
                                    let a1179 = {
                                        let a1178 = v10;
                                        length_bytes(a1178)
                                    };
                                    let a1180 = 4u64;
                                    nat_sub(a1179, a1180)
                                }))?;
                                let a1182 = f8(Str::lit("\u{a}"));
                                append_bytes(a1181, a1182)
                            }), List::cons(f231(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f326(v0: Adt17) -> R<Bytes> {
    let m1183 = v0;
    match m1183 {
        Adt17::C0 => {
            Ok(Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
        }
        Adt17::C1(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C2(v3) => {
            if v3 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt17::C3(v4, v5) => {
            f256(v4, v5)
        }
        Adt17::C4(v6, v7, v8) => {
            f406(v6, v7, v8)
        }
        Adt17::C5(v9) => {
            f407(v9)
        }
        Adt17::C6(r1184) => {
            let v10 = (*r1184).clone();
            Ok(f258(f408(v10)?))
        }
        Adt17::C7(r1185) => {
            let v11 = (*r1185).clone();
            f260(f409(v11)?)
        }
        Adt17::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt17::C10(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f327(v0: Option<Adt17>) -> Option<Bytes> {
    let m1186 = v0;
    match m1186 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1187 = v1;
            match m1187 {
                Adt17::C0 => {
                    None::<Bytes>
                }
                Adt17::C1(_, _) => {
                    None::<Bytes>
                }
                Adt17::C2(_) => {
                    None::<Bytes>
                }
                Adt17::C3(_, _) => {
                    None::<Bytes>
                }
                Adt17::C4(_, _, _) => {
                    None::<Bytes>
                }
                Adt17::C5(v2) => {
                    Some(v2)
                }
                Adt17::C6(_) => {
                    None::<Bytes>
                }
                Adt17::C7(_) => {
                    None::<Bytes>
                }
                Adt17::C8 => {
                    None::<Bytes>
                }
                Adt17::C9(_, _) => {
                    None::<Bytes>
                }
                Adt17::C10(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f328(v0: Adt17) -> List<Adt17> {
    let m1188 = v0;
    match m1188 {
        Adt17::C0 => {
            List::<Adt17>::nil()
        }
        Adt17::C1(_, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C2(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C3(_, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C4(_, _, _) => {
            List::<Adt17>::nil()
        }
        Adt17::C5(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C6(r1189) => {
            let v3 = (*r1189).clone();
            f328(v3)
        }
        Adt17::C7(_) => {
            List::<Adt17>::nil()
        }
        Adt17::C8 => {
            List::<Adt17>::nil()
        }
        Adt17::C9(r1190, r1191) => {
            let v1 = (*r1190).clone();
            let v2 = (*r1191).clone();
            List::cons(v1, f328(v2))
        }
        Adt17::C10(_, _, _) => {
            List::<Adt17>::nil()
        }
    }
}

pub fn f329(v0: Adt24) -> Bytes {
    let a1212 = f410(match v0.clone() {
        Adt24::C0(h1192, _, _, _, _, _, _, _) => {
            h1192
        }
    });
    let a1213 = {
        let a1210 = f410(match v0.clone() {
            Adt24::C0(_, h1193, _, _, _, _, _, _) => {
                h1193
            }
        });
        let a1211 = {
            let a1208 = f410(match v0.clone() {
                Adt24::C0(_, _, h1194, _, _, _, _, _) => {
                    h1194
                }
            });
            let a1209 = {
                let a1206 = f410(match v0.clone() {
                    Adt24::C0(_, _, _, h1195, _, _, _, _) => {
                        h1195
                    }
                });
                let a1207 = {
                    let a1204 = f410(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1196, _, _, _) => {
                            h1196
                        }
                    });
                    let a1205 = {
                        let a1202 = f410(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1197, _, _) => {
                                h1197
                            }
                        });
                        let a1203 = {
                            let a1200 = f410(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1198, _) => {
                                    h1198
                                }
                            });
                            let a1201 = f410(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1199) => {
                                    h1199
                                }
                            });
                            append_bytes(a1200, a1201)
                        };
                        append_bytes(a1202, a1203)
                    };
                    append_bytes(a1204, a1205)
                };
                append_bytes(a1206, a1207)
            };
            append_bytes(a1208, a1209)
        };
        append_bytes(a1210, a1211)
    };
    append_bytes(a1212, a1213)
}

pub fn f330(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1214 = v0.clone();
        length_bytes(a1214)
    };
    let v2: u64 = {
        let a1220 = {
            let a1217 = {
                let a1215 = v1;
                let a1216 = 72u64;
                nat_add(a1215, a1216)?
            };
            let a1218 = 64u64;
            let a1219 = 0u64;
            nat_quot(a1217, a1218, a1219)
        };
        let a1221 = 64u64;
        nat_mul(a1220, a1221)?
    };
    let (_, h1222) = f411(Fn11::F412(v0, v1, v2), {
        let a1223 = v2;
        let a1224 = 64u64;
        let a1225 = 0u64;
        nat_quot(a1223, a1224, a1225)
    }, (0u64, f413()))?;
    Ok(h1222)
}

pub fn f331(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Adt17) -> R<Adt11> {
    let m1226 = f414(f415(v1.clone(), v3.clone(), v4, v6))?;
    match m1226 {
        Ok(v8) => {
            let v9: Bytes = f416(v1.clone());
            Ok(Adt11::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f404(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5))))))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f323(v5)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt13::C2(f240(f53(v0, v2))?)), std::rc::Rc::new(Adt13::C5))))))), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f332(p1230: List<Adt13>, v1: Bytes, v2: Bytes) -> bool {
    let m1229 = p1230;
    {
        let mut m1229 = m1229;
        loop {
            let step = {
                let v0 = m1229;
                let m1227 = v0;
                match m1227.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f344(f28(f47(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m1228 = f79(v3, f8(Str::lit("std")));
                            match m1228 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    f28(f47(v5, f8(Str::lit("row"))), v2.clone())
                                }
                            }
                        }) {
                            Err::<List<Adt13>, bool>(true)
                        } else {
                            Ok::<List<Adt13>, bool>(v4)
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

pub fn f333(v0: Adt17, v1: Adt17) -> R<Option<Bytes>> {
    f417({
        let a1237 = {
            let a1232 = {
                let a1231 = f418(v0.clone())?;
                length_bytes(a1231)
            };
            let a1233 = 2u64;
            nat_add(a1232, a1233)?
        };
        let a1238 = {
            let a1235 = {
                let a1234 = f418(v1.clone())?;
                length_bytes(a1234)
            };
            let a1236 = 2u64;
            nat_add(a1235, a1236)?
        };
        nat_mul(a1237, a1238)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f334() -> Bytes {
    let a1239 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1239)
}

pub fn f335(v0: Adt13, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt13> = f79(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f246(Str::lit("id"), f47(v0.clone(), f8(Str::lit("id"))))?, List::cons(f246(Str::lit("name"), f47(v0.clone(), f8(Str::lit("name"))))?, List::cons(f246(Str::lit("issuer"), f47(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f246(Str::lit("edition"), f47(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f246(Str::lit("canonical_id"), f47(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f246(Str::lit("source_role"), f47(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f246(Str::lit("retrieval_date"), f47(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f246(Str::lit("url"), f392(v2.clone(), f8(Str::lit("url"))))?, List::cons(f246(Str::lit("media_type"), f392(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f246(Str::lit("license"), f392(v2.clone(), f8(Str::lit("license"))))?, List::cons(f246(Str::lit("redistribution"), f392(v2, f8(Str::lit("redistribution"))))?, List::cons(f246(Str::lit("statement"), f47(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1240 = v1;
        match m1240 {
            None => {
                f7(List::cons(f246(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f246(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f246(Str::lit("artifact"), {
                    let (h1241, _) = v3;
                    h1241
                })?, List::cons(f246(Str::lit("checksum"), {
                    let (_, h1242) = v3;
                    h1242
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f419(f174(f156(f79(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f336(p1245: List<List<Bytes>>, v1: Bytes, p1246: List<Bytes>) -> R<List<Bytes>> {
    let m1244 = (p1245, p1246);
    {
        let mut m1244 = m1244;
        loop {
            let step = {
                let (v0, v2) = m1244;
                let m1243 = v0;
                match m1243.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f420(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f338(f421(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: Bytes) -> R<Bytes> {
    Ok(f35(v0.clone(), 0u64, f265(v0)?))
}

pub fn f338(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1247 = v0;
    match m1247.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1248 = v3.clone();
            match m1248.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1251 = v2;
                    let a1252 = {
                        let a1249 = v1.clone();
                        let a1250 = f338(v3, v1);
                        append_bytes(a1249, a1250)
                    };
                    append_bytes(a1251, a1252)
                }
            }
        }
    }
}

pub fn f339(v0: Adt13, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1253 = f79(v0, v1);
    match m1253 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v3) => {
            f156(f79(v3, v2))
        }
    }
}

pub fn f340(v0: Adt13, v1: Bytes) -> bool {
    let m1254 = f79(v0, v1);
    match m1254 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f341(v0: Bytes, p1282: u64, p1283: u64, p1284: bool, p1285: bool, p1286: u64, p1287: Bytes) -> R<Bytes> {
    let m1281 = (p1282, (p1283, (p1284, (p1285, (p1286, p1287)))));
    {
        let mut m1281 = m1281;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m1281;
                let m1255 = v2;
                if m1255 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f422(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m1255 - 1;
                    let m1258 = {
                        let a1256 = v0.clone();
                        let a1257 = v1;
                        index_bytes(a1256, a1257)
                    };
                    match m1258 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f422(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a1259 = v1;
                                    let a1260 = 1u64;
                                    nat_add(a1259, a1260)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a1261 = v9;
                                    let a1262 = 34u64;
                                    !nat_eq(a1261, a1262)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1263 = v9;
                                    let a1264 = 92u64;
                                    nat_eq(a1263, a1264)
                                }, (v5, v6))))))
                            } else {
                                let a1265 = v9;
                                let a1266 = 32u64;
                                if if if if nat_eq(a1265, a1266) {
                                    true
                                } else {
                                    let a1267 = v9;
                                    let a1268 = 9u64;
                                    nat_eq(a1267, a1268)
                                } {
                                    true
                                } else {
                                    let a1269 = v9;
                                    let a1270 = 10u64;
                                    nat_eq(a1269, a1270)
                                } {
                                    true
                                } else {
                                    let a1271 = v9;
                                    let a1272 = 13u64;
                                    nat_eq(a1271, a1272)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1273 = v1;
                                        let a1274 = 1u64;
                                        nat_add(a1273, a1274)?
                                    }, (v7, (false, (false, ({
                                        let a1275 = v1;
                                        let a1276 = 1u64;
                                        nat_add(a1275, a1276)?
                                    }, f422(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1277 = v1;
                                        let a1278 = 1u64;
                                        nat_add(a1277, a1278)?
                                    }, (v7, ({
                                        let a1279 = v9;
                                        let a1280 = 34u64;
                                        nat_eq(a1279, a1280)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f342(v0: Bytes, v1: u64) -> R<u64> {
    let m1290 = {
        let a1288 = v0;
        let a1289 = v1;
        index_bytes(a1288, a1289)
    };
    match m1290 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1335 = {
                let a1311 = {
                    let a1299 = {
                        let a1293 = {
                            let a1291 = v3;
                            let a1292 = 128u8;
                            fixed_u8::bit_and(a1291, a1292)
                        };
                        let a1294 = 128u8;
                        if equal(a1293, a1294) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1300 = {
                        let a1297 = {
                            let a1295 = v3;
                            let a1296 = 64u8;
                            fixed_u8::bit_and(a1295, a1296)
                        };
                        let a1298 = 64u8;
                        if equal(a1297, a1298) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1299, a1300)?
                };
                let a1312 = {
                    let a1309 = {
                        let a1303 = {
                            let a1301 = v3;
                            let a1302 = 32u8;
                            fixed_u8::bit_and(a1301, a1302)
                        };
                        let a1304 = 32u8;
                        if equal(a1303, a1304) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1310 = {
                        let a1307 = {
                            let a1305 = v3;
                            let a1306 = 16u8;
                            fixed_u8::bit_and(a1305, a1306)
                        };
                        let a1308 = 16u8;
                        if equal(a1307, a1308) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1309, a1310)?
                };
                nat_add(a1311, a1312)?
            };
            let a1336 = {
                let a1333 = {
                    let a1321 = {
                        let a1315 = {
                            let a1313 = v3;
                            let a1314 = 8u8;
                            fixed_u8::bit_and(a1313, a1314)
                        };
                        let a1316 = 8u8;
                        if equal(a1315, a1316) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1322 = {
                        let a1319 = {
                            let a1317 = v3;
                            let a1318 = 4u8;
                            fixed_u8::bit_and(a1317, a1318)
                        };
                        let a1320 = 4u8;
                        if equal(a1319, a1320) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1321, a1322)?
                };
                let a1334 = {
                    let a1331 = {
                        let a1325 = {
                            let a1323 = v3;
                            let a1324 = 2u8;
                            fixed_u8::bit_and(a1323, a1324)
                        };
                        let a1326 = 2u8;
                        if equal(a1325, a1326) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1332 = {
                        let a1329 = {
                            let a1327 = v3;
                            let a1328 = 1u8;
                            fixed_u8::bit_and(a1327, a1328)
                        };
                        let a1330 = 1u8;
                        if equal(a1329, a1330) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1331, a1332)?
                };
                nat_add(a1333, a1334)?
            };
            nat_add(a1335, a1336)
        }
    }
}

pub fn f343(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f423(v0.clone(), v1, {
        let a1338 = {
            let a1337 = v0.clone();
            length_bytes(a1337)
        };
        let a1339 = 1u64;
        nat_add(a1338, a1339)?
    })?;
    let a1340 = f342(v0.clone(), v2)?;
    let a1341 = 34u64;
    if nat_eq(a1340, a1341) {
        Ok(Some(({
            let a1342 = v2;
            let a1343 = 1u64;
            nat_add(a1342, a1343)?
        }, f35(v0, v1, {
            let a1344 = v2;
            let a1345 = v1;
            nat_sub(a1344, a1345)
        }))))
    } else {
        f424(v0, v1)
    }
}

pub fn f344(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f345(v0: Bytes, v1: u64) -> R<Option<(u64, Adt17)>> {
    let v2: bool = {
        let a1346 = f342(v0.clone(), v1)?;
        let a1347 = 45u64;
        nat_eq(a1346, a1347)
    };
    let v3: u64 = if v2 {
        let a1348 = v1;
        let a1349 = 1u64;
        nat_add(a1348, a1349)?
    } else {
        v1
    };
    let v4: u64 = f425(v0.clone(), v3)?;
    let v5: u64 = {
        let a1350 = v4;
        let a1351 = v3;
        nat_sub(a1350, a1351)
    };
    let v6: bool = {
        let a1352 = f342(v0.clone(), v4)?;
        let a1353 = 46u64;
        nat_eq(a1352, a1353)
    };
    let v7: u64 = if v6 {
        let a1354 = v4;
        let a1355 = 1u64;
        nat_add(a1354, a1355)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f425(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f342(v0.clone(), v8)?;
    let v10: bool = {
        let a1356 = v9;
        let a1357 = 101u64;
        if nat_eq(a1356, a1357) {
            true
        } else {
            let a1358 = v9;
            let a1359 = 69u64;
            nat_eq(a1358, a1359)
        }
    };
    let v11: u64 = f342(v0.clone(), {
        let a1360 = v8;
        let a1361 = 1u64;
        nat_add(a1360, a1361)?
    })?;
    let v12: bool = f344(v10, f426({
        let a1362 = v11;
        let a1363 = 43u64;
        nat_eq(a1362, a1363)
    }, {
        let a1364 = v11;
        let a1365 = 45u64;
        nat_eq(a1364, a1365)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a1366 = v8;
            let a1367 = 2u64;
            nat_add(a1366, a1367)?
        } else {
            let a1368 = v8;
            let a1369 = 1u64;
            nat_add(a1368, a1369)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f425(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a1370 = v14;
        let a1371 = v13;
        nat_sub(a1370, a1371)
    });
    let v16: u64 = f427(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a1373 = {
            let a1372 = v15;
            length_bytes(a1372)
        };
        let a1374 = v16;
        nat_sub(a1373, a1374)
    });
    let a1375 = v5;
    let a1376 = 0u64;
    if if nat_eq(a1375, a1376) {
        true
    } else {
        if f344({
            let a1377 = 1u64;
            let a1378 = v5;
            nat_lt(a1377, a1378)
        }, {
            let a1379 = f342(v0.clone(), v3)?;
            let a1380 = 48u64;
            nat_eq(a1379, a1380)
        }) {
            true
        } else {
            if f344(v6, {
                let a1381 = v8;
                let a1382 = v7;
                nat_eq(a1381, a1382)
            }) {
                true
            } else {
                f426(if v10 {
                    let a1383 = v14;
                    let a1384 = v13;
                    nat_eq(a1383, a1384)
                } else {
                    false
                }, {
                    let a1386 = 18u64;
                    let a1387 = {
                        let a1385 = v17.clone();
                        length_bytes(a1385)
                    };
                    nat_lt(a1386, a1387)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt17)>)
    } else {
        let m1391 = {
            let a1389 = {
                let a1388 = v17.clone();
                length_bytes(a1388)
            };
            let a1390 = 0u64;
            if nat_eq(a1389, a1390) {
                Some(0u64)
            } else {
                f428(v17)?
            }
        };
        match m1391 {
            None => {
                Ok(None::<(u64, Adt17)>)
            }
            Some(v18) => {
                let m1396 = f429(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a1392 = v8;
                    let a1393 = v7;
                    nat_sub(a1392, a1393)
                }), f344(v12, {
                    let a1394 = v11;
                    let a1395 = 45u64;
                    nat_eq(a1394, a1395)
                }), v18)?;
                match m1396 {
                    None => {
                        Ok(None::<(u64, Adt17)>)
                    }
                    Some(v19) => {
                        let a1399 = 19u64;
                        let a1400 = {
                            let a1398 = {
                                let (h1397, _) = v19.clone();
                                h1397
                            };
                            length_bytes(a1398)
                        };
                        if nat_lt(a1399, a1400) {
                            Ok(None::<(u64, Adt17)>)
                        } else {
                            let m1402 = f428({
                                let (h1401, _) = v19.clone();
                                h1401
                            })?;
                            match m1402 {
                                None => {
                                    Ok(None::<(u64, Adt17)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f344(v2, {
                                        let a1403 = 0u64;
                                        let a1404 = v20;
                                        nat_lt(a1403, a1404)
                                    });
                                    if f430(v21, v20) {
                                        Ok(Some((v14, {
                                            let a1406 = {
                                                let (_, h1405) = v19.clone();
                                                h1405
                                            };
                                            let a1407 = 0u64;
                                            if nat_eq(a1406, a1407) {
                                                Adt17::C3(v21, v20)
                                            } else {
                                                Adt17::C4(v21, v20, {
                                                    let (_, h1408) = v19;
                                                    h1408
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt17)>)
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
    if f199(v0.clone(), Bytes::lit(&[0x0d])) {
        f35(v0.clone(), 0u64, {
            let a1410 = {
                let a1409 = v0;
                length_bytes(a1409)
            };
            let a1411 = 1u64;
            nat_sub(a1410, a1411)
        })
    } else {
        v0
    }
}

pub fn f347(v0: Adt13, v1: Bytes) -> Adt13 {
    let m1412 = v0;
    match m1412 {
        Adt13::C0(v3) => {
            Adt13::C0(v3)
        }
        Adt13::C1(v4, v5) => {
            Adt13::C1(v4, v5)
        }
        Adt13::C2(v2) => {
            Adt13::C2(f431(f49(), v2, v1))
        }
        Adt13::C3(r1413) => {
            let v6 = (*r1413).clone();
            Adt13::C3(std::rc::Rc::new(v6))
        }
        Adt13::C4(r1414) => {
            let v7 = (*r1414).clone();
            Adt13::C4(std::rc::Rc::new(v7))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r1415, r1416) => {
            let v8 = (*r1415).clone();
            let v9 = (*r1416).clone();
            Adt13::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt13::C7(v10, r1417, r1418) => {
            let v11 = (*r1417).clone();
            let v12 = (*r1418).clone();
            Adt13::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f348(v0: u64) -> R<Bytes> {
    f432(f433(), 10u64, v0)
}

pub fn f349(v0: Fn3, p1423: u64, p1424: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1422 = (p1423, p1424);
    {
        let mut m1422 = m1422;
        loop {
            let step = {
                let (v1, v2) = m1422;
                let m1419 = v1;
                if m1419 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1419 - 1;
                    let m1421 = {
                        let c1420: Fn3 = v0.clone();
                        c1420.apply(v2.clone())?
                    };
                    match m1421 {
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
                Ok(next) => { m1422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f434(v0, v1)
}

pub fn f351(v0: Fn4, p1428: List<(Bytes, Bytes)>, p1429: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1427 = (p1428, p1429);
    {
        let mut m1427 = m1427;
        loop {
            let step = {
                let (v1, v2) = m1427;
                let m1425 = v2;
                match m1425.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1426: Fn4 = v0.clone();
                            c1426.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f435(v0, v1)
}

pub fn f353(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f436(v0)
}

pub fn f354(v0: Fn5, p1433: List<Bytes>, p1434: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1432 = (p1433, p1434);
    {
        let mut m1432 = m1432;
        loop {
            let step = {
                let (v1, v2) = m1432;
                let m1430 = v2;
                match m1430.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1431: Fn5 = v0.clone();
                            c1431.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f437(v1)?, v0))
}

pub fn f356(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f257(v0)?, List::cons(f8(Str::lit(":")), List::cons(f257(v1)?, List::<Bytes>::nil())))))
}

pub fn f357(v0: Adt12) -> Bytes {
    let m1435 = v0;
    match m1435 {
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
    let m1436 = v0;
    match m1436 {
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
    let m1437 = v0;
    match m1437 {
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

pub fn f360(v0: Fn6, p1442: u64, p1443: u64) -> R<(u64, bool)> {
    let m1441 = (p1442, p1443);
    {
        let mut m1441 = m1441;
        loop {
            let step = {
                let (v1, v2) = m1441;
                let m1438 = v1;
                if m1438 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1438 - 1;
                    let m1440 = {
                        let c1439: Fn6 = v0.clone();
                        c1439.apply(v2)?
                    };
                    match m1440 {
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
                Ok(next) => { m1441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: Bytes, v1: u64) -> Option<u64> {
    let a1444 = 0u64;
    let a1445 = v1;
    if nat_lt(a1444, a1445) {
        if f438(v0, {
            let a1446 = v1;
            let a1447 = 1u64;
            nat_sub(a1446, a1447)
        }) {
            Some({
                let a1448 = v1;
                let a1449 = 1u64;
                nat_sub(a1448, a1449)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f362(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1450 = v2;
    let a1451 = v0;
    if nat_lt(a1450, a1451) {
        if f438(v1, v2) {
            Ok(Some({
                let a1452 = v2;
                let a1453 = 1u64;
                nat_add(a1452, a1453)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f363(v0: (Bytes, Bytes), p1456: List<(Bytes, Bytes)>) -> bool {
    let m1455 = p1456;
    {
        let mut m1455 = m1455;
        loop {
            let step = {
                let v1 = m1455;
                let m1454 = v1;
                match m1454.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f439(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1457 = v1;
    match m1457.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h1458, _) = v2;
                h1458
            }, v0.clone()) {
                List::cons({
                    let (_, h1459) = v2;
                    h1459
                }, f364(v0, v3))
            } else {
                f364(v0, v3)
            }
        }
    }
}

pub fn f365() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f366(v0: Adt13, v1: Bytes, v2: Bytes) -> bool {
    let m1460 = f79(v0, v1);
    match m1460 {
        None => {
            false
        }
        Some(v3) => {
            f180(v3, v2)
        }
    }
}

pub fn f367() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f368(v0: Bytes) -> bool {
    f199(v0, f8(Str::lit(".lex.tex")))
}

pub fn f369(v0: Bytes) -> R<bool> {
    let v1: Bytes = f166(v0)?;
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
        let a1462 = {
            let a1461 = v1;
            length_bytes(a1461)
        };
        let a1463 = v2;
        nat_sub(a1462, a1463)
    });
    let v5: Bytes = if f72(v3.clone(), f8(Str::lit("("))) {
        let m1464 = f278(v3.clone(), f8(Str::lit(")")))?;
        match m1464 {
            None => {
                v3
            }
            Some(v4) => {
                f35(v3.clone(), {
                    let a1465 = v4;
                    let a1466 = 1u64;
                    nat_add(a1465, a1466)?
                }, {
                    let a1470 = {
                        let a1467 = v3;
                        length_bytes(a1467)
                    };
                    let a1471 = {
                        let a1468 = v4;
                        let a1469 = 1u64;
                        nat_add(a1468, a1469)?
                    };
                    nat_sub(a1470, a1471)
                })
            }
        }
    } else {
        v3
    };
    let a1472 = 0u64;
    let a1473 = v2;
    if nat_lt(a1472, a1473) {
        f440(v5)
    } else {
        Ok(false)
    }
}

pub fn f370(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f371(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1476 = {
            let a1474 = v4;
            let a1475 = v1;
            nat_add(a1474, a1475)?
        };
        let a1477 = v3;
        if nat_lt(a1476, a1477) {
            Ok(Some({
                let a1478 = v4;
                let a1479 = 1u64;
                nat_add(a1478, a1479)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f371(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1484 = {
        let a1481 = v0;
        let a1482 = v2;
        let a1483 = {
            let a1480 = v1.clone();
            length_bytes(a1480)
        };
        slice_bytes(a1481, a1482, a1483)
    };
    match m1484 {
        None => {
            false
        }
        Some(v3) => {
            let a1485 = v3;
            let a1486 = v1;
            equal(a1485, a1486)
        }
    }
}

pub fn f372(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1487 = v0;
    match m1487.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f372(v3, {
                let a1488 = v1;
                let a1489 = 1u64;
                nat_add(a1488, a1489)?
            })?))
        }
    }
}

pub fn f373(v0: Adt19, v1: List<u64>, v2: Adt22) -> bool {
    let m1490 = v2;
    match m1490 {
        Adt22::C0(v3, v4) => {
            if f379(v3, match v0.clone() {
                Adt19::C0(h1491, _, _, _) => {
                    h1491
                }
            }) {
                f441(v0, v4)
            } else {
                false
            }
        }
        Adt22::C1(v5) => {
            f379(v5, match v0 {
                Adt19::C0(_, _, h1492, _) => {
                    h1492
                }
            })
        }
        Adt22::C2(_, v7, v8) => {
            if f375(v7, v1) {
                f379(v8, match v0 {
                    Adt19::C0(_, _, _, h1493) => {
                        h1493
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f374(v0: Adt15, v1: Adt13) -> R<Adt22> {
    let m1494 = f79(v1.clone(), f8(Str::lit("std")));
    match m1494 {
        None => {
            let m1499 = f78(f79(v1.clone(), f8(Str::lit("req"))));
            match m1499 {
                None => {
                    let m1501 = f79(v1, f8(Str::lit("der")));
                    match m1501 {
                        None => {
                            Ok(f442(v0))
                        }
                        Some(v5) => {
                            let m1502 = f443(f47(v5.clone(), f8(Str::lit("rule"))));
                            match m1502 {
                                None => {
                                    Ok(f442(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt22::C2(v6, f286(f174(f156(f79(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1503) => {
                                            h1503
                                        }
                                    })?, f377(f47(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1504, _) => {
                                            h1504
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt22::C1(f377(v4, match v0 {
                        Adt15::C0(_, _, h1500, _, _) => {
                            h1500
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1495 = f78(f79(v2.clone(), f8(Str::lit("row"))));
            match m1495 {
                None => {
                    Ok(Adt22::C0(f377(f47(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1496, _, _, _, _) => {
                            h1496
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt22::C0(f377(f47(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1497, _, _, _, _) => {
                            h1497
                        }
                    })?, Some(f377(v3, match v0 {
                        Adt15::C0(_, h1498, _, _, _) => {
                            h1498
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f375(p1507: List<u64>, v1: List<u64>) -> bool {
    let m1506 = p1507;
    {
        let mut m1506 = m1506;
        loop {
            let step = {
                let v0 = m1506;
                let m1505 = v0;
                match m1505.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f379(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(p1512: List<u64>) -> bool {
    let m1511 = p1512;
    {
        let mut m1511 = m1511;
        loop {
            let step = {
                let v0 = m1511;
                let m1508 = v0;
                match m1508.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1509 = v1;
                        let a1510 = 7u64;
                        if nat_le(a1509, a1510) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1513 = f444(v0, v1.clone(), 0u64)?;
    match m1513 {
        None => {
            let a1514 = v1;
            Ok(length_list(a1514))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f378(v0: Adt13) -> u64 {
    let m1515 = f445(f79(v0, f8(Str::lit("stratum"))));
    match m1515 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f379(v0: u64, p1520: List<u64>) -> bool {
    let m1519 = p1520;
    {
        let mut m1519 = m1519;
        loop {
            let step = {
                let v1 = m1519;
                let m1516 = v1;
                match m1516.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1517 = v2;
                        let a1518 = v0;
                        if nat_eq(a1517, a1518) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
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

pub fn f380(v0: Fn7, p1524: List<Bytes>, p1525: List<(Bytes, ())>) -> List<Bytes> {
    let m1523 = (p1524, p1525);
    {
        let mut m1523 = m1523;
        loop {
            let step = {
                let (v1, v2) = m1523;
                let m1521 = v2;
                match m1521.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1522: Fn7 = v0.clone();
                            c1522.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1526, _) = v1;
        h1526
    }, v0)
}

pub fn f382(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f446(Fn12::F447, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f383(v0: Fn8, p1530: List<List<(Bytes, ())>>, p1531: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1529 = (p1530, p1531);
    {
        let mut m1529 = m1529;
        loop {
            let step = {
                let (v1, v2) = m1529;
                let m1527 = v2;
                match m1527.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1528: Fn8 = v0.clone();
                            c1528.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1529 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f385(v0: Fn9, p1536: u64, p1537: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1535 = (p1536, p1537);
    {
        let mut m1535 = m1535;
        loop {
            let step = {
                let (v1, v2) = m1535;
                let m1532 = v1;
                if m1532 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1532 - 1;
                    let m1534 = {
                        let c1533: Fn9 = v0.clone();
                        c1533.apply(v2.clone())?
                    };
                    match m1534 {
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
                Ok(next) => { m1535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f448(v0)
}

pub fn f387(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f449(Fn13::F450, List::<(Bytes, ())>::nil(), v0)
}

pub fn f388(v0: Fn10, p1541: List<(Bytes, ())>, p1542: List<Bytes>) -> List<(Bytes, ())> {
    let m1540 = (p1541, p1542);
    {
        let mut m1540 = m1540;
        loop {
            let step = {
                let (v1, v2) = m1540;
                let m1538 = v2;
                match m1538.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1539: Fn10 = v0.clone();
                            c1539.apply(v1, v3)
                        }, v4))
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

pub fn f389(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f390(p1545: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1544 = p1545;
    {
        let mut m1544 = m1544;
        loop {
            let step = {
                let v0 = m1544;
                let m1543 = v0;
                match m1543.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f451(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Bytes, p1571: u64, p1572: u64, p1573: Bytes) -> R<Bytes> {
    let m1570 = (p1571, (p1572, p1573));
    {
        let mut m1570 = m1570;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1570;
                let m1546 = v2;
                if m1546 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1546 - 1;
                    let m1549 = {
                        let a1547 = v0.clone();
                        let a1548 = v1;
                        index_bytes(a1547, a1548)
                    };
                    match m1549 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f426({
                                let a1550 = v6;
                                let a1551 = 34u64;
                                nat_eq(a1550, a1551)
                            }, {
                                let a1552 = v6;
                                let a1553 = 92u64;
                                nat_eq(a1552, a1553)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1554 = v1;
                                    let a1555 = 1u64;
                                    nat_add(a1554, a1555)?
                                }, (v4, {
                                    let a1558 = v3;
                                    let a1559 = {
                                        let a1556 = f8(Str::lit("\\"));
                                        let a1557 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1556, a1557)
                                    };
                                    append_bytes(a1558, a1559)
                                })))
                            } else {
                                let a1560 = v6;
                                let a1561 = 10u64;
                                if nat_eq(a1560, a1561) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1562 = v1;
                                        let a1563 = 1u64;
                                        nat_add(a1562, a1563)?
                                    }, (v4, {
                                        let a1564 = v3;
                                        let a1565 = f8(Str::lit("\\n"));
                                        append_bytes(a1564, a1565)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1566 = v1;
                                        let a1567 = 1u64;
                                        nat_add(a1566, a1567)?
                                    }, (v4, {
                                        let a1568 = v3;
                                        let a1569 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1568, a1569)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1570 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: Option<Adt13>, v1: Bytes) -> Bytes {
    let m1574 = v0;
    match m1574 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f47(v2, v1)
        }
    }
}

pub fn f393(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f81(f53(v0.clone(), v1.clone()))?;
    let m1575 = f249(v2.clone())?;
    match m1575 {
        None => {
            f453(v0, v1, v2)
        }
        Some(v3) => {
            f452(v0, v1, v3)
        }
    }
}

pub fn f394(v0: List<Bytes>) -> List<Bytes> {
    let m1576 = v0;
    match m1576.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1578 = {
                    let a1577 = v1.clone();
                    length_bytes(a1577)
                };
                let a1579 = 0u64;
                if nat_eq(a1578, a1579) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f394(v2))
        }
    }
}

pub fn f395(v0: Bytes, v1: u64) -> R<u64> {
    let m1582 = {
        let a1580 = v0;
        let a1581 = v1;
        index_bytes(a1580, a1581)
    };
    match m1582 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f62(v2)
        }
    }
}

pub fn f396(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1586 = {
        let a1583 = v1;
        let a1584 = v2;
        let a1585 = 1u64;
        slice_bytes(a1583, a1584, a1585)
    };
    match m1586 {
        None => {
            v0
        }
        Some(v3) => {
            let a1587 = v0;
            let a1588 = v3;
            append_bytes(a1587, a1588)
        }
    }
}

pub fn f397(p1591: List<Bytes>, p1592: List<Bytes>) -> List<Bytes> {
    let m1590 = (p1591, p1592);
    {
        let mut m1590 = m1590;
        loop {
            let step = {
                let (v0, v1) = m1590;
                let m1589 = v0;
                match m1589.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: List<Bytes>) -> Bytes {
    let m1593 = v0;
    match m1593.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f399(v0: Bytes) -> R<bool> {
    let m1595 = {
        let a1594 = v0.clone();
        length_bytes(a1594)
    };
    if m1595 == 0 {
        Ok(true)
    } else {
        let v1 = m1595 - 1;
        let m1598 = {
            let a1596 = v0;
            let a1597 = v1;
            index_bytes(a1596, a1597)
        };
        match m1598 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f454({
                    let v3: u8 = v2;
                    let a1643 = {
                        let a1619 = {
                            let a1607 = {
                                let a1601 = {
                                    let a1599 = v3;
                                    let a1600 = 128u8;
                                    fixed_u8::bit_and(a1599, a1600)
                                };
                                let a1602 = 128u8;
                                if equal(a1601, a1602) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1608 = {
                                let a1605 = {
                                    let a1603 = v3;
                                    let a1604 = 64u8;
                                    fixed_u8::bit_and(a1603, a1604)
                                };
                                let a1606 = 64u8;
                                if equal(a1605, a1606) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1607, a1608)?
                        };
                        let a1620 = {
                            let a1617 = {
                                let a1611 = {
                                    let a1609 = v3;
                                    let a1610 = 32u8;
                                    fixed_u8::bit_and(a1609, a1610)
                                };
                                let a1612 = 32u8;
                                if equal(a1611, a1612) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1618 = {
                                let a1615 = {
                                    let a1613 = v3;
                                    let a1614 = 16u8;
                                    fixed_u8::bit_and(a1613, a1614)
                                };
                                let a1616 = 16u8;
                                if equal(a1615, a1616) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1617, a1618)?
                        };
                        nat_add(a1619, a1620)?
                    };
                    let a1644 = {
                        let a1641 = {
                            let a1629 = {
                                let a1623 = {
                                    let a1621 = v3;
                                    let a1622 = 8u8;
                                    fixed_u8::bit_and(a1621, a1622)
                                };
                                let a1624 = 8u8;
                                if equal(a1623, a1624) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1630 = {
                                let a1627 = {
                                    let a1625 = v3;
                                    let a1626 = 4u8;
                                    fixed_u8::bit_and(a1625, a1626)
                                };
                                let a1628 = 4u8;
                                if equal(a1627, a1628) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1629, a1630)?
                        };
                        let a1642 = {
                            let a1639 = {
                                let a1633 = {
                                    let a1631 = v3;
                                    let a1632 = 2u8;
                                    fixed_u8::bit_and(a1631, a1632)
                                };
                                let a1634 = 2u8;
                                if equal(a1633, a1634) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1640 = {
                                let a1637 = {
                                    let a1635 = v3;
                                    let a1636 = 1u8;
                                    fixed_u8::bit_and(a1635, a1636)
                                };
                                let a1638 = 1u8;
                                if equal(a1637, a1638) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1639, a1640)?
                        };
                        nat_add(a1641, a1642)?
                    };
                    nat_add(a1643, a1644)?
                }))
            }
        }
    }
}

pub fn f400(v0: Adt17, v1: Bytes) -> bool {
    let a1646 = {
        let a1645 = f236(v0, v1);
        length_bytes(a1645)
    };
    let a1647 = 0u64;
    !nat_eq(a1646, a1647)
}

pub fn f401(v0: Adt17, v1: Bytes) -> bool {
    let m1648 = f233(v0, v1);
    match m1648 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f402(v0: Bytes, p1669: u64, p1670: u64, p1671: bool, p1672: Bytes) -> R<Bytes> {
    let m1668 = (p1669, (p1670, (p1671, p1672)));
    {
        let mut m1668 = m1668;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1668;
                let m1649 = v2;
                if m1649 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1649 - 1;
                    let m1652 = {
                        let a1650 = v0.clone();
                        let a1651 = v1;
                        index_bytes(a1650, a1651)
                    };
                    match m1652 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f455(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1653 = v1;
                                    let a1654 = 1u64;
                                    nat_add(a1653, a1654)?
                                }, (v5, (true, v4))))
                            } else {
                                if f344(f456(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1655 = v1;
                                        let a1656 = 1u64;
                                        nat_add(a1655, a1656)?
                                    }, (v5, (false, {
                                        let a1660 = v4;
                                        let a1661 = f35({
                                            let a1657 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1657)
                                        }, {
                                            let a1658 = v7;
                                            let a1659 = 97u64;
                                            nat_sub(a1658, a1659)
                                        }, 1u64);
                                        append_bytes(a1660, a1661)
                                    }))))
                                } else {
                                    if f426(f456(v7), f426(f457(v7), f458(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1662 = v1;
                                            let a1663 = 1u64;
                                            nat_add(a1662, a1663)?
                                        }, (v5, (false, {
                                            let a1664 = v4;
                                            let a1665 = f35(v0.clone(), v1, 1u64);
                                            append_bytes(a1664, a1665)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1666 = v1;
                                            let a1667 = 1u64;
                                            nat_add(a1666, a1667)?
                                        }, (v5, (v3, v4))))
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

pub fn f403(v0: Adt17, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt17)> = f459(f460(f233(v0.clone(), f8(Str::lit("$defs")))), f460(f233(v0.clone(), f8(Str::lit("definitions")))));
    let m1673 = f461(4096u64, v3.clone(), List::<Bytes>::nil(), f462(v3)?)?;
    match m1673 {
        Ok(v5) => {
            let m1674 = f463(4096u64, f464(f465(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1674 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f338(f202(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f404(v0: List<Adt13>, v1: Bytes, v2: Bytes, v3: Adt13) -> List<Adt13> {
    let m1675 = v0;
    match m1675.uncons() {
        None => {
            List::cons(v3, List::<Adt13>::nil())
        }
        Some((v4, v5)) => {
            if f28(f47(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f466(v5, v1, v2))
            } else {
                List::cons(v4, f404(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f405(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f338(f467(f81(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f406(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f348(v1)?;
    let v4: u64 = {
        let a1676 = v3.clone();
        length_bytes(a1676)
    };
    let v5: Bytes = {
        let a1677 = v2;
        let a1678 = v4;
        if nat_lt(a1677, a1678) {
            f7(List::cons(f35(v3.clone(), 0u64, {
                let a1679 = v4;
                let a1680 = v2;
                nat_sub(a1679, a1680)
            }), List::cons(f8(Str::lit(".")), List::cons(f35(v3, {
                let a1681 = v4;
                let a1682 = v2;
                nat_sub(a1681, a1682)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f468({
                let a1683 = v2;
                let a1684 = v4;
                nat_sub(a1683, a1684)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1685 = f8(Str::lit("-"));
        let a1686 = v5;
        Ok(append_bytes(a1685, a1686))
    } else {
        Ok(v5)
    }
}

pub fn f407(v0: Bytes) -> R<Bytes> {
    let a1691 = f423(v0.clone(), 0u64, {
        let a1688 = {
            let a1687 = v0.clone();
            length_bytes(a1687)
        };
        let a1689 = 1u64;
        nat_add(a1688, a1689)?
    })?;
    let a1692 = {
        let a1690 = v0.clone();
        length_bytes(a1690)
    };
    if nat_eq(a1691, a1692) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f257(v0)
    }
}

pub fn f408(v0: Adt17) -> R<List<Bytes>> {
    let m1693 = v0;
    match m1693 {
        Adt17::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C3(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C4(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt17::C9(r1694, r1695) => {
            let v12 = (*r1694).clone();
            let v13 = (*r1695).clone();
            Ok(List::cons(f326(v12)?, f408(v13)?))
        }
        Adt17::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f409(v0: Adt17) -> R<List<(Bytes, Bytes)>> {
    let m1696 = v0;
    match m1696 {
        Adt17::C0 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C3(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C4(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt17::C10(v14, r1697, r1698) => {
            let v15 = (*r1697).clone();
            let v16 = (*r1698).clone();
            Ok(List::cons((v14, f326(v15)?), f409(v16)?))
        }
    }
}

pub fn f410(v0: u32) -> Bytes {
    let a1727 = f469({
        let a1699 = f470(v0, 28u32);
        let a1700 = 15u32;
        fixed_u32::bit_and(a1699, a1700)
    });
    let a1728 = {
        let a1725 = f469({
            let a1701 = f470(v0, 24u32);
            let a1702 = 15u32;
            fixed_u32::bit_and(a1701, a1702)
        });
        let a1726 = {
            let a1723 = f469({
                let a1703 = f470(v0, 20u32);
                let a1704 = 15u32;
                fixed_u32::bit_and(a1703, a1704)
            });
            let a1724 = {
                let a1721 = f469({
                    let a1705 = f470(v0, 16u32);
                    let a1706 = 15u32;
                    fixed_u32::bit_and(a1705, a1706)
                });
                let a1722 = {
                    let a1719 = f469({
                        let a1707 = f470(v0, 12u32);
                        let a1708 = 15u32;
                        fixed_u32::bit_and(a1707, a1708)
                    });
                    let a1720 = {
                        let a1717 = f469({
                            let a1709 = f470(v0, 8u32);
                            let a1710 = 15u32;
                            fixed_u32::bit_and(a1709, a1710)
                        });
                        let a1718 = {
                            let a1715 = f469({
                                let a1711 = f470(v0, 4u32);
                                let a1712 = 15u32;
                                fixed_u32::bit_and(a1711, a1712)
                            });
                            let a1716 = f469({
                                let a1713 = v0;
                                let a1714 = 15u32;
                                fixed_u32::bit_and(a1713, a1714)
                            });
                            append_bytes(a1715, a1716)
                        };
                        append_bytes(a1717, a1718)
                    };
                    append_bytes(a1719, a1720)
                };
                append_bytes(a1721, a1722)
            };
            append_bytes(a1723, a1724)
        };
        append_bytes(a1725, a1726)
    };
    append_bytes(a1727, a1728)
}

pub fn f411(v0: Fn11, p1732: u64, p1733: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1731 = (p1732, p1733);
    {
        let mut m1731 = m1731;
        loop {
            let step = {
                let (v1, v2) = m1731;
                let m1729 = v1;
                if m1729 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1729 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1730: Fn11 = v0.clone();
                        c1730.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1735 = {
            let (h1734, _) = v3.clone();
            h1734
        };
        let a1736 = 64u64;
        nat_add(a1735, a1736)?
    }, f471({
        let (_, h1737) = v3;
        h1737
    }, f472(v0, v1, v2, {
        let (h1738, _) = v3;
        h1738
    })?)?))
}

pub fn f413() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f414(v0: Adt17) -> R<Result<Bytes, Bytes>> {
    let m1739 = f473(v0.clone());
    match m1739 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f326(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f415(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt17) -> Adt17 {
    Adt17::C7(std::rc::Rc::new(Adt17::C10(f8(Str::lit("citation")), std::rc::Rc::new(Adt17::C6(std::rc::Rc::new(f474(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt17::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt17::C5(v2)), std::rc::Rc::new(Adt17::C10(f8(Str::lit("pointer")), std::rc::Rc::new(Adt17::C5(Bytes::lit(&[]))), std::rc::Rc::new(Adt17::C10(f8(Str::lit("value")), std::rc::Rc::new(v3), std::rc::Rc::new(Adt17::C8))))))))))
}

pub fn f416(v0: Bytes) -> Bytes {
    f7(List::cons(f112(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f417(v0: u64, v1: Adt17, v2: Adt17, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m1740 = v0;
    if m1740 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m1740 - 1;
        let m1741 = v2.clone();
        match m1741 {
            Adt17::C0 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C1(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C2(v6) => {
                if v6 {
                    Ok(None::<Bytes>)
                } else {
                    Ok(f475(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
                }
            }
            Adt17::C3(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C4(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C5(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C6(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C7(r1755) => {
                let v7 = (*r1755).clone();
                let m1742 = f476(f477(v7));
                match m1742 {
                    None => {
                        Ok(f478(f479(v2.clone(), v3.clone(), v4.clone())?, f478({
                            let m1743 = f327(f233(v2.clone(), f8(Str::lit("$ref"))));
                            match m1743 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m1744 = f480(v1.clone(), v9)?;
                                    match m1744 {
                                        Ok(v11) => {
                                            f417(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f478(f481(v5, v1.clone(), f238(f233(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f478({
                            let m1745 = f233(v2.clone(), f8(Str::lit("anyOf")));
                            match m1745 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a1746 = 0u64;
                                    let a1747 = f482(v5, v1.clone(), f238(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a1746, a1747) {
                                        None::<Bytes>
                                    } else {
                                        f475(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f478({
                            let m1748 = f233(v2.clone(), f8(Str::lit("oneOf")));
                            match m1748 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a1749 = f482(v5, v1.clone(), f238(Some(v13)), v3.clone(), v4.clone())?;
                                    let a1750 = 1u64;
                                    if nat_eq(a1749, a1750) {
                                        None::<Bytes>
                                    } else {
                                        f475(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m1751 = v3.clone();
                            match m1751 {
                                Adt17::C0 => {
                                    None::<Bytes>
                                }
                                Adt17::C1(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C2(_) => {
                                    None::<Bytes>
                                }
                                Adt17::C3(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C4(_, _, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C5(_) => {
                                    None::<Bytes>
                                }
                                Adt17::C6(r1753) => {
                                    let v15 = (*r1753).clone();
                                    let m1752 = f233(v2, f8(Str::lit("items")));
                                    match m1752 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f485(v5, v1, v16, f328(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt17::C7(r1754) => {
                                    let v14 = (*r1754).clone();
                                    f478(f483(v5, v1.clone(), f460(f233(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f484(v5, v1, v2, f477(v14), v4)?)
                                }
                                Adt17::C8 => {
                                    None::<Bytes>
                                }
                                Adt17::C9(_, _) => {
                                    None::<Bytes>
                                }
                                Adt17::C10(_, _, _) => {
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
            Adt17::C8 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C9(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt17::C10(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
        }
    }
}

pub fn f418(v0: Adt17) -> R<Bytes> {
    f326(v0)
}

pub fn f419(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f338(f486(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f420(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1756 = f33(v1, 34u8)?;
    match m1756.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1757 = v3;
            match m1757.uncons() {
                None => {
                    Ok(f487(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f421(v0: List<Bytes>) -> List<Bytes> {
    let m1758 = v0;
    match m1758.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("authority = []"))) {
                f421(v2)
            } else {
                List::cons(v1, f421(v2))
            }
        }
    }
}

pub fn f422(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1761 = v3;
    let a1762 = f35(v0, v1, {
        let a1759 = v2;
        let a1760 = v1;
        nat_sub(a1759, a1760)
    });
    append_bytes(a1761, a1762)
}

pub fn f423(v0: Bytes, p1775: u64, p1776: u64) -> R<u64> {
    let m1774 = (p1775, p1776);
    {
        let mut m1774 = m1774;
        loop {
            let step = {
                let (v1, v2) = m1774;
                let m1763 = v2;
                if m1763 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1763 - 1;
                    let v4: u64 = f342(v0.clone(), v1)?;
                    let a1764 = v4;
                    let a1765 = 34u64;
                    if nat_eq(a1764, a1765) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1766 = v4;
                        let a1767 = 92u64;
                        if nat_eq(a1766, a1767) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1768 = v4;
                            let a1769 = 32u64;
                            if nat_lt(a1768, a1769) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1770 = v4;
                                let a1771 = 256u64;
                                if nat_eq(a1770, a1771) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1772 = v1;
                                        let a1773 = 1u64;
                                        nat_add(a1772, a1773)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f488(Fn14::F489(v0.clone()), {
        let a1778 = {
            let a1777 = v0;
            length_bytes(a1777)
        };
        let a1779 = 1u64;
        nat_add(a1778, a1779)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1783 = {
        let (_, h1780) = {
            let (_, h1781) = {
                let (h1782, _) = v2.clone();
                h1782
            };
            h1781
        };
        h1780
    };
    let a1784 = 1u64;
    if nat_eq(a1783, a1784) {
        Ok(Some(({
            let (h1785, _) = {
                let (h1786, _) = v2.clone();
                h1786
            };
            h1785
        }, {
            let (h1787, _) = {
                let (_, h1788) = {
                    let (h1789, _) = v2;
                    h1789
                };
                h1788
            };
            h1787
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f425(v0: Bytes, v1: u64) -> R<u64> {
    let (h1790, _) = f360(Fn6::F490(v0.clone()), {
        let a1792 = {
            let a1791 = v0;
            length_bytes(a1791)
        };
        let a1793 = 1u64;
        nat_add(a1792, a1793)?
    }, v1)?;
    Ok(h1790)
}

pub fn f426(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f427(v0: Bytes) -> R<u64> {
    let (h1794, _) = f360(Fn6::F491(v0.clone()), {
        let a1796 = {
            let a1795 = v0;
            length_bytes(a1795)
        };
        let a1797 = 1u64;
        nat_add(a1796, a1797)?
    }, 0u64)?;
    Ok(h1794)
}

pub fn f428(v0: Bytes) -> R<Option<u64>> {
    if f492(v0.clone())? {
        Ok(Some({
            let (_, h1798) = {
                let (h1799, _) = f493(Fn15::F494(v0.clone()), {
                    let a1800 = v0;
                    length_bytes(a1800)
                }, (0u64, 0u64))?;
                h1799
            };
            h1798
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f429(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a1802 = {
            let a1801 = v1.clone();
            length_bytes(a1801)
        };
        let a1803 = f495(v1)?;
        nat_sub(a1802, a1803)
    });
    let v5: Bytes = {
        let a1804 = v0;
        let a1805 = v4.clone();
        append_bytes(a1804, a1805)
    };
    let v6: u64 = f427(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a1807 = {
            let a1806 = v5;
            length_bytes(a1806)
        };
        let a1808 = v6;
        nat_sub(a1807, a1808)
    });
    let v8: u64 = {
        let a1809 = v4;
        length_bytes(a1809)
    };
    let v9: u64 = {
        let a1810 = v7.clone();
        length_bytes(a1810)
    };
    let a1811 = v9;
    let a1812 = 0u64;
    if nat_eq(a1811, a1812) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f426(v2, {
            let a1813 = v3;
            let a1814 = v8;
            nat_lt(a1813, a1814)
        }) {
            let v10: u64 = if v2 {
                let a1815 = v3;
                let a1816 = v8;
                nat_add(a1815, a1816)?
            } else {
                let a1817 = v8;
                let a1818 = v3;
                nat_sub(a1817, a1818)
            };
            let v11: u64 = f495(v7.clone())?;
            let a1819 = v10;
            let a1820 = v11;
            if nat_le(a1819, a1820) {
                Ok(Some((f35(v7, 0u64, {
                    let a1821 = v9;
                    let a1822 = v10;
                    nat_sub(a1821, a1822)
                }), 0u64)))
            } else {
                let a1825 = {
                    let a1823 = v10;
                    let a1824 = v11;
                    nat_sub(a1823, a1824)
                };
                let a1826 = 19u64;
                if nat_le(a1825, a1826) {
                    Ok(Some((f35(v7, 0u64, {
                        let a1827 = v9;
                        let a1828 = v11;
                        nat_sub(a1827, a1828)
                    }), {
                        let a1829 = v10;
                        let a1830 = v11;
                        nat_sub(a1829, a1830)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1831 = v3;
                let a1832 = v8;
                nat_sub(a1831, a1832)
            };
            let a1835 = {
                let a1833 = v9;
                let a1834 = v12;
                nat_add(a1833, a1834)?
            };
            let a1836 = 19u64;
            if nat_le(a1835, a1836) {
                Ok(Some(({
                    let a1837 = v7;
                    let a1838 = f468(v12);
                    append_bytes(a1837, a1838)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f430(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1839 = 0u64;
        let a1840 = v1;
        if nat_lt(a1839, a1840) {
            let a1841 = v1;
            let a1842 = 9223372036854775808u64;
            nat_le(a1841, a1842)
        } else {
            false
        }
    } else {
        let a1843 = v1;
        let a1844 = 9223372036854775807u64;
        nat_le(a1843, a1844)
    }
}

pub fn f431(p1847: List<Adt12>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1846 = p1847;
    {
        let mut m1846 = m1846;
        loop {
            let step = {
                let v0 = m1846;
                let m1845 = v0;
                match m1845.uncons() {
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
                Ok(next) => { m1846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1848 = v2;
    let a1849 = 0u64;
    if nat_eq(a1848, a1849) {
        Ok(f496(v0, 0u64))
    } else {
        let (_, h1850) = {
            let (h1851, _) = f349(Fn3::F497(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1851
        };
        Ok(h1850)
    }
}

pub fn f433() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f434(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1855 = {
        let a1853 = v0.clone();
        let a1854 = {
            let (h1852, _) = v1.clone();
            h1852
        };
        index_bytes(a1853, a1854)
    };
    match m1855 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1857 = {
                    let (h1856, _) = v1.clone();
                    h1856
                };
                let a1858 = 1u64;
                nat_add(a1857, a1858)?
            }, {
                let a1861 = {
                    let (_, h1859) = v1;
                    h1859
                };
                let a1862 = f498(v0, {
                    let (h1860, _) = v1;
                    h1860
                }, v2)?;
                append_bytes(a1861, a1862)
            })))
        }
    }
}

pub fn f435(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1863 = v0.clone();
    match m1863.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1866 = {
                let (h1864, _) = v2;
                h1864
            };
            let a1867 = {
                let (h1865, _) = v1.clone();
                h1865
            };
            if equal(a1866, a1867) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f436(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f499(f500(Fn16::F501, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1872 = {
        let (h1868, _) = f502(Fn17::F503, {
            let a1870 = {
                let a1869 = v0;
                length_list(a1869)
            };
            let a1871 = 1u64;
            nat_add(a1870, a1871)?
        }, v1)?;
        h1868
    };
    match m1872.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f437(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1877 = {
        let a1874 = f257({
            let (h1873, _) = v0;
            h1873
        })?;
        let a1875 = Bytes::lit(&[0x3a]);
        append_bytes(a1874, a1875)
    };
    let a1878 = {
        let (_, h1876) = v0;
        h1876
    };
    Ok(append_bytes(a1877, a1878))
}

pub fn f438(v0: Bytes, v1: u64) -> bool {
    let m1879 = f167(v0, v1);
    match m1879 {
        None => {
            false
        }
        Some(v2) => {
            f504(v2)
        }
    }
}

pub fn f439(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f344(f28({
        let (h1880, _) = v0;
        h1880
    }, {
        let (h1881, _) = v1;
        h1881
    }), f28({
        let (_, h1882) = v0;
        h1882
    }, {
        let (_, h1883) = v1;
        h1883
    }))
}

pub fn f440(v0: Bytes) -> R<bool> {
    let m1884 = f278(v0.clone(), f8(Str::lit("&&")))?;
    if match m1884 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1885 = f278(v0, f8(Str::lit("||")))?;
        match m1885 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f441(v0: Adt19, v1: Option<u64>) -> bool {
    let m1886 = v1;
    match m1886 {
        None => {
            true
        }
        Some(v2) => {
            f379(v2, match v0 {
                Adt19::C0(_, h1887, _, _) => {
                    h1887
                }
            })
        }
    }
}

pub fn f442(v0: Adt15) -> Adt22 {
    Adt22::C1({
        let a1889 = match v0 {
            Adt15::C0(_, _, h1888, _, _) => {
                h1888
            }
        };
        length_list(a1889)
    })
}

pub fn f443(v0: Bytes) -> Option<Adt23> {
    if f28(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt23::C0)
    } else {
        if f28(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt23::C1)
        } else {
            if f28(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt23::C2)
            } else {
                None::<Adt23>
            }
        }
    }
}

pub fn f444(v0: Bytes, p1894: List<Bytes>, p1895: u64) -> R<Option<u64>> {
    let m1893 = (p1894, p1895);
    {
        let mut m1893 = m1893;
        loop {
            let step = {
                let (v1, v2) = m1893;
                let m1890 = v1;
                match m1890.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f28(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1891 = v2;
                                let a1892 = 1u64;
                                nat_add(a1891, a1892)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: Option<Adt13>) -> Option<u64> {
    let m1896 = v0;
    match m1896 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1897 = v1;
            match m1897 {
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

pub fn f446(v0: Fn12, p1901: List<List<(Bytes, ())>>, p1902: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1900 = (p1901, p1902);
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let (v1, v2) = m1900;
                let m1898 = v2;
                match m1898.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1899: Fn12 = v0.clone();
                            c1899.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f448(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1904 = {
        let a1903 = v0.clone();
        length_list(a1903)
    };
    let a1905 = 1u64;
    if nat_le(a1904, a1905) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f505(v0)?))
    }
}

pub fn f449(v0: Fn13, p1909: List<(Bytes, ())>, p1910: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1908 = (p1909, p1910);
    {
        let mut m1908 = m1908;
        loop {
            let step = {
                let (v1, v2) = m1908;
                let m1906 = v2;
                match m1906.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1907: Fn13 = v0.clone();
                            c1907.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f451(p1914: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1913 = p1914;
    {
        let mut m1913 = m1913;
        loop {
            let step = {
                let v0 = m1913;
                let m1911 = v0;
                match m1911.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f506(v2, v1.clone(), 0u64, {
                            let a1912 = v1.clone();
                            length_bytes(a1912)
                        })? {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1913 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: Adt1, v1: Bytes, v2: List<Adt13>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f115(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f51(v3.clone(), f115(f82(v0, f125())?, f8(Str::lit("at"))));
    let a1916 = {
        let a1915 = v4.clone();
        length_list(a1915)
    };
    let a1917 = 0u64;
    if nat_eq(a1916, a1917) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1920 = {
            let a1918 = v4;
            length_list(a1918)
        };
        let a1921 = {
            let a1919 = v3;
            length_list(a1919)
        };
        if nat_eq(a1920, a1921) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f507(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f453(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt13> = f316(f227(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt13> = f316(f227(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f98(f115(v3.clone(), f8(Str::lit("id"))), f115(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f115(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f51(v5.clone(), f98(v6.clone(), f115(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1923 = {
        let a1922 = v7.clone();
        length_list(a1922)
    };
    let a1924 = 0u64;
    if nat_eq(a1923, a1924) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f40({
            let a1927 = {
                let a1925 = v7;
                length_list(a1925)
            };
            let a1928 = {
                let a1926 = v5;
                length_list(a1926)
            };
            nat_eq(a1927, a1928)
        }) {
            Ok((false, f507(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f118(f98(f51(f508(v4.clone()), f52(f53(v0, f8(Str::lit("model/authorities.toml"))))?), f51(f509(v4), f98(v6, f115(v3, f8(Str::lit("id")))))))?;
            let a1930 = {
                let a1929 = v8.clone();
                length_list(a1929)
            };
            let a1931 = 0u64;
            if nat_eq(a1930, a1931) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f507(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f95(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f454(v0: u64) -> bool {
    let a1932 = v0;
    let a1933 = 47u64;
    nat_eq(a1932, a1933)
}

pub fn f455(v0: u64) -> bool {
    f426({
        let a1934 = v0;
        let a1935 = 95u64;
        nat_eq(a1934, a1935)
    }, f426({
        let a1936 = v0;
        let a1937 = 45u64;
        nat_eq(a1936, a1937)
    }, f426({
        let a1938 = v0;
        let a1939 = 32u64;
        nat_eq(a1938, a1939)
    }, f426({
        let a1940 = v0;
        let a1941 = 46u64;
        nat_eq(a1940, a1941)
    }, {
        let a1942 = v0;
        let a1943 = 47u64;
        nat_eq(a1942, a1943)
    }))))
}

pub fn f456(v0: u64) -> bool {
    f344({
        let a1944 = 97u64;
        let a1945 = v0;
        nat_le(a1944, a1945)
    }, {
        let a1946 = v0;
        let a1947 = 122u64;
        nat_le(a1946, a1947)
    })
}

pub fn f457(v0: u64) -> bool {
    f344({
        let a1948 = 65u64;
        let a1949 = v0;
        nat_le(a1948, a1949)
    }, {
        let a1950 = v0;
        let a1951 = 90u64;
        nat_le(a1950, a1951)
    })
}

pub fn f458(v0: u64) -> bool {
    f344({
        let a1952 = 48u64;
        let a1953 = v0;
        nat_le(a1952, a1953)
    }, {
        let a1954 = v0;
        let a1955 = 57u64;
        nat_le(a1954, a1955)
    })
}

pub fn f459(v0: List<(Bytes, Adt17)>, v1: List<(Bytes, Adt17)>) -> List<(Bytes, Adt17)> {
    let m1956 = v0;
    match m1956.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f459(v3, v1))
        }
    }
}

pub fn f460(v0: Option<Adt17>) -> List<(Bytes, Adt17)> {
    let m1957 = v0;
    match m1957 {
        None => {
            List::<(Bytes, Adt17)>::nil()
        }
        Some(v1) => {
            f477(v1)
        }
    }
}

pub fn f461(v0: u64, v1: List<(Bytes, Adt17)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt17)>, Bytes>> {
    let m1958 = v0;
    if m1958 == 0 {
        let m1959 = v1;
        match m1959.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::<(Bytes, Adt17)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt17)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1958 - 1;
        let m1960 = v1.clone();
        match m1960.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::<(Bytes, Adt17)>::nil()))
            }
            Some((_, _)) => {
                let m1961 = f510(v1.clone(), v2.clone(), v3.clone())?;
                match m1961 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt17)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f338(f462(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1964 = f461(v6, f511(v1, {
                            let (h1962, _) = v9.clone();
                            h1962
                        }), List::cons(f512({
                            let (h1963, _) = v9.clone();
                            h1963
                        })?, v2), v3)?;
                        match m1964 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt17)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt17)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f462(v0: List<(Bytes, Adt17)>) -> R<List<Bytes>> {
    let m1965 = v0;
    match m1965.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f512({
                let (h1966, _) = v1;
                h1966
            })?, f462(v2)?))
        }
    }
}

pub fn f463(p1972: u64, p1973: List<Adt25>, p1974: List<Bytes>, p1975: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1971 = (p1972, (p1973, (p1974, p1975)));
    {
        let mut m1971 = m1971;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1971;
                let m1967 = v0;
                if m1967 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1967 - 1;
                    let m1968 = v1;
                    match m1968.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1969 = v5;
                            match m1969 {
                                Adt25::C0(v8, v9) => {
                                    if f28(v8.clone(), f513()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f41(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1970 = f515(v8.clone(), v9)?;
                                            match m1970 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f464(f516(v11.clone()), List::cons(Adt25::C1(f517(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f41(f513(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f513(), v2), List::cons(f514(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m1976 = v0;
    match m1976.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f464(v3, v1))
        }
    }
}

pub fn f465(v0: List<(Bytes, Adt17)>) -> R<List<Adt25>> {
    let m1977 = v0;
    match m1977.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f512({
                let (h1978, _) = v1;
                h1978
            })?, {
                let (_, h1979) = v1;
                h1979
            }), f465(v2)?))
        }
    }
}

pub fn f466(v0: List<Adt13>, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1980 = v0;
    match m1980.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            if f28(f47(v3.clone(), v1.clone()), v2.clone()) {
                f466(v4, v1, v2)
            } else {
                List::cons(v3, f466(v4, v1, v2))
            }
        }
    }
}

pub fn f467(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1981 = v0;
    match m1981.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f72(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f338(f486(f118(List::cons(v1, f518(f33(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f467(v3, v1)?))
            }
        }
    }
}

pub fn f468(v0: u64) -> Bytes {
    let m1982 = v0;
    if m1982 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1982 - 1;
        let a1983 = f8(Str::lit("0"));
        let a1984 = f468(v1);
        append_bytes(a1983, a1984)
    }
}

pub fn f469(v0: u32) -> Bytes {
    let a1987 = {
        let a1985 = v0;
        let a1986 = 8u32;
        fixed_u32::bit_and(a1985, a1986)
    };
    let a1988 = 0u32;
    if equal(a1987, a1988) {
        let a1991 = {
            let a1989 = v0;
            let a1990 = 4u32;
            fixed_u32::bit_and(a1989, a1990)
        };
        let a1992 = 0u32;
        if equal(a1991, a1992) {
            let a1995 = {
                let a1993 = v0;
                let a1994 = 2u32;
                fixed_u32::bit_and(a1993, a1994)
            };
            let a1996 = 0u32;
            if equal(a1995, a1996) {
                let a1999 = {
                    let a1997 = v0;
                    let a1998 = 1u32;
                    fixed_u32::bit_and(a1997, a1998)
                };
                let a2000 = 0u32;
                if equal(a1999, a2000) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2003 = {
                    let a2001 = v0;
                    let a2002 = 1u32;
                    fixed_u32::bit_and(a2001, a2002)
                };
                let a2004 = 0u32;
                if equal(a2003, a2004) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2007 = {
                let a2005 = v0;
                let a2006 = 2u32;
                fixed_u32::bit_and(a2005, a2006)
            };
            let a2008 = 0u32;
            if equal(a2007, a2008) {
                let a2011 = {
                    let a2009 = v0;
                    let a2010 = 1u32;
                    fixed_u32::bit_and(a2009, a2010)
                };
                let a2012 = 0u32;
                if equal(a2011, a2012) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2015 = {
                    let a2013 = v0;
                    let a2014 = 1u32;
                    fixed_u32::bit_and(a2013, a2014)
                };
                let a2016 = 0u32;
                if equal(a2015, a2016) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2019 = {
            let a2017 = v0;
            let a2018 = 4u32;
            fixed_u32::bit_and(a2017, a2018)
        };
        let a2020 = 0u32;
        if equal(a2019, a2020) {
            let a2023 = {
                let a2021 = v0;
                let a2022 = 2u32;
                fixed_u32::bit_and(a2021, a2022)
            };
            let a2024 = 0u32;
            if equal(a2023, a2024) {
                let a2027 = {
                    let a2025 = v0;
                    let a2026 = 1u32;
                    fixed_u32::bit_and(a2025, a2026)
                };
                let a2028 = 0u32;
                if equal(a2027, a2028) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2031 = {
                    let a2029 = v0;
                    let a2030 = 1u32;
                    fixed_u32::bit_and(a2029, a2030)
                };
                let a2032 = 0u32;
                if equal(a2031, a2032) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2035 = {
                let a2033 = v0;
                let a2034 = 2u32;
                fixed_u32::bit_and(a2033, a2034)
            };
            let a2036 = 0u32;
            if equal(a2035, a2036) {
                let a2039 = {
                    let a2037 = v0;
                    let a2038 = 1u32;
                    fixed_u32::bit_and(a2037, a2038)
                };
                let a2040 = 0u32;
                if equal(a2039, a2040) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2043 = {
                    let a2041 = v0;
                    let a2042 = 1u32;
                    fixed_u32::bit_and(a2041, a2042)
                };
                let a2044 = 0u32;
                if equal(a2043, a2044) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f470(v0: u32, v1: u32) -> u32 {
    let m2047 = {
        let a2045 = v0;
        let a2046 = v1;
        fixed_u32::shift_right(a2045, a2046)
    };
    match m2047 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f471(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f519(Fn18::F520, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2048, _, _, _, _, _, _, _) => {
            h2048
        }
    }, match v0.clone() {
        Adt24::C0(_, h2049, _, _, _, _, _, _) => {
            h2049
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2050, _, _, _, _, _) => {
            h2050
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2051, _, _, _, _) => {
            h2051
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2052, _, _, _) => {
            h2052
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2053, _, _) => {
            h2053
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2054, _) => {
            h2054
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2055) => {
            h2055
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f521(match v0.clone() {
        Adt24::C0(h2056, _, _, _, _, _, _, _) => {
            h2056
        }
    }, match v2.clone() {
        Adt27::C0(h2057, _, _, _, _, _, _, _, _, _) => {
            h2057
        }
    }), f521(match v0.clone() {
        Adt24::C0(_, h2058, _, _, _, _, _, _) => {
            h2058
        }
    }, match v2.clone() {
        Adt27::C0(_, h2059, _, _, _, _, _, _, _, _) => {
            h2059
        }
    }), f521(match v0.clone() {
        Adt24::C0(_, _, h2060, _, _, _, _, _) => {
            h2060
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2061, _, _, _, _, _, _, _) => {
            h2061
        }
    }), f521(match v0.clone() {
        Adt24::C0(_, _, _, h2062, _, _, _, _) => {
            h2062
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2063, _, _, _, _, _, _) => {
            h2063
        }
    }), f521(match v0.clone() {
        Adt24::C0(_, _, _, _, h2064, _, _, _) => {
            h2064
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2065, _, _, _, _, _) => {
            h2065
        }
    }), f521(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2066, _, _) => {
            h2066
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2067, _, _, _, _) => {
            h2067
        }
    }), f521(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2068, _) => {
            h2068
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2069, _, _, _) => {
            h2069
        }
    }), f521(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2070) => {
            h2070
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2071, _, _) => {
            h2071
        }
    })))
}

pub fn f472(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f522(v0.clone(), v1, v2, v3)?, f522(v0.clone(), v1, v2, {
        let a2072 = v3;
        let a2073 = 4u64;
        nat_add(a2072, a2073)?
    })?, f522(v0.clone(), v1, v2, {
        let a2074 = v3;
        let a2075 = 8u64;
        nat_add(a2074, a2075)?
    })?, f522(v0.clone(), v1, v2, {
        let a2076 = v3;
        let a2077 = 12u64;
        nat_add(a2076, a2077)?
    })?, f522(v0.clone(), v1, v2, {
        let a2078 = v3;
        let a2079 = 16u64;
        nat_add(a2078, a2079)?
    })?, f522(v0.clone(), v1, v2, {
        let a2080 = v3;
        let a2081 = 20u64;
        nat_add(a2080, a2081)?
    })?, f522(v0.clone(), v1, v2, {
        let a2082 = v3;
        let a2083 = 24u64;
        nat_add(a2082, a2083)?
    })?, f522(v0.clone(), v1, v2, {
        let a2084 = v3;
        let a2085 = 28u64;
        nat_add(a2084, a2085)?
    })?, f522(v0.clone(), v1, v2, {
        let a2086 = v3;
        let a2087 = 32u64;
        nat_add(a2086, a2087)?
    })?, f522(v0.clone(), v1, v2, {
        let a2088 = v3;
        let a2089 = 36u64;
        nat_add(a2088, a2089)?
    })?, f522(v0.clone(), v1, v2, {
        let a2090 = v3;
        let a2091 = 40u64;
        nat_add(a2090, a2091)?
    })?, f522(v0.clone(), v1, v2, {
        let a2092 = v3;
        let a2093 = 44u64;
        nat_add(a2092, a2093)?
    })?, f522(v0.clone(), v1, v2, {
        let a2094 = v3;
        let a2095 = 48u64;
        nat_add(a2094, a2095)?
    })?, f522(v0.clone(), v1, v2, {
        let a2096 = v3;
        let a2097 = 52u64;
        nat_add(a2096, a2097)?
    })?, f522(v0.clone(), v1, v2, {
        let a2098 = v3;
        let a2099 = 56u64;
        nat_add(a2098, a2099)?
    })?, f522(v0, v1, v2, {
        let a2100 = v3;
        let a2101 = 60u64;
        nat_add(a2100, a2101)?
    })?))
}

pub fn f473(v0: Adt17) -> Option<Bytes> {
    let m2102 = v0;
    match m2102 {
        Adt17::C0 => {
            None::<Bytes>
        }
        Adt17::C1(v1, _) => {
            Some(v1)
        }
        Adt17::C2(_) => {
            None::<Bytes>
        }
        Adt17::C3(_, _) => {
            None::<Bytes>
        }
        Adt17::C4(_, _, _) => {
            None::<Bytes>
        }
        Adt17::C5(_) => {
            None::<Bytes>
        }
        Adt17::C6(r2105) => {
            let v3 = (*r2105).clone();
            f473(v3)
        }
        Adt17::C7(r2106) => {
            let v4 = (*r2106).clone();
            f473(v4)
        }
        Adt17::C8 => {
            None::<Bytes>
        }
        Adt17::C9(r2107, r2108) => {
            let v5 = (*r2107).clone();
            let v6 = (*r2108).clone();
            let m2103 = f473(v5);
            match m2103 {
                None => {
                    f473(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt17::C10(_, r2109, r2110) => {
            let v9 = (*r2109).clone();
            let v10 = (*r2110).clone();
            let m2104 = f473(v9);
            match m2104 {
                None => {
                    f473(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f474(v0: List<Bytes>) -> Adt17 {
    let m2111 = v0;
    match m2111.uncons() {
        None => {
            Adt17::C8
        }
        Some((v1, v2)) => {
            Adt17::C9(std::rc::Rc::new(Adt17::C5(v1)), std::rc::Rc::new(f474(v2)))
        }
    }
}

pub fn f475(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f476(p2116: List<(Bytes, Adt17)>) -> Option<Bytes> {
    let m2115 = p2116;
    {
        let mut m2115 = m2115;
        loop {
            let step = {
                let v0 = m2115;
                let m2112 = v0;
                match m2112.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h2113, _) = v1.clone();
                            h2113
                        }, f523()) {
                            Ok::<List<(Bytes, Adt17)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt17)>, Option<Bytes>>(Some({
                                let (h2114, _) = v1;
                                h2114
                            }))
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

pub fn f477(v0: Adt17) -> List<(Bytes, Adt17)> {
    let m2117 = v0;
    match m2117 {
        Adt17::C0 => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C1(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C2(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C3(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C4(_, _, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C5(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C6(_) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C7(r2118) => {
            let v4 = (*r2118).clone();
            f477(v4)
        }
        Adt17::C8 => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C9(_, _) => {
            List::<(Bytes, Adt17)>::nil()
        }
        Adt17::C10(v1, r2119, r2120) => {
            let v2 = (*r2119).clone();
            let v3 = (*r2120).clone();
            List::cons((v1, v2), f477(v3))
        }
    }
}

pub fn f478(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2121 = v0;
    match m2121 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f479(v0: Adt17, v1: Adt17, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f524(v1.clone());
    let v7: Option<Bytes> = {
        let m2122 = f233(v0.clone(), f8(Str::lit("type")));
        match m2122 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2123 = f327(Some(v4.clone()));
                    match m2123 {
                        None => {
                            f237(f238(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f525(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f475(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f338(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2124 = f233(v0.clone(), f8(Str::lit("enum")));
        match m2124 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f526(f238(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f475(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2125 = f233(v0.clone(), f8(Str::lit("const")));
        match m2125 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f527(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f475(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v41: Option<Bytes> = {
        let m2126 = v1.clone();
        match m2126 {
            Adt17::C0 => {
                None::<Bytes>
            }
            Adt17::C1(v28, _) => {
                f475(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
            }
            Adt17::C2(_) => {
                None::<Bytes>
            }
            Adt17::C3(_, _) => {
                None::<Bytes>
            }
            Adt17::C4(_, _, _) => {
                None::<Bytes>
            }
            Adt17::C5(v12) => {
                let m2127 = f528(v12.clone())?;
                match m2127 {
                    None => {
                        f475(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f529(v13)?;
                        f478({
                            let m2128 = f530(f233(v0.clone(), f8(Str::lit("minLength"))));
                            match m2128 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2129 = v14;
                                    let a2130 = v15;
                                    if nat_lt(a2129, a2130) {
                                        f475(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f478({
                            let m2131 = f530(f233(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2131 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2132 = v16;
                                    let a2133 = v14;
                                    if nat_lt(a2132, a2133) {
                                        f475(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2134 = f327(f233(v0.clone(), f8(Str::lit("pattern"))));
                            match m2134 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2135 = f531(v17.clone(), v12)?;
                                    match m2135 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f475(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
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
            Adt17::C6(r2143) => {
                let v20 = (*r2143).clone();
                let v21: u64 = f532(v20)?;
                f478({
                    let m2136 = f530(f233(v0.clone(), f8(Str::lit("minItems"))));
                    match m2136 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2137 = v21;
                            let a2138 = v22;
                            if nat_lt(a2137, a2138) {
                                f475(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2139 = f530(f233(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2139 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2140 = v23;
                            let a2141 = v21;
                            if nat_lt(a2140, a2141) {
                                f475(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt17::C7(r2144) => {
                let v24 = (*r2144).clone();
                let v25: List<Bytes> = f533(f237(f238(f233(v0.clone(), f8(Str::lit("required"))))), f534(f477(v24)));
                let m2142 = v25;
                match m2142.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f475(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
                    }
                }
            }
            Adt17::C8 => {
                None::<Bytes>
            }
            Adt17::C9(_, _) => {
                None::<Bytes>
            }
            Adt17::C10(_, _, _) => {
                None::<Bytes>
            }
        }
    };
    let v47: Option<Bytes> = {
        let m2145 = f535(v1);
        match m2145 {
            None => {
                None::<Bytes>
            }
            Some(v42) => {
                f478({
                    let m2147 = {
                        let m2146 = f233(v0.clone(), f8(Str::lit("minimum")));
                        match m2146 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v43) => {
                                f535(v43)
                            }
                        }
                    };
                    match m2147 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v44) => {
                            if f536(v42, v44)? {
                                f475(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2149 = {
                        let m2148 = f233(v0, f8(Str::lit("maximum")));
                        match m2148 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v45) => {
                                f535(v45)
                            }
                        }
                    };
                    match m2149 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v46) => {
                            if f536(v46, v42)? {
                                f475(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f478(v7, f478(v9, f478(v11, f478(v41, v47)))))
}

pub fn f480(v0: Adt17, v1: Bytes) -> R<Result<Adt17, Bytes>> {
    let v2: u64 = {
        let a2150 = v1.clone();
        length_bytes(a2150)
    };
    if f28(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt17, Bytes>(v0))
    } else {
        let m2155 = if f72(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f35(v1.clone(), 8u64, {
                let a2151 = v2;
                let a2152 = 8u64;
                nat_sub(a2151, a2152)
            })))
        } else {
            if f72(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f35(v1.clone(), 14u64, {
                    let a2153 = v2;
                    let a2154 = 14u64;
                    nat_sub(a2153, a2154)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2155 {
            None => {
                Ok(Err::<Adt17, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f537({
                    let a2158 = {
                        let a2157 = {
                            let (_, h2156) = v3.clone();
                            h2156
                        };
                        length_bytes(a2157)
                    };
                    let a2159 = 1u64;
                    nat_add(a2158, a2159)?
                }, {
                    let (_, h2160) = v3;
                    h2160
                }, 0u64, Bytes::lit(&[]))?;
                let m2163 = {
                    let m2162 = f233(v0, {
                        let (h2161, _) = v3;
                        h2161
                    });
                    match m2162 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v5) => {
                            f233(v5, v4)
                        }
                    }
                };
                match m2163 {
                    None => {
                        Ok(Err::<Adt17, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" names no definition of this schema (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok(Ok::<Adt17, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f481(v0: u64, v1: Adt17, v2: List<Adt17>, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m2164 = v0;
    if m2164 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2164 - 1;
        let m2165 = v2;
        match m2165.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f478(f417(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f481(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f482(v0: u64, v1: Adt17, v2: List<Adt17>, v3: Adt17, v4: Bytes) -> R<u64> {
    let m2166 = v0;
    if m2166 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2166 - 1;
        let m2167 = v2;
        match m2167.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2169 = {
                    let m2168 = f417(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2168 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2170 = f482(v5, v1, v7, v3, v4)?;
                nat_add(a2169, a2170)
            }
        }
    }
}

pub fn f483(v0: u64, v1: Adt17, v2: List<(Bytes, Adt17)>, v3: Adt17, v4: Bytes) -> R<Option<Bytes>> {
    let m2171 = v0;
    if m2171 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2171 - 1;
        let m2172 = v2;
        match m2172.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f478({
                    let m2174 = f233(v3.clone(), {
                        let (h2173, _) = v6.clone();
                        h2173
                    });
                    match m2174 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f417(v5, v1.clone(), {
                                let (_, h2175) = v6;
                                h2175
                            }, v8, f538(v4.clone(), {
                                let (h2176, _) = v6;
                                h2176
                            })?)?
                        }
                    }
                }, f483(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f484(v0: u64, v1: Adt17, v2: Adt17, v3: List<(Bytes, Adt17)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2177 = v0;
    if m2177 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2177 - 1;
        let m2178 = v3;
        match m2178.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f538(v4.clone(), {
                    let (h2179, _) = v6.clone();
                    h2179
                })?;
                let m2181 = f539(f460(f233(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2180, _) = v6.clone();
                    h2180
                })?;
                match m2181 {
                    Ok(v10) => {
                        Ok(f478(f481(v5, v1.clone(), v10.clone(), {
                            let (_, h2182) = v6.clone();
                            h2182
                        }, v8.clone())?, f478({
                            let m2183 = v10;
                            match m2183.uncons() {
                                None => {
                                    if f41({
                                        let (h2184, _) = v6;
                                        h2184
                                    }, f534(f477({
                                        let m2185 = f233(v2.clone(), f8(Str::lit("properties")));
                                        match m2185 {
                                            None => {
                                                Adt17::C7(std::rc::Rc::new(Adt17::C8))
                                            }
                                            Some(v13) => {
                                                v13
                                            }
                                        }
                                    }))) {
                                        None::<Bytes>
                                    } else {
                                        let m2186 = f233(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2186 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f417(v5, v1.clone(), v14, {
                                                    let (_, h2187) = v6;
                                                    h2187
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f484(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f485(v0: u64, v1: Adt17, v2: Adt17, v3: List<Adt17>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2188 = v0;
    if m2188 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2188 - 1;
        let m2189 = v3;
        match m2189.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f478(f417(v6, v1.clone(), v2.clone(), v7, f538(v4.clone(), f348(v5)?)?)?, f485(v6, v1, v2, v8, v4, {
                    let a2190 = v5;
                    let a2191 = 1u64;
                    nat_add(a2190, a2191)?
                })?))
            }
        }
    }
}

pub fn f486(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2192 = v0;
    match m2192.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f296(v1)?, f486(v2)?))
        }
    }
}

pub fn f487(p2195: List<Bytes>, v1: Bytes) -> bool {
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
                        if f72(v2, v1.clone()) {
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

pub fn f488(v0: Fn14, p2200: u64, p2201: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2199 = (p2200, p2201);
    {
        let mut m2199 = m2199;
        loop {
            let step = {
                let (v1, v2) = m2199;
                let m2196 = v1;
                if m2196 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2196 - 1;
                    let m2198 = {
                        let c2197: Fn14 = v0.clone();
                        c2197.apply(v2.clone())?
                    };
                    match m2198 {
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
                Ok(next) => { m2199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f540(v0, v1)
}

pub fn f490(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f541(v0, v1)
}

pub fn f491(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f542(v0, v1)
}

pub fn f492(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2202 = v0.clone();
        length_bytes(a2202)
    };
    let a2203 = 0u64;
    let a2204 = v1;
    if nat_lt(a2203, a2204) {
        let a2205 = f543(v0.clone())?;
        let a2206 = v1;
        if nat_eq(a2205, a2206) {
            let a2207 = 1u64;
            let a2208 = v1;
            if nat_lt(a2207, a2208) {
                let m2211 = {
                    let a2209 = v0;
                    let a2210 = 0u64;
                    index_bytes(a2209, a2210)
                };
                match m2211 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2212 = v2;
                        let a2213 = 48u8;
                        Ok(!equal(a2212, a2213))
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

pub fn f493(v0: Fn15, p2218: u64, p2219: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2217 = (p2218, p2219);
    {
        let mut m2217 = m2217;
        loop {
            let step = {
                let (v1, v2) = m2217;
                let m2214 = v1;
                if m2214 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2214 - 1;
                    let m2216 = {
                        let c2215: Fn15 = v0.clone();
                        c2215.apply(v2)?
                    };
                    match m2216 {
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
                Ok(next) => { m2217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f544(v0, v1)
}

pub fn f495(v0: Bytes) -> R<u64> {
    let (h2220, _) = f360(Fn6::F545(v0.clone()), {
        let a2222 = {
            let a2221 = v0;
            length_bytes(a2221)
        };
        let a2223 = 1u64;
        nat_add(a2222, a2223)?
    }, 0u64)?;
    Ok(h2220)
}

pub fn f496(v0: Bytes, v1: u64) -> Bytes {
    let m2227 = {
        let a2224 = v0;
        let a2225 = v1;
        let a2226 = 1u64;
        slice_bytes(a2224, a2225, a2226)
    };
    match m2227 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f497(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f546(v1, v0, v2)
}

pub fn f498(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2272 = {
            let a2248 = {
                let a2236 = {
                    let a2230 = {
                        let a2228 = v3;
                        let a2229 = 128u8;
                        fixed_u8::bit_and(a2228, a2229)
                    };
                    let a2231 = 128u8;
                    if equal(a2230, a2231) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2237 = {
                    let a2234 = {
                        let a2232 = v3;
                        let a2233 = 64u8;
                        fixed_u8::bit_and(a2232, a2233)
                    };
                    let a2235 = 64u8;
                    if equal(a2234, a2235) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2236, a2237)?
            };
            let a2249 = {
                let a2246 = {
                    let a2240 = {
                        let a2238 = v3;
                        let a2239 = 32u8;
                        fixed_u8::bit_and(a2238, a2239)
                    };
                    let a2241 = 32u8;
                    if equal(a2240, a2241) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2247 = {
                    let a2244 = {
                        let a2242 = v3;
                        let a2243 = 16u8;
                        fixed_u8::bit_and(a2242, a2243)
                    };
                    let a2245 = 16u8;
                    if equal(a2244, a2245) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2246, a2247)?
            };
            nat_add(a2248, a2249)?
        };
        let a2273 = {
            let a2270 = {
                let a2258 = {
                    let a2252 = {
                        let a2250 = v3;
                        let a2251 = 8u8;
                        fixed_u8::bit_and(a2250, a2251)
                    };
                    let a2253 = 8u8;
                    if equal(a2252, a2253) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2259 = {
                    let a2256 = {
                        let a2254 = v3;
                        let a2255 = 4u8;
                        fixed_u8::bit_and(a2254, a2255)
                    };
                    let a2257 = 4u8;
                    if equal(a2256, a2257) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2258, a2259)?
            };
            let a2271 = {
                let a2268 = {
                    let a2262 = {
                        let a2260 = v3;
                        let a2261 = 2u8;
                        fixed_u8::bit_and(a2260, a2261)
                    };
                    let a2263 = 2u8;
                    if equal(a2262, a2263) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2269 = {
                    let a2266 = {
                        let a2264 = v3;
                        let a2265 = 1u8;
                        fixed_u8::bit_and(a2264, a2265)
                    };
                    let a2267 = 1u8;
                    if equal(a2266, a2267) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2268, a2269)?
            };
            nat_add(a2270, a2271)?
        };
        nat_add(a2272, a2273)?
    };
    let a2274 = v4;
    let a2275 = 34u64;
    if nat_eq(a2274, a2275) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2276 = v4;
        let a2277 = 92u64;
        if nat_eq(a2276, a2277) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2278 = v4;
            let a2279 = 8u64;
            if nat_eq(a2278, a2279) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2280 = v4;
                let a2281 = 12u64;
                if nat_eq(a2280, a2281) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2282 = v4;
                    let a2283 = 10u64;
                    if nat_eq(a2282, a2283) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2284 = v4;
                        let a2285 = 13u64;
                        if nat_eq(a2284, a2285) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2286 = v4;
                            let a2287 = 9u64;
                            if nat_eq(a2286, a2287) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2288 = v4;
                                let a2289 = 32u64;
                                if nat_lt(a2288, a2289) {
                                    let a2290 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2291 = f547(v2)?;
                                    Ok(append_bytes(a2290, a2291))
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

pub fn f499(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f548(Fn19::F549, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f500(v0: Fn16, p2295: List<List<(Bytes, Bytes)>>, p2296: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2294 = (p2295, p2296);
    {
        let mut m2294 = m2294;
        loop {
            let step = {
                let (v1, v2) = m2294;
                let m2292 = v2;
                match m2292.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2293: Fn16 = v0.clone();
                            c2293.apply(v1, v3)
                        }, v4))
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

pub fn f501(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f502(v0: Fn17, p2301: u64, p2302: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2300 = (p2301, p2302);
    {
        let mut m2300 = m2300;
        loop {
            let step = {
                let (v1, v2) = m2300;
                let m2297 = v1;
                if m2297 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2297 - 1;
                    let m2299 = {
                        let c2298: Fn17 = v0.clone();
                        c2298.apply(v2.clone())?
                    };
                    match m2299 {
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
                Ok(next) => { m2300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f550(v0)
}

pub fn f504(v0: u8) -> bool {
    let a2303 = v0;
    let a2304 = 32u8;
    if if if equal(a2303, a2304) {
        true
    } else {
        let a2305 = v0;
        let a2306 = 9u8;
        equal(a2305, a2306)
    } {
        true
    } else {
        let a2307 = v0;
        let a2308 = 10u8;
        equal(a2307, a2308)
    } {
        true
    } else {
        let a2309 = v0;
        let a2310 = 12u8;
        if equal(a2309, a2310) {
            true
        } else {
            let a2311 = v0;
            let a2312 = 13u8;
            equal(a2311, a2312)
        }
    }
}

pub fn f505(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f382({
        let (_, h2313) = {
            let (h2314, _) = f551(Fn20::F552, {
                let a2316 = {
                    let a2315 = v0.clone();
                    length_list(a2315)
                };
                let a2317 = 1u64;
                nat_add(a2316, a2317)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2314
        };
        h2313
    }))
}

pub fn f506(v0: Bytes, v1: Bytes, p2330: u64, p2331: u64) -> R<bool> {
    let m2329 = (p2330, p2331);
    {
        let mut m2329 = m2329;
        loop {
            let step = {
                let (v2, v3) = m2329;
                let m2318 = v3;
                if m2318 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m2318 - 1;
                    let m2321 = {
                        let a2319 = v1.clone();
                        let a2320 = v2;
                        index_bytes(a2319, a2320)
                    };
                    match m2321 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m2324 = {
                                let a2322 = v0.clone();
                                let a2323 = v2;
                                index_bytes(a2322, a2323)
                            };
                            match m2324 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a2325 = v5;
                                    let a2326 = v6;
                                    if equal(a2325, a2326) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a2327 = v2;
                                            let a2328 = 1u64;
                                            nat_add(a2327, a2328)?
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
                Ok(next) => { m2329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f508(v0: List<Adt13>) -> List<Bytes> {
    let m2332 = v0;
    match m2332.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2333 = f79(v1, f8(Str::lit("std")));
            match m2333 {
                None => {
                    f508(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f508(v2))
                }
            }
        }
    }
}

pub fn f509(v0: List<Adt13>) -> List<Bytes> {
    let m2334 = v0;
    match m2334.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2335 = f78(f79(v1, f8(Str::lit("req"))));
            match m2335 {
                None => {
                    f509(v2)
                }
                Some(v3) => {
                    List::cons(v3, f509(v2))
                }
            }
        }
    }
}

pub fn f510(p2340: List<(Bytes, Adt17)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt17)>> {
    let m2339 = p2340;
    {
        let mut m2339 = m2339;
        loop {
            let step = {
                let v0 = m2339;
                let m2336 = v0;
                match m2336.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(Ok(None::<(Bytes, Adt17)>))
                    }
                    Some((v3, v4)) => {
                        if f553(f554({
                            let (_, h2337) = v3.clone();
                            h2337
                        }, List::<Bytes>::nil())?, f512({
                            let (h2338, _) = v3.clone();
                            h2338
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt17)>, R<Option<(Bytes, Adt17)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: List<(Bytes, Adt17)>, v1: Bytes) -> List<(Bytes, Adt17)> {
    let m2341 = v0;
    match m2341.uncons() {
        None => {
            List::<(Bytes, Adt17)>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h2342, _) = v2.clone();
                h2342
            }, v1.clone()) {
                f511(v3, v1)
            } else {
                List::cons(v2, f511(v3, v1))
            }
        }
    }
}

pub fn f512(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f321(v0)?;
    if f41(v1.clone(), f555()) {
        f556(v1)
    } else {
        Ok(v1)
    }
}

pub fn f513() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f514() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f515(v0: Bytes, v1: Adt17) -> R<Result<Adt28, Bytes>> {
    f557(v0, f558(v1))
}

pub fn f516(v0: Adt28) -> List<Adt25> {
    let m2343 = v0;
    match m2343 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f517(v0: Adt28) -> Bytes {
    let m2344 = v0;
    match m2344 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f518(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2345 = v0;
    match m2345.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f518(v3, false))
            } else {
                f518(v3, true)
            }
        }
    }
}

pub fn f519(v0: Fn18, p2349: u64, p2350: Adt27) -> R<Adt27> {
    let m2348 = (p2349, p2350);
    {
        let mut m2348 = m2348;
        loop {
            let step = {
                let (v1, v2) = m2348;
                let m2346 = v1;
                if m2346 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2346 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2347: Fn18 = v0.clone();
                        c2347.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f520(v0: Adt27) -> R<Adt27> {
    f559(v0)
}

pub fn f521(v0: u32, v1: u32) -> u32 {
    let m2353 = {
        let a2351 = f560(v0);
        let a2352 = f560(v1);
        fixed_u64::checked_add(a2351, a2352)
    };
    match m2353 {
        None => {
            0u32
        }
        Some(v2) => {
            f561(v2)
        }
    }
}

pub fn f522(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2356 = {
        let a2354 = v3;
        let a2355 = 8u64;
        nat_add(a2354, a2355)?
    };
    let a2357 = v2;
    if nat_eq(a2356, a2357) {
        Ok(f562({
            let a2360 = {
                let a2358 = v1;
                let a2359 = 8u64;
                nat_mul(a2358, a2359)?
            };
            let a2361 = 4294967296u64;
            let a2362 = 0u64;
            nat_quot(a2360, a2361, a2362)
        }))
    } else {
        let a2365 = {
            let a2363 = v3;
            let a2364 = 4u64;
            nat_add(a2363, a2364)?
        };
        let a2366 = v2;
        if nat_eq(a2365, a2366) {
            Ok(f562({
                let a2369 = {
                    let a2367 = v1;
                    let a2368 = 8u64;
                    nat_mul(a2367, a2368)?
                };
                let a2370 = 4294967296u64;
                let a2371 = 0u64;
                nat_rem(a2369, a2370, a2371)
            }))
        } else {
            Ok(f563(f564(v0.clone(), v1, v3), f564(v0.clone(), v1, {
                let a2372 = v3;
                let a2373 = 1u64;
                nat_add(a2372, a2373)?
            }), f564(v0.clone(), v1, {
                let a2374 = v3;
                let a2375 = 2u64;
                nat_add(a2374, a2375)?
            }), f564(v0, v1, {
                let a2376 = v3;
                let a2377 = 3u64;
                nat_add(a2376, a2377)?
            })))
        }
    }
}

pub fn f523() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f524(v0: Adt17) -> Bytes {
    let m2378 = v0;
    match m2378 {
        Adt17::C0 => {
            f8(Str::lit("null"))
        }
        Adt17::C1(_, _) => {
            f8(Str::lit("tagged"))
        }
        Adt17::C2(_) => {
            f8(Str::lit("boolean"))
        }
        Adt17::C3(_, _) => {
            f8(Str::lit("integer"))
        }
        Adt17::C4(_, _, _) => {
            f8(Str::lit("number"))
        }
        Adt17::C5(_) => {
            f8(Str::lit("string"))
        }
        Adt17::C6(_) => {
            f8(Str::lit("array"))
        }
        Adt17::C7(_) => {
            f8(Str::lit("object"))
        }
        Adt17::C8 => {
            f8(Str::lit("spine"))
        }
        Adt17::C9(_, _) => {
            f8(Str::lit("spine"))
        }
        Adt17::C10(_, _, _) => {
            f8(Str::lit("spine"))
        }
    }
}

pub fn f525(p2381: List<Bytes>, v1: Adt17) -> bool {
    let m2380 = p2381;
    {
        let mut m2380 = m2380;
        loop {
            let step = {
                let v0 = m2380;
                let m2379 = v0;
                match m2379.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f565(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
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

pub fn f526(p2384: List<Adt17>, v1: Adt17) -> R<bool> {
    let m2383 = p2384;
    {
        let mut m2383 = m2383;
        loop {
            let step = {
                let v0 = m2383;
                let m2382 = v0;
                match m2382.uncons() {
                    None => {
                        Err::<List<Adt17>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f527(false, v2, v1.clone())? {
                            Err::<List<Adt17>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt17>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: bool, v1: Adt17, v2: Adt17) -> R<bool> {
    let m2385 = v1;
    match m2385 {
        Adt17::C0 => {
            Ok(f28(f418(v2)?, f8(Str::lit("null"))))
        }
        Adt17::C1(_, _) => {
            Ok(false)
        }
        Adt17::C2(v77) => {
            Ok(f28(f418(v2)?, f418(Adt17::C2(v77))?))
        }
        Adt17::C3(v78, v79) => {
            Ok(f28(f418(v2)?, f418(Adt17::C3(v78, v79))?))
        }
        Adt17::C4(v80, v81, v82) => {
            Ok(f28(f418(v2)?, f418(Adt17::C4(v80, v81, v82))?))
        }
        Adt17::C5(v83) => {
            let m2397 = v2;
            match m2397 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(v84) => {
                    Ok(f28(v83, v84))
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C6(r2398) => {
            let v58 = (*r2398).clone();
            let m2395 = v2;
            match m2395 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(r2396) => {
                    let v59 = (*r2396).clone();
                    f527(false, v58, v59)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C7(r2399) => {
            let v41 = (*r2399).clone();
            let m2391 = v2;
            match m2391 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(r2394) => {
                    let v42 = (*r2394).clone();
                    if f344({
                        let a2392 = f532(v41.clone())?;
                        let a2393 = f532(v42.clone())?;
                        nat_eq(a2392, a2393)
                    }, true) {
                        f527(true, v41, v42)
                    } else {
                        Ok(false)
                    }
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(_, _) => {
                    Ok(false)
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C8 => {
            if v0 {
                Ok(true)
            } else {
                let m2387 = v2;
                match m2387 {
                    Adt17::C0 => {
                        Ok(false)
                    }
                    Adt17::C1(_, _) => {
                        Ok(false)
                    }
                    Adt17::C2(_) => {
                        Ok(false)
                    }
                    Adt17::C3(_, _) => {
                        Ok(false)
                    }
                    Adt17::C4(_, _, _) => {
                        Ok(false)
                    }
                    Adt17::C5(_) => {
                        Ok(false)
                    }
                    Adt17::C6(_) => {
                        Ok(false)
                    }
                    Adt17::C7(_) => {
                        Ok(false)
                    }
                    Adt17::C8 => {
                        Ok(true)
                    }
                    Adt17::C9(_, _) => {
                        Ok(false)
                    }
                    Adt17::C10(_, _, _) => {
                        Ok(false)
                    }
                }
            }
        }
        Adt17::C9(r2400, r2401) => {
            let v23 = (*r2400).clone();
            let v24 = (*r2401).clone();
            let m2388 = v2;
            match m2388 {
                Adt17::C0 => {
                    Ok(false)
                }
                Adt17::C1(_, _) => {
                    Ok(false)
                }
                Adt17::C2(_) => {
                    Ok(false)
                }
                Adt17::C3(_, _) => {
                    Ok(false)
                }
                Adt17::C4(_, _, _) => {
                    Ok(false)
                }
                Adt17::C5(_) => {
                    Ok(false)
                }
                Adt17::C6(_) => {
                    Ok(false)
                }
                Adt17::C7(_) => {
                    Ok(false)
                }
                Adt17::C8 => {
                    Ok(false)
                }
                Adt17::C9(r2389, r2390) => {
                    let v25 = (*r2389).clone();
                    let v26 = (*r2390).clone();
                    if f527(false, v23, v25)? {
                        f527(false, v24, v26)
                    } else {
                        Ok(false)
                    }
                }
                Adt17::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt17::C10(v3, r2402, r2403) => {
            let v4 = (*r2402).clone();
            let v5 = (*r2403).clone();
            let m2386 = f233(v2.clone(), v3);
            if match m2386 {
                None => {
                    false
                }
                Some(v6) => {
                    f527(false, v4, v6)?
                }
            } {
                f527(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f528(v0: Bytes) -> R<Option<List<u64>>> {
    f566({
        let a2405 = {
            let a2404 = v0.clone();
            length_bytes(a2404)
        };
        let a2406 = 1u64;
        nat_add(a2405, a2406)?
    }, v0, 0u64)
}

pub fn f529(v0: List<u64>) -> R<u64> {
    let m2407 = v0;
    match m2407.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2408 = f529(v2)?;
            let a2409 = 1u64;
            nat_add(a2408, a2409)
        }
    }
}

pub fn f530(v0: Option<Adt17>) -> Option<u64> {
    let m2410 = v0;
    match m2410 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2411 = v1;
            match m2411 {
                Adt17::C0 => {
                    None::<u64>
                }
                Adt17::C1(_, _) => {
                    None::<u64>
                }
                Adt17::C2(_) => {
                    None::<u64>
                }
                Adt17::C3(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt17::C4(_, _, _) => {
                    None::<u64>
                }
                Adt17::C5(_) => {
                    None::<u64>
                }
                Adt17::C6(_) => {
                    None::<u64>
                }
                Adt17::C7(_) => {
                    None::<u64>
                }
                Adt17::C8 => {
                    None::<u64>
                }
                Adt17::C9(_, _) => {
                    None::<u64>
                }
                Adt17::C10(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f531(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m2412 = f567(v0)?;
    match m2412 {
        Ok(v3) => {
            let m2413 = f528(v1)?;
            match m2413 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h2414, _) = {
                            let (h2415, _) = v3.clone();
                            h2415
                        };
                        if h2414 {
                            f568({
                                let a2419 = {
                                    let a2417 = f569({
                                        let (_, h2416) = v3.clone();
                                        h2416
                                    })?;
                                    let a2418 = f529(v4.clone())?;
                                    nat_add(a2417, a2418)?
                                };
                                let a2420 = 2u64;
                                nat_add(a2419, a2420)?
                            }, {
                                let (_, h2421) = v3;
                                h2421
                            }, v4, {
                                let (_, h2422) = {
                                    let (h2423, _) = v3;
                                    h2423
                                };
                                h2422
                            })?
                        } else {
                            f570({
                                let (_, h2424) = v3;
                                h2424
                            }, v4, {
                                let (_, h2425) = {
                                    let (h2426, _) = v3;
                                    h2426
                                };
                                h2425
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

pub fn f532(v0: Adt17) -> R<u64> {
    let m2427 = v0;
    match m2427 {
        Adt17::C0 => {
            Ok(0u64)
        }
        Adt17::C1(_, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(_, _) => {
            Ok(0u64)
        }
        Adt17::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C5(_) => {
            Ok(0u64)
        }
        Adt17::C6(_) => {
            Ok(0u64)
        }
        Adt17::C7(_) => {
            Ok(0u64)
        }
        Adt17::C8 => {
            Ok(0u64)
        }
        Adt17::C9(_, r2432) => {
            let v5 = (*r2432).clone();
            let a2430 = f532(v5)?;
            let a2431 = 1u64;
            nat_add(a2430, a2431)
        }
        Adt17::C10(_, _, r2433) => {
            let v3 = (*r2433).clone();
            let a2428 = f532(v3)?;
            let a2429 = 1u64;
            nat_add(a2428, a2429)
        }
    }
}

pub fn f533(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2434 = v0;
    match m2434.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), v1.clone()) {
                f533(v3, v1)
            } else {
                List::cons(v2, f533(v3, v1))
            }
        }
    }
}

pub fn f534(v0: List<(Bytes, Adt17)>) -> List<Bytes> {
    let m2435 = v0;
    match m2435.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2436, _) = v1;
                h2436
            }, f534(v2))
        }
    }
}

pub fn f535(v0: Adt17) -> Option<(bool, (u64, u64))> {
    let m2437 = v0;
    match m2437 {
        Adt17::C0 => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C1(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C2(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C3(v1, v2) => {
            Some((v1, (v2, 0u64)))
        }
        Adt17::C4(v3, v4, v5) => {
            Some((v3, (v4, v5)))
        }
        Adt17::C5(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C6(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C7(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C8 => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C9(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt17::C10(_, _, _) => {
            None::<(bool, (u64, u64))>
        }
    }
}

pub fn f536(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a2442 = {
            let (h2438, _) = {
                let (_, h2439) = v0;
                h2439
            };
            h2438
        };
        let a2443 = f571({
            let (_, h2440) = {
                let (_, h2441) = v1;
                h2441
            };
            h2440
        })?;
        nat_mul(a2442, a2443)?
    };
    let v3: u64 = {
        let a2448 = {
            let (h2444, _) = {
                let (_, h2445) = v1;
                h2445
            };
            h2444
        };
        let a2449 = f571({
            let (_, h2446) = {
                let (_, h2447) = v0;
                h2447
            };
            h2446
        })?;
        nat_mul(a2448, a2449)?
    };
    let (h2450, _) = v0;
    if h2450 {
        let (h2451, _) = v1;
        if h2451 {
            let a2452 = v3;
            let a2453 = v2;
            Ok(nat_lt(a2452, a2453))
        } else {
            Ok(true)
        }
    } else {
        let (h2454, _) = v1;
        if h2454 {
            Ok(false)
        } else {
            let a2455 = v2;
            let a2456 = v3;
            Ok(nat_lt(a2455, a2456))
        }
    }
}

pub fn f537(p2486: u64, v1: Bytes, p2487: u64, p2488: Bytes) -> R<Bytes> {
    let m2485 = (p2486, (p2487, p2488));
    {
        let mut m2485 = m2485;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2485;
                let m2457 = v0;
                if m2457 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2457 - 1;
                    let a2459 = {
                        let a2458 = v1.clone();
                        length_bytes(a2458)
                    };
                    let a2460 = v2;
                    if nat_le(a2459, a2460) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f344({
                            let a2461 = f342(v1.clone(), v2)?;
                            let a2462 = 126u64;
                            nat_eq(a2461, a2462)
                        }, {
                            let a2465 = f342(v1.clone(), {
                                let a2463 = v2;
                                let a2464 = 1u64;
                                nat_add(a2463, a2464)?
                            })?;
                            let a2466 = 48u64;
                            nat_eq(a2465, a2466)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a2467 = v2;
                                let a2468 = 2u64;
                                nat_add(a2467, a2468)?
                            }, {
                                let a2469 = v3;
                                let a2470 = f8(Str::lit("~"));
                                append_bytes(a2469, a2470)
                            })))
                        } else {
                            if f344({
                                let a2471 = f342(v1.clone(), v2)?;
                                let a2472 = 126u64;
                                nat_eq(a2471, a2472)
                            }, {
                                let a2475 = f342(v1.clone(), {
                                    let a2473 = v2;
                                    let a2474 = 1u64;
                                    nat_add(a2473, a2474)?
                                })?;
                                let a2476 = 49u64;
                                nat_eq(a2475, a2476)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2477 = v2;
                                    let a2478 = 2u64;
                                    nat_add(a2477, a2478)?
                                }, {
                                    let a2479 = v3;
                                    let a2480 = f8(Str::lit("/"));
                                    append_bytes(a2479, a2480)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2481 = v2;
                                    let a2482 = 1u64;
                                    nat_add(a2481, a2482)?
                                }, {
                                    let a2483 = v3;
                                    let a2484 = f35(v1.clone(), v2, 1u64);
                                    append_bytes(a2483, a2484)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f572({
        let a2490 = {
            let a2489 = v1.clone();
            length_bytes(a2489)
        };
        let a2491 = 1u64;
        nat_add(a2490, a2491)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f539(v0: List<(Bytes, Adt17)>, v1: Bytes) -> R<Result<List<Adt17>, Bytes>> {
    let m2492 = v0;
    match m2492.uncons() {
        None => {
            Ok(Ok::<List<Adt17>, Bytes>(List::<Adt17>::nil()))
        }
        Some((v2, v3)) => {
            let m2494 = f531({
                let (h2493, _) = v2;
                h2493
            }, v1.clone())?;
            match m2494 {
                Ok(v5) => {
                    let m2495 = f539(v3, v1)?;
                    match m2495 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt17>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h2496) = v2;
                                    h2496
                                }, v7)
                            } else {
                                v7
                            }))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt17>, Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt17>, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f540(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2497, _) = v1.clone();
        h2497
    };
    let v3: Bytes = {
        let (h2498, _) = {
            let (_, h2499) = v1.clone();
            h2499
        };
        h2498
    };
    let a2502 = {
        let (_, h2500) = {
            let (_, h2501) = v1;
            h2501
        };
        h2500
    };
    let a2503 = 0u64;
    if nat_eq(a2502, a2503) {
        let v4: u64 = f342(v0.clone(), v2)?;
        let a2504 = v4;
        let a2505 = 34u64;
        if nat_eq(a2504, a2505) {
            Ok(Some(({
                let a2506 = v2;
                let a2507 = 1u64;
                nat_add(a2506, a2507)?
            }, (v3, 1u64))))
        } else {
            let a2508 = v4;
            let a2509 = 92u64;
            if nat_eq(a2508, a2509) {
                f573(v0, v2, v3)
            } else {
                let a2510 = v4;
                let a2511 = 32u64;
                if if nat_lt(a2510, a2511) {
                    true
                } else {
                    let a2512 = v4;
                    let a2513 = 256u64;
                    nat_eq(a2512, a2513)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2514 = v2;
                        let a2515 = 1u64;
                        nat_add(a2514, a2515)?
                    }, ({
                        let a2516 = v3;
                        let a2517 = f35(v0, v2, 1u64);
                        append_bytes(a2516, a2517)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f541(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f342(v0, v1)?;
    let a2518 = 47u64;
    let a2519 = v2;
    if if nat_lt(a2518, a2519) {
        let a2520 = v2;
        let a2521 = 58u64;
        nat_lt(a2520, a2521)
    } else {
        false
    } {
        Ok(Some({
            let a2522 = v1;
            let a2523 = 1u64;
            nat_add(a2522, a2523)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f542(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2524 = f342(v0, v1)?;
    let a2525 = 48u64;
    if nat_eq(a2524, a2525) {
        Ok(Some({
            let a2526 = v1;
            let a2527 = 1u64;
            nat_add(a2526, a2527)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f543(v0: Bytes) -> R<u64> {
    let (h2528, _) = f360(Fn6::F574(v0.clone()), {
        let a2529 = v0;
        length_bytes(a2529)
    }, 0u64)?;
    Ok(h2528)
}

pub fn f544(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2533 = {
        let a2531 = v0;
        let a2532 = {
            let (h2530, _) = v1;
            h2530
        };
        index_bytes(a2531, a2532)
    };
    match m2533 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2535 = {
                    let (h2534, _) = v1;
                    h2534
                };
                let a2536 = 1u64;
                nat_add(a2535, a2536)?
            }, {
                let a2540 = {
                    let a2538 = {
                        let (_, h2537) = v1;
                        h2537
                    };
                    let a2539 = 10u64;
                    nat_mul(a2538, a2539)?
                };
                let a2541 = f575(v2)?;
                nat_add(a2540, a2541)?
            })))
        }
    }
}

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f576(v0, v1)
}

pub fn f546(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2542, _) = v2;
        h2542
    };
    let a2543 = v3;
    let a2544 = 0u64;
    if nat_eq(a2543, a2544) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2545 = v3;
            let a2546 = v1;
            let a2547 = 0u64;
            nat_quot(a2545, a2546, a2547)
        }, {
            let a2552 = f496(v0, {
                let a2548 = v3;
                let a2549 = v1;
                let a2550 = 0u64;
                nat_rem(a2548, a2549, a2550)
            });
            let a2553 = {
                let (_, h2551) = v2;
                h2551
            };
            append_bytes(a2552, a2553)
        }))
    }
}

pub fn f547(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2598 = {
            let a2574 = {
                let a2562 = {
                    let a2556 = {
                        let a2554 = v1;
                        let a2555 = 128u8;
                        fixed_u8::bit_and(a2554, a2555)
                    };
                    let a2557 = 128u8;
                    if equal(a2556, a2557) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2563 = {
                    let a2560 = {
                        let a2558 = v1;
                        let a2559 = 64u8;
                        fixed_u8::bit_and(a2558, a2559)
                    };
                    let a2561 = 64u8;
                    if equal(a2560, a2561) {
                        64u64
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
                        let a2565 = 32u8;
                        fixed_u8::bit_and(a2564, a2565)
                    };
                    let a2567 = 32u8;
                    if equal(a2566, a2567) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2573 = {
                    let a2570 = {
                        let a2568 = v1;
                        let a2569 = 16u8;
                        fixed_u8::bit_and(a2568, a2569)
                    };
                    let a2571 = 16u8;
                    if equal(a2570, a2571) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2572, a2573)?
            };
            nat_add(a2574, a2575)?
        };
        let a2599 = {
            let a2596 = {
                let a2584 = {
                    let a2578 = {
                        let a2576 = v1;
                        let a2577 = 8u8;
                        fixed_u8::bit_and(a2576, a2577)
                    };
                    let a2579 = 8u8;
                    if equal(a2578, a2579) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2585 = {
                    let a2582 = {
                        let a2580 = v1;
                        let a2581 = 4u8;
                        fixed_u8::bit_and(a2580, a2581)
                    };
                    let a2583 = 4u8;
                    if equal(a2582, a2583) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2584, a2585)?
            };
            let a2597 = {
                let a2594 = {
                    let a2588 = {
                        let a2586 = v1;
                        let a2587 = 2u8;
                        fixed_u8::bit_and(a2586, a2587)
                    };
                    let a2589 = 2u8;
                    if equal(a2588, a2589) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2595 = {
                    let a2592 = {
                        let a2590 = v1;
                        let a2591 = 1u8;
                        fixed_u8::bit_and(a2590, a2591)
                    };
                    let a2593 = 1u8;
                    if equal(a2592, a2593) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2594, a2595)?
            };
            nat_add(a2596, a2597)?
        };
        nat_add(a2598, a2599)?
    };
    let a2606 = f496(f577(), {
        let a2600 = v2;
        let a2601 = 16u64;
        let a2602 = 0u64;
        nat_quot(a2600, a2601, a2602)
    });
    let a2607 = f496(f577(), {
        let a2603 = v2;
        let a2604 = 16u64;
        let a2605 = 0u64;
        nat_rem(a2603, a2604, a2605)
    });
    Ok(append_bytes(a2606, a2607))
}

pub fn f548(v0: Fn19, p2611: List<List<(Bytes, Bytes)>>, p2612: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2610 = (p2611, p2612);
    {
        let mut m2610 = m2610;
        loop {
            let step = {
                let (v1, v2) = m2610;
                let m2608 = v2;
                match m2608.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2609: Fn19 = v0.clone();
                            c2609.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f550(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2614 = {
        let a2613 = v0.clone();
        length_list(a2613)
    };
    let a2615 = 1u64;
    if nat_le(a2614, a2615) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f578(v0)?))
    }
}

pub fn f551(v0: Fn20, p2620: u64, p2621: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2619 = (p2620, p2621);
    {
        let mut m2619 = m2619;
        loop {
            let step = {
                let (v1, v2) = m2619;
                let m2616 = v1;
                if m2616 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2616 - 1;
                    let m2618 = {
                        let c2617: Fn20 = v0.clone();
                        c2617.apply(v2.clone())?
                    };
                    match m2618 {
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
                Ok(next) => { m2619 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f579(v0)
}

pub fn f553(p2624: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2623 = p2624;
    {
        let mut m2623 = m2623;
        loop {
            let step = {
                let v0 = m2623;
                let m2622 = v0;
                match m2622.uncons() {
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
                Ok(next) => { m2623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: Adt17, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2625 = v0;
    match m2625 {
        Adt17::C0 => {
            Ok(v1)
        }
        Adt17::C1(_, _) => {
            Ok(v1)
        }
        Adt17::C2(_) => {
            Ok(v1)
        }
        Adt17::C3(_, _) => {
            Ok(v1)
        }
        Adt17::C4(_, _, _) => {
            Ok(v1)
        }
        Adt17::C5(_) => {
            Ok(v1)
        }
        Adt17::C6(r2628) => {
            let v2 = (*r2628).clone();
            f554(v2, v1)
        }
        Adt17::C7(r2629) => {
            let v3 = (*r2629).clone();
            f554(v3, v1)
        }
        Adt17::C8 => {
            Ok(v1)
        }
        Adt17::C9(r2630, r2631) => {
            let v4 = (*r2630).clone();
            let v5 = (*r2631).clone();
            f554(v4, f554(v5, v1)?)
        }
        Adt17::C10(v6, r2632, r2633) => {
            let v7 = (*r2632).clone();
            let v8 = (*r2633).clone();
            let v9: List<Bytes> = f554(v7.clone(), f554(v8, v1)?)?;
            if f28(v6, f8(Str::lit("$ref"))) {
                let m2626 = f327(Some(v7));
                match m2626 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2627 = f580(v10)?;
                        match m2627 {
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

pub fn f555() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f556(v0: Bytes) -> R<Bytes> {
    let m2636 = {
        let a2634 = v0.clone();
        let a2635 = 0u64;
        index_bytes(a2634, a2635)
    };
    match m2636 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2642 = {
                let a2637 = f8(Str::lit("u"));
                let a2638 = f581(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2637, a2638)
            };
            let a2643 = f35(v0.clone(), 1u64, {
                let a2640 = {
                    let a2639 = v0;
                    length_bytes(a2639)
                };
                let a2641 = 1u64;
                nat_sub(a2640, a2641)
            });
            Ok(append_bytes(a2642, a2643))
        }
    }
}

pub fn f557(v0: Bytes, v1: Adt17) -> R<Result<Adt28, Bytes>> {
    let m2644 = f233(v1.clone(), f8(Str::lit("enum")));
    match m2644 {
        None => {
            let m2646 = f233(v1.clone(), f8(Str::lit("oneOf")));
            match m2646 {
                None => {
                    if f586(v1.clone(), v0.clone())? {
                        let m2647 = f587(v0.clone(), f460(f233(v1.clone(), f8(Str::lit("properties")))), f237(f238(f233(v1, f8(Str::lit("required"))))), true)?;
                        match m2647 {
                            Ok(v7) => {
                                Ok(f584(f588(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f338(f589(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f590(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2648 = f460(f233(v1.clone(), f8(Str::lit("properties"))));
                        match m2648.uncons() {
                            None => {
                                if f591(v1.clone()) {
                                    Ok(f584(f592(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2651 = f593(v1, {
                                        let a2649 = v0.clone();
                                        let a2650 = f8(Str::lit("Value"));
                                        append_bytes(a2649, a2650)
                                    })?;
                                    match m2651 {
                                        Ok(v9) => {
                                            Ok(f584(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f517(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f516(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2652 = f587(v0.clone(), f460(f233(v1.clone(), f8(Str::lit("properties")))), f237(f238(f233(v1, f8(Str::lit("required"))))), false)?;
                                match m2652 {
                                    Ok(v13) => {
                                        Ok(f584(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2653 = f589(v13.clone());
                                            match m2653.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f338(f589(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f590(v13)))
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
                    f585(v0, f328(v5))
                }
            }
        }
        Some(v2) => {
            let m2645 = f582(f328(v2), List::<Bytes>::nil())?;
            match m2645 {
                Ok(v4) => {
                    Ok(f584(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f338(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f583(v3, v0))
                }
            }
        }
    }
}

pub fn f558(v0: Adt17) -> Adt17 {
    let v2: Adt17 = {
        let m2654 = f233(v0.clone(), f8(Str::lit("anyOf")));
        match m2654 {
            None => {
                v0
            }
            Some(v1) => {
                if f594(f328(v1.clone())) {
                    f595(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f596(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f597(f328(v1.clone()))
                    } {
                        v0
                    } else {
                        f598(f595(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt17 = {
        let m2655 = f233(v2.clone(), f8(Str::lit("oneOf")));
        match m2655 {
            None => {
                v2
            }
            Some(v3) => {
                if f594(f328(v3)) {
                    f595(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2656 = f233(v4.clone(), f8(Str::lit("allOf")));
    match m2656 {
        None => {
            v4
        }
        Some(v5) => {
            if f599(f328(v5)) {
                f595(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f559(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f600(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2657) => {
            h2657
        }
    });
    let v2: u32 = f521(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2658, _, _) => {
            h2658
        }
    }, f521(f601(match v0.clone() {
        Adt27::C0(_, _, _, _, h2659, _, _, _, _, _) => {
            h2659
        }
    }), f521(f602(match v0.clone() {
        Adt27::C0(_, _, _, _, h2660, _, _, _, _, _) => {
            h2660
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2661, _, _, _, _) => {
            h2661
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2662, _, _, _) => {
            h2662
        }
    }), f521(v1, {
        let m2665 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2664, _) => {
                h2664
            }
        };
        match m2665 {
            Adt26::C0(h2663, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2663
            }
        }
    }))));
    let v3: u32 = f521(f603(match v0.clone() {
        Adt27::C0(h2666, _, _, _, _, _, _, _, _, _) => {
            h2666
        }
    }), f604(match v0.clone() {
        Adt27::C0(h2667, _, _, _, _, _, _, _, _, _) => {
            h2667
        }
    }, match v0.clone() {
        Adt27::C0(_, h2668, _, _, _, _, _, _, _, _) => {
            h2668
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2669, _, _, _, _, _, _, _) => {
            h2669
        }
    }));
    Ok(Adt27::C0(f521(v2, v3), match v0.clone() {
        Adt27::C0(h2670, _, _, _, _, _, _, _, _, _) => {
            h2670
        }
    }, match v0.clone() {
        Adt27::C0(_, h2671, _, _, _, _, _, _, _, _) => {
            h2671
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2672, _, _, _, _, _, _, _) => {
            h2672
        }
    }, f521(match v0.clone() {
        Adt27::C0(_, _, _, h2673, _, _, _, _, _, _) => {
            h2673
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2674, _, _, _, _, _) => {
            h2674
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2675, _, _, _, _) => {
            h2675
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2676, _, _, _) => {
            h2676
        }
    }, f605(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2677, _) => {
            h2677
        }
    }), {
        let a2679 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2678) => {
                h2678
            }
        };
        let a2680 = 1u64;
        nat_add(a2679, a2680)?
    }))
}

pub fn f560(v0: u32) -> u64 {
    let m2682 = {
        let a2681 = v0;
        fixed_u64::convert(i128::from(a2681))
    };
    match m2682 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f561(v0: u64) -> u32 {
    let m2686 = {
        let a2685 = {
            let a2683 = v0;
            let a2684 = 4294967295u64;
            fixed_u64::bit_and(a2683, a2684)
        };
        fixed_u32::convert(i128::from(a2685))
    };
    match m2686 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f562(v0: u64) -> u32 {
    let (_, h2687) = {
        let (_, h2688) = f606(Fn21::F607, 32u64, (v0, (1u32, 0u32)));
        h2688
    };
    h2687
}

pub fn f563(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2693 = f608(v0, 24u32);
    let a2694 = {
        let a2691 = f608(v1, 16u32);
        let a2692 = {
            let a2689 = f608(v2, 8u32);
            let a2690 = v3;
            fixed_u32::bit_or(a2689, a2690)
        };
        fixed_u32::bit_or(a2691, a2692)
    };
    fixed_u32::bit_or(a2693, a2694)
}

pub fn f564(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2695 = v2;
    let a2696 = v1;
    if nat_lt(a2695, a2696) {
        let m2699 = {
            let a2697 = v0;
            let a2698 = v2;
            index_bytes(a2697, a2698)
        };
        match m2699 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2701 = {
                    let a2700 = v3;
                    fixed_u32::convert(i128::from(a2700))
                };
                match m2701 {
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
        let a2702 = v2;
        let a2703 = v1;
        if nat_eq(a2702, a2703) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f565(v0: Bytes, v1: Adt17) -> bool {
    let v2: Bytes = f524(v1);
    if f28(v0.clone(), f8(Str::lit("number"))) {
        if f28(v2.clone(), f8(Str::lit("integer"))) {
            true
        } else {
            f28(v2, f8(Str::lit("number")))
        }
    } else {
        f28(v0, v2)
    }
}

pub fn f566(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m2704 = v0;
    if m2704 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m2704 - 1;
        let a2706 = {
            let a2705 = v1.clone();
            length_bytes(a2705)
        };
        let a2707 = v2;
        if nat_le(a2706, a2707) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m2708 = f609(v1.clone(), v2)?;
            match m2708 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m2712 = f566(v3, v1, {
                        let a2710 = v2;
                        let a2711 = {
                            let (_, h2709) = v4;
                            h2709
                        };
                        nat_add(a2710, a2711)?
                    })?;
                    match m2712 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h2713, _) = v4;
                                h2713
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f567(v0: Bytes) -> R<Result<((bool, bool), List<Adt29>), Bytes>> {
    let v1: u64 = {
        let a2714 = v0.clone();
        length_bytes(a2714)
    };
    let v2: bool = {
        let a2715 = 0u64;
        let a2716 = v1;
        if nat_lt(a2715, a2716) {
            let a2717 = f342(v0.clone(), 0u64)?;
            let a2718 = 94u64;
            nat_eq(a2717, a2718)
        } else {
            false
        }
    };
    let v3: bool = {
        let a2719 = 0u64;
        let a2720 = v1;
        if nat_lt(a2719, a2720) {
            f344({
                let a2723 = f342(v0.clone(), {
                    let a2721 = v1;
                    let a2722 = 1u64;
                    nat_sub(a2721, a2722)
                })?;
                let a2724 = 36u64;
                nat_eq(a2723, a2724)
            }, f610({
                let a2727 = f342(v0.clone(), {
                    let a2725 = v1;
                    let a2726 = 2u64;
                    nat_sub(a2725, a2726)
                })?;
                let a2728 = 92u64;
                nat_eq(a2727, a2728)
            }))
        } else {
            false
        }
    };
    let m2733 = f611({
        let a2729 = v1;
        let a2730 = 1u64;
        nat_add(a2729, a2730)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a2731 = v1;
        let a2732 = 1u64;
        nat_sub(a2731, a2732)
    } else {
        v1
    })?;
    match m2733 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt29>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt29>), Bytes>(v4))
        }
    }
}

pub fn f568(v0: u64, v1: List<Adt29>, v2: List<u64>, v3: bool) -> R<bool> {
    let m2734 = v0;
    if m2734 == 0 {
        Ok(false)
    } else {
        let v4 = m2734 - 1;
        let m2735 = v1.clone();
        match m2735.uncons() {
            None => {
                if v3 {
                    let a2736 = f529(v2)?;
                    let a2737 = 0u64;
                    Ok(nat_eq(a2736, a2737))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m2738 = v5.clone();
                match m2738 {
                    Adt29::C0(v7, v8, v9) => {
                        let a2739 = v9;
                        let a2740 = 1u64;
                        if nat_eq(a2739, a2740) {
                            if f568(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m2741 = v2;
                                match m2741.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f612(v5, v10) {
                                            f568(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a2742 = v9;
                            let a2743 = 2u64;
                            if nat_eq(a2742, a2743) {
                                let m2744 = v2;
                                match m2744.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f612(v5, v12) {
                                            f568(v4, List::cons(Adt29::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a2745 = v9;
                                let a2746 = 3u64;
                                if nat_eq(a2745, a2746) {
                                    if f568(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m2747 = v2;
                                        match m2747.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f612(v5, v14) {
                                                    f568(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m2748 = v2;
                                    match m2748.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f612(v5, v16) {
                                                f568(v4, v6, v17, v3)
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

pub fn f569(v0: List<Adt29>) -> R<u64> {
    let m2749 = v0;
    match m2749.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2750 = f569(v2)?;
            let a2751 = 1u64;
            nat_add(a2750, a2751)
        }
    }
}

pub fn f570(v0: List<Adt29>, v1: List<u64>, v2: bool) -> R<bool> {
    if f568({
        let a2754 = {
            let a2752 = f569(v0.clone())?;
            let a2753 = f529(v1.clone())?;
            nat_add(a2752, a2753)?
        };
        let a2755 = 2u64;
        nat_add(a2754, a2755)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f613(v0, v1, v2)
    }
}

pub fn f571(v0: u64) -> R<u64> {
    let m2756 = v0;
    if m2756 == 0 {
        Ok(1u64)
    } else {
        let v1 = m2756 - 1;
        let a2757 = f571(v1)?;
        let a2758 = 10u64;
        nat_mul(a2757, a2758)
    }
}

pub fn f572(p2772: u64, v1: Bytes, p2773: u64, p2774: Bytes) -> R<Bytes> {
    let m2771 = (p2772, (p2773, p2774));
    {
        let mut m2771 = m2771;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2771;
                let m2759 = v0;
                if m2759 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2759 - 1;
                    let m2762 = {
                        let a2760 = v1.clone();
                        let a2761 = v2;
                        index_bytes(a2760, a2761)
                    };
                    match m2762 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a2763 = v2;
                                let a2764 = 1u64;
                                nat_add(a2763, a2764)?
                            }, {
                                let a2769 = v3;
                                let a2770 = {
                                    let a2765 = v6;
                                    let a2766 = 126u64;
                                    if nat_eq(a2765, a2766) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a2767 = v6;
                                        let a2768 = 47u64;
                                        if nat_eq(a2767, a2768) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f35(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a2769, a2770)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f342(v0.clone(), {
        let a2775 = v1;
        let a2776 = 1u64;
        nat_add(a2775, a2776)?
    })?;
    let v4: Bytes = {
        let a2777 = v3;
        let a2778 = 117u64;
        if nat_eq(a2777, a2778) {
            f614(v0, v1)?
        } else {
            f615(v3)
        }
    };
    let a2779 = v4.clone();
    let a2780 = Bytes::lit(&[]);
    if equal(a2779, a2780) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2781 = v3;
            let a2782 = 117u64;
            if nat_eq(a2781, a2782) {
                let a2783 = v1;
                let a2784 = 6u64;
                nat_add(a2783, a2784)?
            } else {
                let a2785 = v1;
                let a2786 = 2u64;
                nat_add(a2785, a2786)?
            }
        }, ({
            let a2787 = v2;
            let a2788 = v4;
            append_bytes(a2787, a2788)
        }, 0u64))))
    }
}

pub fn f574(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2791 = {
        let a2789 = v0;
        let a2790 = v1;
        index_bytes(a2789, a2790)
    };
    match m2791 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f616(v2)? {
                Ok(Some({
                    let a2792 = v1;
                    let a2793 = 1u64;
                    nat_add(a2792, a2793)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f575(v0: u8) -> R<u64> {
    let a2840 = {
        let v1: u8 = v0;
        let a2838 = {
            let a2814 = {
                let a2802 = {
                    let a2796 = {
                        let a2794 = v1;
                        let a2795 = 128u8;
                        fixed_u8::bit_and(a2794, a2795)
                    };
                    let a2797 = 128u8;
                    if equal(a2796, a2797) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2803 = {
                    let a2800 = {
                        let a2798 = v1;
                        let a2799 = 64u8;
                        fixed_u8::bit_and(a2798, a2799)
                    };
                    let a2801 = 64u8;
                    if equal(a2800, a2801) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2802, a2803)?
            };
            let a2815 = {
                let a2812 = {
                    let a2806 = {
                        let a2804 = v1;
                        let a2805 = 32u8;
                        fixed_u8::bit_and(a2804, a2805)
                    };
                    let a2807 = 32u8;
                    if equal(a2806, a2807) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2813 = {
                    let a2810 = {
                        let a2808 = v1;
                        let a2809 = 16u8;
                        fixed_u8::bit_and(a2808, a2809)
                    };
                    let a2811 = 16u8;
                    if equal(a2810, a2811) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2812, a2813)?
            };
            nat_add(a2814, a2815)?
        };
        let a2839 = {
            let a2836 = {
                let a2824 = {
                    let a2818 = {
                        let a2816 = v1;
                        let a2817 = 8u8;
                        fixed_u8::bit_and(a2816, a2817)
                    };
                    let a2819 = 8u8;
                    if equal(a2818, a2819) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2825 = {
                    let a2822 = {
                        let a2820 = v1;
                        let a2821 = 4u8;
                        fixed_u8::bit_and(a2820, a2821)
                    };
                    let a2823 = 4u8;
                    if equal(a2822, a2823) {
                        4u64
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
                        let a2827 = 2u8;
                        fixed_u8::bit_and(a2826, a2827)
                    };
                    let a2829 = 2u8;
                    if equal(a2828, a2829) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2835 = {
                    let a2832 = {
                        let a2830 = v1;
                        let a2831 = 1u8;
                        fixed_u8::bit_and(a2830, a2831)
                    };
                    let a2833 = 1u8;
                    if equal(a2832, a2833) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2834, a2835)?
            };
            nat_add(a2836, a2837)?
        };
        nat_add(a2838, a2839)?
    };
    let a2841 = 48u64;
    Ok(nat_sub(a2840, a2841))
}

pub fn f576(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2842 = v0.clone();
        length_bytes(a2842)
    };
    let a2843 = v1;
    let a2844 = v2;
    if if nat_lt(a2843, a2844) {
        let a2849 = f342(v0, {
            let a2847 = v2;
            let a2848 = {
                let a2845 = v1;
                let a2846 = 1u64;
                nat_add(a2845, a2846)?
            };
            nat_sub(a2847, a2848)
        })?;
        let a2850 = 48u64;
        nat_eq(a2849, a2850)
    } else {
        false
    } {
        Ok(Some({
            let a2851 = v1;
            let a2852 = 1u64;
            nat_add(a2851, a2852)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f577() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f578(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f499({
        let (_, h2853) = {
            let (h2854, _) = f617(Fn22::F618, {
                let a2856 = {
                    let a2855 = v0.clone();
                    length_list(a2855)
                };
                let a2857 = 1u64;
                nat_add(a2856, a2857)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2854
        };
        h2853
    }))
}

pub fn f579(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2858, _) = v0;
        h2858
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2859) = v0;
        h2859
    };
    let m2860 = v1;
    match m2860.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2861 = v4;
            match m2861.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f619(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f580(v0: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f512(f35(v0.clone(), 8u64, {
            let a2863 = {
                let a2862 = v0;
                length_bytes(a2862)
            };
            let a2864 = 8u64;
            nat_sub(a2863, a2864)
        }))?))
    } else {
        if f72(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f512(f35(v0.clone(), 14u64, {
                let a2866 = {
                    let a2865 = v0;
                    length_bytes(a2865)
                };
                let a2867 = 14u64;
                nat_sub(a2866, a2867)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f581(p2884: u64, p2885: u64, p2886: u64, p2887: Bytes) -> Bytes {
    let m2883 = (p2884, (p2885, (p2886, p2887)));
    {
        let mut m2883 = m2883;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2883;
                let m2868 = v0;
                if m2868 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2868 - 1;
                    if f344({
                        let a2869 = v1;
                        let a2870 = 0u64;
                        nat_eq(a2869, a2870)
                    }, {
                        let a2871 = v2;
                        let a2872 = 0u64;
                        nat_eq(a2871, a2872)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2873 = v1;
                            let a2874 = 16u64;
                            let a2875 = 0u64;
                            nat_quot(a2873, a2874, a2875)
                        }, ({
                            let a2876 = v2;
                            let a2877 = 1u64;
                            nat_sub(a2876, a2877)
                        }, {
                            let a2881 = f620({
                                let a2878 = v1;
                                let a2879 = 16u64;
                                let a2880 = 0u64;
                                nat_rem(a2878, a2879, a2880)
                            });
                            let a2882 = v3;
                            append_bytes(a2881, a2882)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(v0: List<Adt17>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f621(f622(f237(v0.clone()))?, v0, v1)
}

pub fn f583(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f584(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f585(v0: Bytes, v1: List<Adt17>) -> R<Result<Adt28, Bytes>> {
    let m2888 = v1.clone();
    match m2888.uncons() {
        None => {
            Ok(f583(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2889 = f623(f460(f233(v2, f8(Str::lit("properties")))), v1.clone());
            match m2889.uncons() {
                None => {
                    let m2890 = f624(f622(f625(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2890 {
                        Ok(v5) => {
                            Ok(f584(f588(v0, f589(v5.clone())), f590(v5)))
                        }
                        Err(v4) => {
                            Ok(f583(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2891 = f626(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2891 {
                        Ok(v9) => {
                            Ok(f584(f588(v0, f589(v9.clone())), f590(v9)))
                        }
                        Err(v8) => {
                            Ok(f583(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f586(v0: Adt17, v1: Bytes) -> R<bool> {
    let m2892 = v0;
    match m2892 {
        Adt17::C0 => {
            Ok(false)
        }
        Adt17::C1(_, _) => {
            Ok(false)
        }
        Adt17::C2(_) => {
            Ok(false)
        }
        Adt17::C3(_, _) => {
            Ok(false)
        }
        Adt17::C4(_, _, _) => {
            Ok(false)
        }
        Adt17::C5(_) => {
            Ok(false)
        }
        Adt17::C6(r2893) => {
            let v2 = (*r2893).clone();
            f586(v2, v1)
        }
        Adt17::C7(r2894) => {
            let v3 = (*r2894).clone();
            f586(v3, v1)
        }
        Adt17::C8 => {
            Ok(false)
        }
        Adt17::C9(r2895, r2896) => {
            let v4 = (*r2895).clone();
            let v5 = (*r2896).clone();
            if f586(v4, v1.clone())? {
                Ok(true)
            } else {
                f586(v5, v1)
            }
        }
        Adt17::C10(v6, r2897, r2898) => {
            let v7 = (*r2897).clone();
            let v8 = (*r2898).clone();
            if if f28(v6, f8(Str::lit("$ref"))) {
                f627(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f586(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f586(v8, v1)
                }
            }
        }
    }
}

pub fn f587(v0: Bytes, v1: List<(Bytes, Adt17)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt30, Bytes>> {
    f628(f622(f534(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f588(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f338(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f589(v0: Adt30) -> List<Bytes> {
    let m2899 = v0;
    match m2899 {
        Adt30::C0(v1, _) => {
            v1
        }
    }
}

pub fn f590(v0: Adt30) -> List<Adt25> {
    let m2900 = v0;
    match m2900 {
        Adt30::C0(_, v2) => {
            v2
        }
    }
}

pub fn f591(v0: Adt17) -> bool {
    let m2901 = f460(f233(v0.clone(), f8(Str::lit("properties"))));
    match m2901.uncons() {
        None => {
            let m2902 = f629(v0);
            match m2902 {
                None => {
                    false
                }
                Some(v3) => {
                    f630(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f592(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f593(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2903 = f631(v0.clone());
    match m2903 {
        None => {
            if f28(f236(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2904 = f233(v0, f8(Str::lit("items")));
                match m2904 {
                    None => {
                        Ok(f632(f8(Str::lit("List")), f634()))
                    }
                    Some(v3) => {
                        Ok(f632(f8(Str::lit("List")), f633(v3, {
                            let a2905 = v1;
                            let a2906 = f8(Str::lit("Item"));
                            append_bytes(a2905, a2906)
                        })?))
                    }
                }
            } else {
                f633(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f632(f8(Str::lit("Option")), f633(v2, v1)?))
        }
    }
}

pub fn f594(p2909: List<Adt17>) -> bool {
    let m2908 = p2909;
    {
        let mut m2908 = m2908;
        loop {
            let step = {
                let v0 = m2908;
                let m2907 = v0;
                match m2907.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f635(f477(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt17>, bool>(v2)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: Adt17, v1: Bytes) -> Adt17 {
    let m2910 = v0;
    match m2910 {
        Adt17::C0 => {
            Adt17::C0
        }
        Adt17::C1(v6, r2911) => {
            let v7 = (*r2911).clone();
            Adt17::C1(v6, std::rc::Rc::new(v7))
        }
        Adt17::C2(v8) => {
            Adt17::C2(v8)
        }
        Adt17::C3(v12, v13) => {
            Adt17::C3(v12, v13)
        }
        Adt17::C4(v9, v10, v11) => {
            Adt17::C4(v9, v10, v11)
        }
        Adt17::C5(v14) => {
            Adt17::C5(v14)
        }
        Adt17::C6(r2912) => {
            let v15 = (*r2912).clone();
            Adt17::C6(std::rc::Rc::new(v15))
        }
        Adt17::C7(r2913) => {
            let v5 = (*r2913).clone();
            Adt17::C7(std::rc::Rc::new(f595(v5, v1)))
        }
        Adt17::C8 => {
            Adt17::C8
        }
        Adt17::C9(r2914, r2915) => {
            let v16 = (*r2914).clone();
            let v17 = (*r2915).clone();
            Adt17::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt17::C10(v2, r2916, r2917) => {
            let v3 = (*r2916).clone();
            let v4 = (*r2917).clone();
            if f28(v2.clone(), v1.clone()) {
                f595(v4, v1)
            } else {
                Adt17::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f595(v4, v1)))
            }
        }
    }
}

pub fn f596(v0: Adt17, v1: Bytes) -> bool {
    let m2918 = f233(v0, v1);
    match m2918 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f597(v0: List<Adt17>) -> bool {
    let m2919 = v0.clone();
    match m2919.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2920 = f623(f460(f233(v1, f8(Str::lit("properties")))), v0.clone());
            match m2920.uncons() {
                None => {
                    f637(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f636(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f598(v0: Adt17, v1: Bytes, v2: Adt17) -> Adt17 {
    let m2921 = v0;
    match m2921 {
        Adt17::C0 => {
            Adt17::C0
        }
        Adt17::C1(v7, r2922) => {
            let v8 = (*r2922).clone();
            Adt17::C1(v7, std::rc::Rc::new(v8))
        }
        Adt17::C2(v9) => {
            Adt17::C2(v9)
        }
        Adt17::C3(v13, v14) => {
            Adt17::C3(v13, v14)
        }
        Adt17::C4(v10, v11, v12) => {
            Adt17::C4(v10, v11, v12)
        }
        Adt17::C5(v15) => {
            Adt17::C5(v15)
        }
        Adt17::C6(r2923) => {
            let v16 = (*r2923).clone();
            Adt17::C6(std::rc::Rc::new(v16))
        }
        Adt17::C7(r2924) => {
            let v3 = (*r2924).clone();
            Adt17::C7(std::rc::Rc::new(Adt17::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt17::C8 => {
            Adt17::C8
        }
        Adt17::C9(r2925, r2926) => {
            let v17 = (*r2925).clone();
            let v18 = (*r2926).clone();
            Adt17::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt17::C10(v4, r2927, r2928) => {
            let v5 = (*r2927).clone();
            let v6 = (*r2928).clone();
            Adt17::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f599(p2931: List<Adt17>) -> bool {
    let m2930 = p2931;
    {
        let mut m2930 = m2930;
        loop {
            let step = {
                let v0 = m2930;
                let m2929 = v0;
                match m2929.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f638(v1) {
                            Ok::<List<Adt17>, bool>(v2)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2930 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(v0: u64) -> u32 {
    let a2932 = v0;
    let a2933 = 32u64;
    if nat_lt(a2932, a2933) {
        let a2934 = v0;
        let a2935 = 16u64;
        if nat_lt(a2934, a2935) {
            let a2936 = v0;
            let a2937 = 8u64;
            if nat_lt(a2936, a2937) {
                let a2938 = v0;
                let a2939 = 4u64;
                if nat_lt(a2938, a2939) {
                    let a2940 = v0;
                    let a2941 = 2u64;
                    if nat_lt(a2940, a2941) {
                        let a2942 = v0;
                        let a2943 = 1u64;
                        if nat_lt(a2942, a2943) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2944 = v0;
                        let a2945 = 3u64;
                        if nat_lt(a2944, a2945) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2946 = v0;
                    let a2947 = 6u64;
                    if nat_lt(a2946, a2947) {
                        let a2948 = v0;
                        let a2949 = 5u64;
                        if nat_lt(a2948, a2949) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2950 = v0;
                        let a2951 = 7u64;
                        if nat_lt(a2950, a2951) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2952 = v0;
                let a2953 = 12u64;
                if nat_lt(a2952, a2953) {
                    let a2954 = v0;
                    let a2955 = 10u64;
                    if nat_lt(a2954, a2955) {
                        let a2956 = v0;
                        let a2957 = 9u64;
                        if nat_lt(a2956, a2957) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2958 = v0;
                        let a2959 = 11u64;
                        if nat_lt(a2958, a2959) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2960 = v0;
                    let a2961 = 14u64;
                    if nat_lt(a2960, a2961) {
                        let a2962 = v0;
                        let a2963 = 13u64;
                        if nat_lt(a2962, a2963) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2964 = v0;
                        let a2965 = 15u64;
                        if nat_lt(a2964, a2965) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2966 = v0;
            let a2967 = 24u64;
            if nat_lt(a2966, a2967) {
                let a2968 = v0;
                let a2969 = 20u64;
                if nat_lt(a2968, a2969) {
                    let a2970 = v0;
                    let a2971 = 18u64;
                    if nat_lt(a2970, a2971) {
                        let a2972 = v0;
                        let a2973 = 17u64;
                        if nat_lt(a2972, a2973) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2974 = v0;
                        let a2975 = 19u64;
                        if nat_lt(a2974, a2975) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2976 = v0;
                    let a2977 = 22u64;
                    if nat_lt(a2976, a2977) {
                        let a2978 = v0;
                        let a2979 = 21u64;
                        if nat_lt(a2978, a2979) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2980 = v0;
                        let a2981 = 23u64;
                        if nat_lt(a2980, a2981) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2982 = v0;
                let a2983 = 28u64;
                if nat_lt(a2982, a2983) {
                    let a2984 = v0;
                    let a2985 = 26u64;
                    if nat_lt(a2984, a2985) {
                        let a2986 = v0;
                        let a2987 = 25u64;
                        if nat_lt(a2986, a2987) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2988 = v0;
                        let a2989 = 27u64;
                        if nat_lt(a2988, a2989) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2990 = v0;
                    let a2991 = 30u64;
                    if nat_lt(a2990, a2991) {
                        let a2992 = v0;
                        let a2993 = 29u64;
                        if nat_lt(a2992, a2993) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2994 = v0;
                        let a2995 = 31u64;
                        if nat_lt(a2994, a2995) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2996 = v0;
        let a2997 = 48u64;
        if nat_lt(a2996, a2997) {
            let a2998 = v0;
            let a2999 = 40u64;
            if nat_lt(a2998, a2999) {
                let a3000 = v0;
                let a3001 = 36u64;
                if nat_lt(a3000, a3001) {
                    let a3002 = v0;
                    let a3003 = 34u64;
                    if nat_lt(a3002, a3003) {
                        let a3004 = v0;
                        let a3005 = 33u64;
                        if nat_lt(a3004, a3005) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3006 = v0;
                        let a3007 = 35u64;
                        if nat_lt(a3006, a3007) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3008 = v0;
                    let a3009 = 38u64;
                    if nat_lt(a3008, a3009) {
                        let a3010 = v0;
                        let a3011 = 37u64;
                        if nat_lt(a3010, a3011) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3012 = v0;
                        let a3013 = 39u64;
                        if nat_lt(a3012, a3013) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3014 = v0;
                let a3015 = 44u64;
                if nat_lt(a3014, a3015) {
                    let a3016 = v0;
                    let a3017 = 42u64;
                    if nat_lt(a3016, a3017) {
                        let a3018 = v0;
                        let a3019 = 41u64;
                        if nat_lt(a3018, a3019) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3020 = v0;
                        let a3021 = 43u64;
                        if nat_lt(a3020, a3021) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3022 = v0;
                    let a3023 = 46u64;
                    if nat_lt(a3022, a3023) {
                        let a3024 = v0;
                        let a3025 = 45u64;
                        if nat_lt(a3024, a3025) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3026 = v0;
                        let a3027 = 47u64;
                        if nat_lt(a3026, a3027) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3028 = v0;
            let a3029 = 56u64;
            if nat_lt(a3028, a3029) {
                let a3030 = v0;
                let a3031 = 52u64;
                if nat_lt(a3030, a3031) {
                    let a3032 = v0;
                    let a3033 = 50u64;
                    if nat_lt(a3032, a3033) {
                        let a3034 = v0;
                        let a3035 = 49u64;
                        if nat_lt(a3034, a3035) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3036 = v0;
                        let a3037 = 51u64;
                        if nat_lt(a3036, a3037) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3038 = v0;
                    let a3039 = 54u64;
                    if nat_lt(a3038, a3039) {
                        let a3040 = v0;
                        let a3041 = 53u64;
                        if nat_lt(a3040, a3041) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3042 = v0;
                        let a3043 = 55u64;
                        if nat_lt(a3042, a3043) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3044 = v0;
                let a3045 = 60u64;
                if nat_lt(a3044, a3045) {
                    let a3046 = v0;
                    let a3047 = 58u64;
                    if nat_lt(a3046, a3047) {
                        let a3048 = v0;
                        let a3049 = 57u64;
                        if nat_lt(a3048, a3049) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3050 = v0;
                        let a3051 = 59u64;
                        if nat_lt(a3050, a3051) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3052 = v0;
                    let a3053 = 62u64;
                    if nat_lt(a3052, a3053) {
                        let a3054 = v0;
                        let a3055 = 61u64;
                        if nat_lt(a3054, a3055) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3056 = v0;
                        let a3057 = 63u64;
                        if nat_lt(a3056, a3057) {
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

pub fn f601(v0: u32) -> u32 {
    let a3060 = {
        let a3058 = f639(v0, 6u32, 26u32);
        let a3059 = f639(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3058, a3059)
    };
    let a3061 = f639(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3060, a3061)
}

pub fn f602(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3067 = {
        let a3062 = v0;
        let a3063 = v1;
        fixed_u32::bit_and(a3062, a3063)
    };
    let a3068 = {
        let a3065 = {
            let a3064 = v0;
            fixed_u32::bit_not(a3064)
        };
        let a3066 = v2;
        fixed_u32::bit_and(a3065, a3066)
    };
    fixed_u32::bit_xor(a3067, a3068)
}

pub fn f603(v0: u32) -> u32 {
    let a3071 = {
        let a3069 = f639(v0, 2u32, 30u32);
        let a3070 = f639(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3069, a3070)
    };
    let a3072 = f639(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3071, a3072)
}

pub fn f604(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3081 = {
        let a3077 = {
            let a3073 = v0;
            let a3074 = v1;
            fixed_u32::bit_and(a3073, a3074)
        };
        let a3078 = {
            let a3075 = v0;
            let a3076 = v2;
            fixed_u32::bit_and(a3075, a3076)
        };
        fixed_u32::bit_xor(a3077, a3078)
    };
    let a3082 = {
        let a3079 = v1;
        let a3080 = v2;
        fixed_u32::bit_and(a3079, a3080)
    };
    fixed_u32::bit_xor(a3081, a3082)
}

pub fn f605(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3083, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3083
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3084, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3084
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3085, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3085
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3086, _, _, _, _, _, _, _, _, _, _, _) => {
            h3086
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3087, _, _, _, _, _, _, _, _, _, _) => {
            h3087
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3088, _, _, _, _, _, _, _, _, _) => {
            h3088
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3089, _, _, _, _, _, _, _, _) => {
            h3089
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3090, _, _, _, _, _, _, _) => {
            h3090
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3091, _, _, _, _, _, _) => {
            h3091
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3092, _, _, _, _, _) => {
            h3092
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3093, _, _, _, _) => {
            h3093
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3094, _, _, _) => {
            h3094
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3095, _, _) => {
            h3095
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3096, _) => {
            h3096
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3097) => {
            h3097
        }
    }, f521(f640(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3098, _) => {
            h3098
        }
    }), f521(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3099, _, _, _, _, _, _) => {
            h3099
        }
    }, f521(f641(match v0.clone() {
        Adt26::C0(_, h3100, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3100
        }
    }), match v0 {
        Adt26::C0(h3101, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3101
        }
    }))))
}

pub fn f606(v0: Fn21, p3105: u64, p3106: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3104 = (p3105, p3106);
    {
        let mut m3104 = m3104;
        loop {
            let step = {
                let (v1, v2) = m3104;
                let m3102 = v1;
                if m3102 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3102 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3103: Fn21 = v0.clone();
                        c3103.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f607(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3107, _) = v0;
        h3107
    };
    let v2: u32 = {
        let (h3108, _) = {
            let (_, h3109) = v0;
            h3109
        };
        h3108
    };
    let v3: u32 = {
        let (_, h3110) = {
            let (_, h3111) = v0;
            h3111
        };
        h3110
    };
    ({
        let a3112 = v1;
        let a3113 = 2u64;
        let a3114 = 0u64;
        nat_quot(a3112, a3113, a3114)
    }, (f608(v2, 1u32), {
        let a3118 = {
            let a3115 = v1;
            let a3116 = 2u64;
            let a3117 = 0u64;
            nat_rem(a3115, a3116, a3117)
        };
        let a3119 = 1u64;
        if nat_eq(a3118, a3119) {
            let a3120 = v3;
            let a3121 = v2;
            fixed_u32::bit_or(a3120, a3121)
        } else {
            v3
        }
    }))
}

pub fn f608(v0: u32, v1: u32) -> u32 {
    let m3124 = {
        let a3122 = v0;
        let a3123 = v1;
        fixed_u32::shift_left(a3122, a3123)
    };
    match m3124 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f609(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3127 = {
        let a3125 = v0.clone();
        let a3126 = v1;
        index_bytes(a3125, a3126)
    };
    match m3127 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a3128 = v3;
            let a3129 = 128u64;
            if nat_lt(a3128, a3129) {
                Ok(Some((v3, 1u64)))
            } else {
                if f344({
                    let a3130 = 194u64;
                    let a3131 = v3;
                    nat_le(a3130, a3131)
                }, {
                    let a3132 = v3;
                    let a3133 = 223u64;
                    nat_le(a3132, a3133)
                }) {
                    let m3136 = f642(v0, {
                        let a3134 = v1;
                        let a3135 = 1u64;
                        nat_add(a3134, a3135)?
                    })?;
                    match m3136 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f643({
                                let a3141 = {
                                    let a3139 = {
                                        let a3137 = v3;
                                        let a3138 = 192u64;
                                        nat_sub(a3137, a3138)
                                    };
                                    let a3140 = 64u64;
                                    nat_mul(a3139, a3140)?
                                };
                                let a3142 = v4;
                                nat_add(a3141, a3142)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f344({
                        let a3143 = 224u64;
                        let a3144 = v3;
                        nat_le(a3143, a3144)
                    }, {
                        let a3145 = v3;
                        let a3146 = 239u64;
                        nat_le(a3145, a3146)
                    }) {
                        let m3149 = f642(v0.clone(), {
                            let a3147 = v1;
                            let a3148 = 1u64;
                            nat_add(a3147, a3148)?
                        })?;
                        match m3149 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3152 = f642(v0, {
                                    let a3150 = v1;
                                    let a3151 = 2u64;
                                    nat_add(a3150, a3151)?
                                })?;
                                match m3152 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f643({
                                            let a3161 = {
                                                let a3155 = {
                                                    let a3153 = v3;
                                                    let a3154 = 224u64;
                                                    nat_sub(a3153, a3154)
                                                };
                                                let a3156 = 4096u64;
                                                nat_mul(a3155, a3156)?
                                            };
                                            let a3162 = {
                                                let a3159 = {
                                                    let a3157 = v5;
                                                    let a3158 = 64u64;
                                                    nat_mul(a3157, a3158)?
                                                };
                                                let a3160 = v6;
                                                nat_add(a3159, a3160)?
                                            };
                                            nat_add(a3161, a3162)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f344({
                            let a3163 = 240u64;
                            let a3164 = v3;
                            nat_le(a3163, a3164)
                        }, {
                            let a3165 = v3;
                            let a3166 = 244u64;
                            nat_le(a3165, a3166)
                        }) {
                            let m3169 = f642(v0.clone(), {
                                let a3167 = v1;
                                let a3168 = 1u64;
                                nat_add(a3167, a3168)?
                            })?;
                            match m3169 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3172 = f642(v0.clone(), {
                                        let a3170 = v1;
                                        let a3171 = 2u64;
                                        nat_add(a3170, a3171)?
                                    })?;
                                    match m3172 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3175 = f642(v0, {
                                                let a3173 = v1;
                                                let a3174 = 3u64;
                                                nat_add(a3173, a3174)?
                                            })?;
                                            match m3175 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f643({
                                                        let a3188 = {
                                                            let a3178 = {
                                                                let a3176 = v3;
                                                                let a3177 = 240u64;
                                                                nat_sub(a3176, a3177)
                                                            };
                                                            let a3179 = 262144u64;
                                                            nat_mul(a3178, a3179)?
                                                        };
                                                        let a3189 = {
                                                            let a3186 = {
                                                                let a3180 = v7;
                                                                let a3181 = 4096u64;
                                                                nat_mul(a3180, a3181)?
                                                            };
                                                            let a3187 = {
                                                                let a3184 = {
                                                                    let a3182 = v8;
                                                                    let a3183 = 64u64;
                                                                    nat_mul(a3182, a3183)?
                                                                };
                                                                let a3185 = v9;
                                                                nat_add(a3184, a3185)?
                                                            };
                                                            nat_add(a3186, a3187)?
                                                        };
                                                        nat_add(a3188, a3189)?
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

pub fn f610(v0: bool) -> bool {
    !v0
}

pub fn f611(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt29>, Bytes>> {
    let m3190 = v0;
    if m3190 == 0 {
        Ok(Ok::<List<Adt29>, Bytes>(List::<Adt29>::nil()))
    } else {
        let v4 = m3190 - 1;
        let a3191 = v3;
        let a3192 = v2;
        if nat_le(a3191, a3192) {
            Ok(Ok::<List<Adt29>, Bytes>(List::<Adt29>::nil()))
        } else {
            let v5: u64 = f342(v1.clone(), v2)?;
            let m3241 = {
                let a3193 = v5;
                let a3194 = 91u64;
                if nat_eq(a3193, a3194) {
                    let a3197 = f342(v1.clone(), {
                        let a3195 = v2;
                        let a3196 = 1u64;
                        nat_add(a3195, a3196)?
                    })?;
                    let a3198 = 94u64;
                    if nat_eq(a3197, a3198) {
                        let m3202 = f644({
                            let a3199 = v1.clone();
                            length_bytes(a3199)
                        }, v1.clone(), {
                            let a3200 = v2;
                            let a3201 = 2u64;
                            nat_add(a3200, a3201)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3202 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3203, _) = v7;
                                    h3203
                                }), {
                                    let (_, h3204) = v7;
                                    h3204
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3208 = f644({
                            let a3205 = v1.clone();
                            length_bytes(a3205)
                        }, v1.clone(), {
                            let a3206 = v2;
                            let a3207 = 1u64;
                            nat_add(a3206, a3207)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3208 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3209, _) = v9;
                                    h3209
                                }), {
                                    let (_, h3210) = v9;
                                    h3210
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3211 = v5;
                    let a3212 = 46u64;
                    if nat_eq(a3211, a3212) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3213 = v2;
                            let a3214 = 1u64;
                            nat_add(a3213, a3214)?
                        }))
                    } else {
                        let a3215 = v5;
                        let a3216 = 40u64;
                        if if if if if if nat_eq(a3215, a3216) {
                            true
                        } else {
                            let a3217 = v5;
                            let a3218 = 41u64;
                            nat_eq(a3217, a3218)
                        } {
                            true
                        } else {
                            let a3219 = v5;
                            let a3220 = 124u64;
                            if nat_eq(a3219, a3220) {
                                true
                            } else {
                                let a3221 = v5;
                                let a3222 = 123u64;
                                nat_eq(a3221, a3222)
                            }
                        } {
                            true
                        } else {
                            let a3223 = v5;
                            let a3224 = 125u64;
                            if if nat_eq(a3223, a3224) {
                                true
                            } else {
                                let a3225 = v5;
                                let a3226 = 92u64;
                                nat_eq(a3225, a3226)
                            } {
                                true
                            } else {
                                let a3227 = v5;
                                let a3228 = 94u64;
                                if nat_eq(a3227, a3228) {
                                    true
                                } else {
                                    let a3229 = v5;
                                    let a3230 = 36u64;
                                    nat_eq(a3229, a3230)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3231 = v5;
                            let a3232 = 42u64;
                            if if nat_eq(a3231, a3232) {
                                true
                            } else {
                                let a3233 = v5;
                                let a3234 = 43u64;
                                nat_eq(a3233, a3234)
                            } {
                                true
                            } else {
                                let a3235 = v5;
                                let a3236 = 63u64;
                                nat_eq(a3235, a3236)
                            }
                        } {
                            true
                        } else {
                            let a3237 = 127u64;
                            let a3238 = v5;
                            nat_lt(a3237, a3238)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f35(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3239 = v2;
                                let a3240 = 1u64;
                                nat_add(a3239, a3240)?
                            }))
                        }
                    }
                }
            };
            match m3241 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3242) = v11.clone();
                        h3242
                    };
                    let v13: u64 = {
                        let a3243 = v12;
                        let a3244 = v3;
                        if nat_lt(a3243, a3244) {
                            f645(f342(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3245 = v12;
                    let a3246 = v3;
                    if if nat_lt(a3245, a3246) {
                        let a3247 = f342(v1.clone(), v12)?;
                        let a3248 = 123u64;
                        nat_eq(a3247, a3248)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt29>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3253 = f611(v4, v1, {
                            let a3249 = 0u64;
                            let a3250 = v13;
                            if nat_lt(a3249, a3250) {
                                let a3251 = v12;
                                let a3252 = 1u64;
                                nat_add(a3251, a3252)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3253 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt29>, Bytes>(List::cons(Adt29::C0({
                                    let (h3254, _) = {
                                        let (h3255, _) = v11.clone();
                                        h3255
                                    };
                                    h3254
                                }, {
                                    let (_, h3256) = {
                                        let (h3257, _) = v11;
                                        h3257
                                    };
                                    h3256
                                }, v13), v15)))
                            }
                            Err(v14) => {
                                Ok(Err::<List<Adt29>, Bytes>(v14))
                            }
                        }
                    }
                }
                Err(v10) => {
                    Ok(Err::<List<Adt29>, Bytes>(v10))
                }
            }
        }
    }
}

pub fn f612(v0: Adt29, v1: u64) -> bool {
    let m3258 = v0;
    match m3258 {
        Adt29::C0(v2, v3, _) => {
            if v2 {
                f610(f646(v3, v1))
            } else {
                f646(v3, v1)
            }
        }
    }
}

pub fn f613(v0: List<Adt29>, p3265: List<u64>, v2: bool) -> R<bool> {
    let m3264 = p3265;
    {
        let mut m3264 = m3264;
        loop {
            let step = {
                let v1 = m3264;
                let m3259 = v1;
                match m3259.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f568({
                            let a3262 = {
                                let a3260 = f569(v0.clone())?;
                                let a3261 = f529(v4.clone())?;
                                nat_add(a3260, a3261)?
                            };
                            let a3263 = 2u64;
                            nat_add(a3262, a3263)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3264 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3268 = f35(v0.clone(), {
        let a3266 = v1;
        let a3267 = 2u64;
        nat_add(a3266, a3267)?
    }, 2u64);
    let a3269 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3268, a3269) {
        let m3272 = f647(f35(v0, {
            let a3270 = v1;
            let a3271 = 4u64;
            nat_add(a3270, a3271)?
        }, 2u64))?;
        match m3272 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f342(v2.clone(), 0u64)?;
                let a3273 = v3;
                let a3274 = 32u64;
                if nat_lt(a3273, a3274) {
                    if f648(v3) {
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

pub fn f615(v0: u64) -> Bytes {
    let a3275 = v0;
    let a3276 = 34u64;
    if nat_eq(a3275, a3276) {
        Bytes::lit(&[0x22])
    } else {
        let a3277 = v0;
        let a3278 = 92u64;
        if nat_eq(a3277, a3278) {
            Bytes::lit(&[0x5c])
        } else {
            let a3279 = v0;
            let a3280 = 98u64;
            if nat_eq(a3279, a3280) {
                Bytes::lit(&[0x08])
            } else {
                let a3281 = v0;
                let a3282 = 102u64;
                if nat_eq(a3281, a3282) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3283 = v0;
                    let a3284 = 110u64;
                    if nat_eq(a3283, a3284) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3285 = v0;
                        let a3286 = 114u64;
                        if nat_eq(a3285, a3286) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3287 = v0;
                            let a3288 = 116u64;
                            if nat_eq(a3287, a3288) {
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

pub fn f616(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3333 = {
            let a3309 = {
                let a3297 = {
                    let a3291 = {
                        let a3289 = v1;
                        let a3290 = 128u8;
                        fixed_u8::bit_and(a3289, a3290)
                    };
                    let a3292 = 128u8;
                    if equal(a3291, a3292) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3298 = {
                    let a3295 = {
                        let a3293 = v1;
                        let a3294 = 64u8;
                        fixed_u8::bit_and(a3293, a3294)
                    };
                    let a3296 = 64u8;
                    if equal(a3295, a3296) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3297, a3298)?
            };
            let a3310 = {
                let a3307 = {
                    let a3301 = {
                        let a3299 = v1;
                        let a3300 = 32u8;
                        fixed_u8::bit_and(a3299, a3300)
                    };
                    let a3302 = 32u8;
                    if equal(a3301, a3302) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3308 = {
                    let a3305 = {
                        let a3303 = v1;
                        let a3304 = 16u8;
                        fixed_u8::bit_and(a3303, a3304)
                    };
                    let a3306 = 16u8;
                    if equal(a3305, a3306) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3307, a3308)?
            };
            nat_add(a3309, a3310)?
        };
        let a3334 = {
            let a3331 = {
                let a3319 = {
                    let a3313 = {
                        let a3311 = v1;
                        let a3312 = 8u8;
                        fixed_u8::bit_and(a3311, a3312)
                    };
                    let a3314 = 8u8;
                    if equal(a3313, a3314) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3320 = {
                    let a3317 = {
                        let a3315 = v1;
                        let a3316 = 4u8;
                        fixed_u8::bit_and(a3315, a3316)
                    };
                    let a3318 = 4u8;
                    if equal(a3317, a3318) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3319, a3320)?
            };
            let a3332 = {
                let a3329 = {
                    let a3323 = {
                        let a3321 = v1;
                        let a3322 = 2u8;
                        fixed_u8::bit_and(a3321, a3322)
                    };
                    let a3324 = 2u8;
                    if equal(a3323, a3324) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3330 = {
                    let a3327 = {
                        let a3325 = v1;
                        let a3326 = 1u8;
                        fixed_u8::bit_and(a3325, a3326)
                    };
                    let a3328 = 1u8;
                    if equal(a3327, a3328) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3329, a3330)?
            };
            nat_add(a3331, a3332)?
        };
        nat_add(a3333, a3334)?
    };
    let a3335 = 48u64;
    let a3336 = v2;
    if nat_le(a3335, a3336) {
        let a3337 = v2;
        let a3338 = 57u64;
        Ok(nat_le(a3337, a3338))
    } else {
        Ok(false)
    }
}

pub fn f617(v0: Fn22, p3343: u64, p3344: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3342 = (p3343, p3344);
    {
        let mut m3342 = m3342;
        loop {
            let step = {
                let (v1, v2) = m3342;
                let m3339 = v1;
                if m3339 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3339 - 1;
                    let m3341 = {
                        let c3340: Fn22 = v0.clone();
                        c3340.apply(v2.clone())?
                    };
                    match m3341 {
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
                Ok(next) => { m3342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f649(v0)
}

pub fn f619(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3349 = {
            let a3347 = {
                let a3345 = v0.clone();
                length_list(a3345)
            };
            let a3348 = {
                let a3346 = v1.clone();
                length_list(a3346)
            };
            nat_add(a3347, a3348)?
        };
        let a3350 = 1u64;
        nat_add(a3349, a3350)?
    };
    Ok(f387({
        let (_, h3351) = {
            let (_, h3352) = {
                let (h3353, _) = f650(Fn23::F651, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3353
            };
            h3352
        };
        h3351
    }))
}

pub fn f620(v0: u64) -> Bytes {
    f35({
        let a3354 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3354)
    }, v0, 1u64)
}

pub fn f621(v0: bool, v1: List<Adt17>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3355 = v1;
    match m3355.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3356 = f652(v0, v3.clone())?;
            match m3356 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f418(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3357 = f621(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3357 {
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

pub fn f622(p3361: List<Bytes>) -> R<bool> {
    let m3360 = p3361;
    {
        let mut m3360 = m3360;
        loop {
            let step = {
                let v0 = m3360;
                let m3358 = v0;
                match m3358.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3359 = f653(v1)?;
                        match m3359 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f41(v3, f654(false, v2.clone())?) {
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
                Ok(next) => { m3360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623(v0: List<(Bytes, Adt17)>, v1: List<Adt17>) -> List<Bytes> {
    let m3362 = v0;
    match m3362.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f655(v1.clone(), {
                let (h3363, _) = v2.clone();
                h3363
            }) {
                List::cons({
                    let (h3364, _) = v2;
                    h3364
                }, f623(v3, v1))
            } else {
                f623(v3, v1)
            }
        }
    }
}

pub fn f624(v0: bool, v1: Bytes, v2: List<Adt17>, v3: List<Bytes>) -> R<Result<Adt30, Bytes>> {
    let m3365 = v2;
    match m3365.uncons() {
        None => {
            Ok(Ok::<Adt30, Bytes>(Adt30::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3366 = f656(v4.clone())?;
            match m3366 {
                None => {
                    Ok(Err::<Adt30, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3367 = f657(v0, v6.clone())?;
                    match m3367 {
                        None => {
                            Ok(Err::<Adt30, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f41(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt30, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3370 = f658(v4, {
                                    let a3368 = v1.clone();
                                    let a3369 = f321(v6)?;
                                    append_bytes(a3368, a3369)
                                })?;
                                match m3370 {
                                    Ok(v9) => {
                                        let m3371 = f624(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3371 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt30, Bytes>(Adt30::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f517(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f589(v11.clone())), f464(f516(v9), f590(v11)))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<Adt30, Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<Adt30, Bytes>(v8))
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

pub fn f625(v0: List<Adt17>) -> R<List<Bytes>> {
    let m3372 = v0;
    match m3372.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3373 = f656(v1)?;
            match m3373 {
                None => {
                    f625(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f625(v2)?))
                }
            }
        }
    }
}

pub fn f626(v0: Bytes, v1: List<Adt17>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt30, Bytes>> {
    f659(f622(f660(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f627(v0: Adt17, v1: Bytes) -> R<bool> {
    let m3374 = f327(Some(v0));
    match m3374 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3375 = f580(v2)?;
            match m3375 {
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

pub fn f628(v0: bool, v1: Bytes, v2: List<(Bytes, Adt17)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt30, Bytes>> {
    let m3376 = v2;
    match m3376.uncons() {
        None => {
            Ok(Ok::<Adt30, Bytes>(Adt30::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3378 = f661(v0, {
                let (h3377, _) = v5.clone();
                h3377
            })?;
            match m3378 {
                None => {
                    Ok(Err::<Adt30, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3379, _) = v5;
                        h3379
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f41(v7.clone(), f662(v0, f534(v6.clone()))?) {
                        Ok(Err::<Adt30, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3384 = f593({
                            let (_, h3380) = v5.clone();
                            h3380
                        }, {
                            let a3382 = v1.clone();
                            let a3383 = f321({
                                let (h3381, _) = v5.clone();
                                h3381
                            })?;
                            append_bytes(a3382, a3383)
                        })?;
                        match m3384 {
                            Ok(v9) => {
                                let v10: Bytes = if f41({
                                    let (h3385, _) = v5;
                                    h3385
                                }, v3.clone()) {
                                    f517(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f517(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3386 = f628(v0, v1, v6, v3, v4)?;
                                match m3386 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt30, Bytes>(Adt30::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f589(v12.clone())), f464(f516(v9), f590(v12)))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt30, Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<Adt30, Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f629(v0: Adt17) -> Option<Adt17> {
    let m3387 = f233(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3387 {
        None => {
            f663(v0)
        }
        Some(v1) => {
            if f630(v1.clone()) {
                let m3388 = f663(v0);
                match m3388 {
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

pub fn f630(v0: Adt17) -> bool {
    let m3389 = v0;
    match m3389 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_, _) => {
            false
        }
        Adt17::C2(v3) => {
            !v3
        }
        Adt17::C3(_, _) => {
            false
        }
        Adt17::C4(_, _, _) => {
            false
        }
        Adt17::C5(_) => {
            false
        }
        Adt17::C6(_) => {
            false
        }
        Adt17::C7(_) => {
            false
        }
        Adt17::C8 => {
            false
        }
        Adt17::C9(_, _) => {
            false
        }
        Adt17::C10(_, _, _) => {
            false
        }
    }
}

pub fn f631(v0: Adt17) -> Option<Adt17> {
    let m3390 = f238(f233(v0, f8(Str::lit("oneOf"))));
    match m3390.uncons() {
        None => {
            None::<Adt17>
        }
        Some((v1, v2)) => {
            let m3391 = v2;
            match m3391.uncons() {
                None => {
                    None::<Adt17>
                }
                Some((v3, v4)) => {
                    let m3392 = v4;
                    match m3392.uncons() {
                        None => {
                            if f664(v3.clone()) {
                                Some(v1)
                            } else {
                                if f664(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt17>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt17>
                        }
                    }
                }
            }
        }
    }
}

pub fn f632(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3393 = v1;
    match m3393 {
        Ok(v3) => {
            f584(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f517(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f516(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f633(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3394 = f631(v0.clone());
    match m3394 {
        None => {
            if f28(f236(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3395 = f233(v0, f8(Str::lit("items")));
                match m3395 {
                    None => {
                        Ok(f632(f8(Str::lit("List")), f634()))
                    }
                    Some(v3) => {
                        Ok(f632(f8(Str::lit("List")), f658(v3, {
                            let a3396 = v1;
                            let a3397 = f8(Str::lit("Item"));
                            append_bytes(a3396, a3397)
                        })?))
                    }
                }
            } else {
                f658(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f632(f8(Str::lit("Option")), f658(v2, v1)?))
        }
    }
}

pub fn f634() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f513(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f635(p3401: List<(Bytes, Adt17)>, v1: List<Bytes>) -> bool {
    let m3400 = p3401;
    {
        let mut m3400 = m3400;
        loop {
            let step = {
                let v0 = m3400;
                let m3398 = v0;
                match m3398.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h3399, _) = v2;
                            h3399
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt17)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt17)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f636(p3405: List<Adt17>, v1: Bytes, p3406: List<Bytes>) -> bool {
    let m3404 = (p3405, p3406);
    {
        let mut m3404 = m3404;
        loop {
            let step = {
                let (v0, v2) = m3404;
                let m3402 = v0;
                match m3402.uncons() {
                    None => {
                        Err::<(List<Adt17>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3403 = f665(v3, v1.clone());
                        match m3403 {
                            None => {
                                Err::<(List<Adt17>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f41(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt17>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt17>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f637(p3409: List<Adt17>, p3410: List<Bytes>) -> bool {
    let m3408 = (p3409, p3410);
    {
        let mut m3408 = m3408;
        loop {
            let step = {
                let (v0, v1) = m3408;
                let m3407 = v0;
                match m3407.uncons() {
                    None => {
                        Err::<(List<Adt17>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f236(v2, f8(Str::lit("type")));
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
                                    Ok::<(List<Adt17>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt17>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt17>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt17>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f638(v0: Adt17) -> bool {
    let v2: Adt17 = {
        let m3411 = f233(v0.clone(), f8(Str::lit("anyOf")));
        match m3411 {
            None => {
                Adt17::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f635(f477(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f594(f328(v2))
    } else {
        false
    }
}

pub fn f639(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3412 = f470(v0, v1);
    let a3413 = f608(v0, v2);
    fixed_u32::bit_or(a3412, a3413)
}

pub fn f640(v0: u32) -> u32 {
    let a3416 = {
        let a3414 = f639(v0, 17u32, 15u32);
        let a3415 = f639(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3414, a3415)
    };
    let a3417 = f470(v0, 10u32);
    fixed_u32::bit_xor(a3416, a3417)
}

pub fn f641(v0: u32) -> u32 {
    let a3420 = {
        let a3418 = f639(v0, 7u32, 25u32);
        let a3419 = f639(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3418, a3419)
    };
    let a3421 = f470(v0, 3u32);
    fixed_u32::bit_xor(a3420, a3421)
}

pub fn f642(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3424 = {
        let a3422 = v0;
        let a3423 = v1;
        index_bytes(a3422, a3423)
    };
    match m3424 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f344({
                let a3425 = 128u64;
                let a3426 = v3;
                nat_le(a3425, a3426)
            }, {
                let a3427 = v3;
                let a3428 = 191u64;
                nat_le(a3427, a3428)
            }) {
                Ok(Some({
                    let a3429 = v3;
                    let a3430 = 128u64;
                    nat_sub(a3429, a3430)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f643(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f344({
        let a3431 = v1;
        let a3432 = v0;
        nat_le(a3431, a3432)
    }, f344({
        let a3433 = v0;
        let a3434 = v2;
        nat_le(a3433, a3434)
    }, f426({
        let a3435 = v0;
        let a3436 = 55296u64;
        nat_lt(a3435, a3436)
    }, {
        let a3437 = 57343u64;
        let a3438 = v0;
        nat_lt(a3437, a3438)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f644(p3475: u64, v1: Bytes, p3476: u64, p3477: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3474 = (p3475, (p3476, p3477));
    {
        let mut m3474 = m3474;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3474;
                let m3439 = v0;
                if m3439 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3439 - 1;
                    let v5: u64 = f342(v1.clone(), v2)?;
                    let a3441 = {
                        let a3440 = v1.clone();
                        length_bytes(a3440)
                    };
                    let a3442 = v2;
                    if nat_le(a3441, a3442) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3443 = v5;
                        let a3444 = 93u64;
                        if nat_eq(a3443, a3444) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3445 = v2;
                                let a3446 = 1u64;
                                nat_add(a3445, a3446)?
                            }))))
                        } else {
                            let a3447 = v5;
                            let a3448 = 92u64;
                            if if nat_eq(a3447, a3448) {
                                true
                            } else {
                                let a3449 = 127u64;
                                let a3450 = v5;
                                nat_lt(a3449, a3450)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3453 = f342(v1.clone(), {
                                    let a3451 = v2;
                                    let a3452 = 1u64;
                                    nat_add(a3451, a3452)?
                                })?;
                                let a3454 = 45u64;
                                if if nat_eq(a3453, a3454) {
                                    f344({
                                        let a3458 = {
                                            let a3455 = v2;
                                            let a3456 = 2u64;
                                            nat_add(a3455, a3456)?
                                        };
                                        let a3459 = {
                                            let a3457 = v1.clone();
                                            length_bytes(a3457)
                                        };
                                        nat_lt(a3458, a3459)
                                    }, f344(f610({
                                        let a3462 = f342(v1.clone(), {
                                            let a3460 = v2;
                                            let a3461 = 2u64;
                                            nat_add(a3460, a3461)?
                                        })?;
                                        let a3463 = 93u64;
                                        nat_eq(a3462, a3463)
                                    }), f610({
                                        let a3466 = f342(v1.clone(), {
                                            let a3464 = v2;
                                            let a3465 = 2u64;
                                            nat_add(a3464, a3465)?
                                        })?;
                                        let a3467 = 92u64;
                                        nat_eq(a3466, a3467)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3468 = v2;
                                        let a3469 = 3u64;
                                        nat_add(a3468, a3469)?
                                    }, List::cons((v5, f342(v1.clone(), {
                                        let a3470 = v2;
                                        let a3471 = 2u64;
                                        nat_add(a3470, a3471)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3472 = v2;
                                        let a3473 = 1u64;
                                        nat_add(a3472, a3473)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(v0: u64) -> u64 {
    let a3478 = v0;
    let a3479 = 42u64;
    if nat_eq(a3478, a3479) {
        1u64
    } else {
        let a3480 = v0;
        let a3481 = 43u64;
        if nat_eq(a3480, a3481) {
            2u64
        } else {
            let a3482 = v0;
            let a3483 = 63u64;
            if nat_eq(a3482, a3483) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f646(p3492: List<(u64, u64)>, v1: u64) -> bool {
    let m3491 = p3492;
    {
        let mut m3491 = m3491;
        loop {
            let step = {
                let v0 = m3491;
                let m3484 = v0;
                match m3484.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3486 = {
                            let (h3485, _) = v2;
                            h3485
                        };
                        let a3487 = v1;
                        if if nat_le(a3486, a3487) {
                            let a3489 = v1;
                            let a3490 = {
                                let (_, h3488) = v2;
                                h3488
                            };
                            nat_le(a3489, a3490)
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
                Ok(next) => { m3491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3493 = v0.clone();
        length_bytes(a3493)
    };
    let a3497 = {
        let a3494 = v1;
        let a3495 = 2u64;
        let a3496 = 0u64;
        nat_rem(a3494, a3495, a3496)
    };
    let a3498 = 0u64;
    if nat_eq(a3497, a3498) {
        let (_, h3499) = {
            let (h3500, _) = f666(Fn24::F667(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3500
        };
        Ok(h3499)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f648(v0: u64) -> bool {
    let a3501 = v0;
    let a3502 = 8u64;
    if nat_eq(a3501, a3502) {
        true
    } else {
        let a3503 = v0;
        let a3504 = 9u64;
        if nat_eq(a3503, a3504) {
            true
        } else {
            let a3505 = v0;
            let a3506 = 10u64;
            if nat_eq(a3505, a3506) {
                true
            } else {
                let a3507 = v0;
                let a3508 = 12u64;
                if nat_eq(a3507, a3508) {
                    true
                } else {
                    let a3509 = v0;
                    let a3510 = 13u64;
                    nat_eq(a3509, a3510)
                }
            }
        }
    }
}

pub fn f649(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3511, _) = v0;
        h3511
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3512) = v0;
        h3512
    };
    let m3513 = v1;
    match m3513.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3514 = v4;
            match m3514.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f668(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f650(v0: Fn23, p3519: u64, p3520: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3518 = (p3519, p3520);
    {
        let mut m3518 = m3518;
        loop {
            let step = {
                let (v1, v2) = m3518;
                let m3515 = v1;
                if m3515 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3515 - 1;
                    let m3517 = {
                        let c3516: Fn23 = v0.clone();
                        c3516.apply(v2.clone())
                    };
                    match m3517 {
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
                Ok(next) => { m3518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f651(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f669(v0)
}

pub fn f652(v0: bool, v1: Adt17) -> R<Option<Bytes>> {
    let m3521 = v1;
    match m3521 {
        Adt17::C0 => {
            Ok(None::<Bytes>)
        }
        Adt17::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt17::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C3(v7, v8) => {
            Ok(Some({
                let a3532 = if f344(v7, {
                    let a3530 = 0u64;
                    let a3531 = v8;
                    nat_lt(a3530, a3531)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3533 = f256(false, v8)?;
                append_bytes(a3532, a3533)
            }))
        }
        Adt17::C4(v3, v4, v5) => {
            let m3524 = f670({
                let a3522 = f8(Str::lit("n"));
                let a3523 = f406(false, v4, v5)?;
                append_bytes(a3522, a3523)
            })?;
            match m3524 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3528 = f8(Str::lit("nMinus"));
                        let a3529 = f35(v6.clone(), 1u64, {
                            let a3526 = {
                                let a3525 = v6;
                                length_bytes(a3525)
                            };
                            let a3527 = 1u64;
                            nat_sub(a3526, a3527)
                        });
                        append_bytes(a3528, a3529)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt17::C5(v2) => {
            f657(v0, v2)
        }
        Adt17::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt17::C8 => {
            Ok(None::<Bytes>)
        }
        Adt17::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt17::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f653(v0: Bytes) -> R<Option<Bytes>> {
    let m3534 = f671(v0)?;
    match m3534 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f672(v1)?))
        }
    }
}

pub fn f654(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3535 = v1;
    match m3535.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3536 = f657(v0, v2)?;
            match m3536 {
                None => {
                    f654(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f654(v0, v3)?))
                }
            }
        }
    }
}

pub fn f655(p3540: List<Adt17>, v1: Bytes) -> bool {
    let m3539 = p3540;
    {
        let mut m3539 = m3539;
        loop {
            let step = {
                let v0 = m3539;
                let m3537 = v0;
                match m3537.uncons() {
                    None => {
                        Err::<List<Adt17>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3538 = f665(v2, v1.clone());
                        if match m3538 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt17>, bool>(v3)
                        } else {
                            Err::<List<Adt17>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f656(v0: Adt17) -> R<Option<Bytes>> {
    let m3541 = f327(f233(v0.clone(), f8(Str::lit("$ref"))));
    match m3541 {
        None => {
            Ok(f327(f233(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f580(v1)
        }
    }
}

pub fn f657(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f670(v1)
    } else {
        f653(v1)
    }
}

pub fn f658(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f673(f558(v0), v1)
}

pub fn f659(v0: bool, v1: Bytes, v2: List<Adt17>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt30, Bytes>> {
    let m3542 = v2;
    match m3542.uncons() {
        None => {
            Ok(Ok::<Adt30, Bytes>(Adt30::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3543 = f665(v5.clone(), v3.clone());
            match m3543 {
                None => {
                    Ok(Err::<Adt30, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3544 = f657(v0, v7.clone())?;
                    match m3544 {
                        None => {
                            Ok(Err::<Adt30, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f41(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt30, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3547 = f587({
                                    let a3545 = v1.clone();
                                    let a3546 = f321(v7)?;
                                    append_bytes(a3545, a3546)
                                }, f511(f460(f233(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f237(f238(f233(v5, f8(Str::lit("required"))))), true)?;
                                match m3547 {
                                    Ok(v10) => {
                                        let m3548 = f659(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3548 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt30, Bytes>(Adt30::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f338(f589(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f589(v12.clone())), f464(f590(v10), f590(v12)))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt30, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt30, Bytes>(v9))
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

pub fn f660(v0: List<Adt17>, v1: Bytes) -> List<Bytes> {
    let m3549 = v0;
    match m3549.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3550 = f665(v2, v1.clone());
            match m3550 {
                None => {
                    f660(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f660(v3, v1))
                }
            }
        }
    }
}

pub fn f661(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3551 = f657(v0, v1)?;
    match m3551 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f28(v2.clone(), f8(Str::lit("mk"))) {
                f556(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f662(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3552 = v1;
    match m3552.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3553 = f661(v0, v2)?;
            match m3553 {
                None => {
                    f662(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f662(v0, v3)?))
                }
            }
        }
    }
}

pub fn f663(v0: Adt17) -> Option<Adt17> {
    let m3554 = f460(f233(v0, f8(Str::lit("patternProperties"))));
    match m3554.uncons() {
        None => {
            None::<Adt17>
        }
        Some((v1, v2)) => {
            let m3555 = v2;
            match m3555.uncons() {
                None => {
                    Some({
                        let (_, h3556) = v1;
                        h3556
                    })
                }
                Some((_, _)) => {
                    None::<Adt17>
                }
            }
        }
    }
}

pub fn f664(v0: Adt17) -> bool {
    f28(f236(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f665(v0: Adt17, v1: Bytes) -> Option<Bytes> {
    if f41(v1.clone(), f237(f238(f233(v0.clone(), f8(Str::lit("required")))))) {
        let m3557 = f233(v0, f8(Str::lit("properties")));
        match m3557 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3558 = f233(v2, v1);
                match m3558 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f327(f233(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f666(v0: Fn24, p3563: u64, p3564: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3562 = (p3563, p3564);
    {
        let mut m3562 = m3562;
        loop {
            let step = {
                let (v1, v2) = m3562;
                let m3559 = v1;
                if m3559 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3559 - 1;
                    let m3561 = {
                        let c3560: Fn24 = v0.clone();
                        c3560.apply(v2.clone())?
                    };
                    match m3561 {
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
                Ok(next) => { m3562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f667(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f674(v0, v1)
}

pub fn f668(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3569 = {
            let a3567 = {
                let a3565 = v0.clone();
                length_list(a3565)
            };
            let a3568 = {
                let a3566 = v1.clone();
                length_list(a3566)
            };
            nat_add(a3567, a3568)?
        };
        let a3570 = 1u64;
        nat_add(a3569, a3570)?
    };
    Ok(f675({
        let (_, h3571) = {
            let (_, h3572) = {
                let (h3573, _) = f676(Fn25::F677, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3573
            };
            h3572
        };
        h3571
    }))
}

pub fn f669(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3574, _) = v0.clone();
        h3574
    };
    let v2: List<(Bytes, ())> = {
        let (h3575, _) = {
            let (_, h3576) = v0.clone();
            h3576
        };
        h3575
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3577) = {
            let (_, h3578) = v0;
            h3578
        };
        h3577
    };
    let m3579 = v1.clone();
    match m3579.uncons() {
        None => {
            let m3580 = v2;
            match m3580.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3581 = v2.clone();
            match m3581.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f678({
                        let (h3582, _) = v8.clone();
                        h3582
                    }, {
                        let (h3583, _) = v6.clone();
                        h3583
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

pub fn f670(v0: Bytes) -> R<Option<Bytes>> {
    let m3585 = f679({
        let a3584 = v0.clone();
        length_bytes(a3584)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3585 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3587 = {
                let a3586 = v1.clone();
                length_bytes(a3586)
            };
            let a3588 = 0u64;
            if nat_eq(a3587, a3588) {
                let a3590 = {
                    let a3589 = v0;
                    length_bytes(a3589)
                };
                let a3591 = 0u64;
                if nat_eq(a3590, a3591) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f672(v1)?))
            }
        }
    }
}

pub fn f671(v0: Bytes) -> R<Option<Bytes>> {
    if f680({
        let a3592 = v0.clone();
        length_bytes(a3592)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f681(v0)?;
        if f682(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3595 = {
                let a3593 = v1.clone();
                let a3594 = 0u64;
                index_bytes(a3593, a3594)
            };
            match m3595 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f458(f62(v2)?) {
                        Ok(Some({
                            let a3596 = f8(Str::lit("n"));
                            let a3597 = v1;
                            append_bytes(a3596, a3597)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f683({
            let a3598 = v0.clone();
            length_bytes(a3598)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f672(v0: Bytes) -> R<Bytes> {
    if f41(v0.clone(), f684()) {
        f556(v0)
    } else {
        Ok(v0)
    }
}

pub fn f673(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3599 = v0.clone();
    match m3599 {
        Adt17::C0 => {
            Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C1(_, _) => {
            Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C2(v4) => {
            if v4 {
                Ok(f634())
            } else {
                Ok(f583(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt17::C3(_, _) => {
            Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C4(_, _, _) => {
            Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C5(_) => {
            Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C6(_) => {
            Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C7(_) => {
            let m3600 = f685(f477(v0.clone()));
            match m3600 {
                None => {
                    let m3601 = f327(f233(v0.clone(), f8(Str::lit("$ref"))));
                    match m3601 {
                        None => {
                            let m3603 = f233(v0.clone(), f8(Str::lit("allOf")));
                            match m3603 {
                                None => {
                                    let m3608 = f233(v0.clone(), f8(Str::lit("enum")));
                                    match m3608 {
                                        None => {
                                            let m3609 = f233(v0.clone(), f8(Str::lit("const")));
                                            match m3609 {
                                                None => {
                                                    let m3611 = f233(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3611 {
                                                        None => {
                                                            let v32: Bytes = f236(v0.clone(), f8(Str::lit("type")));
                                                            if if f28(v32.clone(), Bytes::lit(&[])) {
                                                                !f686(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f634())
                                                            } else {
                                                                if f28(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f584(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f28(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f584(f687(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f28(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f584(f688(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f28(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f584(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f28(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3612 = f460(f233(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3612.uncons() {
                                                                                        None => {
                                                                                            if f591(v0.clone()) {
                                                                                                Ok(f584(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3613 = f629(v0);
                                                                                                match m3613 {
                                                                                                    None => {
                                                                                                        Ok(f632(f8(Str::lit("List")), f689(f634())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f632(f8(Str::lit("List")), f689(f690(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f584(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f583(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f584(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3610 = f327(Some(v29));
                                                    match m3610 {
                                                        None => {
                                                            Ok(f583(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f584(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f584(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3604 = f328(v21);
                                    match m3604.uncons() {
                                        None => {
                                            Ok(f583(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3605 = v23;
                                            match m3605.uncons() {
                                                None => {
                                                    let m3606 = f327(f233(v22, f8(Str::lit("$ref"))));
                                                    match m3606 {
                                                        None => {
                                                            Ok(f583(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3607 = f580(v26)?;
                                                            match m3607 {
                                                                None => {
                                                                    Ok(f583(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f584(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f583(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3602 = f580(v19)?;
                            match m3602 {
                                None => {
                                    Ok(f583(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f584(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f583(if f28(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt17::C8 => {
            Ok(f634())
        }
        Adt17::C9(_, _) => {
            Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt17::C10(_, _, _) => {
            Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f674(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3614, _) = v1;
        h3614
    };
    let m3616 = {
        let (_, h3615) = v1;
        h3615
    };
    match m3616 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3619 = {
                let a3617 = v0.clone();
                let a3618 = v2;
                index_bytes(a3617, a3618)
            };
            match m3619 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3624 = {
                        let a3622 = v0;
                        let a3623 = {
                            let a3620 = v2;
                            let a3621 = 1u64;
                            nat_add(a3620, a3621)?
                        };
                        index_bytes(a3622, a3623)
                    };
                    match m3624 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f691(v4)? {
                                f691(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3625 = v2;
                                    let a3626 = 2u64;
                                    nat_add(a3625, a3626)?
                                }, Some({
                                    let a3631 = v3;
                                    let a3632 = f496(f692(), {
                                        let a3629 = {
                                            let a3627 = f693(v4)?;
                                            let a3628 = 16u64;
                                            nat_mul(a3627, a3628)?
                                        };
                                        let a3630 = f693(v5)?;
                                        nat_add(a3629, a3630)?
                                    });
                                    append_bytes(a3631, a3632)
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

pub fn f675(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f351(Fn4::F694, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f676(v0: Fn25, p3637: u64, p3638: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3636 = (p3637, p3638);
    {
        let mut m3636 = m3636;
        loop {
            let step = {
                let (v1, v2) = m3636;
                let m3633 = v1;
                if m3633 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3633 - 1;
                    let m3635 = {
                        let c3634: Fn25 = v0.clone();
                        c3634.apply(v2.clone())
                    };
                    match m3635 {
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
                Ok(next) => { m3636 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f695(v0)
}

pub fn f678(v0: Bytes, v1: Bytes) -> bool {
    let a3643 = {
        let a3639 = v0;
        let a3640 = v1;
        compare_bytes(a3639, a3640)
    };
    let a3644 = {
        let a3641 = Bytes::lit(&[]);
        let a3642 = Bytes::lit(&[0x01]);
        compare_bytes(a3641, a3642)
    };
    equal(a3643, a3644)
}

pub fn f679(p3657: u64, v1: Bytes, p3658: u64, p3659: Bytes) -> R<Option<Bytes>> {
    let m3656 = (p3657, (p3658, p3659));
    {
        let mut m3656 = m3656;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3656;
                let m3645 = v0;
                if m3645 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3645 - 1;
                    let a3647 = {
                        let a3646 = v1.clone();
                        length_bytes(a3646)
                    };
                    let a3648 = v2;
                    if nat_le(a3647, a3648) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3649 = f609(v1.clone(), v2)?;
                        match m3649 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3651 = v2;
                                    let a3652 = {
                                        let (_, h3650) = v5;
                                        h3650
                                    };
                                    nat_add(a3651, a3652)?
                                }, {
                                    let a3654 = v3;
                                    let a3655 = f696(v1.clone(), v2, {
                                        let (h3653, _) = v5;
                                        h3653
                                    });
                                    append_bytes(a3654, a3655)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f680(p3669: u64, v1: Bytes, p3670: u64) -> R<bool> {
    let m3668 = (p3669, p3670);
    {
        let mut m3668 = m3668;
        loop {
            let step = {
                let (v0, v2) = m3668;
                let m3660 = v0;
                if m3660 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3660 - 1;
                    let m3663 = {
                        let a3661 = v1.clone();
                        let a3662 = v2;
                        index_bytes(a3661, a3662)
                    };
                    match m3663 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3664 = f62(v4)?;
                            let a3665 = 128u64;
                            if nat_lt(a3664, a3665) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3666 = v2;
                                    let a3667 = 1u64;
                                    nat_add(a3666, a3667)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f681(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f402(v0.clone(), 0u64, {
        let a3671 = v0;
        length_bytes(a3671)
    }, false, Bytes::lit(&[]))?;
    let m3674 = {
        let a3672 = v1.clone();
        let a3673 = 0u64;
        index_bytes(a3672, a3673)
    };
    match m3674 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f457(v3) {
                let a3681 = f35({
                    let a3675 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3675)
                }, {
                    let a3676 = v3;
                    let a3677 = 65u64;
                    nat_sub(a3676, a3677)
                }, 1u64);
                let a3682 = f35(v1.clone(), 1u64, {
                    let a3679 = {
                        let a3678 = v1;
                        length_bytes(a3678)
                    };
                    let a3680 = 1u64;
                    nat_sub(a3679, a3680)
                });
                Ok(append_bytes(a3681, a3682))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f682(v0: Bytes) -> R<bool> {
    let m3685 = {
        let a3683 = v0;
        let a3684 = 0u64;
        index_bytes(a3683, a3684)
    };
    match m3685 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f426(f456(f62(v1)?), f457(f62(v1)?)))
        }
    }
}

pub fn f683(p3698: u64, v1: Bytes, p3699: u64, p3700: Bytes) -> R<Option<Bytes>> {
    let m3697 = (p3698, (p3699, p3700));
    {
        let mut m3697 = m3697;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3697;
                let m3686 = v0;
                if m3686 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3686 - 1;
                    let a3688 = {
                        let a3687 = v1.clone();
                        length_bytes(a3687)
                    };
                    let a3689 = v2;
                    if nat_le(a3688, a3689) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3690 = f609(v1.clone(), v2)?;
                        match m3690 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3692 = v2;
                                    let a3693 = {
                                        let (_, h3691) = v5;
                                        h3691
                                    };
                                    nat_add(a3692, a3693)?
                                }, {
                                    let a3695 = v3;
                                    let a3696 = f697({
                                        let (h3694, _) = v5;
                                        h3694
                                    });
                                    append_bytes(a3695, a3696)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3697 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f684() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f685(p3705: List<(Bytes, Adt17)>) -> Option<Bytes> {
    let m3704 = p3705;
    {
        let mut m3704 = m3704;
        loop {
            let step = {
                let v0 = m3704;
                let m3701 = v0;
                match m3701.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt17)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h3702, _) = v1.clone();
                            h3702
                        }, f698()) {
                            Err::<List<(Bytes, Adt17)>, Option<Bytes>>(Some({
                                let (h3703, _) = v1;
                                h3703
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt17)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Adt17) -> bool {
    if f596(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f596(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f596(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f596(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f596(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f596(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f687(v0: Adt17) -> Bytes {
    let m3706 = f530(f233(v0, f8(Str::lit("minimum"))));
    match m3706 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f688() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f689(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3707 = v0;
    match m3707 {
        Ok(v2) => {
            f584(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f517(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f516(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f690(v0: Adt17, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f699(16u64, v0, v1)
}

pub fn f691(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3752 = {
            let a3728 = {
                let a3716 = {
                    let a3710 = {
                        let a3708 = v1;
                        let a3709 = 128u8;
                        fixed_u8::bit_and(a3708, a3709)
                    };
                    let a3711 = 128u8;
                    if equal(a3710, a3711) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3717 = {
                    let a3714 = {
                        let a3712 = v1;
                        let a3713 = 64u8;
                        fixed_u8::bit_and(a3712, a3713)
                    };
                    let a3715 = 64u8;
                    if equal(a3714, a3715) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3716, a3717)?
            };
            let a3729 = {
                let a3726 = {
                    let a3720 = {
                        let a3718 = v1;
                        let a3719 = 32u8;
                        fixed_u8::bit_and(a3718, a3719)
                    };
                    let a3721 = 32u8;
                    if equal(a3720, a3721) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3727 = {
                    let a3724 = {
                        let a3722 = v1;
                        let a3723 = 16u8;
                        fixed_u8::bit_and(a3722, a3723)
                    };
                    let a3725 = 16u8;
                    if equal(a3724, a3725) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3726, a3727)?
            };
            nat_add(a3728, a3729)?
        };
        let a3753 = {
            let a3750 = {
                let a3738 = {
                    let a3732 = {
                        let a3730 = v1;
                        let a3731 = 8u8;
                        fixed_u8::bit_and(a3730, a3731)
                    };
                    let a3733 = 8u8;
                    if equal(a3732, a3733) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3739 = {
                    let a3736 = {
                        let a3734 = v1;
                        let a3735 = 4u8;
                        fixed_u8::bit_and(a3734, a3735)
                    };
                    let a3737 = 4u8;
                    if equal(a3736, a3737) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3738, a3739)?
            };
            let a3751 = {
                let a3748 = {
                    let a3742 = {
                        let a3740 = v1;
                        let a3741 = 2u8;
                        fixed_u8::bit_and(a3740, a3741)
                    };
                    let a3743 = 2u8;
                    if equal(a3742, a3743) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3749 = {
                    let a3746 = {
                        let a3744 = v1;
                        let a3745 = 1u8;
                        fixed_u8::bit_and(a3744, a3745)
                    };
                    let a3747 = 1u8;
                    if equal(a3746, a3747) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3748, a3749)?
            };
            nat_add(a3750, a3751)?
        };
        nat_add(a3752, a3753)?
    };
    let a3754 = 48u64;
    let a3755 = v2;
    if if nat_le(a3754, a3755) {
        let a3756 = v2;
        let a3757 = 57u64;
        nat_le(a3756, a3757)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3758 = 97u64;
        let a3759 = v2;
        if nat_le(a3758, a3759) {
            let a3760 = v2;
            let a3761 = 102u64;
            Ok(nat_le(a3760, a3761))
        } else {
            Ok(false)
        }
    }
}

pub fn f692() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f693(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3806 = {
            let a3782 = {
                let a3770 = {
                    let a3764 = {
                        let a3762 = v1;
                        let a3763 = 128u8;
                        fixed_u8::bit_and(a3762, a3763)
                    };
                    let a3765 = 128u8;
                    if equal(a3764, a3765) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3771 = {
                    let a3768 = {
                        let a3766 = v1;
                        let a3767 = 64u8;
                        fixed_u8::bit_and(a3766, a3767)
                    };
                    let a3769 = 64u8;
                    if equal(a3768, a3769) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3770, a3771)?
            };
            let a3783 = {
                let a3780 = {
                    let a3774 = {
                        let a3772 = v1;
                        let a3773 = 32u8;
                        fixed_u8::bit_and(a3772, a3773)
                    };
                    let a3775 = 32u8;
                    if equal(a3774, a3775) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3781 = {
                    let a3778 = {
                        let a3776 = v1;
                        let a3777 = 16u8;
                        fixed_u8::bit_and(a3776, a3777)
                    };
                    let a3779 = 16u8;
                    if equal(a3778, a3779) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3780, a3781)?
            };
            nat_add(a3782, a3783)?
        };
        let a3807 = {
            let a3804 = {
                let a3792 = {
                    let a3786 = {
                        let a3784 = v1;
                        let a3785 = 8u8;
                        fixed_u8::bit_and(a3784, a3785)
                    };
                    let a3787 = 8u8;
                    if equal(a3786, a3787) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3793 = {
                    let a3790 = {
                        let a3788 = v1;
                        let a3789 = 4u8;
                        fixed_u8::bit_and(a3788, a3789)
                    };
                    let a3791 = 4u8;
                    if equal(a3790, a3791) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3792, a3793)?
            };
            let a3805 = {
                let a3802 = {
                    let a3796 = {
                        let a3794 = v1;
                        let a3795 = 2u8;
                        fixed_u8::bit_and(a3794, a3795)
                    };
                    let a3797 = 2u8;
                    if equal(a3796, a3797) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3803 = {
                    let a3800 = {
                        let a3798 = v1;
                        let a3799 = 1u8;
                        fixed_u8::bit_and(a3798, a3799)
                    };
                    let a3801 = 1u8;
                    if equal(a3800, a3801) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3802, a3803)?
            };
            nat_add(a3804, a3805)?
        };
        nat_add(a3806, a3807)?
    };
    let a3808 = v2;
    let a3809 = 57u64;
    if nat_le(a3808, a3809) {
        let a3810 = v2;
        let a3811 = 48u64;
        Ok(nat_sub(a3810, a3811))
    } else {
        let a3812 = v2;
        let a3813 = 87u64;
        Ok(nat_sub(a3812, a3813))
    }
}

pub fn f694(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f695(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3814, _) = v0.clone();
        h3814
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3815, _) = {
            let (_, h3816) = v0.clone();
            h3816
        };
        h3815
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3817) = {
            let (_, h3818) = v0;
            h3818
        };
        h3817
    };
    let m3819 = v1.clone();
    match m3819.uncons() {
        None => {
            let m3820 = v2;
            match m3820.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3821 = v2.clone();
            match m3821.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f678({
                        let (h3822, _) = v8.clone();
                        h3822
                    }, {
                        let (h3823, _) = v6.clone();
                        h3823
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

pub fn f696(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f426(f344(f426(f456(v2), f457(v2)), f344({
        let a3824 = v2;
        let a3825 = 117u64;
        !nat_eq(a3824, a3825)
    }, {
        let a3826 = v2;
        let a3827 = 85u64;
        !nat_eq(a3826, a3827)
    })), f344(f458(v2), {
        let a3828 = 0u64;
        let a3829 = v1;
        nat_lt(a3828, a3829)
    })) {
        f35(v0, v1, 1u64)
    } else {
        let a3830 = v2;
        let a3831 = 65536u64;
        if nat_lt(a3830, a3831) {
            let a3832 = f8(Str::lit("u"));
            let a3833 = f581(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3832, a3833)
        } else {
            let a3834 = f8(Str::lit("U"));
            let a3835 = f581(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3834, a3835)
        }
    }
}

pub fn f697(v0: u64) -> Bytes {
    let a3836 = f8(Str::lit("u"));
    let a3837 = f581(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3836, a3837)
}

pub fn f698() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f699(v0: u64, v1: Adt17, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3838 = v0;
    if m3838 == 0 {
        Ok(f583(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3838 - 1;
        let m3839 = v1.clone();
        match m3839 {
            Adt17::C0 => {
                Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C1(_, _) => {
                Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C2(v6) => {
                if v6 {
                    Ok(f634())
                } else {
                    Ok(f583(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt17::C3(_, _) => {
                Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C4(_, _, _) => {
                Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C5(_) => {
                Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C6(_) => {
                Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C7(_) => {
                let m3840 = f327(f233(v1.clone(), f8(Str::lit("$ref"))));
                match m3840 {
                    None => {
                        if if f596(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f596(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f596(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f584(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f236(v1.clone(), f8(Str::lit("type")));
                            if if f28(v22.clone(), Bytes::lit(&[])) {
                                !f686(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f634())
                            } else {
                                if f28(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f584(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f28(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f584(f687(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f28(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f584(f688(), List::<Adt25>::nil()))
                                        } else {
                                            if f28(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f584(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f28(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3842 = f233(v1, f8(Str::lit("items")));
                                                    match m3842 {
                                                        None => {
                                                            Ok(f632(f8(Str::lit("List")), f634()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f632(f8(Str::lit("List")), f699(v3, v23, {
                                                                let a3843 = v2;
                                                                let a3844 = f8(Str::lit("Item"));
                                                                append_bytes(a3843, a3844)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f28(v22.clone(), f8(Str::lit("object"))) {
                                                        if f591(v1.clone()) {
                                                            Ok(f584(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3845 = f629(v1);
                                                            match m3845 {
                                                                None => {
                                                                    Ok(f632(f8(Str::lit("List")), f689(f634())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f632(f8(Str::lit("List")), f689(f699(v3, v24, {
                                                                        let a3846 = v2;
                                                                        let a3847 = f8(Str::lit("Value"));
                                                                        append_bytes(a3846, a3847)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f583(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3841 = f580(v20)?;
                        match m3841 {
                            None => {
                                Ok(f583(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f584(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt17::C8 => {
                Ok(f634())
            }
            Adt17::C9(_, _) => {
                Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt17::C10(_, _, _) => {
                Ok(f583(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
