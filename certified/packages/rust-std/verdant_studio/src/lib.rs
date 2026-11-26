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
    F191(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F191(k0) => f191(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F122,
    F246,
    F344,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F122 => f122(p0, p1),
            Fn1::F246 => f246(p0, p1),
            Fn1::F344 => f344(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F214(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F214(k0) => f214(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F314(Bytes),
    F315(Bytes),
    F331(Bytes, u64, Bytes, u64),
    F404(Bytes),
    F405(u64, Bytes),
    F412(Bytes),
    F472(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F314(k0) => f314(k0.clone(), p0),
            Fn3::F315(k0) => f315(k0.clone(), p0),
            Fn3::F331(k0, k1, k2, k3) => f331(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F404(k0) => Ok(f404(k0.clone(), p0)),
            Fn3::F405(k0, k1) => f405(*k0, k1.clone(), p0),
            Fn3::F412(k0) => f412(k0.clone(), p0),
            Fn3::F472(k0) => f472(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F318(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F318(k0) => f318(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F394(Bytes),
    F507(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F394(k0) => f394(k0.clone(), p0),
            Fn5::F507(k0, k1) => Ok(f507(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F396,
    F498,
    F658,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F396 => f396(p0, p1),
            Fn6::F498 => f498(p0, p1),
            Fn6::F658 => f658(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F399,
    F499,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F399 => f399(p0, p1),
            Fn7::F499 => f499(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F428,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn8::F428 => f428(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F431,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn9::F431 => f431(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F433,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn10::F433 => f433(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F436,
}

impl Fn11 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn11::F436 => f436(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F458(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn12::F458(k0, k1, k2) => f458(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F477,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn13::F477 => f477(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F480,
}

impl Fn14 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn14::F480 => f480(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F511,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F511 => f511(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F513,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F513 => f513(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F516(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F516(k0) => f516(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F534,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F534 => f534(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F540,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F540 => f540(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F544,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F544 => f544(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F590,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F590 => f590(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F593,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F593 => f593(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F620,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F620 => f620(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F644,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F644 => f644(p0),
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
                            if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                                f185(v3, v1)
                            } else {
                                if f32(v0, f8(Str::lit("record-error"))) {
                                    f186(v3, v1, v2)
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

pub fn f88(v0: Bytes) -> R<bool> {
    f187(v0.clone(), 0u64, {
        let a494 = v0;
        length_bytes(a494)
    })
}

pub fn f89(p499: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m498 = p499;
    {
        let mut m498 = m498;
        loop {
            let step = {
                let v0 = m498;
                let m495 = v0;
                match m495.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h496, _) = v2;
                            h496
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h497) = v2;
                                h497
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m503 = {
        let a501 = match v0 {
            Adt1::C0(h500, _, _, _) => {
                h500
            }
        };
        let a502 = f40(v1);
        index_list(a501, a502)
    };
    match m503 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m504 = f188(f189(v0.clone(), v1.clone()), v1.clone());
    match m504 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Adt2) -> Option<Adt14> {
    let m505 = v0;
    match m505 {
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
        Adt2::C6(r512) => {
            let v10 = (*r512).clone();
            let m506 = f92(v10);
            match m506 {
                None => {
                    None::<Adt14>
                }
                Some(v11) => {
                    Some(Adt14::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt2::C7(r513) => {
            let v12 = (*r513).clone();
            let m507 = f92(v12);
            match m507 {
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
        Adt2::C9(r514, r515) => {
            let v14 = (*r514).clone();
            let v15 = (*r515).clone();
            let m508 = f92(v14);
            match m508 {
                None => {
                    None::<Adt14>
                }
                Some(v16) => {
                    let m509 = f92(v15);
                    match m509 {
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
        Adt2::C10(v18, r516, r517) => {
            let v19 = (*r516).clone();
            let v20 = (*r517).clone();
            let m510 = f92(v19);
            match m510 {
                None => {
                    None::<Adt14>
                }
                Some(v21) => {
                    let m511 = f92(v20);
                    match m511 {
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
    let m518 = v0;
    match m518 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m519 = v1;
            match m519 {
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

pub fn f94(p525: Adt14, v1: Bytes) -> Option<Adt14> {
    let m524 = p525;
    {
        let mut m524 = m524;
        loop {
            let step = {
                let v0 = m524;
                let m520 = v0;
                match m520 {
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
                    Adt14::C4(r521) => {
                        let v5 = (*r521).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r522, r523) => {
                        let v3 = (*r522).clone();
                        let v4 = (*r523).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m526 = v0;
    match m526.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m527 = f190(v2, v1.clone())?;
            match m527 {
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
        let a528 = v0.clone();
        length_bytes(a528)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h529, _) = f76(Fn0::F191(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h529
    };
    let v3: u64 = {
        let (h530, _) = {
            let (_, h531) = v2.clone();
            h531
        };
        h530
    };
    let v4: List<Bytes> = {
        let (_, h532) = {
            let (_, h533) = v2;
            h533
        };
        h532
    };
    let a534 = v3;
    let a535 = v1;
    if nat_lt(a534, a535) {
        Ok(f78(List::cons(f46(v0, v3, {
            let a536 = v1;
            let a537 = v3;
            nat_sub(a536, a537)
        }), v4)))
    } else {
        Ok(f78(v4))
    }
}

pub fn f97(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f192(f57(v0, v1)?))
}

pub fn f98(v0: List<Adt14>, v1: Bytes) -> List<Adt14> {
    let m538 = v0;
    match m538.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f193(v2, v1.clone()), f98(v3, v1))
        }
    }
}

pub fn f99(p543: List<Bytes>, p544: List<Bytes>) -> bool {
    let m542 = (p543, p544);
    {
        let mut m542 = m542;
        loop {
            let step = {
                let (v0, v1) = m542;
                let m539 = v0;
                match m539.uncons() {
                    None => {
                        let m540 = v1;
                        match m540.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m541 = v1;
                        match m541.uncons() {
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
                Ok(next) => { m542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: List<Adt14>) -> List<Bytes> {
    let m545 = v0;
    match m545.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m546 = f94(v1, f8(Str::lit("std")));
            match m546 {
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
    let a547 = f194(v0)?;
    let a548 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a547, a548))
}

pub fn f102(v0: List<Adt14>) -> Adt14 {
    let m549 = v0;
    match m549.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f102(v2)))
        }
    }
}

pub fn f103(v0: Adt13) -> Bytes {
    let m550 = v0;
    match m550 {
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
    let a552 = {
        let a551 = v2.clone();
        length_bytes(a551)
    };
    let a553 = 0u64;
    if nat_eq(a552, a553) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m554 = f195(v0.clone(), f60())?;
        match m554 {
            None => {
                if f51(f52(f198(), match v0.clone() {
                    Adt1::C0(_, _, h555, _) => {
                        h555
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f198()))
                } else {
                    let m556 = f56(v0.clone(), List::cons(f199(), f200(f60())?));
                    match m556.uncons() {
                        None => {
                            Ok(Adt12::C3(f201(v0.clone(), f60(), v1.clone(), v2.clone(), f202(f64(v0, f199()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f196(v3.clone())?, f197(v3, v2)))
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

pub fn f115(v0: Bytes, p584: u64, p585: u64, p586: bool, p587: bool, p588: u64, p589: Bytes) -> R<Bytes> {
    let m583 = (p584, (p585, (p586, (p587, (p588, p589)))));
    {
        let mut m583 = m583;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m583;
                let m557 = v2;
                if m557 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f203(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m557 - 1;
                    let m560 = {
                        let a558 = v0.clone();
                        let a559 = v1;
                        index_bytes(a558, a559)
                    };
                    match m560 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f203(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f72(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a561 = v1;
                                    let a562 = 1u64;
                                    nat_add(a561, a562)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a563 = v9;
                                    let a564 = 34u64;
                                    !nat_eq(a563, a564)
                                }, (if v4 {
                                    false
                                } else {
                                    let a565 = v9;
                                    let a566 = 92u64;
                                    nat_eq(a565, a566)
                                }, (v5, v6))))))
                            } else {
                                let a567 = v9;
                                let a568 = 32u64;
                                if if if if nat_eq(a567, a568) {
                                    true
                                } else {
                                    let a569 = v9;
                                    let a570 = 9u64;
                                    nat_eq(a569, a570)
                                } {
                                    true
                                } else {
                                    let a571 = v9;
                                    let a572 = 10u64;
                                    nat_eq(a571, a572)
                                } {
                                    true
                                } else {
                                    let a573 = v9;
                                    let a574 = 13u64;
                                    nat_eq(a573, a574)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a575 = v1;
                                        let a576 = 1u64;
                                        nat_add(a575, a576)?
                                    }, (v7, (false, (false, ({
                                        let a577 = v1;
                                        let a578 = 1u64;
                                        nat_add(a577, a578)?
                                    }, f203(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a579 = v1;
                                        let a580 = 1u64;
                                        nat_add(a579, a580)?
                                    }, (v7, ({
                                        let a581 = v9;
                                        let a582 = 34u64;
                                        nat_eq(a581, a582)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<u64> {
    let m592 = {
        let a590 = v0;
        let a591 = v1;
        index_bytes(a590, a591)
    };
    match m592 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a637 = {
                let a613 = {
                    let a601 = {
                        let a595 = {
                            let a593 = v3;
                            let a594 = 128u8;
                            fixed_u8::bit_and(a593, a594)
                        };
                        let a596 = 128u8;
                        if equal(a595, a596) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a602 = {
                        let a599 = {
                            let a597 = v3;
                            let a598 = 64u8;
                            fixed_u8::bit_and(a597, a598)
                        };
                        let a600 = 64u8;
                        if equal(a599, a600) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a601, a602)?
                };
                let a614 = {
                    let a611 = {
                        let a605 = {
                            let a603 = v3;
                            let a604 = 32u8;
                            fixed_u8::bit_and(a603, a604)
                        };
                        let a606 = 32u8;
                        if equal(a605, a606) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a612 = {
                        let a609 = {
                            let a607 = v3;
                            let a608 = 16u8;
                            fixed_u8::bit_and(a607, a608)
                        };
                        let a610 = 16u8;
                        if equal(a609, a610) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a611, a612)?
                };
                nat_add(a613, a614)?
            };
            let a638 = {
                let a635 = {
                    let a623 = {
                        let a617 = {
                            let a615 = v3;
                            let a616 = 8u8;
                            fixed_u8::bit_and(a615, a616)
                        };
                        let a618 = 8u8;
                        if equal(a617, a618) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a624 = {
                        let a621 = {
                            let a619 = v3;
                            let a620 = 4u8;
                            fixed_u8::bit_and(a619, a620)
                        };
                        let a622 = 4u8;
                        if equal(a621, a622) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a623, a624)?
                };
                let a636 = {
                    let a633 = {
                        let a627 = {
                            let a625 = v3;
                            let a626 = 2u8;
                            fixed_u8::bit_and(a625, a626)
                        };
                        let a628 = 2u8;
                        if equal(a627, a628) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a634 = {
                        let a631 = {
                            let a629 = v3;
                            let a630 = 1u8;
                            fixed_u8::bit_and(a629, a630)
                        };
                        let a632 = 1u8;
                        if equal(a631, a632) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a633, a634)?
                };
                nat_add(a635, a636)?
            };
            nat_add(a637, a638)
        }
    }
}

pub fn f117(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f204(v0.clone(), v1, {
        let a640 = {
            let a639 = v0.clone();
            length_bytes(a639)
        };
        let a641 = 1u64;
        nat_add(a640, a641)?
    })?;
    let a642 = f116(v0.clone(), v2)?;
    let a643 = 34u64;
    if nat_eq(a642, a643) {
        Ok(Some(({
            let a644 = v2;
            let a645 = 1u64;
            nat_add(a644, a645)?
        }, f46(v0, v1, {
            let a646 = v2;
            let a647 = v1;
            nat_sub(a646, a647)
        }))))
    } else {
        f205(v0.clone(), v1, {
            let a649 = {
                let a648 = v0;
                length_bytes(a648)
            };
            let a650 = 1u64;
            nat_add(a649, a650)?
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
        let a651 = f116(v0.clone(), v1)?;
        let a652 = 45u64;
        nat_eq(a651, a652)
    };
    let v3: u64 = if v2 {
        let a653 = v1;
        let a654 = 1u64;
        nat_add(a653, a654)?
    } else {
        v1
    };
    let v4: u64 = f206(v0.clone(), v3)?;
    let v5: u64 = {
        let a655 = v4;
        let a656 = v3;
        nat_sub(a655, a656)
    };
    let v6: bool = {
        let a657 = f116(v0.clone(), v4)?;
        let a658 = 46u64;
        nat_eq(a657, a658)
    };
    let v7: u64 = if v6 {
        let a659 = v4;
        let a660 = 1u64;
        nat_add(a659, a660)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f206(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f116(v0.clone(), v8)?;
    let v10: bool = {
        let a661 = v9;
        let a662 = 101u64;
        if nat_eq(a661, a662) {
            true
        } else {
            let a663 = v9;
            let a664 = 69u64;
            nat_eq(a663, a664)
        }
    };
    let v11: u64 = f116(v0.clone(), {
        let a665 = v8;
        let a666 = 1u64;
        nat_add(a665, a666)?
    })?;
    let v12: bool = f118(v10, f207({
        let a667 = v11;
        let a668 = 43u64;
        nat_eq(a667, a668)
    }, {
        let a669 = v11;
        let a670 = 45u64;
        nat_eq(a669, a670)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a671 = v8;
            let a672 = 2u64;
            nat_add(a671, a672)?
        } else {
            let a673 = v8;
            let a674 = 1u64;
            nat_add(a673, a674)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f206(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a675 = v14;
        let a676 = v13;
        nat_sub(a675, a676)
    });
    let v16: u64 = f208(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a678 = {
            let a677 = v15;
            length_bytes(a677)
        };
        let a679 = v16;
        nat_sub(a678, a679)
    });
    let a680 = v5;
    let a681 = 0u64;
    if if nat_eq(a680, a681) {
        true
    } else {
        if f118({
            let a682 = 1u64;
            let a683 = v5;
            nat_lt(a682, a683)
        }, {
            let a684 = f116(v0.clone(), v3)?;
            let a685 = 48u64;
            nat_eq(a684, a685)
        }) {
            true
        } else {
            if f118(v6, {
                let a686 = v8;
                let a687 = v7;
                nat_eq(a686, a687)
            }) {
                true
            } else {
                f207(if v10 {
                    let a688 = v14;
                    let a689 = v13;
                    nat_eq(a688, a689)
                } else {
                    false
                }, {
                    let a691 = 18u64;
                    let a692 = {
                        let a690 = v17.clone();
                        length_bytes(a690)
                    };
                    nat_lt(a691, a692)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m696 = {
            let a694 = {
                let a693 = v17.clone();
                length_bytes(a693)
            };
            let a695 = 0u64;
            if nat_eq(a694, a695) {
                Some(0u64)
            } else {
                f209(v17)?
            }
        };
        match m696 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m701 = f210(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a697 = v8;
                    let a698 = v7;
                    nat_sub(a697, a698)
                }), f118(v12, {
                    let a699 = v11;
                    let a700 = 45u64;
                    nat_eq(a699, a700)
                }), v18)?;
                match m701 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a704 = 19u64;
                        let a705 = {
                            let a703 = {
                                let (h702, _) = v19.clone();
                                h702
                            };
                            length_bytes(a703)
                        };
                        if nat_lt(a704, a705) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m707 = f209({
                                let (h706, _) = v19.clone();
                                h706
                            })?;
                            match m707 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f118(v2, {
                                        let a708 = 0u64;
                                        let a709 = v20;
                                        nat_lt(a708, a709)
                                    });
                                    if f211(v21, v20) {
                                        Ok(Some((v14, {
                                            let a711 = {
                                                let (_, h710) = v19.clone();
                                                h710
                                            };
                                            let a712 = 0u64;
                                            if nat_eq(a711, a712) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h713) = v19;
                                                    h713
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
        let (h714, _) = v2.clone();
        h714
    };
    let v4: u64 = {
        let (h715, _) = {
            let (_, h716) = v2.clone();
            h716
        };
        h715
    };
    let v5: List<Bytes> = {
        let (_, h717) = {
            let (_, h718) = v2;
            h718
        };
        h717
    };
    let m719 = f212(v0.clone(), v3);
    match m719 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a720 = v6;
            let a721 = v1;
            if equal(a720, a721) {
                Ok(Some(({
                    let a722 = v3;
                    let a723 = 1u64;
                    nat_add(a722, a723)?
                }, ({
                    let a724 = v3;
                    let a725 = 1u64;
                    nat_add(a724, a725)?
                }, List::cons(f46(v0, v4, {
                    let a726 = v3;
                    let a727 = v4;
                    nat_sub(a726, a727)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a728 = v3;
                    let a729 = 1u64;
                    nat_add(a728, a729)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f121(v0: Fn1, p733: List<Bytes>, p734: List<Bytes>) -> List<Bytes> {
    let m732 = (p733, p734);
    {
        let mut m732 = m732;
        loop {
            let step = {
                let (v1, v2) = m732;
                let m730 = v2;
                match m730.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c731: Fn1 = v0.clone();
                            c731.apply(v1, v3)
                        }, v4))
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

pub fn f122(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m735 = f213(Fn2::F214(v1), None::<Bytes>, v0);
    match m735 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))
}

pub fn f125(v0: Adt1) -> List<Bytes> {
    if f151(match v0.clone() {
        Adt1::C0(_, h736, _, _) => {
            h736
        }
    }, f215()) {
        if f151(match v0 {
            Adt1::C0(_, h737, _, _) => {
                h737
            }
        }, f216()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f216(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f215(), List::<Bytes>::nil())
    }
}

pub fn f126(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m738 = v0;
    match m738.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f217(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f137(v0)?)))))
}

pub fn f128(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f148(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f149(f97(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f129(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m739 = v0;
    match m739.uncons() {
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
    let m740 = v0;
    match m740.uncons() {
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
    let m741 = v0;
    match m741.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f126(f150(f192(f94(v1, f8(Str::lit("defers"))))), f131(v2))
        }
    }
}

pub fn f132(v0: List<Adt14>) -> List<Bytes> {
    let m742 = v0;
    match m742.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m743 = f94(v1, f8(Str::lit("std")));
            match m743 {
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
    let m744 = v0;
    match m744.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f162(f218(v2.clone()), f219(f220(v0, v1.clone(), v2)?));
    f221(f153(f222(v1))?, v3.clone(), {
        let a746 = {
            let a745 = v3;
            length_list(a745)
        };
        let a747 = 1u64;
        nat_add(a746, a747)?
    })
}

pub fn f135(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m748 = v1;
    match m748.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f223(v2.clone(), f8(Str::lit("subject"))) {
                f224(v0.clone(), v2.clone())
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
    let m749 = v1;
    match m749.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m750 = f225(f96(f64(v0.clone(), v2.clone()))?, f85(v2, f216()))?;
            match m750 {
                None => {
                    f136(v0, v3)
                }
                Some(v4) => {
                    Ok(f226(v4, f136(v0, v3)?))
                }
            }
        }
    }
}

pub fn f137(v0: Adt1) -> R<List<Bytes>> {
    Ok(f126(f153(f227(f215(), f155(match v0.clone() {
        Adt1::C0(_, h751, _, _) => {
            h751
        }
    }, f215())))?, f153(f227(f216(), f155(match v0 {
        Adt1::C0(_, h752, _, _) => {
            h752
        }
    }, f216())))?))
}

pub fn f138(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f139() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f217(), List::<Bytes>::nil())))))))
}

pub fn f140(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m753 = v1;
    match m753.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m754 = f57(v0.clone(), v2.clone())?;
                match m754 {
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
    let m755 = v1;
    match m755.uncons() {
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
    let m756 = v0;
    match m756.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f126(f228({
                let (h757, _) = v2;
                h757
            }, {
                let (_, h758) = v2;
                h758
            }, v1), f143(v3, v1))
        }
    }
}

pub fn f144() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f145(v0: Adt1) -> R<Option<Bytes>> {
    f229(f230(v0, f144())?)
}

pub fn f146(v0: Adt1, p762: List<Bytes>) -> R<Option<Bytes>> {
    let m761 = p762;
    {
        let mut m761 = m761;
        loop {
            let step = {
                let v1 = m761;
                let m759 = v1;
                match m759.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m760 = f231(f64(v0.clone(), v2.clone()), f232())?;
                        match m760 {
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
                Ok(next) => { m761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: Adt17) -> R<(bool, Bytes)> {
    let v1: Adt16 = match v0.clone() {
        Adt17::C0(h763, _, _, _, _) => {
            h763
        }
    };
    let v2: Adt18 = f233(v1.clone())?;
    let v3: List<Bytes> = f234(v1.clone());
    let v4: List<Bytes> = f149(match v0.clone() {
        Adt17::C0(_, _, h764, _, _) => {
            h764
        }
    }, f8(Str::lit("path")));
    let m766 = f235(v2, List::<u64>::nil(), f236(v1.clone(), match v0.clone() {
        Adt17::C0(_, h765, _, _, _) => {
            h765
        }
    }, 0u64)?);
    match m766 {
        None => {
            if f238(f239(match v0.clone() {
                Adt17::C0(_, _, _, _, h768) => {
                    h768
                }
            }, f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h769) => {
                    h769
                }
            }, v4.clone()))?, f239(v4.clone(), f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h770) => {
                    h770
                }
            }, v4.clone()))?, f240(match v0.clone() {
                Adt17::C0(_, _, h771, _, _) => {
                    h771
                }
            })) {
                let m775 = f241(f239(match v0.clone() {
                    Adt17::C0(_, _, _, h772, _) => {
                        h772
                    }
                }, f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h773, _) => {
                        h773
                    }
                }))?, f239(v3.clone(), f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h774, _) => {
                        h774
                    }
                }))?);
                match m775 {
                    None => {
                        let m778 = f242(f149(match v0 {
                            Adt17::C0(_, h777, _, _, _) => {
                                h777
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m778 {
                            None => {
                                let m779 = f242(v4, List::<Bytes>::nil());
                                match m779 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f237(f126(v3, match v0 {
                            Adt17::C0(_, _, _, h776, _) => {
                                h776
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f237(match v1 {
                Adt16::C0(_, _, _, _, h767) => {
                    h767
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f148(v0: Adt1) -> R<List<Bytes>> {
    let m780 = f243(f64(v0, f217()))?;
    match m780.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f244(v2))
        }
    }
}

pub fn f149(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m781 = v0;
    match m781.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f149(v3, v1))
        }
    }
}

pub fn f150(v0: List<Adt14>) -> List<Bytes> {
    let m782 = v0;
    match m782.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m783 = f93(Some(v1));
            match m783 {
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

pub fn f151(p787: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m786 = p787;
    {
        let mut m786 = m786;
        loop {
            let step = {
                let v0 = m786;
                let m784 = v0;
                match m784.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h785, _) = v2;
                            h785
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m786 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f153(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f245(f121(Fn1::F246, List::<Bytes>::nil(), f247(v0)?)))
}

pub fn f154(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m788 = v1;
    match m788.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f248(v0.clone(), v2), f154(v0, v3))
            } else {
                f154(v0, v3)
            }
        }
    }
}

pub fn f155(p793: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m792 = p793;
    {
        let mut m792 = m792;
        loop {
            let step = {
                let v0 = m792;
                let m789 = v0;
                match m789.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h790, _) = v2;
                            h790
                        }, v1.clone()) {
                            let (_, h791) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h791)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f156(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p796: List<Bytes>) -> R<Option<Bytes>> {
    let m795 = p796;
    {
        let mut m795 = m795;
        loop {
            let step = {
                let v2 = m795;
                let m794 = v2;
                match m794.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f249(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m797 = v0;
    match m797.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f250(v1.clone())?, v1), f157(v2)?))
        }
    }
}

pub fn f158(p800: List<Bytes>, p801: List<Bytes>, p802: List<List<Bytes>>) -> List<List<Bytes>> {
    let m799 = (p800, (p801, p802));
    {
        let mut m799 = m799;
        loop {
            let step = {
                let (v0, (v1, v2)) = m799;
                let m798 = v0;
                match m798.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f251(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f251(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m799 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m803 = v0;
    match m803.uncons() {
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
        Adt1::C0(_, h804, _, _) => {
            h804
        }
    }, f252())) {
        Ok(Adt12::C1(List::cons(f252(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f159(f153(f154(f252(), f155(match v0.clone() {
            Adt1::C0(_, h805, _, _) => {
                h805
            }
        }, f252())))?, v1.clone());
        let m806 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m806.uncons() {
            None => {
                let v5: List<Bytes> = f149(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m807 = f56(v0.clone(), f253(v0.clone(), v2.clone(), v5.clone())?);
                match m807.uncons() {
                    None => {
                        let m808 = f254(v0.clone(), v2, v5)?;
                        match m808 {
                            None => {
                                f255(v0, v1)
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
    let m809 = v0;
    match m809.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f162(v3, v1))
        }
    }
}

pub fn f163(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m810 = f97(v0.clone(), f161())?;
    match m810.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f256(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f257(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op > target/attempt.txt 2>&1; then\u{a}                tee -a target/operation.txt < target/attempt.txt\u{a}                last=$op\u{a}              else\u{a}                cat target/attempt.txt\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f167() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f257(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
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
    let v4: List<List<Bytes>> = f258(f243(f64(v0, f217()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f259(v4.clone(), f244(v4), f220(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f134(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f171(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f260(f128(v0.clone())?, f136(v0.clone(), f137(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f172(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f261(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f262(f97(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f173() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f174(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m811 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m811.uncons() {
        None => {
            if f263(f96(f64(v0, v1))?)? {
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
    let m812 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m812.uncons() {
        None => {
            let v5: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f264(v5)?
                } else {
                    f265(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f248(f266(), f267(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f248(f266(), f267(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f268()
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
    let m813 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m813.uncons() {
        None => {
            let m814 = f269(f96(f64(v0.clone(), v1.clone()))?)?;
            match m814 {
                None => {
                    Ok(Adt12::C4(f270()))
                }
                Some(v4) => {
                    let m815 = f271(f96(f64(v0, v1))?)?;
                    match m815 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f272(v5)))
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
    let m816 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m816.uncons() {
        None => {
            let m817 = f273(f96(f64(v0, v1))?)?;
            match m817 {
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
    let m818 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m818.uncons() {
        None => {
            let v5: Bytes = f248(if v2 {
                f216()
            } else {
                f215()
            }, f267(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m819 = f225(f96(f64(v0.clone(), v1.clone()))?, v2)?;
                match m819 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a821 = {
                            let a820 = v6;
                            length_list(a820)
                        };
                        let a822 = 0u64;
                        if nat_eq(a821, a822) {
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
    let m823 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m823.uncons() {
        None => {
            let v4: List<Bytes> = f96(f64(v0.clone(), v1))?;
            let m824 = f274(v4.clone(), f8(Str::lit("requirement")))?;
            match m824 {
                None => {
                    Ok(Adt12::C4(f268()))
                }
                Some(v5) => {
                    let m825 = f274(v4, f8(Str::lit("admission")))?;
                    match m825 {
                        None => {
                            Ok(Adt12::C4(f268()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f126(f149(v5.clone(), f8(Str::lit("id"))), f149(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f153(f126(f149(v7.clone(), f8(Str::lit("id"))), f149(v8.clone(), f8(Str::lit("construct")))))?;
                            let m826 = f275(f132(v6.clone()), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m826 {
                                None => {
                                    let a828 = {
                                        let a827 = v9.clone();
                                        length_list(a827)
                                    };
                                    let a829 = 0u64;
                                    if nat_eq(a828, a829) {
                                        Ok(Adt12::C4(f268()))
                                    } else {
                                        if f51({
                                            let a832 = {
                                                let a830 = f153(f126(v9.clone(), v10.clone()))?;
                                                length_list(a830)
                                            };
                                            let a833 = {
                                                let a831 = f126(v9.clone(), v10);
                                                length_list(a831)
                                            };
                                            nat_eq(a832, a833)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f277(f8(Str::lit("model/requirements.json")), f226(v7, v5))?, List::cons(f277(f8(Str::lit("model/admissions.json")), f226(v8, v6))?, List::cons(f278(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f123(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f276(v11))
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
    let m834 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m834.uncons() {
        None => {
            let m835 = f91(v0.clone(), v1.clone())?;
            match m835 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m836 = f279(v4.clone(), f8(Str::lit("schema")));
                    match m836 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f280(v0, f281(v5.clone())?, v5, f282(v4.clone(), f8(Str::lit("name"))), f283(f284(f279(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
    let m837 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m837.uncons() {
        None => {
            let m838 = f57(v0.clone(), v1.clone())?;
            match m838 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f285())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m839 = f94(v4.clone(), f8(Str::lit("source")));
                            match m839 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f93(f94(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f248(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f248(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m840 = v8;
                        match m840 {
                            None => {
                                f286(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f286(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f248(v9, f267(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h841, _) => {
                                            h841
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m842 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m842.uncons() {
                                            None => {
                                                if f32(f287(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f286(v0, v4, v5, v10, Some((v12, v11)))
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
    let m843 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m843.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f243(v4.clone())?;
            if f51(f99(f288(v5.clone()), f289())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f290(Str::lit("id"), v6.clone())?, List::cons(f290(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f290(Str::lit("artifact"), f217())?, List::cons(f290(Str::lit("checksum"), f287(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f217(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f291(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f278(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f123(f244(f258(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f183(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m844 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m844.uncons() {
        None => {
            let m845 = f57(v0, v1)?;
            match m845 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f292(f192(Some(v4))) {
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
    let m846 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m846.uncons() {
        None => {
            let m847 = f91(v0.clone(), v1.clone())?;
            match m847 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f280(v0.clone(), f64(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f185(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m848 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f161(), List::<Bytes>::nil())));
    match m848.uncons() {
        None => {
            let m849 = f293(f96(f64(v0.clone(), v1))?)?;
            match m849 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f97(v0, f161())?;
                    let v6: List<Bytes> = f149(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f153(f149(v5.clone(), f8(Str::lit("at"))))?;
                    let a851 = {
                        let a850 = v6.clone();
                        length_list(a850)
                    };
                    let a852 = 0u64;
                    if if nat_eq(a851, a852) {
                        true
                    } else {
                        f51({
                            let a855 = {
                                let a853 = f153(f126(v6.clone(), v7.clone()))?;
                                length_list(a853)
                            };
                            let a856 = {
                                let a854 = f126(v6.clone(), v7);
                                length_list(a854)
                            };
                            nat_eq(a855, a856)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f277(f161(), f226(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f123(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f186(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m857 = v2;
    match m857.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m858 = v4;
            match m858.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m859 = v6.clone();
                    match m859.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m860 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f294(), List::<Bytes>::nil())));
                            match m860.uncons() {
                                None => {
                                    let v11: Bytes = f123(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt14> = f97(v0, f294())?;
                                    if f295(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f277(f294(), f226(v13, List::cons(f296(v11.clone(), v12, v5, v3), List::<Adt14>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f187(v0: Bytes, p914: u64, p915: u64) -> R<bool> {
    let m913 = (p914, p915);
    {
        let mut m913 = m913;
        loop {
            let step = {
                let (v1, v2) = m913;
                let m861 = v2;
                if m861 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m861 - 1;
                    let m864 = {
                        let a862 = v0.clone();
                        let a863 = v1;
                        index_bytes(a862, a863)
                    };
                    match m864 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f297({
                                let v5: u8 = v4;
                                let a909 = {
                                    let a885 = {
                                        let a873 = {
                                            let a867 = {
                                                let a865 = v5;
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
                                                let a869 = v5;
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
                                                let a875 = v5;
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
                                                let a879 = v5;
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
                                                let a887 = v5;
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
                                                let a891 = v5;
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
                                                let a897 = v5;
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
                                                let a901 = v5;
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
                                nat_add(a909, a910)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a911 = v1;
                                    let a912 = 1u64;
                                    nat_add(a911, a912)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m913 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(p920: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m919 = p920;
    {
        let mut m919 = m919;
        loop {
            let step = {
                let v0 = m919;
                let m916 = v0;
                match m916.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h917, _) = v2;
                            h917
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h918) = v2;
                                h918
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
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

pub fn f189(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m924 = {
        let a922 = match v0 {
            Adt1::C0(_, _, _, h921) => {
                h921
            }
        };
        let a923 = f40(v1);
        index_list(a922, a923)
    };
    match m924 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f190(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m925 = f44(v0, 34u8)?;
        match m925.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m926 = v3;
                match m926.uncons() {
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

pub fn f191(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f298(v0, v1)
}

pub fn f192(v0: Option<Adt14>) -> List<Adt14> {
    let m927 = v0;
    match m927 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f299(v1)
        }
    }
}

pub fn f193(v0: Adt14, v1: Bytes) -> Adt14 {
    let m928 = v0;
    match m928 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r929) => {
            let v10 = (*r929).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r930) => {
            let v5 = (*r930).clone();
            Adt14::C4(std::rc::Rc::new(f193(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r931, r932) => {
            let v11 = (*r931).clone();
            let v12 = (*r932).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r933, r934) => {
            let v3 = (*r933).clone();
            let v4 = (*r934).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt14::C7(v2, std::rc::Rc::new(f300(v3, v1.clone())), std::rc::Rc::new(f193(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f193(v4, v1)))
            }
        }
    }
}

pub fn f194(v0: Adt14) -> R<Bytes> {
    let m935 = v0;
    match m935 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f301(v2, v3)
        }
        Adt14::C2(v4) => {
            f302(v4)
        }
        Adt14::C3(r936) => {
            let v5 = (*r936).clone();
            Ok(f303(f304(v5)?))
        }
        Adt14::C4(r937) => {
            let v6 = (*r937).clone();
            f305(f306(v6)?)
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

pub fn f195(v0: Adt1, p941: List<Adt13>) -> R<Option<Adt13>> {
    let m940 = p941;
    {
        let mut m940 = m940;
        loop {
            let step = {
                let v1 = m940;
                let m938 = v1;
                match m938.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f196(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h939, _) => {
                                h939
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
                Ok(next) => { m940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Adt13) -> R<Bytes> {
    Ok(f248(f307(), f267(f308(v0))?))
}

pub fn f197(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f308(v0), List::<Bytes>::nil())))))
}

pub fn f198() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f199() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f200(v0: List<Adt13>) -> R<List<Bytes>> {
    let m942 = v0;
    match m942.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f196(v1)?, f200(v2)?))
        }
    }
}

pub fn f201(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m943 = v1;
    match m943.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f309(v5.clone(), v2.clone(), v3.clone(), f287(f64(v0.clone(), f196(v5)?))?, v4.clone())?), f201(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f202(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f310(v0.clone())?;
    let v2: u64 = f311(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a944 = v1;
        let a945 = v2;
        nat_sub(a944, a945)
    }))
}

pub fn f203(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a948 = v3;
    let a949 = f46(v0, v1, {
        let a946 = v2;
        let a947 = v1;
        nat_sub(a946, a947)
    });
    append_bytes(a948, a949)
}

pub fn f204(v0: Bytes, p962: u64, p963: u64) -> R<u64> {
    let m961 = (p962, p963);
    {
        let mut m961 = m961;
        loop {
            let step = {
                let (v1, v2) = m961;
                let m950 = v2;
                if m950 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m950 - 1;
                    let v4: u64 = f116(v0.clone(), v1)?;
                    let a951 = v4;
                    let a952 = 34u64;
                    if nat_eq(a951, a952) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a953 = v4;
                        let a954 = 92u64;
                        if nat_eq(a953, a954) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a955 = v4;
                            let a956 = 32u64;
                            if nat_lt(a955, a956) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a957 = v4;
                                let a958 = 256u64;
                                if nat_eq(a957, a958) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a959 = v1;
                                        let a960 = 1u64;
                                        nat_add(a959, a960)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m961 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, p989: u64, p990: u64, p991: Bytes) -> R<Option<(u64, Bytes)>> {
    let m988 = (p989, (p990, p991));
    {
        let mut m988 = m988;
        loop {
            let step = {
                let (v1, (v2, v3)) = m988;
                let m964 = v2;
                if m964 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m964 - 1;
                    let v5: u64 = f204(v0.clone(), v1, {
                        let a968 = {
                            let a966 = {
                                let a965 = v0.clone();
                                length_bytes(a965)
                            };
                            let a967 = v1;
                            nat_sub(a966, a967)
                        };
                        let a969 = 1u64;
                        nat_add(a968, a969)?
                    })?;
                    let v6: Bytes = {
                        let a972 = v3;
                        let a973 = f46(v0.clone(), v1, {
                            let a970 = v5;
                            let a971 = v1;
                            nat_sub(a970, a971)
                        });
                        append_bytes(a972, a973)
                    };
                    let v7: u64 = f116(v0.clone(), v5)?;
                    let a974 = v7;
                    let a975 = 34u64;
                    if nat_eq(a974, a975) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a976 = v5;
                            let a977 = 1u64;
                            nat_add(a976, a977)?
                        }, v6))))
                    } else {
                        let a978 = v7;
                        let a979 = 92u64;
                        if nat_eq(a978, a979) {
                            let m980 = f312(v0.clone(), v5, v6)?;
                            match m980 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a983 = {
                                        let (_, h981) = {
                                            let (_, h982) = v8.clone();
                                            h982
                                        };
                                        h981
                                    };
                                    let a984 = 0u64;
                                    if nat_eq(a983, a984) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h985, _) = v8;
                                            h985
                                        }, (v4, {
                                            let (h986, _) = {
                                                let (_, h987) = v8;
                                                h987
                                            };
                                            h986
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
                Ok(next) => { m988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes, v1: u64) -> R<u64> {
    let (h992, _) = f313(Fn3::F314(v0.clone()), {
        let a994 = {
            let a993 = v0;
            length_bytes(a993)
        };
        let a995 = 1u64;
        nat_add(a994, a995)?
    }, v1)?;
    Ok(h992)
}

pub fn f207(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f208(v0: Bytes) -> R<u64> {
    let (h996, _) = f313(Fn3::F315(v0.clone()), {
        let a998 = {
            let a997 = v0;
            length_bytes(a997)
        };
        let a999 = 1u64;
        nat_add(a998, a999)?
    }, 0u64)?;
    Ok(h996)
}

pub fn f209(v0: Bytes) -> R<Option<u64>> {
    if f316(v0.clone())? {
        Ok(Some({
            let (_, h1000) = {
                let (h1001, _) = f317(Fn4::F318(v0.clone()), {
                    let a1002 = v0;
                    length_bytes(a1002)
                }, (0u64, 0u64))?;
                h1001
            };
            h1000
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f210(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a1004 = {
            let a1003 = v1.clone();
            length_bytes(a1003)
        };
        let a1005 = f319(v1)?;
        nat_sub(a1004, a1005)
    });
    let v5: Bytes = {
        let a1006 = v0;
        let a1007 = v4.clone();
        append_bytes(a1006, a1007)
    };
    let v6: u64 = f208(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a1009 = {
            let a1008 = v5;
            length_bytes(a1008)
        };
        let a1010 = v6;
        nat_sub(a1009, a1010)
    });
    let v8: u64 = {
        let a1011 = v4;
        length_bytes(a1011)
    };
    let v9: u64 = {
        let a1012 = v7.clone();
        length_bytes(a1012)
    };
    let a1013 = v9;
    let a1014 = 0u64;
    if nat_eq(a1013, a1014) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f207(v2, {
            let a1015 = v3;
            let a1016 = v8;
            nat_lt(a1015, a1016)
        }) {
            let v10: u64 = if v2 {
                let a1017 = v3;
                let a1018 = v8;
                nat_add(a1017, a1018)?
            } else {
                let a1019 = v8;
                let a1020 = v3;
                nat_sub(a1019, a1020)
            };
            let v11: u64 = f319(v7.clone())?;
            let a1021 = v10;
            let a1022 = v11;
            if nat_le(a1021, a1022) {
                Ok(Some((f46(v7, 0u64, {
                    let a1023 = v9;
                    let a1024 = v10;
                    nat_sub(a1023, a1024)
                }), 0u64)))
            } else {
                let a1027 = {
                    let a1025 = v10;
                    let a1026 = v11;
                    nat_sub(a1025, a1026)
                };
                let a1028 = 19u64;
                if nat_le(a1027, a1028) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1029 = v9;
                        let a1030 = v11;
                        nat_sub(a1029, a1030)
                    }), {
                        let a1031 = v10;
                        let a1032 = v11;
                        nat_sub(a1031, a1032)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1033 = v3;
                let a1034 = v8;
                nat_sub(a1033, a1034)
            };
            let a1037 = {
                let a1035 = v9;
                let a1036 = v12;
                nat_add(a1035, a1036)?
            };
            let a1038 = 19u64;
            if nat_le(a1037, a1038) {
                Ok(Some(({
                    let a1039 = v7;
                    let a1040 = f320(v12);
                    append_bytes(a1039, a1040)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f211(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1041 = 0u64;
        let a1042 = v1;
        if nat_lt(a1041, a1042) {
            let a1043 = v1;
            let a1044 = 9223372036854775808u64;
            nat_le(a1043, a1044)
        } else {
            false
        }
    } else {
        let a1045 = v1;
        let a1046 = 9223372036854775807u64;
        nat_le(a1045, a1046)
    }
}

pub fn f212(v0: Bytes, v1: u64) -> Option<u8> {
    let a1047 = v0;
    let a1048 = v1;
    index_bytes(a1047, a1048)
}

pub fn f213(v0: Fn2, p1052: Option<Bytes>, p1053: List<Bytes>) -> Option<Bytes> {
    let m1051 = (p1052, p1053);
    {
        let mut m1051 = m1051;
        loop {
            let step = {
                let (v1, v2) = m1051;
                let m1049 = v2;
                match m1049.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1050: Fn2 = v0.clone();
                            c1050.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1051 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1054 = v1;
    match m1054 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1057 = {
                    let a1055 = v3;
                    let a1056 = v0;
                    append_bytes(a1055, a1056)
                };
                let a1058 = v2;
                append_bytes(a1057, a1058)
            })
        }
    }
}

pub fn f215() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f216() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f217() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f218(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1059 = v0;
    match m1059.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f218(v2))
        }
    }
}

pub fn f219(v0: List<Adt19>) -> List<(Bytes, Bytes)> {
    let m1060 = v0;
    match m1060.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt19::C0(h1061, _, _) => {
                    h1061
                }
            }, match v1 {
                Adt19::C0(_, _, h1062) => {
                    h1062
                }
            }), f219(v2))
        }
    }
}

pub fn f220(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt19>> {
    let v3: List<(Bytes, Bytes)> = f321(v1);
    let v4: List<(Bytes, Bytes)> = f322(v0, v2);
    f323(v3.clone(), f162(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f221(p1069: List<Bytes>, v1: List<(Bytes, Bytes)>, p1070: u64) -> R<List<Bytes>> {
    let m1068 = (p1069, p1070);
    {
        let mut m1068 = m1068;
        loop {
            let step = {
                let (v0, v2) = m1068;
                let m1063 = v2;
                if m1063 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1063 - 1;
                    let v4: List<Bytes> = f153(f126(v0.clone(), f324(v0.clone(), v1.clone())))?;
                    let a1066 = {
                        let a1064 = v4.clone();
                        length_list(a1064)
                    };
                    let a1067 = {
                        let a1065 = v0.clone();
                        length_list(a1065)
                    };
                    if nat_eq(a1066, a1067) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: List<Adt14>) -> List<Bytes> {
    let m1071 = v0;
    match m1071.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1072 = f94(v1, f8(Str::lit("std")));
            match m1072 {
                None => {
                    f222(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f223(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f222(v2))
                    } else {
                        f222(v2)
                    })
                }
            }
        }
    }
}

pub fn f223(v0: Adt14, v1: Bytes) -> bool {
    f51({
        let a1074 = {
            let a1073 = f58(v0, v1);
            length_bytes(a1073)
        };
        let a1075 = 0u64;
        nat_eq(a1074, a1075)
    })
}

pub fn f224(v0: Adt15, v1: Adt14) -> bool {
    if f325(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f325(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f225(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m1076 = v0;
    match m1076.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f202(v2)?;
            let a1078 = {
                let a1077 = v4.clone();
                length_bytes(a1077)
            };
            let a1079 = 0u64;
            if nat_eq(a1078, a1079) {
                f225(v3, v1)
            } else {
                let m1080 = f326(v4)?;
                match m1080 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f327(v5.clone())
                        } else {
                            f328(v5.clone())
                        } {
                            let m1081 = f225(v3, v1)?;
                            match m1081 {
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

pub fn f226(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m1082 = v0;
    match m1082.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f226(v3, v1))
        }
    }
}

pub fn f227(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1083 = v1;
    match m1083.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f248(v0.clone(), v2), f227(v0, v3))
            } else {
                f227(v0, v3)
            }
        }
    }
}

pub fn f228(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1084 = v1;
    match m1084.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f329(v3.clone())
            } else {
                f51(f329(v3.clone()))
            } {
                List::cons(f248(v0.clone(), v3), f228(v0, v4, v2))
            } else {
                f228(v0, v4, v2)
            }
        }
    }
}

pub fn f229(p1087: List<Bytes>) -> R<Option<Bytes>> {
    let m1086 = p1087;
    {
        let mut m1086 = m1086;
        loop {
            let step = {
                let v0 = m1086;
                let m1085 = v0;
                match m1085.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f330(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f202(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1086 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1088 = v1;
    match m1088.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f96(f64(v0.clone(), v2))?, f230(v0, v3)?))
        }
    }
}

pub fn f231(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1089 = v0.clone();
        length_bytes(a1089)
    };
    let v3: u64 = {
        let a1090 = v1.clone();
        length_bytes(a1090)
    };
    let v4: u64 = {
        let (h1091, _) = f313(Fn3::F331(v1.clone(), v3, v0.clone(), v2), {
            let a1092 = v2;
            let a1093 = 1u64;
            nat_add(a1092, a1093)?
        }, 0u64)?;
        h1091
    };
    if f332(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f232() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f233(v0: Adt16) -> R<Adt18> {
    Ok(Adt18::C0(f333(match v0.clone() {
        Adt16::C0(h1094, _, _, _, _) => {
            h1094
        }
    }, 0u64)?, f333(match v0.clone() {
        Adt16::C0(_, h1095, _, _, _) => {
            h1095
        }
    }, 0u64)?, f333(match v0.clone() {
        Adt16::C0(_, _, h1096, _, _) => {
            h1096
        }
    }, 0u64)?, f333(match v0 {
        Adt16::C0(_, _, _, h1097, _) => {
            h1097
        }
    }, 0u64)?))
}

pub fn f234(v0: Adt16) -> List<Bytes> {
    f126(match v0.clone() {
        Adt16::C0(h1098, _, _, _, _) => {
            h1098
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, h1099, _, _) => {
            h1099
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, _, h1100, _) => {
            h1100
        }
    }, match v0 {
        Adt16::C0(_, _, _, _, h1101) => {
            h1101
        }
    })))
}

pub fn f235(v0: Adt18, p1107: List<u64>, p1108: List<Adt20>) -> Option<u64> {
    let m1106 = (p1107, p1108);
    {
        let mut m1106 = m1106;
        loop {
            let step = {
                let (v1, v2) = m1106;
                let m1102 = v2;
                match m1102.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f334(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1103) => {
                                h1103
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1104, _) => {
                                    h1104
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1105, _) => {
                                    h1105
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Adt16, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m1109 = v1;
    match m1109.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f335(v0.clone(), v3)?), f236(v0, v4, {
                let a1110 = v2;
                let a1111 = 1u64;
                nat_add(a1110, a1111)?
            })?))
        }
    }
}

pub fn f237(p1115: List<Bytes>, p1116: u64) -> Bytes {
    let m1114 = (p1115, p1116);
    {
        let mut m1114 = m1114;
        loop {
            let step = {
                let (v0, v1) = m1114;
                let m1112 = v0;
                match m1112.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1113 = v1;
                        if m1113 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1113 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f336(v0.clone(), v1.clone()) {
        if f336(v1, v0) {
            f337(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f239(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1117 = v0;
    match m1117.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f338(v2, v1.clone())?, f239(v3, v1)?))
        }
    }
}

pub fn f240(v0: List<Adt14>) -> List<u64> {
    let m1118 = v0;
    match m1118.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f339(v1), f240(v2))
        }
    }
}

pub fn f241(p1121: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1120 = p1121;
    {
        let mut m1120 = m1120;
        loop {
            let step = {
                let v0 = m1120;
                let m1119 = v0;
                match m1119.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f340(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(p1124: List<Bytes>, p1125: List<Bytes>) -> Option<Bytes> {
    let m1123 = (p1124, p1125);
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let (v0, v1) = m1123;
                let m1122 = v0;
                match m1122.uncons() {
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
                Ok(next) => { m1123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt23 = f341(v0.clone(), 0u64, {
        let a1126 = v0;
        length_bytes(a1126)
    }, Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt23 = {
        let a1129 = {
            let a1128 = match v1.clone() {
                Adt23::C0(_, _, h1127, _, _) => {
                    h1127
                }
            };
            length_bytes(a1128)
        };
        let a1130 = 0u64;
        if if nat_eq(a1129, a1130) {
            let m1132 = match v1.clone() {
                Adt23::C0(_, _, _, h1131, _) => {
                    h1131
                }
            };
            match m1132.uncons() {
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
            f342(v1)
        }
    };
    Ok(f343(match v4 {
        Adt23::C0(_, _, _, _, h1133) => {
            h1133
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f244(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1134 = v0;
    match m1134.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1135 = v1;
            match m1135.uncons() {
                None => {
                    f244(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f244(v2))
                }
            }
        }
    }
}

pub fn f245(v0: List<Bytes>) -> List<Bytes> {
    f121(Fn1::F344, List::<Bytes>::nil(), v0)
}

pub fn f246(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f345(v0, v1)
}

pub fn f247(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f346(f347(f348(v0))?))
}

pub fn f248(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f249(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1136 = f57(v0, v2)?;
    match m1136 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f349(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1137 = v4.clone();
            match m1137.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1139 = f93(f94({
                        let m1138 = f94(v3.clone(), f8(Str::lit("source")));
                        match m1138 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1139 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f350(f351(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f250(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1140 = v0;
    match m1140.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f352(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1144 = f44(f46(v1.clone(), 6u64, {
                    let a1142 = {
                        let a1141 = v1;
                        length_bytes(a1141)
                    };
                    let a1143 = 6u64;
                    nat_sub(a1142, a1143)
                }), 34u8)?;
                match m1144.uncons() {
                    None => {
                        f250(v2)
                    }
                    Some((v3, v4)) => {
                        let m1145 = v4;
                        match m1145.uncons() {
                            None => {
                                f250(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f250(v2)?))
                            }
                        }
                    }
                }
            } else {
                f250(v2)
            }
        }
    }
}

pub fn f251(p1148: List<Bytes>, p1149: List<Bytes>) -> List<Bytes> {
    let m1147 = (p1148, p1149);
    {
        let mut m1147 = m1147;
        loop {
            let step = {
                let (v0, v1) = m1147;
                let m1146 = v0;
                match m1146.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f253(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1150 = v1;
    match m1150.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f353(v0.clone(), v3)?;
            if f52(f354(f282(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f355(v5)?, f253(v0, v4, v2)?))
            } else {
                f253(v0, v4, v2)
            }
        }
    }
}

pub fn f254(v0: Adt1, p1154: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1153 = p1154;
    {
        let mut m1153 = m1153;
        loop {
            let step = {
                let v1 = m1153;
                let m1151 = v1;
                match m1151.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f353(v0.clone(), v3.clone())?;
                        if if f52(f354(f282(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f281({
                                let m1152 = f279(v5.clone(), f8(Str::lit("schema")));
                                match m1152 {
                                    None => {
                                        Adt2::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f355(v5)?))
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
                Ok(next) => { m1153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f151(match v0.clone() {
        Adt1::C0(_, h1155, _, _) => {
            h1155
        }
    }, f266())) {
        Ok(Adt12::C1(List::cons(f266(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f159(f153(f356(f155(match v0.clone() {
            Adt1::C0(_, h1156, _, _) => {
                h1156
            }
        }, f266())))?, v1);
        let m1157 = f56(v0.clone(), List::cons(f161(), List::cons(f217(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1157.uncons() {
            None => {
                let v5: (bool, Bytes) = f357(v0.clone(), v2)?;
                let (h1158, _) = v5;
                if h1158 {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h1159) = v5;
                        h1159
                    }))
                } else {
                    let m1160 = f125(v0.clone());
                    match m1160.uncons() {
                        None => {
                            let m1161 = f127(v0.clone())?;
                            match m1161.uncons() {
                                None => {
                                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h1162) = v5;
                                        h1162
                                    }, List::cons(f7(f153(f358(f128(v0.clone())?, f136(v0.clone(), f137(v0.clone())?)?))?), List::cons(f359(v0)?, List::<Bytes>::nil()))))))
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

pub fn f256(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f360(f97(v0, f161())?)?)))
}

pub fn f257() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f258(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1163 = v0;
    match m1163.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f259(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt19>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m1164 = v1;
    match m1164.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f126(f361(v5.clone(), v2.clone()), f362(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f363(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1166 = {
                    let a1165 = v7.clone();
                    length_list(a1165)
                };
                let a1167 = 0u64;
                if nat_eq(a1166, a1167) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f123(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f259(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f260(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1168 = v1;
    match m1168.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f223(v2.clone(), f8(Str::lit("domain"))) {
                f364(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1169 = f94(v2.clone(), f8(Str::lit("role")));
                    match m1169 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f365(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1170 = f94(v2.clone(), f8(Str::lit("origin")));
                    match m1170 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1171 = f94(v2, f8(Str::lit("evidence")));
                    match m1171 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f260(v0, v3))
            } else {
                f260(v0, v3)
            }
        }
    }
}

pub fn f261(v0: List<Adt14>) -> List<Bytes> {
    let m1172 = v0;
    match m1172.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f261(v2))
        }
    }
}

pub fn f262(v0: List<Adt14>) -> List<Bytes> {
    let m1173 = v0;
    match m1173.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f366(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f262(v2))
        }
    }
}

pub fn f263(p1179: List<Bytes>) -> R<bool> {
    let m1178 = p1179;
    {
        let mut m1178 = m1178;
        loop {
            let step = {
                let v0 = m1178;
                let m1174 = v0;
                match m1174.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1176 = {
                            let a1175 = f202(v1)?;
                            length_bytes(a1175)
                        };
                        let a1177 = 0u64;
                        if nat_eq(a1176, a1177) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: List<Bytes>) -> R<bool> {
    if f367(f293(v0.clone())?) {
        Ok(f51({
            let a1181 = {
                let a1180 = f368(f293(v0)?);
                length_list(a1180)
            };
            let a1182 = 0u64;
            nat_eq(a1181, a1182)
        }))
    } else {
        Ok(false)
    }
}

pub fn f265(v0: List<Bytes>) -> R<bool> {
    if f367(f274(v0.clone(), f8(Str::lit("requirement")))?) {
        if f367(f274(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1184 = {
                    let a1183 = f226(f368(f274(v0.clone(), f8(Str::lit("requirement")))?), f368(f274(v0, f8(Str::lit("admission")))?));
                    length_list(a1183)
                };
                let a1185 = 0u64;
                nat_eq(a1184, a1185)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f266() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f267(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f268() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f269(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1186 = v0;
    match m1186.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f202(v1)?;
            let a1188 = {
                let a1187 = v3.clone();
                length_bytes(a1187)
            };
            let a1189 = 0u64;
            if nat_eq(a1188, a1189) {
                f269(v2)
            } else {
                let m1190 = f326(v3.clone())?;
                match m1190 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f369(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1191 = f269(v2)?;
                            match m1191 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f370(v5), {
                                        let a1192 = v3;
                                        let a1193 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1192, a1193)
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

pub fn f270() -> Bytes {
    let a1194 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1194)
}

pub fn f271(p1199: List<Bytes>) -> R<Option<Bytes>> {
    let m1198 = p1199;
    {
        let mut m1198 = m1198;
        loop {
            let step = {
                let v0 = m1198;
                let m1195 = v0;
                match m1195.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1196 = f326(f202(v1)?)?;
                        match m1196 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1197 = f94(v3.clone(), f8(Str::lit("source")));
                                match m1197 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f371(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272(v0: Bytes) -> Bytes {
    let a1204 = {
        let a1200 = Str::lit("binding ");
        utf8_encode(a1200)
    };
    let a1205 = {
        let a1202 = v0;
        let a1203 = {
            let a1201 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1201)
        };
        append_bytes(a1202, a1203)
    };
    append_bytes(a1204, a1205)
}

pub fn f273(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1206 = v0;
    match m1206.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f202(v1)?;
            let a1208 = {
                let a1207 = v3.clone();
                length_bytes(a1207)
            };
            let a1209 = 0u64;
            if nat_eq(a1208, a1209) {
                f273(v2)
            } else {
                let m1210 = f43(v3.clone())?;
                match m1210 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f372(v4.clone()) {
                            let m1211 = f273(v2)?;
                            match m1211 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f252(), List::cons(f8(Str::lit("/")), List::cons(f373(f282(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1212 = v3;
                                        let a1213 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1212, a1213)
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

pub fn f274(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m1214 = v0;
    match m1214.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f202(v2)?;
            let a1216 = {
                let a1215 = v4.clone();
                length_bytes(a1215)
            };
            let a1217 = 0u64;
            if nat_eq(a1216, a1217) {
                f274(v3, v1)
            } else {
                let m1218 = f326(v4)?;
                match m1218 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f374(v5.clone());
                        let a1220 = {
                            let a1219 = v6.clone();
                            length_bytes(a1219)
                        };
                        let a1221 = 0u64;
                        if nat_eq(a1220, a1221) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m1222 = f274(v3, v1.clone())?;
                            match m1222 {
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

pub fn f275(p1225: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1224 = p1225;
    {
        let mut m1224 = m1224;
        loop {
            let step = {
                let v0 = m1224;
                let m1223 = v0;
                match m1223.uncons() {
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
                Ok(next) => { m1224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f277(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f101(Adt14::C3(std::rc::Rc::new(f102(v1))))?))
}

pub fn f278(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f101(Adt14::C3(std::rc::Rc::new(f375(f153(f126(v1, f150(f97(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f279(p1231: Adt2, v1: Bytes) -> Option<Adt2> {
    let m1230 = p1231;
    {
        let mut m1230 = m1230;
        loop {
            let step = {
                let v0 = m1230;
                let m1226 = v0;
                match m1226 {
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
                    Adt2::C7(r1227) => {
                        let v5 = (*r1227).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C8 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(v2, r1228, r1229) => {
                        let v3 = (*r1228).clone();
                        let v4 = (*r1229).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1232 = f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1232.uncons() {
        None => {
            let m1234 = f275(f132(List::cons(Adt14::C4(std::rc::Rc::new({
                let m1233 = f376(v4.clone());
                match m1233 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1234 {
                None => {
                    f377(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f276(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f281(v0: Adt2) -> R<Bytes> {
    let a1235 = f378(v0)?;
    let a1236 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1235, a1236))
}

pub fn f282(v0: Adt2, v1: Bytes) -> Bytes {
    let m1237 = f379(f279(v0, v1));
    match m1237 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f283(v0: List<Adt2>) -> List<Bytes> {
    let m1238 = v0;
    match m1238.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1239 = f379(Some(v1));
            match m1239 {
                None => {
                    f283(v2)
                }
                Some(v3) => {
                    List::cons(v3, f283(v2))
                }
            }
        }
    }
}

pub fn f284(v0: Option<Adt2>) -> List<Adt2> {
    let m1240 = v0;
    match m1240 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f380(v1)
        }
    }
}

pub fn f285() -> Bytes {
    f381()
}

pub fn f286(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f291(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f382(v1, v4)?)?), List::cons(f278(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f287(v0: Bytes) -> R<Bytes> {
    Ok(f383(f384(v0)?))
}

pub fn f288(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1241 = v0;
    match m1241.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f289() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f290(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f350(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f291(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f385(f158(f96(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f386(f387(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f292(p1244: List<Adt14>) -> bool {
    let m1243 = p1244;
    {
        let mut m1243 = m1243;
        loop {
            let step = {
                let v0 = m1243;
                let m1242 = v0;
                match m1242.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f149(f388(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m1245 = v0;
    match m1245.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f202(v1)?;
            let a1247 = {
                let a1246 = v3.clone();
                length_bytes(a1246)
            };
            let a1248 = 0u64;
            if nat_eq(a1247, a1248) {
                f293(v2)
            } else {
                let m1249 = f326(v3)?;
                match m1249 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f223(v4.clone(), f8(Str::lit("agent"))) {
                            if f223(v4.clone(), f8(Str::lit("at"))) {
                                if f223(v4.clone(), f8(Str::lit("quote"))) {
                                    f389(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1250 = f293(v2)?;
                            match m1250 {
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

pub fn f294() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f295(p1253: List<Adt14>, v1: Bytes, v2: Bytes) -> bool {
    let m1252 = p1253;
    {
        let mut m1252 = m1252;
        loop {
            let step = {
                let v0 = m1252;
                let m1251 = v0;
                match m1251.uncons() {
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
                Ok(next) => { m1252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt14 {
    Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt14::C2(v2)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt14::C2(v0)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5))))))))))))
}

pub fn f297(v0: u64) -> bool {
    let a1254 = v0;
    let a1255 = 127u64;
    nat_le(a1254, a1255)
}

pub fn f298(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1256, _) = v1.clone();
        h1256
    };
    let v3: u64 = {
        let (h1257, _) = {
            let (_, h1258) = v1.clone();
            h1258
        };
        h1257
    };
    let v4: List<Bytes> = {
        let (_, h1259) = {
            let (_, h1260) = v1;
            h1260
        };
        h1259
    };
    let m1261 = f212(v0.clone(), v2);
    match m1261 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1262 = v5;
            let a1263 = 10u8;
            if equal(a1262, a1263) {
                Ok(Some(({
                    let a1264 = v2;
                    let a1265 = 1u64;
                    nat_add(a1264, a1265)?
                }, ({
                    let a1266 = v2;
                    let a1267 = 1u64;
                    nat_add(a1266, a1267)?
                }, List::cons(f390(f46(v0, v3, {
                    let a1268 = v2;
                    let a1269 = v3;
                    nat_sub(a1268, a1269)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1270 = v2;
                    let a1271 = 1u64;
                    nat_add(a1270, a1271)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f299(v0: Adt14) -> List<Adt14> {
    let m1272 = v0;
    match m1272 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1273) => {
            let v3 = (*r1273).clone();
            f299(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1274, r1275) => {
            let v1 = (*r1274).clone();
            let v2 = (*r1275).clone();
            List::cons(v1, f299(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f300(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1276 = v0;
    match m1276 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r1277) => {
            let v10 = (*r1277).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r1278) => {
            let v5 = (*r1278).clone();
            Adt14::C4(std::rc::Rc::new(f300(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1279, r1280) => {
            let v11 = (*r1279).clone();
            let v12 = (*r1280).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r1281, r1282) => {
            let v3 = (*r1281).clone();
            let v4 = (*r1282).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt14::C7(v2, std::rc::Rc::new(f391(v3, v1.clone())), std::rc::Rc::new(f300(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f300(v4, v1)))
            }
        }
    }
}

pub fn f301(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1283 = 0u64;
        let a1284 = v1;
        nat_lt(a1283, a1284)
    } else {
        false
    } {
        let a1285 = Bytes::lit(&[0x2d]);
        let a1286 = f392(v1)?;
        Ok(append_bytes(a1285, a1286))
    } else {
        f392(v1)
    }
}

pub fn f302(v0: Bytes) -> R<Bytes> {
    let a1292 = {
        let a1290 = Bytes::lit(&[0x22]);
        let a1291 = {
            let (_, h1287) = {
                let (h1288, _) = f393(Fn5::F394(v0.clone()), {
                    let a1289 = v0;
                    length_bytes(a1289)
                }, (0u64, Bytes::lit(&[])))?;
                h1288
            };
            h1287
        };
        append_bytes(a1290, a1291)
    };
    let a1293 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1292, a1293))
}

pub fn f303(v0: List<Bytes>) -> Bytes {
    let a1296 = {
        let a1294 = Bytes::lit(&[0x5b]);
        let a1295 = f123(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1294, a1295)
    };
    let a1297 = Bytes::lit(&[0x5d]);
    append_bytes(a1296, a1297)
}

pub fn f304(v0: Adt14) -> R<List<Bytes>> {
    let m1298 = v0;
    match m1298 {
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
        Adt14::C6(r1299, r1300) => {
            let v7 = (*r1299).clone();
            let v8 = (*r1300).clone();
            Ok(List::cons(f194(v7)?, f304(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f305(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f395(Fn6::F396, List::<(Bytes, Bytes)>::nil(), f397(v0)?);
    let v2: List<Bytes> = f398(Fn7::F399, List::<Bytes>::nil(), v1)?;
    let a1303 = {
        let a1301 = Bytes::lit(&[0x7b]);
        let a1302 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1301, a1302)
    };
    let a1304 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1303, a1304))
}

pub fn f306(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1305 = v0;
    match m1305 {
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
        Adt14::C7(v9, r1306, r1307) => {
            let v10 = (*r1306).clone();
            let v11 = (*r1307).clone();
            Ok(List::cons((v9, f194(v10)?), f306(v11)?))
        }
    }
}

pub fn f307() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f308(v0: Adt13) -> Bytes {
    let m1308 = v0;
    match m1308 {
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

pub fn f309(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f400(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f400(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f400(f8(Str::lit("name")), f401(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f400(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f400(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f400(f8(Str::lit("statement")), f402(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f400(f8(Str::lit("media_type")), f403(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f400(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f400(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f400(f8(Str::lit("url")), f197(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f310(v0: Bytes) -> R<u64> {
    let (h1309, _) = f313(Fn3::F404(v0.clone()), {
        let a1310 = v0.clone();
        length_bytes(a1310)
    }, {
        let a1311 = v0;
        length_bytes(a1311)
    })?;
    Ok(h1309)
}

pub fn f311(v0: Bytes, v1: u64) -> R<u64> {
    let (h1312, _) = f313(Fn3::F405(v1, v0), v1, 0u64)?;
    Ok(h1312)
}

pub fn f312(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f116(v0.clone(), {
        let a1313 = v1;
        let a1314 = 1u64;
        nat_add(a1313, a1314)?
    })?;
    let v4: Bytes = {
        let a1315 = v3;
        let a1316 = 117u64;
        if nat_eq(a1315, a1316) {
            f406(v0, v1)?
        } else {
            f407(v3)
        }
    };
    let a1317 = v4.clone();
    let a1318 = Bytes::lit(&[]);
    if equal(a1317, a1318) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1319 = v3;
            let a1320 = 117u64;
            if nat_eq(a1319, a1320) {
                let a1321 = v1;
                let a1322 = 6u64;
                nat_add(a1321, a1322)?
            } else {
                let a1323 = v1;
                let a1324 = 2u64;
                nat_add(a1323, a1324)?
            }
        }, ({
            let a1325 = v2;
            let a1326 = v4;
            append_bytes(a1325, a1326)
        }, 0u64))))
    }
}

pub fn f313(v0: Fn3, p1331: u64, p1332: u64) -> R<(u64, bool)> {
    let m1330 = (p1331, p1332);
    {
        let mut m1330 = m1330;
        loop {
            let step = {
                let (v1, v2) = m1330;
                let m1327 = v1;
                if m1327 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1327 - 1;
                    let m1329 = {
                        let c1328: Fn3 = v0.clone();
                        c1328.apply(v2)?
                    };
                    match m1329 {
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
                Ok(next) => { m1330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f314(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f408(v0, v1)
}

pub fn f315(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f409(v0, v1)
}

pub fn f316(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1333 = v0.clone();
        length_bytes(a1333)
    };
    let a1334 = 0u64;
    let a1335 = v1;
    if nat_lt(a1334, a1335) {
        let a1336 = f410(v0.clone())?;
        let a1337 = v1;
        if nat_eq(a1336, a1337) {
            let a1338 = 1u64;
            let a1339 = v1;
            if nat_lt(a1338, a1339) {
                let m1342 = {
                    let a1340 = v0;
                    let a1341 = 0u64;
                    index_bytes(a1340, a1341)
                };
                match m1342 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1343 = v2;
                        let a1344 = 48u8;
                        Ok(!equal(a1343, a1344))
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

pub fn f317(v0: Fn4, p1349: u64, p1350: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1348 = (p1349, p1350);
    {
        let mut m1348 = m1348;
        loop {
            let step = {
                let (v1, v2) = m1348;
                let m1345 = v1;
                if m1345 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1345 - 1;
                    let m1347 = {
                        let c1346: Fn4 = v0.clone();
                        c1346.apply(v2)?
                    };
                    match m1347 {
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
                Ok(next) => { m1348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f411(v0, v1)
}

pub fn f319(v0: Bytes) -> R<u64> {
    let (h1351, _) = f313(Fn3::F412(v0.clone()), {
        let a1353 = {
            let a1352 = v0;
            length_bytes(a1352)
        };
        let a1354 = 1u64;
        nat_add(a1353, a1354)?
    }, 0u64)?;
    Ok(h1351)
}

pub fn f320(v0: u64) -> Bytes {
    let m1355 = v0;
    if m1355 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1355 - 1;
        let a1356 = f8(Str::lit("0"));
        let a1357 = f320(v1);
        append_bytes(a1356, a1357)
    }
}

pub fn f321(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1358 = v0;
    match m1358.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1359 = f94(v1, f8(Str::lit("std")));
            match m1359 {
                None => {
                    f321(v2)
                }
                Some(v3) => {
                    if f223(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f321(v2))
                    } else {
                        f321(v2)
                    }
                }
            }
        }
    }
}

pub fn f322(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1360 = v1;
    match m1360.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt15::C0(h1361, _, _) => {
                    h1361
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1362, _) => {
                        h1362
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f322(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt15::C0(h1363, _, _) => {
                        h1363
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1364, _) => {
                            h1364
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f322(v0, v3))
                } else {
                    f322(v0, v3)
                }
            }
        }
    }
}

pub fn f323(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt19>> {
    let m1365 = v0;
    match m1365.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v4, v5)) => {
            if if f413(v4.clone(), v2.clone()) {
                true
            } else {
                f413(v4.clone(), v3.clone())
            } {
                f323(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt19::C0({
                    let (h1366, _) = v4.clone();
                    h1366
                }, {
                    let a1369 = {
                        let a1368 = f153(f414({
                            let (h1367, _) = v4.clone();
                            h1367
                        }, v1.clone()))?;
                        length_list(a1368)
                    };
                    let a1370 = 1u64;
                    if nat_eq(a1369, a1370) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1371) = v4.clone();
                    h1371
                }), f323(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f324(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1372 = v1;
    match m1372.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1373, _) = v2.clone();
                h1373
            }, v0.clone()) {
                List::cons({
                    let (_, h1374) = v2;
                    h1374
                }, f324(v0, v3))
            } else {
                if f52({
                    let (_, h1375) = v2;
                    h1375
                }, v0.clone()) {
                    List::cons({
                        let (h1376, _) = v2;
                        h1376
                    }, f324(v0, v3))
                } else {
                    f324(v0, v3)
                }
            }
        }
    }
}

pub fn f325(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt15::C0(h1377, _, _) => {
            h1377
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1378, _) => {
                h1378
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt15::C0(_, _, h1379) => {
                    h1379
                }
            }) {
                true
            } else {
                if f223(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1380, _) => {
                            h1380
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f326(v0: Bytes) -> R<Option<Adt14>> {
    let m1381 = f43(v0)?;
    match m1381 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v1) => {
            Ok(f92(v1))
        }
    }
}

pub fn f327(v0: Adt14) -> bool {
    if f223(v0.clone(), f8(Str::lit("domain"))) {
        if f223(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f415()) {
                if f416(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f416(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f416(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f416(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f328(v0: Adt14) -> bool {
    if f223(v0.clone(), f8(Str::lit("subject"))) {
        if f223(v0.clone(), f8(Str::lit("object"))) {
            if f223(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f417())
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

pub fn f329(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f330(v0: Bytes) -> R<bool> {
    let v1: Bytes = f202(v0)?;
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
        let a1383 = {
            let a1382 = v1;
            length_bytes(a1382)
        };
        let a1384 = v2;
        nat_sub(a1383, a1384)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1385 = f231(v3.clone(), f8(Str::lit(")")))?;
        match m1385 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1386 = v4;
                    let a1387 = 1u64;
                    nat_add(a1386, a1387)?
                }, {
                    let a1391 = {
                        let a1388 = v3;
                        length_bytes(a1388)
                    };
                    let a1392 = {
                        let a1389 = v4;
                        let a1390 = 1u64;
                        nat_add(a1389, a1390)?
                    };
                    nat_sub(a1391, a1392)
                })
            }
        }
    } else {
        v3
    };
    let a1393 = 0u64;
    let a1394 = v2;
    if nat_lt(a1393, a1394) {
        f418(v5)
    } else {
        Ok(false)
    }
}

pub fn f331(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f332(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1397 = {
            let a1395 = v4;
            let a1396 = v1;
            nat_add(a1395, a1396)?
        };
        let a1398 = v3;
        if nat_lt(a1397, a1398) {
            Ok(Some({
                let a1399 = v4;
                let a1400 = 1u64;
                nat_add(a1399, a1400)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f332(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1405 = {
        let a1402 = v0;
        let a1403 = v2;
        let a1404 = {
            let a1401 = v1.clone();
            length_bytes(a1401)
        };
        slice_bytes(a1402, a1403, a1404)
    };
    match m1405 {
        None => {
            false
        }
        Some(v3) => {
            let a1406 = v3;
            let a1407 = v1;
            equal(a1406, a1407)
        }
    }
}

pub fn f333(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1408 = v0;
    match m1408.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f333(v3, {
                let a1409 = v1;
                let a1410 = 1u64;
                nat_add(a1409, a1410)?
            })?))
        }
    }
}

pub fn f334(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1411 = v2;
    match m1411 {
        Adt21::C0(v3, v4) => {
            if f340(v3, match v0.clone() {
                Adt18::C0(h1412, _, _, _) => {
                    h1412
                }
            }) {
                f419(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f340(v5, match v0 {
                Adt18::C0(_, _, h1413, _) => {
                    h1413
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f336(v7, v1) {
                f340(v8, match v0 {
                    Adt18::C0(_, _, _, h1414) => {
                        h1414
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f335(v0: Adt16, v1: Adt14) -> R<Adt21> {
    let m1415 = f94(v1.clone(), f8(Str::lit("std")));
    match m1415 {
        None => {
            let m1420 = f93(f94(v1.clone(), f8(Str::lit("req"))));
            match m1420 {
                None => {
                    let m1422 = f94(v1, f8(Str::lit("der")));
                    match m1422 {
                        None => {
                            Ok(f420(v0))
                        }
                        Some(v5) => {
                            let m1423 = f421(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1423 {
                                None => {
                                    Ok(f420(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f239(f150(f192(f94(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt16::C0(_, _, _, _, h1424) => {
                                            h1424
                                        }
                                    })?, f338(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt16::C0(_, _, _, h1425, _) => {
                                            h1425
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f338(v4, match v0 {
                        Adt16::C0(_, _, h1421, _, _) => {
                            h1421
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1416 = f93(f94(v2.clone(), f8(Str::lit("row"))));
            match m1416 {
                None => {
                    Ok(Adt21::C0(f338(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt16::C0(h1417, _, _, _, _) => {
                            h1417
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f338(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt16::C0(h1418, _, _, _, _) => {
                            h1418
                        }
                    })?, Some(f338(v3, match v0 {
                        Adt16::C0(_, h1419, _, _, _) => {
                            h1419
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f336(p1428: List<u64>, v1: List<u64>) -> bool {
    let m1427 = p1428;
    {
        let mut m1427 = m1427;
        loop {
            let step = {
                let v0 = m1427;
                let m1426 = v0;
                match m1426.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f340(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f337(p1433: List<u64>) -> bool {
    let m1432 = p1433;
    {
        let mut m1432 = m1432;
        loop {
            let step = {
                let v0 = m1432;
                let m1429 = v0;
                match m1429.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1430 = v1;
                        let a1431 = 7u64;
                        if nat_le(a1430, a1431) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f338(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1434 = f422(v0, v1.clone(), 0u64)?;
    match m1434 {
        None => {
            let a1435 = v1;
            Ok(length_list(a1435))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f339(v0: Adt14) -> u64 {
    let m1436 = f423(f94(v0, f8(Str::lit("stratum"))));
    match m1436 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f340(v0: u64, p1441: List<u64>) -> bool {
    let m1440 = p1441;
    {
        let mut m1440 = m1440;
        loop {
            let step = {
                let v1 = m1440;
                let m1437 = v1;
                match m1437.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1438 = v2;
                        let a1439 = v0;
                        if nat_eq(a1438, a1439) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: Bytes, p1499: u64, p1500: u64, p1501: Adt23) -> R<Adt23> {
    let m1498 = (p1499, (p1500, p1501));
    {
        let mut m1498 = m1498;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1498;
                let m1442 = v2;
                if m1442 == 0 {
                    Err::<(u64, (u64, Adt23)), R<Adt23>>(Ok(v3))
                } else {
                    let v4 = m1442 - 1;
                    let v5: u64 = f424(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt23::C0(_, h1443, _, _, _) => {
                            h1443
                        }
                    } {
                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                            let a1444 = v1;
                            let a1445 = 1u64;
                            nat_add(a1444, a1445)?
                        }, (v4, Adt23::C0(true, false, match v3.clone() {
                            Adt23::C0(_, _, h1446, _, _) => {
                                h1446
                            }
                        }, match v3.clone() {
                            Adt23::C0(_, _, _, h1447, _) => {
                                h1447
                            }
                        }, match v3 {
                            Adt23::C0(_, _, _, _, h1448) => {
                                h1448
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt23::C0(h1449, _, _, _, _) => {
                                h1449
                            }
                        } {
                            let a1450 = v5;
                            let a1451 = 34u64;
                            if nat_eq(a1450, a1451) {
                                let a1454 = f424(v0.clone(), {
                                    let a1452 = v1;
                                    let a1453 = 1u64;
                                    nat_add(a1452, a1453)?
                                })?;
                                let a1455 = 34u64;
                                if nat_eq(a1454, a1455) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1456 = v1;
                                        let a1457 = 1u64;
                                        nat_add(a1456, a1457)?
                                    }, (v4, Adt23::C0(true, true, f425(match v3.clone() {
                                        Adt23::C0(_, _, h1458, _, _) => {
                                            h1458
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt23::C0(_, _, _, h1459, _) => {
                                            h1459
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1460) => {
                                            h1460
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1461 = v1;
                                        let a1462 = 1u64;
                                        nat_add(a1461, a1462)?
                                    }, (v4, Adt23::C0(false, false, match v3.clone() {
                                        Adt23::C0(_, _, h1463, _, _) => {
                                            h1463
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1464, _) => {
                                            h1464
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1465) => {
                                            h1465
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1466 = v1;
                                    let a1467 = 1u64;
                                    nat_add(a1466, a1467)?
                                }, (v4, Adt23::C0(true, false, f425(match v3.clone() {
                                    Adt23::C0(_, _, h1468, _, _) => {
                                        h1468
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt23::C0(_, _, _, h1469, _) => {
                                        h1469
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1470) => {
                                        h1470
                                    }
                                }))))
                            }
                        } else {
                            let a1471 = v5;
                            let a1472 = 34u64;
                            if nat_eq(a1471, a1472) {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1473 = v1;
                                    let a1474 = 1u64;
                                    nat_add(a1473, a1474)?
                                }, (v4, Adt23::C0(true, false, match v3.clone() {
                                    Adt23::C0(_, _, h1475, _, _) => {
                                        h1475
                                    }
                                }, match v3.clone() {
                                    Adt23::C0(_, _, _, h1476, _) => {
                                        h1476
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1477) => {
                                        h1477
                                    }
                                }))))
                            } else {
                                let a1478 = v5;
                                let a1479 = 44u64;
                                if nat_eq(a1478, a1479) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1480 = v1;
                                        let a1481 = 1u64;
                                        nat_add(a1480, a1481)?
                                    }, (v4, Adt23::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt23::C0(_, _, h1482, _, _) => {
                                            h1482
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1483, _) => {
                                            h1483
                                        }
                                    }), match v3 {
                                        Adt23::C0(_, _, _, _, h1484) => {
                                            h1484
                                        }
                                    }))))
                                } else {
                                    let a1485 = v5;
                                    let a1486 = 13u64;
                                    if nat_eq(a1485, a1486) {
                                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                            let a1487 = v1;
                                            let a1488 = 1u64;
                                            nat_add(a1487, a1488)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1489 = v5;
                                        let a1490 = 10u64;
                                        if nat_eq(a1489, a1490) {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1491 = v1;
                                                let a1492 = 1u64;
                                                nat_add(a1491, a1492)?
                                            }, (v4, f342(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1493 = v1;
                                                let a1494 = 1u64;
                                                nat_add(a1493, a1494)?
                                            }, (v4, Adt23::C0(false, false, f425(match v3.clone() {
                                                Adt23::C0(_, _, h1495, _, _) => {
                                                    h1495
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt23::C0(_, _, _, h1496, _) => {
                                                    h1496
                                                }
                                            }, match v3 {
                                                Adt23::C0(_, _, _, _, h1497) => {
                                                    h1497
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
                Ok(next) => { m1498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f342(v0: Adt23) -> Adt23 {
    Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f426(List::cons(match v0.clone() {
        Adt23::C0(_, _, h1502, _, _) => {
            h1502
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1503, _) => {
            h1503
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt23::C0(_, _, _, _, h1504) => {
            h1504
        }
    }))
}

pub fn f343(p1507: List<List<Bytes>>, p1508: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1506 = (p1507, p1508);
    {
        let mut m1506 = m1506;
        loop {
            let step = {
                let (v0, v1) = m1506;
                let m1505 = v0;
                match m1505.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f344(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f345(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1509 = v0.clone();
    match m1509.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1510 = v2;
            let a1511 = v1.clone();
            if equal(a1510, a1511) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f346(v0: List<(Bytes, ())>) -> List<Bytes> {
    f245(f427(Fn8::F428, List::<Bytes>::nil(), v0))
}

pub fn f347(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f429(f430(Fn9::F431, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1516 = {
        let (h1512, _) = f432(Fn10::F433, {
            let a1514 = {
                let a1513 = v0;
                length_list(a1513)
            };
            let a1515 = 1u64;
            nat_add(a1514, a1515)?
        }, v1)?;
        h1512
    };
    match m1516.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f348(v0: List<Bytes>) -> List<(Bytes, ())> {
    f434(f435(Fn11::F436, List::<(Bytes, ())>::nil(), v0))
}

pub fn f349(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1517 = f44(v1, 34u8)?;
    match m1517.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1518 = v3;
            match m1518.uncons() {
                None => {
                    Ok(f437(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f350(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f438(v0.clone(), 0u64, {
        let a1519 = v0;
        length_bytes(a1519)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f351(v0: Adt14, v1: Bytes) -> Bytes {
    if f32(f439(f94(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f352(v0: Bytes, v1: Bytes, p1532: u64, p1533: u64) -> R<bool> {
    let m1531 = (p1532, p1533);
    {
        let mut m1531 = m1531;
        loop {
            let step = {
                let (v2, v3) = m1531;
                let m1520 = v3;
                if m1520 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1520 - 1;
                    let m1523 = {
                        let a1521 = v1.clone();
                        let a1522 = v2;
                        index_bytes(a1521, a1522)
                    };
                    match m1523 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1526 = {
                                let a1524 = v0.clone();
                                let a1525 = v2;
                                index_bytes(a1524, a1525)
                            };
                            match m1526 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1527 = v5;
                                    let a1528 = v6;
                                    if equal(a1527, a1528) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1529 = v2;
                                            let a1530 = 1u64;
                                            nat_add(a1529, a1530)?
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
                Ok(next) => { m1531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1534 = f91(v0, v1)?;
    match m1534 {
        None => {
            Ok(Adt2::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f354(v0: Bytes) -> R<Bytes> {
    let a1535 = f8(Str::lit("Catalog."));
    let a1536 = f373(v0)?;
    Ok(append_bytes(a1535, a1536))
}

pub fn f355(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f373(f282(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f356(v0: List<Bytes>) -> List<Bytes> {
    let m1537 = v0;
    match m1537.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f248(f266(), v1), f356(v2))
            } else {
                f356(v2)
            }
        }
    }
}

pub fn f357(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1538 = v1;
    match m1538.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f440(v0.clone(), v2)?;
            let (h1539, _) = v4.clone();
            if h1539 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f357(v0, v3)?;
                let (h1540, _) = v5.clone();
                if h1540 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1543 = {
                            let (_, h1541) = v4;
                            h1541
                        };
                        let a1544 = {
                            let (_, h1542) = v5;
                            h1542
                        };
                        append_bytes(a1543, a1544)
                    }))
                }
            }
        }
    }
}

pub fn f358(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1545 = v1;
    match m1545.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f223(v2.clone(), f8(Str::lit("subject"))) {
                if f224(v0.clone(), v2.clone()) {
                    f358(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f358(v0, v3))
                }
            } else {
                if f364(v0.clone(), v2.clone()) {
                    f358(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f365(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f358(v0, v3))
                }
            }
        }
    }
}

pub fn f359(v0: Adt1) -> R<Bytes> {
    let a1547 = {
        let a1546 = f129(f130(f244(f258(f243(f64(v0.clone(), f217()))?)), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(f97(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1546)
    };
    let a1548 = 0u64;
    if nat_eq(a1547, a1548) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f360(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1549 = v0;
    match m1549.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f123(f150(f192(f94(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f441(f96(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f360(v2)?))
        }
    }
}

pub fn f361(v0: Bytes, v1: List<Adt19>) -> List<Bytes> {
    let m1550 = v1;
    match m1550.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt19::C0(h1551, _, _) => {
                    h1551
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt19::C0(_, h1552, _) => {
                        h1552
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt19::C0(_, _, h1553) => {
                        h1553
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f361(v0, v3))
            } else {
                f361(v0, v3)
            }
        }
    }
}

pub fn f362(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1554 = v1;
    match m1554.uncons() {
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

pub fn f363(p1559: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1558 = p1559;
    {
        let mut m1558 = m1558;
        loop {
            let step = {
                let v0 = m1558;
                let m1555 = v0;
                match m1555.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1556 = v2;
                        match m1556.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1557 = v5;
                                    match m1557.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f442(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364(v0: Adt15, v1: Adt14) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1560, _, _) => {
            h1560
        }
    }) {
        f52(f365(v1), match v0 {
            Adt15::C0(_, h1561, _) => {
                h1561
            }
        })
    } else {
        false
    }
}

pub fn f365(v0: Adt14) -> Bytes {
    let m1562 = f94(v0, f8(Str::lit("evidence")));
    match m1562 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f366(v0: Adt14) -> Bytes {
    if f223(v0.clone(), f8(Str::lit("req"))) {
        let a1563 = f8(Str::lit("req "));
        let a1564 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1563, a1564)
    } else {
        let m1565 = f94(v0.clone(), f8(Str::lit("std")));
        match m1565 {
            None => {
                let m1568 = f94(v0, f8(Str::lit("der")));
                match m1568 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f123(f150(f192(f94(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f223(v1.clone(), f8(Str::lit("row"))) {
                    let a1566 = f8(Str::lit(" row "));
                    let a1567 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1566, a1567)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f367(v0: Option<List<Adt14>>) -> bool {
    let m1569 = v0;
    match m1569 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f368(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1570 = v0;
    match m1570 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f369(v0: Bytes, v1: Bytes) -> bool {
    let a1571 = v0;
    let a1572 = f381();
    if equal(a1571, a1572) {
        let a1576 = {
            let a1574 = {
                let a1573 = v1;
                length_bytes(a1573)
            };
            let a1575 = 0u64;
            nat_eq(a1574, a1575)
        };
        bool_not(a1576)
    } else {
        false
    }
}

pub fn f370(v0: Bytes) -> Bytes {
    f7(List::cons(f152(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f371(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1578 = v0;
    let a1579 = {
        let a1577 = Str::lit("redistributable");
        utf8_encode(a1577)
    };
    if equal(a1578, a1579) {
        f443(v1)
    } else {
        Ok(false)
    }
}

pub fn f372(v0: Adt2) -> bool {
    if f444(v0.clone(), f8(Str::lit("name"))) {
        if f445(v0.clone(), f8(Str::lit("schema"))) {
            let m1580 = f376(f283(f284(f279(v0, f8(Str::lit("citation"))))));
            match m1580 {
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
    f446(v0.clone(), 0u64, {
        let a1581 = v0;
        length_bytes(a1581)
    }, true, Bytes::lit(&[]))
}

pub fn f374(v0: Adt14) -> Bytes {
    if f223(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f223(v0.clone(), f8(Str::lit("id"))) {
            if f223(v0.clone(), f8(Str::lit("statement"))) {
                if f223(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f389(v0, f8(Str::lit("evidence")))
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

pub fn f375(v0: List<Bytes>) -> Adt14 {
    let m1582 = v0;
    match m1582.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f375(v2)))
        }
    }
}

pub fn f376(v0: List<Bytes>) -> Option<Adt14> {
    let m1583 = v0;
    match m1583.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1584 = v2;
                match m1584.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1585 = v2;
                    match m1585.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1586 = v6;
                                match m1586.uncons() {
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
                        let m1587 = v2;
                        match m1587.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1588 = v10;
                                match m1588.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f375(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f377(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1589 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1589.uncons() {
        None => {
            let m1590 = f376(v4);
            match m1590 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1591 = f8(Str::lit("Catalog."));
                        let a1592 = f373(v3.clone())?;
                        append_bytes(a1591, a1592)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f373(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1593 = f447(v2, v9.clone(), f373(v3.clone())?)?;
                    match m1593 {
                        Ok(v12) => {
                            Ok(Adt12::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f373(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f448(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f448(f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1597 = f449(f64(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v10.clone(), 4u64, {
                                    let a1595 = {
                                        let a1594 = v10;
                                        length_bytes(a1594)
                                    };
                                    let a1596 = 4u64;
                                    nat_sub(a1595, a1596)
                                }))?;
                                let a1598 = f8(Str::lit("\u{a}"));
                                append_bytes(a1597, a1598)
                            }), List::cons(f278(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Err(v11) => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v11, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f378(v0: Adt2) -> R<Bytes> {
    let m1599 = v0;
    match m1599 {
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
            f301(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f450(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f451(v9)
        }
        Adt2::C6(r1600) => {
            let v10 = (*r1600).clone();
            Ok(f303(f452(v10)?))
        }
        Adt2::C7(r1601) => {
            let v11 = (*r1601).clone();
            f453(f454(v11)?)
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

pub fn f379(v0: Option<Adt2>) -> Option<Bytes> {
    let m1602 = v0;
    match m1602 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1603 = v1;
            match m1603 {
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

pub fn f380(v0: Adt2) -> List<Adt2> {
    let m1604 = v0;
    match m1604 {
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
        Adt2::C6(r1605) => {
            let v3 = (*r1605).clone();
            f380(v3)
        }
        Adt2::C7(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C8 => {
            List::<Adt2>::nil()
        }
        Adt2::C9(r1606, r1607) => {
            let v1 = (*r1606).clone();
            let v2 = (*r1607).clone();
            List::cons(v1, f380(v2))
        }
        Adt2::C10(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f381() -> Bytes {
    let a1608 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1608)
}

pub fn f382(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f94(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f290(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f290(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f290(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f290(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f290(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f290(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f290(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f290(Str::lit("url"), f439(v2.clone(), f8(Str::lit("url"))))?, List::cons(f290(Str::lit("media_type"), f439(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f290(Str::lit("license"), f439(v2.clone(), f8(Str::lit("license"))))?, List::cons(f290(Str::lit("redistribution"), f439(v2, f8(Str::lit("redistribution"))))?, List::cons(f290(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1609 = v1;
        match m1609 {
            None => {
                f7(List::cons(f290(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f290(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f290(Str::lit("artifact"), {
                    let (h1610, _) = v3;
                    h1610
                })?, List::cons(f290(Str::lit("checksum"), {
                    let (_, h1611) = v3;
                    h1611
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f455(f150(f192(f94(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f383(v0: Adt24) -> Bytes {
    let a1632 = f456(match v0.clone() {
        Adt24::C0(h1612, _, _, _, _, _, _, _) => {
            h1612
        }
    });
    let a1633 = {
        let a1630 = f456(match v0.clone() {
            Adt24::C0(_, h1613, _, _, _, _, _, _) => {
                h1613
            }
        });
        let a1631 = {
            let a1628 = f456(match v0.clone() {
                Adt24::C0(_, _, h1614, _, _, _, _, _) => {
                    h1614
                }
            });
            let a1629 = {
                let a1626 = f456(match v0.clone() {
                    Adt24::C0(_, _, _, h1615, _, _, _, _) => {
                        h1615
                    }
                });
                let a1627 = {
                    let a1624 = f456(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1616, _, _, _) => {
                            h1616
                        }
                    });
                    let a1625 = {
                        let a1622 = f456(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1617, _, _) => {
                                h1617
                            }
                        });
                        let a1623 = {
                            let a1620 = f456(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1618, _) => {
                                    h1618
                                }
                            });
                            let a1621 = f456(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1619) => {
                                    h1619
                                }
                            });
                            append_bytes(a1620, a1621)
                        };
                        append_bytes(a1622, a1623)
                    };
                    append_bytes(a1624, a1625)
                };
                append_bytes(a1626, a1627)
            };
            append_bytes(a1628, a1629)
        };
        append_bytes(a1630, a1631)
    };
    append_bytes(a1632, a1633)
}

pub fn f384(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1634 = v0.clone();
        length_bytes(a1634)
    };
    let v2: u64 = {
        let a1640 = {
            let a1637 = {
                let a1635 = v1;
                let a1636 = 72u64;
                nat_add(a1635, a1636)?
            };
            let a1638 = 64u64;
            let a1639 = 0u64;
            nat_quot(a1637, a1638, a1639)
        };
        let a1641 = 64u64;
        nat_mul(a1640, a1641)?
    };
    let (_, h1642) = f457(Fn12::F458(v0, v1, v2), {
        let a1643 = v2;
        let a1644 = 64u64;
        let a1645 = 0u64;
        nat_quot(a1643, a1644, a1645)
    }, (0u64, f459()))?;
    Ok(h1642)
}

pub fn f385(p1648: List<List<Bytes>>, v1: Bytes, p1649: List<Bytes>) -> R<List<Bytes>> {
    let m1647 = (p1648, p1649);
    {
        let mut m1647 = m1647;
        loop {
            let step = {
                let (v0, v2) = m1647;
                let m1646 = v0;
                match m1646.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f460(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f387(f461(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
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

pub fn f386(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f310(v0)?))
}

pub fn f387(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1650 = v0;
    match m1650.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1651 = v3.clone();
            match m1651.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1654 = v2;
                    let a1655 = {
                        let a1652 = v1.clone();
                        let a1653 = f387(v3, v1);
                        append_bytes(a1652, a1653)
                    };
                    append_bytes(a1654, a1655)
                }
            }
        }
    }
}

pub fn f388(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1656 = f94(v0, v1);
    match m1656 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f192(f94(v3, v2))
        }
    }
}

pub fn f389(v0: Adt14, v1: Bytes) -> bool {
    let m1657 = f94(v0, v1);
    match m1657 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f390(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1659 = {
                let a1658 = v0;
                length_bytes(a1658)
            };
            let a1660 = 1u64;
            nat_sub(a1659, a1660)
        })
    } else {
        v0
    }
}

pub fn f391(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1661 = v0;
    match m1661 {
        Adt14::C0(v3) => {
            Adt14::C0(v3)
        }
        Adt14::C1(v4, v5) => {
            Adt14::C1(v4, v5)
        }
        Adt14::C2(v2) => {
            Adt14::C2(f462(f60(), v2, v1))
        }
        Adt14::C3(r1662) => {
            let v6 = (*r1662).clone();
            Adt14::C3(std::rc::Rc::new(v6))
        }
        Adt14::C4(r1663) => {
            let v7 = (*r1663).clone();
            Adt14::C4(std::rc::Rc::new(v7))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1664, r1665) => {
            let v8 = (*r1664).clone();
            let v9 = (*r1665).clone();
            Adt14::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt14::C7(v10, r1666, r1667) => {
            let v11 = (*r1666).clone();
            let v12 = (*r1667).clone();
            Adt14::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f392(v0: u64) -> R<Bytes> {
    f463(f464(), 10u64, v0)
}

pub fn f393(v0: Fn5, p1672: u64, p1673: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1671 = (p1672, p1673);
    {
        let mut m1671 = m1671;
        loop {
            let step = {
                let (v1, v2) = m1671;
                let m1668 = v1;
                if m1668 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1668 - 1;
                    let m1670 = {
                        let c1669: Fn5 = v0.clone();
                        c1669.apply(v2.clone())?
                    };
                    match m1670 {
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
                Ok(next) => { m1671 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f465(v0, v1)
}

pub fn f395(v0: Fn6, p1677: List<(Bytes, Bytes)>, p1678: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1676 = (p1677, p1678);
    {
        let mut m1676 = m1676;
        loop {
            let step = {
                let (v1, v2) = m1676;
                let m1674 = v2;
                match m1674.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1675: Fn6 = v0.clone();
                            c1675.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1676 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f466(v0, v1)
}

pub fn f397(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f467(v0)
}

pub fn f398(v0: Fn7, p1682: List<Bytes>, p1683: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1681 = (p1682, p1683);
    {
        let mut m1681 = m1681;
        loop {
            let step = {
                let (v1, v2) = m1681;
                let m1679 = v2;
                match m1679.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1680: Fn7 = v0.clone();
                            c1680.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1681 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f468(v1)?, v0))
}

pub fn f400(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f302(v0)?, List::cons(f8(Str::lit(":")), List::cons(f302(v1)?, List::<Bytes>::nil())))))
}

pub fn f401(v0: Adt13) -> Bytes {
    let m1684 = v0;
    match m1684 {
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

pub fn f402(v0: Adt13) -> Bytes {
    let m1685 = v0;
    match m1685 {
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

pub fn f403(v0: Adt13) -> Bytes {
    let m1686 = v0;
    match m1686 {
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

pub fn f404(v0: Bytes, v1: u64) -> Option<u64> {
    let a1687 = 0u64;
    let a1688 = v1;
    if nat_lt(a1687, a1688) {
        if f469(v0, {
            let a1689 = v1;
            let a1690 = 1u64;
            nat_sub(a1689, a1690)
        }) {
            Some({
                let a1691 = v1;
                let a1692 = 1u64;
                nat_sub(a1691, a1692)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f405(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1693 = v2;
    let a1694 = v0;
    if nat_lt(a1693, a1694) {
        if f469(v1, v2) {
            Ok(Some({
                let a1695 = v2;
                let a1696 = 1u64;
                nat_add(a1695, a1696)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f406(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1699 = f46(v0.clone(), {
        let a1697 = v1;
        let a1698 = 2u64;
        nat_add(a1697, a1698)?
    }, 2u64);
    let a1700 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1699, a1700) {
        let m1703 = f470(f46(v0, {
            let a1701 = v1;
            let a1702 = 4u64;
            nat_add(a1701, a1702)?
        }, 2u64))?;
        match m1703 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f116(v2.clone(), 0u64)?;
                let a1704 = v3;
                let a1705 = 32u64;
                if nat_lt(a1704, a1705) {
                    if f471(v3) {
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

pub fn f407(v0: u64) -> Bytes {
    let a1706 = v0;
    let a1707 = 34u64;
    if nat_eq(a1706, a1707) {
        Bytes::lit(&[0x22])
    } else {
        let a1708 = v0;
        let a1709 = 92u64;
        if nat_eq(a1708, a1709) {
            Bytes::lit(&[0x5c])
        } else {
            let a1710 = v0;
            let a1711 = 98u64;
            if nat_eq(a1710, a1711) {
                Bytes::lit(&[0x08])
            } else {
                let a1712 = v0;
                let a1713 = 102u64;
                if nat_eq(a1712, a1713) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1714 = v0;
                    let a1715 = 110u64;
                    if nat_eq(a1714, a1715) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1716 = v0;
                        let a1717 = 114u64;
                        if nat_eq(a1716, a1717) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1718 = v0;
                            let a1719 = 116u64;
                            if nat_eq(a1718, a1719) {
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

pub fn f408(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f116(v0, v1)?;
    let a1720 = 47u64;
    let a1721 = v2;
    if if nat_lt(a1720, a1721) {
        let a1722 = v2;
        let a1723 = 58u64;
        nat_lt(a1722, a1723)
    } else {
        false
    } {
        Ok(Some({
            let a1724 = v1;
            let a1725 = 1u64;
            nat_add(a1724, a1725)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f409(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1726 = f116(v0, v1)?;
    let a1727 = 48u64;
    if nat_eq(a1726, a1727) {
        Ok(Some({
            let a1728 = v1;
            let a1729 = 1u64;
            nat_add(a1728, a1729)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f410(v0: Bytes) -> R<u64> {
    let (h1730, _) = f313(Fn3::F472(v0.clone()), {
        let a1731 = v0;
        length_bytes(a1731)
    }, 0u64)?;
    Ok(h1730)
}

pub fn f411(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1735 = {
        let a1733 = v0;
        let a1734 = {
            let (h1732, _) = v1;
            h1732
        };
        index_bytes(a1733, a1734)
    };
    match m1735 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1737 = {
                    let (h1736, _) = v1;
                    h1736
                };
                let a1738 = 1u64;
                nat_add(a1737, a1738)?
            }, {
                let a1742 = {
                    let a1740 = {
                        let (_, h1739) = v1;
                        h1739
                    };
                    let a1741 = 10u64;
                    nat_mul(a1740, a1741)?
                };
                let a1743 = f473(v2)?;
                nat_add(a1742, a1743)?
            })))
        }
    }
}

pub fn f412(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f474(v0, v1)
}

pub fn f413(v0: (Bytes, Bytes), p1746: List<(Bytes, Bytes)>) -> bool {
    let m1745 = p1746;
    {
        let mut m1745 = m1745;
        loop {
            let step = {
                let v1 = m1745;
                let m1744 = v1;
                match m1744.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f475(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1745 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1747 = v1;
    match m1747.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1748, _) = v2;
                h1748
            }, v0.clone()) {
                List::cons({
                    let (_, h1749) = v2;
                    h1749
                }, f414(v0, v3))
            } else {
                f414(v0, v3)
            }
        }
    }
}

pub fn f415() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f416(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1750 = f94(v0, v1);
    match m1750 {
        None => {
            false
        }
        Some(v3) => {
            f223(v3, v2)
        }
    }
}

pub fn f417() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f418(v0: Bytes) -> R<bool> {
    let m1751 = f231(v0.clone(), f8(Str::lit("&&")))?;
    if match m1751 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1752 = f231(v0, f8(Str::lit("||")))?;
        match m1752 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f419(v0: Adt18, v1: Option<u64>) -> bool {
    let m1753 = v1;
    match m1753 {
        None => {
            true
        }
        Some(v2) => {
            f340(v2, match v0 {
                Adt18::C0(_, h1754, _, _) => {
                    h1754
                }
            })
        }
    }
}

pub fn f420(v0: Adt16) -> Adt21 {
    Adt21::C1({
        let a1756 = match v0 {
            Adt16::C0(_, _, h1755, _, _) => {
                h1755
            }
        };
        length_list(a1756)
    })
}

pub fn f421(v0: Bytes) -> Option<Adt22> {
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

pub fn f422(v0: Bytes, p1761: List<Bytes>, p1762: u64) -> R<Option<u64>> {
    let m1760 = (p1761, p1762);
    {
        let mut m1760 = m1760;
        loop {
            let step = {
                let (v1, v2) = m1760;
                let m1757 = v1;
                match m1757.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1758 = v2;
                                let a1759 = 1u64;
                                nat_add(a1758, a1759)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1760 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(v0: Option<Adt14>) -> Option<u64> {
    let m1763 = v0;
    match m1763 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1764 = v1;
            match m1764 {
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

pub fn f424(v0: Bytes, v1: u64) -> R<u64> {
    let m1767 = {
        let a1765 = v0;
        let a1766 = v1;
        index_bytes(a1765, a1766)
    };
    match m1767 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f425(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1771 = {
        let a1768 = v1;
        let a1769 = v2;
        let a1770 = 1u64;
        slice_bytes(a1768, a1769, a1770)
    };
    match m1771 {
        None => {
            v0
        }
        Some(v3) => {
            let a1772 = v0;
            let a1773 = v3;
            append_bytes(a1772, a1773)
        }
    }
}

pub fn f426(p1776: List<Bytes>, p1777: List<Bytes>) -> List<Bytes> {
    let m1775 = (p1776, p1777);
    {
        let mut m1775 = m1775;
        loop {
            let step = {
                let (v0, v1) = m1775;
                let m1774 = v0;
                match m1774.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(v0: Fn8, p1781: List<Bytes>, p1782: List<(Bytes, ())>) -> List<Bytes> {
    let m1780 = (p1781, p1782);
    {
        let mut m1780 = m1780;
        loop {
            let step = {
                let (v1, v2) = m1780;
                let m1778 = v2;
                match m1778.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1779: Fn8 = v0.clone();
                            c1779.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f428(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1783, _) = v1;
        h1783
    }, v0)
}

pub fn f429(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f476(Fn13::F477, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f430(v0: Fn9, p1787: List<List<(Bytes, ())>>, p1788: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1786 = (p1787, p1788);
    {
        let mut m1786 = m1786;
        loop {
            let step = {
                let (v1, v2) = m1786;
                let m1784 = v2;
                match m1784.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1785: Fn9 = v0.clone();
                            c1785.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1786 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f431(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f432(v0: Fn10, p1793: u64, p1794: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1792 = (p1793, p1794);
    {
        let mut m1792 = m1792;
        loop {
            let step = {
                let (v1, v2) = m1792;
                let m1789 = v1;
                if m1789 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1789 - 1;
                    let m1791 = {
                        let c1790: Fn10 = v0.clone();
                        c1790.apply(v2.clone())?
                    };
                    match m1791 {
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
                Ok(next) => { m1792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f478(v0)
}

pub fn f434(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f479(Fn14::F480, List::<(Bytes, ())>::nil(), v0)
}

pub fn f435(v0: Fn11, p1798: List<(Bytes, ())>, p1799: List<Bytes>) -> List<(Bytes, ())> {
    let m1797 = (p1798, p1799);
    {
        let mut m1797 = m1797;
        loop {
            let step = {
                let (v1, v2) = m1797;
                let m1795 = v2;
                match m1795.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1796: Fn11 = v0.clone();
                            c1796.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1797 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f437(p1804: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1803 = p1804;
    {
        let mut m1803 = m1803;
        loop {
            let step = {
                let v0 = m1803;
                let m1800 = v0;
                match m1800.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1801, _) = v2;
                            h1801
                        }) {
                            let (_, h1802) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1802)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1803 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: Bytes, p1830: u64, p1831: u64, p1832: Bytes) -> R<Bytes> {
    let m1829 = (p1830, (p1831, p1832));
    {
        let mut m1829 = m1829;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1829;
                let m1805 = v2;
                if m1805 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1805 - 1;
                    let m1808 = {
                        let a1806 = v0.clone();
                        let a1807 = v1;
                        index_bytes(a1806, a1807)
                    };
                    match m1808 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f207({
                                let a1809 = v6;
                                let a1810 = 34u64;
                                nat_eq(a1809, a1810)
                            }, {
                                let a1811 = v6;
                                let a1812 = 92u64;
                                nat_eq(a1811, a1812)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1813 = v1;
                                    let a1814 = 1u64;
                                    nat_add(a1813, a1814)?
                                }, (v4, {
                                    let a1817 = v3;
                                    let a1818 = {
                                        let a1815 = f8(Str::lit("\\"));
                                        let a1816 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1815, a1816)
                                    };
                                    append_bytes(a1817, a1818)
                                })))
                            } else {
                                let a1819 = v6;
                                let a1820 = 10u64;
                                if nat_eq(a1819, a1820) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1821 = v1;
                                        let a1822 = 1u64;
                                        nat_add(a1821, a1822)?
                                    }, (v4, {
                                        let a1823 = v3;
                                        let a1824 = f8(Str::lit("\\n"));
                                        append_bytes(a1823, a1824)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1825 = v1;
                                        let a1826 = 1u64;
                                        nat_add(a1825, a1826)?
                                    }, (v4, {
                                        let a1827 = v3;
                                        let a1828 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1827, a1828)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1833 = v0;
    match m1833 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f440(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
    let m1834 = f293(v2.clone())?;
    match m1834 {
        None => {
            f482(v0, v1, v2)
        }
        Some(v3) => {
            f481(v0, v1, v3)
        }
    }
}

pub fn f441(v0: List<Bytes>) -> List<Bytes> {
    let m1835 = v0;
    match m1835.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1837 = {
                    let a1836 = v1.clone();
                    length_bytes(a1836)
                };
                let a1838 = 0u64;
                if nat_eq(a1837, a1838) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f441(v2))
        }
    }
}

pub fn f442(v0: List<Bytes>) -> Bytes {
    let m1839 = v0;
    match m1839.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f443(v0: Bytes) -> R<bool> {
    let m1841 = {
        let a1840 = v0.clone();
        length_bytes(a1840)
    };
    if m1841 == 0 {
        Ok(true)
    } else {
        let v1 = m1841 - 1;
        let m1844 = {
            let a1842 = v0;
            let a1843 = v1;
            index_bytes(a1842, a1843)
        };
        match m1844 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f483({
                    let v3: u8 = v2;
                    let a1889 = {
                        let a1865 = {
                            let a1853 = {
                                let a1847 = {
                                    let a1845 = v3;
                                    let a1846 = 128u8;
                                    fixed_u8::bit_and(a1845, a1846)
                                };
                                let a1848 = 128u8;
                                if equal(a1847, a1848) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1854 = {
                                let a1851 = {
                                    let a1849 = v3;
                                    let a1850 = 64u8;
                                    fixed_u8::bit_and(a1849, a1850)
                                };
                                let a1852 = 64u8;
                                if equal(a1851, a1852) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1853, a1854)?
                        };
                        let a1866 = {
                            let a1863 = {
                                let a1857 = {
                                    let a1855 = v3;
                                    let a1856 = 32u8;
                                    fixed_u8::bit_and(a1855, a1856)
                                };
                                let a1858 = 32u8;
                                if equal(a1857, a1858) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1864 = {
                                let a1861 = {
                                    let a1859 = v3;
                                    let a1860 = 16u8;
                                    fixed_u8::bit_and(a1859, a1860)
                                };
                                let a1862 = 16u8;
                                if equal(a1861, a1862) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1863, a1864)?
                        };
                        nat_add(a1865, a1866)?
                    };
                    let a1890 = {
                        let a1887 = {
                            let a1875 = {
                                let a1869 = {
                                    let a1867 = v3;
                                    let a1868 = 8u8;
                                    fixed_u8::bit_and(a1867, a1868)
                                };
                                let a1870 = 8u8;
                                if equal(a1869, a1870) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1876 = {
                                let a1873 = {
                                    let a1871 = v3;
                                    let a1872 = 4u8;
                                    fixed_u8::bit_and(a1871, a1872)
                                };
                                let a1874 = 4u8;
                                if equal(a1873, a1874) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1875, a1876)?
                        };
                        let a1888 = {
                            let a1885 = {
                                let a1879 = {
                                    let a1877 = v3;
                                    let a1878 = 2u8;
                                    fixed_u8::bit_and(a1877, a1878)
                                };
                                let a1880 = 2u8;
                                if equal(a1879, a1880) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1886 = {
                                let a1883 = {
                                    let a1881 = v3;
                                    let a1882 = 1u8;
                                    fixed_u8::bit_and(a1881, a1882)
                                };
                                let a1884 = 1u8;
                                if equal(a1883, a1884) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1885, a1886)?
                        };
                        nat_add(a1887, a1888)?
                    };
                    nat_add(a1889, a1890)?
                }))
            }
        }
    }
}

pub fn f444(v0: Adt2, v1: Bytes) -> bool {
    let a1892 = {
        let a1891 = f282(v0, v1);
        length_bytes(a1891)
    };
    let a1893 = 0u64;
    !nat_eq(a1892, a1893)
}

pub fn f445(v0: Adt2, v1: Bytes) -> bool {
    let m1894 = f279(v0, v1);
    match m1894 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f446(v0: Bytes, p1915: u64, p1916: u64, p1917: bool, p1918: Bytes) -> R<Bytes> {
    let m1914 = (p1915, (p1916, (p1917, p1918)));
    {
        let mut m1914 = m1914;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1914;
                let m1895 = v2;
                if m1895 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1895 - 1;
                    let m1898 = {
                        let a1896 = v0.clone();
                        let a1897 = v1;
                        index_bytes(a1896, a1897)
                    };
                    match m1898 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f484(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1899 = v1;
                                    let a1900 = 1u64;
                                    nat_add(a1899, a1900)?
                                }, (v5, (true, v4))))
                            } else {
                                if f118(f485(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1901 = v1;
                                        let a1902 = 1u64;
                                        nat_add(a1901, a1902)?
                                    }, (v5, (false, {
                                        let a1906 = v4;
                                        let a1907 = f46({
                                            let a1903 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1903)
                                        }, {
                                            let a1904 = v7;
                                            let a1905 = 97u64;
                                            nat_sub(a1904, a1905)
                                        }, 1u64);
                                        append_bytes(a1906, a1907)
                                    }))))
                                } else {
                                    if f207(f485(v7), f207(f486(v7), f487(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1908 = v1;
                                            let a1909 = 1u64;
                                            nat_add(a1908, a1909)?
                                        }, (v5, (false, {
                                            let a1910 = v4;
                                            let a1911 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1910, a1911)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1912 = v1;
                                            let a1913 = 1u64;
                                            nat_add(a1912, a1913)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1914 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f488(f489(f279(v0.clone(), f8(Str::lit("$defs")))), f489(f279(v0.clone(), f8(Str::lit("definitions")))));
    let m1919 = f490(4096u64, v3.clone(), List::<Bytes>::nil(), f491(v3)?)?;
    match m1919 {
        Ok(v5) => {
            let m1920 = f492(4096u64, f493(f494(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1920 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f387(f251(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f448(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1921 = v0;
    match m1921.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f495(v5, v1, v2))
            } else {
                List::cons(v4, f448(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f449(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f387(f496(f96(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f450(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f392(v1)?;
    let v4: u64 = {
        let a1922 = v3.clone();
        length_bytes(a1922)
    };
    let v5: Bytes = {
        let a1923 = v2;
        let a1924 = v4;
        if nat_lt(a1923, a1924) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1925 = v4;
                let a1926 = v2;
                nat_sub(a1925, a1926)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1927 = v4;
                let a1928 = v2;
                nat_sub(a1927, a1928)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f320({
                let a1929 = v2;
                let a1930 = v4;
                nat_sub(a1929, a1930)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1931 = f8(Str::lit("-"));
        let a1932 = v5;
        Ok(append_bytes(a1931, a1932))
    } else {
        Ok(v5)
    }
}

pub fn f451(v0: Bytes) -> R<Bytes> {
    let a1937 = f204(v0.clone(), 0u64, {
        let a1934 = {
            let a1933 = v0.clone();
            length_bytes(a1933)
        };
        let a1935 = 1u64;
        nat_add(a1934, a1935)?
    })?;
    let a1938 = {
        let a1936 = v0.clone();
        length_bytes(a1936)
    };
    if nat_eq(a1937, a1938) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1944 = {
            let a1942 = Bytes::lit(&[0x22]);
            let a1943 = f497(v0.clone(), 0u64, {
                let a1940 = {
                    let a1939 = v0;
                    length_bytes(a1939)
                };
                let a1941 = 1u64;
                nat_add(a1940, a1941)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1942, a1943)
        };
        let a1945 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1944, a1945))
    }
}

pub fn f452(v0: Adt2) -> R<List<Bytes>> {
    let m1946 = v0;
    match m1946 {
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
        Adt2::C9(r1947, r1948) => {
            let v12 = (*r1947).clone();
            let v13 = (*r1948).clone();
            Ok(List::cons(f378(v12)?, f452(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f453(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f395(Fn6::F498, List::<(Bytes, Bytes)>::nil(), f397(v0)?);
    let v2: List<Bytes> = f398(Fn7::F499, List::<Bytes>::nil(), v1)?;
    let a1951 = {
        let a1949 = Bytes::lit(&[0x7b]);
        let a1950 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1949, a1950)
    };
    let a1952 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1951, a1952))
}

pub fn f454(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1953 = v0;
    match m1953 {
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
        Adt2::C10(v14, r1954, r1955) => {
            let v15 = (*r1954).clone();
            let v16 = (*r1955).clone();
            Ok(List::cons((v14, f378(v15)?), f454(v16)?))
        }
    }
}

pub fn f455(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f387(f500(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f456(v0: u32) -> Bytes {
    let a1984 = f501({
        let a1956 = f502(v0, 28u32);
        let a1957 = 15u32;
        fixed_u32::bit_and(a1956, a1957)
    });
    let a1985 = {
        let a1982 = f501({
            let a1958 = f502(v0, 24u32);
            let a1959 = 15u32;
            fixed_u32::bit_and(a1958, a1959)
        });
        let a1983 = {
            let a1980 = f501({
                let a1960 = f502(v0, 20u32);
                let a1961 = 15u32;
                fixed_u32::bit_and(a1960, a1961)
            });
            let a1981 = {
                let a1978 = f501({
                    let a1962 = f502(v0, 16u32);
                    let a1963 = 15u32;
                    fixed_u32::bit_and(a1962, a1963)
                });
                let a1979 = {
                    let a1976 = f501({
                        let a1964 = f502(v0, 12u32);
                        let a1965 = 15u32;
                        fixed_u32::bit_and(a1964, a1965)
                    });
                    let a1977 = {
                        let a1974 = f501({
                            let a1966 = f502(v0, 8u32);
                            let a1967 = 15u32;
                            fixed_u32::bit_and(a1966, a1967)
                        });
                        let a1975 = {
                            let a1972 = f501({
                                let a1968 = f502(v0, 4u32);
                                let a1969 = 15u32;
                                fixed_u32::bit_and(a1968, a1969)
                            });
                            let a1973 = f501({
                                let a1970 = v0;
                                let a1971 = 15u32;
                                fixed_u32::bit_and(a1970, a1971)
                            });
                            append_bytes(a1972, a1973)
                        };
                        append_bytes(a1974, a1975)
                    };
                    append_bytes(a1976, a1977)
                };
                append_bytes(a1978, a1979)
            };
            append_bytes(a1980, a1981)
        };
        append_bytes(a1982, a1983)
    };
    append_bytes(a1984, a1985)
}

pub fn f457(v0: Fn12, p1989: u64, p1990: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1988 = (p1989, p1990);
    {
        let mut m1988 = m1988;
        loop {
            let step = {
                let (v1, v2) = m1988;
                let m1986 = v1;
                if m1986 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1986 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1987: Fn12 = v0.clone();
                        c1987.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1992 = {
            let (h1991, _) = v3.clone();
            h1991
        };
        let a1993 = 64u64;
        nat_add(a1992, a1993)?
    }, f503({
        let (_, h1994) = v3;
        h1994
    }, f504(v0, v1, v2, {
        let (h1995, _) = v3;
        h1995
    })?)?))
}

pub fn f459() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f460(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1996 = f44(v1, 34u8)?;
    match m1996.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1997 = v3;
            match m1997.uncons() {
                None => {
                    Ok(f505(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f461(v0: List<Bytes>) -> List<Bytes> {
    let m1998 = v0;
    match m1998.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f461(v2)
            } else {
                List::cons(v1, f461(v2))
            }
        }
    }
}

pub fn f462(p2001: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2000 = p2001;
    {
        let mut m2000 = m2000;
        loop {
            let step = {
                let v0 = m2000;
                let m1999 = v0;
                match m1999.uncons() {
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
                Ok(next) => { m2000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2002 = v2;
    let a2003 = 0u64;
    if nat_eq(a2002, a2003) {
        Ok(f506(v0, 0u64))
    } else {
        let (_, h2004) = {
            let (h2005, _) = f393(Fn5::F507(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2005
        };
        Ok(h2004)
    }
}

pub fn f464() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f465(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2009 = {
        let a2007 = v0.clone();
        let a2008 = {
            let (h2006, _) = v1.clone();
            h2006
        };
        index_bytes(a2007, a2008)
    };
    match m2009 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2011 = {
                    let (h2010, _) = v1.clone();
                    h2010
                };
                let a2012 = 1u64;
                nat_add(a2011, a2012)?
            }, {
                let a2015 = {
                    let (_, h2013) = v1;
                    h2013
                };
                let a2016 = f508(v0, {
                    let (h2014, _) = v1;
                    h2014
                }, v2)?;
                append_bytes(a2015, a2016)
            })))
        }
    }
}

pub fn f466(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2017 = v0.clone();
    match m2017.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2020 = {
                let (h2018, _) = v2;
                h2018
            };
            let a2021 = {
                let (h2019, _) = v1.clone();
                h2019
            };
            if equal(a2020, a2021) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f467(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f509(f510(Fn15::F511, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2026 = {
        let (h2022, _) = f512(Fn16::F513, {
            let a2024 = {
                let a2023 = v0;
                length_list(a2023)
            };
            let a2025 = 1u64;
            nat_add(a2024, a2025)?
        }, v1)?;
        h2022
    };
    match m2026.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f468(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2031 = {
        let a2028 = f302({
            let (h2027, _) = v0;
            h2027
        })?;
        let a2029 = Bytes::lit(&[0x3a]);
        append_bytes(a2028, a2029)
    };
    let a2032 = {
        let (_, h2030) = v0;
        h2030
    };
    Ok(append_bytes(a2031, a2032))
}

pub fn f469(v0: Bytes, v1: u64) -> bool {
    let m2033 = f212(v0, v1);
    match m2033 {
        None => {
            false
        }
        Some(v2) => {
            f514(v2)
        }
    }
}

pub fn f470(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2034 = v0.clone();
        length_bytes(a2034)
    };
    let a2038 = {
        let a2035 = v1;
        let a2036 = 2u64;
        let a2037 = 0u64;
        nat_rem(a2035, a2036, a2037)
    };
    let a2039 = 0u64;
    if nat_eq(a2038, a2039) {
        let (_, h2040) = {
            let (h2041, _) = f515(Fn17::F516(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2041
        };
        Ok(h2040)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f471(v0: u64) -> bool {
    let a2042 = v0;
    let a2043 = 8u64;
    if nat_eq(a2042, a2043) {
        true
    } else {
        let a2044 = v0;
        let a2045 = 9u64;
        if nat_eq(a2044, a2045) {
            true
        } else {
            let a2046 = v0;
            let a2047 = 10u64;
            if nat_eq(a2046, a2047) {
                true
            } else {
                let a2048 = v0;
                let a2049 = 12u64;
                if nat_eq(a2048, a2049) {
                    true
                } else {
                    let a2050 = v0;
                    let a2051 = 13u64;
                    nat_eq(a2050, a2051)
                }
            }
        }
    }
}

pub fn f472(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2054 = {
        let a2052 = v0;
        let a2053 = v1;
        index_bytes(a2052, a2053)
    };
    match m2054 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f517(v2)? {
                Ok(Some({
                    let a2055 = v1;
                    let a2056 = 1u64;
                    nat_add(a2055, a2056)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f473(v0: u8) -> R<u64> {
    let a2103 = {
        let v1: u8 = v0;
        let a2101 = {
            let a2077 = {
                let a2065 = {
                    let a2059 = {
                        let a2057 = v1;
                        let a2058 = 128u8;
                        fixed_u8::bit_and(a2057, a2058)
                    };
                    let a2060 = 128u8;
                    if equal(a2059, a2060) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2066 = {
                    let a2063 = {
                        let a2061 = v1;
                        let a2062 = 64u8;
                        fixed_u8::bit_and(a2061, a2062)
                    };
                    let a2064 = 64u8;
                    if equal(a2063, a2064) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2065, a2066)?
            };
            let a2078 = {
                let a2075 = {
                    let a2069 = {
                        let a2067 = v1;
                        let a2068 = 32u8;
                        fixed_u8::bit_and(a2067, a2068)
                    };
                    let a2070 = 32u8;
                    if equal(a2069, a2070) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2076 = {
                    let a2073 = {
                        let a2071 = v1;
                        let a2072 = 16u8;
                        fixed_u8::bit_and(a2071, a2072)
                    };
                    let a2074 = 16u8;
                    if equal(a2073, a2074) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2075, a2076)?
            };
            nat_add(a2077, a2078)?
        };
        let a2102 = {
            let a2099 = {
                let a2087 = {
                    let a2081 = {
                        let a2079 = v1;
                        let a2080 = 8u8;
                        fixed_u8::bit_and(a2079, a2080)
                    };
                    let a2082 = 8u8;
                    if equal(a2081, a2082) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2088 = {
                    let a2085 = {
                        let a2083 = v1;
                        let a2084 = 4u8;
                        fixed_u8::bit_and(a2083, a2084)
                    };
                    let a2086 = 4u8;
                    if equal(a2085, a2086) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2087, a2088)?
            };
            let a2100 = {
                let a2097 = {
                    let a2091 = {
                        let a2089 = v1;
                        let a2090 = 2u8;
                        fixed_u8::bit_and(a2089, a2090)
                    };
                    let a2092 = 2u8;
                    if equal(a2091, a2092) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2098 = {
                    let a2095 = {
                        let a2093 = v1;
                        let a2094 = 1u8;
                        fixed_u8::bit_and(a2093, a2094)
                    };
                    let a2096 = 1u8;
                    if equal(a2095, a2096) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2097, a2098)?
            };
            nat_add(a2099, a2100)?
        };
        nat_add(a2101, a2102)?
    };
    let a2104 = 48u64;
    Ok(nat_sub(a2103, a2104))
}

pub fn f474(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2105 = v0.clone();
        length_bytes(a2105)
    };
    let a2106 = v1;
    let a2107 = v2;
    if if nat_lt(a2106, a2107) {
        let a2112 = f116(v0, {
            let a2110 = v2;
            let a2111 = {
                let a2108 = v1;
                let a2109 = 1u64;
                nat_add(a2108, a2109)?
            };
            nat_sub(a2110, a2111)
        })?;
        let a2113 = 48u64;
        nat_eq(a2112, a2113)
    } else {
        false
    } {
        Ok(Some({
            let a2114 = v1;
            let a2115 = 1u64;
            nat_add(a2114, a2115)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f475(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f118(f32({
        let (h2116, _) = v0;
        h2116
    }, {
        let (h2117, _) = v1;
        h2117
    }), f32({
        let (_, h2118) = v0;
        h2118
    }, {
        let (_, h2119) = v1;
        h2119
    }))
}

pub fn f476(v0: Fn13, p2123: List<List<(Bytes, ())>>, p2124: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2122 = (p2123, p2124);
    {
        let mut m2122 = m2122;
        loop {
            let step = {
                let (v1, v2) = m2122;
                let m2120 = v2;
                match m2120.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2121: Fn13 = v0.clone();
                            c2121.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2122 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f478(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2126 = {
        let a2125 = v0.clone();
        length_list(a2125)
    };
    let a2127 = 1u64;
    if nat_le(a2126, a2127) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f518(v0)?))
    }
}

pub fn f479(v0: Fn14, p2131: List<(Bytes, ())>, p2132: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2130 = (p2131, p2132);
    {
        let mut m2130 = m2130;
        loop {
            let step = {
                let (v1, v2) = m2130;
                let m2128 = v2;
                match m2128.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2129: Fn14 = v0.clone();
                            c2129.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f481(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f149(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f149(f97(v0, f161())?, f8(Str::lit("at"))));
    let a2134 = {
        let a2133 = v4.clone();
        length_list(a2133)
    };
    let a2135 = 0u64;
    if nat_eq(a2134, a2135) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2138 = {
            let a2136 = v4;
            length_list(a2136)
        };
        let a2139 = {
            let a2137 = v3;
            length_list(a2137)
        };
        if nat_eq(a2138, a2139) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f519(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f482(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f368(f274(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f368(f274(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f126(f149(v3.clone(), f8(Str::lit("id"))), f149(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f149(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f126(v6.clone(), f149(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2141 = {
        let a2140 = v7.clone();
        length_list(a2140)
    };
    let a2142 = 0u64;
    if nat_eq(a2141, a2142) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2145 = {
                let a2143 = v7;
                length_list(a2143)
            };
            let a2146 = {
                let a2144 = v5;
                length_list(a2144)
            };
            nat_eq(a2145, a2146)
        }) {
            Ok((false, f519(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f153(f126(f62(f520(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f521(v4), f126(v6, f149(v3, f8(Str::lit("id")))))))?;
            let a2148 = {
                let a2147 = v8.clone();
                length_list(a2147)
            };
            let a2149 = 0u64;
            if nat_eq(a2148, a2149) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f519(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f483(v0: u64) -> bool {
    let a2150 = v0;
    let a2151 = 47u64;
    nat_eq(a2150, a2151)
}

pub fn f484(v0: u64) -> bool {
    f207({
        let a2152 = v0;
        let a2153 = 95u64;
        nat_eq(a2152, a2153)
    }, f207({
        let a2154 = v0;
        let a2155 = 45u64;
        nat_eq(a2154, a2155)
    }, f207({
        let a2156 = v0;
        let a2157 = 32u64;
        nat_eq(a2156, a2157)
    }, f207({
        let a2158 = v0;
        let a2159 = 46u64;
        nat_eq(a2158, a2159)
    }, {
        let a2160 = v0;
        let a2161 = 47u64;
        nat_eq(a2160, a2161)
    }))))
}

pub fn f485(v0: u64) -> bool {
    f118({
        let a2162 = 97u64;
        let a2163 = v0;
        nat_le(a2162, a2163)
    }, {
        let a2164 = v0;
        let a2165 = 122u64;
        nat_le(a2164, a2165)
    })
}

pub fn f486(v0: u64) -> bool {
    f118({
        let a2166 = 65u64;
        let a2167 = v0;
        nat_le(a2166, a2167)
    }, {
        let a2168 = v0;
        let a2169 = 90u64;
        nat_le(a2168, a2169)
    })
}

pub fn f487(v0: u64) -> bool {
    f118({
        let a2170 = 48u64;
        let a2171 = v0;
        nat_le(a2170, a2171)
    }, {
        let a2172 = v0;
        let a2173 = 57u64;
        nat_le(a2172, a2173)
    })
}

pub fn f488(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2174 = v0;
    match m2174.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f488(v3, v1))
        }
    }
}

pub fn f489(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2175 = v0;
    match m2175 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f522(v1)
        }
    }
}

pub fn f490(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2176 = v0;
    if m2176 == 0 {
        let m2177 = v1;
        match m2177.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2176 - 1;
        let m2178 = v1.clone();
        match m2178.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2179 = f523(v1.clone(), v2.clone(), v3.clone())?;
                match m2179 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f387(f491(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2182 = f490(v6, f524(v1, {
                            let (h2180, _) = v9.clone();
                            h2180
                        }), List::cons(f525({
                            let (h2181, _) = v9.clone();
                            h2181
                        })?, v2), v3)?;
                        match m2182 {
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

pub fn f491(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2183 = v0;
    match m2183.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f525({
                let (h2184, _) = v1;
                h2184
            })?, f491(v2)?))
        }
    }
}

pub fn f492(p2190: u64, p2191: List<Adt25>, p2192: List<Bytes>, p2193: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2189 = (p2190, (p2191, (p2192, p2193)));
    {
        let mut m2189 = m2189;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2189;
                let m2185 = v0;
                if m2185 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2185 - 1;
                    let m2186 = v1;
                    match m2186.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2187 = v5;
                            match m2187 {
                                Adt25::C0(v8, v9) => {
                                    if f32(v8.clone(), f526()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2188 = f528(v8.clone(), v9)?;
                                            match m2188 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f493(f529(v11.clone()), List::cons(Adt25::C1(f530(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f52(f526(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f526(), v2), List::cons(f527(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2189 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2194 = v0;
    match m2194.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f493(v3, v1))
        }
    }
}

pub fn f494(v0: List<(Bytes, Adt2)>) -> R<List<Adt25>> {
    let m2195 = v0;
    match m2195.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f525({
                let (h2196, _) = v1;
                h2196
            })?, {
                let (_, h2197) = v1;
                h2197
            }), f494(v2)?))
        }
    }
}

pub fn f495(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2198 = v0;
    match m2198.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f495(v4, v1, v2)
            } else {
                List::cons(v3, f495(v4, v1, v2))
            }
        }
    }
}

pub fn f496(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2199 = v0;
    match m2199.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f387(f500(f153(List::cons(v1, f531(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f496(v3, v1)?))
            }
        }
    }
}

pub fn f497(v0: Bytes, p2218: u64, p2219: u64, p2220: Bytes) -> R<Bytes> {
    let m2217 = (p2218, (p2219, p2220));
    {
        let mut m2217 = m2217;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2217;
                let m2200 = v2;
                if m2200 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2200 - 1;
                    let v5: u64 = f204(v0.clone(), v1, {
                        let a2204 = {
                            let a2202 = {
                                let a2201 = v0.clone();
                                length_bytes(a2201)
                            };
                            let a2203 = v1;
                            nat_sub(a2202, a2203)
                        };
                        let a2205 = 1u64;
                        nat_add(a2204, a2205)?
                    })?;
                    let v6: Bytes = {
                        let a2208 = v3;
                        let a2209 = f46(v0.clone(), v1, {
                            let a2206 = v5;
                            let a2207 = v1;
                            nat_sub(a2206, a2207)
                        });
                        append_bytes(a2208, a2209)
                    };
                    let m2212 = {
                        let a2210 = v0.clone();
                        let a2211 = v5;
                        index_bytes(a2210, a2211)
                    };
                    match m2212 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2213 = v5;
                                let a2214 = 1u64;
                                nat_add(a2213, a2214)?
                            }, (v4, {
                                let a2215 = v6;
                                let a2216 = f508(v0.clone(), v5, v7)?;
                                append_bytes(a2215, a2216)
                            })))
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

pub fn f498(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f466(v0, v1)
}

pub fn f499(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f532(v1)?, v0))
}

pub fn f500(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2221 = v0;
    match m2221.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f350(v1)?, f500(v2)?))
        }
    }
}

pub fn f501(v0: u32) -> Bytes {
    let a2224 = {
        let a2222 = v0;
        let a2223 = 8u32;
        fixed_u32::bit_and(a2222, a2223)
    };
    let a2225 = 0u32;
    if equal(a2224, a2225) {
        let a2228 = {
            let a2226 = v0;
            let a2227 = 4u32;
            fixed_u32::bit_and(a2226, a2227)
        };
        let a2229 = 0u32;
        if equal(a2228, a2229) {
            let a2232 = {
                let a2230 = v0;
                let a2231 = 2u32;
                fixed_u32::bit_and(a2230, a2231)
            };
            let a2233 = 0u32;
            if equal(a2232, a2233) {
                let a2236 = {
                    let a2234 = v0;
                    let a2235 = 1u32;
                    fixed_u32::bit_and(a2234, a2235)
                };
                let a2237 = 0u32;
                if equal(a2236, a2237) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2240 = {
                    let a2238 = v0;
                    let a2239 = 1u32;
                    fixed_u32::bit_and(a2238, a2239)
                };
                let a2241 = 0u32;
                if equal(a2240, a2241) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2244 = {
                let a2242 = v0;
                let a2243 = 2u32;
                fixed_u32::bit_and(a2242, a2243)
            };
            let a2245 = 0u32;
            if equal(a2244, a2245) {
                let a2248 = {
                    let a2246 = v0;
                    let a2247 = 1u32;
                    fixed_u32::bit_and(a2246, a2247)
                };
                let a2249 = 0u32;
                if equal(a2248, a2249) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2252 = {
                    let a2250 = v0;
                    let a2251 = 1u32;
                    fixed_u32::bit_and(a2250, a2251)
                };
                let a2253 = 0u32;
                if equal(a2252, a2253) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2256 = {
            let a2254 = v0;
            let a2255 = 4u32;
            fixed_u32::bit_and(a2254, a2255)
        };
        let a2257 = 0u32;
        if equal(a2256, a2257) {
            let a2260 = {
                let a2258 = v0;
                let a2259 = 2u32;
                fixed_u32::bit_and(a2258, a2259)
            };
            let a2261 = 0u32;
            if equal(a2260, a2261) {
                let a2264 = {
                    let a2262 = v0;
                    let a2263 = 1u32;
                    fixed_u32::bit_and(a2262, a2263)
                };
                let a2265 = 0u32;
                if equal(a2264, a2265) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2268 = {
                    let a2266 = v0;
                    let a2267 = 1u32;
                    fixed_u32::bit_and(a2266, a2267)
                };
                let a2269 = 0u32;
                if equal(a2268, a2269) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2272 = {
                let a2270 = v0;
                let a2271 = 2u32;
                fixed_u32::bit_and(a2270, a2271)
            };
            let a2273 = 0u32;
            if equal(a2272, a2273) {
                let a2276 = {
                    let a2274 = v0;
                    let a2275 = 1u32;
                    fixed_u32::bit_and(a2274, a2275)
                };
                let a2277 = 0u32;
                if equal(a2276, a2277) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2280 = {
                    let a2278 = v0;
                    let a2279 = 1u32;
                    fixed_u32::bit_and(a2278, a2279)
                };
                let a2281 = 0u32;
                if equal(a2280, a2281) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f502(v0: u32, v1: u32) -> u32 {
    let m2284 = {
        let a2282 = v0;
        let a2283 = v1;
        fixed_u32::shift_right(a2282, a2283)
    };
    match m2284 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f503(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f533(Fn18::F534, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2285, _, _, _, _, _, _, _) => {
            h2285
        }
    }, match v0.clone() {
        Adt24::C0(_, h2286, _, _, _, _, _, _) => {
            h2286
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2287, _, _, _, _, _) => {
            h2287
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2288, _, _, _, _) => {
            h2288
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2289, _, _, _) => {
            h2289
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2290, _, _) => {
            h2290
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2291, _) => {
            h2291
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2292) => {
            h2292
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f535(match v0.clone() {
        Adt24::C0(h2293, _, _, _, _, _, _, _) => {
            h2293
        }
    }, match v2.clone() {
        Adt27::C0(h2294, _, _, _, _, _, _, _, _, _) => {
            h2294
        }
    }), f535(match v0.clone() {
        Adt24::C0(_, h2295, _, _, _, _, _, _) => {
            h2295
        }
    }, match v2.clone() {
        Adt27::C0(_, h2296, _, _, _, _, _, _, _, _) => {
            h2296
        }
    }), f535(match v0.clone() {
        Adt24::C0(_, _, h2297, _, _, _, _, _) => {
            h2297
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2298, _, _, _, _, _, _, _) => {
            h2298
        }
    }), f535(match v0.clone() {
        Adt24::C0(_, _, _, h2299, _, _, _, _) => {
            h2299
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2300, _, _, _, _, _, _) => {
            h2300
        }
    }), f535(match v0.clone() {
        Adt24::C0(_, _, _, _, h2301, _, _, _) => {
            h2301
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2302, _, _, _, _, _) => {
            h2302
        }
    }), f535(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2303, _, _) => {
            h2303
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2304, _, _, _, _) => {
            h2304
        }
    }), f535(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2305, _) => {
            h2305
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2306, _, _, _) => {
            h2306
        }
    }), f535(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2307) => {
            h2307
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2308, _, _) => {
            h2308
        }
    })))
}

pub fn f504(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f536(v0.clone(), v1, v2, v3)?, f536(v0.clone(), v1, v2, {
        let a2309 = v3;
        let a2310 = 4u64;
        nat_add(a2309, a2310)?
    })?, f536(v0.clone(), v1, v2, {
        let a2311 = v3;
        let a2312 = 8u64;
        nat_add(a2311, a2312)?
    })?, f536(v0.clone(), v1, v2, {
        let a2313 = v3;
        let a2314 = 12u64;
        nat_add(a2313, a2314)?
    })?, f536(v0.clone(), v1, v2, {
        let a2315 = v3;
        let a2316 = 16u64;
        nat_add(a2315, a2316)?
    })?, f536(v0.clone(), v1, v2, {
        let a2317 = v3;
        let a2318 = 20u64;
        nat_add(a2317, a2318)?
    })?, f536(v0.clone(), v1, v2, {
        let a2319 = v3;
        let a2320 = 24u64;
        nat_add(a2319, a2320)?
    })?, f536(v0.clone(), v1, v2, {
        let a2321 = v3;
        let a2322 = 28u64;
        nat_add(a2321, a2322)?
    })?, f536(v0.clone(), v1, v2, {
        let a2323 = v3;
        let a2324 = 32u64;
        nat_add(a2323, a2324)?
    })?, f536(v0.clone(), v1, v2, {
        let a2325 = v3;
        let a2326 = 36u64;
        nat_add(a2325, a2326)?
    })?, f536(v0.clone(), v1, v2, {
        let a2327 = v3;
        let a2328 = 40u64;
        nat_add(a2327, a2328)?
    })?, f536(v0.clone(), v1, v2, {
        let a2329 = v3;
        let a2330 = 44u64;
        nat_add(a2329, a2330)?
    })?, f536(v0.clone(), v1, v2, {
        let a2331 = v3;
        let a2332 = 48u64;
        nat_add(a2331, a2332)?
    })?, f536(v0.clone(), v1, v2, {
        let a2333 = v3;
        let a2334 = 52u64;
        nat_add(a2333, a2334)?
    })?, f536(v0.clone(), v1, v2, {
        let a2335 = v3;
        let a2336 = 56u64;
        nat_add(a2335, a2336)?
    })?, f536(v0, v1, v2, {
        let a2337 = v3;
        let a2338 = 60u64;
        nat_add(a2337, a2338)?
    })?))
}

pub fn f505(p2341: List<Bytes>, v1: Bytes) -> bool {
    let m2340 = p2341;
    {
        let mut m2340 = m2340;
        loop {
            let step = {
                let v0 = m2340;
                let m2339 = v0;
                match m2339.uncons() {
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
                Ok(next) => { m2340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: Bytes, v1: u64) -> Bytes {
    let m2345 = {
        let a2342 = v0;
        let a2343 = v1;
        let a2344 = 1u64;
        slice_bytes(a2342, a2343, a2344)
    };
    match m2345 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f507(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f537(v1, v0, v2)
}

pub fn f508(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2390 = {
            let a2366 = {
                let a2354 = {
                    let a2348 = {
                        let a2346 = v3;
                        let a2347 = 128u8;
                        fixed_u8::bit_and(a2346, a2347)
                    };
                    let a2349 = 128u8;
                    if equal(a2348, a2349) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2355 = {
                    let a2352 = {
                        let a2350 = v3;
                        let a2351 = 64u8;
                        fixed_u8::bit_and(a2350, a2351)
                    };
                    let a2353 = 64u8;
                    if equal(a2352, a2353) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2354, a2355)?
            };
            let a2367 = {
                let a2364 = {
                    let a2358 = {
                        let a2356 = v3;
                        let a2357 = 32u8;
                        fixed_u8::bit_and(a2356, a2357)
                    };
                    let a2359 = 32u8;
                    if equal(a2358, a2359) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2365 = {
                    let a2362 = {
                        let a2360 = v3;
                        let a2361 = 16u8;
                        fixed_u8::bit_and(a2360, a2361)
                    };
                    let a2363 = 16u8;
                    if equal(a2362, a2363) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2364, a2365)?
            };
            nat_add(a2366, a2367)?
        };
        let a2391 = {
            let a2388 = {
                let a2376 = {
                    let a2370 = {
                        let a2368 = v3;
                        let a2369 = 8u8;
                        fixed_u8::bit_and(a2368, a2369)
                    };
                    let a2371 = 8u8;
                    if equal(a2370, a2371) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2377 = {
                    let a2374 = {
                        let a2372 = v3;
                        let a2373 = 4u8;
                        fixed_u8::bit_and(a2372, a2373)
                    };
                    let a2375 = 4u8;
                    if equal(a2374, a2375) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2376, a2377)?
            };
            let a2389 = {
                let a2386 = {
                    let a2380 = {
                        let a2378 = v3;
                        let a2379 = 2u8;
                        fixed_u8::bit_and(a2378, a2379)
                    };
                    let a2381 = 2u8;
                    if equal(a2380, a2381) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2387 = {
                    let a2384 = {
                        let a2382 = v3;
                        let a2383 = 1u8;
                        fixed_u8::bit_and(a2382, a2383)
                    };
                    let a2385 = 1u8;
                    if equal(a2384, a2385) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2386, a2387)?
            };
            nat_add(a2388, a2389)?
        };
        nat_add(a2390, a2391)?
    };
    let a2392 = v4;
    let a2393 = 34u64;
    if nat_eq(a2392, a2393) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2394 = v4;
        let a2395 = 92u64;
        if nat_eq(a2394, a2395) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2396 = v4;
            let a2397 = 8u64;
            if nat_eq(a2396, a2397) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2398 = v4;
                let a2399 = 12u64;
                if nat_eq(a2398, a2399) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2400 = v4;
                    let a2401 = 10u64;
                    if nat_eq(a2400, a2401) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2402 = v4;
                        let a2403 = 13u64;
                        if nat_eq(a2402, a2403) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2404 = v4;
                            let a2405 = 9u64;
                            if nat_eq(a2404, a2405) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2406 = v4;
                                let a2407 = 32u64;
                                if nat_lt(a2406, a2407) {
                                    let a2408 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2409 = f538(v2)?;
                                    Ok(append_bytes(a2408, a2409))
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

pub fn f509(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f539(Fn19::F540, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f510(v0: Fn15, p2413: List<List<(Bytes, Bytes)>>, p2414: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2412 = (p2413, p2414);
    {
        let mut m2412 = m2412;
        loop {
            let step = {
                let (v1, v2) = m2412;
                let m2410 = v2;
                match m2410.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2411: Fn15 = v0.clone();
                            c2411.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2412 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f512(v0: Fn16, p2419: u64, p2420: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2418 = (p2419, p2420);
    {
        let mut m2418 = m2418;
        loop {
            let step = {
                let (v1, v2) = m2418;
                let m2415 = v1;
                if m2415 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2415 - 1;
                    let m2417 = {
                        let c2416: Fn16 = v0.clone();
                        c2416.apply(v2.clone())?
                    };
                    match m2417 {
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
                Ok(next) => { m2418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f541(v0)
}

pub fn f514(v0: u8) -> bool {
    let a2421 = v0;
    let a2422 = 32u8;
    if if if equal(a2421, a2422) {
        true
    } else {
        let a2423 = v0;
        let a2424 = 9u8;
        equal(a2423, a2424)
    } {
        true
    } else {
        let a2425 = v0;
        let a2426 = 10u8;
        equal(a2425, a2426)
    } {
        true
    } else {
        let a2427 = v0;
        let a2428 = 12u8;
        if equal(a2427, a2428) {
            true
        } else {
            let a2429 = v0;
            let a2430 = 13u8;
            equal(a2429, a2430)
        }
    }
}

pub fn f515(v0: Fn17, p2435: u64, p2436: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2434 = (p2435, p2436);
    {
        let mut m2434 = m2434;
        loop {
            let step = {
                let (v1, v2) = m2434;
                let m2431 = v1;
                if m2431 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2431 - 1;
                    let m2433 = {
                        let c2432: Fn17 = v0.clone();
                        c2432.apply(v2.clone())?
                    };
                    match m2433 {
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
                Ok(next) => { m2434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f542(v0, v1)
}

pub fn f517(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2481 = {
            let a2457 = {
                let a2445 = {
                    let a2439 = {
                        let a2437 = v1;
                        let a2438 = 128u8;
                        fixed_u8::bit_and(a2437, a2438)
                    };
                    let a2440 = 128u8;
                    if equal(a2439, a2440) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2446 = {
                    let a2443 = {
                        let a2441 = v1;
                        let a2442 = 64u8;
                        fixed_u8::bit_and(a2441, a2442)
                    };
                    let a2444 = 64u8;
                    if equal(a2443, a2444) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2445, a2446)?
            };
            let a2458 = {
                let a2455 = {
                    let a2449 = {
                        let a2447 = v1;
                        let a2448 = 32u8;
                        fixed_u8::bit_and(a2447, a2448)
                    };
                    let a2450 = 32u8;
                    if equal(a2449, a2450) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2456 = {
                    let a2453 = {
                        let a2451 = v1;
                        let a2452 = 16u8;
                        fixed_u8::bit_and(a2451, a2452)
                    };
                    let a2454 = 16u8;
                    if equal(a2453, a2454) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2455, a2456)?
            };
            nat_add(a2457, a2458)?
        };
        let a2482 = {
            let a2479 = {
                let a2467 = {
                    let a2461 = {
                        let a2459 = v1;
                        let a2460 = 8u8;
                        fixed_u8::bit_and(a2459, a2460)
                    };
                    let a2462 = 8u8;
                    if equal(a2461, a2462) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2468 = {
                    let a2465 = {
                        let a2463 = v1;
                        let a2464 = 4u8;
                        fixed_u8::bit_and(a2463, a2464)
                    };
                    let a2466 = 4u8;
                    if equal(a2465, a2466) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2467, a2468)?
            };
            let a2480 = {
                let a2477 = {
                    let a2471 = {
                        let a2469 = v1;
                        let a2470 = 2u8;
                        fixed_u8::bit_and(a2469, a2470)
                    };
                    let a2472 = 2u8;
                    if equal(a2471, a2472) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2478 = {
                    let a2475 = {
                        let a2473 = v1;
                        let a2474 = 1u8;
                        fixed_u8::bit_and(a2473, a2474)
                    };
                    let a2476 = 1u8;
                    if equal(a2475, a2476) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2477, a2478)?
            };
            nat_add(a2479, a2480)?
        };
        nat_add(a2481, a2482)?
    };
    let a2483 = 48u64;
    let a2484 = v2;
    if nat_le(a2483, a2484) {
        let a2485 = v2;
        let a2486 = 57u64;
        Ok(nat_le(a2485, a2486))
    } else {
        Ok(false)
    }
}

pub fn f518(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f429({
        let (_, h2487) = {
            let (h2488, _) = f543(Fn20::F544, {
                let a2490 = {
                    let a2489 = v0.clone();
                    length_list(a2489)
                };
                let a2491 = 1u64;
                nat_add(a2490, a2491)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2488
        };
        h2487
    }))
}

pub fn f519(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f520(v0: List<Adt14>) -> List<Bytes> {
    let m2492 = v0;
    match m2492.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2493 = f94(v1, f8(Str::lit("std")));
            match m2493 {
                None => {
                    f520(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f520(v2))
                }
            }
        }
    }
}

pub fn f521(v0: List<Adt14>) -> List<Bytes> {
    let m2494 = v0;
    match m2494.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2495 = f93(f94(v1, f8(Str::lit("req"))));
            match m2495 {
                None => {
                    f521(v2)
                }
                Some(v3) => {
                    List::cons(v3, f521(v2))
                }
            }
        }
    }
}

pub fn f522(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2496 = v0;
    match m2496 {
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
        Adt2::C7(r2497) => {
            let v4 = (*r2497).clone();
            f522(v4)
        }
        Adt2::C8 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C9(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(v1, r2498, r2499) => {
            let v2 = (*r2498).clone();
            let v3 = (*r2499).clone();
            List::cons((v1, v2), f522(v3))
        }
    }
}

pub fn f523(p2504: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2503 = p2504;
    {
        let mut m2503 = m2503;
        loop {
            let step = {
                let v0 = m2503;
                let m2500 = v0;
                match m2500.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f545(f546({
                            let (_, h2501) = v3.clone();
                            h2501
                        }, List::<Bytes>::nil())?, f525({
                            let (h2502, _) = v3.clone();
                            h2502
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2505 = v0;
    match m2505.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2506, _) = v2.clone();
                h2506
            }, v1.clone()) {
                f524(v3, v1)
            } else {
                List::cons(v2, f524(v3, v1))
            }
        }
    }
}

pub fn f525(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f373(v0)?;
    if f52(v1.clone(), f547()) {
        f548(v1)
    } else {
        Ok(v1)
    }
}

pub fn f526() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f527() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f528(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    f549(v0, f550(v1))
}

pub fn f529(v0: Adt28) -> List<Adt25> {
    let m2507 = v0;
    match m2507 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f530(v0: Adt28) -> Bytes {
    let m2508 = v0;
    match m2508 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f531(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2509 = v0;
    match m2509.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f531(v3, false))
            } else {
                f531(v3, true)
            }
        }
    }
}

pub fn f532(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2514 = {
        let a2511 = f451({
            let (h2510, _) = v0;
            h2510
        })?;
        let a2512 = Bytes::lit(&[0x3a]);
        append_bytes(a2511, a2512)
    };
    let a2515 = {
        let (_, h2513) = v0;
        h2513
    };
    Ok(append_bytes(a2514, a2515))
}

pub fn f533(v0: Fn18, p2519: u64, p2520: Adt27) -> R<Adt27> {
    let m2518 = (p2519, p2520);
    {
        let mut m2518 = m2518;
        loop {
            let step = {
                let (v1, v2) = m2518;
                let m2516 = v1;
                if m2516 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2516 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2517: Fn18 = v0.clone();
                        c2517.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: Adt27) -> R<Adt27> {
    f551(v0)
}

pub fn f535(v0: u32, v1: u32) -> u32 {
    let m2523 = {
        let a2521 = f552(v0);
        let a2522 = f552(v1);
        fixed_u64::checked_add(a2521, a2522)
    };
    match m2523 {
        None => {
            0u32
        }
        Some(v2) => {
            f553(v2)
        }
    }
}

pub fn f536(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2526 = {
        let a2524 = v3;
        let a2525 = 8u64;
        nat_add(a2524, a2525)?
    };
    let a2527 = v2;
    if nat_eq(a2526, a2527) {
        Ok(f554({
            let a2530 = {
                let a2528 = v1;
                let a2529 = 8u64;
                nat_mul(a2528, a2529)?
            };
            let a2531 = 4294967296u64;
            let a2532 = 0u64;
            nat_quot(a2530, a2531, a2532)
        }))
    } else {
        let a2535 = {
            let a2533 = v3;
            let a2534 = 4u64;
            nat_add(a2533, a2534)?
        };
        let a2536 = v2;
        if nat_eq(a2535, a2536) {
            Ok(f554({
                let a2539 = {
                    let a2537 = v1;
                    let a2538 = 8u64;
                    nat_mul(a2537, a2538)?
                };
                let a2540 = 4294967296u64;
                let a2541 = 0u64;
                nat_rem(a2539, a2540, a2541)
            }))
        } else {
            Ok(f555(f556(v0.clone(), v1, v3), f556(v0.clone(), v1, {
                let a2542 = v3;
                let a2543 = 1u64;
                nat_add(a2542, a2543)?
            }), f556(v0.clone(), v1, {
                let a2544 = v3;
                let a2545 = 2u64;
                nat_add(a2544, a2545)?
            }), f556(v0, v1, {
                let a2546 = v3;
                let a2547 = 3u64;
                nat_add(a2546, a2547)?
            })))
        }
    }
}

pub fn f537(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2548, _) = v2;
        h2548
    };
    let a2549 = v3;
    let a2550 = 0u64;
    if nat_eq(a2549, a2550) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2551 = v3;
            let a2552 = v1;
            let a2553 = 0u64;
            nat_quot(a2551, a2552, a2553)
        }, {
            let a2558 = f506(v0, {
                let a2554 = v3;
                let a2555 = v1;
                let a2556 = 0u64;
                nat_rem(a2554, a2555, a2556)
            });
            let a2559 = {
                let (_, h2557) = v2;
                h2557
            };
            append_bytes(a2558, a2559)
        }))
    }
}

pub fn f538(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2604 = {
            let a2580 = {
                let a2568 = {
                    let a2562 = {
                        let a2560 = v1;
                        let a2561 = 128u8;
                        fixed_u8::bit_and(a2560, a2561)
                    };
                    let a2563 = 128u8;
                    if equal(a2562, a2563) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2569 = {
                    let a2566 = {
                        let a2564 = v1;
                        let a2565 = 64u8;
                        fixed_u8::bit_and(a2564, a2565)
                    };
                    let a2567 = 64u8;
                    if equal(a2566, a2567) {
                        64u64
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
                        let a2571 = 32u8;
                        fixed_u8::bit_and(a2570, a2571)
                    };
                    let a2573 = 32u8;
                    if equal(a2572, a2573) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2579 = {
                    let a2576 = {
                        let a2574 = v1;
                        let a2575 = 16u8;
                        fixed_u8::bit_and(a2574, a2575)
                    };
                    let a2577 = 16u8;
                    if equal(a2576, a2577) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2578, a2579)?
            };
            nat_add(a2580, a2581)?
        };
        let a2605 = {
            let a2602 = {
                let a2590 = {
                    let a2584 = {
                        let a2582 = v1;
                        let a2583 = 8u8;
                        fixed_u8::bit_and(a2582, a2583)
                    };
                    let a2585 = 8u8;
                    if equal(a2584, a2585) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2591 = {
                    let a2588 = {
                        let a2586 = v1;
                        let a2587 = 4u8;
                        fixed_u8::bit_and(a2586, a2587)
                    };
                    let a2589 = 4u8;
                    if equal(a2588, a2589) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2590, a2591)?
            };
            let a2603 = {
                let a2600 = {
                    let a2594 = {
                        let a2592 = v1;
                        let a2593 = 2u8;
                        fixed_u8::bit_and(a2592, a2593)
                    };
                    let a2595 = 2u8;
                    if equal(a2594, a2595) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2601 = {
                    let a2598 = {
                        let a2596 = v1;
                        let a2597 = 1u8;
                        fixed_u8::bit_and(a2596, a2597)
                    };
                    let a2599 = 1u8;
                    if equal(a2598, a2599) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2600, a2601)?
            };
            nat_add(a2602, a2603)?
        };
        nat_add(a2604, a2605)?
    };
    let a2612 = f506(f557(), {
        let a2606 = v2;
        let a2607 = 16u64;
        let a2608 = 0u64;
        nat_quot(a2606, a2607, a2608)
    });
    let a2613 = f506(f557(), {
        let a2609 = v2;
        let a2610 = 16u64;
        let a2611 = 0u64;
        nat_rem(a2609, a2610, a2611)
    });
    Ok(append_bytes(a2612, a2613))
}

pub fn f539(v0: Fn19, p2617: List<List<(Bytes, Bytes)>>, p2618: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2616 = (p2617, p2618);
    {
        let mut m2616 = m2616;
        loop {
            let step = {
                let (v1, v2) = m2616;
                let m2614 = v2;
                match m2614.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2615: Fn19 = v0.clone();
                            c2615.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f541(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2620 = {
        let a2619 = v0.clone();
        length_list(a2619)
    };
    let a2621 = 1u64;
    if nat_le(a2620, a2621) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f558(v0)?))
    }
}

pub fn f542(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2622, _) = v1;
        h2622
    };
    let m2624 = {
        let (_, h2623) = v1;
        h2623
    };
    match m2624 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2627 = {
                let a2625 = v0.clone();
                let a2626 = v2;
                index_bytes(a2625, a2626)
            };
            match m2627 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2632 = {
                        let a2630 = v0;
                        let a2631 = {
                            let a2628 = v2;
                            let a2629 = 1u64;
                            nat_add(a2628, a2629)?
                        };
                        index_bytes(a2630, a2631)
                    };
                    match m2632 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f559(v4)? {
                                f559(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2633 = v2;
                                    let a2634 = 2u64;
                                    nat_add(a2633, a2634)?
                                }, Some({
                                    let a2639 = v3;
                                    let a2640 = f506(f560(), {
                                        let a2637 = {
                                            let a2635 = f561(v4)?;
                                            let a2636 = 16u64;
                                            nat_mul(a2635, a2636)?
                                        };
                                        let a2638 = f561(v5)?;
                                        nat_add(a2637, a2638)?
                                    });
                                    append_bytes(a2639, a2640)
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

pub fn f543(v0: Fn20, p2645: u64, p2646: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2644 = (p2645, p2646);
    {
        let mut m2644 = m2644;
        loop {
            let step = {
                let (v1, v2) = m2644;
                let m2641 = v1;
                if m2641 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2641 - 1;
                    let m2643 = {
                        let c2642: Fn20 = v0.clone();
                        c2642.apply(v2.clone())?
                    };
                    match m2643 {
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
                Ok(next) => { m2644 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f562(v0)
}

pub fn f545(p2649: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2648 = p2649;
    {
        let mut m2648 = m2648;
        loop {
            let step = {
                let v0 = m2648;
                let m2647 = v0;
                match m2647.uncons() {
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
                Ok(next) => { m2648 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2650 = v0;
    match m2650 {
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
        Adt2::C6(r2653) => {
            let v2 = (*r2653).clone();
            f546(v2, v1)
        }
        Adt2::C7(r2654) => {
            let v3 = (*r2654).clone();
            f546(v3, v1)
        }
        Adt2::C8 => {
            Ok(v1)
        }
        Adt2::C9(r2655, r2656) => {
            let v4 = (*r2655).clone();
            let v5 = (*r2656).clone();
            f546(v4, f546(v5, v1)?)
        }
        Adt2::C10(v6, r2657, r2658) => {
            let v7 = (*r2657).clone();
            let v8 = (*r2658).clone();
            let v9: List<Bytes> = f546(v7.clone(), f546(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2651 = f379(Some(v7));
                match m2651 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2652 = f563(v10)?;
                        match m2652 {
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

pub fn f547() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f548(v0: Bytes) -> R<Bytes> {
    let m2661 = {
        let a2659 = v0.clone();
        let a2660 = 0u64;
        index_bytes(a2659, a2660)
    };
    match m2661 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2667 = {
                let a2662 = f8(Str::lit("u"));
                let a2663 = f564(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2662, a2663)
            };
            let a2668 = f46(v0.clone(), 1u64, {
                let a2665 = {
                    let a2664 = v0;
                    length_bytes(a2664)
                };
                let a2666 = 1u64;
                nat_sub(a2665, a2666)
            });
            Ok(append_bytes(a2667, a2668))
        }
    }
}

pub fn f549(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    let m2669 = f279(v1.clone(), f8(Str::lit("enum")));
    match m2669 {
        None => {
            let m2671 = f279(v1.clone(), f8(Str::lit("oneOf")));
            match m2671 {
                None => {
                    if f569(v1.clone(), v0.clone())? {
                        let m2672 = f570(v0.clone(), f489(f279(v1.clone(), f8(Str::lit("properties")))), f283(f284(f279(v1, f8(Str::lit("required"))))), true)?;
                        match m2672 {
                            Ok(v7) => {
                                Ok(f567(f571(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f387(f572(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f573(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2673 = f489(f279(v1.clone(), f8(Str::lit("properties"))));
                        match m2673.uncons() {
                            None => {
                                if f574(v1.clone()) {
                                    Ok(f567(f575(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2676 = f576(v1, {
                                        let a2674 = v0.clone();
                                        let a2675 = f8(Str::lit("Value"));
                                        append_bytes(a2674, a2675)
                                    })?;
                                    match m2676 {
                                        Ok(v9) => {
                                            Ok(f567(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f530(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f529(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2677 = f570(v0.clone(), f489(f279(v1.clone(), f8(Str::lit("properties")))), f283(f284(f279(v1, f8(Str::lit("required"))))), false)?;
                                match m2677 {
                                    Ok(v13) => {
                                        Ok(f567(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2678 = f572(v13.clone());
                                            match m2678.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f387(f572(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f573(v13)))
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
                    f568(v0, f380(v5))
                }
            }
        }
        Some(v2) => {
            let m2670 = f565(f380(v2), List::<Bytes>::nil())?;
            match m2670 {
                Ok(v4) => {
                    Ok(f567(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f387(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f566(v3, v0))
                }
            }
        }
    }
}

pub fn f550(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2679 = f279(v0.clone(), f8(Str::lit("anyOf")));
        match m2679 {
            None => {
                v0
            }
            Some(v1) => {
                if f577(f380(v1.clone())) {
                    f578(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f579(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f580(f380(v1.clone()))
                    } {
                        v0
                    } else {
                        f581(f578(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2680 = f279(v2.clone(), f8(Str::lit("oneOf")));
        match m2680 {
            None => {
                v2
            }
            Some(v3) => {
                if f577(f380(v3)) {
                    f578(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2681 = f279(v4.clone(), f8(Str::lit("allOf")));
    match m2681 {
        None => {
            v4
        }
        Some(v5) => {
            if f582(f380(v5)) {
                f578(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f551(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f583(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2682) => {
            h2682
        }
    });
    let v2: u32 = f535(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2683, _, _) => {
            h2683
        }
    }, f535(f584(match v0.clone() {
        Adt27::C0(_, _, _, _, h2684, _, _, _, _, _) => {
            h2684
        }
    }), f535(f585(match v0.clone() {
        Adt27::C0(_, _, _, _, h2685, _, _, _, _, _) => {
            h2685
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2686, _, _, _, _) => {
            h2686
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2687, _, _, _) => {
            h2687
        }
    }), f535(v1, {
        let m2690 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2689, _) => {
                h2689
            }
        };
        match m2690 {
            Adt26::C0(h2688, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2688
            }
        }
    }))));
    let v3: u32 = f535(f586(match v0.clone() {
        Adt27::C0(h2691, _, _, _, _, _, _, _, _, _) => {
            h2691
        }
    }), f587(match v0.clone() {
        Adt27::C0(h2692, _, _, _, _, _, _, _, _, _) => {
            h2692
        }
    }, match v0.clone() {
        Adt27::C0(_, h2693, _, _, _, _, _, _, _, _) => {
            h2693
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2694, _, _, _, _, _, _, _) => {
            h2694
        }
    }));
    Ok(Adt27::C0(f535(v2, v3), match v0.clone() {
        Adt27::C0(h2695, _, _, _, _, _, _, _, _, _) => {
            h2695
        }
    }, match v0.clone() {
        Adt27::C0(_, h2696, _, _, _, _, _, _, _, _) => {
            h2696
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2697, _, _, _, _, _, _, _) => {
            h2697
        }
    }, f535(match v0.clone() {
        Adt27::C0(_, _, _, h2698, _, _, _, _, _, _) => {
            h2698
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2699, _, _, _, _, _) => {
            h2699
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2700, _, _, _, _) => {
            h2700
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2701, _, _, _) => {
            h2701
        }
    }, f588(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2702, _) => {
            h2702
        }
    }), {
        let a2704 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2703) => {
                h2703
            }
        };
        let a2705 = 1u64;
        nat_add(a2704, a2705)?
    }))
}

pub fn f552(v0: u32) -> u64 {
    let m2707 = {
        let a2706 = v0;
        fixed_u64::convert(i128::from(a2706))
    };
    match m2707 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f553(v0: u64) -> u32 {
    let m2711 = {
        let a2710 = {
            let a2708 = v0;
            let a2709 = 4294967295u64;
            fixed_u64::bit_and(a2708, a2709)
        };
        fixed_u32::convert(i128::from(a2710))
    };
    match m2711 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f554(v0: u64) -> u32 {
    let (_, h2712) = {
        let (_, h2713) = f589(Fn21::F590, 32u64, (v0, (1u32, 0u32)));
        h2713
    };
    h2712
}

pub fn f555(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2718 = f591(v0, 24u32);
    let a2719 = {
        let a2716 = f591(v1, 16u32);
        let a2717 = {
            let a2714 = f591(v2, 8u32);
            let a2715 = v3;
            fixed_u32::bit_or(a2714, a2715)
        };
        fixed_u32::bit_or(a2716, a2717)
    };
    fixed_u32::bit_or(a2718, a2719)
}

pub fn f556(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2720 = v2;
    let a2721 = v1;
    if nat_lt(a2720, a2721) {
        let m2724 = {
            let a2722 = v0;
            let a2723 = v2;
            index_bytes(a2722, a2723)
        };
        match m2724 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2726 = {
                    let a2725 = v3;
                    fixed_u32::convert(i128::from(a2725))
                };
                match m2726 {
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
        let a2727 = v2;
        let a2728 = v1;
        if nat_eq(a2727, a2728) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f557() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f558(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f509({
        let (_, h2729) = {
            let (h2730, _) = f592(Fn22::F593, {
                let a2732 = {
                    let a2731 = v0.clone();
                    length_list(a2731)
                };
                let a2733 = 1u64;
                nat_add(a2732, a2733)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2730
        };
        h2729
    }))
}

pub fn f559(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2778 = {
            let a2754 = {
                let a2742 = {
                    let a2736 = {
                        let a2734 = v1;
                        let a2735 = 128u8;
                        fixed_u8::bit_and(a2734, a2735)
                    };
                    let a2737 = 128u8;
                    if equal(a2736, a2737) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2743 = {
                    let a2740 = {
                        let a2738 = v1;
                        let a2739 = 64u8;
                        fixed_u8::bit_and(a2738, a2739)
                    };
                    let a2741 = 64u8;
                    if equal(a2740, a2741) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2742, a2743)?
            };
            let a2755 = {
                let a2752 = {
                    let a2746 = {
                        let a2744 = v1;
                        let a2745 = 32u8;
                        fixed_u8::bit_and(a2744, a2745)
                    };
                    let a2747 = 32u8;
                    if equal(a2746, a2747) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2753 = {
                    let a2750 = {
                        let a2748 = v1;
                        let a2749 = 16u8;
                        fixed_u8::bit_and(a2748, a2749)
                    };
                    let a2751 = 16u8;
                    if equal(a2750, a2751) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2752, a2753)?
            };
            nat_add(a2754, a2755)?
        };
        let a2779 = {
            let a2776 = {
                let a2764 = {
                    let a2758 = {
                        let a2756 = v1;
                        let a2757 = 8u8;
                        fixed_u8::bit_and(a2756, a2757)
                    };
                    let a2759 = 8u8;
                    if equal(a2758, a2759) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2765 = {
                    let a2762 = {
                        let a2760 = v1;
                        let a2761 = 4u8;
                        fixed_u8::bit_and(a2760, a2761)
                    };
                    let a2763 = 4u8;
                    if equal(a2762, a2763) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2764, a2765)?
            };
            let a2777 = {
                let a2774 = {
                    let a2768 = {
                        let a2766 = v1;
                        let a2767 = 2u8;
                        fixed_u8::bit_and(a2766, a2767)
                    };
                    let a2769 = 2u8;
                    if equal(a2768, a2769) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2775 = {
                    let a2772 = {
                        let a2770 = v1;
                        let a2771 = 1u8;
                        fixed_u8::bit_and(a2770, a2771)
                    };
                    let a2773 = 1u8;
                    if equal(a2772, a2773) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2774, a2775)?
            };
            nat_add(a2776, a2777)?
        };
        nat_add(a2778, a2779)?
    };
    let a2780 = 48u64;
    let a2781 = v2;
    if if nat_le(a2780, a2781) {
        let a2782 = v2;
        let a2783 = 57u64;
        nat_le(a2782, a2783)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2784 = 97u64;
        let a2785 = v2;
        if nat_le(a2784, a2785) {
            let a2786 = v2;
            let a2787 = 102u64;
            Ok(nat_le(a2786, a2787))
        } else {
            Ok(false)
        }
    }
}

pub fn f560() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f561(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2832 = {
            let a2808 = {
                let a2796 = {
                    let a2790 = {
                        let a2788 = v1;
                        let a2789 = 128u8;
                        fixed_u8::bit_and(a2788, a2789)
                    };
                    let a2791 = 128u8;
                    if equal(a2790, a2791) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2797 = {
                    let a2794 = {
                        let a2792 = v1;
                        let a2793 = 64u8;
                        fixed_u8::bit_and(a2792, a2793)
                    };
                    let a2795 = 64u8;
                    if equal(a2794, a2795) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2796, a2797)?
            };
            let a2809 = {
                let a2806 = {
                    let a2800 = {
                        let a2798 = v1;
                        let a2799 = 32u8;
                        fixed_u8::bit_and(a2798, a2799)
                    };
                    let a2801 = 32u8;
                    if equal(a2800, a2801) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2807 = {
                    let a2804 = {
                        let a2802 = v1;
                        let a2803 = 16u8;
                        fixed_u8::bit_and(a2802, a2803)
                    };
                    let a2805 = 16u8;
                    if equal(a2804, a2805) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2806, a2807)?
            };
            nat_add(a2808, a2809)?
        };
        let a2833 = {
            let a2830 = {
                let a2818 = {
                    let a2812 = {
                        let a2810 = v1;
                        let a2811 = 8u8;
                        fixed_u8::bit_and(a2810, a2811)
                    };
                    let a2813 = 8u8;
                    if equal(a2812, a2813) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2819 = {
                    let a2816 = {
                        let a2814 = v1;
                        let a2815 = 4u8;
                        fixed_u8::bit_and(a2814, a2815)
                    };
                    let a2817 = 4u8;
                    if equal(a2816, a2817) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2818, a2819)?
            };
            let a2831 = {
                let a2828 = {
                    let a2822 = {
                        let a2820 = v1;
                        let a2821 = 2u8;
                        fixed_u8::bit_and(a2820, a2821)
                    };
                    let a2823 = 2u8;
                    if equal(a2822, a2823) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2829 = {
                    let a2826 = {
                        let a2824 = v1;
                        let a2825 = 1u8;
                        fixed_u8::bit_and(a2824, a2825)
                    };
                    let a2827 = 1u8;
                    if equal(a2826, a2827) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2828, a2829)?
            };
            nat_add(a2830, a2831)?
        };
        nat_add(a2832, a2833)?
    };
    let a2834 = v2;
    let a2835 = 57u64;
    if nat_le(a2834, a2835) {
        let a2836 = v2;
        let a2837 = 48u64;
        Ok(nat_sub(a2836, a2837))
    } else {
        let a2838 = v2;
        let a2839 = 87u64;
        Ok(nat_sub(a2838, a2839))
    }
}

pub fn f562(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2840, _) = v0;
        h2840
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2841) = v0;
        h2841
    };
    let m2842 = v1;
    match m2842.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2843 = v4;
            match m2843.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f594(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f563(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f525(f46(v0.clone(), 8u64, {
            let a2845 = {
                let a2844 = v0;
                length_bytes(a2844)
            };
            let a2846 = 8u64;
            nat_sub(a2845, a2846)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f525(f46(v0.clone(), 14u64, {
                let a2848 = {
                    let a2847 = v0;
                    length_bytes(a2847)
                };
                let a2849 = 14u64;
                nat_sub(a2848, a2849)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f564(p2866: u64, p2867: u64, p2868: u64, p2869: Bytes) -> Bytes {
    let m2865 = (p2866, (p2867, (p2868, p2869)));
    {
        let mut m2865 = m2865;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2865;
                let m2850 = v0;
                if m2850 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2850 - 1;
                    if f118({
                        let a2851 = v1;
                        let a2852 = 0u64;
                        nat_eq(a2851, a2852)
                    }, {
                        let a2853 = v2;
                        let a2854 = 0u64;
                        nat_eq(a2853, a2854)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2855 = v1;
                            let a2856 = 16u64;
                            let a2857 = 0u64;
                            nat_quot(a2855, a2856, a2857)
                        }, ({
                            let a2858 = v2;
                            let a2859 = 1u64;
                            nat_sub(a2858, a2859)
                        }, {
                            let a2863 = f595({
                                let a2860 = v1;
                                let a2861 = 16u64;
                                let a2862 = 0u64;
                                nat_rem(a2860, a2861, a2862)
                            });
                            let a2864 = v3;
                            append_bytes(a2863, a2864)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f565(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f596(f597(f283(v0.clone()))?, v0, v1)
}

pub fn f566(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f567(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f568(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt28, Bytes>> {
    let m2870 = v1.clone();
    match m2870.uncons() {
        None => {
            Ok(f566(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2871 = f598(f489(f279(v2, f8(Str::lit("properties")))), v1.clone());
            match m2871.uncons() {
                None => {
                    let m2872 = f599(f597(f600(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2872 {
                        Ok(v5) => {
                            Ok(f567(f571(v0, f572(v5.clone())), f573(v5)))
                        }
                        Err(v4) => {
                            Ok(f566(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2873 = f601(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2873 {
                        Ok(v9) => {
                            Ok(f567(f571(v0, f572(v9.clone())), f573(v9)))
                        }
                        Err(v8) => {
                            Ok(f566(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f569(v0: Adt2, v1: Bytes) -> R<bool> {
    let m2874 = v0;
    match m2874 {
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
        Adt2::C6(r2875) => {
            let v2 = (*r2875).clone();
            f569(v2, v1)
        }
        Adt2::C7(r2876) => {
            let v3 = (*r2876).clone();
            f569(v3, v1)
        }
        Adt2::C8 => {
            Ok(false)
        }
        Adt2::C9(r2877, r2878) => {
            let v4 = (*r2877).clone();
            let v5 = (*r2878).clone();
            if f569(v4, v1.clone())? {
                Ok(true)
            } else {
                f569(v5, v1)
            }
        }
        Adt2::C10(v6, r2879, r2880) => {
            let v7 = (*r2879).clone();
            let v8 = (*r2880).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f602(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f569(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f569(v8, v1)
                }
            }
        }
    }
}

pub fn f570(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f603(f597(f604(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f571(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f387(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f572(v0: Adt29) -> List<Bytes> {
    let m2881 = v0;
    match m2881 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f573(v0: Adt29) -> List<Adt25> {
    let m2882 = v0;
    match m2882 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f574(v0: Adt2) -> bool {
    let m2883 = f489(f279(v0.clone(), f8(Str::lit("properties"))));
    match m2883.uncons() {
        None => {
            let m2884 = f605(v0);
            match m2884 {
                None => {
                    false
                }
                Some(v3) => {
                    f606(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f575(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f576(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2885 = f607(v0.clone());
    match m2885 {
        None => {
            if f32(f282(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2886 = f279(v0, f8(Str::lit("items")));
                match m2886 {
                    None => {
                        Ok(f608(f8(Str::lit("List")), f610()))
                    }
                    Some(v3) => {
                        Ok(f608(f8(Str::lit("List")), f609(v3, {
                            let a2887 = v1;
                            let a2888 = f8(Str::lit("Item"));
                            append_bytes(a2887, a2888)
                        })?))
                    }
                }
            } else {
                f609(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f608(f8(Str::lit("Option")), f609(v2, v1)?))
        }
    }
}

pub fn f577(p2891: List<Adt2>) -> bool {
    let m2890 = p2891;
    {
        let mut m2890 = m2890;
        loop {
            let step = {
                let v0 = m2890;
                let m2889 = v0;
                match m2889.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f611(f522(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(v0: Adt2, v1: Bytes) -> Adt2 {
    let m2892 = v0;
    match m2892 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r2893) => {
            let v7 = (*r2893).clone();
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
        Adt2::C6(r2894) => {
            let v15 = (*r2894).clone();
            Adt2::C6(std::rc::Rc::new(v15))
        }
        Adt2::C7(r2895) => {
            let v5 = (*r2895).clone();
            Adt2::C7(std::rc::Rc::new(f578(v5, v1)))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r2896, r2897) => {
            let v16 = (*r2896).clone();
            let v17 = (*r2897).clone();
            Adt2::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt2::C10(v2, r2898, r2899) => {
            let v3 = (*r2898).clone();
            let v4 = (*r2899).clone();
            if f32(v2.clone(), v1.clone()) {
                f578(v4, v1)
            } else {
                Adt2::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f578(v4, v1)))
            }
        }
    }
}

pub fn f579(v0: Adt2, v1: Bytes) -> bool {
    let m2900 = f279(v0, v1);
    match m2900 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f580(v0: List<Adt2>) -> bool {
    let m2901 = v0.clone();
    match m2901.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2902 = f598(f489(f279(v1, f8(Str::lit("properties")))), v0.clone());
            match m2902.uncons() {
                None => {
                    f613(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f612(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f581(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m2903 = v0;
    match m2903 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r2904) => {
            let v8 = (*r2904).clone();
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
        Adt2::C6(r2905) => {
            let v16 = (*r2905).clone();
            Adt2::C6(std::rc::Rc::new(v16))
        }
        Adt2::C7(r2906) => {
            let v3 = (*r2906).clone();
            Adt2::C7(std::rc::Rc::new(Adt2::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r2907, r2908) => {
            let v17 = (*r2907).clone();
            let v18 = (*r2908).clone();
            Adt2::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt2::C10(v4, r2909, r2910) => {
            let v5 = (*r2909).clone();
            let v6 = (*r2910).clone();
            Adt2::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f582(p2913: List<Adt2>) -> bool {
    let m2912 = p2913;
    {
        let mut m2912 = m2912;
        loop {
            let step = {
                let v0 = m2912;
                let m2911 = v0;
                match m2911.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f614(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: u64) -> u32 {
    let a2914 = v0;
    let a2915 = 32u64;
    if nat_lt(a2914, a2915) {
        let a2916 = v0;
        let a2917 = 16u64;
        if nat_lt(a2916, a2917) {
            let a2918 = v0;
            let a2919 = 8u64;
            if nat_lt(a2918, a2919) {
                let a2920 = v0;
                let a2921 = 4u64;
                if nat_lt(a2920, a2921) {
                    let a2922 = v0;
                    let a2923 = 2u64;
                    if nat_lt(a2922, a2923) {
                        let a2924 = v0;
                        let a2925 = 1u64;
                        if nat_lt(a2924, a2925) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2926 = v0;
                        let a2927 = 3u64;
                        if nat_lt(a2926, a2927) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2928 = v0;
                    let a2929 = 6u64;
                    if nat_lt(a2928, a2929) {
                        let a2930 = v0;
                        let a2931 = 5u64;
                        if nat_lt(a2930, a2931) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2932 = v0;
                        let a2933 = 7u64;
                        if nat_lt(a2932, a2933) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2934 = v0;
                let a2935 = 12u64;
                if nat_lt(a2934, a2935) {
                    let a2936 = v0;
                    let a2937 = 10u64;
                    if nat_lt(a2936, a2937) {
                        let a2938 = v0;
                        let a2939 = 9u64;
                        if nat_lt(a2938, a2939) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2940 = v0;
                        let a2941 = 11u64;
                        if nat_lt(a2940, a2941) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2942 = v0;
                    let a2943 = 14u64;
                    if nat_lt(a2942, a2943) {
                        let a2944 = v0;
                        let a2945 = 13u64;
                        if nat_lt(a2944, a2945) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2946 = v0;
                        let a2947 = 15u64;
                        if nat_lt(a2946, a2947) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2948 = v0;
            let a2949 = 24u64;
            if nat_lt(a2948, a2949) {
                let a2950 = v0;
                let a2951 = 20u64;
                if nat_lt(a2950, a2951) {
                    let a2952 = v0;
                    let a2953 = 18u64;
                    if nat_lt(a2952, a2953) {
                        let a2954 = v0;
                        let a2955 = 17u64;
                        if nat_lt(a2954, a2955) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2956 = v0;
                        let a2957 = 19u64;
                        if nat_lt(a2956, a2957) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2958 = v0;
                    let a2959 = 22u64;
                    if nat_lt(a2958, a2959) {
                        let a2960 = v0;
                        let a2961 = 21u64;
                        if nat_lt(a2960, a2961) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2962 = v0;
                        let a2963 = 23u64;
                        if nat_lt(a2962, a2963) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2964 = v0;
                let a2965 = 28u64;
                if nat_lt(a2964, a2965) {
                    let a2966 = v0;
                    let a2967 = 26u64;
                    if nat_lt(a2966, a2967) {
                        let a2968 = v0;
                        let a2969 = 25u64;
                        if nat_lt(a2968, a2969) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2970 = v0;
                        let a2971 = 27u64;
                        if nat_lt(a2970, a2971) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2972 = v0;
                    let a2973 = 30u64;
                    if nat_lt(a2972, a2973) {
                        let a2974 = v0;
                        let a2975 = 29u64;
                        if nat_lt(a2974, a2975) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2976 = v0;
                        let a2977 = 31u64;
                        if nat_lt(a2976, a2977) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2978 = v0;
        let a2979 = 48u64;
        if nat_lt(a2978, a2979) {
            let a2980 = v0;
            let a2981 = 40u64;
            if nat_lt(a2980, a2981) {
                let a2982 = v0;
                let a2983 = 36u64;
                if nat_lt(a2982, a2983) {
                    let a2984 = v0;
                    let a2985 = 34u64;
                    if nat_lt(a2984, a2985) {
                        let a2986 = v0;
                        let a2987 = 33u64;
                        if nat_lt(a2986, a2987) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2988 = v0;
                        let a2989 = 35u64;
                        if nat_lt(a2988, a2989) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2990 = v0;
                    let a2991 = 38u64;
                    if nat_lt(a2990, a2991) {
                        let a2992 = v0;
                        let a2993 = 37u64;
                        if nat_lt(a2992, a2993) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2994 = v0;
                        let a2995 = 39u64;
                        if nat_lt(a2994, a2995) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2996 = v0;
                let a2997 = 44u64;
                if nat_lt(a2996, a2997) {
                    let a2998 = v0;
                    let a2999 = 42u64;
                    if nat_lt(a2998, a2999) {
                        let a3000 = v0;
                        let a3001 = 41u64;
                        if nat_lt(a3000, a3001) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3002 = v0;
                        let a3003 = 43u64;
                        if nat_lt(a3002, a3003) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3004 = v0;
                    let a3005 = 46u64;
                    if nat_lt(a3004, a3005) {
                        let a3006 = v0;
                        let a3007 = 45u64;
                        if nat_lt(a3006, a3007) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3008 = v0;
                        let a3009 = 47u64;
                        if nat_lt(a3008, a3009) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3010 = v0;
            let a3011 = 56u64;
            if nat_lt(a3010, a3011) {
                let a3012 = v0;
                let a3013 = 52u64;
                if nat_lt(a3012, a3013) {
                    let a3014 = v0;
                    let a3015 = 50u64;
                    if nat_lt(a3014, a3015) {
                        let a3016 = v0;
                        let a3017 = 49u64;
                        if nat_lt(a3016, a3017) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3018 = v0;
                        let a3019 = 51u64;
                        if nat_lt(a3018, a3019) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3020 = v0;
                    let a3021 = 54u64;
                    if nat_lt(a3020, a3021) {
                        let a3022 = v0;
                        let a3023 = 53u64;
                        if nat_lt(a3022, a3023) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3024 = v0;
                        let a3025 = 55u64;
                        if nat_lt(a3024, a3025) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3026 = v0;
                let a3027 = 60u64;
                if nat_lt(a3026, a3027) {
                    let a3028 = v0;
                    let a3029 = 58u64;
                    if nat_lt(a3028, a3029) {
                        let a3030 = v0;
                        let a3031 = 57u64;
                        if nat_lt(a3030, a3031) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3032 = v0;
                        let a3033 = 59u64;
                        if nat_lt(a3032, a3033) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3034 = v0;
                    let a3035 = 62u64;
                    if nat_lt(a3034, a3035) {
                        let a3036 = v0;
                        let a3037 = 61u64;
                        if nat_lt(a3036, a3037) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3038 = v0;
                        let a3039 = 63u64;
                        if nat_lt(a3038, a3039) {
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

pub fn f584(v0: u32) -> u32 {
    let a3042 = {
        let a3040 = f615(v0, 6u32, 26u32);
        let a3041 = f615(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3040, a3041)
    };
    let a3043 = f615(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3042, a3043)
}

pub fn f585(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3049 = {
        let a3044 = v0;
        let a3045 = v1;
        fixed_u32::bit_and(a3044, a3045)
    };
    let a3050 = {
        let a3047 = {
            let a3046 = v0;
            fixed_u32::bit_not(a3046)
        };
        let a3048 = v2;
        fixed_u32::bit_and(a3047, a3048)
    };
    fixed_u32::bit_xor(a3049, a3050)
}

pub fn f586(v0: u32) -> u32 {
    let a3053 = {
        let a3051 = f615(v0, 2u32, 30u32);
        let a3052 = f615(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3051, a3052)
    };
    let a3054 = f615(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3053, a3054)
}

pub fn f587(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3063 = {
        let a3059 = {
            let a3055 = v0;
            let a3056 = v1;
            fixed_u32::bit_and(a3055, a3056)
        };
        let a3060 = {
            let a3057 = v0;
            let a3058 = v2;
            fixed_u32::bit_and(a3057, a3058)
        };
        fixed_u32::bit_xor(a3059, a3060)
    };
    let a3064 = {
        let a3061 = v1;
        let a3062 = v2;
        fixed_u32::bit_and(a3061, a3062)
    };
    fixed_u32::bit_xor(a3063, a3064)
}

pub fn f588(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3065, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3065
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3066, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3066
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3067, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3067
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3068, _, _, _, _, _, _, _, _, _, _, _) => {
            h3068
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3069, _, _, _, _, _, _, _, _, _, _) => {
            h3069
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3070, _, _, _, _, _, _, _, _, _) => {
            h3070
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3071, _, _, _, _, _, _, _, _) => {
            h3071
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3072, _, _, _, _, _, _, _) => {
            h3072
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3073, _, _, _, _, _, _) => {
            h3073
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3074, _, _, _, _, _) => {
            h3074
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3075, _, _, _, _) => {
            h3075
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3076, _, _, _) => {
            h3076
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3077, _, _) => {
            h3077
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3078, _) => {
            h3078
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3079) => {
            h3079
        }
    }, f535(f616(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3080, _) => {
            h3080
        }
    }), f535(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3081, _, _, _, _, _, _) => {
            h3081
        }
    }, f535(f617(match v0.clone() {
        Adt26::C0(_, h3082, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3082
        }
    }), match v0 {
        Adt26::C0(h3083, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3083
        }
    }))))
}

pub fn f589(v0: Fn21, p3087: u64, p3088: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3086 = (p3087, p3088);
    {
        let mut m3086 = m3086;
        loop {
            let step = {
                let (v1, v2) = m3086;
                let m3084 = v1;
                if m3084 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3084 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3085: Fn21 = v0.clone();
                        c3085.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3086 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3089, _) = v0;
        h3089
    };
    let v2: u32 = {
        let (h3090, _) = {
            let (_, h3091) = v0;
            h3091
        };
        h3090
    };
    let v3: u32 = {
        let (_, h3092) = {
            let (_, h3093) = v0;
            h3093
        };
        h3092
    };
    ({
        let a3094 = v1;
        let a3095 = 2u64;
        let a3096 = 0u64;
        nat_quot(a3094, a3095, a3096)
    }, (f591(v2, 1u32), {
        let a3100 = {
            let a3097 = v1;
            let a3098 = 2u64;
            let a3099 = 0u64;
            nat_rem(a3097, a3098, a3099)
        };
        let a3101 = 1u64;
        if nat_eq(a3100, a3101) {
            let a3102 = v3;
            let a3103 = v2;
            fixed_u32::bit_or(a3102, a3103)
        } else {
            v3
        }
    }))
}

pub fn f591(v0: u32, v1: u32) -> u32 {
    let m3106 = {
        let a3104 = v0;
        let a3105 = v1;
        fixed_u32::shift_left(a3104, a3105)
    };
    match m3106 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f592(v0: Fn22, p3111: u64, p3112: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3110 = (p3111, p3112);
    {
        let mut m3110 = m3110;
        loop {
            let step = {
                let (v1, v2) = m3110;
                let m3107 = v1;
                if m3107 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3107 - 1;
                    let m3109 = {
                        let c3108: Fn22 = v0.clone();
                        c3108.apply(v2.clone())?
                    };
                    match m3109 {
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
                Ok(next) => { m3110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f618(v0)
}

pub fn f594(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3117 = {
            let a3115 = {
                let a3113 = v0.clone();
                length_list(a3113)
            };
            let a3116 = {
                let a3114 = v1.clone();
                length_list(a3114)
            };
            nat_add(a3115, a3116)?
        };
        let a3118 = 1u64;
        nat_add(a3117, a3118)?
    };
    Ok(f434({
        let (_, h3119) = {
            let (_, h3120) = {
                let (h3121, _) = f619(Fn23::F620, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3121
            };
            h3120
        };
        h3119
    }))
}

pub fn f595(v0: u64) -> Bytes {
    f46({
        let a3122 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3122)
    }, v0, 1u64)
}

pub fn f596(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3123 = v1;
    match m3123.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3124 = f621(v0, v3.clone())?;
            match m3124 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f622(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3125 = f596(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3125 {
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

pub fn f597(p3129: List<Bytes>) -> R<bool> {
    let m3128 = p3129;
    {
        let mut m3128 = m3128;
        loop {
            let step = {
                let v0 = m3128;
                let m3126 = v0;
                match m3126.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3127 = f623(v1)?;
                        match m3127 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f624(false, v2.clone())?) {
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
                Ok(next) => { m3128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3130 = v0;
    match m3130.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f625(v1.clone(), {
                let (h3131, _) = v2.clone();
                h3131
            }) {
                List::cons({
                    let (h3132, _) = v2;
                    h3132
                }, f598(v3, v1))
            } else {
                f598(v3, v1)
            }
        }
    }
}

pub fn f599(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3133 = v2;
    match m3133.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3134 = f626(v4.clone())?;
            match m3134 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3135 = f627(v0, v6.clone())?;
                    match m3135 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3138 = f628(v4, {
                                    let a3136 = v1.clone();
                                    let a3137 = f373(v6)?;
                                    append_bytes(a3136, a3137)
                                })?;
                                match m3138 {
                                    Ok(v9) => {
                                        let m3139 = f599(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3139 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f530(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f572(v11.clone())), f493(f529(v9), f573(v11)))))
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

pub fn f600(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3140 = v0;
    match m3140.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3141 = f626(v1)?;
            match m3141 {
                None => {
                    f600(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f600(v2)?))
                }
            }
        }
    }
}

pub fn f601(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f629(f597(f630(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f602(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3142 = f379(Some(v0));
    match m3142 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3143 = f563(v2)?;
            match m3143 {
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

pub fn f603(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m3144 = v2;
    match m3144.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3146 = f631(v0, {
                let (h3145, _) = v5.clone();
                h3145
            })?;
            match m3146 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3147, _) = v5;
                        h3147
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f632(v0, f604(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3152 = f576({
                            let (_, h3148) = v5.clone();
                            h3148
                        }, {
                            let a3150 = v1.clone();
                            let a3151 = f373({
                                let (h3149, _) = v5.clone();
                                h3149
                            })?;
                            append_bytes(a3150, a3151)
                        })?;
                        match m3152 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3153, _) = v5;
                                    h3153
                                }, v3.clone()) {
                                    f530(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f530(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3154 = f603(v0, v1, v6, v3, v4)?;
                                match m3154 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f572(v12.clone())), f493(f529(v9), f573(v12)))))
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

pub fn f604(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m3155 = v0;
    match m3155.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3156, _) = v1;
                h3156
            }, f604(v2))
        }
    }
}

pub fn f605(v0: Adt2) -> Option<Adt2> {
    let m3157 = f279(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3157 {
        None => {
            f633(v0)
        }
        Some(v1) => {
            if f606(v1.clone()) {
                let m3158 = f633(v0);
                match m3158 {
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

pub fn f606(v0: Adt2) -> bool {
    let m3159 = v0;
    match m3159 {
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

pub fn f607(v0: Adt2) -> Option<Adt2> {
    let m3160 = f284(f279(v0, f8(Str::lit("oneOf"))));
    match m3160.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3161 = v2;
            match m3161.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3162 = v4;
                    match m3162.uncons() {
                        None => {
                            if f634(v3.clone()) {
                                Some(v1)
                            } else {
                                if f634(v1) {
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

pub fn f608(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3163 = v1;
    match m3163 {
        Ok(v3) => {
            f567(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f530(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f529(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f609(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3164 = f607(v0.clone());
    match m3164 {
        None => {
            if f32(f282(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3165 = f279(v0, f8(Str::lit("items")));
                match m3165 {
                    None => {
                        Ok(f608(f8(Str::lit("List")), f610()))
                    }
                    Some(v3) => {
                        Ok(f608(f8(Str::lit("List")), f628(v3, {
                            let a3166 = v1;
                            let a3167 = f8(Str::lit("Item"));
                            append_bytes(a3166, a3167)
                        })?))
                    }
                }
            } else {
                f628(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f608(f8(Str::lit("Option")), f628(v2, v1)?))
        }
    }
}

pub fn f610() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f526(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f611(p3171: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3170 = p3171;
    {
        let mut m3170 = m3170;
        loop {
            let step = {
                let v0 = m3170;
                let m3168 = v0;
                match m3168.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3169, _) = v2;
                            h3169
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f612(p3175: List<Adt2>, v1: Bytes, p3176: List<Bytes>) -> bool {
    let m3174 = (p3175, p3176);
    {
        let mut m3174 = m3174;
        loop {
            let step = {
                let (v0, v2) = m3174;
                let m3172 = v0;
                match m3172.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3173 = f635(v3, v1.clone());
                        match m3173 {
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
                Ok(next) => { m3174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f613(p3179: List<Adt2>, p3180: List<Bytes>) -> bool {
    let m3178 = (p3179, p3180);
    {
        let mut m3178 = m3178;
        loop {
            let step = {
                let (v0, v1) = m3178;
                let m3177 = v0;
                match m3177.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f282(v2, f8(Str::lit("type")));
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
                Ok(next) => { m3178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3181 = f279(v0.clone(), f8(Str::lit("anyOf")));
        match m3181 {
            None => {
                Adt2::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f611(f522(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f577(f380(v2))
    } else {
        false
    }
}

pub fn f615(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3182 = f502(v0, v1);
    let a3183 = f591(v0, v2);
    fixed_u32::bit_or(a3182, a3183)
}

pub fn f616(v0: u32) -> u32 {
    let a3186 = {
        let a3184 = f615(v0, 17u32, 15u32);
        let a3185 = f615(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3184, a3185)
    };
    let a3187 = f502(v0, 10u32);
    fixed_u32::bit_xor(a3186, a3187)
}

pub fn f617(v0: u32) -> u32 {
    let a3190 = {
        let a3188 = f615(v0, 7u32, 25u32);
        let a3189 = f615(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3188, a3189)
    };
    let a3191 = f502(v0, 3u32);
    fixed_u32::bit_xor(a3190, a3191)
}

pub fn f618(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3192, _) = v0;
        h3192
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3193) = v0;
        h3193
    };
    let m3194 = v1;
    match m3194.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3195 = v4;
            match m3195.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f636(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f619(v0: Fn23, p3200: u64, p3201: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3199 = (p3200, p3201);
    {
        let mut m3199 = m3199;
        loop {
            let step = {
                let (v1, v2) = m3199;
                let m3196 = v1;
                if m3196 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3196 - 1;
                    let m3198 = {
                        let c3197: Fn23 = v0.clone();
                        c3197.apply(v2.clone())
                    };
                    match m3198 {
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
                Ok(next) => { m3199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f637(v0)
}

pub fn f621(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3202 = v1;
    match m3202 {
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
                let a3213 = if f118(v7, {
                    let a3211 = 0u64;
                    let a3212 = v8;
                    nat_lt(a3211, a3212)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3214 = f301(false, v8)?;
                append_bytes(a3213, a3214)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3205 = f638({
                let a3203 = f8(Str::lit("n"));
                let a3204 = f450(false, v4, v5)?;
                append_bytes(a3203, a3204)
            })?;
            match m3205 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3209 = f8(Str::lit("nMinus"));
                        let a3210 = f46(v6.clone(), 1u64, {
                            let a3207 = {
                                let a3206 = v6;
                                length_bytes(a3206)
                            };
                            let a3208 = 1u64;
                            nat_sub(a3207, a3208)
                        });
                        append_bytes(a3209, a3210)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f627(v0, v2)
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

pub fn f622(v0: Adt2) -> R<Bytes> {
    f378(v0)
}

pub fn f623(v0: Bytes) -> R<Option<Bytes>> {
    let m3215 = f639(v0)?;
    match m3215 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f640(v1)?))
        }
    }
}

pub fn f624(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3216 = v1;
    match m3216.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3217 = f627(v0, v2)?;
            match m3217 {
                None => {
                    f624(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f624(v0, v3)?))
                }
            }
        }
    }
}

pub fn f625(p3221: List<Adt2>, v1: Bytes) -> bool {
    let m3220 = p3221;
    {
        let mut m3220 = m3220;
        loop {
            let step = {
                let v0 = m3220;
                let m3218 = v0;
                match m3218.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3219 = f635(v2, v1.clone());
                        if match m3219 {
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
                Ok(next) => { m3220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(v0: Adt2) -> R<Option<Bytes>> {
    let m3222 = f379(f279(v0.clone(), f8(Str::lit("$ref"))));
    match m3222 {
        None => {
            Ok(f379(f279(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f563(v1)
        }
    }
}

pub fn f627(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f638(v1)
    } else {
        f623(v1)
    }
}

pub fn f628(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f641(f550(v0), v1)
}

pub fn f629(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3223 = v2;
    match m3223.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3224 = f635(v5.clone(), v3.clone());
            match m3224 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3225 = f627(v0, v7.clone())?;
                    match m3225 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3228 = f570({
                                    let a3226 = v1.clone();
                                    let a3227 = f373(v7)?;
                                    append_bytes(a3226, a3227)
                                }, f524(f489(f279(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f283(f284(f279(v5, f8(Str::lit("required"))))), true)?;
                                match m3228 {
                                    Ok(v10) => {
                                        let m3229 = f629(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3229 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f387(f572(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f572(v12.clone())), f493(f573(v10), f573(v12)))))
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

pub fn f630(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3230 = v0;
    match m3230.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3231 = f635(v2, v1.clone());
            match m3231 {
                None => {
                    f630(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f630(v3, v1))
                }
            }
        }
    }
}

pub fn f631(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3232 = f627(v0, v1)?;
    match m3232 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f548(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f632(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3233 = v1;
    match m3233.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3234 = f631(v0, v2)?;
            match m3234 {
                None => {
                    f632(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f632(v0, v3)?))
                }
            }
        }
    }
}

pub fn f633(v0: Adt2) -> Option<Adt2> {
    let m3235 = f489(f279(v0, f8(Str::lit("patternProperties"))));
    match m3235.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3236 = v2;
            match m3236.uncons() {
                None => {
                    Some({
                        let (_, h3237) = v1;
                        h3237
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f634(v0: Adt2) -> bool {
    f32(f282(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f635(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f283(f284(f279(v0.clone(), f8(Str::lit("required")))))) {
        let m3238 = f279(v0, f8(Str::lit("properties")));
        match m3238 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3239 = f279(v2, v1);
                match m3239 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f379(f279(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f636(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3244 = {
            let a3242 = {
                let a3240 = v0.clone();
                length_list(a3240)
            };
            let a3243 = {
                let a3241 = v1.clone();
                length_list(a3241)
            };
            nat_add(a3242, a3243)?
        };
        let a3245 = 1u64;
        nat_add(a3244, a3245)?
    };
    Ok(f642({
        let (_, h3246) = {
            let (_, h3247) = {
                let (h3248, _) = f643(Fn24::F644, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3248
            };
            h3247
        };
        h3246
    }))
}

pub fn f637(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3249, _) = v0.clone();
        h3249
    };
    let v2: List<(Bytes, ())> = {
        let (h3250, _) = {
            let (_, h3251) = v0.clone();
            h3251
        };
        h3250
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3252) = {
            let (_, h3253) = v0;
            h3253
        };
        h3252
    };
    let m3254 = v1.clone();
    match m3254.uncons() {
        None => {
            let m3255 = v2;
            match m3255.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3256 = v2.clone();
            match m3256.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f645({
                        let (h3257, _) = v8.clone();
                        h3257
                    }, {
                        let (h3258, _) = v6.clone();
                        h3258
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

pub fn f638(v0: Bytes) -> R<Option<Bytes>> {
    let m3260 = f646({
        let a3259 = v0.clone();
        length_bytes(a3259)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3260 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3262 = {
                let a3261 = v1.clone();
                length_bytes(a3261)
            };
            let a3263 = 0u64;
            if nat_eq(a3262, a3263) {
                let a3265 = {
                    let a3264 = v0;
                    length_bytes(a3264)
                };
                let a3266 = 0u64;
                if nat_eq(a3265, a3266) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f640(v1)?))
            }
        }
    }
}

pub fn f639(v0: Bytes) -> R<Option<Bytes>> {
    if f647({
        let a3267 = v0.clone();
        length_bytes(a3267)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f648(v0)?;
        if f649(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3270 = {
                let a3268 = v1.clone();
                let a3269 = 0u64;
                index_bytes(a3268, a3269)
            };
            match m3270 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f487(f72(v2)?) {
                        Ok(Some({
                            let a3271 = f8(Str::lit("n"));
                            let a3272 = v1;
                            append_bytes(a3271, a3272)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f650({
            let a3273 = v0.clone();
            length_bytes(a3273)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f640(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f651()) {
        f548(v0)
    } else {
        Ok(v0)
    }
}

pub fn f641(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3274 = v0.clone();
    match m3274 {
        Adt2::C0 => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f610())
            } else {
                Ok(f566(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            let m3275 = f652(f522(v0.clone()));
            match m3275 {
                None => {
                    let m3276 = f379(f279(v0.clone(), f8(Str::lit("$ref"))));
                    match m3276 {
                        None => {
                            let m3278 = f279(v0.clone(), f8(Str::lit("allOf")));
                            match m3278 {
                                None => {
                                    let m3283 = f279(v0.clone(), f8(Str::lit("enum")));
                                    match m3283 {
                                        None => {
                                            let m3284 = f279(v0.clone(), f8(Str::lit("const")));
                                            match m3284 {
                                                None => {
                                                    let m3286 = f279(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3286 {
                                                        None => {
                                                            let v32: Bytes = f282(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v32.clone(), Bytes::lit(&[])) {
                                                                !f653(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f610())
                                                            } else {
                                                                if f32(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f567(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f32(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f567(f654(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f32(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f567(f655(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f32(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f567(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f32(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3287 = f489(f279(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3287.uncons() {
                                                                                        None => {
                                                                                            if f574(v0.clone()) {
                                                                                                Ok(f567(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3288 = f605(v0);
                                                                                                match m3288 {
                                                                                                    None => {
                                                                                                        Ok(f608(f8(Str::lit("List")), f656(f610())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f608(f8(Str::lit("List")), f656(f657(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f567(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f566(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f567(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3285 = f379(Some(v29));
                                                    match m3285 {
                                                        None => {
                                                            Ok(f566(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f567(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f567(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3279 = f380(v21);
                                    match m3279.uncons() {
                                        None => {
                                            Ok(f566(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3280 = v23;
                                            match m3280.uncons() {
                                                None => {
                                                    let m3281 = f379(f279(v22, f8(Str::lit("$ref"))));
                                                    match m3281 {
                                                        None => {
                                                            Ok(f566(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3282 = f563(v26)?;
                                                            match m3282 {
                                                                None => {
                                                                    Ok(f566(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f567(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f566(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3277 = f563(v19)?;
                            match m3277 {
                                None => {
                                    Ok(f566(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f567(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f566(if f32(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C8 => {
            Ok(f610())
        }
        Adt2::C9(_, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C10(_, _, _) => {
            Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f642(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f395(Fn6::F658, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f643(v0: Fn24, p3293: u64, p3294: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3292 = (p3293, p3294);
    {
        let mut m3292 = m3292;
        loop {
            let step = {
                let (v1, v2) = m3292;
                let m3289 = v1;
                if m3289 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3289 - 1;
                    let m3291 = {
                        let c3290: Fn24 = v0.clone();
                        c3290.apply(v2.clone())
                    };
                    match m3291 {
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
                Ok(next) => { m3292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f659(v0)
}

pub fn f645(v0: Bytes, v1: Bytes) -> bool {
    let a3299 = {
        let a3295 = v0;
        let a3296 = v1;
        compare_bytes(a3295, a3296)
    };
    let a3300 = {
        let a3297 = Bytes::lit(&[]);
        let a3298 = Bytes::lit(&[0x01]);
        compare_bytes(a3297, a3298)
    };
    equal(a3299, a3300)
}

pub fn f646(p3313: u64, v1: Bytes, p3314: u64, p3315: Bytes) -> R<Option<Bytes>> {
    let m3312 = (p3313, (p3314, p3315));
    {
        let mut m3312 = m3312;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3312;
                let m3301 = v0;
                if m3301 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3301 - 1;
                    let a3303 = {
                        let a3302 = v1.clone();
                        length_bytes(a3302)
                    };
                    let a3304 = v2;
                    if nat_le(a3303, a3304) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3305 = f660(v1.clone(), v2)?;
                        match m3305 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3307 = v2;
                                    let a3308 = {
                                        let (_, h3306) = v5;
                                        h3306
                                    };
                                    nat_add(a3307, a3308)?
                                }, {
                                    let a3310 = v3;
                                    let a3311 = f661(v1.clone(), v2, {
                                        let (h3309, _) = v5;
                                        h3309
                                    });
                                    append_bytes(a3310, a3311)
                                })))
                            }
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

pub fn f647(p3325: u64, v1: Bytes, p3326: u64) -> R<bool> {
    let m3324 = (p3325, p3326);
    {
        let mut m3324 = m3324;
        loop {
            let step = {
                let (v0, v2) = m3324;
                let m3316 = v0;
                if m3316 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3316 - 1;
                    let m3319 = {
                        let a3317 = v1.clone();
                        let a3318 = v2;
                        index_bytes(a3317, a3318)
                    };
                    match m3319 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3320 = f72(v4)?;
                            let a3321 = 128u64;
                            if nat_lt(a3320, a3321) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3322 = v2;
                                    let a3323 = 1u64;
                                    nat_add(a3322, a3323)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f446(v0.clone(), 0u64, {
        let a3327 = v0;
        length_bytes(a3327)
    }, false, Bytes::lit(&[]))?;
    let m3330 = {
        let a3328 = v1.clone();
        let a3329 = 0u64;
        index_bytes(a3328, a3329)
    };
    match m3330 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f486(v3) {
                let a3337 = f46({
                    let a3331 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3331)
                }, {
                    let a3332 = v3;
                    let a3333 = 65u64;
                    nat_sub(a3332, a3333)
                }, 1u64);
                let a3338 = f46(v1.clone(), 1u64, {
                    let a3335 = {
                        let a3334 = v1;
                        length_bytes(a3334)
                    };
                    let a3336 = 1u64;
                    nat_sub(a3335, a3336)
                });
                Ok(append_bytes(a3337, a3338))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f649(v0: Bytes) -> R<bool> {
    let m3341 = {
        let a3339 = v0;
        let a3340 = 0u64;
        index_bytes(a3339, a3340)
    };
    match m3341 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f207(f485(f72(v1)?), f486(f72(v1)?)))
        }
    }
}

pub fn f650(p3354: u64, v1: Bytes, p3355: u64, p3356: Bytes) -> R<Option<Bytes>> {
    let m3353 = (p3354, (p3355, p3356));
    {
        let mut m3353 = m3353;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3353;
                let m3342 = v0;
                if m3342 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3342 - 1;
                    let a3344 = {
                        let a3343 = v1.clone();
                        length_bytes(a3343)
                    };
                    let a3345 = v2;
                    if nat_le(a3344, a3345) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3346 = f660(v1.clone(), v2)?;
                        match m3346 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3348 = v2;
                                    let a3349 = {
                                        let (_, h3347) = v5;
                                        h3347
                                    };
                                    nat_add(a3348, a3349)?
                                }, {
                                    let a3351 = v3;
                                    let a3352 = f662({
                                        let (h3350, _) = v5;
                                        h3350
                                    });
                                    append_bytes(a3351, a3352)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3353 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f651() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f652(p3361: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3360 = p3361;
    {
        let mut m3360 = m3360;
        loop {
            let step = {
                let v0 = m3360;
                let m3357 = v0;
                match m3357.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h3358, _) = v1.clone();
                            h3358
                        }, f663()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3359, _) = v1;
                                h3359
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
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

pub fn f653(v0: Adt2) -> bool {
    if f579(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f579(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f579(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f579(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f579(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f579(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f654(v0: Adt2) -> Bytes {
    let m3362 = f664(f279(v0, f8(Str::lit("minimum"))));
    match m3362 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f655() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f656(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3363 = v0;
    match m3363 {
        Ok(v2) => {
            f567(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f530(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f529(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f657(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f665(16u64, v0, v1)
}

pub fn f658(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f659(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3364, _) = v0.clone();
        h3364
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3365, _) = {
            let (_, h3366) = v0.clone();
            h3366
        };
        h3365
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3367) = {
            let (_, h3368) = v0;
            h3368
        };
        h3367
    };
    let m3369 = v1.clone();
    match m3369.uncons() {
        None => {
            let m3370 = v2;
            match m3370.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3371 = v2.clone();
            match m3371.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f645({
                        let (h3372, _) = v8.clone();
                        h3372
                    }, {
                        let (h3373, _) = v6.clone();
                        h3373
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

pub fn f660(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3376 = {
        let a3374 = v0.clone();
        let a3375 = v1;
        index_bytes(a3374, a3375)
    };
    match m3376 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3377 = v3;
            let a3378 = 128u64;
            if nat_lt(a3377, a3378) {
                Ok(Some((v3, 1u64)))
            } else {
                if f118({
                    let a3379 = 194u64;
                    let a3380 = v3;
                    nat_le(a3379, a3380)
                }, {
                    let a3381 = v3;
                    let a3382 = 223u64;
                    nat_le(a3381, a3382)
                }) {
                    let m3385 = f666(v0, {
                        let a3383 = v1;
                        let a3384 = 1u64;
                        nat_add(a3383, a3384)?
                    })?;
                    match m3385 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f667({
                                let a3390 = {
                                    let a3388 = {
                                        let a3386 = v3;
                                        let a3387 = 192u64;
                                        nat_sub(a3386, a3387)
                                    };
                                    let a3389 = 64u64;
                                    nat_mul(a3388, a3389)?
                                };
                                let a3391 = v4;
                                nat_add(a3390, a3391)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f118({
                        let a3392 = 224u64;
                        let a3393 = v3;
                        nat_le(a3392, a3393)
                    }, {
                        let a3394 = v3;
                        let a3395 = 239u64;
                        nat_le(a3394, a3395)
                    }) {
                        let m3398 = f666(v0.clone(), {
                            let a3396 = v1;
                            let a3397 = 1u64;
                            nat_add(a3396, a3397)?
                        })?;
                        match m3398 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3401 = f666(v0, {
                                    let a3399 = v1;
                                    let a3400 = 2u64;
                                    nat_add(a3399, a3400)?
                                })?;
                                match m3401 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f667({
                                            let a3410 = {
                                                let a3404 = {
                                                    let a3402 = v3;
                                                    let a3403 = 224u64;
                                                    nat_sub(a3402, a3403)
                                                };
                                                let a3405 = 4096u64;
                                                nat_mul(a3404, a3405)?
                                            };
                                            let a3411 = {
                                                let a3408 = {
                                                    let a3406 = v5;
                                                    let a3407 = 64u64;
                                                    nat_mul(a3406, a3407)?
                                                };
                                                let a3409 = v6;
                                                nat_add(a3408, a3409)?
                                            };
                                            nat_add(a3410, a3411)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f118({
                            let a3412 = 240u64;
                            let a3413 = v3;
                            nat_le(a3412, a3413)
                        }, {
                            let a3414 = v3;
                            let a3415 = 244u64;
                            nat_le(a3414, a3415)
                        }) {
                            let m3418 = f666(v0.clone(), {
                                let a3416 = v1;
                                let a3417 = 1u64;
                                nat_add(a3416, a3417)?
                            })?;
                            match m3418 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3421 = f666(v0.clone(), {
                                        let a3419 = v1;
                                        let a3420 = 2u64;
                                        nat_add(a3419, a3420)?
                                    })?;
                                    match m3421 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3424 = f666(v0, {
                                                let a3422 = v1;
                                                let a3423 = 3u64;
                                                nat_add(a3422, a3423)?
                                            })?;
                                            match m3424 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f667({
                                                        let a3437 = {
                                                            let a3427 = {
                                                                let a3425 = v3;
                                                                let a3426 = 240u64;
                                                                nat_sub(a3425, a3426)
                                                            };
                                                            let a3428 = 262144u64;
                                                            nat_mul(a3427, a3428)?
                                                        };
                                                        let a3438 = {
                                                            let a3435 = {
                                                                let a3429 = v7;
                                                                let a3430 = 4096u64;
                                                                nat_mul(a3429, a3430)?
                                                            };
                                                            let a3436 = {
                                                                let a3433 = {
                                                                    let a3431 = v8;
                                                                    let a3432 = 64u64;
                                                                    nat_mul(a3431, a3432)?
                                                                };
                                                                let a3434 = v9;
                                                                nat_add(a3433, a3434)?
                                                            };
                                                            nat_add(a3435, a3436)?
                                                        };
                                                        nat_add(a3437, a3438)?
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

pub fn f661(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f207(f118(f207(f485(v2), f486(v2)), f118({
        let a3439 = v2;
        let a3440 = 117u64;
        !nat_eq(a3439, a3440)
    }, {
        let a3441 = v2;
        let a3442 = 85u64;
        !nat_eq(a3441, a3442)
    })), f118(f487(v2), {
        let a3443 = 0u64;
        let a3444 = v1;
        nat_lt(a3443, a3444)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a3445 = v2;
        let a3446 = 65536u64;
        if nat_lt(a3445, a3446) {
            let a3447 = f8(Str::lit("u"));
            let a3448 = f564(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3447, a3448)
        } else {
            let a3449 = f8(Str::lit("U"));
            let a3450 = f564(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3449, a3450)
        }
    }
}

pub fn f662(v0: u64) -> Bytes {
    let a3451 = f8(Str::lit("u"));
    let a3452 = f564(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3451, a3452)
}

pub fn f663() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f664(v0: Option<Adt2>) -> Option<u64> {
    let m3453 = v0;
    match m3453 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m3454 = v1;
            match m3454 {
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

pub fn f665(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3455 = v0;
    if m3455 == 0 {
        Ok(f566(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3455 - 1;
        let m3456 = v1.clone();
        match m3456 {
            Adt2::C0 => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f610())
                } else {
                    Ok(f566(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                let m3457 = f379(f279(v1.clone(), f8(Str::lit("$ref"))));
                match m3457 {
                    None => {
                        if if f579(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f579(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f579(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f567(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f282(v1.clone(), f8(Str::lit("type")));
                            if if f32(v22.clone(), Bytes::lit(&[])) {
                                !f653(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f610())
                            } else {
                                if f32(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f567(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f32(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f567(f654(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f32(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f567(f655(), List::<Adt25>::nil()))
                                        } else {
                                            if f32(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f567(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f32(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3459 = f279(v1, f8(Str::lit("items")));
                                                    match m3459 {
                                                        None => {
                                                            Ok(f608(f8(Str::lit("List")), f610()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f608(f8(Str::lit("List")), f665(v3, v23, {
                                                                let a3460 = v2;
                                                                let a3461 = f8(Str::lit("Item"));
                                                                append_bytes(a3460, a3461)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v22.clone(), f8(Str::lit("object"))) {
                                                        if f574(v1.clone()) {
                                                            Ok(f567(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3462 = f605(v1);
                                                            match m3462 {
                                                                None => {
                                                                    Ok(f608(f8(Str::lit("List")), f656(f610())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f608(f8(Str::lit("List")), f656(f665(v3, v24, {
                                                                        let a3463 = v2;
                                                                        let a3464 = f8(Str::lit("Value"));
                                                                        append_bytes(a3463, a3464)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f566(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3458 = f563(v20)?;
                        match m3458 {
                            None => {
                                Ok(f566(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f567(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C8 => {
                Ok(f610())
            }
            Adt2::C9(_, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C10(_, _, _) => {
                Ok(f566(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f666(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3467 = {
        let a3465 = v0;
        let a3466 = v1;
        index_bytes(a3465, a3466)
    };
    match m3467 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f118({
                let a3468 = 128u64;
                let a3469 = v3;
                nat_le(a3468, a3469)
            }, {
                let a3470 = v3;
                let a3471 = 191u64;
                nat_le(a3470, a3471)
            }) {
                Ok(Some({
                    let a3472 = v3;
                    let a3473 = 128u64;
                    nat_sub(a3472, a3473)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f667(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f118({
        let a3474 = v1;
        let a3475 = v0;
        nat_le(a3474, a3475)
    }, f118({
        let a3476 = v0;
        let a3477 = v2;
        nat_le(a3476, a3477)
    }, f207({
        let a3478 = v0;
        let a3479 = 55296u64;
        nat_lt(a3478, a3479)
    }, {
        let a3480 = 57343u64;
        let a3481 = v0;
        nat_lt(a3480, a3481)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
