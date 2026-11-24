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
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, Adt14),
    C1(Bytes),
    C2,
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
    F56(u8, Bytes),
    F176(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F56(k0, k1) => f56(*k0, k1.clone(), p0),
            Fn0::F176(k0) => f176(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F154,
    F172,
    F238,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F154 => f154(p0, p1),
            Fn1::F172 => f172(p0, p1),
            Fn1::F238 => f238(p0, p1),
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
    F227(Bytes, u64, Bytes, u64),
    F317(Bytes),
    F318(u64, Bytes),
    F384(Bytes),
    F437(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F227(k0, k1, k2, k3) => f227(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F317(k0) => Ok(f317(k0.clone(), p0)),
            Fn3::F318(k0, k1) => f318(*k0, k1.clone(), p0),
            Fn3::F384(k0) => f384(k0.clone(), p0),
            Fn3::F437(k0) => f437(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F269(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt22)) -> R<(u64, Adt22)> {
        match self {
            Fn4::F269(k0, k1, k2) => f269(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F298,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F298 => f298(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F301,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F301 => f301(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F303,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F303 => f303(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F306,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F306 => f306(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F335(Bytes),
    F396(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F335(k0) => f335(k0.clone(), p0),
            Fn9::F396(k0, k1) => Ok(f396(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F337,
    F490,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F337 => f337(p0, p1),
            Fn10::F490 => f490(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F340,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F340 => f340(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F343,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F343 => f343(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F346,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F346 => f346(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F351,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn14::F351 => f351(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F356(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F356(k0) => f356(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F387(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F387(k0) => f387(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F400,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F400 => f400(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F402,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F402 => f402(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F404,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F404 => f404(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F412,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F412 => f412(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F429,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F429 => f429(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F459,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F459 => f459(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F461,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F461 => f461(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F463(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F463(k0) => f463(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F489,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F489 => f489(p0),
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
            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ")), List::cons(v9, List::cons(f8(Str::lit(" is not\u{a}")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f28(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m177 = v0;
    match m177.uncons() {
        None => {
            Ok(Adt11::C4(f35()))
        }
        Some((v2, v3)) => {
            if f36(v2.clone(), f8(Str::lit("check"))) {
                f37(v1)
            } else {
                if f36(v2.clone(), f8(Str::lit("next"))) {
                    f38(v1, v3)
                } else {
                    if f36(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f39(v1))
                    } else {
                        if f36(v2.clone(), f8(Str::lit("publish"))) {
                            f40(v1)
                        } else {
                            if f36(v2.clone(), f8(Str::lit("agenda"))) {
                                f41(v1)
                            } else {
                                let m178 = v3;
                                match m178.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f35()))
                                    }
                                    Some((v4, v5)) => {
                                        if f36(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f42(v1, v4)
                                        } else {
                                            if f36(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                f43(v1.clone(), v4.clone(), f44(v1, v4)?)
                                            } else {
                                                if f36(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                    f43(v1.clone(), v4.clone(), f45(v1, v4, false)?)
                                                } else {
                                                    if f36(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                        f43(v1.clone(), v4.clone(), f45(v1, v4, true)?)
                                                    } else {
                                                        if f36(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                            f43(v1.clone(), v4.clone(), f46(v1, v4, false)?)
                                                        } else {
                                                            if f36(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                f43(v1.clone(), v4.clone(), f46(v1, v4, true)?)
                                                            } else {
                                                                if f36(v2.clone(), f8(Str::lit("admit"))) {
                                                                    f47(v1, v4)
                                                                } else {
                                                                    if f36(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                        f43(v1.clone(), v4.clone(), f48(v1, v4)?)
                                                                    } else {
                                                                        if f36(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                            f49(v1, v4)
                                                                        } else {
                                                                            if f36(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                f50(v1, v4)
                                                                            } else {
                                                                                if f36(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                    f51(v1, v4)
                                                                                } else {
                                                                                    if f36(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                        f52(v1, v4)
                                                                                    } else {
                                                                                        if f36(v2, f8(Str::lit("propose-encoding"))) {
                                                                                            let m179 = v5;
                                                                                            match m179.uncons() {
                                                                                                None => {
                                                                                                    Ok(Adt11::C4(f35()))
                                                                                                }
                                                                                                Some((v6, v7)) => {
                                                                                                    f53(v1, v4, v6, v7)
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            Ok(Adt11::C4(f35()))
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
        let a226 = f54();
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
        let (h230, _) = f55(Fn0::F56(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f57(List::cons(f32(v0, v4, {
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

pub fn f33(p248: List<Bytes>) -> R<Option<Bytes>> {
    let m247 = p248;
    {
        let mut m247 = m247;
        loop {
            let step = {
                let v0 = m247;
                let m245 = v0;
                match m245.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f58(v1.clone(), 0u64, {
                            let a246 = v1.clone();
                            length_bytes(a246)
                        })? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m249 = v0;
    match m249.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h250, _) = v1;
                h250
            }, f34(v2))
        }
    }
}

pub fn f35() -> Bytes {
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a251 = v0;
    let a252 = v1;
    equal(a251, a252)
}

pub fn f37(v0: Adt1) -> R<Adt11> {
    let m253 = f59(v0.clone(), f60());
    match m253.uncons() {
        None => {
            let m254 = f61(v0.clone(), f60())?;
            match m254.uncons() {
                None => {
                    let m257 = f62(match v0.clone() {
                        Adt1::C0(_, h255, _) => {
                            h255
                        }
                    }, f63(f64(), f65(match v0.clone() {
                        Adt1::C0(_, h256, _) => {
                            h256
                        }
                    }, false)));
                    match m257.uncons() {
                        None => {
                            let m259 = f59(v0.clone(), f65(match v0.clone() {
                                Adt1::C0(_, h258, _) => {
                                    h258
                                }
                            }, true));
                            match m259.uncons() {
                                None => {
                                    let m261 = f66(v0.clone(), f65(match v0.clone() {
                                        Adt1::C0(_, h260, _) => {
                                            h260
                                        }
                                    }, true))?;
                                    match m261 {
                                        None => {
                                            let v10: (bool, Bytes) = f67(Adt13::C0(Adt12::C0(f68(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f70(v0.clone())?, f71(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f68(f69(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f72(v0.clone(), f8(Str::lit("model/strata.json")))?, f73(f72(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f65(match v0 {
                                                Adt1::C0(_, h262, _) => {
                                                    h262
                                                }
                                            }, true)))?;
                                            let (h263, _) = v10;
                                            if h263 {
                                                Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                                                    let (_, h264) = v10;
                                                    h264
                                                }))
                                            } else {
                                                Ok(Adt11::C4({
                                                    let (_, h265) = v10;
                                                    h265
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

pub fn f38(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f74(f75(match v0.clone() {
        Adt1::C0(_, h266, _) => {
            h266
        }
    }, f76())) {
        Ok(Adt11::C1(List::cons(f76(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f77(f78(f76(), f79(match v0.clone() {
            Adt1::C0(_, h267, _) => {
                h267
            }
        }, f76())))?;
        let m268 = f59(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m268.uncons() {
            None => {
                let m269 = f80(v0.clone(), f81(f82(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f83(v2, v1.clone()))?;
                match m269 {
                    None => {
                        f84(v0, v1)
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

pub fn f39(v0: Adt1) -> Adt11 {
    if f74(f85(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h270) => {
            h270
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f86(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f74(f85(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h271) => {
                h271
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f86(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m272 = f87(v0.clone());
    match m272.uncons() {
        None => {
            let m273 = f63(f59(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f88(), List::<Bytes>::nil())))), f89(v0.clone())?);
            match m273.uncons() {
                None => {
                    Ok(Adt11::C3(f90(f91(v0.clone())?, List::cons((f92(), f93()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f94()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f95()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f96()), List::cons((f8(Str::lit("docs/catalog.md")), f97(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f98(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f99(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f100()), List::<(Bytes, Bytes)>::nil()))))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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

pub fn f41(v0: Adt1) -> R<Adt11> {
    let m274 = f87(v0.clone());
    match m274.uncons() {
        None => {
            let m275 = f63(f59(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f89(v0.clone())?);
            match m275.uncons() {
                None => {
                    let v5: Adt15 = f101(v0.clone())?;
                    let v6: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f63(f102(f103(match v5.clone() {
                        Adt15::C0(h276, _, _) => {
                            h276
                        }
                    }, f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f105(v6.clone())), f106(f103(match v5.clone() {
                        Adt15::C0(h277, _, _) => {
                            h277
                        }
                    }, f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f107(v5.clone(), v6, f108(v5, f109(v0.clone(), f110(v0)?)?))?)))))
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

pub fn f42(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m278 = f59(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m278.uncons() {
        None => {
            let v4: Bytes = f69(v0.clone(), v1);
            let v5: List<List<Bytes>> = f111(v4.clone())?;
            if f74(f112(f113(v5.clone()), f114())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f115(Str::lit("id"), v6.clone())?, List::cons(f115(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f115(Str::lit("artifact"), f116())?, List::cons(f115(Str::lit("checksum"), f117(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f116(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f118(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f119(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f120(f121(f122(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f43(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m279 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m279.uncons() {
        None => {
            if f123(f82(f69(v0, v1))?)? {
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

pub fn f44(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m280 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m280.uncons() {
        None => {
            let m281 = f124(f82(f69(v0, v1))?)?;
            match m281 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f120(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f45(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m282 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m282.uncons() {
        None => {
            let v5: Bytes = f125(if v2 {
                f126()
            } else {
                f127()
            }, f128(v1.clone())?);
            if f74(f129(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m283 = f130(f82(f69(v0.clone(), v1.clone()))?, v2)?;
                match m283 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a285 = {
                            let a284 = v6;
                            length_list(a284)
                        };
                        let a286 = 0u64;
                        if nat_eq(a285, a286) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f69(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f46(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m287 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m287.uncons() {
        None => {
            let v5: List<Bytes> = f82(f69(v0.clone(), v1.clone()))?;
            if f74(f129(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f131(v5)?
                } else {
                    f132(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f125(f133(), f128(v1.clone())?), f69(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f125(f133(), f128(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f134()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f47(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m288 = f59(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m288.uncons() {
        None => {
            let v4: List<Bytes> = f82(f69(v0.clone(), v1))?;
            let m289 = f135(v4.clone(), f8(Str::lit("requirement")))?;
            match m289 {
                None => {
                    Ok(Adt11::C4(f134()))
                }
                Some(v5) => {
                    let m290 = f135(v4, f8(Str::lit("admission")))?;
                    match m290 {
                        None => {
                            Ok(Adt11::C4(f134()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f63(f71(v5.clone(), f8(Str::lit("id"))), f71(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f77(f63(f71(v7.clone(), f8(Str::lit("id"))), f71(v8.clone(), f8(Str::lit("construct")))))?;
                            let m291 = f136(f105(v6.clone()), f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m291 {
                                None => {
                                    let a293 = {
                                        let a292 = v9.clone();
                                        length_list(a292)
                                    };
                                    let a294 = 0u64;
                                    if nat_eq(a293, a294) {
                                        Ok(Adt11::C4(f134()))
                                    } else {
                                        if f74({
                                            let a297 = {
                                                let a295 = f77(f63(v9.clone(), v10.clone()))?;
                                                length_list(a295)
                                            };
                                            let a298 = {
                                                let a296 = f63(v9.clone(), v10);
                                                length_list(a296)
                                            };
                                            nat_eq(a297, a298)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f138(f8(Str::lit("model/requirements.json")), f139(v7, v5))?, List::cons(f138(f8(Str::lit("model/admissions.json")), f139(v8, v6))?, List::cons(f119(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f120(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f137(v11))
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

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m299 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m299.uncons() {
        None => {
            let m300 = f140(f82(f69(v0, v1))?)?;
            match m300 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f120(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m301 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m301.uncons() {
        None => {
            let m302 = f141(v0.clone(), v1.clone())?;
            match m302 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m303 = f142(v4.clone(), f8(Str::lit("schema")));
                    match m303 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f143(v0, f144(v5.clone())?, v5, f145(v4.clone(), f8(Str::lit("name"))), f73(f146(f142(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m304 = f59(v0.clone(), List::cons(v1.clone(), List::cons(f88(), List::<Bytes>::nil())));
    match m304.uncons() {
        None => {
            let m305 = f147(f82(f69(v0.clone(), v1))?)?;
            match m305 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f72(v0, f88())?;
                    let v6: List<Bytes> = f71(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f77(f71(v5.clone(), f8(Str::lit("at"))))?;
                    let a307 = {
                        let a306 = v6.clone();
                        length_list(a306)
                    };
                    let a308 = 0u64;
                    if if nat_eq(a307, a308) {
                        true
                    } else {
                        f74({
                            let a311 = {
                                let a309 = f77(f63(v6.clone(), v7.clone()))?;
                                length_list(a309)
                            };
                            let a312 = {
                                let a310 = f63(v6.clone(), v7);
                                length_list(a310)
                            };
                            nat_eq(a311, a312)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f138(f88(), f139(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f120(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m313 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m313.uncons() {
        None => {
            let m314 = f141(v0, v1)?;
            match m314 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f148(f146(Some(v4))) {
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

pub fn f52(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m315 = f59(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m315.uncons() {
        None => {
            let m316 = f141(v0.clone(), v1.clone())?;
            match m316 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f74(f36(f145(v4.clone(), f8(Str::lit("schema"))), f149())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f145(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m317 = f142(v4.clone(), f8(Str::lit("source")));
                            match m317 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f150(f142(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f125(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f125(v9.clone(), f8(Str::lit("binding.json"))), f69(v0.clone(), v1));
                        let m318 = v8;
                        match m318 {
                            None => {
                                f151(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f74(f36(f145(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f151(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f125(v9, f128(f145(v7.clone(), f8(Str::lit("url"))))?);
                                    if f74(f85(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h319) => {
                                            h319
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f145(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m320 = f59(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m320.uncons() {
                                            None => {
                                                if f36(f117(f69(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f151(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f53(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m321 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m321.uncons() {
        None => {
            let m322 = f141(v0.clone(), v1.clone())?;
            match m322 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f143(v0.clone(), f69(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f54() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f55(v0: Fn0, p327: u64, p328: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m326 = (p327, p328);
    {
        let mut m326 = m326;
        loop {
            let step = {
                let (v1, v2) = m326;
                let m323 = v1;
                if m323 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m323 - 1;
                    let m325 = {
                        let c324: Fn0 = v0.clone();
                        c324.apply(v2.clone())?
                    };
                    match m325 {
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
                Ok(next) => { m326 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f152(v1, v0, v2)
}

pub fn f57(v0: List<Bytes>) -> List<Bytes> {
    f153(Fn1::F154, List::<Bytes>::nil(), v0)
}

pub fn f58(v0: Bytes, p384: u64, p385: u64) -> R<bool> {
    let m383 = (p384, p385);
    {
        let mut m383 = m383;
        loop {
            let step = {
                let (v1, v2) = m383;
                let m329 = v2;
                if m329 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m329 - 1;
                    let m332 = {
                        let a330 = v0.clone();
                        let a331 = v1;
                        index_bytes(a330, a331)
                    };
                    match m332 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a379 = {
                                let v5: u8 = v4;
                                let a377 = {
                                    let a353 = {
                                        let a341 = {
                                            let a335 = {
                                                let a333 = v5;
                                                let a334 = 128u8;
                                                fixed_u8::bit_and(a333, a334)
                                            };
                                            let a336 = 128u8;
                                            if equal(a335, a336) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a342 = {
                                            let a339 = {
                                                let a337 = v5;
                                                let a338 = 64u8;
                                                fixed_u8::bit_and(a337, a338)
                                            };
                                            let a340 = 64u8;
                                            if equal(a339, a340) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a341, a342)?
                                    };
                                    let a354 = {
                                        let a351 = {
                                            let a345 = {
                                                let a343 = v5;
                                                let a344 = 32u8;
                                                fixed_u8::bit_and(a343, a344)
                                            };
                                            let a346 = 32u8;
                                            if equal(a345, a346) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a352 = {
                                            let a349 = {
                                                let a347 = v5;
                                                let a348 = 16u8;
                                                fixed_u8::bit_and(a347, a348)
                                            };
                                            let a350 = 16u8;
                                            if equal(a349, a350) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a351, a352)?
                                    };
                                    nat_add(a353, a354)?
                                };
                                let a378 = {
                                    let a375 = {
                                        let a363 = {
                                            let a357 = {
                                                let a355 = v5;
                                                let a356 = 8u8;
                                                fixed_u8::bit_and(a355, a356)
                                            };
                                            let a358 = 8u8;
                                            if equal(a357, a358) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a364 = {
                                            let a361 = {
                                                let a359 = v5;
                                                let a360 = 4u8;
                                                fixed_u8::bit_and(a359, a360)
                                            };
                                            let a362 = 4u8;
                                            if equal(a361, a362) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a363, a364)?
                                    };
                                    let a376 = {
                                        let a373 = {
                                            let a367 = {
                                                let a365 = v5;
                                                let a366 = 2u8;
                                                fixed_u8::bit_and(a365, a366)
                                            };
                                            let a368 = 2u8;
                                            if equal(a367, a368) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a374 = {
                                            let a371 = {
                                                let a369 = v5;
                                                let a370 = 1u8;
                                                fixed_u8::bit_and(a369, a370)
                                            };
                                            let a372 = 1u8;
                                            if equal(a371, a372) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a373, a374)?
                                    };
                                    nat_add(a375, a376)?
                                };
                                nat_add(a377, a378)?
                            };
                            let a380 = 127u64;
                            if nat_le(a379, a380) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a381 = v1;
                                    let a382 = 1u64;
                                    nat_add(a381, a382)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m386 = v1;
    match m386.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m388 = f155(match v0.clone() {
                Adt1::C0(h387, _, _) => {
                    h387
                }
            }, v2.clone());
            match m388 {
                None => {
                    List::cons(v2, f59(v0, v3))
                }
                Some(_) => {
                    f59(v0, v3)
                }
            }
        }
    }
}

pub fn f60() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f116(), List::<Bytes>::nil())))))))
}

pub fn f61(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m389 = v1;
    match m389.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f156(v0.clone(), v2.clone()) {
                f129(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m390 = f141(v0.clone(), v2.clone())?;
                match m390 {
                    None => {
                        Ok(List::cons(v2, f61(v0, v3)?))
                    }
                    Some(_) => {
                        f61(v0, v3)
                    }
                }
            } else {
                f61(v0, v3)
            }
        }
    }
}

pub fn f62(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m391 = v1;
    match m391.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f75(v0.clone(), v2.clone()) {
                f62(v0, v3)
            } else {
                List::cons(v2, f62(v0, v3))
            }
        }
    }
}

pub fn f63(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m392 = v0;
    match m392.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f63(v3, v1))
        }
    }
}

pub fn f64() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f65(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m393 = v0;
    match m393.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f63(f157({
                let (h394, _) = v2;
                h394
            }, {
                let (_, h395) = v2;
                h395
            }, v1), f65(v3, v1))
        }
    }
}

pub fn f66(v0: Adt1, p399: List<Bytes>) -> R<Option<Bytes>> {
    let m398 = p399;
    {
        let mut m398 = m398;
        loop {
            let step = {
                let v1 = m398;
                let m396 = v1;
                match m396.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m397 = f158(f69(v0.clone(), v2.clone()), f159())?;
                        match m397 {
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
                Ok(next) => { m398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h400, _, _, _, _) => {
            h400
        }
    };
    let v2: Adt16 = f160(v1.clone())?;
    let v3: List<Bytes> = f161(v1.clone());
    let v4: List<Bytes> = f71(match v0.clone() {
        Adt13::C0(_, _, h401, _, _) => {
            h401
        }
    }, f8(Str::lit("path")));
    let m403 = f162(v2, List::<u64>::nil(), f163(v1.clone(), match v0.clone() {
        Adt13::C0(_, h402, _, _, _) => {
            h402
        }
    }, 0u64)?);
    match m403 {
        None => {
            if f165(f166(match v0.clone() {
                Adt13::C0(_, _, _, _, h405) => {
                    h405
                }
            }, f63(match v0.clone() {
                Adt13::C0(_, _, _, _, h406) => {
                    h406
                }
            }, v4.clone()))?, f166(v4.clone(), f63(match v0.clone() {
                Adt13::C0(_, _, _, _, h407) => {
                    h407
                }
            }, v4.clone()))?, f167(match v0.clone() {
                Adt13::C0(_, _, h408, _, _) => {
                    h408
                }
            })) {
                let m412 = f168(f166(match v0.clone() {
                    Adt13::C0(_, _, _, h409, _) => {
                        h409
                    }
                }, f63(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h410, _) => {
                        h410
                    }
                }))?, f166(v3.clone(), f63(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h411, _) => {
                        h411
                    }
                }))?);
                match m412 {
                    None => {
                        let m415 = f169(f71(match v0 {
                            Adt13::C0(_, h414, _, _, _) => {
                                h414
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m415 {
                            None => {
                                let m416 = f169(v4, List::<Bytes>::nil());
                                match m416 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f120(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f164(f63(v3, match v0 {
                            Adt13::C0(_, _, _, h413, _) => {
                                h413
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f164(match v1 {
                Adt12::C0(_, _, _, _, h404) => {
                    h404
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f68(v0: Bytes) -> R<List<Bytes>> {
    f170(f82(v0)?, f8(Str::lit("id = \"")))
}

pub fn f69(v0: Adt1, v1: Bytes) -> Bytes {
    let m418 = f155(match v0 {
        Adt1::C0(h417, _, _) => {
            h417
        }
    }, v1);
    match m418 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m419 = v2;
            match m419 {
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

pub fn f70(v0: Adt1) -> R<List<Bytes>> {
    let m420 = f111(f69(v0, f116()))?;
    match m420.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f121(v2))
        }
    }
}

pub fn f71(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m421 = v0;
    match m421.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f145(v2, v1.clone()), f71(v3, v1))
        }
    }
}

pub fn f72(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f146(f141(v0, v1)?))
}

pub fn f73(v0: List<Adt14>) -> List<Bytes> {
    let m422 = v0;
    match m422.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m423 = f150(Some(v1));
            match m423 {
                None => {
                    f73(v2)
                }
                Some(v3) => {
                    List::cons(v3, f73(v2))
                }
            }
        }
    }
}

pub fn f74(v0: bool) -> bool {
    !v0
}

pub fn f75(p427: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m426 = p427;
    {
        let mut m426 = m426;
        loop {
            let step = {
                let v0 = m426;
                let m424 = v0;
                match m424.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h425, _) = v2;
                            h425
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f77(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f171(f153(Fn1::F172, List::<Bytes>::nil(), f173(v0)?)))
}

pub fn f78(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m428 = v1;
    match m428.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f129(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f125(v0.clone(), v2), f78(v0, v3))
            } else {
                f78(v0, v3)
            }
        }
    }
}

pub fn f79(p433: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m432 = p433;
    {
        let mut m432 = m432;
        loop {
            let step = {
                let v0 = m432;
                let m429 = v0;
                match m429.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h430, _) = v2;
                            h430
                        }, v1.clone()) {
                            let (_, h431) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h431)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f80(v0: Adt1, v1: List<List<Bytes>>, p436: List<Bytes>) -> R<Option<Bytes>> {
    let m435 = p436;
    {
        let mut m435 = m435;
        loop {
            let step = {
                let v2 = m435;
                let m434 = v2;
                match m434.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f174(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(p439: List<Bytes>, p440: List<Bytes>, p441: List<List<Bytes>>) -> List<List<Bytes>> {
    let m438 = (p439, (p440, p441));
    {
        let mut m438 = m438;
        loop {
            let step = {
                let (v0, (v1, v2)) = m438;
                let m437 = v0;
                match m437.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f175(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f175(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a442 = v0.clone();
        length_bytes(a442)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h443, _) = f55(Fn0::F176(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h443
    };
    let v3: u64 = {
        let (h444, _) = {
            let (_, h445) = v2.clone();
            h445
        };
        h444
    };
    let v4: List<Bytes> = {
        let (_, h446) = {
            let (_, h447) = v2;
            h447
        };
        h446
    };
    let a448 = v3;
    let a449 = v1;
    if nat_lt(a448, a449) {
        Ok(f57(List::cons(f32(v0, v3, {
            let a450 = v1;
            let a451 = v3;
            nat_sub(a450, a451)
        }), v4)))
    } else {
        Ok(f57(v4))
    }
}

pub fn f83(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m452 = v0;
    match m452.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f83(v3, v1)
            } else {
                List::cons(v2, f83(v3, v1))
            }
        }
    }
}

pub fn f84(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f74(f75(match v0.clone() {
        Adt1::C0(_, h453, _) => {
            h453
        }
    }, f177())) {
        Ok(Adt11::C1(List::cons(f177(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f83(f77(f78(f177(), f79(match v0.clone() {
            Adt1::C0(_, h454, _) => {
                h454
            }
        }, f177())))?, v1.clone());
        let m455 = f59(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m455.uncons() {
            None => {
                let m456 = f178(v0.clone(), v2, f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m456 {
                    None => {
                        f179(v0, v1)
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

pub fn f85(v0: Bytes, p459: List<Bytes>) -> bool {
    let m458 = p459;
    {
        let mut m458 = m458;
        loop {
            let step = {
                let v1 = m458;
                let m457 = v1;
                match m457.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f87(v0: Adt1) -> List<Bytes> {
    if f75(match v0.clone() {
        Adt1::C0(_, h460, _) => {
            h460
        }
    }, f127()) {
        if f75(match v0 {
            Adt1::C0(_, h461, _) => {
                h461
            }
        }, f126()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f126(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f127(), List::<Bytes>::nil())
    }
}

pub fn f88() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f89(v0: Adt1) -> R<List<Bytes>> {
    Ok(f59(v0.clone(), List::cons(f116(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f110(v0)?)))))
}

pub fn f90(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m462 = v0;
    match m462.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f90(v3, v1))
        }
    }
}

pub fn f91(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m463 = f72(v0.clone(), f88())?;
    match m463.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f180(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f92() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f93() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f94() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/encodings/**', 'model/queue/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | grep '^  name: ' | tail -n 1; done < target/hunks.txt\u{a}              git diff -U0 -- studio/src | grep '^[+]  name: ' | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f95() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src and\u{a}# README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one.\u{a}name: refresh\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  refresh:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply each change to the new main, export it again, and push it to its own pull request\u{a}        if: steps.stale.outputs.stale != ''\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ steps.stale.outputs.stale }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again either way.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /'\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}"))
}

pub fn f96() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f97(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f101(v0.clone())?;
    let v2: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f108(v1.clone(), f109(v0.clone(), f110(v0.clone())?)?);
    let v4: List<List<Bytes>> = f122(f111(f69(v0, f116()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f181(v4.clone(), f121(v4), f182(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f107(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f183(f101(v0.clone())?, f109(v0.clone(), f110(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f99(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f184(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f185(f72(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f100() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f101(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f70(v0.clone())?, f68(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f71(f72(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f102(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m464 = v0;
    match m464.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f102(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f102(v3, v1))
            }
        }
    }
}

pub fn f103(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m465 = v0;
    match m465.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f103(v3, v1)
            } else {
                List::cons(v2, f103(v3, v1))
            }
        }
    }
}

pub fn f104(v0: List<Adt14>) -> List<Bytes> {
    let m466 = v0;
    match m466.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f63(f73(f146(f142(v1, f8(Str::lit("defers"))))), f104(v2))
        }
    }
}

pub fn f105(v0: List<Adt14>) -> List<Bytes> {
    let m467 = v0;
    match m467.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m468 = f142(v1, f8(Str::lit("std")));
            match m468 {
                None => {
                    f105(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3, f8(Str::lit("row"))), f105(v2))
                }
            }
        }
    }
}

pub fn f106(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m469 = v0;
    match m469.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f106(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f106(v3, v1))
            }
        }
    }
}

pub fn f107(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f90(f186(v2.clone()), f187(f182(v0, v1.clone(), v2)?));
    f188(f77(f189(v1))?, v3.clone(), {
        let a471 = {
            let a470 = v3;
            length_list(a470)
        };
        let a472 = 1u64;
        nat_add(a471, a472)?
    })
}

pub fn f108(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m473 = v1;
    match m473.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f190(v2.clone(), f8(Str::lit("subject"))) {
                f191(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f108(v0, v3))
            } else {
                f108(v0, v3)
            }
        }
    }
}

pub fn f109(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m474 = v1;
    match m474.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m475 = f130(f82(f69(v0.clone(), v2.clone()))?, f192(v2, f126()))?;
            match m475 {
                None => {
                    f109(v0, v3)
                }
                Some(v4) => {
                    Ok(f139(v4, f109(v0, v3)?))
                }
            }
        }
    }
}

pub fn f110(v0: Adt1) -> R<List<Bytes>> {
    Ok(f63(f77(f193(f127(), f79(match v0.clone() {
        Adt1::C0(_, h476, _) => {
            h476
        }
    }, f127())))?, f77(f193(f126(), f79(match v0 {
        Adt1::C0(_, h477, _) => {
            h477
        }
    }, f126())))?))
}

pub fn f111(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f194(v0.clone(), 0u64, {
        let a478 = v0;
        length_bytes(a478)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a481 = {
            let a480 = match v1.clone() {
                Adt17::C0(_, _, h479, _, _) => {
                    h479
                }
            };
            length_bytes(a480)
        };
        let a482 = 0u64;
        if if nat_eq(a481, a482) {
            let m484 = match v1.clone() {
                Adt17::C0(_, _, _, h483, _) => {
                    h483
                }
            };
            match m484.uncons() {
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
            f195(v1)
        }
    };
    Ok(f196(match v4 {
        Adt17::C0(_, _, _, _, h485) => {
            h485
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f112(p490: List<Bytes>, p491: List<Bytes>) -> bool {
    let m489 = (p490, p491);
    {
        let mut m489 = m489;
        loop {
            let step = {
                let (v0, v1) = m489;
                let m486 = v0;
                match m486.uncons() {
                    None => {
                        let m487 = v1;
                        match m487.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m488 = v1;
                        match m488.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                if f36(v4, v6) {
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
                Ok(next) => { m489 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: List<List<Bytes>>) -> List<Bytes> {
    let m492 = v0;
    match m492.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f114() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f115(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f197(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f116() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f117(v0: Bytes) -> R<Bytes> {
    Ok(f198(f199(v0)?))
}

pub fn f118(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f200(f81(f82(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f201(f202(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f119(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f144(Adt14::C3(std::rc::Rc::new(f203(f77(f63(v1, f73(f72(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f120(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m493 = f204(Fn2::F205(v1), None::<Bytes>, v0);
    match m493 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f121(v0: List<List<Bytes>>) -> List<Bytes> {
    let m494 = v0;
    match m494.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m495 = v1;
            match m495.uncons() {
                None => {
                    f121(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f121(v2))
                }
            }
        }
    }
}

pub fn f122(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m496 = v0;
    match m496.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f123(p502: List<Bytes>) -> R<bool> {
    let m501 = p502;
    {
        let mut m501 = m501;
        loop {
            let step = {
                let v0 = m501;
                let m497 = v0;
                match m497.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a499 = {
                            let a498 = f206(v1)?;
                            length_bytes(a498)
                        };
                        let a500 = 0u64;
                        if nat_eq(a499, a500) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m503 = v0;
    match m503.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f206(v1)?;
            let a505 = {
                let a504 = v3.clone();
                length_bytes(a504)
            };
            let a506 = 0u64;
            if nat_eq(a505, a506) {
                f124(v2)
            } else {
                let m507 = f207(v3.clone())?;
                match m507 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f145(v4.clone(), f8(Str::lit("id")));
                        if if f36(f145(v4, f8(Str::lit("schema"))), f149()) {
                            f74({
                                let a509 = {
                                    let a508 = v5.clone();
                                    length_bytes(a508)
                                };
                                let a510 = 0u64;
                                nat_eq(a509, a510)
                            })
                        } else {
                            false
                        } {
                            let m511 = f124(v2)?;
                            match m511 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f208(v5), {
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

pub fn f125(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f126() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f127() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f128(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f129(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a514 = v0.clone();
        length_bytes(a514)
    };
    let v3: u64 = {
        let a515 = v1.clone();
        length_bytes(a515)
    };
    let a516 = v3;
    let a517 = v2;
    if nat_le(a516, a517) {
        let m523 = {
            let a520 = v0;
            let a521 = {
                let a518 = v2;
                let a519 = v3;
                nat_sub(a518, a519)
            };
            let a522 = v3;
            slice_bytes(a520, a521, a522)
        };
        match m523 {
            None => {
                false
            }
            Some(v4) => {
                let a524 = v4;
                let a525 = v1;
                equal(a524, a525)
            }
        }
    } else {
        false
    }
}

pub fn f130(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m526 = v0;
    match m526.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f206(v2)?;
            let a528 = {
                let a527 = v4.clone();
                length_bytes(a527)
            };
            let a529 = 0u64;
            if nat_eq(a528, a529) {
                f130(v3, v1)
            } else {
                let m530 = f207(v4)?;
                match m530 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f209(v5.clone())
                        } else {
                            f210(v5.clone())
                        } {
                            let m531 = f130(v3, v1)?;
                            match m531 {
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

pub fn f131(v0: List<Bytes>) -> R<bool> {
    if f211(f147(v0.clone())?) {
        Ok(f74({
            let a533 = {
                let a532 = f212(f147(v0)?);
                length_list(a532)
            };
            let a534 = 0u64;
            nat_eq(a533, a534)
        }))
    } else {
        Ok(false)
    }
}

pub fn f132(v0: List<Bytes>) -> R<bool> {
    if f211(f135(v0.clone(), f8(Str::lit("requirement")))?) {
        if f211(f135(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f74({
                let a536 = {
                    let a535 = f139(f212(f135(v0.clone(), f8(Str::lit("requirement")))?), f212(f135(v0, f8(Str::lit("admission")))?));
                    length_list(a535)
                };
                let a537 = 0u64;
                nat_eq(a536, a537)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f133() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f134() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f135(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m538 = v0;
    match m538.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f206(v2)?;
            let a540 = {
                let a539 = v4.clone();
                length_bytes(a539)
            };
            let a541 = 0u64;
            if nat_eq(a540, a541) {
                f135(v3, v1)
            } else {
                let m542 = f207(v4)?;
                match m542 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f213(v5.clone());
                        let a544 = {
                            let a543 = v6.clone();
                            length_bytes(a543)
                        };
                        let a545 = 0u64;
                        if nat_eq(a544, a545) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m546 = f135(v3, v1.clone())?;
                            match m546 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f36(v6, v1) {
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

pub fn f136(p549: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m548 = p549;
    {
        let mut m548 = m548;
        loop {
            let step = {
                let v0 = m548;
                let m547 = v0;
                match m547.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f85(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
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

pub fn f137(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f138(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f144(Adt14::C3(std::rc::Rc::new(f214(v1))))?))
}

pub fn f139(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m550 = v0;
    match m550.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f139(v3, v1))
        }
    }
}

pub fn f140(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m551 = v0;
    match m551.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f206(v1)?;
            let a553 = {
                let a552 = v3.clone();
                length_bytes(a552)
            };
            let a554 = 0u64;
            if nat_eq(a553, a554) {
                f140(v2)
            } else {
                let m555 = f207(v3.clone())?;
                match m555 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f215(v4.clone()) {
                            let m556 = f140(v2)?;
                            match m556 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f177(), List::cons(f8(Str::lit("/")), List::cons(f216(f145(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a557 = v3;
                                        let a558 = Bytes::lit(&[0x0a]);
                                        append_bytes(a557, a558)
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

pub fn f141(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f207(f69(v0, v1))
}

pub fn f142(p564: Adt14, v1: Bytes) -> Option<Adt14> {
    let m563 = p564;
    {
        let mut m563 = m563;
        loop {
            let step = {
                let v0 = m563;
                let m559 = v0;
                match m559 {
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
                    Adt14::C4(r560) => {
                        let v5 = (*r560).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r561, r562) => {
                        let v3 = (*r561).clone();
                        let v4 = (*r562).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m565 = f59(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m565.uncons() {
        None => {
            let m567 = f136(f105(List::cons(Adt14::C4(std::rc::Rc::new({
                let m566 = f217(v4.clone());
                match m566 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m567 {
                None => {
                    f218(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f137(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f144(v0: Adt14) -> R<Bytes> {
    let a568 = f219(v0)?;
    let a569 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a568, a569))
}

pub fn f145(v0: Adt14, v1: Bytes) -> Bytes {
    let m570 = f150(f142(v0, v1));
    match m570 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f146(v0: Option<Adt14>) -> List<Adt14> {
    let m571 = v0;
    match m571 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f220(v1)
        }
    }
}

pub fn f147(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m572 = v0;
    match m572.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f206(v1)?;
            let a574 = {
                let a573 = v3.clone();
                length_bytes(a573)
            };
            let a575 = 0u64;
            if nat_eq(a574, a575) {
                f147(v2)
            } else {
                let m576 = f207(v3)?;
                match m576 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f190(v4.clone(), f8(Str::lit("agent"))) {
                            if f190(v4.clone(), f8(Str::lit("at"))) {
                                if f190(v4.clone(), f8(Str::lit("quote"))) {
                                    f221(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m577 = f147(v2)?;
                            match m577 {
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

pub fn f148(p580: List<Adt14>) -> bool {
    let m579 = p580;
    {
        let mut m579 = m579;
        loop {
            let step = {
                let v0 = m579;
                let m578 = v0;
                match m578.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f145(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f85(f8(Str::lit("studio")), f71(f222(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f150(v0: Option<Adt14>) -> Option<Bytes> {
    let m581 = v0;
    match m581 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m582 = v1;
            match m582 {
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

pub fn f151(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f118(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f223(v1, v4)?)?), List::cons(f119(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f152(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h583, _) = v2.clone();
        h583
    };
    let v4: u64 = {
        let (h584, _) = {
            let (_, h585) = v2.clone();
            h585
        };
        h584
    };
    let v5: List<Bytes> = {
        let (_, h586) = {
            let (_, h587) = v2;
            h587
        };
        h586
    };
    let m588 = f224(v0.clone(), v3);
    match m588 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a589 = v6;
            let a590 = v1;
            if equal(a589, a590) {
                Ok(Some(({
                    let a591 = v3;
                    let a592 = 1u64;
                    nat_add(a591, a592)?
                }, ({
                    let a593 = v3;
                    let a594 = 1u64;
                    nat_add(a593, a594)?
                }, List::cons(f32(v0, v4, {
                    let a595 = v3;
                    let a596 = v4;
                    nat_sub(a595, a596)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a597 = v3;
                    let a598 = 1u64;
                    nat_add(a597, a598)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f153(v0: Fn1, p602: List<Bytes>, p603: List<Bytes>) -> List<Bytes> {
    let m601 = (p602, p603);
    {
        let mut m601 = m601;
        loop {
            let step = {
                let (v1, v2) = m601;
                let m599 = v2;
                match m599.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c600: Fn1 = v0.clone();
                            c600.apply(v1, v3)
                        }, v4))
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

pub fn f154(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f155(p608: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m607 = p608;
    {
        let mut m607 = m607;
        loop {
            let step = {
                let v0 = m607;
                let m604 = v0;
                match m604.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h605, _) = v2;
                            h605
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h606) = v2;
                                h606
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Adt1, v1: Bytes) -> bool {
    let m610 = f155(match v0 {
        Adt1::C0(h609, _, _) => {
            h609
        }
    }, v1);
    match m610 {
        None => {
            false
        }
        Some(v2) => {
            let m611 = v2;
            match m611 {
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

pub fn f157(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m612 = v1;
    match m612.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f225(v3.clone())
            } else {
                f74(f225(v3.clone()))
            } {
                List::cons(f125(v0.clone(), v3), f157(v0, v4, v2))
            } else {
                f157(v0, v4, v2)
            }
        }
    }
}

pub fn f158(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a613 = v0.clone();
        length_bytes(a613)
    };
    let v3: u64 = {
        let a614 = v1.clone();
        length_bytes(a614)
    };
    let v4: u64 = {
        let (h615, _) = f226(Fn3::F227(v1.clone(), v3, v0.clone(), v2), {
            let a616 = v2;
            let a617 = 1u64;
            nat_add(a616, a617)?
        }, 0u64)?;
        h615
    };
    if f228(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f159() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f160(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f229(match v0.clone() {
        Adt12::C0(h618, _, _, _, _) => {
            h618
        }
    }, 0u64)?, f229(match v0.clone() {
        Adt12::C0(_, h619, _, _, _) => {
            h619
        }
    }, 0u64)?, f229(match v0.clone() {
        Adt12::C0(_, _, h620, _, _) => {
            h620
        }
    }, 0u64)?, f229(match v0 {
        Adt12::C0(_, _, _, h621, _) => {
            h621
        }
    }, 0u64)?))
}

pub fn f161(v0: Adt12) -> List<Bytes> {
    f63(match v0.clone() {
        Adt12::C0(h622, _, _, _, _) => {
            h622
        }
    }, f63(match v0.clone() {
        Adt12::C0(_, _, h623, _, _) => {
            h623
        }
    }, f63(match v0.clone() {
        Adt12::C0(_, _, _, h624, _) => {
            h624
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h625) => {
            h625
        }
    })))
}

pub fn f162(v0: Adt16, p631: List<u64>, p632: List<Adt18>) -> Option<u64> {
    let m630 = (p631, p632);
    {
        let mut m630 = m630;
        loop {
            let step = {
                let (v1, v2) = m630;
                let m626 = v2;
                match m626.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f230(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h627) => {
                                h627
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h628, _) => {
                                    h628
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h629, _) => {
                                    h629
                                }
                            }))
                        }
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

pub fn f163(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m633 = v1;
    match m633.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f231(v0.clone(), v3)?), f163(v0, v4, {
                let a634 = v2;
                let a635 = 1u64;
                nat_add(a634, a635)?
            })?))
        }
    }
}

pub fn f164(p639: List<Bytes>, p640: u64) -> Bytes {
    let m638 = (p639, p640);
    {
        let mut m638 = m638;
        loop {
            let step = {
                let (v0, v1) = m638;
                let m636 = v0;
                match m636.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m637 = v1;
                        if m637 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m637 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f232(v0.clone(), v1.clone()) {
        if f232(v1, v0) {
            f233(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f166(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m641 = v0;
    match m641.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f234(v2, v1.clone())?, f166(v3, v1)?))
        }
    }
}

pub fn f167(v0: List<Adt14>) -> List<u64> {
    let m642 = v0;
    match m642.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f235(v1), f167(v2))
        }
    }
}

pub fn f168(p645: List<u64>, v1: List<u64>) -> Option<u64> {
    let m644 = p645;
    {
        let mut m644 = m644;
        loop {
            let step = {
                let v0 = m644;
                let m643 = v0;
                match m643.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f236(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
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

pub fn f169(p648: List<Bytes>, p649: List<Bytes>) -> Option<Bytes> {
    let m647 = (p648, p649);
    {
        let mut m647 = m647;
        loop {
            let step = {
                let (v0, v1) = m647;
                let m646 = v0;
                match m646.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f85(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m650 = v0;
    match m650.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m651 = f237(v2, v1.clone())?;
            match m651 {
                None => {
                    f170(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f170(v3, v1)?))
                }
            }
        }
    }
}

pub fn f171(v0: List<Bytes>) -> List<Bytes> {
    f153(Fn1::F238, List::<Bytes>::nil(), v0)
}

pub fn f172(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f239(v0, v1)
}

pub fn f173(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f240(f241(f242(v0))?))
}

pub fn f174(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m652 = f141(v0, v2)?;
    match m652 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f243(v1, f145(v3.clone(), f8(Str::lit("id"))))?;
            let m653 = v4.clone();
            match m653.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m655 = f150(f142({
                        let m654 = f142(v3.clone(), f8(Str::lit("source")));
                        match m654 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m655 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f74(f85(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f197(f244(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f175(p658: List<Bytes>, p659: List<Bytes>) -> List<Bytes> {
    let m657 = (p658, p659);
    {
        let mut m657 = m657;
        loop {
            let step = {
                let (v0, v1) = m657;
                let m656 = v0;
                match m656.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f245(v0, v1)
}

pub fn f177() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f178(v0: Adt1, p663: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m662 = p663;
    {
        let mut m662 = m662;
        loop {
            let step = {
                let v1 = m662;
                let m660 = v1;
                match m660.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f85(f246(f145({
                            let m661 = f141(v0.clone(), v3.clone())?;
                            match m661 {
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
                Ok(next) => { m662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f74(f75(match v0.clone() {
        Adt1::C0(_, h664, _) => {
            h664
        }
    }, f133())) {
        Ok(Adt11::C1(List::cons(f133(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f83(f77(f247(f79(match v0.clone() {
            Adt1::C0(_, h665, _) => {
                h665
            }
        }, f133())))?, v1);
        let m666 = f59(v0.clone(), List::cons(f88(), List::cons(f116(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m666.uncons() {
            None => {
                let v5: (bool, Bytes) = f248(v0.clone(), v2)?;
                let (h667, _) = v5;
                if h667 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h668) = v5;
                        h668
                    }))
                } else {
                    let m669 = f87(v0.clone());
                    match m669.uncons() {
                        None => {
                            let m670 = f89(v0.clone())?;
                            match m670.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h671) = v5;
                                        h671
                                    }, List::cons(f7(f77(f249(f101(v0.clone())?, f109(v0.clone(), f110(v0.clone())?)?))?), List::cons(f250(v0)?, List::<Bytes>::nil()))))))
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

pub fn f180(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f251(f72(v0, f88())?)?)))
}

pub fn f181(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt21>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m672 = v1;
    match m672.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f63(f252(v5.clone(), v2.clone()), f253(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f254(v0.clone(), v5.clone()), List::cons(if f85(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a674 = {
                    let a673 = v7.clone();
                    length_list(a673)
                };
                let a675 = 0u64;
                if nat_eq(a674, a675) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f120(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f181(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f182(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt21>> {
    let v3: List<(Bytes, Bytes)> = f255(v1);
    let v4: List<(Bytes, Bytes)> = f256(v0, v2);
    f257(v3.clone(), f90(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f183(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m676 = v1;
    match m676.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f190(v2.clone(), f8(Str::lit("domain"))) {
                f258(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f145(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f145(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m677 = f142(v2.clone(), f8(Str::lit("role")));
                    match m677 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f145(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f145(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f259(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m678 = f142(v2.clone(), f8(Str::lit("origin")));
                    match m678 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f145(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m679 = f142(v2, f8(Str::lit("evidence")));
                    match m679 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f145(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f183(v0, v3))
            } else {
                f183(v0, v3)
            }
        }
    }
}

pub fn f184(v0: List<Adt14>) -> List<Bytes> {
    let m680 = v0;
    match m680.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f145(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f145(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f145(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f184(v2))
        }
    }
}

pub fn f185(v0: List<Adt14>) -> List<Bytes> {
    let m681 = v0;
    match m681.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f145(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f260(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f185(v2))
        }
    }
}

pub fn f186(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m682 = v0;
    match m682.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f145(v1.clone(), f8(Str::lit("subject"))), f145(v1, f8(Str::lit("object")))), f186(v2))
        }
    }
}

pub fn f187(v0: List<Adt21>) -> List<(Bytes, Bytes)> {
    let m683 = v0;
    match m683.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt21::C0(h684, _, _) => {
                    h684
                }
            }, match v1 {
                Adt21::C0(_, _, h685) => {
                    h685
                }
            }), f187(v2))
        }
    }
}

pub fn f188(p692: List<Bytes>, v1: List<(Bytes, Bytes)>, p693: u64) -> R<List<Bytes>> {
    let m691 = (p692, p693);
    {
        let mut m691 = m691;
        loop {
            let step = {
                let (v0, v2) = m691;
                let m686 = v2;
                if m686 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m686 - 1;
                    let v4: List<Bytes> = f77(f63(v0.clone(), f261(v0.clone(), v1.clone())))?;
                    let a689 = {
                        let a687 = v4.clone();
                        length_list(a687)
                    };
                    let a690 = {
                        let a688 = v0.clone();
                        length_list(a688)
                    };
                    if nat_eq(a689, a690) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: List<Adt14>) -> List<Bytes> {
    let m694 = v0;
    match m694.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m695 = f142(v1, f8(Str::lit("std")));
            match m695 {
                None => {
                    f189(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3.clone(), f8(Str::lit("authority"))), if f190(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f145(v3, f8(Str::lit("row"))), f189(v2))
                    } else {
                        f189(v2)
                    })
                }
            }
        }
    }
}

pub fn f190(v0: Adt14, v1: Bytes) -> bool {
    f74({
        let a697 = {
            let a696 = f145(v0, v1);
            length_bytes(a696)
        };
        let a698 = 0u64;
        nat_eq(a697, a698)
    })
}

pub fn f191(v0: Adt15, v1: Adt14) -> bool {
    if f262(v0.clone(), v1.clone(), f145(v1.clone(), f8(Str::lit("subject")))) {
        f262(v0, v1.clone(), f145(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f192(v0: Bytes, v1: Bytes) -> bool {
    let m703 = {
        let a700 = v0;
        let a701 = 0u64;
        let a702 = {
            let a699 = v1.clone();
            length_bytes(a699)
        };
        slice_bytes(a700, a701, a702)
    };
    match m703 {
        None => {
            false
        }
        Some(v2) => {
            let a704 = v2;
            let a705 = v1;
            equal(a704, a705)
        }
    }
}

pub fn f193(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m706 = v1;
    match m706.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f129(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f125(v0.clone(), v2), f193(v0, v3))
            } else {
                f193(v0, v3)
            }
        }
    }
}

pub fn f194(v0: Bytes, p764: u64, p765: u64, p766: Adt17) -> R<Adt17> {
    let m763 = (p764, (p765, p766));
    {
        let mut m763 = m763;
        loop {
            let step = {
                let (v1, (v2, v3)) = m763;
                let m707 = v2;
                if m707 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m707 - 1;
                    let v5: u64 = f263(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h708, _, _, _) => {
                            h708
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a709 = v1;
                            let a710 = 1u64;
                            nat_add(a709, a710)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h711, _, _) => {
                                h711
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h712, _) => {
                                h712
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h713) => {
                                h713
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h714, _, _, _, _) => {
                                h714
                            }
                        } {
                            let a715 = v5;
                            let a716 = 34u64;
                            if nat_eq(a715, a716) {
                                let a719 = f263(v0.clone(), {
                                    let a717 = v1;
                                    let a718 = 1u64;
                                    nat_add(a717, a718)?
                                })?;
                                let a720 = 34u64;
                                if nat_eq(a719, a720) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a721 = v1;
                                        let a722 = 1u64;
                                        nat_add(a721, a722)?
                                    }, (v4, Adt17::C0(true, true, f264(match v3.clone() {
                                        Adt17::C0(_, _, h723, _, _) => {
                                            h723
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h724, _) => {
                                            h724
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h725) => {
                                            h725
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a726 = v1;
                                        let a727 = 1u64;
                                        nat_add(a726, a727)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h728, _, _) => {
                                            h728
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h729, _) => {
                                            h729
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h730) => {
                                            h730
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a731 = v1;
                                    let a732 = 1u64;
                                    nat_add(a731, a732)?
                                }, (v4, Adt17::C0(true, false, f264(match v3.clone() {
                                    Adt17::C0(_, _, h733, _, _) => {
                                        h733
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h734, _) => {
                                        h734
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h735) => {
                                        h735
                                    }
                                }))))
                            }
                        } else {
                            let a736 = v5;
                            let a737 = 34u64;
                            if nat_eq(a736, a737) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a738 = v1;
                                    let a739 = 1u64;
                                    nat_add(a738, a739)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h740, _, _) => {
                                        h740
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h741, _) => {
                                        h741
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h742) => {
                                        h742
                                    }
                                }))))
                            } else {
                                let a743 = v5;
                                let a744 = 44u64;
                                if nat_eq(a743, a744) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a745 = v1;
                                        let a746 = 1u64;
                                        nat_add(a745, a746)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h747, _, _) => {
                                            h747
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h748, _) => {
                                            h748
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h749) => {
                                            h749
                                        }
                                    }))))
                                } else {
                                    let a750 = v5;
                                    let a751 = 13u64;
                                    if nat_eq(a750, a751) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a752 = v1;
                                            let a753 = 1u64;
                                            nat_add(a752, a753)?
                                        }, (v4, v3)))
                                    } else {
                                        let a754 = v5;
                                        let a755 = 10u64;
                                        if nat_eq(a754, a755) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a756 = v1;
                                                let a757 = 1u64;
                                                nat_add(a756, a757)?
                                            }, (v4, f195(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a758 = v1;
                                                let a759 = 1u64;
                                                nat_add(a758, a759)?
                                            }, (v4, Adt17::C0(false, false, f264(match v3.clone() {
                                                Adt17::C0(_, _, h760, _, _) => {
                                                    h760
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h761, _) => {
                                                    h761
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h762) => {
                                                    h762
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
                Ok(next) => { m763 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f265(List::cons(match v0.clone() {
        Adt17::C0(_, _, h767, _, _) => {
            h767
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h768, _) => {
            h768
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h769) => {
            h769
        }
    }))
}

pub fn f196(p772: List<List<Bytes>>, p773: List<List<Bytes>>) -> List<List<Bytes>> {
    let m771 = (p772, p773);
    {
        let mut m771 = m771;
        loop {
            let step = {
                let (v0, v1) = m771;
                let m770 = v0;
                match m770.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f197(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f266(v0.clone(), 0u64, {
        let a774 = v0;
        length_bytes(a774)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f198(v0: Adt22) -> Bytes {
    let a795 = f267(match v0.clone() {
        Adt22::C0(h775, _, _, _, _, _, _, _) => {
            h775
        }
    });
    let a796 = {
        let a793 = f267(match v0.clone() {
            Adt22::C0(_, h776, _, _, _, _, _, _) => {
                h776
            }
        });
        let a794 = {
            let a791 = f267(match v0.clone() {
                Adt22::C0(_, _, h777, _, _, _, _, _) => {
                    h777
                }
            });
            let a792 = {
                let a789 = f267(match v0.clone() {
                    Adt22::C0(_, _, _, h778, _, _, _, _) => {
                        h778
                    }
                });
                let a790 = {
                    let a787 = f267(match v0.clone() {
                        Adt22::C0(_, _, _, _, h779, _, _, _) => {
                            h779
                        }
                    });
                    let a788 = {
                        let a785 = f267(match v0.clone() {
                            Adt22::C0(_, _, _, _, _, h780, _, _) => {
                                h780
                            }
                        });
                        let a786 = {
                            let a783 = f267(match v0.clone() {
                                Adt22::C0(_, _, _, _, _, _, h781, _) => {
                                    h781
                                }
                            });
                            let a784 = f267(match v0 {
                                Adt22::C0(_, _, _, _, _, _, _, h782) => {
                                    h782
                                }
                            });
                            append_bytes(a783, a784)
                        };
                        append_bytes(a785, a786)
                    };
                    append_bytes(a787, a788)
                };
                append_bytes(a789, a790)
            };
            append_bytes(a791, a792)
        };
        append_bytes(a793, a794)
    };
    append_bytes(a795, a796)
}

pub fn f199(v0: Bytes) -> R<Adt22> {
    let v1: u64 = {
        let a797 = v0.clone();
        length_bytes(a797)
    };
    let v2: u64 = {
        let a803 = {
            let a800 = {
                let a798 = v1;
                let a799 = 72u64;
                nat_add(a798, a799)?
            };
            let a801 = 64u64;
            let a802 = 0u64;
            nat_quot(a800, a801, a802)
        };
        let a804 = 64u64;
        nat_mul(a803, a804)?
    };
    let (_, h805) = f268(Fn4::F269(v0, v1, v2), {
        let a806 = v2;
        let a807 = 64u64;
        let a808 = 0u64;
        nat_quot(a806, a807, a808)
    }, (0u64, f270()))?;
    Ok(h805)
}

pub fn f200(p811: List<List<Bytes>>, v1: Bytes, p812: List<Bytes>) -> R<List<Bytes>> {
    let m810 = (p811, p812);
    {
        let mut m810 = m810;
        loop {
            let step = {
                let (v0, v2) = m810;
                let m809 = v0;
                match m809.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f271(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f202(f272(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f273(v0)?))
}

pub fn f202(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m813 = v0;
    match m813.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m814 = v3.clone();
            match m814.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a817 = v2;
                    let a818 = {
                        let a815 = v1.clone();
                        let a816 = f202(v3, v1);
                        append_bytes(a815, a816)
                    };
                    append_bytes(a817, a818)
                }
            }
        }
    }
}

pub fn f203(v0: List<Bytes>) -> Adt14 {
    let m819 = v0;
    match m819.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f203(v2)))
        }
    }
}

pub fn f204(v0: Fn2, p823: Option<Bytes>, p824: List<Bytes>) -> Option<Bytes> {
    let m822 = (p823, p824);
    {
        let mut m822 = m822;
        loop {
            let step = {
                let (v1, v2) = m822;
                let m820 = v2;
                match m820.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c821: Fn2 = v0.clone();
                            c821.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m825 = v1;
    match m825 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a828 = {
                    let a826 = v3;
                    let a827 = v0;
                    append_bytes(a826, a827)
                };
                let a829 = v2;
                append_bytes(a828, a829)
            })
        }
    }
}

pub fn f206(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f273(v0.clone())?;
    let v2: u64 = f274(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a830 = v1;
        let a831 = v2;
        nat_sub(a830, a831)
    }))
}

pub fn f207(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f275(v0)?;
    let m835 = f276(v1.clone(), 0u64, {
        let a833 = {
            let a832 = v1.clone();
            length_bytes(a832)
        };
        let a834 = 2u64;
        nat_add(a833, a834)?
    }, 0u64)?;
    match m835 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a838 = {
                let (_, h836) = v2;
                h836
            };
            let a839 = {
                let a837 = v1;
                length_bytes(a837)
            };
            if nat_eq(a838, a839) {
                Ok(Some({
                    let (h840, _) = v2;
                    h840
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f208(v0: Bytes) -> Bytes {
    f7(List::cons(f76(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f209(v0: Adt14) -> bool {
    if f190(v0.clone(), f8(Str::lit("domain"))) {
        if f190(v0.clone(), f8(Str::lit("row"))) {
            if f85(f145(v0.clone(), f8(Str::lit("relation"))), f277()) {
                if f278(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f278(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f278(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f278(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f210(v0: Adt14) -> bool {
    if f190(v0.clone(), f8(Str::lit("subject"))) {
        if f190(v0.clone(), f8(Str::lit("object"))) {
            if f190(v0.clone(), f8(Str::lit("evidence"))) {
                f85(f145(v0, f8(Str::lit("relation"))), f279())
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

pub fn f211(v0: Option<List<Adt14>>) -> bool {
    let m841 = v0;
    match m841 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f212(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m842 = v0;
    match m842 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f213(v0: Adt14) -> Bytes {
    if f190(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f190(v0.clone(), f8(Str::lit("id"))) {
            if f190(v0.clone(), f8(Str::lit("statement"))) {
                if f190(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f221(v0, f8(Str::lit("evidence")))
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

pub fn f214(v0: List<Adt14>) -> Adt14 {
    let m843 = v0;
    match m843.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f214(v2)))
        }
    }
}

pub fn f215(v0: Adt14) -> bool {
    if f190(v0.clone(), f8(Str::lit("name"))) {
        if f221(v0.clone(), f8(Str::lit("schema"))) {
            let m844 = f217(f73(f146(f142(v0, f8(Str::lit("citation"))))));
            match m844 {
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

pub fn f216(v0: Bytes) -> R<Bytes> {
    f280(v0.clone(), 0u64, {
        let a845 = v0;
        length_bytes(a845)
    }, true, Bytes::lit(&[]))
}

pub fn f217(v0: List<Bytes>) -> Option<Adt14> {
    let m846 = v0;
    match m846.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m847 = v2;
                match m847.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m848 = v2;
                    match m848.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m849 = v6;
                                match m849.uncons() {
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
                    if f36(v1, f8(Str::lit("der"))) {
                        let m850 = v2;
                        match m850.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m851 = v10;
                                match m851.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f203(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f218(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m852 = f59(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m852.uncons() {
        None => {
            let m853 = f217(v4);
            match m853 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a854 = f8(Str::lit("Catalog."));
                        let a855 = f216(v3.clone())?;
                        append_bytes(a854, a855)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f216(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m856 = f281(v2, v9.clone(), f216(v3.clone())?)?;
                    match m856 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f216(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f144(Adt14::C3(std::rc::Rc::new(f214(f282(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f144(Adt14::C3(std::rc::Rc::new(f214(f282(f72(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a860 = f283(f69(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a858 = {
                                        let a857 = v10;
                                        length_bytes(a857)
                                    };
                                    let a859 = 4u64;
                                    nat_sub(a858, a859)
                                }))?;
                                let a861 = Bytes::lit(&[0x0a]);
                                append_bytes(a860, a861)
                            }), List::cons(f119(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f219(v0: Adt14) -> R<Bytes> {
    let m862 = v0;
    match m862 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f284(v2, v3)
        }
        Adt14::C2(v4) => {
            f285(v4)
        }
        Adt14::C3(r863) => {
            let v5 = (*r863).clone();
            Ok(f286(f287(v5)?))
        }
        Adt14::C4(r864) => {
            let v6 = (*r864).clone();
            f288(f289(v6)?)
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

pub fn f220(v0: Adt14) -> List<Adt14> {
    let m865 = v0;
    match m865 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r866) => {
            let v3 = (*r866).clone();
            f220(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r867, r868) => {
            let v1 = (*r867).clone();
            let v2 = (*r868).clone();
            List::cons(v1, f220(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f221(v0: Adt14, v1: Bytes) -> bool {
    let m869 = f142(v0, v1);
    match m869 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f222(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m870 = f142(v0, v1);
    match m870 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f146(f142(v3, v2))
        }
    }
}

pub fn f223(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f142(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f115(Str::lit("id"), f145(v0.clone(), f8(Str::lit("id"))))?, List::cons(f115(Str::lit("name"), f145(v0.clone(), f8(Str::lit("name"))))?, List::cons(f115(Str::lit("issuer"), f145(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f115(Str::lit("edition"), f145(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f115(Str::lit("canonical_id"), f145(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f115(Str::lit("source_role"), f145(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f115(Str::lit("retrieval_date"), f145(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f115(Str::lit("url"), f290(v2.clone(), f8(Str::lit("url"))))?, List::cons(f115(Str::lit("media_type"), f290(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f115(Str::lit("license"), f290(v2.clone(), f8(Str::lit("license"))))?, List::cons(f115(Str::lit("redistribution"), f290(v2, f8(Str::lit("redistribution"))))?, List::cons(f115(Str::lit("statement"), f145(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m871 = v1;
        match m871 {
            None => {
                f7(List::cons(f115(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f115(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f115(Str::lit("artifact"), {
                    let (h872, _) = v3;
                    h872
                })?, List::cons(f115(Str::lit("checksum"), {
                    let (_, h873) = v3;
                    h873
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f291(f73(f146(f142(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f224(v0: Bytes, v1: u64) -> Option<u8> {
    let a874 = v0;
    let a875 = v1;
    index_bytes(a874, a875)
}

pub fn f225(v0: Bytes) -> bool {
    f129(v0, f8(Str::lit(".lex.tex")))
}

pub fn f226(v0: Fn3, p880: u64, p881: u64) -> R<(u64, bool)> {
    let m879 = (p880, p881);
    {
        let mut m879 = m879;
        loop {
            let step = {
                let (v1, v2) = m879;
                let m876 = v1;
                if m876 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m876 - 1;
                    let m878 = {
                        let c877: Fn3 = v0.clone();
                        c877.apply(v2)?
                    };
                    match m878 {
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
                Ok(next) => { m879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f228(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a884 = {
            let a882 = v4;
            let a883 = v1;
            nat_add(a882, a883)?
        };
        let a885 = v3;
        if nat_lt(a884, a885) {
            Ok(Some({
                let a886 = v4;
                let a887 = 1u64;
                nat_add(a886, a887)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f228(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m892 = {
        let a889 = v0;
        let a890 = v2;
        let a891 = {
            let a888 = v1.clone();
            length_bytes(a888)
        };
        slice_bytes(a889, a890, a891)
    };
    match m892 {
        None => {
            false
        }
        Some(v3) => {
            let a893 = v3;
            let a894 = v1;
            equal(a893, a894)
        }
    }
}

pub fn f229(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m895 = v0;
    match m895.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f229(v3, {
                let a896 = v1;
                let a897 = 1u64;
                nat_add(a896, a897)?
            })?))
        }
    }
}

pub fn f230(v0: Adt16, v1: List<u64>, v2: Adt19) -> bool {
    let m898 = v2;
    match m898 {
        Adt19::C0(v3, v4) => {
            if f236(v3, match v0.clone() {
                Adt16::C0(h899, _, _, _) => {
                    h899
                }
            }) {
                f292(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f236(v5, match v0 {
                Adt16::C0(_, _, h900, _) => {
                    h900
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f232(v7, v1) {
                f236(v8, match v0 {
                    Adt16::C0(_, _, _, h901) => {
                        h901
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f231(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m902 = f142(v1.clone(), f8(Str::lit("std")));
    match m902 {
        None => {
            let m907 = f150(f142(v1.clone(), f8(Str::lit("req"))));
            match m907 {
                None => {
                    let m909 = f142(v1, f8(Str::lit("der")));
                    match m909 {
                        None => {
                            Ok(f293(v0))
                        }
                        Some(v5) => {
                            let m910 = f294(f145(v5.clone(), f8(Str::lit("rule"))));
                            match m910 {
                                None => {
                                    Ok(f293(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f166(f73(f146(f142(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h911) => {
                                            h911
                                        }
                                    })?, f234(f145(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h912, _) => {
                                            h912
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f234(v4, match v0 {
                        Adt12::C0(_, _, h908, _, _) => {
                            h908
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m903 = f150(f142(v2.clone(), f8(Str::lit("row"))));
            match m903 {
                None => {
                    Ok(Adt19::C0(f234(f145(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h904, _, _, _, _) => {
                            h904
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f234(f145(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h905, _, _, _, _) => {
                            h905
                        }
                    })?, Some(f234(v3, match v0 {
                        Adt12::C0(_, h906, _, _, _) => {
                            h906
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f232(p915: List<u64>, v1: List<u64>) -> bool {
    let m914 = p915;
    {
        let mut m914 = m914;
        loop {
            let step = {
                let v0 = m914;
                let m913 = v0;
                match m913.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f236(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m914 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233(p920: List<u64>) -> bool {
    let m919 = p920;
    {
        let mut m919 = m919;
        loop {
            let step = {
                let v0 = m919;
                let m916 = v0;
                match m916.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a917 = v1;
                        let a918 = 7u64;
                        if nat_le(a917, a918) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m919 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m921 = f295(v0, v1.clone(), 0u64)?;
    match m921 {
        None => {
            let a922 = v1;
            Ok(length_list(a922))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f235(v0: Adt14) -> u64 {
    let m923 = f296(f142(v0, f8(Str::lit("stratum"))));
    match m923 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f236(v0: u64, p928: List<u64>) -> bool {
    let m927 = p928;
    {
        let mut m927 = m927;
        loop {
            let step = {
                let v1 = m927;
                let m924 = v1;
                match m924.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a925 = v2;
                        let a926 = v0;
                        if nat_eq(a925, a926) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m927 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f192(v0.clone(), v1) {
        let m929 = f30(v0, 34u8)?;
        match m929.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m930 = v3;
                match m930.uncons() {
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

pub fn f238(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f239(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m931 = v0.clone();
    match m931.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a932 = v2;
            let a933 = v1.clone();
            if equal(a932, a933) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f240(v0: List<(Bytes, ())>) -> List<Bytes> {
    f171(f297(Fn5::F298, List::<Bytes>::nil(), v0))
}

pub fn f241(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f299(f300(Fn6::F301, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m938 = {
        let (h934, _) = f302(Fn7::F303, {
            let a936 = {
                let a935 = v0;
                length_list(a935)
            };
            let a937 = 1u64;
            nat_add(a936, a937)?
        }, v1)?;
        h934
    };
    match m938.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f242(v0: List<Bytes>) -> List<(Bytes, ())> {
    f304(f305(Fn8::F306, List::<(Bytes, ())>::nil(), v0))
}

pub fn f243(p941: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m940 = p941;
    {
        let mut m940 = m940;
        loop {
            let step = {
                let v0 = m940;
                let m939 = v0;
                match m939.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f271(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f290(f142(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f245(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h942, _) = v1.clone();
        h942
    };
    let v3: u64 = {
        let (h943, _) = {
            let (_, h944) = v1.clone();
            h944
        };
        h943
    };
    let v4: List<Bytes> = {
        let (_, h945) = {
            let (_, h946) = v1;
            h946
        };
        h945
    };
    let m947 = f224(v0.clone(), v2);
    match m947 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a948 = v5;
            let a949 = 10u8;
            if equal(a948, a949) {
                Ok(Some(({
                    let a950 = v2;
                    let a951 = 1u64;
                    nat_add(a950, a951)?
                }, ({
                    let a952 = v2;
                    let a953 = 1u64;
                    nat_add(a952, a953)?
                }, List::cons(f307(f32(v0, v3, {
                    let a954 = v2;
                    let a955 = v3;
                    nat_sub(a954, a955)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a956 = v2;
                    let a957 = 1u64;
                    nat_add(a956, a957)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f246(v0: Bytes) -> R<Bytes> {
    let a958 = f8(Str::lit("Catalog."));
    let a959 = f216(v0)?;
    Ok(append_bytes(a958, a959))
}

pub fn f247(v0: List<Bytes>) -> List<Bytes> {
    let m960 = v0;
    match m960.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f129(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f125(f133(), v1), f247(v2))
            } else {
                f247(v2)
            }
        }
    }
}

pub fn f248(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m961 = v1;
    match m961.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f308(v0.clone(), v2)?;
            let (h962, _) = v4.clone();
            if h962 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f248(v0, v3)?;
                let (h963, _) = v5.clone();
                if h963 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a966 = {
                            let (_, h964) = v4;
                            h964
                        };
                        let a967 = {
                            let (_, h965) = v5;
                            h965
                        };
                        append_bytes(a966, a967)
                    }))
                }
            }
        }
    }
}

pub fn f249(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m968 = v1;
    match m968.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f190(v2.clone(), f8(Str::lit("subject"))) {
                if f191(v0.clone(), v2.clone()) {
                    f249(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f145(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f249(v0, v3))
                }
            } else {
                if f258(v0.clone(), v2.clone()) {
                    f249(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f145(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f259(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f249(v0, v3))
                }
            }
        }
    }
}

pub fn f250(v0: Adt1) -> R<Bytes> {
    let a970 = {
        let a969 = f102(f103(f121(f122(f111(f69(v0.clone(), f116()))?)), f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f105(f72(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a969)
    };
    let a971 = 0u64;
    if nat_eq(a970, a971) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f251(v0: List<Adt14>) -> R<List<Bytes>> {
    let m972 = v0;
    match m972.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f120(f73(f146(f142(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f145(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f145(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f309(f82(f145(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f251(v2)?))
        }
    }
}

pub fn f252(v0: Bytes, v1: List<Adt21>) -> List<Bytes> {
    let m973 = v1;
    match m973.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt21::C0(h974, _, _) => {
                    h974
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt21::C0(_, h975, _) => {
                        h975
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt21::C0(_, _, h976) => {
                        h976
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f252(v0, v3))
            } else {
                f252(v0, v3)
            }
        }
    }
}

pub fn f253(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m977 = v1;
    match m977.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f145(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f253(v0, v3))
            } else {
                if f36(f145(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f145(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f253(v0, v3))
                } else {
                    f253(v0, v3)
                }
            }
        }
    }
}

pub fn f254(p982: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m981 = p982;
    {
        let mut m981 = m981;
        loop {
            let step = {
                let v0 = m981;
                let m978 = v0;
                match m978.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m979 = v2;
                        match m979.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m980 = v5;
                                    match m980.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f310(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m983 = v0;
    match m983.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m984 = f142(v1, f8(Str::lit("std")));
            match m984 {
                None => {
                    f255(v2)
                }
                Some(v3) => {
                    if f190(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f145(v3.clone(), f8(Str::lit("row"))), f145(v3, f8(Str::lit("authority")))), f255(v2))
                    } else {
                        f255(v2)
                    }
                }
            }
        }
    }
}

pub fn f256(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m985 = v1;
    match m985.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f145(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f145(v2, f8(Str::lit("object")));
            if if f85(v4.clone(), match v0.clone() {
                Adt15::C0(h986, _, _) => {
                    h986
                }
            }) {
                f85(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h987, _) => {
                        h987
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f256(v0, v3))
            } else {
                if if f85(v5.clone(), match v0.clone() {
                    Adt15::C0(h988, _, _) => {
                        h988
                    }
                }) {
                    f85(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h989, _) => {
                            h989
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f256(v0, v3))
                } else {
                    f256(v0, v3)
                }
            }
        }
    }
}

pub fn f257(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt21>> {
    let m990 = v0;
    match m990.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v4, v5)) => {
            if if f311(v4.clone(), v2.clone()) {
                true
            } else {
                f311(v4.clone(), v3.clone())
            } {
                f257(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt21::C0({
                    let (h991, _) = v4.clone();
                    h991
                }, {
                    let a994 = {
                        let a993 = f77(f312({
                            let (h992, _) = v4.clone();
                            h992
                        }, v1.clone()))?;
                        length_list(a993)
                    };
                    let a995 = 1u64;
                    if nat_eq(a994, a995) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h996) = v4.clone();
                    h996
                }), f257(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f258(v0: Adt15, v1: Adt14) -> bool {
    if f85(f145(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h997, _, _) => {
            h997
        }
    }) {
        f85(f259(v1), match v0 {
            Adt15::C0(_, h998, _) => {
                h998
            }
        })
    } else {
        false
    }
}

pub fn f259(v0: Adt14) -> Bytes {
    let m999 = f142(v0, f8(Str::lit("evidence")));
    match m999 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f145(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f260(v0: Adt14) -> Bytes {
    if f190(v0.clone(), f8(Str::lit("req"))) {
        let a1000 = f8(Str::lit("req "));
        let a1001 = f145(v0, f8(Str::lit("req")));
        append_bytes(a1000, a1001)
    } else {
        let m1002 = f142(v0.clone(), f8(Str::lit("std")));
        match m1002 {
            None => {
                let m1005 = f142(v0, f8(Str::lit("der")));
                match m1005 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f145(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f120(f73(f146(f142(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f145(v1.clone(), f8(Str::lit("authority"))), List::cons(if f190(v1.clone(), f8(Str::lit("row"))) {
                    let a1003 = f8(Str::lit(" row "));
                    let a1004 = f145(v1, f8(Str::lit("row")));
                    append_bytes(a1003, a1004)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f261(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1006 = v1;
    match m1006.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85({
                let (h1007, _) = v2.clone();
                h1007
            }, v0.clone()) {
                List::cons({
                    let (_, h1008) = v2;
                    h1008
                }, f261(v0, v3))
            } else {
                if f85({
                    let (_, h1009) = v2;
                    h1009
                }, v0.clone()) {
                    List::cons({
                        let (h1010, _) = v2;
                        h1010
                    }, f261(v0, v3))
                } else {
                    f261(v0, v3)
                }
            }
        }
    }
}

pub fn f262(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f85(v2.clone(), match v0.clone() {
        Adt15::C0(h1011, _, _) => {
            h1011
        }
    }) {
        true
    } else {
        if f85(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1012, _) => {
                h1012
            }
        }) {
            true
        } else {
            if f85(v2, match v0.clone() {
                Adt15::C0(_, _, h1013) => {
                    h1013
                }
            }) {
                true
            } else {
                if f190(v1.clone(), f8(Str::lit("scheme"))) {
                    f85(f145(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1014, _) => {
                            h1014
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f263(v0: Bytes, v1: u64) -> R<u64> {
    let m1017 = {
        let a1015 = v0;
        let a1016 = v1;
        index_bytes(a1015, a1016)
    };
    match m1017 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1062 = {
                let a1038 = {
                    let a1026 = {
                        let a1020 = {
                            let a1018 = v3;
                            let a1019 = 128u8;
                            fixed_u8::bit_and(a1018, a1019)
                        };
                        let a1021 = 128u8;
                        if equal(a1020, a1021) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1027 = {
                        let a1024 = {
                            let a1022 = v3;
                            let a1023 = 64u8;
                            fixed_u8::bit_and(a1022, a1023)
                        };
                        let a1025 = 64u8;
                        if equal(a1024, a1025) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1026, a1027)?
                };
                let a1039 = {
                    let a1036 = {
                        let a1030 = {
                            let a1028 = v3;
                            let a1029 = 32u8;
                            fixed_u8::bit_and(a1028, a1029)
                        };
                        let a1031 = 32u8;
                        if equal(a1030, a1031) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1037 = {
                        let a1034 = {
                            let a1032 = v3;
                            let a1033 = 16u8;
                            fixed_u8::bit_and(a1032, a1033)
                        };
                        let a1035 = 16u8;
                        if equal(a1034, a1035) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1036, a1037)?
                };
                nat_add(a1038, a1039)?
            };
            let a1063 = {
                let a1060 = {
                    let a1048 = {
                        let a1042 = {
                            let a1040 = v3;
                            let a1041 = 8u8;
                            fixed_u8::bit_and(a1040, a1041)
                        };
                        let a1043 = 8u8;
                        if equal(a1042, a1043) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1049 = {
                        let a1046 = {
                            let a1044 = v3;
                            let a1045 = 4u8;
                            fixed_u8::bit_and(a1044, a1045)
                        };
                        let a1047 = 4u8;
                        if equal(a1046, a1047) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1048, a1049)?
                };
                let a1061 = {
                    let a1058 = {
                        let a1052 = {
                            let a1050 = v3;
                            let a1051 = 2u8;
                            fixed_u8::bit_and(a1050, a1051)
                        };
                        let a1053 = 2u8;
                        if equal(a1052, a1053) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1059 = {
                        let a1056 = {
                            let a1054 = v3;
                            let a1055 = 1u8;
                            fixed_u8::bit_and(a1054, a1055)
                        };
                        let a1057 = 1u8;
                        if equal(a1056, a1057) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1058, a1059)?
                };
                nat_add(a1060, a1061)?
            };
            nat_add(a1062, a1063)
        }
    }
}

pub fn f264(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1067 = {
        let a1064 = v1;
        let a1065 = v2;
        let a1066 = 1u64;
        slice_bytes(a1064, a1065, a1066)
    };
    match m1067 {
        None => {
            v0
        }
        Some(v3) => {
            let a1068 = v0;
            let a1069 = v3;
            append_bytes(a1068, a1069)
        }
    }
}

pub fn f265(p1072: List<Bytes>, p1073: List<Bytes>) -> List<Bytes> {
    let m1071 = (p1072, p1073);
    {
        let mut m1071 = m1071;
        loop {
            let step = {
                let (v0, v1) = m1071;
                let m1070 = v0;
                match m1070.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f266(v0: Bytes, p1145: u64, p1146: u64, p1147: Bytes) -> R<Bytes> {
    let m1144 = (p1145, (p1146, p1147));
    {
        let mut m1144 = m1144;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1144;
                let m1074 = v2;
                if m1074 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1074 - 1;
                    let m1077 = {
                        let a1075 = v0.clone();
                        let a1076 = v1;
                        index_bytes(a1075, a1076)
                    };
                    match m1077 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1122 = {
                                    let a1098 = {
                                        let a1086 = {
                                            let a1080 = {
                                                let a1078 = v6;
                                                let a1079 = 128u8;
                                                fixed_u8::bit_and(a1078, a1079)
                                            };
                                            let a1081 = 128u8;
                                            if equal(a1080, a1081) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1087 = {
                                            let a1084 = {
                                                let a1082 = v6;
                                                let a1083 = 64u8;
                                                fixed_u8::bit_and(a1082, a1083)
                                            };
                                            let a1085 = 64u8;
                                            if equal(a1084, a1085) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1086, a1087)?
                                    };
                                    let a1099 = {
                                        let a1096 = {
                                            let a1090 = {
                                                let a1088 = v6;
                                                let a1089 = 32u8;
                                                fixed_u8::bit_and(a1088, a1089)
                                            };
                                            let a1091 = 32u8;
                                            if equal(a1090, a1091) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1097 = {
                                            let a1094 = {
                                                let a1092 = v6;
                                                let a1093 = 16u8;
                                                fixed_u8::bit_and(a1092, a1093)
                                            };
                                            let a1095 = 16u8;
                                            if equal(a1094, a1095) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1096, a1097)?
                                    };
                                    nat_add(a1098, a1099)?
                                };
                                let a1123 = {
                                    let a1120 = {
                                        let a1108 = {
                                            let a1102 = {
                                                let a1100 = v6;
                                                let a1101 = 8u8;
                                                fixed_u8::bit_and(a1100, a1101)
                                            };
                                            let a1103 = 8u8;
                                            if equal(a1102, a1103) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1109 = {
                                            let a1106 = {
                                                let a1104 = v6;
                                                let a1105 = 4u8;
                                                fixed_u8::bit_and(a1104, a1105)
                                            };
                                            let a1107 = 4u8;
                                            if equal(a1106, a1107) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1108, a1109)?
                                    };
                                    let a1121 = {
                                        let a1118 = {
                                            let a1112 = {
                                                let a1110 = v6;
                                                let a1111 = 2u8;
                                                fixed_u8::bit_and(a1110, a1111)
                                            };
                                            let a1113 = 2u8;
                                            if equal(a1112, a1113) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1119 = {
                                            let a1116 = {
                                                let a1114 = v6;
                                                let a1115 = 1u8;
                                                fixed_u8::bit_and(a1114, a1115)
                                            };
                                            let a1117 = 1u8;
                                            if equal(a1116, a1117) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1118, a1119)?
                                    };
                                    nat_add(a1120, a1121)?
                                };
                                nat_add(a1122, a1123)?
                            };
                            let a1124 = v7;
                            let a1125 = 34u64;
                            if if nat_eq(a1124, a1125) {
                                true
                            } else {
                                let a1126 = v7;
                                let a1127 = 92u64;
                                nat_eq(a1126, a1127)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1128 = v1;
                                    let a1129 = 1u64;
                                    nat_add(a1128, a1129)?
                                }, (v4, {
                                    let a1132 = v3;
                                    let a1133 = {
                                        let a1130 = Bytes::lit(&[0x5c]);
                                        let a1131 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1130, a1131)
                                    };
                                    append_bytes(a1132, a1133)
                                })))
                            } else {
                                let a1134 = v7;
                                let a1135 = 10u64;
                                if nat_eq(a1134, a1135) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1136 = v1;
                                        let a1137 = 1u64;
                                        nat_add(a1136, a1137)?
                                    }, (v4, {
                                        let a1138 = v3;
                                        let a1139 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1138, a1139)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1140 = v1;
                                        let a1141 = 1u64;
                                        nat_add(a1140, a1141)?
                                    }, (v4, {
                                        let a1142 = v3;
                                        let a1143 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1142, a1143)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: u32) -> Bytes {
    let a1176 = f313({
        let a1148 = f314(v0, 28u32);
        let a1149 = 15u32;
        fixed_u32::bit_and(a1148, a1149)
    });
    let a1177 = {
        let a1174 = f313({
            let a1150 = f314(v0, 24u32);
            let a1151 = 15u32;
            fixed_u32::bit_and(a1150, a1151)
        });
        let a1175 = {
            let a1172 = f313({
                let a1152 = f314(v0, 20u32);
                let a1153 = 15u32;
                fixed_u32::bit_and(a1152, a1153)
            });
            let a1173 = {
                let a1170 = f313({
                    let a1154 = f314(v0, 16u32);
                    let a1155 = 15u32;
                    fixed_u32::bit_and(a1154, a1155)
                });
                let a1171 = {
                    let a1168 = f313({
                        let a1156 = f314(v0, 12u32);
                        let a1157 = 15u32;
                        fixed_u32::bit_and(a1156, a1157)
                    });
                    let a1169 = {
                        let a1166 = f313({
                            let a1158 = f314(v0, 8u32);
                            let a1159 = 15u32;
                            fixed_u32::bit_and(a1158, a1159)
                        });
                        let a1167 = {
                            let a1164 = f313({
                                let a1160 = f314(v0, 4u32);
                                let a1161 = 15u32;
                                fixed_u32::bit_and(a1160, a1161)
                            });
                            let a1165 = f313({
                                let a1162 = v0;
                                let a1163 = 15u32;
                                fixed_u32::bit_and(a1162, a1163)
                            });
                            append_bytes(a1164, a1165)
                        };
                        append_bytes(a1166, a1167)
                    };
                    append_bytes(a1168, a1169)
                };
                append_bytes(a1170, a1171)
            };
            append_bytes(a1172, a1173)
        };
        append_bytes(a1174, a1175)
    };
    append_bytes(a1176, a1177)
}

pub fn f268(v0: Fn4, p1181: u64, p1182: (u64, Adt22)) -> R<(u64, Adt22)> {
    let m1180 = (p1181, p1182);
    {
        let mut m1180 = m1180;
        loop {
            let step = {
                let (v1, v2) = m1180;
                let m1178 = v1;
                if m1178 == 0 {
                    Err::<(u64, (u64, Adt22)), R<(u64, Adt22)>>(Ok(v2))
                } else {
                    let v3 = m1178 - 1;
                    Ok::<(u64, (u64, Adt22)), R<(u64, Adt22)>>((v3, {
                        let c1179: Fn4 = v0.clone();
                        c1179.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f269(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt22)) -> R<(u64, Adt22)> {
    Ok(({
        let a1184 = {
            let (h1183, _) = v3.clone();
            h1183
        };
        let a1185 = 64u64;
        nat_add(a1184, a1185)?
    }, f315({
        let (_, h1186) = v3;
        h1186
    }, f316(v0, v1, v2, {
        let (h1187, _) = v3;
        h1187
    })?)?))
}

pub fn f270() -> Adt22 {
    Adt22::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f271(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f85(v1, f68(f202(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f272(v0: List<Bytes>) -> List<Bytes> {
    let m1188 = v0;
    match m1188.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f272(v2)
            } else {
                List::cons(v1, f272(v2))
            }
        }
    }
}

pub fn f273(v0: Bytes) -> R<u64> {
    let (h1189, _) = f226(Fn3::F317(v0.clone()), {
        let a1190 = v0.clone();
        length_bytes(a1190)
    }, {
        let a1191 = v0;
        length_bytes(a1191)
    })?;
    Ok(h1189)
}

pub fn f274(v0: Bytes, v1: u64) -> R<u64> {
    let (h1192, _) = f226(Fn3::F318(v1, v0), v1, 0u64)?;
    Ok(h1192)
}

pub fn f275(v0: Bytes) -> R<Bytes> {
    f319(v0.clone(), 0u64, {
        let a1193 = v0;
        length_bytes(a1193)
    }, false, false, Bytes::lit(&[]))
}

pub fn f276(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1194 = v2;
    if m1194 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1194 - 1;
        let v5: u64 = f320(v0.clone(), v1)?;
        let a1195 = v3;
        let a1196 = 1u64;
        if nat_eq(a1195, a1196) {
            let m1197 = f276(v0.clone(), v1, v4, 0u64)?;
            match m1197 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f320(v0.clone(), {
                        let (_, h1198) = v6.clone();
                        h1198
                    })?;
                    let a1199 = v7;
                    let a1200 = 44u64;
                    if nat_eq(a1199, a1200) {
                        let m1204 = f276(v0, {
                            let a1202 = {
                                let (_, h1201) = v6;
                                h1201
                            };
                            let a1203 = 1u64;
                            nat_add(a1202, a1203)?
                        }, v4, 1u64)?;
                        match m1204 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1205, _) = v6;
                                    h1205
                                }), std::rc::Rc::new({
                                    let (h1206, _) = v8;
                                    h1206
                                })), {
                                    let (_, h1207) = v8;
                                    h1207
                                })))
                            }
                        }
                    } else {
                        let a1208 = v7;
                        let a1209 = 93u64;
                        if nat_eq(a1208, a1209) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1210, _) = v6;
                                h1210
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1212 = {
                                    let (_, h1211) = v6;
                                    h1211
                                };
                                let a1213 = 1u64;
                                nat_add(a1212, a1213)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1214 = v3;
            let a1215 = 2u64;
            if nat_eq(a1214, a1215) {
                let m1218 = f321(v0.clone(), {
                    let a1216 = v1;
                    let a1217 = 1u64;
                    nat_add(a1216, a1217)?
                })?;
                match m1218 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1219 = v5;
                        let a1220 = 34u64;
                        if if nat_eq(a1219, a1220) {
                            let a1222 = f320(v0.clone(), {
                                let (h1221, _) = v9.clone();
                                h1221
                            })?;
                            let a1223 = 58u64;
                            nat_eq(a1222, a1223)
                        } else {
                            false
                        } {
                            let m1227 = f276(v0.clone(), {
                                let a1225 = {
                                    let (h1224, _) = v9;
                                    h1224
                                };
                                let a1226 = 1u64;
                                nat_add(a1225, a1226)?
                            }, v4, 0u64)?;
                            match m1227 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f320(v0.clone(), {
                                        let (_, h1228) = v10.clone();
                                        h1228
                                    })?;
                                    let a1229 = v11;
                                    let a1230 = 44u64;
                                    if nat_eq(a1229, a1230) {
                                        let m1234 = f276(v0, {
                                            let a1232 = {
                                                let (_, h1231) = v10;
                                                h1231
                                            };
                                            let a1233 = 1u64;
                                            nat_add(a1232, a1233)?
                                        }, v4, 2u64)?;
                                        match m1234 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1235) = v9;
                                                    h1235
                                                }, std::rc::Rc::new({
                                                    let (h1236, _) = v10;
                                                    h1236
                                                }), std::rc::Rc::new({
                                                    let (h1237, _) = v12;
                                                    h1237
                                                })), {
                                                    let (_, h1238) = v12;
                                                    h1238
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1239 = v11;
                                        let a1240 = 125u64;
                                        if nat_eq(a1239, a1240) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1241) = v9;
                                                h1241
                                            }, std::rc::Rc::new({
                                                let (h1242, _) = v10;
                                                h1242
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1244 = {
                                                    let (_, h1243) = v10;
                                                    h1243
                                                };
                                                let a1245 = 1u64;
                                                nat_add(a1244, a1245)?
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
                let a1246 = v5;
                let a1247 = 34u64;
                if nat_eq(a1246, a1247) {
                    let m1250 = f321(v0, {
                        let a1248 = v1;
                        let a1249 = 1u64;
                        nat_add(a1248, a1249)?
                    })?;
                    match m1250 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1251) = v13;
                                h1251
                            }), {
                                let (h1252, _) = v13;
                                h1252
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1253 = v1;
                            let a1254 = 4u64;
                            nat_add(a1253, a1254)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1255 = v1;
                                let a1256 = 5u64;
                                nat_add(a1255, a1256)?
                            })))
                        } else {
                            let a1257 = v5;
                            let a1258 = 91u64;
                            if nat_eq(a1257, a1258) {
                                let a1261 = f320(v0.clone(), {
                                    let a1259 = v1;
                                    let a1260 = 1u64;
                                    nat_add(a1259, a1260)?
                                })?;
                                let a1262 = 93u64;
                                if nat_eq(a1261, a1262) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1263 = v1;
                                        let a1264 = 2u64;
                                        nat_add(a1263, a1264)?
                                    })))
                                } else {
                                    let m1267 = f276(v0, {
                                        let a1265 = v1;
                                        let a1266 = 1u64;
                                        nat_add(a1265, a1266)?
                                    }, v4, 1u64)?;
                                    match m1267 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1268, _) = v14;
                                                h1268
                                            })), {
                                                let (_, h1269) = v14;
                                                h1269
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1270 = v5;
                                let a1271 = 123u64;
                                if nat_eq(a1270, a1271) {
                                    let a1274 = f320(v0.clone(), {
                                        let a1272 = v1;
                                        let a1273 = 1u64;
                                        nat_add(a1272, a1273)?
                                    })?;
                                    let a1275 = 125u64;
                                    if nat_eq(a1274, a1275) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1276 = v1;
                                            let a1277 = 2u64;
                                            nat_add(a1276, a1277)?
                                        })))
                                    } else {
                                        let m1280 = f276(v0, {
                                            let a1278 = v1;
                                            let a1279 = 1u64;
                                            nat_add(a1278, a1279)?
                                        }, v4, 2u64)?;
                                        match m1280 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1281, _) = v15;
                                                    h1281
                                                })), {
                                                    let (_, h1282) = v15;
                                                    h1282
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1283 = f322(v0, v1)?;
                                    match m1283 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1284) = v16;
                                                h1284
                                            }, {
                                                let (h1285, _) = v16;
                                                h1285
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

pub fn f277() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f278(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1286 = f142(v0, v1);
    match m1286 {
        None => {
            false
        }
        Some(v3) => {
            f190(v3, v2)
        }
    }
}

pub fn f279() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f280(v0: Bytes, p1353: u64, p1354: u64, p1355: bool, p1356: Bytes) -> R<Bytes> {
    let m1352 = (p1353, (p1354, (p1355, p1356)));
    {
        let mut m1352 = m1352;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1352;
                let m1287 = v2;
                if m1287 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1287 - 1;
                    let m1290 = {
                        let a1288 = v0.clone();
                        let a1289 = v1;
                        index_bytes(a1288, a1289)
                    };
                    match m1290 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1335 = {
                                    let a1311 = {
                                        let a1299 = {
                                            let a1293 = {
                                                let a1291 = v7;
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
                                                let a1295 = v7;
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
                                                let a1301 = v7;
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
                                                let a1305 = v7;
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
                                                let a1313 = v7;
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
                                                let a1317 = v7;
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
                                                let a1323 = v7;
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
                                                let a1327 = v7;
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
                                nat_add(a1335, a1336)?
                            };
                            if f323(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1337 = v1;
                                    let a1338 = 1u64;
                                    nat_add(a1337, a1338)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f324(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1339 = v1;
                                        let a1340 = 1u64;
                                        nat_add(a1339, a1340)?
                                    }, (v5, (false, {
                                        let a1344 = v4;
                                        let a1345 = f32({
                                            let a1341 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1341)
                                        }, {
                                            let a1342 = v8;
                                            let a1343 = 97u64;
                                            nat_sub(a1342, a1343)
                                        }, 1u64);
                                        append_bytes(a1344, a1345)
                                    }))))
                                } else {
                                    if if f324(v8) {
                                        true
                                    } else {
                                        if f325(v8) {
                                            true
                                        } else {
                                            f326(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1346 = v1;
                                            let a1347 = 1u64;
                                            nat_add(a1346, a1347)?
                                        }, (v5, (false, {
                                            let a1348 = v4;
                                            let a1349 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1348, a1349)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1350 = v1;
                                            let a1351 = 1u64;
                                            nat_add(a1350, a1351)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1352 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1357 = f327(4096u64, f328(f329(f330(f142(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt23::C0(v2, v0), List::<Adt23>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m1357 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f202(f175(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f282(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1358 = v0;
    match m1358.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f145(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f331(v5, v1, v2))
            } else {
                List::cons(v4, f282(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f283(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f202(f332(f82(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f284(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1359 = 0u64;
        let a1360 = v1;
        nat_lt(a1359, a1360)
    } else {
        false
    } {
        let a1361 = Bytes::lit(&[0x2d]);
        let a1362 = f333(v1)?;
        Ok(append_bytes(a1361, a1362))
    } else {
        f333(v1)
    }
}

pub fn f285(v0: Bytes) -> R<Bytes> {
    let a1368 = {
        let a1366 = Bytes::lit(&[0x22]);
        let a1367 = {
            let (_, h1363) = {
                let (h1364, _) = f334(Fn9::F335(v0.clone()), {
                    let a1365 = v0;
                    length_bytes(a1365)
                }, (0u64, Bytes::lit(&[])))?;
                h1364
            };
            h1363
        };
        append_bytes(a1366, a1367)
    };
    let a1369 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1368, a1369))
}

pub fn f286(v0: List<Bytes>) -> Bytes {
    let a1372 = {
        let a1370 = Bytes::lit(&[0x5b]);
        let a1371 = f120(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1370, a1371)
    };
    let a1373 = Bytes::lit(&[0x5d]);
    append_bytes(a1372, a1373)
}

pub fn f287(v0: Adt14) -> R<List<Bytes>> {
    let m1374 = v0;
    match m1374 {
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
        Adt14::C6(r1375, r1376) => {
            let v7 = (*r1375).clone();
            let v8 = (*r1376).clone();
            Ok(List::cons(f219(v7)?, f287(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f288(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f336(Fn10::F337, List::<(Bytes, Bytes)>::nil(), f338(v0)?);
    let v2: List<Bytes> = f339(Fn11::F340, List::<Bytes>::nil(), v1)?;
    let a1379 = {
        let a1377 = Bytes::lit(&[0x7b]);
        let a1378 = f120(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1377, a1378)
    };
    let a1380 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1379, a1380))
}

pub fn f289(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1381 = v0;
    match m1381 {
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
        Adt14::C7(v9, r1382, r1383) => {
            let v10 = (*r1382).clone();
            let v11 = (*r1383).clone();
            Ok(List::cons((v9, f219(v10)?), f289(v11)?))
        }
    }
}

pub fn f290(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1384 = v0;
    match m1384 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f145(v2, v1)
        }
    }
}

pub fn f291(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f202(f341(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f292(v0: Adt16, v1: Option<u64>) -> bool {
    let m1385 = v1;
    match m1385 {
        None => {
            true
        }
        Some(v2) => {
            f236(v2, match v0 {
                Adt16::C0(_, h1386, _, _) => {
                    h1386
                }
            })
        }
    }
}

pub fn f293(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1388 = match v0 {
            Adt12::C0(_, _, h1387, _, _) => {
                h1387
            }
        };
        length_list(a1388)
    })
}

pub fn f294(v0: Bytes) -> Option<Adt20> {
    if f36(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt20::C0)
    } else {
        if f36(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt20::C1)
        } else {
            if f36(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt20::C2)
            } else {
                None::<Adt20>
            }
        }
    }
}

pub fn f295(v0: Bytes, p1393: List<Bytes>, p1394: u64) -> R<Option<u64>> {
    let m1392 = (p1393, p1394);
    {
        let mut m1392 = m1392;
        loop {
            let step = {
                let (v1, v2) = m1392;
                let m1389 = v1;
                match m1389.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1390 = v2;
                                let a1391 = 1u64;
                                nat_add(a1390, a1391)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: Option<Adt14>) -> Option<u64> {
    let m1395 = v0;
    match m1395 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1396 = v1;
            match m1396 {
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

pub fn f297(v0: Fn5, p1400: List<Bytes>, p1401: List<(Bytes, ())>) -> List<Bytes> {
    let m1399 = (p1400, p1401);
    {
        let mut m1399 = m1399;
        loop {
            let step = {
                let (v1, v2) = m1399;
                let m1397 = v2;
                match m1397.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1398: Fn5 = v0.clone();
                            c1398.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1402, _) = v1;
        h1402
    }, v0)
}

pub fn f299(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f342(Fn12::F343, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f300(v0: Fn6, p1406: List<List<(Bytes, ())>>, p1407: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1405 = (p1406, p1407);
    {
        let mut m1405 = m1405;
        loop {
            let step = {
                let (v1, v2) = m1405;
                let m1403 = v2;
                match m1403.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1404: Fn6 = v0.clone();
                            c1404.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f302(v0: Fn7, p1412: u64, p1413: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1411 = (p1412, p1413);
    {
        let mut m1411 = m1411;
        loop {
            let step = {
                let (v1, v2) = m1411;
                let m1408 = v1;
                if m1408 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1408 - 1;
                    let m1410 = {
                        let c1409: Fn7 = v0.clone();
                        c1409.apply(v2.clone())?
                    };
                    match m1410 {
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
                Ok(next) => { m1411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f303(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f344(v0)
}

pub fn f304(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f345(Fn13::F346, List::<(Bytes, ())>::nil(), v0)
}

pub fn f305(v0: Fn8, p1417: List<(Bytes, ())>, p1418: List<Bytes>) -> List<(Bytes, ())> {
    let m1416 = (p1417, p1418);
    {
        let mut m1416 = m1416;
        loop {
            let step = {
                let (v1, v2) = m1416;
                let m1414 = v2;
                match m1414.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1415: Fn8 = v0.clone();
                            c1415.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1416 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f307(v0: Bytes) -> Bytes {
    if f129(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1420 = {
                let a1419 = v0;
                length_bytes(a1419)
            };
            let a1421 = 1u64;
            nat_sub(a1420, a1421)
        })
    } else {
        v0
    }
}

pub fn f308(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f82(f69(v0.clone(), v1.clone()))?;
    let m1422 = f147(v2.clone())?;
    match m1422 {
        None => {
            f348(v0, v1, v2)
        }
        Some(v3) => {
            f347(v0, v1, v3)
        }
    }
}

pub fn f309(v0: List<Bytes>) -> List<Bytes> {
    let m1423 = v0;
    match m1423.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1425 = {
                    let a1424 = v1.clone();
                    length_bytes(a1424)
                };
                let a1426 = 0u64;
                if nat_eq(a1425, a1426) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f309(v2))
        }
    }
}

pub fn f310(v0: List<Bytes>) -> Bytes {
    let m1427 = v0;
    match m1427.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f311(v0: (Bytes, Bytes), p1430: List<(Bytes, Bytes)>) -> bool {
    let m1429 = p1430;
    {
        let mut m1429 = m1429;
        loop {
            let step = {
                let v1 = m1429;
                let m1428 = v1;
                match m1428.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f349(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1431 = v1;
    match m1431.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1432, _) = v2;
                h1432
            }, v0.clone()) {
                List::cons({
                    let (_, h1433) = v2;
                    h1433
                }, f312(v0, v3))
            } else {
                f312(v0, v3)
            }
        }
    }
}

pub fn f313(v0: u32) -> Bytes {
    let a1436 = {
        let a1434 = v0;
        let a1435 = 8u32;
        fixed_u32::bit_and(a1434, a1435)
    };
    let a1437 = 0u32;
    if equal(a1436, a1437) {
        let a1440 = {
            let a1438 = v0;
            let a1439 = 4u32;
            fixed_u32::bit_and(a1438, a1439)
        };
        let a1441 = 0u32;
        if equal(a1440, a1441) {
            let a1444 = {
                let a1442 = v0;
                let a1443 = 2u32;
                fixed_u32::bit_and(a1442, a1443)
            };
            let a1445 = 0u32;
            if equal(a1444, a1445) {
                let a1448 = {
                    let a1446 = v0;
                    let a1447 = 1u32;
                    fixed_u32::bit_and(a1446, a1447)
                };
                let a1449 = 0u32;
                if equal(a1448, a1449) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1452 = {
                    let a1450 = v0;
                    let a1451 = 1u32;
                    fixed_u32::bit_and(a1450, a1451)
                };
                let a1453 = 0u32;
                if equal(a1452, a1453) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1456 = {
                let a1454 = v0;
                let a1455 = 2u32;
                fixed_u32::bit_and(a1454, a1455)
            };
            let a1457 = 0u32;
            if equal(a1456, a1457) {
                let a1460 = {
                    let a1458 = v0;
                    let a1459 = 1u32;
                    fixed_u32::bit_and(a1458, a1459)
                };
                let a1461 = 0u32;
                if equal(a1460, a1461) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1464 = {
                    let a1462 = v0;
                    let a1463 = 1u32;
                    fixed_u32::bit_and(a1462, a1463)
                };
                let a1465 = 0u32;
                if equal(a1464, a1465) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1468 = {
            let a1466 = v0;
            let a1467 = 4u32;
            fixed_u32::bit_and(a1466, a1467)
        };
        let a1469 = 0u32;
        if equal(a1468, a1469) {
            let a1472 = {
                let a1470 = v0;
                let a1471 = 2u32;
                fixed_u32::bit_and(a1470, a1471)
            };
            let a1473 = 0u32;
            if equal(a1472, a1473) {
                let a1476 = {
                    let a1474 = v0;
                    let a1475 = 1u32;
                    fixed_u32::bit_and(a1474, a1475)
                };
                let a1477 = 0u32;
                if equal(a1476, a1477) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1480 = {
                    let a1478 = v0;
                    let a1479 = 1u32;
                    fixed_u32::bit_and(a1478, a1479)
                };
                let a1481 = 0u32;
                if equal(a1480, a1481) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1484 = {
                let a1482 = v0;
                let a1483 = 2u32;
                fixed_u32::bit_and(a1482, a1483)
            };
            let a1485 = 0u32;
            if equal(a1484, a1485) {
                let a1488 = {
                    let a1486 = v0;
                    let a1487 = 1u32;
                    fixed_u32::bit_and(a1486, a1487)
                };
                let a1489 = 0u32;
                if equal(a1488, a1489) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1492 = {
                    let a1490 = v0;
                    let a1491 = 1u32;
                    fixed_u32::bit_and(a1490, a1491)
                };
                let a1493 = 0u32;
                if equal(a1492, a1493) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f314(v0: u32, v1: u32) -> u32 {
    let m1496 = {
        let a1494 = v0;
        let a1495 = v1;
        fixed_u32::shift_right(a1494, a1495)
    };
    match m1496 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f315(v0: Adt22, v1: Adt24) -> R<Adt22> {
    let v2: Adt25 = f350(Fn14::F351, 64u64, Adt25::C0(match v0.clone() {
        Adt22::C0(h1497, _, _, _, _, _, _, _) => {
            h1497
        }
    }, match v0.clone() {
        Adt22::C0(_, h1498, _, _, _, _, _, _) => {
            h1498
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h1499, _, _, _, _, _) => {
            h1499
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h1500, _, _, _, _) => {
            h1500
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h1501, _, _, _) => {
            h1501
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1502, _, _) => {
            h1502
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1503, _) => {
            h1503
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h1504) => {
            h1504
        }
    }, v1, 0u64))?;
    Ok(Adt22::C0(f352(match v0.clone() {
        Adt22::C0(h1505, _, _, _, _, _, _, _) => {
            h1505
        }
    }, match v2.clone() {
        Adt25::C0(h1506, _, _, _, _, _, _, _, _, _) => {
            h1506
        }
    }), f352(match v0.clone() {
        Adt22::C0(_, h1507, _, _, _, _, _, _) => {
            h1507
        }
    }, match v2.clone() {
        Adt25::C0(_, h1508, _, _, _, _, _, _, _, _) => {
            h1508
        }
    }), f352(match v0.clone() {
        Adt22::C0(_, _, h1509, _, _, _, _, _) => {
            h1509
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h1510, _, _, _, _, _, _, _) => {
            h1510
        }
    }), f352(match v0.clone() {
        Adt22::C0(_, _, _, h1511, _, _, _, _) => {
            h1511
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h1512, _, _, _, _, _, _) => {
            h1512
        }
    }), f352(match v0.clone() {
        Adt22::C0(_, _, _, _, h1513, _, _, _) => {
            h1513
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h1514, _, _, _, _, _) => {
            h1514
        }
    }), f352(match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1515, _, _) => {
            h1515
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h1516, _, _, _, _) => {
            h1516
        }
    }), f352(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1517, _) => {
            h1517
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h1518, _, _, _) => {
            h1518
        }
    }), f352(match v0 {
        Adt22::C0(_, _, _, _, _, _, _, h1519) => {
            h1519
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h1520, _, _) => {
            h1520
        }
    })))
}

pub fn f316(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f353(v0.clone(), v1, v2, v3)?, f353(v0.clone(), v1, v2, {
        let a1521 = v3;
        let a1522 = 4u64;
        nat_add(a1521, a1522)?
    })?, f353(v0.clone(), v1, v2, {
        let a1523 = v3;
        let a1524 = 8u64;
        nat_add(a1523, a1524)?
    })?, f353(v0.clone(), v1, v2, {
        let a1525 = v3;
        let a1526 = 12u64;
        nat_add(a1525, a1526)?
    })?, f353(v0.clone(), v1, v2, {
        let a1527 = v3;
        let a1528 = 16u64;
        nat_add(a1527, a1528)?
    })?, f353(v0.clone(), v1, v2, {
        let a1529 = v3;
        let a1530 = 20u64;
        nat_add(a1529, a1530)?
    })?, f353(v0.clone(), v1, v2, {
        let a1531 = v3;
        let a1532 = 24u64;
        nat_add(a1531, a1532)?
    })?, f353(v0.clone(), v1, v2, {
        let a1533 = v3;
        let a1534 = 28u64;
        nat_add(a1533, a1534)?
    })?, f353(v0.clone(), v1, v2, {
        let a1535 = v3;
        let a1536 = 32u64;
        nat_add(a1535, a1536)?
    })?, f353(v0.clone(), v1, v2, {
        let a1537 = v3;
        let a1538 = 36u64;
        nat_add(a1537, a1538)?
    })?, f353(v0.clone(), v1, v2, {
        let a1539 = v3;
        let a1540 = 40u64;
        nat_add(a1539, a1540)?
    })?, f353(v0.clone(), v1, v2, {
        let a1541 = v3;
        let a1542 = 44u64;
        nat_add(a1541, a1542)?
    })?, f353(v0.clone(), v1, v2, {
        let a1543 = v3;
        let a1544 = 48u64;
        nat_add(a1543, a1544)?
    })?, f353(v0.clone(), v1, v2, {
        let a1545 = v3;
        let a1546 = 52u64;
        nat_add(a1545, a1546)?
    })?, f353(v0.clone(), v1, v2, {
        let a1547 = v3;
        let a1548 = 56u64;
        nat_add(a1547, a1548)?
    })?, f353(v0, v1, v2, {
        let a1549 = v3;
        let a1550 = 60u64;
        nat_add(a1549, a1550)?
    })?))
}

pub fn f317(v0: Bytes, v1: u64) -> Option<u64> {
    let a1551 = 0u64;
    let a1552 = v1;
    if nat_lt(a1551, a1552) {
        if f354(v0, {
            let a1553 = v1;
            let a1554 = 1u64;
            nat_sub(a1553, a1554)
        }) {
            Some({
                let a1555 = v1;
                let a1556 = 1u64;
                nat_sub(a1555, a1556)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f318(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1557 = v2;
    let a1558 = v0;
    if nat_lt(a1557, a1558) {
        if f354(v1, v2) {
            Ok(Some({
                let a1559 = v2;
                let a1560 = 1u64;
                nat_add(a1559, a1560)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f319(v0: Bytes, p1640: u64, p1641: u64, p1642: bool, p1643: bool, p1644: Bytes) -> R<Bytes> {
    let m1639 = (p1640, (p1641, (p1642, (p1643, p1644))));
    {
        let mut m1639 = m1639;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1639;
                let m1561 = v2;
                if m1561 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1561 - 1;
                    let m1564 = {
                        let a1562 = v0.clone();
                        let a1563 = v1;
                        index_bytes(a1562, a1563)
                    };
                    match m1564 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1609 = {
                                    let a1585 = {
                                        let a1573 = {
                                            let a1567 = {
                                                let a1565 = v8;
                                                let a1566 = 128u8;
                                                fixed_u8::bit_and(a1565, a1566)
                                            };
                                            let a1568 = 128u8;
                                            if equal(a1567, a1568) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1574 = {
                                            let a1571 = {
                                                let a1569 = v8;
                                                let a1570 = 64u8;
                                                fixed_u8::bit_and(a1569, a1570)
                                            };
                                            let a1572 = 64u8;
                                            if equal(a1571, a1572) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1573, a1574)?
                                    };
                                    let a1586 = {
                                        let a1583 = {
                                            let a1577 = {
                                                let a1575 = v8;
                                                let a1576 = 32u8;
                                                fixed_u8::bit_and(a1575, a1576)
                                            };
                                            let a1578 = 32u8;
                                            if equal(a1577, a1578) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1584 = {
                                            let a1581 = {
                                                let a1579 = v8;
                                                let a1580 = 16u8;
                                                fixed_u8::bit_and(a1579, a1580)
                                            };
                                            let a1582 = 16u8;
                                            if equal(a1581, a1582) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1583, a1584)?
                                    };
                                    nat_add(a1585, a1586)?
                                };
                                let a1610 = {
                                    let a1607 = {
                                        let a1595 = {
                                            let a1589 = {
                                                let a1587 = v8;
                                                let a1588 = 8u8;
                                                fixed_u8::bit_and(a1587, a1588)
                                            };
                                            let a1590 = 8u8;
                                            if equal(a1589, a1590) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1596 = {
                                            let a1593 = {
                                                let a1591 = v8;
                                                let a1592 = 4u8;
                                                fixed_u8::bit_and(a1591, a1592)
                                            };
                                            let a1594 = 4u8;
                                            if equal(a1593, a1594) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1595, a1596)?
                                    };
                                    let a1608 = {
                                        let a1605 = {
                                            let a1599 = {
                                                let a1597 = v8;
                                                let a1598 = 2u8;
                                                fixed_u8::bit_and(a1597, a1598)
                                            };
                                            let a1600 = 2u8;
                                            if equal(a1599, a1600) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1606 = {
                                            let a1603 = {
                                                let a1601 = v8;
                                                let a1602 = 1u8;
                                                fixed_u8::bit_and(a1601, a1602)
                                            };
                                            let a1604 = 1u8;
                                            if equal(a1603, a1604) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1605, a1606)?
                                    };
                                    nat_add(a1607, a1608)?
                                };
                                nat_add(a1609, a1610)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1611 = v1;
                                    let a1612 = 1u64;
                                    nat_add(a1611, a1612)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1613 = v9;
                                    let a1614 = 34u64;
                                    !nat_eq(a1613, a1614)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1615 = v9;
                                    let a1616 = 92u64;
                                    nat_eq(a1615, a1616)
                                }, {
                                    let a1617 = v5;
                                    let a1618 = v10;
                                    append_bytes(a1617, a1618)
                                })))))
                            } else {
                                let a1619 = v9;
                                let a1620 = 32u64;
                                if if if if nat_eq(a1619, a1620) {
                                    true
                                } else {
                                    let a1621 = v9;
                                    let a1622 = 9u64;
                                    nat_eq(a1621, a1622)
                                } {
                                    true
                                } else {
                                    let a1623 = v9;
                                    let a1624 = 10u64;
                                    nat_eq(a1623, a1624)
                                } {
                                    true
                                } else {
                                    let a1625 = v9;
                                    let a1626 = 13u64;
                                    nat_eq(a1625, a1626)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1627 = v1;
                                        let a1628 = 1u64;
                                        nat_add(a1627, a1628)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1629 = v1;
                                            let a1630 = 4u64;
                                            nat_add(a1629, a1630)?
                                        }, (v6, (false, (false, {
                                            let a1631 = v5;
                                            let a1632 = f8(Str::lit("[]"));
                                            append_bytes(a1631, a1632)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1633 = v1;
                                            let a1634 = 1u64;
                                            nat_add(a1633, a1634)?
                                        }, (v6, ({
                                            let a1635 = v9;
                                            let a1636 = 34u64;
                                            nat_eq(a1635, a1636)
                                        }, (false, {
                                            let a1637 = v5;
                                            let a1638 = v10;
                                            append_bytes(a1637, a1638)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f320(v0: Bytes, v1: u64) -> R<u64> {
    let m1647 = {
        let a1645 = v0;
        let a1646 = v1;
        index_bytes(a1645, a1646)
    };
    match m1647 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1692 = {
                let a1668 = {
                    let a1656 = {
                        let a1650 = {
                            let a1648 = v3;
                            let a1649 = 128u8;
                            fixed_u8::bit_and(a1648, a1649)
                        };
                        let a1651 = 128u8;
                        if equal(a1650, a1651) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1657 = {
                        let a1654 = {
                            let a1652 = v3;
                            let a1653 = 64u8;
                            fixed_u8::bit_and(a1652, a1653)
                        };
                        let a1655 = 64u8;
                        if equal(a1654, a1655) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1656, a1657)?
                };
                let a1669 = {
                    let a1666 = {
                        let a1660 = {
                            let a1658 = v3;
                            let a1659 = 32u8;
                            fixed_u8::bit_and(a1658, a1659)
                        };
                        let a1661 = 32u8;
                        if equal(a1660, a1661) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1667 = {
                        let a1664 = {
                            let a1662 = v3;
                            let a1663 = 16u8;
                            fixed_u8::bit_and(a1662, a1663)
                        };
                        let a1665 = 16u8;
                        if equal(a1664, a1665) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1666, a1667)?
                };
                nat_add(a1668, a1669)?
            };
            let a1693 = {
                let a1690 = {
                    let a1678 = {
                        let a1672 = {
                            let a1670 = v3;
                            let a1671 = 8u8;
                            fixed_u8::bit_and(a1670, a1671)
                        };
                        let a1673 = 8u8;
                        if equal(a1672, a1673) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1679 = {
                        let a1676 = {
                            let a1674 = v3;
                            let a1675 = 4u8;
                            fixed_u8::bit_and(a1674, a1675)
                        };
                        let a1677 = 4u8;
                        if equal(a1676, a1677) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1678, a1679)?
                };
                let a1691 = {
                    let a1688 = {
                        let a1682 = {
                            let a1680 = v3;
                            let a1681 = 2u8;
                            fixed_u8::bit_and(a1680, a1681)
                        };
                        let a1683 = 2u8;
                        if equal(a1682, a1683) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1689 = {
                        let a1686 = {
                            let a1684 = v3;
                            let a1685 = 1u8;
                            fixed_u8::bit_and(a1684, a1685)
                        };
                        let a1687 = 1u8;
                        if equal(a1686, a1687) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1688, a1689)?
                };
                nat_add(a1690, a1691)?
            };
            nat_add(a1692, a1693)
        }
    }
}

pub fn f321(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f355(Fn15::F356(v0.clone()), {
        let a1695 = {
            let a1694 = v0;
            length_bytes(a1694)
        };
        let a1696 = 1u64;
        nat_add(a1695, a1696)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1700 = {
        let (_, h1697) = {
            let (_, h1698) = {
                let (h1699, _) = v2.clone();
                h1699
            };
            h1698
        };
        h1697
    };
    let a1701 = 1u64;
    if nat_eq(a1700, a1701) {
        Ok(Some(({
            let (h1702, _) = {
                let (h1703, _) = v2.clone();
                h1703
            };
            h1702
        }, {
            let (h1704, _) = {
                let (_, h1705) = {
                    let (h1706, _) = v2;
                    h1706
                };
                h1705
            };
            h1704
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f322(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1707 = f320(v0.clone(), v1)?;
        let a1708 = 45u64;
        nat_eq(a1707, a1708)
    };
    let v3: u64 = if v2 {
        let a1709 = v1;
        let a1710 = 1u64;
        nat_add(a1709, a1710)?
    } else {
        v1
    };
    let v4: u64 = f357(v0.clone(), v3)?;
    let a1713 = 19u64;
    let a1714 = {
        let a1711 = v4;
        let a1712 = v3;
        nat_sub(a1711, a1712)
    };
    if nat_lt(a1713, a1714) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1717 = f358(f32(v0, v3, {
            let a1715 = v4;
            let a1716 = v3;
            nat_sub(a1715, a1716)
        }))?;
        match m1717 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f359(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f323(v0: u64) -> bool {
    let a1718 = v0;
    let a1719 = 95u64;
    if nat_eq(a1718, a1719) {
        true
    } else {
        let a1720 = v0;
        let a1721 = 45u64;
        if nat_eq(a1720, a1721) {
            true
        } else {
            let a1722 = v0;
            let a1723 = 32u64;
            if nat_eq(a1722, a1723) {
                true
            } else {
                let a1724 = v0;
                let a1725 = 46u64;
                if nat_eq(a1724, a1725) {
                    true
                } else {
                    let a1726 = v0;
                    let a1727 = 47u64;
                    nat_eq(a1726, a1727)
                }
            }
        }
    }
}

pub fn f324(v0: u64) -> bool {
    let a1728 = 97u64;
    let a1729 = v0;
    if nat_le(a1728, a1729) {
        let a1730 = v0;
        let a1731 = 122u64;
        nat_le(a1730, a1731)
    } else {
        false
    }
}

pub fn f325(v0: u64) -> bool {
    let a1732 = 65u64;
    let a1733 = v0;
    if nat_le(a1732, a1733) {
        let a1734 = v0;
        let a1735 = 90u64;
        nat_le(a1734, a1735)
    } else {
        false
    }
}

pub fn f326(v0: u64) -> bool {
    let a1736 = 48u64;
    let a1737 = v0;
    if nat_le(a1736, a1737) {
        let a1738 = v0;
        let a1739 = 57u64;
        nat_le(a1738, a1739)
    } else {
        false
    }
}

pub fn f327(p1747: u64, p1748: List<Adt23>, p1749: List<Bytes>, p1750: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1746 = (p1747, (p1748, (p1749, p1750)));
    {
        let mut m1746 = m1746;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1746;
                let m1740 = v0;
                if m1740 == 0 {
                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1740 - 1;
                    let m1741 = v1;
                    match m1741.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1742 = v5;
                            match m1742 {
                                Adt23::C0(v8, v9) => {
                                    if f36(v8.clone(), f360()) {
                                        Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f85(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1743 = f362(v8.clone(), v9)?;
                                            match m1743 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f328({
                                                        let (_, h1744) = v11;
                                                        h1744
                                                    }, List::cons(Adt23::C1({
                                                        let (h1745, _) = v11;
                                                        h1745
                                                    }), v6)), (List::cons(v8, v2), v3))))
                                                }
                                                Err(v10) => {
                                                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt23::C1(v7) => {
                                    Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                                Adt23::C2 => {
                                    if f85(f360(), v2.clone()) {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f360(), v2), List::cons(f361(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(v0: List<Adt23>, v1: List<Adt23>) -> List<Adt23> {
    let m1751 = v0;
    match m1751.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f328(v3, v1))
        }
    }
}

pub fn f329(v0: List<(Bytes, Adt14)>) -> R<List<Adt23>> {
    let m1752 = v0;
    match m1752.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt23::C0(f216({
                let (h1753, _) = v1;
                h1753
            })?, {
                let (_, h1754) = v1;
                h1754
            }), f329(v2)?))
        }
    }
}

pub fn f330(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1755 = v0;
    match m1755 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f363(v1)
        }
    }
}

pub fn f331(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1756 = v0;
    match m1756.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f145(v3.clone(), v1.clone()), v2.clone()) {
                f331(v4, v1, v2)
            } else {
                List::cons(v3, f331(v4, v1, v2))
            }
        }
    }
}

pub fn f332(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1757 = v0;
    match m1757.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f192(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f202(f341(f77(List::cons(v1, f364(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f332(v3, v1)?))
            }
        }
    }
}

pub fn f333(v0: u64) -> R<Bytes> {
    f365(f366(), 10u64, v0)
}

pub fn f334(v0: Fn9, p1762: u64, p1763: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1761 = (p1762, p1763);
    {
        let mut m1761 = m1761;
        loop {
            let step = {
                let (v1, v2) = m1761;
                let m1758 = v1;
                if m1758 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1758 - 1;
                    let m1760 = {
                        let c1759: Fn9 = v0.clone();
                        c1759.apply(v2.clone())?
                    };
                    match m1760 {
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
                Ok(next) => { m1761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f367(v0, v1)
}

pub fn f336(v0: Fn10, p1767: List<(Bytes, Bytes)>, p1768: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1766 = (p1767, p1768);
    {
        let mut m1766 = m1766;
        loop {
            let step = {
                let (v1, v2) = m1766;
                let m1764 = v2;
                match m1764.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1765: Fn10 = v0.clone();
                            c1765.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1766 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f368(v0, v1)
}

pub fn f338(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f369(v0)
}

pub fn f339(v0: Fn11, p1772: List<Bytes>, p1773: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1771 = (p1772, p1773);
    {
        let mut m1771 = m1771;
        loop {
            let step = {
                let (v1, v2) = m1771;
                let m1769 = v2;
                match m1769.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1770: Fn11 = v0.clone();
                            c1770.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f340(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f370(v1)?, v0))
}

pub fn f341(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1774 = v0;
    match m1774.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f197(v1)?, f341(v2)?))
        }
    }
}

pub fn f342(v0: Fn12, p1778: List<List<(Bytes, ())>>, p1779: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1777 = (p1778, p1779);
    {
        let mut m1777 = m1777;
        loop {
            let step = {
                let (v1, v2) = m1777;
                let m1775 = v2;
                match m1775.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1776: Fn12 = v0.clone();
                            c1776.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1777 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f344(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1781 = {
        let a1780 = v0.clone();
        length_list(a1780)
    };
    let a1782 = 1u64;
    if nat_le(a1781, a1782) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f371(v0)?))
    }
}

pub fn f345(v0: Fn13, p1786: List<(Bytes, ())>, p1787: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1785 = (p1786, p1787);
    {
        let mut m1785 = m1785;
        loop {
            let step = {
                let (v1, v2) = m1785;
                let m1783 = v2;
                match m1783.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1784: Fn13 = v0.clone();
                            c1784.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f347(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f71(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f372(v3.clone(), f71(f72(v0, f88())?, f8(Str::lit("at"))));
    let a1789 = {
        let a1788 = v4.clone();
        length_list(a1788)
    };
    let a1790 = 0u64;
    if nat_eq(a1789, a1790) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1793 = {
            let a1791 = v4;
            length_list(a1791)
        };
        let a1794 = {
            let a1792 = v3;
            length_list(a1792)
        };
        if nat_eq(a1793, a1794) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f373(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f348(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f212(f135(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f212(f135(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f63(f71(v3.clone(), f8(Str::lit("id"))), f71(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f71(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f372(v5.clone(), f63(v6.clone(), f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1796 = {
        let a1795 = v7.clone();
        length_list(a1795)
    };
    let a1797 = 0u64;
    if nat_eq(a1796, a1797) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f74({
            let a1800 = {
                let a1798 = v7;
                length_list(a1798)
            };
            let a1801 = {
                let a1799 = v5;
                length_list(a1799)
            };
            nat_eq(a1800, a1801)
        }) {
            Ok((false, f373(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f77(f63(f372(f374(v4.clone()), f68(f69(v0, f8(Str::lit("model/authorities.toml"))))?), f372(f375(v4), f63(v6, f71(v3, f8(Str::lit("id")))))))?;
            let a1803 = {
                let a1802 = v8.clone();
                length_list(a1802)
            };
            let a1804 = 0u64;
            if nat_eq(a1803, a1804) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f373(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f120(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f349(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h1805, _) = v0;
        h1805
    }, {
        let (h1806, _) = v1;
        h1806
    }) {
        f36({
            let (_, h1807) = v0;
            h1807
        }, {
            let (_, h1808) = v1;
            h1808
        })
    } else {
        false
    }
}

pub fn f350(v0: Fn14, p1812: u64, p1813: Adt25) -> R<Adt25> {
    let m1811 = (p1812, p1813);
    {
        let mut m1811 = m1811;
        loop {
            let step = {
                let (v1, v2) = m1811;
                let m1809 = v1;
                if m1809 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m1809 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c1810: Fn14 = v0.clone();
                        c1810.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1811 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(v0: Adt25) -> R<Adt25> {
    f376(v0)
}

pub fn f352(v0: u32, v1: u32) -> u32 {
    let m1816 = {
        let a1814 = f377(v0);
        let a1815 = f377(v1);
        fixed_u64::checked_add(a1814, a1815)
    };
    match m1816 {
        None => {
            0u32
        }
        Some(v2) => {
            f378(v2)
        }
    }
}

pub fn f353(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1819 = {
        let a1817 = v3;
        let a1818 = 8u64;
        nat_add(a1817, a1818)?
    };
    let a1820 = v2;
    if nat_eq(a1819, a1820) {
        Ok(f379({
            let a1823 = {
                let a1821 = v1;
                let a1822 = 8u64;
                nat_mul(a1821, a1822)?
            };
            let a1824 = 4294967296u64;
            let a1825 = 0u64;
            nat_quot(a1823, a1824, a1825)
        }))
    } else {
        let a1828 = {
            let a1826 = v3;
            let a1827 = 4u64;
            nat_add(a1826, a1827)?
        };
        let a1829 = v2;
        if nat_eq(a1828, a1829) {
            Ok(f379({
                let a1832 = {
                    let a1830 = v1;
                    let a1831 = 8u64;
                    nat_mul(a1830, a1831)?
                };
                let a1833 = 4294967296u64;
                let a1834 = 0u64;
                nat_rem(a1832, a1833, a1834)
            }))
        } else {
            Ok(f380(f381(v0.clone(), v1, v3), f381(v0.clone(), v1, {
                let a1835 = v3;
                let a1836 = 1u64;
                nat_add(a1835, a1836)?
            }), f381(v0.clone(), v1, {
                let a1837 = v3;
                let a1838 = 2u64;
                nat_add(a1837, a1838)?
            }), f381(v0, v1, {
                let a1839 = v3;
                let a1840 = 3u64;
                nat_add(a1839, a1840)?
            })))
        }
    }
}

pub fn f354(v0: Bytes, v1: u64) -> bool {
    let m1841 = f224(v0, v1);
    match m1841 {
        None => {
            false
        }
        Some(v2) => {
            f382(v2)
        }
    }
}

pub fn f355(v0: Fn15, p1846: u64, p1847: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1845 = (p1846, p1847);
    {
        let mut m1845 = m1845;
        loop {
            let step = {
                let (v1, v2) = m1845;
                let m1842 = v1;
                if m1842 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1842 - 1;
                    let m1844 = {
                        let c1843: Fn15 = v0.clone();
                        c1843.apply(v2.clone())?
                    };
                    match m1844 {
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
                Ok(next) => { m1845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f383(v0, v1)
}

pub fn f357(v0: Bytes, v1: u64) -> R<u64> {
    let (h1848, _) = f226(Fn3::F384(v0.clone()), {
        let a1850 = {
            let a1849 = v0;
            length_bytes(a1849)
        };
        let a1851 = 1u64;
        nat_add(a1850, a1851)?
    }, v1)?;
    Ok(h1848)
}

pub fn f358(v0: Bytes) -> R<Option<u64>> {
    if f385(v0.clone())? {
        Ok(Some({
            let (_, h1852) = {
                let (h1853, _) = f386(Fn16::F387(v0.clone()), {
                    let a1854 = v0;
                    length_bytes(a1854)
                }, (0u64, 0u64))?;
                h1853
            };
            h1852
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f359(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1855 = 0u64;
        let a1856 = v1;
        if nat_lt(a1855, a1856) {
            let a1857 = v1;
            let a1858 = 9223372036854775808u64;
            nat_le(a1857, a1858)
        } else {
            false
        }
    } else {
        let a1859 = v1;
        let a1860 = 9223372036854775807u64;
        nat_le(a1859, a1860)
    }
}

pub fn f360() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f361() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f362(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m1861 = f142(v1.clone(), f8(Str::lit("enum")));
    match m1861 {
        None => {
            let m1863 = f142(v1.clone(), f8(Str::lit("oneOf")));
            match m1863 {
                None => {
                    if f392(v1.clone(), v0.clone())? {
                        let m1864 = f393(v0.clone(), f330(f142(v1.clone(), f8(Str::lit("properties")))), f73(f146(f142(v1, f8(Str::lit("required"))))), true)?;
                        match m1864 {
                            Ok(v7) => {
                                Ok(f390(f394(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f202({
                                    let (h1865, _) = v7;
                                    h1865
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h1866) = v7;
                                    h1866
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt23>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m1867 = f393(v0.clone(), f330(f142(v1.clone(), f8(Str::lit("properties")))), f73(f146(f142(v1, f8(Str::lit("required"))))), false)?;
                        match m1867 {
                            Ok(v9) => {
                                Ok(f390(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                    let m1869 = {
                                        let (h1868, _) = v9.clone();
                                        h1868
                                    };
                                    match m1869.uncons() {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some((_, _)) => {
                                            f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f202({
                                                let (h1870, _) = v9;
                                                h1870
                                            }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                        }
                                    }
                                }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                    let (_, h1871) = v9;
                                    h1871
                                }))
                            }
                            Err(v8) => {
                                Ok(Err::<(Bytes, List<Adt23>), Bytes>(v8))
                            }
                        }
                    }
                }
                Some(v5) => {
                    f391(v0, f220(v5))
                }
            }
        }
        Some(v2) => {
            let m1862 = f388(f220(v2), List::<Bytes>::nil())?;
            match m1862 {
                Ok(v4) => {
                    Ok(f390(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f202(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt23>::nil()))
                }
                Err(v3) => {
                    Ok(f389(v3, v0))
                }
            }
        }
    }
}

pub fn f363(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1872 = v0;
    match m1872 {
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
        Adt14::C4(r1873) => {
            let v4 = (*r1873).clone();
            f363(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1874, r1875) => {
            let v2 = (*r1874).clone();
            let v3 = (*r1875).clone();
            List::cons((v1, v2), f363(v3))
        }
    }
}

pub fn f364(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1876 = v0;
    match m1876.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f364(v3, false))
            } else {
                f364(v3, true)
            }
        }
    }
}

pub fn f365(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1877 = v2;
    let a1878 = 0u64;
    if nat_eq(a1877, a1878) {
        Ok(f395(v0, 0u64))
    } else {
        let (_, h1879) = {
            let (h1880, _) = f334(Fn9::F396(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1880
        };
        Ok(h1879)
    }
}

pub fn f366() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f367(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1884 = {
        let a1882 = v0.clone();
        let a1883 = {
            let (h1881, _) = v1.clone();
            h1881
        };
        index_bytes(a1882, a1883)
    };
    match m1884 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1886 = {
                    let (h1885, _) = v1.clone();
                    h1885
                };
                let a1887 = 1u64;
                nat_add(a1886, a1887)?
            }, {
                let a1890 = {
                    let (_, h1888) = v1;
                    h1888
                };
                let a1891 = f397(v0, {
                    let (h1889, _) = v1;
                    h1889
                }, v2)?;
                append_bytes(a1890, a1891)
            })))
        }
    }
}

pub fn f368(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1892 = v0.clone();
    match m1892.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1895 = {
                let (h1893, _) = v2;
                h1893
            };
            let a1896 = {
                let (h1894, _) = v1.clone();
                h1894
            };
            if equal(a1895, a1896) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f369(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f398(f399(Fn17::F400, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1901 = {
        let (h1897, _) = f401(Fn18::F402, {
            let a1899 = {
                let a1898 = v0;
                length_list(a1898)
            };
            let a1900 = 1u64;
            nat_add(a1899, a1900)?
        }, v1)?;
        h1897
    };
    match m1901.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f370(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1906 = {
        let a1903 = f285({
            let (h1902, _) = v0;
            h1902
        })?;
        let a1904 = Bytes::lit(&[0x3a]);
        append_bytes(a1903, a1904)
    };
    let a1907 = {
        let (_, h1905) = v0;
        h1905
    };
    Ok(append_bytes(a1906, a1907))
}

pub fn f371(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f299({
        let (_, h1908) = {
            let (h1909, _) = f403(Fn19::F404, {
                let a1911 = {
                    let a1910 = v0.clone();
                    length_list(a1910)
                };
                let a1912 = 1u64;
                nat_add(a1911, a1912)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1909
        };
        h1908
    }))
}

pub fn f372(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1913 = v0;
    match m1913.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f372(v3, v1)
            } else {
                List::cons(v2, f372(v3, v1))
            }
        }
    }
}

pub fn f373(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f374(v0: List<Adt14>) -> List<Bytes> {
    let m1914 = v0;
    match m1914.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1915 = f142(v1, f8(Str::lit("std")));
            match m1915 {
                None => {
                    f374(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3, f8(Str::lit("authority"))), f374(v2))
                }
            }
        }
    }
}

pub fn f375(v0: List<Adt14>) -> List<Bytes> {
    let m1916 = v0;
    match m1916.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1917 = f150(f142(v1, f8(Str::lit("req"))));
            match m1917 {
                None => {
                    f375(v2)
                }
                Some(v3) => {
                    List::cons(v3, f375(v2))
                }
            }
        }
    }
}

pub fn f376(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f405(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h1918) => {
            h1918
        }
    });
    let v2: u32 = f352(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h1919, _, _) => {
            h1919
        }
    }, f352(f406(match v0.clone() {
        Adt25::C0(_, _, _, _, h1920, _, _, _, _, _) => {
            h1920
        }
    }), f352(f407(match v0.clone() {
        Adt25::C0(_, _, _, _, h1921, _, _, _, _, _) => {
            h1921
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1922, _, _, _, _) => {
            h1922
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1923, _, _, _) => {
            h1923
        }
    }), f352(v1, {
        let m1926 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h1925, _) => {
                h1925
            }
        };
        match m1926 {
            Adt24::C0(h1924, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1924
            }
        }
    }))));
    let v3: u32 = f352(f408(match v0.clone() {
        Adt25::C0(h1927, _, _, _, _, _, _, _, _, _) => {
            h1927
        }
    }), f409(match v0.clone() {
        Adt25::C0(h1928, _, _, _, _, _, _, _, _, _) => {
            h1928
        }
    }, match v0.clone() {
        Adt25::C0(_, h1929, _, _, _, _, _, _, _, _) => {
            h1929
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1930, _, _, _, _, _, _, _) => {
            h1930
        }
    }));
    Ok(Adt25::C0(f352(v2, v3), match v0.clone() {
        Adt25::C0(h1931, _, _, _, _, _, _, _, _, _) => {
            h1931
        }
    }, match v0.clone() {
        Adt25::C0(_, h1932, _, _, _, _, _, _, _, _) => {
            h1932
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1933, _, _, _, _, _, _, _) => {
            h1933
        }
    }, f352(match v0.clone() {
        Adt25::C0(_, _, _, h1934, _, _, _, _, _, _) => {
            h1934
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h1935, _, _, _, _, _) => {
            h1935
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1936, _, _, _, _) => {
            h1936
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1937, _, _, _) => {
            h1937
        }
    }, f410(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h1938, _) => {
            h1938
        }
    }), {
        let a1940 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h1939) => {
                h1939
            }
        };
        let a1941 = 1u64;
        nat_add(a1940, a1941)?
    }))
}

pub fn f377(v0: u32) -> u64 {
    let m1943 = {
        let a1942 = v0;
        fixed_u64::convert(i128::from(a1942))
    };
    match m1943 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f378(v0: u64) -> u32 {
    let m1947 = {
        let a1946 = {
            let a1944 = v0;
            let a1945 = 4294967295u64;
            fixed_u64::bit_and(a1944, a1945)
        };
        fixed_u32::convert(i128::from(a1946))
    };
    match m1947 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f379(v0: u64) -> u32 {
    let (_, h1948) = {
        let (_, h1949) = f411(Fn20::F412, 32u64, (v0, (1u32, 0u32)));
        h1949
    };
    h1948
}

pub fn f380(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1954 = f413(v0, 24u32);
    let a1955 = {
        let a1952 = f413(v1, 16u32);
        let a1953 = {
            let a1950 = f413(v2, 8u32);
            let a1951 = v3;
            fixed_u32::bit_or(a1950, a1951)
        };
        fixed_u32::bit_or(a1952, a1953)
    };
    fixed_u32::bit_or(a1954, a1955)
}

pub fn f381(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1956 = v2;
    let a1957 = v1;
    if nat_lt(a1956, a1957) {
        let m1960 = {
            let a1958 = v0;
            let a1959 = v2;
            index_bytes(a1958, a1959)
        };
        match m1960 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1962 = {
                    let a1961 = v3;
                    fixed_u32::convert(i128::from(a1961))
                };
                match m1962 {
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
        let a1963 = v2;
        let a1964 = v1;
        if nat_eq(a1963, a1964) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f382(v0: u8) -> bool {
    let a1965 = v0;
    let a1966 = 32u8;
    if if if equal(a1965, a1966) {
        true
    } else {
        let a1967 = v0;
        let a1968 = 9u8;
        equal(a1967, a1968)
    } {
        true
    } else {
        let a1969 = v0;
        let a1970 = 10u8;
        equal(a1969, a1970)
    } {
        true
    } else {
        let a1971 = v0;
        let a1972 = 12u8;
        if equal(a1971, a1972) {
            true
        } else {
            let a1973 = v0;
            let a1974 = 13u8;
            equal(a1973, a1974)
        }
    }
}

pub fn f383(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1975, _) = v1.clone();
        h1975
    };
    let v3: Bytes = {
        let (h1976, _) = {
            let (_, h1977) = v1.clone();
            h1977
        };
        h1976
    };
    let a1980 = {
        let (_, h1978) = {
            let (_, h1979) = v1;
            h1979
        };
        h1978
    };
    let a1981 = 0u64;
    if nat_eq(a1980, a1981) {
        let v4: u64 = f320(v0.clone(), v2)?;
        let a1982 = v4;
        let a1983 = 34u64;
        if nat_eq(a1982, a1983) {
            Ok(Some(({
                let a1984 = v2;
                let a1985 = 1u64;
                nat_add(a1984, a1985)?
            }, (v3, 1u64))))
        } else {
            let a1986 = v4;
            let a1987 = 92u64;
            if nat_eq(a1986, a1987) {
                f414(v0, v2, v3)
            } else {
                let a1988 = v4;
                let a1989 = 32u64;
                if if nat_lt(a1988, a1989) {
                    true
                } else {
                    let a1990 = v4;
                    let a1991 = 256u64;
                    nat_eq(a1990, a1991)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1992 = v2;
                        let a1993 = 1u64;
                        nat_add(a1992, a1993)?
                    }, ({
                        let a1994 = v3;
                        let a1995 = f32(v0, v2, 1u64);
                        append_bytes(a1994, a1995)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f384(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f415(v0, v1)
}

pub fn f385(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1996 = v0.clone();
        length_bytes(a1996)
    };
    let a1997 = 0u64;
    let a1998 = v1;
    if nat_lt(a1997, a1998) {
        let a1999 = f416(v0.clone())?;
        let a2000 = v1;
        if nat_eq(a1999, a2000) {
            let a2001 = 1u64;
            let a2002 = v1;
            if nat_lt(a2001, a2002) {
                let m2005 = {
                    let a2003 = v0;
                    let a2004 = 0u64;
                    index_bytes(a2003, a2004)
                };
                match m2005 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2006 = v2;
                        let a2007 = 48u8;
                        Ok(!equal(a2006, a2007))
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

pub fn f386(v0: Fn16, p2012: u64, p2013: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2011 = (p2012, p2013);
    {
        let mut m2011 = m2011;
        loop {
            let step = {
                let (v1, v2) = m2011;
                let m2008 = v1;
                if m2008 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2008 - 1;
                    let m2010 = {
                        let c2009: Fn16 = v0.clone();
                        c2009.apply(v2)?
                    };
                    match m2010 {
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
                Ok(next) => { m2011 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f417(v0, v1)
}

pub fn f388(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2014 = v0;
    match m2014.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let m2015 = f418(v2.clone())?;
            match m2015 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f419(v2)?, List::<Bytes>::nil())))))
                }
                Some(v4) => {
                    if f85(v4.clone(), v1.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v4, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2016 = f388(v3, List::cons(v4.clone(), v1))?;
                        match m2016 {
                            Ok(v6) => {
                                Ok(Ok::<List<Bytes>, Bytes>(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v4, List::cons(f8(Str::lit(", fields: []}")), List::<Bytes>::nil())))), v6)))
                            }
                            Err(v5) => {
                                Ok(Err::<List<Bytes>, Bytes>(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f389(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt23>), Bytes> {
    Err::<(Bytes, List<Adt23>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f390(v0: Bytes, v1: List<Adt23>) -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((v0, v1))
}

pub fn f391(v0: Bytes, v1: List<Adt14>) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2017 = v1.clone();
    match m2017.uncons() {
        None => {
            Ok(f389(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2018 = f420(f330(f142(v2, f8(Str::lit("properties")))), v1.clone());
            match m2018.uncons() {
                None => {
                    Ok(f389(f8(Str::lit("untagged oneOf: no property is required by every branch with a const string")), v0))
                }
                Some((v4, v5)) => {
                    let m2019 = v5;
                    match m2019.uncons() {
                        None => {
                            let m2020 = f421(v0.clone(), v1, v4, List::<Bytes>::nil())?;
                            match m2020 {
                                Ok(v9) => {
                                    Ok(f390(f394(v0, {
                                        let (h2021, _) = v9;
                                        h2021
                                    }), {
                                        let (_, h2022) = v9;
                                        h2022
                                    }))
                                }
                                Err(v8) => {
                                    Ok(f389(v8, v0))
                                }
                            }
                        }
                        Some((v6, _)) => {
                            Ok(f389(f7(List::cons(f8(Str::lit("ambiguous oneOf: more than one property tags every branch (")), List::cons(v4, List::cons(f8(Str::lit(", ")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f392(v0: Adt14, v1: Bytes) -> R<bool> {
    let m2023 = v0;
    match m2023 {
        Adt14::C0(_) => {
            Ok(false)
        }
        Adt14::C1(_, _) => {
            Ok(false)
        }
        Adt14::C2(_) => {
            Ok(false)
        }
        Adt14::C3(r2024) => {
            let v2 = (*r2024).clone();
            f392(v2, v1)
        }
        Adt14::C4(r2025) => {
            let v3 = (*r2025).clone();
            f392(v3, v1)
        }
        Adt14::C5 => {
            Ok(false)
        }
        Adt14::C6(r2026, r2027) => {
            let v4 = (*r2026).clone();
            let v5 = (*r2027).clone();
            if f392(v4, v1.clone())? {
                Ok(true)
            } else {
                f392(v5, v1)
            }
        }
        Adt14::C7(v6, r2028, r2029) => {
            let v7 = (*r2028).clone();
            let v8 = (*r2029).clone();
            if if f36(v6, f8(Str::lit("$ref"))) {
                f422(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f392(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f392(v8, v1)
                }
            }
        }
    }
}

pub fn f393(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2030 = v1;
    match m2030.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f423({
                let (h2031, _) = v4.clone();
                h2031
            })?;
            if f424(v6.clone())? {
                let m2036 = f425({
                    let (_, h2032) = v4.clone();
                    h2032
                }, {
                    let a2034 = v0.clone();
                    let a2035 = f216({
                        let (h2033, _) = v4.clone();
                        h2033
                    })?;
                    append_bytes(a2034, a2035)
                })?;
                match m2036 {
                    Ok(v8) => {
                        let v9: Bytes = if f85({
                            let (h2037, _) = v4;
                            h2037
                        }, v2.clone()) {
                            let (h2038, _) = v8;
                            h2038
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h2039, _) = v8;
                                h2039
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m2040 = f393(v0, v5, v2, v3)?;
                        match m2040 {
                            Ok(v11) => {
                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(if v3 {
                                    v9
                                } else {
                                    f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v6, List::cons(f8(Str::lit(", type: ")), List::cons(v9, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                }, {
                                    let (h2041, _) = v11;
                                    h2041
                                }), f328({
                                    let (_, h2042) = v8;
                                    h2042
                                }, {
                                    let (_, h2043) = v11;
                                    h2043
                                }))))
                            }
                            Err(v10) => {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v10))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v7))
                    }
                }
            } else {
                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                    let (h2044, _) = v4;
                    h2044
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f394(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f202(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f395(v0: Bytes, v1: u64) -> Bytes {
    let m2048 = {
        let a2045 = v0;
        let a2046 = v1;
        let a2047 = 1u64;
        slice_bytes(a2045, a2046, a2047)
    };
    match m2048 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f396(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f426(v1, v0, v2)
}

pub fn f397(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2093 = {
            let a2069 = {
                let a2057 = {
                    let a2051 = {
                        let a2049 = v3;
                        let a2050 = 128u8;
                        fixed_u8::bit_and(a2049, a2050)
                    };
                    let a2052 = 128u8;
                    if equal(a2051, a2052) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2058 = {
                    let a2055 = {
                        let a2053 = v3;
                        let a2054 = 64u8;
                        fixed_u8::bit_and(a2053, a2054)
                    };
                    let a2056 = 64u8;
                    if equal(a2055, a2056) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2057, a2058)?
            };
            let a2070 = {
                let a2067 = {
                    let a2061 = {
                        let a2059 = v3;
                        let a2060 = 32u8;
                        fixed_u8::bit_and(a2059, a2060)
                    };
                    let a2062 = 32u8;
                    if equal(a2061, a2062) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2068 = {
                    let a2065 = {
                        let a2063 = v3;
                        let a2064 = 16u8;
                        fixed_u8::bit_and(a2063, a2064)
                    };
                    let a2066 = 16u8;
                    if equal(a2065, a2066) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2067, a2068)?
            };
            nat_add(a2069, a2070)?
        };
        let a2094 = {
            let a2091 = {
                let a2079 = {
                    let a2073 = {
                        let a2071 = v3;
                        let a2072 = 8u8;
                        fixed_u8::bit_and(a2071, a2072)
                    };
                    let a2074 = 8u8;
                    if equal(a2073, a2074) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2080 = {
                    let a2077 = {
                        let a2075 = v3;
                        let a2076 = 4u8;
                        fixed_u8::bit_and(a2075, a2076)
                    };
                    let a2078 = 4u8;
                    if equal(a2077, a2078) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2079, a2080)?
            };
            let a2092 = {
                let a2089 = {
                    let a2083 = {
                        let a2081 = v3;
                        let a2082 = 2u8;
                        fixed_u8::bit_and(a2081, a2082)
                    };
                    let a2084 = 2u8;
                    if equal(a2083, a2084) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2090 = {
                    let a2087 = {
                        let a2085 = v3;
                        let a2086 = 1u8;
                        fixed_u8::bit_and(a2085, a2086)
                    };
                    let a2088 = 1u8;
                    if equal(a2087, a2088) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2089, a2090)?
            };
            nat_add(a2091, a2092)?
        };
        nat_add(a2093, a2094)?
    };
    let a2095 = v4;
    let a2096 = 34u64;
    if nat_eq(a2095, a2096) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2097 = v4;
        let a2098 = 92u64;
        if nat_eq(a2097, a2098) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2099 = v4;
            let a2100 = 8u64;
            if nat_eq(a2099, a2100) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2101 = v4;
                let a2102 = 12u64;
                if nat_eq(a2101, a2102) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2103 = v4;
                    let a2104 = 10u64;
                    if nat_eq(a2103, a2104) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2105 = v4;
                        let a2106 = 13u64;
                        if nat_eq(a2105, a2106) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2107 = v4;
                            let a2108 = 9u64;
                            if nat_eq(a2107, a2108) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2109 = v4;
                                let a2110 = 32u64;
                                if nat_lt(a2109, a2110) {
                                    let a2111 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2112 = f427(v2)?;
                                    Ok(append_bytes(a2111, a2112))
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

pub fn f398(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f428(Fn21::F429, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f399(v0: Fn17, p2116: List<List<(Bytes, Bytes)>>, p2117: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2115 = (p2116, p2117);
    {
        let mut m2115 = m2115;
        loop {
            let step = {
                let (v1, v2) = m2115;
                let m2113 = v2;
                match m2113.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2114: Fn17 = v0.clone();
                            c2114.apply(v1, v3)
                        }, v4))
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

pub fn f400(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f401(v0: Fn18, p2122: u64, p2123: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2121 = (p2122, p2123);
    {
        let mut m2121 = m2121;
        loop {
            let step = {
                let (v1, v2) = m2121;
                let m2118 = v1;
                if m2118 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2118 - 1;
                    let m2120 = {
                        let c2119: Fn18 = v0.clone();
                        c2119.apply(v2.clone())?
                    };
                    match m2120 {
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
                Ok(next) => { m2121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f430(v0)
}

pub fn f403(v0: Fn19, p2128: u64, p2129: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2127 = (p2128, p2129);
    {
        let mut m2127 = m2127;
        loop {
            let step = {
                let (v1, v2) = m2127;
                let m2124 = v1;
                if m2124 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2124 - 1;
                    let m2126 = {
                        let c2125: Fn19 = v0.clone();
                        c2125.apply(v2.clone())?
                    };
                    match m2126 {
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
                Ok(next) => { m2127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f431(v0)
}

pub fn f405(v0: u64) -> u32 {
    let a2130 = v0;
    let a2131 = 32u64;
    if nat_lt(a2130, a2131) {
        let a2132 = v0;
        let a2133 = 16u64;
        if nat_lt(a2132, a2133) {
            let a2134 = v0;
            let a2135 = 8u64;
            if nat_lt(a2134, a2135) {
                let a2136 = v0;
                let a2137 = 4u64;
                if nat_lt(a2136, a2137) {
                    let a2138 = v0;
                    let a2139 = 2u64;
                    if nat_lt(a2138, a2139) {
                        let a2140 = v0;
                        let a2141 = 1u64;
                        if nat_lt(a2140, a2141) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2142 = v0;
                        let a2143 = 3u64;
                        if nat_lt(a2142, a2143) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2144 = v0;
                    let a2145 = 6u64;
                    if nat_lt(a2144, a2145) {
                        let a2146 = v0;
                        let a2147 = 5u64;
                        if nat_lt(a2146, a2147) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2148 = v0;
                        let a2149 = 7u64;
                        if nat_lt(a2148, a2149) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2150 = v0;
                let a2151 = 12u64;
                if nat_lt(a2150, a2151) {
                    let a2152 = v0;
                    let a2153 = 10u64;
                    if nat_lt(a2152, a2153) {
                        let a2154 = v0;
                        let a2155 = 9u64;
                        if nat_lt(a2154, a2155) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2156 = v0;
                        let a2157 = 11u64;
                        if nat_lt(a2156, a2157) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2158 = v0;
                    let a2159 = 14u64;
                    if nat_lt(a2158, a2159) {
                        let a2160 = v0;
                        let a2161 = 13u64;
                        if nat_lt(a2160, a2161) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2162 = v0;
                        let a2163 = 15u64;
                        if nat_lt(a2162, a2163) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2164 = v0;
            let a2165 = 24u64;
            if nat_lt(a2164, a2165) {
                let a2166 = v0;
                let a2167 = 20u64;
                if nat_lt(a2166, a2167) {
                    let a2168 = v0;
                    let a2169 = 18u64;
                    if nat_lt(a2168, a2169) {
                        let a2170 = v0;
                        let a2171 = 17u64;
                        if nat_lt(a2170, a2171) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2172 = v0;
                        let a2173 = 19u64;
                        if nat_lt(a2172, a2173) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2174 = v0;
                    let a2175 = 22u64;
                    if nat_lt(a2174, a2175) {
                        let a2176 = v0;
                        let a2177 = 21u64;
                        if nat_lt(a2176, a2177) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2178 = v0;
                        let a2179 = 23u64;
                        if nat_lt(a2178, a2179) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2180 = v0;
                let a2181 = 28u64;
                if nat_lt(a2180, a2181) {
                    let a2182 = v0;
                    let a2183 = 26u64;
                    if nat_lt(a2182, a2183) {
                        let a2184 = v0;
                        let a2185 = 25u64;
                        if nat_lt(a2184, a2185) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2186 = v0;
                        let a2187 = 27u64;
                        if nat_lt(a2186, a2187) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2188 = v0;
                    let a2189 = 30u64;
                    if nat_lt(a2188, a2189) {
                        let a2190 = v0;
                        let a2191 = 29u64;
                        if nat_lt(a2190, a2191) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2192 = v0;
                        let a2193 = 31u64;
                        if nat_lt(a2192, a2193) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2194 = v0;
        let a2195 = 48u64;
        if nat_lt(a2194, a2195) {
            let a2196 = v0;
            let a2197 = 40u64;
            if nat_lt(a2196, a2197) {
                let a2198 = v0;
                let a2199 = 36u64;
                if nat_lt(a2198, a2199) {
                    let a2200 = v0;
                    let a2201 = 34u64;
                    if nat_lt(a2200, a2201) {
                        let a2202 = v0;
                        let a2203 = 33u64;
                        if nat_lt(a2202, a2203) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2204 = v0;
                        let a2205 = 35u64;
                        if nat_lt(a2204, a2205) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2206 = v0;
                    let a2207 = 38u64;
                    if nat_lt(a2206, a2207) {
                        let a2208 = v0;
                        let a2209 = 37u64;
                        if nat_lt(a2208, a2209) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2210 = v0;
                        let a2211 = 39u64;
                        if nat_lt(a2210, a2211) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2212 = v0;
                let a2213 = 44u64;
                if nat_lt(a2212, a2213) {
                    let a2214 = v0;
                    let a2215 = 42u64;
                    if nat_lt(a2214, a2215) {
                        let a2216 = v0;
                        let a2217 = 41u64;
                        if nat_lt(a2216, a2217) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2218 = v0;
                        let a2219 = 43u64;
                        if nat_lt(a2218, a2219) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2220 = v0;
                    let a2221 = 46u64;
                    if nat_lt(a2220, a2221) {
                        let a2222 = v0;
                        let a2223 = 45u64;
                        if nat_lt(a2222, a2223) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2224 = v0;
                        let a2225 = 47u64;
                        if nat_lt(a2224, a2225) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2226 = v0;
            let a2227 = 56u64;
            if nat_lt(a2226, a2227) {
                let a2228 = v0;
                let a2229 = 52u64;
                if nat_lt(a2228, a2229) {
                    let a2230 = v0;
                    let a2231 = 50u64;
                    if nat_lt(a2230, a2231) {
                        let a2232 = v0;
                        let a2233 = 49u64;
                        if nat_lt(a2232, a2233) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2234 = v0;
                        let a2235 = 51u64;
                        if nat_lt(a2234, a2235) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2236 = v0;
                    let a2237 = 54u64;
                    if nat_lt(a2236, a2237) {
                        let a2238 = v0;
                        let a2239 = 53u64;
                        if nat_lt(a2238, a2239) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2240 = v0;
                        let a2241 = 55u64;
                        if nat_lt(a2240, a2241) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2242 = v0;
                let a2243 = 60u64;
                if nat_lt(a2242, a2243) {
                    let a2244 = v0;
                    let a2245 = 58u64;
                    if nat_lt(a2244, a2245) {
                        let a2246 = v0;
                        let a2247 = 57u64;
                        if nat_lt(a2246, a2247) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2248 = v0;
                        let a2249 = 59u64;
                        if nat_lt(a2248, a2249) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2250 = v0;
                    let a2251 = 62u64;
                    if nat_lt(a2250, a2251) {
                        let a2252 = v0;
                        let a2253 = 61u64;
                        if nat_lt(a2252, a2253) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2254 = v0;
                        let a2255 = 63u64;
                        if nat_lt(a2254, a2255) {
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

pub fn f406(v0: u32) -> u32 {
    let a2258 = {
        let a2256 = f432(v0, 6u32, 26u32);
        let a2257 = f432(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2256, a2257)
    };
    let a2259 = f432(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2258, a2259)
}

pub fn f407(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2265 = {
        let a2260 = v0;
        let a2261 = v1;
        fixed_u32::bit_and(a2260, a2261)
    };
    let a2266 = {
        let a2263 = {
            let a2262 = v0;
            fixed_u32::bit_not(a2262)
        };
        let a2264 = v2;
        fixed_u32::bit_and(a2263, a2264)
    };
    fixed_u32::bit_xor(a2265, a2266)
}

pub fn f408(v0: u32) -> u32 {
    let a2269 = {
        let a2267 = f432(v0, 2u32, 30u32);
        let a2268 = f432(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2267, a2268)
    };
    let a2270 = f432(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2269, a2270)
}

pub fn f409(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2279 = {
        let a2275 = {
            let a2271 = v0;
            let a2272 = v1;
            fixed_u32::bit_and(a2271, a2272)
        };
        let a2276 = {
            let a2273 = v0;
            let a2274 = v2;
            fixed_u32::bit_and(a2273, a2274)
        };
        fixed_u32::bit_xor(a2275, a2276)
    };
    let a2280 = {
        let a2277 = v1;
        let a2278 = v2;
        fixed_u32::bit_and(a2277, a2278)
    };
    fixed_u32::bit_xor(a2279, a2280)
}

pub fn f410(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h2281, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2281
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2282, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2282
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2283, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2283
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2284, _, _, _, _, _, _, _, _, _, _, _) => {
            h2284
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2285, _, _, _, _, _, _, _, _, _, _) => {
            h2285
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2286, _, _, _, _, _, _, _, _, _) => {
            h2286
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2287, _, _, _, _, _, _, _, _) => {
            h2287
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h2288, _, _, _, _, _, _, _) => {
            h2288
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2289, _, _, _, _, _, _) => {
            h2289
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h2290, _, _, _, _, _) => {
            h2290
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h2291, _, _, _, _) => {
            h2291
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2292, _, _, _) => {
            h2292
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2293, _, _) => {
            h2293
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2294, _) => {
            h2294
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2295) => {
            h2295
        }
    }, f352(f433(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2296, _) => {
            h2296
        }
    }), f352(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2297, _, _, _, _, _, _) => {
            h2297
        }
    }, f352(f434(match v0.clone() {
        Adt24::C0(_, h2298, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2298
        }
    }), match v0 {
        Adt24::C0(h2299, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2299
        }
    }))))
}

pub fn f411(v0: Fn20, p2303: u64, p2304: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2302 = (p2303, p2304);
    {
        let mut m2302 = m2302;
        loop {
            let step = {
                let (v1, v2) = m2302;
                let m2300 = v1;
                if m2300 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2300 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2301: Fn20 = v0.clone();
                        c2301.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2305, _) = v0;
        h2305
    };
    let v2: u32 = {
        let (h2306, _) = {
            let (_, h2307) = v0;
            h2307
        };
        h2306
    };
    let v3: u32 = {
        let (_, h2308) = {
            let (_, h2309) = v0;
            h2309
        };
        h2308
    };
    ({
        let a2310 = v1;
        let a2311 = 2u64;
        let a2312 = 0u64;
        nat_quot(a2310, a2311, a2312)
    }, (f413(v2, 1u32), {
        let a2316 = {
            let a2313 = v1;
            let a2314 = 2u64;
            let a2315 = 0u64;
            nat_rem(a2313, a2314, a2315)
        };
        let a2317 = 1u64;
        if nat_eq(a2316, a2317) {
            let a2318 = v3;
            let a2319 = v2;
            fixed_u32::bit_or(a2318, a2319)
        } else {
            v3
        }
    }))
}

pub fn f413(v0: u32, v1: u32) -> u32 {
    let m2322 = {
        let a2320 = v0;
        let a2321 = v1;
        fixed_u32::shift_left(a2320, a2321)
    };
    match m2322 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f414(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f320(v0.clone(), {
        let a2323 = v1;
        let a2324 = 1u64;
        nat_add(a2323, a2324)?
    })?;
    let v4: Bytes = {
        let a2325 = v3;
        let a2326 = 117u64;
        if nat_eq(a2325, a2326) {
            f435(v0, v1)?
        } else {
            f436(v3)
        }
    };
    let a2327 = v4.clone();
    let a2328 = Bytes::lit(&[]);
    if equal(a2327, a2328) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2329 = v3;
            let a2330 = 117u64;
            if nat_eq(a2329, a2330) {
                let a2331 = v1;
                let a2332 = 6u64;
                nat_add(a2331, a2332)?
            } else {
                let a2333 = v1;
                let a2334 = 2u64;
                nat_add(a2333, a2334)?
            }
        }, ({
            let a2335 = v2;
            let a2336 = v4;
            append_bytes(a2335, a2336)
        }, 0u64))))
    }
}

pub fn f415(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f320(v0, v1)?;
    let a2337 = 47u64;
    let a2338 = v2;
    if if nat_lt(a2337, a2338) {
        let a2339 = v2;
        let a2340 = 58u64;
        nat_lt(a2339, a2340)
    } else {
        false
    } {
        Ok(Some({
            let a2341 = v1;
            let a2342 = 1u64;
            nat_add(a2341, a2342)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f416(v0: Bytes) -> R<u64> {
    let (h2343, _) = f226(Fn3::F437(v0.clone()), {
        let a2344 = v0;
        length_bytes(a2344)
    }, 0u64)?;
    Ok(h2343)
}

pub fn f417(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2348 = {
        let a2346 = v0;
        let a2347 = {
            let (h2345, _) = v1;
            h2345
        };
        index_bytes(a2346, a2347)
    };
    match m2348 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2350 = {
                    let (h2349, _) = v1;
                    h2349
                };
                let a2351 = 1u64;
                nat_add(a2350, a2351)?
            }, {
                let a2355 = {
                    let a2353 = {
                        let (_, h2352) = v1;
                        h2352
                    };
                    let a2354 = 10u64;
                    nat_mul(a2353, a2354)?
                };
                let a2356 = f438(v2)?;
                nat_add(a2355, a2356)?
            })))
        }
    }
}

pub fn f418(v0: Adt14) -> R<Option<Bytes>> {
    let m2357 = v0;
    match m2357 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v2, v3) => {
            Ok(Some({
                let a2360 = if if v2 {
                    let a2358 = 0u64;
                    let a2359 = v3;
                    nat_lt(a2358, a2359)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2361 = f284(false, v3)?;
                append_bytes(a2360, a2361)
            }))
        }
        Adt14::C2(v1) => {
            f439(v1)
        }
        Adt14::C3(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C5 => {
            Ok(None::<Bytes>)
        }
        Adt14::C6(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt14::C7(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f419(v0: Adt14) -> R<Bytes> {
    f219(v0)
}

pub fn f420(v0: List<(Bytes, Adt14)>, v1: List<Adt14>) -> List<Bytes> {
    let m2362 = v0;
    match m2362.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f440(v1.clone(), {
                let (h2363, _) = v2.clone();
                h2363
            }) {
                List::cons({
                    let (h2364, _) = v2;
                    h2364
                }, f420(v3, v1))
            } else {
                f420(v3, v1)
            }
        }
    }
}

pub fn f421(v0: Bytes, v1: List<Adt14>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2365 = v1;
    match m2365.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v4, v5)) => {
            let m2366 = f441(v4.clone(), v2.clone());
            match m2366 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v6) => {
                    let m2367 = f439(v6.clone())?;
                    match m2367 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f85(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2370 = f393({
                                    let a2368 = v0.clone();
                                    let a2369 = f216(v6)?;
                                    append_bytes(a2368, a2369)
                                }, f442(f330(f142(v4.clone(), f8(Str::lit("properties")))), v2.clone()), f73(f146(f142(v4, f8(Str::lit("required"))))), true)?;
                                match m2370 {
                                    Ok(v9) => {
                                        let m2371 = f421(v0, v5, v2, List::cons(v7.clone(), v3))?;
                                        match m2371 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f202({
                                                    let (h2372, _) = v9;
                                                    h2372
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h2373, _) = v11;
                                                    h2373
                                                }), f328({
                                                    let (_, h2374) = v9;
                                                    h2374
                                                }, {
                                                    let (_, h2375) = v11;
                                                    h2375
                                                }))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v8))
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

pub fn f422(v0: Adt14, v1: Bytes) -> R<bool> {
    let m2376 = f150(Some(v0));
    match m2376 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2377 = f443(v2)?;
            match m2377 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f36(v3, v1))
                }
            }
        }
    }
}

pub fn f423(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f280(v0.clone(), 0u64, {
        let a2378 = v0;
        length_bytes(a2378)
    }, false, Bytes::lit(&[]))?;
    let m2381 = {
        let a2379 = v1.clone();
        let a2380 = 0u64;
        index_bytes(a2379, a2380)
    };
    match m2381 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2426 = {
                    let a2402 = {
                        let a2390 = {
                            let a2384 = {
                                let a2382 = v3;
                                let a2383 = 128u8;
                                fixed_u8::bit_and(a2382, a2383)
                            };
                            let a2385 = 128u8;
                            if equal(a2384, a2385) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2391 = {
                            let a2388 = {
                                let a2386 = v3;
                                let a2387 = 64u8;
                                fixed_u8::bit_and(a2386, a2387)
                            };
                            let a2389 = 64u8;
                            if equal(a2388, a2389) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2390, a2391)?
                    };
                    let a2403 = {
                        let a2400 = {
                            let a2394 = {
                                let a2392 = v3;
                                let a2393 = 32u8;
                                fixed_u8::bit_and(a2392, a2393)
                            };
                            let a2395 = 32u8;
                            if equal(a2394, a2395) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2401 = {
                            let a2398 = {
                                let a2396 = v3;
                                let a2397 = 16u8;
                                fixed_u8::bit_and(a2396, a2397)
                            };
                            let a2399 = 16u8;
                            if equal(a2398, a2399) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2400, a2401)?
                    };
                    nat_add(a2402, a2403)?
                };
                let a2427 = {
                    let a2424 = {
                        let a2412 = {
                            let a2406 = {
                                let a2404 = v3;
                                let a2405 = 8u8;
                                fixed_u8::bit_and(a2404, a2405)
                            };
                            let a2407 = 8u8;
                            if equal(a2406, a2407) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2413 = {
                            let a2410 = {
                                let a2408 = v3;
                                let a2409 = 4u8;
                                fixed_u8::bit_and(a2408, a2409)
                            };
                            let a2411 = 4u8;
                            if equal(a2410, a2411) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2412, a2413)?
                    };
                    let a2425 = {
                        let a2422 = {
                            let a2416 = {
                                let a2414 = v3;
                                let a2415 = 2u8;
                                fixed_u8::bit_and(a2414, a2415)
                            };
                            let a2417 = 2u8;
                            if equal(a2416, a2417) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2423 = {
                            let a2420 = {
                                let a2418 = v3;
                                let a2419 = 1u8;
                                fixed_u8::bit_and(a2418, a2419)
                            };
                            let a2421 = 1u8;
                            if equal(a2420, a2421) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2422, a2423)?
                    };
                    nat_add(a2424, a2425)?
                };
                nat_add(a2426, a2427)?
            };
            if f325(v4) {
                let a2434 = f32({
                    let a2428 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2428)
                }, {
                    let a2429 = v4;
                    let a2430 = 65u64;
                    nat_sub(a2429, a2430)
                }, 1u64);
                let a2435 = f32(v1.clone(), 1u64, {
                    let a2432 = {
                        let a2431 = v1;
                        length_bytes(a2431)
                    };
                    let a2433 = 1u64;
                    nat_sub(a2432, a2433)
                });
                Ok(append_bytes(a2434, a2435))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f424(v0: Bytes) -> R<bool> {
    let m2438 = {
        let a2436 = v0;
        let a2437 = 0u64;
        index_bytes(a2436, a2437)
    };
    match m2438 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f324({
                let v2: u8 = v1;
                let a2483 = {
                    let a2459 = {
                        let a2447 = {
                            let a2441 = {
                                let a2439 = v2;
                                let a2440 = 128u8;
                                fixed_u8::bit_and(a2439, a2440)
                            };
                            let a2442 = 128u8;
                            if equal(a2441, a2442) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2448 = {
                            let a2445 = {
                                let a2443 = v2;
                                let a2444 = 64u8;
                                fixed_u8::bit_and(a2443, a2444)
                            };
                            let a2446 = 64u8;
                            if equal(a2445, a2446) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2447, a2448)?
                    };
                    let a2460 = {
                        let a2457 = {
                            let a2451 = {
                                let a2449 = v2;
                                let a2450 = 32u8;
                                fixed_u8::bit_and(a2449, a2450)
                            };
                            let a2452 = 32u8;
                            if equal(a2451, a2452) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2458 = {
                            let a2455 = {
                                let a2453 = v2;
                                let a2454 = 16u8;
                                fixed_u8::bit_and(a2453, a2454)
                            };
                            let a2456 = 16u8;
                            if equal(a2455, a2456) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2457, a2458)?
                    };
                    nat_add(a2459, a2460)?
                };
                let a2484 = {
                    let a2481 = {
                        let a2469 = {
                            let a2463 = {
                                let a2461 = v2;
                                let a2462 = 8u8;
                                fixed_u8::bit_and(a2461, a2462)
                            };
                            let a2464 = 8u8;
                            if equal(a2463, a2464) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2470 = {
                            let a2467 = {
                                let a2465 = v2;
                                let a2466 = 4u8;
                                fixed_u8::bit_and(a2465, a2466)
                            };
                            let a2468 = 4u8;
                            if equal(a2467, a2468) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2469, a2470)?
                    };
                    let a2482 = {
                        let a2479 = {
                            let a2473 = {
                                let a2471 = v2;
                                let a2472 = 2u8;
                                fixed_u8::bit_and(a2471, a2472)
                            };
                            let a2474 = 2u8;
                            if equal(a2473, a2474) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2480 = {
                            let a2477 = {
                                let a2475 = v2;
                                let a2476 = 1u8;
                                fixed_u8::bit_and(a2475, a2476)
                            };
                            let a2478 = 1u8;
                            if equal(a2477, a2478) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2479, a2480)?
                    };
                    nat_add(a2481, a2482)?
                };
                nat_add(a2483, a2484)?
            }) {
                Ok(true)
            } else {
                Ok(f325({
                    let v3: u8 = v1;
                    let a2529 = {
                        let a2505 = {
                            let a2493 = {
                                let a2487 = {
                                    let a2485 = v3;
                                    let a2486 = 128u8;
                                    fixed_u8::bit_and(a2485, a2486)
                                };
                                let a2488 = 128u8;
                                if equal(a2487, a2488) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a2494 = {
                                let a2491 = {
                                    let a2489 = v3;
                                    let a2490 = 64u8;
                                    fixed_u8::bit_and(a2489, a2490)
                                };
                                let a2492 = 64u8;
                                if equal(a2491, a2492) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2493, a2494)?
                        };
                        let a2506 = {
                            let a2503 = {
                                let a2497 = {
                                    let a2495 = v3;
                                    let a2496 = 32u8;
                                    fixed_u8::bit_and(a2495, a2496)
                                };
                                let a2498 = 32u8;
                                if equal(a2497, a2498) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a2504 = {
                                let a2501 = {
                                    let a2499 = v3;
                                    let a2500 = 16u8;
                                    fixed_u8::bit_and(a2499, a2500)
                                };
                                let a2502 = 16u8;
                                if equal(a2501, a2502) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2503, a2504)?
                        };
                        nat_add(a2505, a2506)?
                    };
                    let a2530 = {
                        let a2527 = {
                            let a2515 = {
                                let a2509 = {
                                    let a2507 = v3;
                                    let a2508 = 8u8;
                                    fixed_u8::bit_and(a2507, a2508)
                                };
                                let a2510 = 8u8;
                                if equal(a2509, a2510) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2516 = {
                                let a2513 = {
                                    let a2511 = v3;
                                    let a2512 = 4u8;
                                    fixed_u8::bit_and(a2511, a2512)
                                };
                                let a2514 = 4u8;
                                if equal(a2513, a2514) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2515, a2516)?
                        };
                        let a2528 = {
                            let a2525 = {
                                let a2519 = {
                                    let a2517 = v3;
                                    let a2518 = 2u8;
                                    fixed_u8::bit_and(a2517, a2518)
                                };
                                let a2520 = 2u8;
                                if equal(a2519, a2520) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2526 = {
                                let a2523 = {
                                    let a2521 = v3;
                                    let a2522 = 1u8;
                                    fixed_u8::bit_and(a2521, a2522)
                                };
                                let a2524 = 1u8;
                                if equal(a2523, a2524) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2525, a2526)?
                        };
                        nat_add(a2527, a2528)?
                    };
                    nat_add(a2529, a2530)?
                }))
            }
        }
    }
}

pub fn f425(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2531 = f444(v0.clone());
    match m2531 {
        None => {
            if f36(f145(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2532 = f142(v0, f8(Str::lit("items")));
                match m2532 {
                    None => {
                        Ok(f445(f8(Str::lit("List")), f447()))
                    }
                    Some(v3) => {
                        Ok(f445(f8(Str::lit("List")), f446(v3, {
                            let a2533 = v1;
                            let a2534 = f8(Str::lit("Item"));
                            append_bytes(a2533, a2534)
                        })?))
                    }
                }
            } else {
                f446(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f445(f8(Str::lit("Option")), f446(v2, v1)?))
        }
    }
}

pub fn f426(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2535, _) = v2;
        h2535
    };
    let a2536 = v3;
    let a2537 = 0u64;
    if nat_eq(a2536, a2537) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2538 = v3;
            let a2539 = v1;
            let a2540 = 0u64;
            nat_quot(a2538, a2539, a2540)
        }, {
            let a2545 = f395(v0, {
                let a2541 = v3;
                let a2542 = v1;
                let a2543 = 0u64;
                nat_rem(a2541, a2542, a2543)
            });
            let a2546 = {
                let (_, h2544) = v2;
                h2544
            };
            append_bytes(a2545, a2546)
        }))
    }
}

pub fn f427(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2591 = {
            let a2567 = {
                let a2555 = {
                    let a2549 = {
                        let a2547 = v1;
                        let a2548 = 128u8;
                        fixed_u8::bit_and(a2547, a2548)
                    };
                    let a2550 = 128u8;
                    if equal(a2549, a2550) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2556 = {
                    let a2553 = {
                        let a2551 = v1;
                        let a2552 = 64u8;
                        fixed_u8::bit_and(a2551, a2552)
                    };
                    let a2554 = 64u8;
                    if equal(a2553, a2554) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2555, a2556)?
            };
            let a2568 = {
                let a2565 = {
                    let a2559 = {
                        let a2557 = v1;
                        let a2558 = 32u8;
                        fixed_u8::bit_and(a2557, a2558)
                    };
                    let a2560 = 32u8;
                    if equal(a2559, a2560) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2566 = {
                    let a2563 = {
                        let a2561 = v1;
                        let a2562 = 16u8;
                        fixed_u8::bit_and(a2561, a2562)
                    };
                    let a2564 = 16u8;
                    if equal(a2563, a2564) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2565, a2566)?
            };
            nat_add(a2567, a2568)?
        };
        let a2592 = {
            let a2589 = {
                let a2577 = {
                    let a2571 = {
                        let a2569 = v1;
                        let a2570 = 8u8;
                        fixed_u8::bit_and(a2569, a2570)
                    };
                    let a2572 = 8u8;
                    if equal(a2571, a2572) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2578 = {
                    let a2575 = {
                        let a2573 = v1;
                        let a2574 = 4u8;
                        fixed_u8::bit_and(a2573, a2574)
                    };
                    let a2576 = 4u8;
                    if equal(a2575, a2576) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2577, a2578)?
            };
            let a2590 = {
                let a2587 = {
                    let a2581 = {
                        let a2579 = v1;
                        let a2580 = 2u8;
                        fixed_u8::bit_and(a2579, a2580)
                    };
                    let a2582 = 2u8;
                    if equal(a2581, a2582) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2588 = {
                    let a2585 = {
                        let a2583 = v1;
                        let a2584 = 1u8;
                        fixed_u8::bit_and(a2583, a2584)
                    };
                    let a2586 = 1u8;
                    if equal(a2585, a2586) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2587, a2588)?
            };
            nat_add(a2589, a2590)?
        };
        nat_add(a2591, a2592)?
    };
    let a2599 = f395(f448(), {
        let a2593 = v2;
        let a2594 = 16u64;
        let a2595 = 0u64;
        nat_quot(a2593, a2594, a2595)
    });
    let a2600 = f395(f448(), {
        let a2596 = v2;
        let a2597 = 16u64;
        let a2598 = 0u64;
        nat_rem(a2596, a2597, a2598)
    });
    Ok(append_bytes(a2599, a2600))
}

pub fn f428(v0: Fn21, p2604: List<List<(Bytes, Bytes)>>, p2605: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2603 = (p2604, p2605);
    {
        let mut m2603 = m2603;
        loop {
            let step = {
                let (v1, v2) = m2603;
                let m2601 = v2;
                match m2601.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2602: Fn21 = v0.clone();
                            c2602.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2603 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f430(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2607 = {
        let a2606 = v0.clone();
        length_list(a2606)
    };
    let a2608 = 1u64;
    if nat_le(a2607, a2608) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f449(v0)?))
    }
}

pub fn f431(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2609, _) = v0;
        h2609
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2610) = v0;
        h2610
    };
    let m2611 = v1;
    match m2611.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2612 = v4;
            match m2612.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f450(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f432(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2613 = f314(v0, v1);
    let a2614 = f413(v0, v2);
    fixed_u32::bit_or(a2613, a2614)
}

pub fn f433(v0: u32) -> u32 {
    let a2617 = {
        let a2615 = f432(v0, 17u32, 15u32);
        let a2616 = f432(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2615, a2616)
    };
    let a2618 = f314(v0, 10u32);
    fixed_u32::bit_xor(a2617, a2618)
}

pub fn f434(v0: u32) -> u32 {
    let a2621 = {
        let a2619 = f432(v0, 7u32, 25u32);
        let a2620 = f432(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2619, a2620)
    };
    let a2622 = f314(v0, 3u32);
    fixed_u32::bit_xor(a2621, a2622)
}

pub fn f435(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2625 = f32(v0.clone(), {
        let a2623 = v1;
        let a2624 = 2u64;
        nat_add(a2623, a2624)?
    }, 2u64);
    let a2626 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2625, a2626) {
        let m2629 = f451(f32(v0, {
            let a2627 = v1;
            let a2628 = 4u64;
            nat_add(a2627, a2628)?
        }, 2u64))?;
        match m2629 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f320(v2.clone(), 0u64)?;
                let a2630 = v3;
                let a2631 = 32u64;
                if nat_lt(a2630, a2631) {
                    if f452(v3) {
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

pub fn f436(v0: u64) -> Bytes {
    let a2632 = v0;
    let a2633 = 34u64;
    if nat_eq(a2632, a2633) {
        Bytes::lit(&[0x22])
    } else {
        let a2634 = v0;
        let a2635 = 92u64;
        if nat_eq(a2634, a2635) {
            Bytes::lit(&[0x5c])
        } else {
            let a2636 = v0;
            let a2637 = 98u64;
            if nat_eq(a2636, a2637) {
                Bytes::lit(&[0x08])
            } else {
                let a2638 = v0;
                let a2639 = 102u64;
                if nat_eq(a2638, a2639) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2640 = v0;
                    let a2641 = 110u64;
                    if nat_eq(a2640, a2641) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2642 = v0;
                        let a2643 = 114u64;
                        if nat_eq(a2642, a2643) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2644 = v0;
                            let a2645 = 116u64;
                            if nat_eq(a2644, a2645) {
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

pub fn f437(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2648 = {
        let a2646 = v0;
        let a2647 = v1;
        index_bytes(a2646, a2647)
    };
    match m2648 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f453(v2)? {
                Ok(Some({
                    let a2649 = v1;
                    let a2650 = 1u64;
                    nat_add(a2649, a2650)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f438(v0: u8) -> R<u64> {
    let a2697 = {
        let v1: u8 = v0;
        let a2695 = {
            let a2671 = {
                let a2659 = {
                    let a2653 = {
                        let a2651 = v1;
                        let a2652 = 128u8;
                        fixed_u8::bit_and(a2651, a2652)
                    };
                    let a2654 = 128u8;
                    if equal(a2653, a2654) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2660 = {
                    let a2657 = {
                        let a2655 = v1;
                        let a2656 = 64u8;
                        fixed_u8::bit_and(a2655, a2656)
                    };
                    let a2658 = 64u8;
                    if equal(a2657, a2658) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2659, a2660)?
            };
            let a2672 = {
                let a2669 = {
                    let a2663 = {
                        let a2661 = v1;
                        let a2662 = 32u8;
                        fixed_u8::bit_and(a2661, a2662)
                    };
                    let a2664 = 32u8;
                    if equal(a2663, a2664) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2670 = {
                    let a2667 = {
                        let a2665 = v1;
                        let a2666 = 16u8;
                        fixed_u8::bit_and(a2665, a2666)
                    };
                    let a2668 = 16u8;
                    if equal(a2667, a2668) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2669, a2670)?
            };
            nat_add(a2671, a2672)?
        };
        let a2696 = {
            let a2693 = {
                let a2681 = {
                    let a2675 = {
                        let a2673 = v1;
                        let a2674 = 8u8;
                        fixed_u8::bit_and(a2673, a2674)
                    };
                    let a2676 = 8u8;
                    if equal(a2675, a2676) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2682 = {
                    let a2679 = {
                        let a2677 = v1;
                        let a2678 = 4u8;
                        fixed_u8::bit_and(a2677, a2678)
                    };
                    let a2680 = 4u8;
                    if equal(a2679, a2680) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2681, a2682)?
            };
            let a2694 = {
                let a2691 = {
                    let a2685 = {
                        let a2683 = v1;
                        let a2684 = 2u8;
                        fixed_u8::bit_and(a2683, a2684)
                    };
                    let a2686 = 2u8;
                    if equal(a2685, a2686) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2692 = {
                    let a2689 = {
                        let a2687 = v1;
                        let a2688 = 1u8;
                        fixed_u8::bit_and(a2687, a2688)
                    };
                    let a2690 = 1u8;
                    if equal(a2689, a2690) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2691, a2692)?
            };
            nat_add(a2693, a2694)?
        };
        nat_add(a2695, a2696)?
    };
    let a2698 = 48u64;
    Ok(nat_sub(a2697, a2698))
}

pub fn f439(v0: Bytes) -> R<Option<Bytes>> {
    if f454({
        let a2699 = v0.clone();
        length_bytes(a2699)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f423(v0)?;
        if f424(v1.clone())? {
            Ok(Some(v1))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        f455({
            let a2700 = v0.clone();
            length_bytes(a2700)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f440(p2704: List<Adt14>, v1: Bytes) -> bool {
    let m2703 = p2704;
    {
        let mut m2703 = m2703;
        loop {
            let step = {
                let v0 = m2703;
                let m2701 = v0;
                match m2701.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m2702 = f441(v2, v1.clone());
                        if match m2702 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt14>, bool>(v3)
                        } else {
                            Err::<List<Adt14>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2703 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: Adt14, v1: Bytes) -> Option<Bytes> {
    if f85(v1.clone(), f73(f146(f142(v0.clone(), f8(Str::lit("required")))))) {
        let m2705 = f142(v0, f8(Str::lit("properties")));
        match m2705 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m2706 = f142(v2, v1);
                match m2706 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f150(f142(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f442(v0: List<(Bytes, Adt14)>, v1: Bytes) -> List<(Bytes, Adt14)> {
    let m2707 = v0;
    match m2707.uncons() {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h2708, _) = v2.clone();
                h2708
            }, v1.clone()) {
                f442(v3, v1)
            } else {
                List::cons(v2, f442(v3, v1))
            }
        }
    }
}

pub fn f443(v0: Bytes) -> R<Option<Bytes>> {
    if f192(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f216(f32(v0.clone(), 8u64, {
            let a2710 = {
                let a2709 = v0;
                length_bytes(a2709)
            };
            let a2711 = 8u64;
            nat_sub(a2710, a2711)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f444(v0: Adt14) -> Option<Adt14> {
    let m2712 = f146(f142(v0, f8(Str::lit("oneOf"))));
    match m2712.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2713 = v2;
            match m2713.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2714 = v4;
                    match m2714.uncons() {
                        None => {
                            if f456(v3.clone()) {
                                Some(v1)
                            } else {
                                if f456(v1) {
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

pub fn f445(v0: Bytes, v1: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m2715 = v1;
    match m2715 {
        Ok(v3) => {
            f390(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2716, _) = v3;
                h2716
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2717) = v3;
                h2717
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v2)
        }
    }
}

pub fn f446(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2718 = f444(v0.clone());
    match m2718 {
        None => {
            if f36(f145(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2719 = f142(v0, f8(Str::lit("items")));
                match m2719 {
                    None => {
                        Ok(f445(f8(Str::lit("List")), f447()))
                    }
                    Some(v3) => {
                        Ok(f445(f8(Str::lit("List")), f457(v3, {
                            let a2720 = v1;
                            let a2721 = f8(Str::lit("Item"));
                            append_bytes(a2720, a2721)
                        })?))
                    }
                }
            } else {
                f457(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f445(f8(Str::lit("Option")), f457(v2, v1)?))
        }
    }
}

pub fn f447() -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((f360(), List::cons(Adt23::C2, List::<Adt23>::nil())))
}

pub fn f448() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f449(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f398({
        let (_, h2722) = {
            let (h2723, _) = f458(Fn22::F459, {
                let a2725 = {
                    let a2724 = v0.clone();
                    length_list(a2724)
                };
                let a2726 = 1u64;
                nat_add(a2725, a2726)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2723
        };
        h2722
    }))
}

pub fn f450(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2731 = {
            let a2729 = {
                let a2727 = v0.clone();
                length_list(a2727)
            };
            let a2730 = {
                let a2728 = v1.clone();
                length_list(a2728)
            };
            nat_add(a2729, a2730)?
        };
        let a2732 = 1u64;
        nat_add(a2731, a2732)?
    };
    Ok(f304({
        let (_, h2733) = {
            let (_, h2734) = {
                let (h2735, _) = f460(Fn23::F461, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2735
            };
            h2734
        };
        h2733
    }))
}

pub fn f451(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2736 = v0.clone();
        length_bytes(a2736)
    };
    let a2740 = {
        let a2737 = v1;
        let a2738 = 2u64;
        let a2739 = 0u64;
        nat_rem(a2737, a2738, a2739)
    };
    let a2741 = 0u64;
    if nat_eq(a2740, a2741) {
        let (_, h2742) = {
            let (h2743, _) = f462(Fn24::F463(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2743
        };
        Ok(h2742)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f452(v0: u64) -> bool {
    let a2744 = v0;
    let a2745 = 8u64;
    if nat_eq(a2744, a2745) {
        true
    } else {
        let a2746 = v0;
        let a2747 = 9u64;
        if nat_eq(a2746, a2747) {
            true
        } else {
            let a2748 = v0;
            let a2749 = 10u64;
            if nat_eq(a2748, a2749) {
                true
            } else {
                let a2750 = v0;
                let a2751 = 12u64;
                if nat_eq(a2750, a2751) {
                    true
                } else {
                    let a2752 = v0;
                    let a2753 = 13u64;
                    nat_eq(a2752, a2753)
                }
            }
        }
    }
}

pub fn f453(v0: u8) -> R<bool> {
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
    if nat_le(a2800, a2801) {
        let a2802 = v2;
        let a2803 = 57u64;
        Ok(nat_le(a2802, a2803))
    } else {
        Ok(false)
    }
}

pub fn f454(p2859: u64, v1: Bytes, p2860: u64) -> R<bool> {
    let m2858 = (p2859, p2860);
    {
        let mut m2858 = m2858;
        loop {
            let step = {
                let (v0, v2) = m2858;
                let m2804 = v0;
                if m2804 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2804 - 1;
                    let m2807 = {
                        let a2805 = v1.clone();
                        let a2806 = v2;
                        index_bytes(a2805, a2806)
                    };
                    match m2807 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2854 = {
                                let v5: u8 = v4;
                                let a2852 = {
                                    let a2828 = {
                                        let a2816 = {
                                            let a2810 = {
                                                let a2808 = v5;
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
                                                let a2812 = v5;
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
                                                let a2818 = v5;
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
                                                let a2822 = v5;
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
                                                let a2830 = v5;
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
                                                let a2834 = v5;
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
                                                let a2840 = v5;
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
                                                let a2844 = v5;
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
                            let a2855 = 128u64;
                            if nat_lt(a2854, a2855) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2856 = v2;
                                    let a2857 = 1u64;
                                    nat_add(a2856, a2857)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(p2873: u64, v1: Bytes, p2874: u64, p2875: Bytes) -> R<Option<Bytes>> {
    let m2872 = (p2873, (p2874, p2875));
    {
        let mut m2872 = m2872;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2872;
                let m2861 = v0;
                if m2861 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2861 - 1;
                    let a2863 = {
                        let a2862 = v1.clone();
                        length_bytes(a2862)
                    };
                    let a2864 = v2;
                    if nat_le(a2863, a2864) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2865 = f464(v1.clone(), v2)?;
                        match m2865 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2867 = v2;
                                    let a2868 = {
                                        let (_, h2866) = v5;
                                        h2866
                                    };
                                    nat_add(a2867, a2868)?
                                }, {
                                    let a2870 = v3;
                                    let a2871 = f465({
                                        let (h2869, _) = v5;
                                        h2869
                                    });
                                    append_bytes(a2870, a2871)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2872 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: Adt14) -> bool {
    f36(f145(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f457(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2876 = v0.clone();
    match m2876 {
        Adt14::C0(v2) => {
            if v2 {
                Ok(f447())
            } else {
                Ok(f389(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt14::C1(_, _) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C2(_) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C3(_) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C4(_) => {
            let m2877 = f466(f363(v0.clone()));
            match m2877 {
                None => {
                    let m2878 = f150(f142(v0.clone(), f8(Str::lit("$ref"))));
                    match m2878 {
                        None => {
                            let m2880 = f142(v0.clone(), f8(Str::lit("enum")));
                            match m2880 {
                                None => {
                                    let m2881 = f142(v0.clone(), f8(Str::lit("const")));
                                    match m2881 {
                                        None => {
                                            let m2883 = f142(v0.clone(), f8(Str::lit("oneOf")));
                                            match m2883 {
                                                None => {
                                                    let v20: Bytes = f145(v0.clone(), f8(Str::lit("type")));
                                                    if if f36(v20.clone(), Bytes::lit(&[])) {
                                                        !f467(v0.clone())
                                                    } else {
                                                        false
                                                    } {
                                                        Ok(f447())
                                                    } else {
                                                        if f36(v20.clone(), f8(Str::lit("string"))) {
                                                            Ok(f390(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                                        } else {
                                                            if f36(v20.clone(), f8(Str::lit("integer"))) {
                                                                Ok(f390(f468(v0), List::<Adt23>::nil()))
                                                            } else {
                                                                if f36(v20.clone(), f8(Str::lit("number"))) {
                                                                    Ok(f390(f469(), List::<Adt23>::nil()))
                                                                } else {
                                                                    if f36(v20.clone(), f8(Str::lit("boolean"))) {
                                                                        Ok(f390(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                                                    } else {
                                                                        if f36(v20.clone(), f8(Str::lit("object"))) {
                                                                            let m2884 = f330(f142(v0.clone(), f8(Str::lit("properties"))));
                                                                            match m2884.uncons() {
                                                                                None => {
                                                                                    let m2885 = f142(v0, f8(Str::lit("additionalProperties")));
                                                                                    match m2885 {
                                                                                        None => {
                                                                                            Ok(f445(f8(Str::lit("List")), f470(f447())))
                                                                                        }
                                                                                        Some(v23) => {
                                                                                            Ok(f445(f8(Str::lit("List")), f470(f471(v23, v1)?)))
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Some((_, _)) => {
                                                                                    Ok(f390(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(f389(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v1))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some(_) => {
                                                    Ok(f390(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                }
                                            }
                                        }
                                        Some(v17) => {
                                            let m2882 = f150(Some(v17));
                                            match m2882 {
                                                None => {
                                                    Ok(f389(f8(Str::lit("unsupported const")), v1))
                                                }
                                                Some(_) => {
                                                    Ok(f390(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(_) => {
                                    Ok(f390(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                }
                            }
                        }
                        Some(v14) => {
                            let m2879 = f443(v14)?;
                            match m2879 {
                                None => {
                                    Ok(f389(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v15) => {
                                    Ok(f390(v15, List::<Adt23>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v13) => {
                    Ok(f389(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v13, List::<Bytes>::nil()))), v1))
                }
            }
        }
        Adt14::C5 => {
            Ok(f447())
        }
        Adt14::C6(_, _) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C7(_, _, _) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f458(v0: Fn22, p2890: u64, p2891: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2889 = (p2890, p2891);
    {
        let mut m2889 = m2889;
        loop {
            let step = {
                let (v1, v2) = m2889;
                let m2886 = v1;
                if m2886 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2886 - 1;
                    let m2888 = {
                        let c2887: Fn22 = v0.clone();
                        c2887.apply(v2.clone())?
                    };
                    match m2888 {
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
                Ok(next) => { m2889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f472(v0)
}

pub fn f460(v0: Fn23, p2896: u64, p2897: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2895 = (p2896, p2897);
    {
        let mut m2895 = m2895;
        loop {
            let step = {
                let (v1, v2) = m2895;
                let m2892 = v1;
                if m2892 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2892 - 1;
                    let m2894 = {
                        let c2893: Fn23 = v0.clone();
                        c2893.apply(v2.clone())
                    };
                    match m2894 {
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
                Ok(next) => { m2895 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f473(v0)
}

pub fn f462(v0: Fn24, p2902: u64, p2903: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2901 = (p2902, p2903);
    {
        let mut m2901 = m2901;
        loop {
            let step = {
                let (v1, v2) = m2901;
                let m2898 = v1;
                if m2898 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2898 - 1;
                    let m2900 = {
                        let c2899: Fn24 = v0.clone();
                        c2899.apply(v2.clone())?
                    };
                    match m2900 {
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
                Ok(next) => { m2901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f474(v0, v1)
}

pub fn f464(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2906 = {
        let a2904 = v0.clone();
        let a2905 = v1;
        index_bytes(a2904, a2905)
    };
    match m2906 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2951 = {
                    let a2927 = {
                        let a2915 = {
                            let a2909 = {
                                let a2907 = v3;
                                let a2908 = 128u8;
                                fixed_u8::bit_and(a2907, a2908)
                            };
                            let a2910 = 128u8;
                            if equal(a2909, a2910) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2916 = {
                            let a2913 = {
                                let a2911 = v3;
                                let a2912 = 64u8;
                                fixed_u8::bit_and(a2911, a2912)
                            };
                            let a2914 = 64u8;
                            if equal(a2913, a2914) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2915, a2916)?
                    };
                    let a2928 = {
                        let a2925 = {
                            let a2919 = {
                                let a2917 = v3;
                                let a2918 = 32u8;
                                fixed_u8::bit_and(a2917, a2918)
                            };
                            let a2920 = 32u8;
                            if equal(a2919, a2920) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2926 = {
                            let a2923 = {
                                let a2921 = v3;
                                let a2922 = 16u8;
                                fixed_u8::bit_and(a2921, a2922)
                            };
                            let a2924 = 16u8;
                            if equal(a2923, a2924) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2925, a2926)?
                    };
                    nat_add(a2927, a2928)?
                };
                let a2952 = {
                    let a2949 = {
                        let a2937 = {
                            let a2931 = {
                                let a2929 = v3;
                                let a2930 = 8u8;
                                fixed_u8::bit_and(a2929, a2930)
                            };
                            let a2932 = 8u8;
                            if equal(a2931, a2932) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2938 = {
                            let a2935 = {
                                let a2933 = v3;
                                let a2934 = 4u8;
                                fixed_u8::bit_and(a2933, a2934)
                            };
                            let a2936 = 4u8;
                            if equal(a2935, a2936) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2937, a2938)?
                    };
                    let a2950 = {
                        let a2947 = {
                            let a2941 = {
                                let a2939 = v3;
                                let a2940 = 2u8;
                                fixed_u8::bit_and(a2939, a2940)
                            };
                            let a2942 = 2u8;
                            if equal(a2941, a2942) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2948 = {
                            let a2945 = {
                                let a2943 = v3;
                                let a2944 = 1u8;
                                fixed_u8::bit_and(a2943, a2944)
                            };
                            let a2946 = 1u8;
                            if equal(a2945, a2946) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2947, a2948)?
                    };
                    nat_add(a2949, a2950)?
                };
                nat_add(a2951, a2952)?
            };
            let a2953 = v4;
            let a2954 = 128u64;
            if nat_lt(a2953, a2954) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2955 = 194u64;
                let a2956 = v4;
                if if nat_le(a2955, a2956) {
                    let a2957 = v4;
                    let a2958 = 223u64;
                    nat_le(a2957, a2958)
                } else {
                    false
                } {
                    let m2961 = f475(v0, {
                        let a2959 = v1;
                        let a2960 = 1u64;
                        nat_add(a2959, a2960)?
                    })?;
                    match m2961 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f476({
                                let a2966 = {
                                    let a2964 = {
                                        let a2962 = v4;
                                        let a2963 = 192u64;
                                        nat_sub(a2962, a2963)
                                    };
                                    let a2965 = 64u64;
                                    nat_mul(a2964, a2965)?
                                };
                                let a2967 = v5;
                                nat_add(a2966, a2967)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2968 = 224u64;
                    let a2969 = v4;
                    if if nat_le(a2968, a2969) {
                        let a2970 = v4;
                        let a2971 = 239u64;
                        nat_le(a2970, a2971)
                    } else {
                        false
                    } {
                        let m2974 = f475(v0.clone(), {
                            let a2972 = v1;
                            let a2973 = 1u64;
                            nat_add(a2972, a2973)?
                        })?;
                        match m2974 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2977 = f475(v0, {
                                    let a2975 = v1;
                                    let a2976 = 2u64;
                                    nat_add(a2975, a2976)?
                                })?;
                                match m2977 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f476({
                                            let a2986 = {
                                                let a2980 = {
                                                    let a2978 = v4;
                                                    let a2979 = 224u64;
                                                    nat_sub(a2978, a2979)
                                                };
                                                let a2981 = 4096u64;
                                                nat_mul(a2980, a2981)?
                                            };
                                            let a2987 = {
                                                let a2984 = {
                                                    let a2982 = v6;
                                                    let a2983 = 64u64;
                                                    nat_mul(a2982, a2983)?
                                                };
                                                let a2985 = v7;
                                                nat_add(a2984, a2985)?
                                            };
                                            nat_add(a2986, a2987)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2988 = 240u64;
                        let a2989 = v4;
                        if if nat_le(a2988, a2989) {
                            let a2990 = v4;
                            let a2991 = 244u64;
                            nat_le(a2990, a2991)
                        } else {
                            false
                        } {
                            let m2994 = f475(v0.clone(), {
                                let a2992 = v1;
                                let a2993 = 1u64;
                                nat_add(a2992, a2993)?
                            })?;
                            match m2994 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2997 = f475(v0.clone(), {
                                        let a2995 = v1;
                                        let a2996 = 2u64;
                                        nat_add(a2995, a2996)?
                                    })?;
                                    match m2997 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m3000 = f475(v0, {
                                                let a2998 = v1;
                                                let a2999 = 3u64;
                                                nat_add(a2998, a2999)?
                                            })?;
                                            match m3000 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f476({
                                                        let a3013 = {
                                                            let a3003 = {
                                                                let a3001 = v4;
                                                                let a3002 = 240u64;
                                                                nat_sub(a3001, a3002)
                                                            };
                                                            let a3004 = 262144u64;
                                                            nat_mul(a3003, a3004)?
                                                        };
                                                        let a3014 = {
                                                            let a3011 = {
                                                                let a3005 = v8;
                                                                let a3006 = 4096u64;
                                                                nat_mul(a3005, a3006)?
                                                            };
                                                            let a3012 = {
                                                                let a3009 = {
                                                                    let a3007 = v9;
                                                                    let a3008 = 64u64;
                                                                    nat_mul(a3007, a3008)?
                                                                };
                                                                let a3010 = v10;
                                                                nat_add(a3009, a3010)?
                                                            };
                                                            nat_add(a3011, a3012)?
                                                        };
                                                        nat_add(a3013, a3014)?
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

pub fn f465(v0: u64) -> Bytes {
    let a3015 = f8(Str::lit("u"));
    let a3016 = f477(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3015, a3016)
}

pub fn f466(p3022: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m3021 = p3022;
    {
        let mut m3021 = m3021;
        loop {
            let step = {
                let v0 = m3021;
                let m3017 = v0;
                match m3017.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f85({
                            let (h3018, _) = v1.clone();
                            h3018
                        }, f478()) {
                            true
                        } else {
                            f85({
                                let (h3019, _) = v1.clone();
                                h3019
                            }, f479())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h3020, _) = v1;
                                h3020
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3021 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Adt14) -> bool {
    if f480(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f480(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f480(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f480(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    f480(v0, f8(Str::lit("additionalProperties")))
                }
            }
        }
    }
}

pub fn f468(v0: Adt14) -> Bytes {
    let m3023 = f296(f142(v0, f8(Str::lit("minimum"))));
    match m3023 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f469() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f470(v0: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m3024 = v0;
    match m3024 {
        Ok(v2) => {
            f390(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h3025, _) = v2;
                h3025
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h3026) = v2;
                h3026
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v1)
        }
    }
}

pub fn f471(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m3027 = v0.clone();
    match m3027 {
        Adt14::C0(v2) => {
            if v2 {
                Ok(f447())
            } else {
                Ok(f389(f8(Str::lit("an object without properties that admits no member has no field to translate")), v1))
            }
        }
        Adt14::C1(_, _) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C2(_) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C3(_) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C4(_) => {
            let m3028 = f150(f142(v0.clone(), f8(Str::lit("$ref"))));
            match m3028 {
                None => {
                    let v15: Bytes = f145(v0.clone(), f8(Str::lit("type")));
                    if if f36(v15.clone(), Bytes::lit(&[])) {
                        !f467(v0.clone())
                    } else {
                        false
                    } {
                        Ok(f447())
                    } else {
                        if f36(v15.clone(), f8(Str::lit("string"))) {
                            Ok(f390(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                        } else {
                            if f36(v15.clone(), f8(Str::lit("integer"))) {
                                Ok(f390(f468(v0), List::<Adt23>::nil()))
                            } else {
                                if f36(v15.clone(), f8(Str::lit("number"))) {
                                    Ok(f390(f469(), List::<Adt23>::nil()))
                                } else {
                                    if f36(v15, f8(Str::lit("boolean"))) {
                                        Ok(f390(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                    } else {
                                        Ok(f389(f8(Str::lit("additionalProperties is translated only as a scalar, a reference or any value; give the members a $defs entry and reference it")), v1))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v13) => {
                    let m3029 = f443(v13)?;
                    match m3029 {
                        None => {
                            Ok(f389(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v14) => {
                            Ok(f390(v14, List::<Adt23>::nil()))
                        }
                    }
                }
            }
        }
        Adt14::C5 => {
            Ok(f447())
        }
        Adt14::C6(_, _) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C7(_, _, _) => {
            Ok(f389(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f472(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3030, _) = v0;
        h3030
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3031) = v0;
        h3031
    };
    let m3032 = v1;
    match m3032.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3033 = v4;
            match m3033.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f481(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f473(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3034, _) = v0.clone();
        h3034
    };
    let v2: List<(Bytes, ())> = {
        let (h3035, _) = {
            let (_, h3036) = v0.clone();
            h3036
        };
        h3035
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3037) = {
            let (_, h3038) = v0;
            h3038
        };
        h3037
    };
    let m3039 = v1.clone();
    match m3039.uncons() {
        None => {
            let m3040 = v2;
            match m3040.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3041 = v2.clone();
            match m3041.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f482({
                        let (h3042, _) = v8.clone();
                        h3042
                    }, {
                        let (h3043, _) = v6.clone();
                        h3043
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

pub fn f474(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3044, _) = v1;
        h3044
    };
    let m3046 = {
        let (_, h3045) = v1;
        h3045
    };
    match m3046 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3049 = {
                let a3047 = v0.clone();
                let a3048 = v2;
                index_bytes(a3047, a3048)
            };
            match m3049 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3054 = {
                        let a3052 = v0;
                        let a3053 = {
                            let a3050 = v2;
                            let a3051 = 1u64;
                            nat_add(a3050, a3051)?
                        };
                        index_bytes(a3052, a3053)
                    };
                    match m3054 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f483(v4)? {
                                f483(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3055 = v2;
                                    let a3056 = 2u64;
                                    nat_add(a3055, a3056)?
                                }, Some({
                                    let a3061 = v3;
                                    let a3062 = f395(f484(), {
                                        let a3059 = {
                                            let a3057 = f485(v4)?;
                                            let a3058 = 16u64;
                                            nat_mul(a3057, a3058)?
                                        };
                                        let a3060 = f485(v5)?;
                                        nat_add(a3059, a3060)?
                                    });
                                    append_bytes(a3061, a3062)
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

pub fn f475(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3065 = {
        let a3063 = v0;
        let a3064 = v1;
        index_bytes(a3063, a3064)
    };
    match m3065 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3110 = {
                    let a3086 = {
                        let a3074 = {
                            let a3068 = {
                                let a3066 = v3;
                                let a3067 = 128u8;
                                fixed_u8::bit_and(a3066, a3067)
                            };
                            let a3069 = 128u8;
                            if equal(a3068, a3069) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3075 = {
                            let a3072 = {
                                let a3070 = v3;
                                let a3071 = 64u8;
                                fixed_u8::bit_and(a3070, a3071)
                            };
                            let a3073 = 64u8;
                            if equal(a3072, a3073) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3074, a3075)?
                    };
                    let a3087 = {
                        let a3084 = {
                            let a3078 = {
                                let a3076 = v3;
                                let a3077 = 32u8;
                                fixed_u8::bit_and(a3076, a3077)
                            };
                            let a3079 = 32u8;
                            if equal(a3078, a3079) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3085 = {
                            let a3082 = {
                                let a3080 = v3;
                                let a3081 = 16u8;
                                fixed_u8::bit_and(a3080, a3081)
                            };
                            let a3083 = 16u8;
                            if equal(a3082, a3083) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3084, a3085)?
                    };
                    nat_add(a3086, a3087)?
                };
                let a3111 = {
                    let a3108 = {
                        let a3096 = {
                            let a3090 = {
                                let a3088 = v3;
                                let a3089 = 8u8;
                                fixed_u8::bit_and(a3088, a3089)
                            };
                            let a3091 = 8u8;
                            if equal(a3090, a3091) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3097 = {
                            let a3094 = {
                                let a3092 = v3;
                                let a3093 = 4u8;
                                fixed_u8::bit_and(a3092, a3093)
                            };
                            let a3095 = 4u8;
                            if equal(a3094, a3095) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3096, a3097)?
                    };
                    let a3109 = {
                        let a3106 = {
                            let a3100 = {
                                let a3098 = v3;
                                let a3099 = 2u8;
                                fixed_u8::bit_and(a3098, a3099)
                            };
                            let a3101 = 2u8;
                            if equal(a3100, a3101) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3107 = {
                            let a3104 = {
                                let a3102 = v3;
                                let a3103 = 1u8;
                                fixed_u8::bit_and(a3102, a3103)
                            };
                            let a3105 = 1u8;
                            if equal(a3104, a3105) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3106, a3107)?
                    };
                    nat_add(a3108, a3109)?
                };
                nat_add(a3110, a3111)?
            };
            let a3112 = 128u64;
            let a3113 = v4;
            if if nat_le(a3112, a3113) {
                let a3114 = v4;
                let a3115 = 191u64;
                nat_le(a3114, a3115)
            } else {
                false
            } {
                Ok(Some({
                    let a3116 = v4;
                    let a3117 = 128u64;
                    nat_sub(a3116, a3117)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f476(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3118 = v1;
    let a3119 = v0;
    if if nat_le(a3118, a3119) {
        let a3120 = v0;
        let a3121 = v2;
        if nat_le(a3120, a3121) {
            let a3122 = v0;
            let a3123 = 55296u64;
            if nat_lt(a3122, a3123) {
                true
            } else {
                let a3124 = 57343u64;
                let a3125 = v0;
                nat_lt(a3124, a3125)
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

pub fn f477(p3142: u64, p3143: u64, p3144: u64, p3145: Bytes) -> Bytes {
    let m3141 = (p3142, (p3143, (p3144, p3145)));
    {
        let mut m3141 = m3141;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3141;
                let m3126 = v0;
                if m3126 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3126 - 1;
                    let a3127 = v1;
                    let a3128 = 0u64;
                    if if nat_eq(a3127, a3128) {
                        let a3129 = v2;
                        let a3130 = 0u64;
                        nat_eq(a3129, a3130)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3131 = v1;
                            let a3132 = 16u64;
                            let a3133 = 0u64;
                            nat_quot(a3131, a3132, a3133)
                        }, ({
                            let a3134 = v2;
                            let a3135 = 1u64;
                            nat_sub(a3134, a3135)
                        }, {
                            let a3139 = f486({
                                let a3136 = v1;
                                let a3137 = 16u64;
                                let a3138 = 0u64;
                                nat_rem(a3136, a3137, a3138)
                            });
                            let a3140 = v3;
                            append_bytes(a3139, a3140)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f479() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil()))))))))))))))))))))
}

pub fn f480(v0: Adt14, v1: Bytes) -> bool {
    let m3146 = f142(v0, v1);
    match m3146 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f481(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3151 = {
            let a3149 = {
                let a3147 = v0.clone();
                length_list(a3147)
            };
            let a3150 = {
                let a3148 = v1.clone();
                length_list(a3148)
            };
            nat_add(a3149, a3150)?
        };
        let a3152 = 1u64;
        nat_add(a3151, a3152)?
    };
    Ok(f487({
        let (_, h3153) = {
            let (_, h3154) = {
                let (h3155, _) = f488(Fn25::F489, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3155
            };
            h3154
        };
        h3153
    }))
}

pub fn f482(v0: Bytes, v1: Bytes) -> bool {
    let a3160 = {
        let a3156 = v0;
        let a3157 = v1;
        compare_bytes(a3156, a3157)
    };
    let a3161 = {
        let a3158 = Bytes::lit(&[]);
        let a3159 = Bytes::lit(&[0x01]);
        compare_bytes(a3158, a3159)
    };
    equal(a3160, a3161)
}

pub fn f483(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3206 = {
            let a3182 = {
                let a3170 = {
                    let a3164 = {
                        let a3162 = v1;
                        let a3163 = 128u8;
                        fixed_u8::bit_and(a3162, a3163)
                    };
                    let a3165 = 128u8;
                    if equal(a3164, a3165) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3171 = {
                    let a3168 = {
                        let a3166 = v1;
                        let a3167 = 64u8;
                        fixed_u8::bit_and(a3166, a3167)
                    };
                    let a3169 = 64u8;
                    if equal(a3168, a3169) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3170, a3171)?
            };
            let a3183 = {
                let a3180 = {
                    let a3174 = {
                        let a3172 = v1;
                        let a3173 = 32u8;
                        fixed_u8::bit_and(a3172, a3173)
                    };
                    let a3175 = 32u8;
                    if equal(a3174, a3175) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3181 = {
                    let a3178 = {
                        let a3176 = v1;
                        let a3177 = 16u8;
                        fixed_u8::bit_and(a3176, a3177)
                    };
                    let a3179 = 16u8;
                    if equal(a3178, a3179) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3180, a3181)?
            };
            nat_add(a3182, a3183)?
        };
        let a3207 = {
            let a3204 = {
                let a3192 = {
                    let a3186 = {
                        let a3184 = v1;
                        let a3185 = 8u8;
                        fixed_u8::bit_and(a3184, a3185)
                    };
                    let a3187 = 8u8;
                    if equal(a3186, a3187) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3193 = {
                    let a3190 = {
                        let a3188 = v1;
                        let a3189 = 4u8;
                        fixed_u8::bit_and(a3188, a3189)
                    };
                    let a3191 = 4u8;
                    if equal(a3190, a3191) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3192, a3193)?
            };
            let a3205 = {
                let a3202 = {
                    let a3196 = {
                        let a3194 = v1;
                        let a3195 = 2u8;
                        fixed_u8::bit_and(a3194, a3195)
                    };
                    let a3197 = 2u8;
                    if equal(a3196, a3197) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3203 = {
                    let a3200 = {
                        let a3198 = v1;
                        let a3199 = 1u8;
                        fixed_u8::bit_and(a3198, a3199)
                    };
                    let a3201 = 1u8;
                    if equal(a3200, a3201) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3202, a3203)?
            };
            nat_add(a3204, a3205)?
        };
        nat_add(a3206, a3207)?
    };
    let a3208 = 48u64;
    let a3209 = v2;
    if if nat_le(a3208, a3209) {
        let a3210 = v2;
        let a3211 = 57u64;
        nat_le(a3210, a3211)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3212 = 97u64;
        let a3213 = v2;
        if nat_le(a3212, a3213) {
            let a3214 = v2;
            let a3215 = 102u64;
            Ok(nat_le(a3214, a3215))
        } else {
            Ok(false)
        }
    }
}

pub fn f484() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f485(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3260 = {
            let a3236 = {
                let a3224 = {
                    let a3218 = {
                        let a3216 = v1;
                        let a3217 = 128u8;
                        fixed_u8::bit_and(a3216, a3217)
                    };
                    let a3219 = 128u8;
                    if equal(a3218, a3219) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3225 = {
                    let a3222 = {
                        let a3220 = v1;
                        let a3221 = 64u8;
                        fixed_u8::bit_and(a3220, a3221)
                    };
                    let a3223 = 64u8;
                    if equal(a3222, a3223) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3224, a3225)?
            };
            let a3237 = {
                let a3234 = {
                    let a3228 = {
                        let a3226 = v1;
                        let a3227 = 32u8;
                        fixed_u8::bit_and(a3226, a3227)
                    };
                    let a3229 = 32u8;
                    if equal(a3228, a3229) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3235 = {
                    let a3232 = {
                        let a3230 = v1;
                        let a3231 = 16u8;
                        fixed_u8::bit_and(a3230, a3231)
                    };
                    let a3233 = 16u8;
                    if equal(a3232, a3233) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3234, a3235)?
            };
            nat_add(a3236, a3237)?
        };
        let a3261 = {
            let a3258 = {
                let a3246 = {
                    let a3240 = {
                        let a3238 = v1;
                        let a3239 = 8u8;
                        fixed_u8::bit_and(a3238, a3239)
                    };
                    let a3241 = 8u8;
                    if equal(a3240, a3241) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3247 = {
                    let a3244 = {
                        let a3242 = v1;
                        let a3243 = 4u8;
                        fixed_u8::bit_and(a3242, a3243)
                    };
                    let a3245 = 4u8;
                    if equal(a3244, a3245) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3246, a3247)?
            };
            let a3259 = {
                let a3256 = {
                    let a3250 = {
                        let a3248 = v1;
                        let a3249 = 2u8;
                        fixed_u8::bit_and(a3248, a3249)
                    };
                    let a3251 = 2u8;
                    if equal(a3250, a3251) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3257 = {
                    let a3254 = {
                        let a3252 = v1;
                        let a3253 = 1u8;
                        fixed_u8::bit_and(a3252, a3253)
                    };
                    let a3255 = 1u8;
                    if equal(a3254, a3255) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3256, a3257)?
            };
            nat_add(a3258, a3259)?
        };
        nat_add(a3260, a3261)?
    };
    let a3262 = v2;
    let a3263 = 57u64;
    if nat_le(a3262, a3263) {
        let a3264 = v2;
        let a3265 = 48u64;
        Ok(nat_sub(a3264, a3265))
    } else {
        let a3266 = v2;
        let a3267 = 87u64;
        Ok(nat_sub(a3266, a3267))
    }
}

pub fn f486(v0: u64) -> Bytes {
    f32({
        let a3268 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3268)
    }, v0, 1u64)
}

pub fn f487(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f336(Fn10::F490, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f488(v0: Fn25, p3273: u64, p3274: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3272 = (p3273, p3274);
    {
        let mut m3272 = m3272;
        loop {
            let step = {
                let (v1, v2) = m3272;
                let m3269 = v1;
                if m3269 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3269 - 1;
                    let m3271 = {
                        let c3270: Fn25 = v0.clone();
                        c3270.apply(v2.clone())
                    };
                    match m3271 {
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
                Ok(next) => { m3272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f491(v0)
}

pub fn f490(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f491(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3275, _) = v0.clone();
        h3275
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3276, _) = {
            let (_, h3277) = v0.clone();
            h3277
        };
        h3276
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3278) = {
            let (_, h3279) = v0;
            h3279
        };
        h3278
    };
    let m3280 = v1.clone();
    match m3280.uncons() {
        None => {
            let m3281 = v2;
            match m3281.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3282 = v2.clone();
            match m3282.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f482({
                        let (h3283, _) = v8.clone();
                        h3283
                    }, {
                        let (h3284, _) = v6.clone();
                        h3284
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
