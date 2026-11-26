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
    C0,
    C1(Bytes, std::rc::Rc<Adt18>),
    C2(bool),
    C3(bool, u64),
    C4(bool, u64, u64),
    C5(Bytes),
    C6(std::rc::Rc<Adt18>),
    C7(std::rc::Rc<Adt18>),
    C8,
    C9(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C10(Bytes, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
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
    F162(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F64(k0, k1) => f64(*k0, k1.clone(), p0),
            Fn0::F162(k0) => f162(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F94,
    F209,
    F300,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F94 => f94(p0, p1),
            Fn1::F209 => f209(p0, p1),
            Fn1::F300 => f300(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F176(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F176(k0) => f176(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F287(Bytes, u64, Bytes, u64),
    F364(Bytes),
    F365(u64, Bytes),
    F463(Bytes),
    F464(Bytes),
    F502(Bytes),
    F523(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F287(k0, k1, k2, k3) => f287(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F364(k0) => Ok(f364(k0.clone(), p0)),
            Fn3::F365(k0, k1) => f365(*k0, k1.clone(), p0),
            Fn3::F463(k0) => f463(k0.clone(), p0),
            Fn3::F464(k0) => f464(k0.clone(), p0),
            Fn3::F502(k0) => f502(k0.clone(), p0),
            Fn3::F523(k0) => f523(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F354(Bytes),
    F470(u64, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F354(k0) => f354(k0.clone(), p0),
            Fn4::F470(k0, k1) => Ok(f470(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F356,
    F635,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F356 => f356(p0, p1),
            Fn5::F635 => f635(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F359,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F359 => f359(p0, p1),
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
    F410(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn11::F410(k0, k1, k2) => f410(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F433,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F433 => f433(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F436,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F436 => f436(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F462(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn14::F462(k0) => f462(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F467(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn15::F467(k0) => f467(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F474,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F474 => f474(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F476,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F476 => f476(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F494,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F494 => f494(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F506,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F506 => f506(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F509,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F509 => f509(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F556,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F556 => f556(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F562,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F562 => f562(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F591,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F591 => f591(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F608(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F608(k0) => f608(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F618,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F618 => f618(p0),
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
                            if f28(v0.clone(), f8(Str::lit("record-decision"))) {
                                f156(v3, v1)
                            } else {
                                if f28(v0, f8(Str::lit("record-error"))) {
                                    f157(v3, v1, v2)
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
    f158(v0.clone(), 0u64, {
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
    let m343 = f159(v0)?;
    match m343 {
        None => {
            Ok(None::<Adt13>)
        }
        Some(v1) => {
            Ok(f160(v1))
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
            let m353 = f161(v2, v1.clone())?;
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
        let (h355, _) = f63(Fn0::F162(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
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
    Ok(f163(f46(v0, v1)?))
}

pub fn f83(v0: List<Adt13>, v1: Bytes) -> List<Adt13> {
    let m364 = v0;
    match m364.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f164(v2, v1.clone()), f83(v3, v1))
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
    let a373 = f165(v0)?;
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
        let m380 = f166(v0.clone(), f49())?;
        match m380 {
            None => {
                if f40(f41(f169(), match v0.clone() {
                    Adt1::C0(_, _, h381) => {
                        h381
                    }
                })) {
                    Ok(Adt11::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f169()))
                } else {
                    let m382 = f45(v0.clone(), List::cons(f170(), f171(f49())?));
                    match m382.uncons() {
                        None => {
                            Ok(Adt11::C3(f172(v0.clone(), f49(), v1.clone(), v2.clone(), f173(f53(v0, f170()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt11::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f43(f167(v3.clone())?, f168(v3, v2)))
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
    let m388 = f174(v0.clone(), v3);
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
    let m404 = f175(Fn2::F176(v1), None::<Bytes>, v0);
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))
}

pub fn f97(v0: Adt1) -> List<Bytes> {
    if f123(match v0.clone() {
        Adt1::C0(_, h405, _) => {
            h405
        }
    }, f177()) {
        if f123(match v0 {
            Adt1::C0(_, h406, _) => {
                h406
            }
        }, f178()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f178(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f177(), List::<Bytes>::nil())
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
    Ok(f45(v0.clone(), List::cons(f179(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f109(v0)?)))))
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
            f98(f122(f163(f79(v1, f8(Str::lit("defers"))))), f103(v2))
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
    let v3: List<(Bytes, Bytes)> = f133(f180(v2.clone()), f181(f182(v0, v1.clone(), v2)?));
    f183(f125(f184(v1))?, v3.clone(), {
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
            if if f185(v2.clone(), f8(Str::lit("subject"))) {
                f186(v0.clone(), v2.clone())
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
            let m419 = f187(f81(f53(v0.clone(), v2.clone()))?, f72(v2, f178()))?;
            match m419 {
                None => {
                    f108(v0, v3)
                }
                Some(v4) => {
                    Ok(f188(v4, f108(v0, v3)?))
                }
            }
        }
    }
}

pub fn f109(v0: Adt1) -> R<List<Bytes>> {
    Ok(f98(f125(f189(f177(), f127(match v0.clone() {
        Adt1::C0(_, h420, _) => {
            h420
        }
    }, f177())))?, f125(f189(f178(), f127(match v0 {
        Adt1::C0(_, h421, _) => {
            h421
        }
    }, f178())))?))
}

pub fn f110(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f111() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f179(), List::<Bytes>::nil())))))))
}

pub fn f112(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m422 = v1;
    match m422.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f56(v0.clone(), v2.clone()) {
                f190(v2.clone(), f8(Str::lit(".json")))
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
            f98(f191({
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
    f192(f193(v0, f116())?)
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
                        let m429 = f194(f53(v0.clone(), v2.clone()), f195())?;
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
    let v2: Adt17 = f196(v1.clone())?;
    let v3: List<Bytes> = f197(v1.clone());
    let v4: List<Bytes> = f121(match v0.clone() {
        Adt16::C0(_, _, h433, _, _) => {
            h433
        }
    }, f8(Str::lit("path")));
    let m435 = f198(v2, List::<u64>::nil(), f199(v1.clone(), match v0.clone() {
        Adt16::C0(_, h434, _, _, _) => {
            h434
        }
    }, 0u64)?);
    match m435 {
        None => {
            if f201(f202(match v0.clone() {
                Adt16::C0(_, _, _, _, h437) => {
                    h437
                }
            }, f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h438) => {
                    h438
                }
            }, v4.clone()))?, f202(v4.clone(), f98(match v0.clone() {
                Adt16::C0(_, _, _, _, h439) => {
                    h439
                }
            }, v4.clone()))?, f203(match v0.clone() {
                Adt16::C0(_, _, h440, _, _) => {
                    h440
                }
            })) {
                let m444 = f204(f202(match v0.clone() {
                    Adt16::C0(_, _, _, h441, _) => {
                        h441
                    }
                }, f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h442, _) => {
                        h442
                    }
                }))?, f202(v3.clone(), f98(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, h443, _) => {
                        h443
                    }
                }))?);
                match m444 {
                    None => {
                        let m447 = f205(f121(match v0 {
                            Adt16::C0(_, h446, _, _, _) => {
                                h446
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m447 {
                            None => {
                                let m448 = f205(v4, List::<Bytes>::nil());
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f200(f98(v3, match v0 {
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
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f200(match v1 {
                Adt15::C0(_, _, _, _, h436) => {
                    h436
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f120(v0: Adt1) -> R<List<Bytes>> {
    let m449 = f206(f53(v0, f179()))?;
    match m449.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f207(v2))
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
    Ok(f208(f93(Fn1::F209, List::<Bytes>::nil(), f210(v0)?)))
}

pub fn f126(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m457 = v1;
    match m457.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f190(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f211(v0.clone(), v2), f126(v0, v3))
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
                        if f212(v0.clone(), v1.clone(), v3.clone())? {
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
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f213(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f28(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f213(v1, List::<Bytes>::nil()), v2))))
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
    }, f214())) {
        Ok(Adt11::C1(List::cons(f214(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f130(f125(f126(f214(), f127(match v0.clone() {
            Adt1::C0(_, h473, _) => {
                h473
            }
        }, f214())))?, v1.clone());
        let m474 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m474.uncons() {
            None => {
                let v5: List<Bytes> = f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m475 = f45(v0.clone(), f215(v0.clone(), v2.clone(), v5.clone())?);
                match m475.uncons() {
                    None => {
                        let m476 = f216(v0.clone(), v2, v5)?;
                        match m476 {
                            None => {
                                f217(v0, v1)
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
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f218(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f219(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op > target/attempt.txt 2>&1; then\u{a}                tee -a target/operation.txt < target/attempt.txt\u{a}                last=$op\u{a}              else\u{a}                cat target/attempt.txt\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f138() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f219(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
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
    let v4: List<List<Bytes>> = f220(f206(f53(v0, f179()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f221(v4.clone(), f207(v4), f182(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f106(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f142(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f222(f100(v0.clone())?, f108(v0.clone(), f109(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f143(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f223(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f224(f82(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f144() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f145(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m479 = f45(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m479.uncons() {
        None => {
            if f225(f81(f53(v0, v1))?)? {
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
            if f40(f190(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f226(v5)?
                } else {
                    f227(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f211(f228(), f229(v1.clone())?), f53(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f211(f228(), f229(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f230()
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
            let m482 = f231(f81(f53(v0.clone(), v1.clone()))?)?;
            match m482 {
                None => {
                    Ok(Adt11::C4(f232()))
                }
                Some(v4) => {
                    let m483 = f233(f81(f53(v0, v1))?)?;
                    match m483 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f95(f38(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f234(v5)))
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
            let m485 = f235(f81(f53(v0, v1))?)?;
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
            let v5: Bytes = f211(if v2 {
                f178()
            } else {
                f177()
            }, f229(v1.clone())?);
            if f40(f190(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m487 = f187(f81(f53(v0.clone(), v1.clone()))?, v2)?;
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
            let m492 = f236(v4.clone(), f8(Str::lit("requirement")))?;
            match m492 {
                None => {
                    Ok(Adt11::C4(f230()))
                }
                Some(v5) => {
                    let m493 = f236(v4, f8(Str::lit("admission")))?;
                    match m493 {
                        None => {
                            Ok(Adt11::C4(f230()))
                        }
                        Some(v6) => {
                            let v7: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt13> = f82(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f98(f121(v5.clone(), f8(Str::lit("id"))), f121(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f125(f98(f121(v7.clone(), f8(Str::lit("id"))), f121(v8.clone(), f8(Str::lit("construct")))))?;
                            let m494 = f237(f104(v6.clone()), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m494 {
                                None => {
                                    let a496 = {
                                        let a495 = v9.clone();
                                        length_list(a495)
                                    };
                                    let a497 = 0u64;
                                    if nat_eq(a496, a497) {
                                        Ok(Adt11::C4(f230()))
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
                                            Ok(Adt11::C3(List::cons(f239(f8(Str::lit("model/requirements.json")), f188(v7, v5))?, List::cons(f239(f8(Str::lit("model/admissions.json")), f188(v8, v6))?, List::cons(f240(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f95(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f238(v11))
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
            let m503 = f241(v0.clone(), v1.clone())?;
            match m503 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m504 = f242(v4.clone(), f8(Str::lit("schema")));
                    match m504 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f243(v0, f244(v5.clone())?, v5, f245(v4.clone(), f8(Str::lit("name"))), f246(f247(f242(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
                    if f40(f28(f47(v4.clone(), f8(Str::lit("schema"))), f248())) {
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
                        let v9: Bytes = f211(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f211(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m508 = v8;
                        match m508 {
                            None => {
                                f249(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f40(f28(f47(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f249(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f211(v9, f229(f47(v7.clone(), f8(Str::lit("url"))))?);
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
                                                if f28(f250(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f249(v0, v4, v5, v10, Some((v12, v11)))
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
            let v5: List<List<Bytes>> = f206(v4.clone())?;
            if f40(f84(f251(v5.clone()), f252())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f253(Str::lit("id"), v6.clone())?, List::cons(f253(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f253(Str::lit("artifact"), f179())?, List::cons(f253(Str::lit("checksum"), f250(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f179(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f254(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f240(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f95(f207(f220(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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
                    if f255(f163(Some(v4))) {
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
            let m515 = f241(v0.clone(), v1.clone())?;
            match m515 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f243(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
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
            let m517 = f256(f81(f53(v0.clone(), v1))?)?;
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
                        Ok(Adt11::C3(List::cons(f239(f132(), f188(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f95(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f157(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt11> {
    let m525 = v2;
    match m525.uncons() {
        None => {
            Ok(Adt11::C4(f66()))
        }
        Some((v3, v4)) => {
            let m526 = v4;
            match m526.uncons() {
                None => {
                    Ok(Adt11::C4(f66()))
                }
                Some((v5, v6)) => {
                    let m527 = v6.clone();
                    match m527.uncons() {
                        None => {
                            Ok(Adt11::C4(f66()))
                        }
                        Some((_, _)) => {
                            let m528 = f45(v0.clone(), List::cons(v1.clone(), List::cons(f257(), List::<Bytes>::nil())));
                            match m528.uncons() {
                                None => {
                                    let v11: Bytes = f95(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f53(v0.clone(), v1);
                                    let v13: List<Adt13> = f82(v0, f257())?;
                                    if f258(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt11::C3(List::cons(f239(f257(), f188(v13, List::cons(f259(v11.clone(), v12, v5, v3), List::<Adt13>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                }
                                Some((v9, v10)) => {
                                    Ok(Adt11::C0(List::cons(v9, v10)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f158(v0: Bytes, p582: u64, p583: u64) -> R<bool> {
    let m581 = (p582, p583);
    {
        let mut m581 = m581;
        loop {
            let step = {
                let (v1, v2) = m581;
                let m529 = v2;
                if m529 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m529 - 1;
                    let m532 = {
                        let a530 = v0.clone();
                        let a531 = v1;
                        index_bytes(a530, a531)
                    };
                    match m532 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f260({
                                let v5: u8 = v4;
                                let a577 = {
                                    let a553 = {
                                        let a541 = {
                                            let a535 = {
                                                let a533 = v5;
                                                let a534 = 128u8;
                                                fixed_u8::bit_and(a533, a534)
                                            };
                                            let a536 = 128u8;
                                            if equal(a535, a536) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a542 = {
                                            let a539 = {
                                                let a537 = v5;
                                                let a538 = 64u8;
                                                fixed_u8::bit_and(a537, a538)
                                            };
                                            let a540 = 64u8;
                                            if equal(a539, a540) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a541, a542)?
                                    };
                                    let a554 = {
                                        let a551 = {
                                            let a545 = {
                                                let a543 = v5;
                                                let a544 = 32u8;
                                                fixed_u8::bit_and(a543, a544)
                                            };
                                            let a546 = 32u8;
                                            if equal(a545, a546) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a552 = {
                                            let a549 = {
                                                let a547 = v5;
                                                let a548 = 16u8;
                                                fixed_u8::bit_and(a547, a548)
                                            };
                                            let a550 = 16u8;
                                            if equal(a549, a550) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a551, a552)?
                                    };
                                    nat_add(a553, a554)?
                                };
                                let a578 = {
                                    let a575 = {
                                        let a563 = {
                                            let a557 = {
                                                let a555 = v5;
                                                let a556 = 8u8;
                                                fixed_u8::bit_and(a555, a556)
                                            };
                                            let a558 = 8u8;
                                            if equal(a557, a558) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a564 = {
                                            let a561 = {
                                                let a559 = v5;
                                                let a560 = 4u8;
                                                fixed_u8::bit_and(a559, a560)
                                            };
                                            let a562 = 4u8;
                                            if equal(a561, a562) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a563, a564)?
                                    };
                                    let a576 = {
                                        let a573 = {
                                            let a567 = {
                                                let a565 = v5;
                                                let a566 = 2u8;
                                                fixed_u8::bit_and(a565, a566)
                                            };
                                            let a568 = 2u8;
                                            if equal(a567, a568) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a574 = {
                                            let a571 = {
                                                let a569 = v5;
                                                let a570 = 1u8;
                                                fixed_u8::bit_and(a569, a570)
                                            };
                                            let a572 = 1u8;
                                            if equal(a571, a572) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a573, a574)?
                                    };
                                    nat_add(a575, a576)?
                                };
                                nat_add(a577, a578)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a579 = v1;
                                    let a580 = 1u64;
                                    nat_add(a579, a580)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m581 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: Bytes) -> R<Option<Adt18>> {
    let v1: Bytes = f261(v0)?;
    let m587 = f262(v1.clone(), 0u64, {
        let a585 = {
            let a584 = v1.clone();
            length_bytes(a584)
        };
        let a586 = 2u64;
        nat_add(a585, a586)?
    }, 0u64)?;
    match m587 {
        None => {
            Ok(None::<Adt18>)
        }
        Some(v2) => {
            let a590 = {
                let (_, h588) = v2;
                h588
            };
            let a591 = {
                let a589 = v1;
                length_bytes(a589)
            };
            if nat_eq(a590, a591) {
                Ok(Some({
                    let (h592, _) = v2;
                    h592
                }))
            } else {
                Ok(None::<Adt18>)
            }
        }
    }
}

pub fn f160(v0: Adt18) -> Option<Adt13> {
    let m593 = v0;
    match m593 {
        Adt18::C0 => {
            Some(Adt13::C3(std::rc::Rc::new(Adt13::C5)))
        }
        Adt18::C1(_, _) => {
            None::<Adt13>
        }
        Adt18::C2(v3) => {
            Some(Adt13::C0(v3))
        }
        Adt18::C3(v4, v5) => {
            Some(Adt13::C1(v4, v5))
        }
        Adt18::C4(_, _, _) => {
            None::<Adt13>
        }
        Adt18::C5(v9) => {
            Some(Adt13::C2(v9))
        }
        Adt18::C6(r600) => {
            let v10 = (*r600).clone();
            let m594 = f160(v10);
            match m594 {
                None => {
                    None::<Adt13>
                }
                Some(v11) => {
                    Some(Adt13::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt18::C7(r601) => {
            let v12 = (*r601).clone();
            let m595 = f160(v12);
            match m595 {
                None => {
                    None::<Adt13>
                }
                Some(v13) => {
                    Some(Adt13::C4(std::rc::Rc::new(v13)))
                }
            }
        }
        Adt18::C8 => {
            Some(Adt13::C5)
        }
        Adt18::C9(r602, r603) => {
            let v14 = (*r602).clone();
            let v15 = (*r603).clone();
            let m596 = f160(v14);
            match m596 {
                None => {
                    None::<Adt13>
                }
                Some(v16) => {
                    let m597 = f160(v15);
                    match m597 {
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
        Adt18::C10(v18, r604, r605) => {
            let v19 = (*r604).clone();
            let v20 = (*r605).clone();
            let m598 = f160(v19);
            match m598 {
                None => {
                    None::<Adt13>
                }
                Some(v21) => {
                    let m599 = f160(v20);
                    match m599 {
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

pub fn f161(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), v1) {
        let m606 = f33(v0, 34u8)?;
        match m606.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m607 = v3;
                match m607.uncons() {
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

pub fn f162(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f263(v0, v1)
}

pub fn f163(v0: Option<Adt13>) -> List<Adt13> {
    let m608 = v0;
    match m608 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            f264(v1)
        }
    }
}

pub fn f164(v0: Adt13, v1: Bytes) -> Adt13 {
    let m609 = v0;
    match m609 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r610) => {
            let v10 = (*r610).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r611) => {
            let v5 = (*r611).clone();
            Adt13::C4(std::rc::Rc::new(f164(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r612, r613) => {
            let v11 = (*r612).clone();
            let v12 = (*r613).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r614, r615) => {
            let v3 = (*r614).clone();
            let v4 = (*r615).clone();
            if f28(v2.clone(), f8(Str::lit("std"))) {
                Adt13::C7(v2, std::rc::Rc::new(f265(v3, v1.clone())), std::rc::Rc::new(f164(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f164(v4, v1)))
            }
        }
    }
}

pub fn f165(v0: Adt13) -> R<Bytes> {
    let m616 = v0;
    match m616 {
        Adt13::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt13::C1(v2, v3) => {
            f266(v2, v3)
        }
        Adt13::C2(v4) => {
            f267(v4)
        }
        Adt13::C3(r617) => {
            let v5 = (*r617).clone();
            Ok(f268(f269(v5)?))
        }
        Adt13::C4(r618) => {
            let v6 = (*r618).clone();
            f270(f271(v6)?)
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

pub fn f166(v0: Adt1, p622: List<Adt12>) -> R<Option<Adt12>> {
    let m621 = p622;
    {
        let mut m621 = m621;
        loop {
            let step = {
                let v1 = m621;
                let m619 = v1;
                match m619.uncons() {
                    None => {
                        Err::<List<Adt12>, R<Option<Adt12>>>(Ok(None::<Adt12>))
                    }
                    Some((v2, v3)) => {
                        if f41(f167(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h620) => {
                                h620
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
                Ok(next) => { m621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Adt12) -> R<Bytes> {
    Ok(f211(f272(), f229(f273(v0))?))
}

pub fn f168(v0: Adt12, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f273(v0), List::<Bytes>::nil())))))
}

pub fn f169() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f170() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f171(v0: List<Adt12>) -> R<List<Bytes>> {
    let m623 = v0;
    match m623.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f167(v1)?, f171(v2)?))
        }
    }
}

pub fn f172(v0: Adt1, v1: List<Adt12>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m624 = v1;
    match m624.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f57(f58(v5.clone(), f50(v2.clone()))), f274(v5.clone(), v2.clone(), v3.clone(), f250(f53(v0.clone(), f167(v5)?))?, v4.clone())?), f172(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f173(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f275(v0.clone())?;
    let v2: u64 = f276(v0.clone(), v1)?;
    Ok(f35(v0, v2, {
        let a625 = v1;
        let a626 = v2;
        nat_sub(a625, a626)
    }))
}

pub fn f174(v0: Bytes, v1: u64) -> Option<u8> {
    let a627 = v0;
    let a628 = v1;
    index_bytes(a627, a628)
}

pub fn f175(v0: Fn2, p632: Option<Bytes>, p633: List<Bytes>) -> Option<Bytes> {
    let m631 = (p632, p633);
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v1, v2) = m631;
                let m629 = v2;
                match m629.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c630: Fn2 = v0.clone();
                            c630.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m634 = v1;
    match m634 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a637 = {
                    let a635 = v3;
                    let a636 = v0;
                    append_bytes(a635, a636)
                };
                let a638 = v2;
                append_bytes(a637, a638)
            })
        }
    }
}

pub fn f177() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f178() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f179() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f180(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m639 = v0;
    match m639.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f47(v1.clone(), f8(Str::lit("subject"))), f47(v1, f8(Str::lit("object")))), f180(v2))
        }
    }
}

pub fn f181(v0: List<Adt19>) -> List<(Bytes, Bytes)> {
    let m640 = v0;
    match m640.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt19::C0(h641, _, _) => {
                    h641
                }
            }, match v1 {
                Adt19::C0(_, _, h642) => {
                    h642
                }
            }), f181(v2))
        }
    }
}

pub fn f182(v0: Adt14, v1: List<Adt13>, v2: List<Adt13>) -> R<List<Adt19>> {
    let v3: List<(Bytes, Bytes)> = f277(v1);
    let v4: List<(Bytes, Bytes)> = f278(v0, v2);
    f279(v3.clone(), f133(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f183(p649: List<Bytes>, v1: List<(Bytes, Bytes)>, p650: u64) -> R<List<Bytes>> {
    let m648 = (p649, p650);
    {
        let mut m648 = m648;
        loop {
            let step = {
                let (v0, v2) = m648;
                let m643 = v2;
                if m643 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m643 - 1;
                    let v4: List<Bytes> = f125(f98(v0.clone(), f280(v0.clone(), v1.clone())))?;
                    let a646 = {
                        let a644 = v4.clone();
                        length_list(a644)
                    };
                    let a647 = {
                        let a645 = v0.clone();
                        length_list(a645)
                    };
                    if nat_eq(a646, a647) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m648 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: List<Adt13>) -> List<Bytes> {
    let m651 = v0;
    match m651.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m652 = f79(v1, f8(Str::lit("std")));
            match m652 {
                None => {
                    f184(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3.clone(), f8(Str::lit("authority"))), if f185(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f47(v3, f8(Str::lit("row"))), f184(v2))
                    } else {
                        f184(v2)
                    })
                }
            }
        }
    }
}

pub fn f185(v0: Adt13, v1: Bytes) -> bool {
    f40({
        let a654 = {
            let a653 = f47(v0, v1);
            length_bytes(a653)
        };
        let a655 = 0u64;
        nat_eq(a654, a655)
    })
}

pub fn f186(v0: Adt14, v1: Adt13) -> bool {
    if f281(v0.clone(), v1.clone(), f47(v1.clone(), f8(Str::lit("subject")))) {
        f281(v0, v1.clone(), f47(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f187(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt13>>> {
    let m656 = v0;
    match m656.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f173(v2)?;
            let a658 = {
                let a657 = v4.clone();
                length_bytes(a657)
            };
            let a659 = 0u64;
            if nat_eq(a658, a659) {
                f187(v3, v1)
            } else {
                let m660 = f77(v4)?;
                match m660 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f282(v5.clone())
                        } else {
                            f283(v5.clone())
                        } {
                            let m661 = f187(v3, v1)?;
                            match m661 {
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

pub fn f188(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m662 = v0;
    match m662.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f188(v3, v1))
        }
    }
}

pub fn f189(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m663 = v1;
    match m663.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f190(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f211(v0.clone(), v2), f189(v0, v3))
            } else {
                f189(v0, v3)
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a664 = v0.clone();
        length_bytes(a664)
    };
    let v3: u64 = {
        let a665 = v1.clone();
        length_bytes(a665)
    };
    let a666 = v3;
    let a667 = v2;
    if nat_le(a666, a667) {
        let m673 = {
            let a670 = v0;
            let a671 = {
                let a668 = v2;
                let a669 = v3;
                nat_sub(a668, a669)
            };
            let a672 = v3;
            slice_bytes(a670, a671, a672)
        };
        match m673 {
            None => {
                false
            }
            Some(v4) => {
                let a674 = v4;
                let a675 = v1;
                equal(a674, a675)
            }
        }
    } else {
        false
    }
}

pub fn f191(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m676 = v1;
    match m676.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f284(v3.clone())
            } else {
                f40(f284(v3.clone()))
            } {
                List::cons(f211(v0.clone(), v3), f191(v0, v4, v2))
            } else {
                f191(v0, v4, v2)
            }
        }
    }
}

pub fn f192(p679: List<Bytes>) -> R<Option<Bytes>> {
    let m678 = p679;
    {
        let mut m678 = m678;
        loop {
            let step = {
                let v0 = m678;
                let m677 = v0;
                match m677.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f285(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f173(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
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

pub fn f193(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m680 = v1;
    match m680.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f98(f81(f53(v0.clone(), v2))?, f193(v0, v3)?))
        }
    }
}

pub fn f194(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a681 = v0.clone();
        length_bytes(a681)
    };
    let v3: u64 = {
        let a682 = v1.clone();
        length_bytes(a682)
    };
    let v4: u64 = {
        let (h683, _) = f286(Fn3::F287(v1.clone(), v3, v0.clone(), v2), {
            let a684 = v2;
            let a685 = 1u64;
            nat_add(a684, a685)?
        }, 0u64)?;
        h683
    };
    if f288(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f195() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f196(v0: Adt15) -> R<Adt17> {
    Ok(Adt17::C0(f289(match v0.clone() {
        Adt15::C0(h686, _, _, _, _) => {
            h686
        }
    }, 0u64)?, f289(match v0.clone() {
        Adt15::C0(_, h687, _, _, _) => {
            h687
        }
    }, 0u64)?, f289(match v0.clone() {
        Adt15::C0(_, _, h688, _, _) => {
            h688
        }
    }, 0u64)?, f289(match v0 {
        Adt15::C0(_, _, _, h689, _) => {
            h689
        }
    }, 0u64)?))
}

pub fn f197(v0: Adt15) -> List<Bytes> {
    f98(match v0.clone() {
        Adt15::C0(h690, _, _, _, _) => {
            h690
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, h691, _, _) => {
            h691
        }
    }, f98(match v0.clone() {
        Adt15::C0(_, _, _, h692, _) => {
            h692
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h693) => {
            h693
        }
    })))
}

pub fn f198(v0: Adt17, p699: List<u64>, p700: List<Adt20>) -> Option<u64> {
    let m698 = (p699, p700);
    {
        let mut m698 = m698;
        loop {
            let step = {
                let (v1, v2) = m698;
                let m694 = v2;
                match m694.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f290(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h695) => {
                                h695
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h696, _) => {
                                    h696
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h697, _) => {
                                    h697
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: Adt15, v1: List<Adt13>, v2: u64) -> R<List<Adt20>> {
    let m701 = v1;
    match m701.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f291(v0.clone(), v3)?), f199(v0, v4, {
                let a702 = v2;
                let a703 = 1u64;
                nat_add(a702, a703)?
            })?))
        }
    }
}

pub fn f200(p707: List<Bytes>, p708: u64) -> Bytes {
    let m706 = (p707, p708);
    {
        let mut m706 = m706;
        loop {
            let step = {
                let (v0, v1) = m706;
                let m704 = v0;
                match m704.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m705 = v1;
                        if m705 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m705 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
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

pub fn f201(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f292(v0.clone(), v1.clone()) {
        if f292(v1, v0) {
            f293(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f202(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m709 = v0;
    match m709.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f294(v2, v1.clone())?, f202(v3, v1)?))
        }
    }
}

pub fn f203(v0: List<Adt13>) -> List<u64> {
    let m710 = v0;
    match m710.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f295(v1), f203(v2))
        }
    }
}

pub fn f204(p713: List<u64>, v1: List<u64>) -> Option<u64> {
    let m712 = p713;
    {
        let mut m712 = m712;
        loop {
            let step = {
                let v0 = m712;
                let m711 = v0;
                match m711.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f296(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m712 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(p716: List<Bytes>, p717: List<Bytes>) -> Option<Bytes> {
    let m715 = (p716, p717);
    {
        let mut m715 = m715;
        loop {
            let step = {
                let (v0, v1) = m715;
                let m714 = v0;
                match m714.uncons() {
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
                Ok(next) => { m715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt23 = f297(v0.clone(), 0u64, {
        let a718 = v0;
        length_bytes(a718)
    }, Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt23 = {
        let a721 = {
            let a720 = match v1.clone() {
                Adt23::C0(_, _, h719, _, _) => {
                    h719
                }
            };
            length_bytes(a720)
        };
        let a722 = 0u64;
        if if nat_eq(a721, a722) {
            let m724 = match v1.clone() {
                Adt23::C0(_, _, _, h723, _) => {
                    h723
                }
            };
            match m724.uncons() {
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
            f298(v1)
        }
    };
    Ok(f299(match v4 {
        Adt23::C0(_, _, _, _, h725) => {
            h725
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f207(v0: List<List<Bytes>>) -> List<Bytes> {
    let m726 = v0;
    match m726.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m727 = v1;
            match m727.uncons() {
                None => {
                    f207(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f207(v2))
                }
            }
        }
    }
}

pub fn f208(v0: List<Bytes>) -> List<Bytes> {
    f93(Fn1::F300, List::<Bytes>::nil(), v0)
}

pub fn f209(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f301(v0, v1)
}

pub fn f210(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f302(f303(f304(v0))?))
}

pub fn f211(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f212(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m728 = f46(v0, v2)?;
    match m728 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f305(v1, f47(v3.clone(), f8(Str::lit("id"))))?;
            let m729 = v4.clone();
            match m729.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m731 = f78(f79({
                        let m730 = f79(v3.clone(), f8(Str::lit("source")));
                        match m730 {
                            None => {
                                Adt13::C5
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
                            Ok(f40(f41(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f306(f307(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f213(p734: List<Bytes>, p735: List<Bytes>) -> List<Bytes> {
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

pub fn f214() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f215(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m736 = v1;
    match m736.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt18 = f308(v0.clone(), v3)?;
            if f41(f309(f245(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f310(v5)?, f215(v0, v4, v2)?))
            } else {
                f215(v0, v4, v2)
            }
        }
    }
}

pub fn f216(v0: Adt1, p740: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
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
                        let v5: Adt18 = f308(v0.clone(), v3.clone())?;
                        if if f41(f309(f245(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f28(f244({
                                let m738 = f242(v5.clone(), f8(Str::lit("schema")));
                                match m738 {
                                    None => {
                                        Adt18::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f53(v0.clone(), f310(v5)?))
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

pub fn f217(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f40(f123(match v0.clone() {
        Adt1::C0(_, h741, _) => {
            h741
        }
    }, f228())) {
        Ok(Adt11::C1(List::cons(f228(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f130(f125(f311(f127(match v0.clone() {
            Adt1::C0(_, h742, _) => {
                h742
            }
        }, f228())))?, v1);
        let m743 = f45(v0.clone(), List::cons(f132(), List::cons(f179(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m743.uncons() {
            None => {
                let v5: (bool, Bytes) = f312(v0.clone(), v2)?;
                let (h744, _) = v5;
                if h744 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h745) = v5;
                        h745
                    }))
                } else {
                    let m746 = f97(v0.clone());
                    match m746.uncons() {
                        None => {
                            let m747 = f99(v0.clone())?;
                            match m747.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h748) = v5;
                                        h748
                                    }, List::cons(f7(f125(f313(f100(v0.clone())?, f108(v0.clone(), f109(v0.clone())?)?))?), List::cons(f314(v0)?, List::<Bytes>::nil()))))))
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

pub fn f218(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f315(f82(v0, f132())?)?)))
}

pub fn f219() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f220(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m749 = v0;
    match m749.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f221(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt19>, v3: List<Adt13>, v4: List<Bytes>) -> List<Bytes> {
    let m750 = v1;
    match m750.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f98(f316(v5.clone(), v2.clone()), f317(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f318(v0.clone(), v5.clone()), List::cons(if f41(v5, v4.clone()) {
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
                    f95(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f221(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f222(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m754 = v1;
    match m754.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f185(v2.clone(), f8(Str::lit("domain"))) {
                f319(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f47(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m755 = f79(v2.clone(), f8(Str::lit("role")));
                    match m755 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f47(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f47(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f320(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m756 = f79(v2.clone(), f8(Str::lit("origin")));
                    match m756 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f47(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m757 = f79(v2, f8(Str::lit("evidence")));
                    match m757 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f47(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f222(v0, v3))
            } else {
                f222(v0, v3)
            }
        }
    }
}

pub fn f223(v0: List<Adt13>) -> List<Bytes> {
    let m758 = v0;
    match m758.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f47(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f47(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f47(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f223(v2))
        }
    }
}

pub fn f224(v0: List<Adt13>) -> List<Bytes> {
    let m759 = v0;
    match m759.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f47(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f321(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f224(v2))
        }
    }
}

pub fn f225(p765: List<Bytes>) -> R<bool> {
    let m764 = p765;
    {
        let mut m764 = m764;
        loop {
            let step = {
                let v0 = m764;
                let m760 = v0;
                match m760.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a762 = {
                            let a761 = f173(v1)?;
                            length_bytes(a761)
                        };
                        let a763 = 0u64;
                        if nat_eq(a762, a763) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m764 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: List<Bytes>) -> R<bool> {
    if f322(f256(v0.clone())?) {
        Ok(f40({
            let a767 = {
                let a766 = f323(f256(v0)?);
                length_list(a766)
            };
            let a768 = 0u64;
            nat_eq(a767, a768)
        }))
    } else {
        Ok(false)
    }
}

pub fn f227(v0: List<Bytes>) -> R<bool> {
    if f322(f236(v0.clone(), f8(Str::lit("requirement")))?) {
        if f322(f236(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f40({
                let a770 = {
                    let a769 = f188(f323(f236(v0.clone(), f8(Str::lit("requirement")))?), f323(f236(v0, f8(Str::lit("admission")))?));
                    length_list(a769)
                };
                let a771 = 0u64;
                nat_eq(a770, a771)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f228() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f229(v0: Bytes) -> R<Bytes> {
    Ok(f34(f33(v0, 47u8)?))
}

pub fn f230() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f231(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m772 = v0;
    match m772.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a774 = {
                let a773 = v3.clone();
                length_bytes(a773)
            };
            let a775 = 0u64;
            if nat_eq(a774, a775) {
                f231(v2)
            } else {
                let m776 = f77(v3.clone())?;
                match m776 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f47(v4.clone(), f8(Str::lit("id")));
                        if f324(f47(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m777 = f231(v2)?;
                            match m777 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f325(v5), {
                                        let a778 = v3;
                                        let a779 = f8(Str::lit("\u{a}"));
                                        append_bytes(a778, a779)
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

pub fn f232() -> Bytes {
    let a780 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a780)
}

pub fn f233(p785: List<Bytes>) -> R<Option<Bytes>> {
    let m784 = p785;
    {
        let mut m784 = m784;
        loop {
            let step = {
                let v0 = m784;
                let m781 = v0;
                match m781.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m782 = f77(f173(v1)?)?;
                        match m782 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m783 = f79(v3.clone(), f8(Str::lit("source")));
                                match m783 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f326(f47(v4.clone(), f8(Str::lit("redistribution"))), f47(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: Bytes) -> Bytes {
    let a790 = {
        let a786 = Str::lit("binding ");
        utf8_encode(a786)
    };
    let a791 = {
        let a788 = v0;
        let a789 = {
            let a787 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a787)
        };
        append_bytes(a788, a789)
    };
    append_bytes(a790, a791)
}

pub fn f235(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m792 = v0;
    match m792.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a794 = {
                let a793 = v3.clone();
                length_bytes(a793)
            };
            let a795 = 0u64;
            if nat_eq(a794, a795) {
                f235(v2)
            } else {
                let m796 = f159(v3.clone())?;
                match m796 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f327(v4.clone()) {
                            let m797 = f235(v2)?;
                            match m797 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f214(), List::cons(f8(Str::lit("/")), List::cons(f328(f245(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a798 = v3;
                                        let a799 = f8(Str::lit("\u{a}"));
                                        append_bytes(a798, a799)
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

pub fn f236(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt13>>> {
    let m800 = v0;
    match m800.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f173(v2)?;
            let a802 = {
                let a801 = v4.clone();
                length_bytes(a801)
            };
            let a803 = 0u64;
            if nat_eq(a802, a803) {
                f236(v3, v1)
            } else {
                let m804 = f77(v4)?;
                match m804 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f329(v5.clone());
                        let a806 = {
                            let a805 = v6.clone();
                            length_bytes(a805)
                        };
                        let a807 = 0u64;
                        if nat_eq(a806, a807) {
                            Ok(None::<List<Adt13>>)
                        } else {
                            let m808 = f236(v3, v1.clone())?;
                            match m808 {
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

pub fn f237(p811: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m810 = p811;
    {
        let mut m810 = m810;
        loop {
            let step = {
                let v0 = m810;
                let m809 = v0;
                match m809.uncons() {
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
                Ok(next) => { m810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f239(v0: Bytes, v1: List<Adt13>) -> R<(Bytes, Bytes)> {
    Ok((v0, f86(Adt13::C3(std::rc::Rc::new(f87(v1))))?))
}

pub fn f240(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f86(Adt13::C3(std::rc::Rc::new(f330(f125(f98(v1, f122(f82(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f241(v0: Adt1, v1: Bytes) -> R<Option<Adt18>> {
    f159(f53(v0, v1))
}

pub fn f242(p817: Adt18, v1: Bytes) -> Option<Adt18> {
    let m816 = p817;
    {
        let mut m816 = m816;
        loop {
            let step = {
                let v0 = m816;
                let m812 = v0;
                match m812 {
                    Adt18::C0 => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C1(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C2(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C3(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C4(_, _, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C5(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C6(_) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C7(r813) => {
                        let v5 = (*r813).clone();
                        Ok::<Adt18, Option<Adt18>>(v5)
                    }
                    Adt18::C8 => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C9(_, _) => {
                        Err::<Adt18, Option<Adt18>>(None::<Adt18>)
                    }
                    Adt18::C10(v2, r814, r815) => {
                        let v3 = (*r814).clone();
                        let v4 = (*r815).clone();
                        if f28(v2, v1.clone()) {
                            Err::<Adt18, Option<Adt18>>(Some(v3))
                        } else {
                            Ok::<Adt18, Option<Adt18>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m816 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: Adt1, v1: Bytes, v2: Adt18, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m818 = f45(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m818.uncons() {
        None => {
            let m820 = f237(f104(List::cons(Adt13::C4(std::rc::Rc::new({
                let m819 = f331(v4.clone());
                match m819 {
                    None => {
                        Adt13::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt13>::nil())), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m820 {
                None => {
                    f332(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f238(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f244(v0: Adt18) -> R<Bytes> {
    let a821 = f333(v0)?;
    let a822 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a821, a822))
}

pub fn f245(v0: Adt18, v1: Bytes) -> Bytes {
    let m823 = f334(f242(v0, v1));
    match m823 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f246(v0: List<Adt18>) -> List<Bytes> {
    let m824 = v0;
    match m824.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m825 = f334(Some(v1));
            match m825 {
                None => {
                    f246(v2)
                }
                Some(v3) => {
                    List::cons(v3, f246(v2))
                }
            }
        }
    }
}

pub fn f247(v0: Option<Adt18>) -> List<Adt18> {
    let m826 = v0;
    match m826 {
        None => {
            List::<Adt18>::nil()
        }
        Some(v1) => {
            f335(v1)
        }
    }
}

pub fn f248() -> Bytes {
    f336()
}

pub fn f249(v0: Adt1, v1: Adt13, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f254(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f337(v1, v4)?)?), List::cons(f240(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f250(v0: Bytes) -> R<Bytes> {
    Ok(f338(f339(v0)?))
}

pub fn f251(v0: List<List<Bytes>>) -> List<Bytes> {
    let m827 = v0;
    match m827.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f252() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f253(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f306(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f254(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f340(f129(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f341(f342(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f255(p830: List<Adt13>) -> bool {
    let m829 = p830;
    {
        let mut m829 = m829;
        loop {
            let step = {
                let v0 = m829;
                let m828 = v0;
                match m828.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f28(f47(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f41(f8(Str::lit("studio")), f121(f343(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: List<Bytes>) -> R<Option<List<Adt13>>> {
    let m831 = v0;
    match m831.uncons() {
        None => {
            Ok(Some(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f173(v1)?;
            let a833 = {
                let a832 = v3.clone();
                length_bytes(a832)
            };
            let a834 = 0u64;
            if nat_eq(a833, a834) {
                f256(v2)
            } else {
                let m835 = f77(v3)?;
                match m835 {
                    None => {
                        Ok(None::<List<Adt13>>)
                    }
                    Some(v4) => {
                        if if f185(v4.clone(), f8(Str::lit("agent"))) {
                            if f185(v4.clone(), f8(Str::lit("at"))) {
                                if f185(v4.clone(), f8(Str::lit("quote"))) {
                                    f344(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m836 = f256(v2)?;
                            match m836 {
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

pub fn f257() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f258(p839: List<Adt13>, v1: Bytes, v2: Bytes) -> bool {
    let m838 = p839;
    {
        let mut m838 = m838;
        loop {
            let step = {
                let v0 = m838;
                let m837 = v0;
                match m837.uncons() {
                    None => {
                        Err::<List<Adt13>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f345(f28(f47(v3.clone(), f8(Str::lit("observedProperty"))), v1.clone()), f28(f47(v3, f8(Str::lit("hasSimpleResult"))), v2.clone())) {
                            Err::<List<Adt13>, bool>(true)
                        } else {
                            Ok::<List<Adt13>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m838 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt13 {
    Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt13::C2(v2)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt13::C2(v0)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5))))))))))))
}

pub fn f260(v0: u64) -> bool {
    let a840 = v0;
    let a841 = 127u64;
    nat_le(a840, a841)
}

pub fn f261(v0: Bytes) -> R<Bytes> {
    f346(v0.clone(), 0u64, {
        let a842 = v0;
        length_bytes(a842)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f262(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt18, u64)>> {
    let m843 = v2;
    if m843 == 0 {
        Ok(None::<(Adt18, u64)>)
    } else {
        let v4 = m843 - 1;
        let v5: u64 = f347(v0.clone(), v1)?;
        let a844 = v3;
        let a845 = 1u64;
        if nat_eq(a844, a845) {
            let m846 = f262(v0.clone(), v1, v4, 0u64)?;
            match m846 {
                None => {
                    Ok(None::<(Adt18, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f347(v0.clone(), {
                        let (_, h847) = v6.clone();
                        h847
                    })?;
                    let a848 = v7;
                    let a849 = 44u64;
                    if nat_eq(a848, a849) {
                        let m853 = f262(v0, {
                            let a851 = {
                                let (_, h850) = v6;
                                h850
                            };
                            let a852 = 1u64;
                            nat_add(a851, a852)?
                        }, v4, 1u64)?;
                        match m853 {
                            None => {
                                Ok(None::<(Adt18, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt18::C9(std::rc::Rc::new({
                                    let (h854, _) = v6;
                                    h854
                                }), std::rc::Rc::new({
                                    let (h855, _) = v8;
                                    h855
                                })), {
                                    let (_, h856) = v8;
                                    h856
                                })))
                            }
                        }
                    } else {
                        let a857 = v7;
                        let a858 = 93u64;
                        if nat_eq(a857, a858) {
                            Ok(Some((Adt18::C9(std::rc::Rc::new({
                                let (h859, _) = v6;
                                h859
                            }), std::rc::Rc::new(Adt18::C8)), {
                                let a861 = {
                                    let (_, h860) = v6;
                                    h860
                                };
                                let a862 = 1u64;
                                nat_add(a861, a862)?
                            })))
                        } else {
                            Ok(None::<(Adt18, u64)>)
                        }
                    }
                }
            }
        } else {
            let a863 = v3;
            let a864 = 2u64;
            if nat_eq(a863, a864) {
                let m867 = f348(v0.clone(), {
                    let a865 = v1;
                    let a866 = 1u64;
                    nat_add(a865, a866)?
                })?;
                match m867 {
                    None => {
                        Ok(None::<(Adt18, u64)>)
                    }
                    Some(v9) => {
                        if f345({
                            let a868 = v5;
                            let a869 = 34u64;
                            nat_eq(a868, a869)
                        }, {
                            let a871 = f347(v0.clone(), {
                                let (h870, _) = v9.clone();
                                h870
                            })?;
                            let a872 = 58u64;
                            nat_eq(a871, a872)
                        }) {
                            let m876 = f262(v0.clone(), {
                                let a874 = {
                                    let (h873, _) = v9;
                                    h873
                                };
                                let a875 = 1u64;
                                nat_add(a874, a875)?
                            }, v4, 0u64)?;
                            match m876 {
                                None => {
                                    Ok(None::<(Adt18, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f347(v0.clone(), {
                                        let (_, h877) = v10.clone();
                                        h877
                                    })?;
                                    let a878 = v11;
                                    let a879 = 44u64;
                                    if nat_eq(a878, a879) {
                                        let m883 = f262(v0, {
                                            let a881 = {
                                                let (_, h880) = v10;
                                                h880
                                            };
                                            let a882 = 1u64;
                                            nat_add(a881, a882)?
                                        }, v4, 2u64)?;
                                        match m883 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt18::C10({
                                                    let (_, h884) = v9;
                                                    h884
                                                }, std::rc::Rc::new({
                                                    let (h885, _) = v10;
                                                    h885
                                                }), std::rc::Rc::new({
                                                    let (h886, _) = v12;
                                                    h886
                                                })), {
                                                    let (_, h887) = v12;
                                                    h887
                                                })))
                                            }
                                        }
                                    } else {
                                        let a888 = v11;
                                        let a889 = 125u64;
                                        if nat_eq(a888, a889) {
                                            Ok(Some((Adt18::C10({
                                                let (_, h890) = v9;
                                                h890
                                            }, std::rc::Rc::new({
                                                let (h891, _) = v10;
                                                h891
                                            }), std::rc::Rc::new(Adt18::C8)), {
                                                let a893 = {
                                                    let (_, h892) = v10;
                                                    h892
                                                };
                                                let a894 = 1u64;
                                                nat_add(a893, a894)?
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
                let a895 = v5;
                let a896 = 34u64;
                if nat_eq(a895, a896) {
                    let m899 = f348(v0, {
                        let a897 = v1;
                        let a898 = 1u64;
                        nat_add(a897, a898)?
                    })?;
                    match m899 {
                        None => {
                            Ok(None::<(Adt18, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt18::C5({
                                let (_, h900) = v13;
                                h900
                            }), {
                                let (h901, _) = v13;
                                h901
                            })))
                        }
                    }
                } else {
                    if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt18::C0, {
                            let a902 = v1;
                            let a903 = 4u64;
                            nat_add(a902, a903)?
                        })))
                    } else {
                        if f28(f35(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt18::C2(true), {
                                let a904 = v1;
                                let a905 = 4u64;
                                nat_add(a904, a905)?
                            })))
                        } else {
                            if f28(f35(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt18::C2(false), {
                                    let a906 = v1;
                                    let a907 = 5u64;
                                    nat_add(a906, a907)?
                                })))
                            } else {
                                let a908 = v5;
                                let a909 = 91u64;
                                if nat_eq(a908, a909) {
                                    let a912 = f347(v0.clone(), {
                                        let a910 = v1;
                                        let a911 = 1u64;
                                        nat_add(a910, a911)?
                                    })?;
                                    let a913 = 93u64;
                                    if nat_eq(a912, a913) {
                                        Ok(Some((Adt18::C6(std::rc::Rc::new(Adt18::C8)), {
                                            let a914 = v1;
                                            let a915 = 2u64;
                                            nat_add(a914, a915)?
                                        })))
                                    } else {
                                        let m918 = f262(v0, {
                                            let a916 = v1;
                                            let a917 = 1u64;
                                            nat_add(a916, a917)?
                                        }, v4, 1u64)?;
                                        match m918 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt18::C6(std::rc::Rc::new({
                                                    let (h919, _) = v14;
                                                    h919
                                                })), {
                                                    let (_, h920) = v14;
                                                    h920
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a921 = v5;
                                    let a922 = 123u64;
                                    if nat_eq(a921, a922) {
                                        let a925 = f347(v0.clone(), {
                                            let a923 = v1;
                                            let a924 = 1u64;
                                            nat_add(a923, a924)?
                                        })?;
                                        let a926 = 125u64;
                                        if nat_eq(a925, a926) {
                                            Ok(Some((Adt18::C7(std::rc::Rc::new(Adt18::C8)), {
                                                let a927 = v1;
                                                let a928 = 2u64;
                                                nat_add(a927, a928)?
                                            })))
                                        } else {
                                            let m931 = f262(v0, {
                                                let a929 = v1;
                                                let a930 = 1u64;
                                                nat_add(a929, a930)?
                                            }, v4, 2u64)?;
                                            match m931 {
                                                None => {
                                                    Ok(None::<(Adt18, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt18::C7(std::rc::Rc::new({
                                                        let (h932, _) = v15;
                                                        h932
                                                    })), {
                                                        let (_, h933) = v15;
                                                        h933
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m934 = f349(v0, v1)?;
                                        match m934 {
                                            None => {
                                                Ok(None::<(Adt18, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h935) = v16;
                                                    h935
                                                }, {
                                                    let (h936, _) = v16;
                                                    h936
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

pub fn f263(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h937, _) = v1.clone();
        h937
    };
    let v3: u64 = {
        let (h938, _) = {
            let (_, h939) = v1.clone();
            h939
        };
        h938
    };
    let v4: List<Bytes> = {
        let (_, h940) = {
            let (_, h941) = v1;
            h941
        };
        h940
    };
    let m942 = f174(v0.clone(), v2);
    match m942 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a943 = v5;
            let a944 = 10u8;
            if equal(a943, a944) {
                Ok(Some(({
                    let a945 = v2;
                    let a946 = 1u64;
                    nat_add(a945, a946)?
                }, ({
                    let a947 = v2;
                    let a948 = 1u64;
                    nat_add(a947, a948)?
                }, List::cons(f350(f35(v0, v3, {
                    let a949 = v2;
                    let a950 = v3;
                    nat_sub(a949, a950)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a951 = v2;
                    let a952 = 1u64;
                    nat_add(a951, a952)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f264(v0: Adt13) -> List<Adt13> {
    let m953 = v0;
    match m953 {
        Adt13::C0(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C1(_, _) => {
            List::<Adt13>::nil()
        }
        Adt13::C2(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C3(r954) => {
            let v3 = (*r954).clone();
            f264(v3)
        }
        Adt13::C4(_) => {
            List::<Adt13>::nil()
        }
        Adt13::C5 => {
            List::<Adt13>::nil()
        }
        Adt13::C6(r955, r956) => {
            let v1 = (*r955).clone();
            let v2 = (*r956).clone();
            List::cons(v1, f264(v2))
        }
        Adt13::C7(_, _, _) => {
            List::<Adt13>::nil()
        }
    }
}

pub fn f265(v0: Adt13, v1: Bytes) -> Adt13 {
    let m957 = v0;
    match m957 {
        Adt13::C0(v6) => {
            Adt13::C0(v6)
        }
        Adt13::C1(v7, v8) => {
            Adt13::C1(v7, v8)
        }
        Adt13::C2(v9) => {
            Adt13::C2(v9)
        }
        Adt13::C3(r958) => {
            let v10 = (*r958).clone();
            Adt13::C3(std::rc::Rc::new(v10))
        }
        Adt13::C4(r959) => {
            let v5 = (*r959).clone();
            Adt13::C4(std::rc::Rc::new(f265(v5, v1)))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r960, r961) => {
            let v11 = (*r960).clone();
            let v12 = (*r961).clone();
            Adt13::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt13::C7(v2, r962, r963) => {
            let v3 = (*r962).clone();
            let v4 = (*r963).clone();
            if f28(v2.clone(), f8(Str::lit("authority"))) {
                Adt13::C7(v2, std::rc::Rc::new(f351(v3, v1.clone())), std::rc::Rc::new(f265(v4, v1)))
            } else {
                Adt13::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f265(v4, v1)))
            }
        }
    }
}

pub fn f266(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a964 = 0u64;
        let a965 = v1;
        nat_lt(a964, a965)
    } else {
        false
    } {
        let a966 = Bytes::lit(&[0x2d]);
        let a967 = f352(v1)?;
        Ok(append_bytes(a966, a967))
    } else {
        f352(v1)
    }
}

pub fn f267(v0: Bytes) -> R<Bytes> {
    let a973 = {
        let a971 = Bytes::lit(&[0x22]);
        let a972 = {
            let (_, h968) = {
                let (h969, _) = f353(Fn4::F354(v0.clone()), {
                    let a970 = v0;
                    length_bytes(a970)
                }, (0u64, Bytes::lit(&[])))?;
                h969
            };
            h968
        };
        append_bytes(a971, a972)
    };
    let a974 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a973, a974))
}

pub fn f268(v0: List<Bytes>) -> Bytes {
    let a977 = {
        let a975 = Bytes::lit(&[0x5b]);
        let a976 = f95(v0, Bytes::lit(&[0x2c]));
        append_bytes(a975, a976)
    };
    let a978 = Bytes::lit(&[0x5d]);
    append_bytes(a977, a978)
}

pub fn f269(v0: Adt13) -> R<List<Bytes>> {
    let m979 = v0;
    match m979 {
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
        Adt13::C6(r980, r981) => {
            let v7 = (*r980).clone();
            let v8 = (*r981).clone();
            Ok(List::cons(f165(v7)?, f269(v8)?))
        }
        Adt13::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f270(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f355(Fn5::F356, List::<(Bytes, Bytes)>::nil(), f357(v0)?);
    let v2: List<Bytes> = f358(Fn6::F359, List::<Bytes>::nil(), v1)?;
    let a984 = {
        let a982 = Bytes::lit(&[0x7b]);
        let a983 = f95(v2, Bytes::lit(&[0x2c]));
        append_bytes(a982, a983)
    };
    let a985 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a984, a985))
}

pub fn f271(v0: Adt13) -> R<List<(Bytes, Bytes)>> {
    let m986 = v0;
    match m986 {
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
        Adt13::C7(v9, r987, r988) => {
            let v10 = (*r987).clone();
            let v11 = (*r988).clone();
            Ok(List::cons((v9, f165(v10)?), f271(v11)?))
        }
    }
}

pub fn f272() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f273(v0: Adt12) -> Bytes {
    let m989 = v0;
    match m989 {
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

pub fn f274(v0: Adt12, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f360(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f360(f8(Str::lit("id")), f58(v0.clone(), f50(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f360(f8(Str::lit("name")), f361(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f360(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f360(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f360(f8(Str::lit("statement")), f362(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f360(f8(Str::lit("media_type")), f363(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f360(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f360(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f360(f8(Str::lit("url")), f168(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f275(v0: Bytes) -> R<u64> {
    let (h990, _) = f286(Fn3::F364(v0.clone()), {
        let a991 = v0.clone();
        length_bytes(a991)
    }, {
        let a992 = v0;
        length_bytes(a992)
    })?;
    Ok(h990)
}

pub fn f276(v0: Bytes, v1: u64) -> R<u64> {
    let (h993, _) = f286(Fn3::F365(v1, v0), v1, 0u64)?;
    Ok(h993)
}

pub fn f277(v0: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m994 = v0;
    match m994.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m995 = f79(v1, f8(Str::lit("std")));
            match m995 {
                None => {
                    f277(v2)
                }
                Some(v3) => {
                    if f185(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f47(v3.clone(), f8(Str::lit("row"))), f47(v3, f8(Str::lit("authority")))), f277(v2))
                    } else {
                        f277(v2)
                    }
                }
            }
        }
    }
}

pub fn f278(v0: Adt14, v1: List<Adt13>) -> List<(Bytes, Bytes)> {
    let m996 = v1;
    match m996.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f47(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f47(v2, f8(Str::lit("object")));
            if if f41(v4.clone(), match v0.clone() {
                Adt14::C0(h997, _, _) => {
                    h997
                }
            }) {
                f41(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h998, _) => {
                        h998
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f278(v0, v3))
            } else {
                if if f41(v5.clone(), match v0.clone() {
                    Adt14::C0(h999, _, _) => {
                        h999
                    }
                }) {
                    f41(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1000, _) => {
                            h1000
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f278(v0, v3))
                } else {
                    f278(v0, v3)
                }
            }
        }
    }
}

pub fn f279(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt19>> {
    let m1001 = v0;
    match m1001.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v4, v5)) => {
            if if f366(v4.clone(), v2.clone()) {
                true
            } else {
                f366(v4.clone(), v3.clone())
            } {
                f279(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt19::C0({
                    let (h1002, _) = v4.clone();
                    h1002
                }, {
                    let a1005 = {
                        let a1004 = f125(f367({
                            let (h1003, _) = v4.clone();
                            h1003
                        }, v1.clone()))?;
                        length_list(a1004)
                    };
                    let a1006 = 1u64;
                    if nat_eq(a1005, a1006) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1007) = v4.clone();
                    h1007
                }), f279(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f280(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1008 = v1;
    match m1008.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41({
                let (h1009, _) = v2.clone();
                h1009
            }, v0.clone()) {
                List::cons({
                    let (_, h1010) = v2;
                    h1010
                }, f280(v0, v3))
            } else {
                if f41({
                    let (_, h1011) = v2;
                    h1011
                }, v0.clone()) {
                    List::cons({
                        let (h1012, _) = v2;
                        h1012
                    }, f280(v0, v3))
                } else {
                    f280(v0, v3)
                }
            }
        }
    }
}

pub fn f281(v0: Adt14, v1: Adt13, v2: Bytes) -> bool {
    if f41(v2.clone(), match v0.clone() {
        Adt14::C0(h1013, _, _) => {
            h1013
        }
    }) {
        true
    } else {
        if f41(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1014, _) => {
                h1014
            }
        }) {
            true
        } else {
            if f41(v2, match v0.clone() {
                Adt14::C0(_, _, h1015) => {
                    h1015
                }
            }) {
                true
            } else {
                if f185(v1.clone(), f8(Str::lit("scheme"))) {
                    f41(f47(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1016, _) => {
                            h1016
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f282(v0: Adt13) -> bool {
    if f185(v0.clone(), f8(Str::lit("domain"))) {
        if f185(v0.clone(), f8(Str::lit("row"))) {
            if f41(f47(v0.clone(), f8(Str::lit("relation"))), f368()) {
                if f369(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f369(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f369(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f369(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f283(v0: Adt13) -> bool {
    if f185(v0.clone(), f8(Str::lit("subject"))) {
        if f185(v0.clone(), f8(Str::lit("object"))) {
            if f185(v0.clone(), f8(Str::lit("evidence"))) {
                f41(f47(v0, f8(Str::lit("relation"))), f370())
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

pub fn f284(v0: Bytes) -> bool {
    f190(v0, f8(Str::lit(".lex.tex")))
}

pub fn f285(v0: Bytes) -> R<bool> {
    let v1: Bytes = f173(v0)?;
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
        let a1018 = {
            let a1017 = v1;
            length_bytes(a1017)
        };
        let a1019 = v2;
        nat_sub(a1018, a1019)
    });
    let v5: Bytes = if f72(v3.clone(), f8(Str::lit("("))) {
        let m1020 = f194(v3.clone(), f8(Str::lit(")")))?;
        match m1020 {
            None => {
                v3
            }
            Some(v4) => {
                f35(v3.clone(), {
                    let a1021 = v4;
                    let a1022 = 1u64;
                    nat_add(a1021, a1022)?
                }, {
                    let a1026 = {
                        let a1023 = v3;
                        length_bytes(a1023)
                    };
                    let a1027 = {
                        let a1024 = v4;
                        let a1025 = 1u64;
                        nat_add(a1024, a1025)?
                    };
                    nat_sub(a1026, a1027)
                })
            }
        }
    } else {
        v3
    };
    let a1028 = 0u64;
    let a1029 = v2;
    if nat_lt(a1028, a1029) {
        f371(v5)
    } else {
        Ok(false)
    }
}

pub fn f286(v0: Fn3, p1034: u64, p1035: u64) -> R<(u64, bool)> {
    let m1033 = (p1034, p1035);
    {
        let mut m1033 = m1033;
        loop {
            let step = {
                let (v1, v2) = m1033;
                let m1030 = v1;
                if m1030 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1030 - 1;
                    let m1032 = {
                        let c1031: Fn3 = v0.clone();
                        c1031.apply(v2)?
                    };
                    match m1032 {
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
                Ok(next) => { m1033 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f288(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1038 = {
            let a1036 = v4;
            let a1037 = v1;
            nat_add(a1036, a1037)?
        };
        let a1039 = v3;
        if nat_lt(a1038, a1039) {
            Ok(Some({
                let a1040 = v4;
                let a1041 = 1u64;
                nat_add(a1040, a1041)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f288(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1046 = {
        let a1043 = v0;
        let a1044 = v2;
        let a1045 = {
            let a1042 = v1.clone();
            length_bytes(a1042)
        };
        slice_bytes(a1043, a1044, a1045)
    };
    match m1046 {
        None => {
            false
        }
        Some(v3) => {
            let a1047 = v3;
            let a1048 = v1;
            equal(a1047, a1048)
        }
    }
}

pub fn f289(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1049 = v0;
    match m1049.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f289(v3, {
                let a1050 = v1;
                let a1051 = 1u64;
                nat_add(a1050, a1051)?
            })?))
        }
    }
}

pub fn f290(v0: Adt17, v1: List<u64>, v2: Adt21) -> bool {
    let m1052 = v2;
    match m1052 {
        Adt21::C0(v3, v4) => {
            if f296(v3, match v0.clone() {
                Adt17::C0(h1053, _, _, _) => {
                    h1053
                }
            }) {
                f372(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f296(v5, match v0 {
                Adt17::C0(_, _, h1054, _) => {
                    h1054
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f292(v7, v1) {
                f296(v8, match v0 {
                    Adt17::C0(_, _, _, h1055) => {
                        h1055
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f291(v0: Adt15, v1: Adt13) -> R<Adt21> {
    let m1056 = f79(v1.clone(), f8(Str::lit("std")));
    match m1056 {
        None => {
            let m1061 = f78(f79(v1.clone(), f8(Str::lit("req"))));
            match m1061 {
                None => {
                    let m1063 = f79(v1, f8(Str::lit("der")));
                    match m1063 {
                        None => {
                            Ok(f373(v0))
                        }
                        Some(v5) => {
                            let m1064 = f374(f47(v5.clone(), f8(Str::lit("rule"))));
                            match m1064 {
                                None => {
                                    Ok(f373(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f202(f122(f163(f79(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1065) => {
                                            h1065
                                        }
                                    })?, f294(f47(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1066, _) => {
                                            h1066
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f294(v4, match v0 {
                        Adt15::C0(_, _, h1062, _, _) => {
                            h1062
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1057 = f78(f79(v2.clone(), f8(Str::lit("row"))));
            match m1057 {
                None => {
                    Ok(Adt21::C0(f294(f47(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1058, _, _, _, _) => {
                            h1058
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f294(f47(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1059, _, _, _, _) => {
                            h1059
                        }
                    })?, Some(f294(v3, match v0 {
                        Adt15::C0(_, h1060, _, _, _) => {
                            h1060
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f292(p1069: List<u64>, v1: List<u64>) -> bool {
    let m1068 = p1069;
    {
        let mut m1068 = m1068;
        loop {
            let step = {
                let v0 = m1068;
                let m1067 = v0;
                match m1067.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f296(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f293(p1074: List<u64>) -> bool {
    let m1073 = p1074;
    {
        let mut m1073 = m1073;
        loop {
            let step = {
                let v0 = m1073;
                let m1070 = v0;
                match m1070.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1071 = v1;
                        let a1072 = 7u64;
                        if nat_le(a1071, a1072) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1073 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1075 = f375(v0, v1.clone(), 0u64)?;
    match m1075 {
        None => {
            let a1076 = v1;
            Ok(length_list(a1076))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f295(v0: Adt13) -> u64 {
    let m1077 = f376(f79(v0, f8(Str::lit("stratum"))));
    match m1077 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f296(v0: u64, p1082: List<u64>) -> bool {
    let m1081 = p1082;
    {
        let mut m1081 = m1081;
        loop {
            let step = {
                let v1 = m1081;
                let m1078 = v1;
                match m1078.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1079 = v2;
                        let a1080 = v0;
                        if nat_eq(a1079, a1080) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1081 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: Bytes, p1140: u64, p1141: u64, p1142: Adt23) -> R<Adt23> {
    let m1139 = (p1140, (p1141, p1142));
    {
        let mut m1139 = m1139;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1139;
                let m1083 = v2;
                if m1083 == 0 {
                    Err::<(u64, (u64, Adt23)), R<Adt23>>(Ok(v3))
                } else {
                    let v4 = m1083 - 1;
                    let v5: u64 = f377(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt23::C0(_, h1084, _, _, _) => {
                            h1084
                        }
                    } {
                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                            let a1085 = v1;
                            let a1086 = 1u64;
                            nat_add(a1085, a1086)?
                        }, (v4, Adt23::C0(true, false, match v3.clone() {
                            Adt23::C0(_, _, h1087, _, _) => {
                                h1087
                            }
                        }, match v3.clone() {
                            Adt23::C0(_, _, _, h1088, _) => {
                                h1088
                            }
                        }, match v3 {
                            Adt23::C0(_, _, _, _, h1089) => {
                                h1089
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt23::C0(h1090, _, _, _, _) => {
                                h1090
                            }
                        } {
                            let a1091 = v5;
                            let a1092 = 34u64;
                            if nat_eq(a1091, a1092) {
                                let a1095 = f377(v0.clone(), {
                                    let a1093 = v1;
                                    let a1094 = 1u64;
                                    nat_add(a1093, a1094)?
                                })?;
                                let a1096 = 34u64;
                                if nat_eq(a1095, a1096) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1097 = v1;
                                        let a1098 = 1u64;
                                        nat_add(a1097, a1098)?
                                    }, (v4, Adt23::C0(true, true, f378(match v3.clone() {
                                        Adt23::C0(_, _, h1099, _, _) => {
                                            h1099
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt23::C0(_, _, _, h1100, _) => {
                                            h1100
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1101) => {
                                            h1101
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1102 = v1;
                                        let a1103 = 1u64;
                                        nat_add(a1102, a1103)?
                                    }, (v4, Adt23::C0(false, false, match v3.clone() {
                                        Adt23::C0(_, _, h1104, _, _) => {
                                            h1104
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1105, _) => {
                                            h1105
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1106) => {
                                            h1106
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1107 = v1;
                                    let a1108 = 1u64;
                                    nat_add(a1107, a1108)?
                                }, (v4, Adt23::C0(true, false, f378(match v3.clone() {
                                    Adt23::C0(_, _, h1109, _, _) => {
                                        h1109
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt23::C0(_, _, _, h1110, _) => {
                                        h1110
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1111) => {
                                        h1111
                                    }
                                }))))
                            }
                        } else {
                            let a1112 = v5;
                            let a1113 = 34u64;
                            if nat_eq(a1112, a1113) {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1114 = v1;
                                    let a1115 = 1u64;
                                    nat_add(a1114, a1115)?
                                }, (v4, Adt23::C0(true, false, match v3.clone() {
                                    Adt23::C0(_, _, h1116, _, _) => {
                                        h1116
                                    }
                                }, match v3.clone() {
                                    Adt23::C0(_, _, _, h1117, _) => {
                                        h1117
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1118) => {
                                        h1118
                                    }
                                }))))
                            } else {
                                let a1119 = v5;
                                let a1120 = 44u64;
                                if nat_eq(a1119, a1120) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1121 = v1;
                                        let a1122 = 1u64;
                                        nat_add(a1121, a1122)?
                                    }, (v4, Adt23::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt23::C0(_, _, h1123, _, _) => {
                                            h1123
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1124, _) => {
                                            h1124
                                        }
                                    }), match v3 {
                                        Adt23::C0(_, _, _, _, h1125) => {
                                            h1125
                                        }
                                    }))))
                                } else {
                                    let a1126 = v5;
                                    let a1127 = 13u64;
                                    if nat_eq(a1126, a1127) {
                                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                            let a1128 = v1;
                                            let a1129 = 1u64;
                                            nat_add(a1128, a1129)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1130 = v5;
                                        let a1131 = 10u64;
                                        if nat_eq(a1130, a1131) {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1132 = v1;
                                                let a1133 = 1u64;
                                                nat_add(a1132, a1133)?
                                            }, (v4, f298(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1134 = v1;
                                                let a1135 = 1u64;
                                                nat_add(a1134, a1135)?
                                            }, (v4, Adt23::C0(false, false, f378(match v3.clone() {
                                                Adt23::C0(_, _, h1136, _, _) => {
                                                    h1136
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt23::C0(_, _, _, h1137, _) => {
                                                    h1137
                                                }
                                            }, match v3 {
                                                Adt23::C0(_, _, _, _, h1138) => {
                                                    h1138
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
                Ok(next) => { m1139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: Adt23) -> Adt23 {
    Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f379(List::cons(match v0.clone() {
        Adt23::C0(_, _, h1143, _, _) => {
            h1143
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1144, _) => {
            h1144
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt23::C0(_, _, _, _, h1145) => {
            h1145
        }
    }))
}

pub fn f299(p1148: List<List<Bytes>>, p1149: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1147 = (p1148, p1149);
    {
        let mut m1147 = m1147;
        loop {
            let step = {
                let (v0, v1) = m1147;
                let m1146 = v0;
                match m1146.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f300(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f301(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1150 = v0.clone();
    match m1150.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1151 = v2;
            let a1152 = v1.clone();
            if equal(a1151, a1152) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f302(v0: List<(Bytes, ())>) -> List<Bytes> {
    f208(f380(Fn7::F381, List::<Bytes>::nil(), v0))
}

pub fn f303(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f382(f383(Fn8::F384, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1157 = {
        let (h1153, _) = f385(Fn9::F386, {
            let a1155 = {
                let a1154 = v0;
                length_list(a1154)
            };
            let a1156 = 1u64;
            nat_add(a1155, a1156)?
        }, v1)?;
        h1153
    };
    match m1157.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f304(v0: List<Bytes>) -> List<(Bytes, ())> {
    f387(f388(Fn10::F389, List::<(Bytes, ())>::nil(), v0))
}

pub fn f305(v0: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1158 = f33(v1, 34u8)?;
    match m1158.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1159 = v3;
            match m1159.uncons() {
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

pub fn f306(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f391(v0.clone(), 0u64, {
        let a1160 = v0;
        length_bytes(a1160)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f307(v0: Adt13, v1: Bytes) -> Bytes {
    if f28(f392(f79(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f308(v0: Adt1, v1: Bytes) -> R<Adt18> {
    let m1161 = f241(v0, v1)?;
    match m1161 {
        None => {
            Ok(Adt18::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f309(v0: Bytes) -> R<Bytes> {
    let a1162 = f8(Str::lit("Catalog."));
    let a1163 = f328(v0)?;
    Ok(append_bytes(a1162, a1163))
}

pub fn f310(v0: Adt18) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f328(f245(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f311(v0: List<Bytes>) -> List<Bytes> {
    let m1164 = v0;
    match m1164.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f190(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f211(f228(), v1), f311(v2))
            } else {
                f311(v2)
            }
        }
    }
}

pub fn f312(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1165 = v1;
    match m1165.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f393(v0.clone(), v2)?;
            let (h1166, _) = v4.clone();
            if h1166 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f312(v0, v3)?;
                let (h1167, _) = v5.clone();
                if h1167 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1170 = {
                            let (_, h1168) = v4;
                            h1168
                        };
                        let a1171 = {
                            let (_, h1169) = v5;
                            h1169
                        };
                        append_bytes(a1170, a1171)
                    }))
                }
            }
        }
    }
}

pub fn f313(v0: Adt14, v1: List<Adt13>) -> List<Bytes> {
    let m1172 = v1;
    match m1172.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f185(v2.clone(), f8(Str::lit("subject"))) {
                if f186(v0.clone(), v2.clone()) {
                    f313(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f47(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f313(v0, v3))
                }
            } else {
                if f319(v0.clone(), v2.clone()) {
                    f313(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f47(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f320(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f313(v0, v3))
                }
            }
        }
    }
}

pub fn f314(v0: Adt1) -> R<Bytes> {
    let a1174 = {
        let a1173 = f101(f102(f207(f220(f206(f53(v0.clone(), f179()))?)), f103(f82(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(f82(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1173)
    };
    let a1175 = 0u64;
    if nat_eq(a1174, a1175) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f315(v0: List<Adt13>) -> R<List<Bytes>> {
    let m1176 = v0;
    match m1176.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f95(f122(f163(f79(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f47(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f47(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f394(f81(f47(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f315(v2)?))
        }
    }
}

pub fn f316(v0: Bytes, v1: List<Adt19>) -> List<Bytes> {
    let m1177 = v1;
    match m1177.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(match v2.clone() {
                Adt19::C0(h1178, _, _) => {
                    h1178
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt19::C0(_, h1179, _) => {
                        h1179
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt19::C0(_, _, h1180) => {
                        h1180
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f316(v0, v3))
            } else {
                f316(v0, v3)
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: List<Adt13>) -> List<Bytes> {
    let m1181 = v1;
    match m1181.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28(f47(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f47(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f317(v0, v3))
            } else {
                if f28(f47(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f47(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f47(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f317(v0, v3))
                } else {
                    f317(v0, v3)
                }
            }
        }
    }
}

pub fn f318(p1186: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1185 = p1186;
    {
        let mut m1185 = m1185;
        loop {
            let step = {
                let v0 = m1185;
                let m1182 = v0;
                match m1182.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1183 = v2;
                        match m1183.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f28(v4, v1.clone()) {
                                    let m1184 = v5;
                                    match m1184.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f395(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: Adt14, v1: Adt13) -> bool {
    if f41(f47(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1187, _, _) => {
            h1187
        }
    }) {
        f41(f320(v1), match v0 {
            Adt14::C0(_, h1188, _) => {
                h1188
            }
        })
    } else {
        false
    }
}

pub fn f320(v0: Adt13) -> Bytes {
    let m1189 = f79(v0, f8(Str::lit("evidence")));
    match m1189 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f47(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f321(v0: Adt13) -> Bytes {
    if f185(v0.clone(), f8(Str::lit("req"))) {
        let a1190 = f8(Str::lit("req "));
        let a1191 = f47(v0, f8(Str::lit("req")));
        append_bytes(a1190, a1191)
    } else {
        let m1192 = f79(v0.clone(), f8(Str::lit("std")));
        match m1192 {
            None => {
                let m1195 = f79(v0, f8(Str::lit("der")));
                match m1195 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f47(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f95(f122(f163(f79(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f47(v1.clone(), f8(Str::lit("authority"))), List::cons(if f185(v1.clone(), f8(Str::lit("row"))) {
                    let a1193 = f8(Str::lit(" row "));
                    let a1194 = f47(v1, f8(Str::lit("row")));
                    append_bytes(a1193, a1194)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f322(v0: Option<List<Adt13>>) -> bool {
    let m1196 = v0;
    match m1196 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f323(v0: Option<List<Adt13>>) -> List<Adt13> {
    let m1197 = v0;
    match m1197 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f324(v0: Bytes, v1: Bytes) -> bool {
    let a1198 = v0;
    let a1199 = f336();
    if equal(a1198, a1199) {
        let a1203 = {
            let a1201 = {
                let a1200 = v1;
                length_bytes(a1200)
            };
            let a1202 = 0u64;
            nat_eq(a1201, a1202)
        };
        bool_not(a1203)
    } else {
        false
    }
}

pub fn f325(v0: Bytes) -> Bytes {
    f7(List::cons(f124(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f326(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1205 = v0;
    let a1206 = {
        let a1204 = Str::lit("redistributable");
        utf8_encode(a1204)
    };
    if equal(a1205, a1206) {
        f396(v1)
    } else {
        Ok(false)
    }
}

pub fn f327(v0: Adt18) -> bool {
    if f397(v0.clone(), f8(Str::lit("name"))) {
        if f398(v0.clone(), f8(Str::lit("schema"))) {
            let m1207 = f331(f246(f247(f242(v0, f8(Str::lit("citation"))))));
            match m1207 {
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

pub fn f328(v0: Bytes) -> R<Bytes> {
    f399(v0.clone(), 0u64, {
        let a1208 = v0;
        length_bytes(a1208)
    }, true, Bytes::lit(&[]))
}

pub fn f329(v0: Adt13) -> Bytes {
    if f185(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f185(v0.clone(), f8(Str::lit("id"))) {
            if f185(v0.clone(), f8(Str::lit("statement"))) {
                if f185(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f344(v0, f8(Str::lit("evidence")))
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

pub fn f330(v0: List<Bytes>) -> Adt13 {
    let m1209 = v0;
    match m1209.uncons() {
        None => {
            Adt13::C5
        }
        Some((v1, v2)) => {
            Adt13::C6(std::rc::Rc::new(Adt13::C2(v1)), std::rc::Rc::new(f330(v2)))
        }
    }
}

pub fn f331(v0: List<Bytes>) -> Option<Adt13> {
    let m1210 = v0;
    match m1210.uncons() {
        None => {
            None::<Adt13>
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("req"))) {
                let m1211 = v2;
                match m1211.uncons() {
                    None => {
                        None::<Adt13>
                    }
                    Some((v3, _)) => {
                        Some(Adt13::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt13::C2(v3)), std::rc::Rc::new(Adt13::C5)))
                    }
                }
            } else {
                if f28(v1.clone(), f8(Str::lit("std"))) {
                    let m1212 = v2;
                    match m1212.uncons() {
                        None => {
                            None::<Adt13>
                        }
                        Some((v5, v6)) => {
                            Some(Adt13::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt13::C2(v5)), std::rc::Rc::new({
                                let m1213 = v6;
                                match m1213.uncons() {
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
                        let m1214 = v2;
                        match m1214.uncons() {
                            None => {
                                None::<Adt13>
                            }
                            Some((v9, v10)) => {
                                let m1215 = v10;
                                match m1215.uncons() {
                                    None => {
                                        None::<Adt13>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt13::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt13::C2(v11)), std::rc::Rc::new(Adt13::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt13::C3(std::rc::Rc::new(f330(v12)))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt13::C2(v9)), std::rc::Rc::new(Adt13::C5))))))))), std::rc::Rc::new(Adt13::C5)))
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

pub fn f332(v0: Adt1, v1: Bytes, v2: Adt18, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1216 = f45(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1216.uncons() {
        None => {
            let m1217 = f331(v4);
            match m1217 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1218 = f8(Str::lit("Catalog."));
                        let a1219 = f328(v3.clone())?;
                        append_bytes(a1218, a1219)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f328(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1220 = f400(v2, v9.clone(), f328(v3.clone())?)?;
                    match m1220 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f328(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f401(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt13::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f86(Adt13::C3(std::rc::Rc::new(f87(f401(f82(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt13::C4(std::rc::Rc::new(Adt13::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt13::C2(v10.clone())), std::rc::Rc::new(Adt13::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt13::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt13::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt13::C1(false, 0u64)), std::rc::Rc::new(Adt13::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1224 = f402(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f35(v10.clone(), 4u64, {
                                    let a1222 = {
                                        let a1221 = v10;
                                        length_bytes(a1221)
                                    };
                                    let a1223 = 4u64;
                                    nat_sub(a1222, a1223)
                                }))?;
                                let a1225 = f8(Str::lit("\u{a}"));
                                append_bytes(a1224, a1225)
                            }), List::cons(f240(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f333(v0: Adt18) -> R<Bytes> {
    let m1226 = v0;
    match m1226 {
        Adt18::C0 => {
            Ok(Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
        }
        Adt18::C1(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C2(v3) => {
            if v3 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt18::C3(v4, v5) => {
            f266(v4, v5)
        }
        Adt18::C4(v6, v7, v8) => {
            f403(v6, v7, v8)
        }
        Adt18::C5(v9) => {
            f404(v9)
        }
        Adt18::C6(r1227) => {
            let v10 = (*r1227).clone();
            Ok(f268(f405(v10)?))
        }
        Adt18::C7(r1228) => {
            let v11 = (*r1228).clone();
            f270(f406(v11)?)
        }
        Adt18::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt18::C10(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f334(v0: Option<Adt18>) -> Option<Bytes> {
    let m1229 = v0;
    match m1229 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1230 = v1;
            match m1230 {
                Adt18::C0 => {
                    None::<Bytes>
                }
                Adt18::C1(_, _) => {
                    None::<Bytes>
                }
                Adt18::C2(_) => {
                    None::<Bytes>
                }
                Adt18::C3(_, _) => {
                    None::<Bytes>
                }
                Adt18::C4(_, _, _) => {
                    None::<Bytes>
                }
                Adt18::C5(v2) => {
                    Some(v2)
                }
                Adt18::C6(_) => {
                    None::<Bytes>
                }
                Adt18::C7(_) => {
                    None::<Bytes>
                }
                Adt18::C8 => {
                    None::<Bytes>
                }
                Adt18::C9(_, _) => {
                    None::<Bytes>
                }
                Adt18::C10(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f335(v0: Adt18) -> List<Adt18> {
    let m1231 = v0;
    match m1231 {
        Adt18::C0 => {
            List::<Adt18>::nil()
        }
        Adt18::C1(_, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C2(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C3(_, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C4(_, _, _) => {
            List::<Adt18>::nil()
        }
        Adt18::C5(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C6(r1232) => {
            let v3 = (*r1232).clone();
            f335(v3)
        }
        Adt18::C7(_) => {
            List::<Adt18>::nil()
        }
        Adt18::C8 => {
            List::<Adt18>::nil()
        }
        Adt18::C9(r1233, r1234) => {
            let v1 = (*r1233).clone();
            let v2 = (*r1234).clone();
            List::cons(v1, f335(v2))
        }
        Adt18::C10(_, _, _) => {
            List::<Adt18>::nil()
        }
    }
}

pub fn f336() -> Bytes {
    let a1235 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1235)
}

pub fn f337(v0: Adt13, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt13> = f79(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f253(Str::lit("id"), f47(v0.clone(), f8(Str::lit("id"))))?, List::cons(f253(Str::lit("name"), f47(v0.clone(), f8(Str::lit("name"))))?, List::cons(f253(Str::lit("issuer"), f47(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f253(Str::lit("edition"), f47(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f253(Str::lit("canonical_id"), f47(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f253(Str::lit("source_role"), f47(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f253(Str::lit("retrieval_date"), f47(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f253(Str::lit("url"), f392(v2.clone(), f8(Str::lit("url"))))?, List::cons(f253(Str::lit("media_type"), f392(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f253(Str::lit("license"), f392(v2.clone(), f8(Str::lit("license"))))?, List::cons(f253(Str::lit("redistribution"), f392(v2, f8(Str::lit("redistribution"))))?, List::cons(f253(Str::lit("statement"), f47(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1236 = v1;
        match m1236 {
            None => {
                f7(List::cons(f253(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f253(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f253(Str::lit("artifact"), {
                    let (h1237, _) = v3;
                    h1237
                })?, List::cons(f253(Str::lit("checksum"), {
                    let (_, h1238) = v3;
                    h1238
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f407(f122(f163(f79(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f338(v0: Adt24) -> Bytes {
    let a1259 = f408(match v0.clone() {
        Adt24::C0(h1239, _, _, _, _, _, _, _) => {
            h1239
        }
    });
    let a1260 = {
        let a1257 = f408(match v0.clone() {
            Adt24::C0(_, h1240, _, _, _, _, _, _) => {
                h1240
            }
        });
        let a1258 = {
            let a1255 = f408(match v0.clone() {
                Adt24::C0(_, _, h1241, _, _, _, _, _) => {
                    h1241
                }
            });
            let a1256 = {
                let a1253 = f408(match v0.clone() {
                    Adt24::C0(_, _, _, h1242, _, _, _, _) => {
                        h1242
                    }
                });
                let a1254 = {
                    let a1251 = f408(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1243, _, _, _) => {
                            h1243
                        }
                    });
                    let a1252 = {
                        let a1249 = f408(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1244, _, _) => {
                                h1244
                            }
                        });
                        let a1250 = {
                            let a1247 = f408(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1245, _) => {
                                    h1245
                                }
                            });
                            let a1248 = f408(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1246) => {
                                    h1246
                                }
                            });
                            append_bytes(a1247, a1248)
                        };
                        append_bytes(a1249, a1250)
                    };
                    append_bytes(a1251, a1252)
                };
                append_bytes(a1253, a1254)
            };
            append_bytes(a1255, a1256)
        };
        append_bytes(a1257, a1258)
    };
    append_bytes(a1259, a1260)
}

pub fn f339(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1261 = v0.clone();
        length_bytes(a1261)
    };
    let v2: u64 = {
        let a1267 = {
            let a1264 = {
                let a1262 = v1;
                let a1263 = 72u64;
                nat_add(a1262, a1263)?
            };
            let a1265 = 64u64;
            let a1266 = 0u64;
            nat_quot(a1264, a1265, a1266)
        };
        let a1268 = 64u64;
        nat_mul(a1267, a1268)?
    };
    let (_, h1269) = f409(Fn11::F410(v0, v1, v2), {
        let a1270 = v2;
        let a1271 = 64u64;
        let a1272 = 0u64;
        nat_quot(a1270, a1271, a1272)
    }, (0u64, f411()))?;
    Ok(h1269)
}

pub fn f340(p1275: List<List<Bytes>>, v1: Bytes, p1276: List<Bytes>) -> R<List<Bytes>> {
    let m1274 = (p1275, p1276);
    {
        let mut m1274 = m1274;
        loop {
            let step = {
                let (v0, v2) = m1274;
                let m1273 = v0;
                match m1273.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f412(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f342(f413(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: Bytes) -> R<Bytes> {
    Ok(f35(v0.clone(), 0u64, f275(v0)?))
}

pub fn f342(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1277 = v0;
    match m1277.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1278 = v3.clone();
            match m1278.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1281 = v2;
                    let a1282 = {
                        let a1279 = v1.clone();
                        let a1280 = f342(v3, v1);
                        append_bytes(a1279, a1280)
                    };
                    append_bytes(a1281, a1282)
                }
            }
        }
    }
}

pub fn f343(v0: Adt13, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1283 = f79(v0, v1);
    match m1283 {
        None => {
            List::<Adt13>::nil()
        }
        Some(v3) => {
            f163(f79(v3, v2))
        }
    }
}

pub fn f344(v0: Adt13, v1: Bytes) -> bool {
    let m1284 = f79(v0, v1);
    match m1284 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f345(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f346(v0: Bytes, p1312: u64, p1313: u64, p1314: bool, p1315: bool, p1316: u64, p1317: Bytes) -> R<Bytes> {
    let m1311 = (p1312, (p1313, (p1314, (p1315, (p1316, p1317)))));
    {
        let mut m1311 = m1311;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m1311;
                let m1285 = v2;
                if m1285 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f414(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m1285 - 1;
                    let m1288 = {
                        let a1286 = v0.clone();
                        let a1287 = v1;
                        index_bytes(a1286, a1287)
                    };
                    match m1288 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f414(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f62(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a1289 = v1;
                                    let a1290 = 1u64;
                                    nat_add(a1289, a1290)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a1291 = v9;
                                    let a1292 = 34u64;
                                    !nat_eq(a1291, a1292)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1293 = v9;
                                    let a1294 = 92u64;
                                    nat_eq(a1293, a1294)
                                }, (v5, v6))))))
                            } else {
                                let a1295 = v9;
                                let a1296 = 32u64;
                                if if if if nat_eq(a1295, a1296) {
                                    true
                                } else {
                                    let a1297 = v9;
                                    let a1298 = 9u64;
                                    nat_eq(a1297, a1298)
                                } {
                                    true
                                } else {
                                    let a1299 = v9;
                                    let a1300 = 10u64;
                                    nat_eq(a1299, a1300)
                                } {
                                    true
                                } else {
                                    let a1301 = v9;
                                    let a1302 = 13u64;
                                    nat_eq(a1301, a1302)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1303 = v1;
                                        let a1304 = 1u64;
                                        nat_add(a1303, a1304)?
                                    }, (v7, (false, (false, ({
                                        let a1305 = v1;
                                        let a1306 = 1u64;
                                        nat_add(a1305, a1306)?
                                    }, f414(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a1307 = v1;
                                        let a1308 = 1u64;
                                        nat_add(a1307, a1308)?
                                    }, (v7, ({
                                        let a1309 = v9;
                                        let a1310 = 34u64;
                                        nat_eq(a1309, a1310)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: Bytes, v1: u64) -> R<u64> {
    let m1320 = {
        let a1318 = v0;
        let a1319 = v1;
        index_bytes(a1318, a1319)
    };
    match m1320 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1365 = {
                let a1341 = {
                    let a1329 = {
                        let a1323 = {
                            let a1321 = v3;
                            let a1322 = 128u8;
                            fixed_u8::bit_and(a1321, a1322)
                        };
                        let a1324 = 128u8;
                        if equal(a1323, a1324) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1330 = {
                        let a1327 = {
                            let a1325 = v3;
                            let a1326 = 64u8;
                            fixed_u8::bit_and(a1325, a1326)
                        };
                        let a1328 = 64u8;
                        if equal(a1327, a1328) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1329, a1330)?
                };
                let a1342 = {
                    let a1339 = {
                        let a1333 = {
                            let a1331 = v3;
                            let a1332 = 32u8;
                            fixed_u8::bit_and(a1331, a1332)
                        };
                        let a1334 = 32u8;
                        if equal(a1333, a1334) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1340 = {
                        let a1337 = {
                            let a1335 = v3;
                            let a1336 = 16u8;
                            fixed_u8::bit_and(a1335, a1336)
                        };
                        let a1338 = 16u8;
                        if equal(a1337, a1338) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1339, a1340)?
                };
                nat_add(a1341, a1342)?
            };
            let a1366 = {
                let a1363 = {
                    let a1351 = {
                        let a1345 = {
                            let a1343 = v3;
                            let a1344 = 8u8;
                            fixed_u8::bit_and(a1343, a1344)
                        };
                        let a1346 = 8u8;
                        if equal(a1345, a1346) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1352 = {
                        let a1349 = {
                            let a1347 = v3;
                            let a1348 = 4u8;
                            fixed_u8::bit_and(a1347, a1348)
                        };
                        let a1350 = 4u8;
                        if equal(a1349, a1350) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1351, a1352)?
                };
                let a1364 = {
                    let a1361 = {
                        let a1355 = {
                            let a1353 = v3;
                            let a1354 = 2u8;
                            fixed_u8::bit_and(a1353, a1354)
                        };
                        let a1356 = 2u8;
                        if equal(a1355, a1356) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1362 = {
                        let a1359 = {
                            let a1357 = v3;
                            let a1358 = 1u8;
                            fixed_u8::bit_and(a1357, a1358)
                        };
                        let a1360 = 1u8;
                        if equal(a1359, a1360) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1361, a1362)?
                };
                nat_add(a1363, a1364)?
            };
            nat_add(a1365, a1366)
        }
    }
}

pub fn f348(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f415(v0.clone(), v1, {
        let a1368 = {
            let a1367 = v0.clone();
            length_bytes(a1367)
        };
        let a1369 = 1u64;
        nat_add(a1368, a1369)?
    })?;
    let a1370 = f347(v0.clone(), v2)?;
    let a1371 = 34u64;
    if nat_eq(a1370, a1371) {
        Ok(Some(({
            let a1372 = v2;
            let a1373 = 1u64;
            nat_add(a1372, a1373)?
        }, f35(v0, v1, {
            let a1374 = v2;
            let a1375 = v1;
            nat_sub(a1374, a1375)
        }))))
    } else {
        f416(v0, v1)
    }
}

pub fn f349(v0: Bytes, v1: u64) -> R<Option<(u64, Adt18)>> {
    let v2: bool = {
        let a1376 = f347(v0.clone(), v1)?;
        let a1377 = 45u64;
        nat_eq(a1376, a1377)
    };
    let v3: u64 = if v2 {
        let a1378 = v1;
        let a1379 = 1u64;
        nat_add(a1378, a1379)?
    } else {
        v1
    };
    let v4: u64 = f417(v0.clone(), v3)?;
    let v5: u64 = {
        let a1380 = v4;
        let a1381 = v3;
        nat_sub(a1380, a1381)
    };
    let v6: bool = {
        let a1382 = f347(v0.clone(), v4)?;
        let a1383 = 46u64;
        nat_eq(a1382, a1383)
    };
    let v7: u64 = if v6 {
        let a1384 = v4;
        let a1385 = 1u64;
        nat_add(a1384, a1385)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f417(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f347(v0.clone(), v8)?;
    let v10: bool = {
        let a1386 = v9;
        let a1387 = 101u64;
        if nat_eq(a1386, a1387) {
            true
        } else {
            let a1388 = v9;
            let a1389 = 69u64;
            nat_eq(a1388, a1389)
        }
    };
    let v11: u64 = f347(v0.clone(), {
        let a1390 = v8;
        let a1391 = 1u64;
        nat_add(a1390, a1391)?
    })?;
    let v12: bool = f345(v10, f418({
        let a1392 = v11;
        let a1393 = 43u64;
        nat_eq(a1392, a1393)
    }, {
        let a1394 = v11;
        let a1395 = 45u64;
        nat_eq(a1394, a1395)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a1396 = v8;
            let a1397 = 2u64;
            nat_add(a1396, a1397)?
        } else {
            let a1398 = v8;
            let a1399 = 1u64;
            nat_add(a1398, a1399)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f417(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f35(v0.clone(), v13, {
        let a1400 = v14;
        let a1401 = v13;
        nat_sub(a1400, a1401)
    });
    let v16: u64 = f419(v15.clone())?;
    let v17: Bytes = f35(v15.clone(), v16, {
        let a1403 = {
            let a1402 = v15;
            length_bytes(a1402)
        };
        let a1404 = v16;
        nat_sub(a1403, a1404)
    });
    let a1405 = v5;
    let a1406 = 0u64;
    if if nat_eq(a1405, a1406) {
        true
    } else {
        if f345({
            let a1407 = 1u64;
            let a1408 = v5;
            nat_lt(a1407, a1408)
        }, {
            let a1409 = f347(v0.clone(), v3)?;
            let a1410 = 48u64;
            nat_eq(a1409, a1410)
        }) {
            true
        } else {
            if f345(v6, {
                let a1411 = v8;
                let a1412 = v7;
                nat_eq(a1411, a1412)
            }) {
                true
            } else {
                f418(if v10 {
                    let a1413 = v14;
                    let a1414 = v13;
                    nat_eq(a1413, a1414)
                } else {
                    false
                }, {
                    let a1416 = 18u64;
                    let a1417 = {
                        let a1415 = v17.clone();
                        length_bytes(a1415)
                    };
                    nat_lt(a1416, a1417)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt18)>)
    } else {
        let m1421 = {
            let a1419 = {
                let a1418 = v17.clone();
                length_bytes(a1418)
            };
            let a1420 = 0u64;
            if nat_eq(a1419, a1420) {
                Some(0u64)
            } else {
                f420(v17)?
            }
        };
        match m1421 {
            None => {
                Ok(None::<(u64, Adt18)>)
            }
            Some(v18) => {
                let m1426 = f421(f35(v0.clone(), v3, v5), f35(v0, v7, {
                    let a1422 = v8;
                    let a1423 = v7;
                    nat_sub(a1422, a1423)
                }), f345(v12, {
                    let a1424 = v11;
                    let a1425 = 45u64;
                    nat_eq(a1424, a1425)
                }), v18)?;
                match m1426 {
                    None => {
                        Ok(None::<(u64, Adt18)>)
                    }
                    Some(v19) => {
                        let a1429 = 19u64;
                        let a1430 = {
                            let a1428 = {
                                let (h1427, _) = v19.clone();
                                h1427
                            };
                            length_bytes(a1428)
                        };
                        if nat_lt(a1429, a1430) {
                            Ok(None::<(u64, Adt18)>)
                        } else {
                            let m1432 = f420({
                                let (h1431, _) = v19.clone();
                                h1431
                            })?;
                            match m1432 {
                                None => {
                                    Ok(None::<(u64, Adt18)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f345(v2, {
                                        let a1433 = 0u64;
                                        let a1434 = v20;
                                        nat_lt(a1433, a1434)
                                    });
                                    if f422(v21, v20) {
                                        Ok(Some((v14, {
                                            let a1436 = {
                                                let (_, h1435) = v19.clone();
                                                h1435
                                            };
                                            let a1437 = 0u64;
                                            if nat_eq(a1436, a1437) {
                                                Adt18::C3(v21, v20)
                                            } else {
                                                Adt18::C4(v21, v20, {
                                                    let (_, h1438) = v19;
                                                    h1438
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

pub fn f350(v0: Bytes) -> Bytes {
    if f190(v0.clone(), Bytes::lit(&[0x0d])) {
        f35(v0.clone(), 0u64, {
            let a1440 = {
                let a1439 = v0;
                length_bytes(a1439)
            };
            let a1441 = 1u64;
            nat_sub(a1440, a1441)
        })
    } else {
        v0
    }
}

pub fn f351(v0: Adt13, v1: Bytes) -> Adt13 {
    let m1442 = v0;
    match m1442 {
        Adt13::C0(v3) => {
            Adt13::C0(v3)
        }
        Adt13::C1(v4, v5) => {
            Adt13::C1(v4, v5)
        }
        Adt13::C2(v2) => {
            Adt13::C2(f423(f49(), v2, v1))
        }
        Adt13::C3(r1443) => {
            let v6 = (*r1443).clone();
            Adt13::C3(std::rc::Rc::new(v6))
        }
        Adt13::C4(r1444) => {
            let v7 = (*r1444).clone();
            Adt13::C4(std::rc::Rc::new(v7))
        }
        Adt13::C5 => {
            Adt13::C5
        }
        Adt13::C6(r1445, r1446) => {
            let v8 = (*r1445).clone();
            let v9 = (*r1446).clone();
            Adt13::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt13::C7(v10, r1447, r1448) => {
            let v11 = (*r1447).clone();
            let v12 = (*r1448).clone();
            Adt13::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f352(v0: u64) -> R<Bytes> {
    f424(f425(), 10u64, v0)
}

pub fn f353(v0: Fn4, p1453: u64, p1454: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1452 = (p1453, p1454);
    {
        let mut m1452 = m1452;
        loop {
            let step = {
                let (v1, v2) = m1452;
                let m1449 = v1;
                if m1449 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1449 - 1;
                    let m1451 = {
                        let c1450: Fn4 = v0.clone();
                        c1450.apply(v2.clone())?
                    };
                    match m1451 {
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
                Ok(next) => { m1452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f426(v0, v1)
}

pub fn f355(v0: Fn5, p1458: List<(Bytes, Bytes)>, p1459: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1457 = (p1458, p1459);
    {
        let mut m1457 = m1457;
        loop {
            let step = {
                let (v1, v2) = m1457;
                let m1455 = v2;
                match m1455.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1456: Fn5 = v0.clone();
                            c1456.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1457 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f427(v0, v1)
}

pub fn f357(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f428(v0)
}

pub fn f358(v0: Fn6, p1463: List<Bytes>, p1464: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1462 = (p1463, p1464);
    {
        let mut m1462 = m1462;
        loop {
            let step = {
                let (v1, v2) = m1462;
                let m1460 = v2;
                match m1460.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1461: Fn6 = v0.clone();
                            c1461.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f429(v1)?, v0))
}

pub fn f360(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f267(v0)?, List::cons(f8(Str::lit(":")), List::cons(f267(v1)?, List::<Bytes>::nil())))))
}

pub fn f361(v0: Adt12) -> Bytes {
    let m1465 = v0;
    match m1465 {
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

pub fn f362(v0: Adt12) -> Bytes {
    let m1466 = v0;
    match m1466 {
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

pub fn f363(v0: Adt12) -> Bytes {
    let m1467 = v0;
    match m1467 {
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

pub fn f364(v0: Bytes, v1: u64) -> Option<u64> {
    let a1468 = 0u64;
    let a1469 = v1;
    if nat_lt(a1468, a1469) {
        if f430(v0, {
            let a1470 = v1;
            let a1471 = 1u64;
            nat_sub(a1470, a1471)
        }) {
            Some({
                let a1472 = v1;
                let a1473 = 1u64;
                nat_sub(a1472, a1473)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f365(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1474 = v2;
    let a1475 = v0;
    if nat_lt(a1474, a1475) {
        if f430(v1, v2) {
            Ok(Some({
                let a1476 = v2;
                let a1477 = 1u64;
                nat_add(a1476, a1477)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f366(v0: (Bytes, Bytes), p1480: List<(Bytes, Bytes)>) -> bool {
    let m1479 = p1480;
    {
        let mut m1479 = m1479;
        loop {
            let step = {
                let v1 = m1479;
                let m1478 = v1;
                match m1478.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f431(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f367(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1481 = v1;
    match m1481.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h1482, _) = v2;
                h1482
            }, v0.clone()) {
                List::cons({
                    let (_, h1483) = v2;
                    h1483
                }, f367(v0, v3))
            } else {
                f367(v0, v3)
            }
        }
    }
}

pub fn f368() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f369(v0: Adt13, v1: Bytes, v2: Bytes) -> bool {
    let m1484 = f79(v0, v1);
    match m1484 {
        None => {
            false
        }
        Some(v3) => {
            f185(v3, v2)
        }
    }
}

pub fn f370() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f371(v0: Bytes) -> R<bool> {
    let m1485 = f194(v0.clone(), f8(Str::lit("&&")))?;
    if match m1485 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1486 = f194(v0, f8(Str::lit("||")))?;
        match m1486 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f372(v0: Adt17, v1: Option<u64>) -> bool {
    let m1487 = v1;
    match m1487 {
        None => {
            true
        }
        Some(v2) => {
            f296(v2, match v0 {
                Adt17::C0(_, h1488, _, _) => {
                    h1488
                }
            })
        }
    }
}

pub fn f373(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a1490 = match v0 {
            Adt15::C0(_, _, h1489, _, _) => {
                h1489
            }
        };
        length_list(a1490)
    })
}

pub fn f374(v0: Bytes) -> Option<Adt22> {
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

pub fn f375(v0: Bytes, p1495: List<Bytes>, p1496: u64) -> R<Option<u64>> {
    let m1494 = (p1495, p1496);
    {
        let mut m1494 = m1494;
        loop {
            let step = {
                let (v1, v2) = m1494;
                let m1491 = v1;
                match m1491.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f28(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1492 = v2;
                                let a1493 = 1u64;
                                nat_add(a1492, a1493)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Option<Adt13>) -> Option<u64> {
    let m1497 = v0;
    match m1497 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1498 = v1;
            match m1498 {
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

pub fn f377(v0: Bytes, v1: u64) -> R<u64> {
    let m1501 = {
        let a1499 = v0;
        let a1500 = v1;
        index_bytes(a1499, a1500)
    };
    match m1501 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f62(v2)
        }
    }
}

pub fn f378(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1505 = {
        let a1502 = v1;
        let a1503 = v2;
        let a1504 = 1u64;
        slice_bytes(a1502, a1503, a1504)
    };
    match m1505 {
        None => {
            v0
        }
        Some(v3) => {
            let a1506 = v0;
            let a1507 = v3;
            append_bytes(a1506, a1507)
        }
    }
}

pub fn f379(p1510: List<Bytes>, p1511: List<Bytes>) -> List<Bytes> {
    let m1509 = (p1510, p1511);
    {
        let mut m1509 = m1509;
        loop {
            let step = {
                let (v0, v1) = m1509;
                let m1508 = v0;
                match m1508.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1509 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Fn7, p1515: List<Bytes>, p1516: List<(Bytes, ())>) -> List<Bytes> {
    let m1514 = (p1515, p1516);
    {
        let mut m1514 = m1514;
        loop {
            let step = {
                let (v1, v2) = m1514;
                let m1512 = v2;
                match m1512.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1513: Fn7 = v0.clone();
                            c1513.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1514 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1517, _) = v1;
        h1517
    }, v0)
}

pub fn f382(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f432(Fn12::F433, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f383(v0: Fn8, p1521: List<List<(Bytes, ())>>, p1522: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1520 = (p1521, p1522);
    {
        let mut m1520 = m1520;
        loop {
            let step = {
                let (v1, v2) = m1520;
                let m1518 = v2;
                match m1518.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1519: Fn8 = v0.clone();
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

pub fn f384(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f385(v0: Fn9, p1527: u64, p1528: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1526 = (p1527, p1528);
    {
        let mut m1526 = m1526;
        loop {
            let step = {
                let (v1, v2) = m1526;
                let m1523 = v1;
                if m1523 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1523 - 1;
                    let m1525 = {
                        let c1524: Fn9 = v0.clone();
                        c1524.apply(v2.clone())?
                    };
                    match m1525 {
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
                Ok(next) => { m1526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f434(v0)
}

pub fn f387(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f435(Fn13::F436, List::<(Bytes, ())>::nil(), v0)
}

pub fn f388(v0: Fn10, p1532: List<(Bytes, ())>, p1533: List<Bytes>) -> List<(Bytes, ())> {
    let m1531 = (p1532, p1533);
    {
        let mut m1531 = m1531;
        loop {
            let step = {
                let (v1, v2) = m1531;
                let m1529 = v2;
                match m1529.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1530: Fn10 = v0.clone();
                            c1530.apply(v1, v3)
                        }, v4))
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

pub fn f389(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f390(p1536: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1535 = p1536;
    {
        let mut m1535 = m1535;
        loop {
            let step = {
                let v0 = m1535;
                let m1534 = v0;
                match m1534.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f437(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
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

pub fn f391(v0: Bytes, p1562: u64, p1563: u64, p1564: Bytes) -> R<Bytes> {
    let m1561 = (p1562, (p1563, p1564));
    {
        let mut m1561 = m1561;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1561;
                let m1537 = v2;
                if m1537 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1537 - 1;
                    let m1540 = {
                        let a1538 = v0.clone();
                        let a1539 = v1;
                        index_bytes(a1538, a1539)
                    };
                    match m1540 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f62(v5)?;
                            if f418({
                                let a1541 = v6;
                                let a1542 = 34u64;
                                nat_eq(a1541, a1542)
                            }, {
                                let a1543 = v6;
                                let a1544 = 92u64;
                                nat_eq(a1543, a1544)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1545 = v1;
                                    let a1546 = 1u64;
                                    nat_add(a1545, a1546)?
                                }, (v4, {
                                    let a1549 = v3;
                                    let a1550 = {
                                        let a1547 = f8(Str::lit("\\"));
                                        let a1548 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1547, a1548)
                                    };
                                    append_bytes(a1549, a1550)
                                })))
                            } else {
                                let a1551 = v6;
                                let a1552 = 10u64;
                                if nat_eq(a1551, a1552) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1553 = v1;
                                        let a1554 = 1u64;
                                        nat_add(a1553, a1554)?
                                    }, (v4, {
                                        let a1555 = v3;
                                        let a1556 = f8(Str::lit("\\n"));
                                        append_bytes(a1555, a1556)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1557 = v1;
                                        let a1558 = 1u64;
                                        nat_add(a1557, a1558)?
                                    }, (v4, {
                                        let a1559 = v3;
                                        let a1560 = f35(v0.clone(), v1, 1u64);
                                        append_bytes(a1559, a1560)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1561 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f392(v0: Option<Adt13>, v1: Bytes) -> Bytes {
    let m1565 = v0;
    match m1565 {
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
    let m1566 = f256(v2.clone())?;
    match m1566 {
        None => {
            f439(v0, v1, v2)
        }
        Some(v3) => {
            f438(v0, v1, v3)
        }
    }
}

pub fn f394(v0: List<Bytes>) -> List<Bytes> {
    let m1567 = v0;
    match m1567.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1569 = {
                    let a1568 = v1.clone();
                    length_bytes(a1568)
                };
                let a1570 = 0u64;
                if nat_eq(a1569, a1570) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f394(v2))
        }
    }
}

pub fn f395(v0: List<Bytes>) -> Bytes {
    let m1571 = v0;
    match m1571.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f396(v0: Bytes) -> R<bool> {
    let m1573 = {
        let a1572 = v0.clone();
        length_bytes(a1572)
    };
    if m1573 == 0 {
        Ok(true)
    } else {
        let v1 = m1573 - 1;
        let m1576 = {
            let a1574 = v0;
            let a1575 = v1;
            index_bytes(a1574, a1575)
        };
        match m1576 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f440({
                    let v3: u8 = v2;
                    let a1621 = {
                        let a1597 = {
                            let a1585 = {
                                let a1579 = {
                                    let a1577 = v3;
                                    let a1578 = 128u8;
                                    fixed_u8::bit_and(a1577, a1578)
                                };
                                let a1580 = 128u8;
                                if equal(a1579, a1580) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1586 = {
                                let a1583 = {
                                    let a1581 = v3;
                                    let a1582 = 64u8;
                                    fixed_u8::bit_and(a1581, a1582)
                                };
                                let a1584 = 64u8;
                                if equal(a1583, a1584) {
                                    64u64
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
                                    let a1588 = 32u8;
                                    fixed_u8::bit_and(a1587, a1588)
                                };
                                let a1590 = 32u8;
                                if equal(a1589, a1590) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1596 = {
                                let a1593 = {
                                    let a1591 = v3;
                                    let a1592 = 16u8;
                                    fixed_u8::bit_and(a1591, a1592)
                                };
                                let a1594 = 16u8;
                                if equal(a1593, a1594) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1595, a1596)?
                        };
                        nat_add(a1597, a1598)?
                    };
                    let a1622 = {
                        let a1619 = {
                            let a1607 = {
                                let a1601 = {
                                    let a1599 = v3;
                                    let a1600 = 8u8;
                                    fixed_u8::bit_and(a1599, a1600)
                                };
                                let a1602 = 8u8;
                                if equal(a1601, a1602) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1608 = {
                                let a1605 = {
                                    let a1603 = v3;
                                    let a1604 = 4u8;
                                    fixed_u8::bit_and(a1603, a1604)
                                };
                                let a1606 = 4u8;
                                if equal(a1605, a1606) {
                                    4u64
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
                                    let a1610 = 2u8;
                                    fixed_u8::bit_and(a1609, a1610)
                                };
                                let a1612 = 2u8;
                                if equal(a1611, a1612) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1618 = {
                                let a1615 = {
                                    let a1613 = v3;
                                    let a1614 = 1u8;
                                    fixed_u8::bit_and(a1613, a1614)
                                };
                                let a1616 = 1u8;
                                if equal(a1615, a1616) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1617, a1618)?
                        };
                        nat_add(a1619, a1620)?
                    };
                    nat_add(a1621, a1622)?
                }))
            }
        }
    }
}

pub fn f397(v0: Adt18, v1: Bytes) -> bool {
    let a1624 = {
        let a1623 = f245(v0, v1);
        length_bytes(a1623)
    };
    let a1625 = 0u64;
    !nat_eq(a1624, a1625)
}

pub fn f398(v0: Adt18, v1: Bytes) -> bool {
    let m1626 = f242(v0, v1);
    match m1626 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f399(v0: Bytes, p1647: u64, p1648: u64, p1649: bool, p1650: Bytes) -> R<Bytes> {
    let m1646 = (p1647, (p1648, (p1649, p1650)));
    {
        let mut m1646 = m1646;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1646;
                let m1627 = v2;
                if m1627 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1627 - 1;
                    let m1630 = {
                        let a1628 = v0.clone();
                        let a1629 = v1;
                        index_bytes(a1628, a1629)
                    };
                    match m1630 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f62(v6)?;
                            if f441(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1631 = v1;
                                    let a1632 = 1u64;
                                    nat_add(a1631, a1632)?
                                }, (v5, (true, v4))))
                            } else {
                                if f345(f442(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1633 = v1;
                                        let a1634 = 1u64;
                                        nat_add(a1633, a1634)?
                                    }, (v5, (false, {
                                        let a1638 = v4;
                                        let a1639 = f35({
                                            let a1635 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1635)
                                        }, {
                                            let a1636 = v7;
                                            let a1637 = 97u64;
                                            nat_sub(a1636, a1637)
                                        }, 1u64);
                                        append_bytes(a1638, a1639)
                                    }))))
                                } else {
                                    if f418(f442(v7), f418(f443(v7), f444(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1640 = v1;
                                            let a1641 = 1u64;
                                            nat_add(a1640, a1641)?
                                        }, (v5, (false, {
                                            let a1642 = v4;
                                            let a1643 = f35(v0.clone(), v1, 1u64);
                                            append_bytes(a1642, a1643)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1644 = v1;
                                            let a1645 = 1u64;
                                            nat_add(a1644, a1645)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1646 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: Adt18, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt18)> = f445(f446(f242(v0.clone(), f8(Str::lit("$defs")))), f446(f242(v0.clone(), f8(Str::lit("definitions")))));
    let m1651 = f447(4096u64, v3.clone(), List::<Bytes>::nil(), f448(v3)?)?;
    match m1651 {
        Ok(v5) => {
            let m1652 = f449(4096u64, f450(f451(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1652 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f342(f213(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f401(v0: List<Adt13>, v1: Bytes, v2: Bytes, v3: Adt13) -> List<Adt13> {
    let m1653 = v0;
    match m1653.uncons() {
        None => {
            List::cons(v3, List::<Adt13>::nil())
        }
        Some((v4, v5)) => {
            if f28(f47(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f452(v5, v1, v2))
            } else {
                List::cons(v4, f401(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f402(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f342(f453(f81(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f403(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f352(v1)?;
    let v4: u64 = {
        let a1654 = v3.clone();
        length_bytes(a1654)
    };
    let v5: Bytes = {
        let a1655 = v2;
        let a1656 = v4;
        if nat_lt(a1655, a1656) {
            f7(List::cons(f35(v3.clone(), 0u64, {
                let a1657 = v4;
                let a1658 = v2;
                nat_sub(a1657, a1658)
            }), List::cons(f8(Str::lit(".")), List::cons(f35(v3, {
                let a1659 = v4;
                let a1660 = v2;
                nat_sub(a1659, a1660)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f454({
                let a1661 = v2;
                let a1662 = v4;
                nat_sub(a1661, a1662)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1663 = f8(Str::lit("-"));
        let a1664 = v5;
        Ok(append_bytes(a1663, a1664))
    } else {
        Ok(v5)
    }
}

pub fn f404(v0: Bytes) -> R<Bytes> {
    let a1669 = f415(v0.clone(), 0u64, {
        let a1666 = {
            let a1665 = v0.clone();
            length_bytes(a1665)
        };
        let a1667 = 1u64;
        nat_add(a1666, a1667)?
    })?;
    let a1670 = {
        let a1668 = v0.clone();
        length_bytes(a1668)
    };
    if nat_eq(a1669, a1670) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f267(v0)
    }
}

pub fn f405(v0: Adt18) -> R<List<Bytes>> {
    let m1671 = v0;
    match m1671 {
        Adt18::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C3(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C4(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt18::C9(r1672, r1673) => {
            let v12 = (*r1672).clone();
            let v13 = (*r1673).clone();
            Ok(List::cons(f333(v12)?, f405(v13)?))
        }
        Adt18::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f406(v0: Adt18) -> R<List<(Bytes, Bytes)>> {
    let m1674 = v0;
    match m1674 {
        Adt18::C0 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C3(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C4(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt18::C10(v14, r1675, r1676) => {
            let v15 = (*r1675).clone();
            let v16 = (*r1676).clone();
            Ok(List::cons((v14, f333(v15)?), f406(v16)?))
        }
    }
}

pub fn f407(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f342(f455(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f408(v0: u32) -> Bytes {
    let a1705 = f456({
        let a1677 = f457(v0, 28u32);
        let a1678 = 15u32;
        fixed_u32::bit_and(a1677, a1678)
    });
    let a1706 = {
        let a1703 = f456({
            let a1679 = f457(v0, 24u32);
            let a1680 = 15u32;
            fixed_u32::bit_and(a1679, a1680)
        });
        let a1704 = {
            let a1701 = f456({
                let a1681 = f457(v0, 20u32);
                let a1682 = 15u32;
                fixed_u32::bit_and(a1681, a1682)
            });
            let a1702 = {
                let a1699 = f456({
                    let a1683 = f457(v0, 16u32);
                    let a1684 = 15u32;
                    fixed_u32::bit_and(a1683, a1684)
                });
                let a1700 = {
                    let a1697 = f456({
                        let a1685 = f457(v0, 12u32);
                        let a1686 = 15u32;
                        fixed_u32::bit_and(a1685, a1686)
                    });
                    let a1698 = {
                        let a1695 = f456({
                            let a1687 = f457(v0, 8u32);
                            let a1688 = 15u32;
                            fixed_u32::bit_and(a1687, a1688)
                        });
                        let a1696 = {
                            let a1693 = f456({
                                let a1689 = f457(v0, 4u32);
                                let a1690 = 15u32;
                                fixed_u32::bit_and(a1689, a1690)
                            });
                            let a1694 = f456({
                                let a1691 = v0;
                                let a1692 = 15u32;
                                fixed_u32::bit_and(a1691, a1692)
                            });
                            append_bytes(a1693, a1694)
                        };
                        append_bytes(a1695, a1696)
                    };
                    append_bytes(a1697, a1698)
                };
                append_bytes(a1699, a1700)
            };
            append_bytes(a1701, a1702)
        };
        append_bytes(a1703, a1704)
    };
    append_bytes(a1705, a1706)
}

pub fn f409(v0: Fn11, p1710: u64, p1711: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1709 = (p1710, p1711);
    {
        let mut m1709 = m1709;
        loop {
            let step = {
                let (v1, v2) = m1709;
                let m1707 = v1;
                if m1707 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1707 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1708: Fn11 = v0.clone();
                        c1708.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1709 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1713 = {
            let (h1712, _) = v3.clone();
            h1712
        };
        let a1714 = 64u64;
        nat_add(a1713, a1714)?
    }, f458({
        let (_, h1715) = v3;
        h1715
    }, f459(v0, v1, v2, {
        let (h1716, _) = v3;
        h1716
    })?)?))
}

pub fn f411() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f412(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1717 = f33(v1, 34u8)?;
    match m1717.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1718 = v3;
            match m1718.uncons() {
                None => {
                    Ok(f460(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f413(v0: List<Bytes>) -> List<Bytes> {
    let m1719 = v0;
    match m1719.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f28(v1.clone(), f8(Str::lit("authority = []"))) {
                f413(v2)
            } else {
                List::cons(v1, f413(v2))
            }
        }
    }
}

pub fn f414(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1722 = v3;
    let a1723 = f35(v0, v1, {
        let a1720 = v2;
        let a1721 = v1;
        nat_sub(a1720, a1721)
    });
    append_bytes(a1722, a1723)
}

pub fn f415(v0: Bytes, p1736: u64, p1737: u64) -> R<u64> {
    let m1735 = (p1736, p1737);
    {
        let mut m1735 = m1735;
        loop {
            let step = {
                let (v1, v2) = m1735;
                let m1724 = v2;
                if m1724 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1724 - 1;
                    let v4: u64 = f347(v0.clone(), v1)?;
                    let a1725 = v4;
                    let a1726 = 34u64;
                    if nat_eq(a1725, a1726) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1727 = v4;
                        let a1728 = 92u64;
                        if nat_eq(a1727, a1728) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1729 = v4;
                            let a1730 = 32u64;
                            if nat_lt(a1729, a1730) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1731 = v4;
                                let a1732 = 256u64;
                                if nat_eq(a1731, a1732) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1733 = v1;
                                        let a1734 = 1u64;
                                        nat_add(a1733, a1734)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f461(Fn14::F462(v0.clone()), {
        let a1739 = {
            let a1738 = v0;
            length_bytes(a1738)
        };
        let a1740 = 1u64;
        nat_add(a1739, a1740)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1744 = {
        let (_, h1741) = {
            let (_, h1742) = {
                let (h1743, _) = v2.clone();
                h1743
            };
            h1742
        };
        h1741
    };
    let a1745 = 1u64;
    if nat_eq(a1744, a1745) {
        Ok(Some(({
            let (h1746, _) = {
                let (h1747, _) = v2.clone();
                h1747
            };
            h1746
        }, {
            let (h1748, _) = {
                let (_, h1749) = {
                    let (h1750, _) = v2;
                    h1750
                };
                h1749
            };
            h1748
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f417(v0: Bytes, v1: u64) -> R<u64> {
    let (h1751, _) = f286(Fn3::F463(v0.clone()), {
        let a1753 = {
            let a1752 = v0;
            length_bytes(a1752)
        };
        let a1754 = 1u64;
        nat_add(a1753, a1754)?
    }, v1)?;
    Ok(h1751)
}

pub fn f418(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f419(v0: Bytes) -> R<u64> {
    let (h1755, _) = f286(Fn3::F464(v0.clone()), {
        let a1757 = {
            let a1756 = v0;
            length_bytes(a1756)
        };
        let a1758 = 1u64;
        nat_add(a1757, a1758)?
    }, 0u64)?;
    Ok(h1755)
}

pub fn f420(v0: Bytes) -> R<Option<u64>> {
    if f465(v0.clone())? {
        Ok(Some({
            let (_, h1759) = {
                let (h1760, _) = f466(Fn15::F467(v0.clone()), {
                    let a1761 = v0;
                    length_bytes(a1761)
                }, (0u64, 0u64))?;
                h1760
            };
            h1759
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f421(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f35(v1.clone(), 0u64, {
        let a1763 = {
            let a1762 = v1.clone();
            length_bytes(a1762)
        };
        let a1764 = f468(v1)?;
        nat_sub(a1763, a1764)
    });
    let v5: Bytes = {
        let a1765 = v0;
        let a1766 = v4.clone();
        append_bytes(a1765, a1766)
    };
    let v6: u64 = f419(v5.clone())?;
    let v7: Bytes = f35(v5.clone(), v6, {
        let a1768 = {
            let a1767 = v5;
            length_bytes(a1767)
        };
        let a1769 = v6;
        nat_sub(a1768, a1769)
    });
    let v8: u64 = {
        let a1770 = v4;
        length_bytes(a1770)
    };
    let v9: u64 = {
        let a1771 = v7.clone();
        length_bytes(a1771)
    };
    let a1772 = v9;
    let a1773 = 0u64;
    if nat_eq(a1772, a1773) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f418(v2, {
            let a1774 = v3;
            let a1775 = v8;
            nat_lt(a1774, a1775)
        }) {
            let v10: u64 = if v2 {
                let a1776 = v3;
                let a1777 = v8;
                nat_add(a1776, a1777)?
            } else {
                let a1778 = v8;
                let a1779 = v3;
                nat_sub(a1778, a1779)
            };
            let v11: u64 = f468(v7.clone())?;
            let a1780 = v10;
            let a1781 = v11;
            if nat_le(a1780, a1781) {
                Ok(Some((f35(v7, 0u64, {
                    let a1782 = v9;
                    let a1783 = v10;
                    nat_sub(a1782, a1783)
                }), 0u64)))
            } else {
                let a1786 = {
                    let a1784 = v10;
                    let a1785 = v11;
                    nat_sub(a1784, a1785)
                };
                let a1787 = 19u64;
                if nat_le(a1786, a1787) {
                    Ok(Some((f35(v7, 0u64, {
                        let a1788 = v9;
                        let a1789 = v11;
                        nat_sub(a1788, a1789)
                    }), {
                        let a1790 = v10;
                        let a1791 = v11;
                        nat_sub(a1790, a1791)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1792 = v3;
                let a1793 = v8;
                nat_sub(a1792, a1793)
            };
            let a1796 = {
                let a1794 = v9;
                let a1795 = v12;
                nat_add(a1794, a1795)?
            };
            let a1797 = 19u64;
            if nat_le(a1796, a1797) {
                Ok(Some(({
                    let a1798 = v7;
                    let a1799 = f454(v12);
                    append_bytes(a1798, a1799)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f422(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1800 = 0u64;
        let a1801 = v1;
        if nat_lt(a1800, a1801) {
            let a1802 = v1;
            let a1803 = 9223372036854775808u64;
            nat_le(a1802, a1803)
        } else {
            false
        }
    } else {
        let a1804 = v1;
        let a1805 = 9223372036854775807u64;
        nat_le(a1804, a1805)
    }
}

pub fn f423(p1808: List<Adt12>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1807 = p1808;
    {
        let mut m1807 = m1807;
        loop {
            let step = {
                let v0 = m1807;
                let m1806 = v0;
                match m1806.uncons() {
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
                Ok(next) => { m1807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1809 = v2;
    let a1810 = 0u64;
    if nat_eq(a1809, a1810) {
        Ok(f469(v0, 0u64))
    } else {
        let (_, h1811) = {
            let (h1812, _) = f353(Fn4::F470(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1812
        };
        Ok(h1811)
    }
}

pub fn f425() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f426(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1816 = {
        let a1814 = v0.clone();
        let a1815 = {
            let (h1813, _) = v1.clone();
            h1813
        };
        index_bytes(a1814, a1815)
    };
    match m1816 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1818 = {
                    let (h1817, _) = v1.clone();
                    h1817
                };
                let a1819 = 1u64;
                nat_add(a1818, a1819)?
            }, {
                let a1822 = {
                    let (_, h1820) = v1;
                    h1820
                };
                let a1823 = f471(v0, {
                    let (h1821, _) = v1;
                    h1821
                }, v2)?;
                append_bytes(a1822, a1823)
            })))
        }
    }
}

pub fn f427(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1824 = v0.clone();
    match m1824.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1827 = {
                let (h1825, _) = v2;
                h1825
            };
            let a1828 = {
                let (h1826, _) = v1.clone();
                h1826
            };
            if equal(a1827, a1828) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f428(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f472(f473(Fn16::F474, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1833 = {
        let (h1829, _) = f475(Fn17::F476, {
            let a1831 = {
                let a1830 = v0;
                length_list(a1830)
            };
            let a1832 = 1u64;
            nat_add(a1831, a1832)?
        }, v1)?;
        h1829
    };
    match m1833.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f429(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1838 = {
        let a1835 = f267({
            let (h1834, _) = v0;
            h1834
        })?;
        let a1836 = Bytes::lit(&[0x3a]);
        append_bytes(a1835, a1836)
    };
    let a1839 = {
        let (_, h1837) = v0;
        h1837
    };
    Ok(append_bytes(a1838, a1839))
}

pub fn f430(v0: Bytes, v1: u64) -> bool {
    let m1840 = f174(v0, v1);
    match m1840 {
        None => {
            false
        }
        Some(v2) => {
            f477(v2)
        }
    }
}

pub fn f431(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f345(f28({
        let (h1841, _) = v0;
        h1841
    }, {
        let (h1842, _) = v1;
        h1842
    }), f28({
        let (_, h1843) = v0;
        h1843
    }, {
        let (_, h1844) = v1;
        h1844
    }))
}

pub fn f432(v0: Fn12, p1848: List<List<(Bytes, ())>>, p1849: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1847 = (p1848, p1849);
    {
        let mut m1847 = m1847;
        loop {
            let step = {
                let (v1, v2) = m1847;
                let m1845 = v2;
                match m1845.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1846: Fn12 = v0.clone();
                            c1846.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f434(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1851 = {
        let a1850 = v0.clone();
        length_list(a1850)
    };
    let a1852 = 1u64;
    if nat_le(a1851, a1852) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f478(v0)?))
    }
}

pub fn f435(v0: Fn13, p1856: List<(Bytes, ())>, p1857: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1855 = (p1856, p1857);
    {
        let mut m1855 = m1855;
        loop {
            let step = {
                let (v1, v2) = m1855;
                let m1853 = v2;
                match m1853.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1854: Fn13 = v0.clone();
                            c1854.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f437(p1861: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1860 = p1861;
    {
        let mut m1860 = m1860;
        loop {
            let step = {
                let v0 = m1860;
                let m1858 = v0;
                match m1858.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f479(v2, v1.clone(), 0u64, {
                            let a1859 = v1.clone();
                            length_bytes(a1859)
                        })? {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: Adt1, v1: Bytes, v2: List<Adt13>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f121(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f51(v3.clone(), f121(f82(v0, f132())?, f8(Str::lit("at"))));
    let a1863 = {
        let a1862 = v4.clone();
        length_list(a1862)
    };
    let a1864 = 0u64;
    if nat_eq(a1863, a1864) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1867 = {
            let a1865 = v4;
            length_list(a1865)
        };
        let a1868 = {
            let a1866 = v3;
            length_list(a1866)
        };
        if nat_eq(a1867, a1868) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f480(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f439(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt13> = f323(f236(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt13> = f323(f236(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f98(f121(v3.clone(), f8(Str::lit("id"))), f121(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f121(f82(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f51(v5.clone(), f98(v6.clone(), f121(f82(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1870 = {
        let a1869 = v7.clone();
        length_list(a1869)
    };
    let a1871 = 0u64;
    if nat_eq(a1870, a1871) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f40({
            let a1874 = {
                let a1872 = v7;
                length_list(a1872)
            };
            let a1875 = {
                let a1873 = v5;
                length_list(a1873)
            };
            nat_eq(a1874, a1875)
        }) {
            Ok((false, f480(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f125(f98(f51(f481(v4.clone()), f52(f53(v0, f8(Str::lit("model/authorities.toml"))))?), f51(f482(v4), f98(v6, f121(v3, f8(Str::lit("id")))))))?;
            let a1877 = {
                let a1876 = v8.clone();
                length_list(a1876)
            };
            let a1878 = 0u64;
            if nat_eq(a1877, a1878) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f480(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f95(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f440(v0: u64) -> bool {
    let a1879 = v0;
    let a1880 = 47u64;
    nat_eq(a1879, a1880)
}

pub fn f441(v0: u64) -> bool {
    f418({
        let a1881 = v0;
        let a1882 = 95u64;
        nat_eq(a1881, a1882)
    }, f418({
        let a1883 = v0;
        let a1884 = 45u64;
        nat_eq(a1883, a1884)
    }, f418({
        let a1885 = v0;
        let a1886 = 32u64;
        nat_eq(a1885, a1886)
    }, f418({
        let a1887 = v0;
        let a1888 = 46u64;
        nat_eq(a1887, a1888)
    }, {
        let a1889 = v0;
        let a1890 = 47u64;
        nat_eq(a1889, a1890)
    }))))
}

pub fn f442(v0: u64) -> bool {
    f345({
        let a1891 = 97u64;
        let a1892 = v0;
        nat_le(a1891, a1892)
    }, {
        let a1893 = v0;
        let a1894 = 122u64;
        nat_le(a1893, a1894)
    })
}

pub fn f443(v0: u64) -> bool {
    f345({
        let a1895 = 65u64;
        let a1896 = v0;
        nat_le(a1895, a1896)
    }, {
        let a1897 = v0;
        let a1898 = 90u64;
        nat_le(a1897, a1898)
    })
}

pub fn f444(v0: u64) -> bool {
    f345({
        let a1899 = 48u64;
        let a1900 = v0;
        nat_le(a1899, a1900)
    }, {
        let a1901 = v0;
        let a1902 = 57u64;
        nat_le(a1901, a1902)
    })
}

pub fn f445(v0: List<(Bytes, Adt18)>, v1: List<(Bytes, Adt18)>) -> List<(Bytes, Adt18)> {
    let m1903 = v0;
    match m1903.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f445(v3, v1))
        }
    }
}

pub fn f446(v0: Option<Adt18>) -> List<(Bytes, Adt18)> {
    let m1904 = v0;
    match m1904 {
        None => {
            List::<(Bytes, Adt18)>::nil()
        }
        Some(v1) => {
            f483(v1)
        }
    }
}

pub fn f447(v0: u64, v1: List<(Bytes, Adt18)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt18)>, Bytes>> {
    let m1905 = v0;
    if m1905 == 0 {
        let m1906 = v1;
        match m1906.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::<(Bytes, Adt18)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt18)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1905 - 1;
        let m1907 = v1.clone();
        match m1907.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt18)>, Bytes>(List::<(Bytes, Adt18)>::nil()))
            }
            Some((_, _)) => {
                let m1908 = f484(v1.clone(), v2.clone(), v3.clone())?;
                match m1908 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt18)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f342(f448(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1911 = f447(v6, f485(v1, {
                            let (h1909, _) = v9.clone();
                            h1909
                        }), List::cons(f486({
                            let (h1910, _) = v9.clone();
                            h1910
                        })?, v2), v3)?;
                        match m1911 {
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

pub fn f448(v0: List<(Bytes, Adt18)>) -> R<List<Bytes>> {
    let m1912 = v0;
    match m1912.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f486({
                let (h1913, _) = v1;
                h1913
            })?, f448(v2)?))
        }
    }
}

pub fn f449(p1919: u64, p1920: List<Adt25>, p1921: List<Bytes>, p1922: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1918 = (p1919, (p1920, (p1921, p1922)));
    {
        let mut m1918 = m1918;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1918;
                let m1914 = v0;
                if m1914 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1914 - 1;
                    let m1915 = v1;
                    match m1915.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1916 = v5;
                            match m1916 {
                                Adt25::C0(v8, v9) => {
                                    if f28(v8.clone(), f487()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f41(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1917 = f489(v8.clone(), v9)?;
                                            match m1917 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f450(f490(v11.clone()), List::cons(Adt25::C1(f491(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f41(f487(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f487(), v2), List::cons(f488(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m1923 = v0;
    match m1923.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f450(v3, v1))
        }
    }
}

pub fn f451(v0: List<(Bytes, Adt18)>) -> R<List<Adt25>> {
    let m1924 = v0;
    match m1924.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f486({
                let (h1925, _) = v1;
                h1925
            })?, {
                let (_, h1926) = v1;
                h1926
            }), f451(v2)?))
        }
    }
}

pub fn f452(v0: List<Adt13>, v1: Bytes, v2: Bytes) -> List<Adt13> {
    let m1927 = v0;
    match m1927.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            if f28(f47(v3.clone(), v1.clone()), v2.clone()) {
                f452(v4, v1, v2)
            } else {
                List::cons(v3, f452(v4, v1, v2))
            }
        }
    }
}

pub fn f453(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1928 = v0;
    match m1928.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f72(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f342(f455(f125(List::cons(v1, f492(f33(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f453(v3, v1)?))
            }
        }
    }
}

pub fn f454(v0: u64) -> Bytes {
    let m1929 = v0;
    if m1929 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1929 - 1;
        let a1930 = f8(Str::lit("0"));
        let a1931 = f454(v1);
        append_bytes(a1930, a1931)
    }
}

pub fn f455(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1932 = v0;
    match m1932.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f306(v1)?, f455(v2)?))
        }
    }
}

pub fn f456(v0: u32) -> Bytes {
    let a1935 = {
        let a1933 = v0;
        let a1934 = 8u32;
        fixed_u32::bit_and(a1933, a1934)
    };
    let a1936 = 0u32;
    if equal(a1935, a1936) {
        let a1939 = {
            let a1937 = v0;
            let a1938 = 4u32;
            fixed_u32::bit_and(a1937, a1938)
        };
        let a1940 = 0u32;
        if equal(a1939, a1940) {
            let a1943 = {
                let a1941 = v0;
                let a1942 = 2u32;
                fixed_u32::bit_and(a1941, a1942)
            };
            let a1944 = 0u32;
            if equal(a1943, a1944) {
                let a1947 = {
                    let a1945 = v0;
                    let a1946 = 1u32;
                    fixed_u32::bit_and(a1945, a1946)
                };
                let a1948 = 0u32;
                if equal(a1947, a1948) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1951 = {
                    let a1949 = v0;
                    let a1950 = 1u32;
                    fixed_u32::bit_and(a1949, a1950)
                };
                let a1952 = 0u32;
                if equal(a1951, a1952) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1955 = {
                let a1953 = v0;
                let a1954 = 2u32;
                fixed_u32::bit_and(a1953, a1954)
            };
            let a1956 = 0u32;
            if equal(a1955, a1956) {
                let a1959 = {
                    let a1957 = v0;
                    let a1958 = 1u32;
                    fixed_u32::bit_and(a1957, a1958)
                };
                let a1960 = 0u32;
                if equal(a1959, a1960) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1963 = {
                    let a1961 = v0;
                    let a1962 = 1u32;
                    fixed_u32::bit_and(a1961, a1962)
                };
                let a1964 = 0u32;
                if equal(a1963, a1964) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1967 = {
            let a1965 = v0;
            let a1966 = 4u32;
            fixed_u32::bit_and(a1965, a1966)
        };
        let a1968 = 0u32;
        if equal(a1967, a1968) {
            let a1971 = {
                let a1969 = v0;
                let a1970 = 2u32;
                fixed_u32::bit_and(a1969, a1970)
            };
            let a1972 = 0u32;
            if equal(a1971, a1972) {
                let a1975 = {
                    let a1973 = v0;
                    let a1974 = 1u32;
                    fixed_u32::bit_and(a1973, a1974)
                };
                let a1976 = 0u32;
                if equal(a1975, a1976) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1979 = {
                    let a1977 = v0;
                    let a1978 = 1u32;
                    fixed_u32::bit_and(a1977, a1978)
                };
                let a1980 = 0u32;
                if equal(a1979, a1980) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1983 = {
                let a1981 = v0;
                let a1982 = 2u32;
                fixed_u32::bit_and(a1981, a1982)
            };
            let a1984 = 0u32;
            if equal(a1983, a1984) {
                let a1987 = {
                    let a1985 = v0;
                    let a1986 = 1u32;
                    fixed_u32::bit_and(a1985, a1986)
                };
                let a1988 = 0u32;
                if equal(a1987, a1988) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1991 = {
                    let a1989 = v0;
                    let a1990 = 1u32;
                    fixed_u32::bit_and(a1989, a1990)
                };
                let a1992 = 0u32;
                if equal(a1991, a1992) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f457(v0: u32, v1: u32) -> u32 {
    let m1995 = {
        let a1993 = v0;
        let a1994 = v1;
        fixed_u32::shift_right(a1993, a1994)
    };
    match m1995 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f458(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f493(Fn18::F494, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h1996, _, _, _, _, _, _, _) => {
            h1996
        }
    }, match v0.clone() {
        Adt24::C0(_, h1997, _, _, _, _, _, _) => {
            h1997
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h1998, _, _, _, _, _) => {
            h1998
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h1999, _, _, _, _) => {
            h1999
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2000, _, _, _) => {
            h2000
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2001, _, _) => {
            h2001
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2002, _) => {
            h2002
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2003) => {
            h2003
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f495(match v0.clone() {
        Adt24::C0(h2004, _, _, _, _, _, _, _) => {
            h2004
        }
    }, match v2.clone() {
        Adt27::C0(h2005, _, _, _, _, _, _, _, _, _) => {
            h2005
        }
    }), f495(match v0.clone() {
        Adt24::C0(_, h2006, _, _, _, _, _, _) => {
            h2006
        }
    }, match v2.clone() {
        Adt27::C0(_, h2007, _, _, _, _, _, _, _, _) => {
            h2007
        }
    }), f495(match v0.clone() {
        Adt24::C0(_, _, h2008, _, _, _, _, _) => {
            h2008
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2009, _, _, _, _, _, _, _) => {
            h2009
        }
    }), f495(match v0.clone() {
        Adt24::C0(_, _, _, h2010, _, _, _, _) => {
            h2010
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2011, _, _, _, _, _, _) => {
            h2011
        }
    }), f495(match v0.clone() {
        Adt24::C0(_, _, _, _, h2012, _, _, _) => {
            h2012
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2013, _, _, _, _, _) => {
            h2013
        }
    }), f495(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2014, _, _) => {
            h2014
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2015, _, _, _, _) => {
            h2015
        }
    }), f495(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2016, _) => {
            h2016
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2017, _, _, _) => {
            h2017
        }
    }), f495(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2018) => {
            h2018
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2019, _, _) => {
            h2019
        }
    })))
}

pub fn f459(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f496(v0.clone(), v1, v2, v3)?, f496(v0.clone(), v1, v2, {
        let a2020 = v3;
        let a2021 = 4u64;
        nat_add(a2020, a2021)?
    })?, f496(v0.clone(), v1, v2, {
        let a2022 = v3;
        let a2023 = 8u64;
        nat_add(a2022, a2023)?
    })?, f496(v0.clone(), v1, v2, {
        let a2024 = v3;
        let a2025 = 12u64;
        nat_add(a2024, a2025)?
    })?, f496(v0.clone(), v1, v2, {
        let a2026 = v3;
        let a2027 = 16u64;
        nat_add(a2026, a2027)?
    })?, f496(v0.clone(), v1, v2, {
        let a2028 = v3;
        let a2029 = 20u64;
        nat_add(a2028, a2029)?
    })?, f496(v0.clone(), v1, v2, {
        let a2030 = v3;
        let a2031 = 24u64;
        nat_add(a2030, a2031)?
    })?, f496(v0.clone(), v1, v2, {
        let a2032 = v3;
        let a2033 = 28u64;
        nat_add(a2032, a2033)?
    })?, f496(v0.clone(), v1, v2, {
        let a2034 = v3;
        let a2035 = 32u64;
        nat_add(a2034, a2035)?
    })?, f496(v0.clone(), v1, v2, {
        let a2036 = v3;
        let a2037 = 36u64;
        nat_add(a2036, a2037)?
    })?, f496(v0.clone(), v1, v2, {
        let a2038 = v3;
        let a2039 = 40u64;
        nat_add(a2038, a2039)?
    })?, f496(v0.clone(), v1, v2, {
        let a2040 = v3;
        let a2041 = 44u64;
        nat_add(a2040, a2041)?
    })?, f496(v0.clone(), v1, v2, {
        let a2042 = v3;
        let a2043 = 48u64;
        nat_add(a2042, a2043)?
    })?, f496(v0.clone(), v1, v2, {
        let a2044 = v3;
        let a2045 = 52u64;
        nat_add(a2044, a2045)?
    })?, f496(v0.clone(), v1, v2, {
        let a2046 = v3;
        let a2047 = 56u64;
        nat_add(a2046, a2047)?
    })?, f496(v0, v1, v2, {
        let a2048 = v3;
        let a2049 = 60u64;
        nat_add(a2048, a2049)?
    })?))
}

pub fn f460(p2052: List<Bytes>, v1: Bytes) -> bool {
    let m2051 = p2052;
    {
        let mut m2051 = m2051;
        loop {
            let step = {
                let v0 = m2051;
                let m2050 = v0;
                match m2050.uncons() {
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
                Ok(next) => { m2051 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: Fn14, p2057: u64, p2058: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2056 = (p2057, p2058);
    {
        let mut m2056 = m2056;
        loop {
            let step = {
                let (v1, v2) = m2056;
                let m2053 = v1;
                if m2053 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2053 - 1;
                    let m2055 = {
                        let c2054: Fn14 = v0.clone();
                        c2054.apply(v2.clone())?
                    };
                    match m2055 {
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
                Ok(next) => { m2056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f497(v0, v1)
}

pub fn f463(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f498(v0, v1)
}

pub fn f464(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f499(v0, v1)
}

pub fn f465(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2059 = v0.clone();
        length_bytes(a2059)
    };
    let a2060 = 0u64;
    let a2061 = v1;
    if nat_lt(a2060, a2061) {
        let a2062 = f500(v0.clone())?;
        let a2063 = v1;
        if nat_eq(a2062, a2063) {
            let a2064 = 1u64;
            let a2065 = v1;
            if nat_lt(a2064, a2065) {
                let m2068 = {
                    let a2066 = v0;
                    let a2067 = 0u64;
                    index_bytes(a2066, a2067)
                };
                match m2068 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2069 = v2;
                        let a2070 = 48u8;
                        Ok(!equal(a2069, a2070))
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

pub fn f466(v0: Fn15, p2075: u64, p2076: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2074 = (p2075, p2076);
    {
        let mut m2074 = m2074;
        loop {
            let step = {
                let (v1, v2) = m2074;
                let m2071 = v1;
                if m2071 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2071 - 1;
                    let m2073 = {
                        let c2072: Fn15 = v0.clone();
                        c2072.apply(v2)?
                    };
                    match m2073 {
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
                Ok(next) => { m2074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f501(v0, v1)
}

pub fn f468(v0: Bytes) -> R<u64> {
    let (h2077, _) = f286(Fn3::F502(v0.clone()), {
        let a2079 = {
            let a2078 = v0;
            length_bytes(a2078)
        };
        let a2080 = 1u64;
        nat_add(a2079, a2080)?
    }, 0u64)?;
    Ok(h2077)
}

pub fn f469(v0: Bytes, v1: u64) -> Bytes {
    let m2084 = {
        let a2081 = v0;
        let a2082 = v1;
        let a2083 = 1u64;
        slice_bytes(a2081, a2082, a2083)
    };
    match m2084 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f470(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f503(v1, v0, v2)
}

pub fn f471(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2129 = {
            let a2105 = {
                let a2093 = {
                    let a2087 = {
                        let a2085 = v3;
                        let a2086 = 128u8;
                        fixed_u8::bit_and(a2085, a2086)
                    };
                    let a2088 = 128u8;
                    if equal(a2087, a2088) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2094 = {
                    let a2091 = {
                        let a2089 = v3;
                        let a2090 = 64u8;
                        fixed_u8::bit_and(a2089, a2090)
                    };
                    let a2092 = 64u8;
                    if equal(a2091, a2092) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2093, a2094)?
            };
            let a2106 = {
                let a2103 = {
                    let a2097 = {
                        let a2095 = v3;
                        let a2096 = 32u8;
                        fixed_u8::bit_and(a2095, a2096)
                    };
                    let a2098 = 32u8;
                    if equal(a2097, a2098) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2104 = {
                    let a2101 = {
                        let a2099 = v3;
                        let a2100 = 16u8;
                        fixed_u8::bit_and(a2099, a2100)
                    };
                    let a2102 = 16u8;
                    if equal(a2101, a2102) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2103, a2104)?
            };
            nat_add(a2105, a2106)?
        };
        let a2130 = {
            let a2127 = {
                let a2115 = {
                    let a2109 = {
                        let a2107 = v3;
                        let a2108 = 8u8;
                        fixed_u8::bit_and(a2107, a2108)
                    };
                    let a2110 = 8u8;
                    if equal(a2109, a2110) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2116 = {
                    let a2113 = {
                        let a2111 = v3;
                        let a2112 = 4u8;
                        fixed_u8::bit_and(a2111, a2112)
                    };
                    let a2114 = 4u8;
                    if equal(a2113, a2114) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2115, a2116)?
            };
            let a2128 = {
                let a2125 = {
                    let a2119 = {
                        let a2117 = v3;
                        let a2118 = 2u8;
                        fixed_u8::bit_and(a2117, a2118)
                    };
                    let a2120 = 2u8;
                    if equal(a2119, a2120) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2126 = {
                    let a2123 = {
                        let a2121 = v3;
                        let a2122 = 1u8;
                        fixed_u8::bit_and(a2121, a2122)
                    };
                    let a2124 = 1u8;
                    if equal(a2123, a2124) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2125, a2126)?
            };
            nat_add(a2127, a2128)?
        };
        nat_add(a2129, a2130)?
    };
    let a2131 = v4;
    let a2132 = 34u64;
    if nat_eq(a2131, a2132) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2133 = v4;
        let a2134 = 92u64;
        if nat_eq(a2133, a2134) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2135 = v4;
            let a2136 = 8u64;
            if nat_eq(a2135, a2136) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2137 = v4;
                let a2138 = 12u64;
                if nat_eq(a2137, a2138) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2139 = v4;
                    let a2140 = 10u64;
                    if nat_eq(a2139, a2140) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2141 = v4;
                        let a2142 = 13u64;
                        if nat_eq(a2141, a2142) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2143 = v4;
                            let a2144 = 9u64;
                            if nat_eq(a2143, a2144) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2145 = v4;
                                let a2146 = 32u64;
                                if nat_lt(a2145, a2146) {
                                    let a2147 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2148 = f504(v2)?;
                                    Ok(append_bytes(a2147, a2148))
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

pub fn f472(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f505(Fn19::F506, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f473(v0: Fn16, p2152: List<List<(Bytes, Bytes)>>, p2153: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2151 = (p2152, p2153);
    {
        let mut m2151 = m2151;
        loop {
            let step = {
                let (v1, v2) = m2151;
                let m2149 = v2;
                match m2149.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2150: Fn16 = v0.clone();
                            c2150.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2151 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f475(v0: Fn17, p2158: u64, p2159: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2157 = (p2158, p2159);
    {
        let mut m2157 = m2157;
        loop {
            let step = {
                let (v1, v2) = m2157;
                let m2154 = v1;
                if m2154 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2154 - 1;
                    let m2156 = {
                        let c2155: Fn17 = v0.clone();
                        c2155.apply(v2.clone())?
                    };
                    match m2156 {
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
                Ok(next) => { m2157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f507(v0)
}

pub fn f477(v0: u8) -> bool {
    let a2160 = v0;
    let a2161 = 32u8;
    if if if equal(a2160, a2161) {
        true
    } else {
        let a2162 = v0;
        let a2163 = 9u8;
        equal(a2162, a2163)
    } {
        true
    } else {
        let a2164 = v0;
        let a2165 = 10u8;
        equal(a2164, a2165)
    } {
        true
    } else {
        let a2166 = v0;
        let a2167 = 12u8;
        if equal(a2166, a2167) {
            true
        } else {
            let a2168 = v0;
            let a2169 = 13u8;
            equal(a2168, a2169)
        }
    }
}

pub fn f478(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f382({
        let (_, h2170) = {
            let (h2171, _) = f508(Fn20::F509, {
                let a2173 = {
                    let a2172 = v0.clone();
                    length_list(a2172)
                };
                let a2174 = 1u64;
                nat_add(a2173, a2174)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2171
        };
        h2170
    }))
}

pub fn f479(v0: Bytes, v1: Bytes, p2187: u64, p2188: u64) -> R<bool> {
    let m2186 = (p2187, p2188);
    {
        let mut m2186 = m2186;
        loop {
            let step = {
                let (v2, v3) = m2186;
                let m2175 = v3;
                if m2175 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m2175 - 1;
                    let m2178 = {
                        let a2176 = v1.clone();
                        let a2177 = v2;
                        index_bytes(a2176, a2177)
                    };
                    match m2178 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m2181 = {
                                let a2179 = v0.clone();
                                let a2180 = v2;
                                index_bytes(a2179, a2180)
                            };
                            match m2181 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a2182 = v5;
                                    let a2183 = v6;
                                    if equal(a2182, a2183) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a2184 = v2;
                                            let a2185 = 1u64;
                                            nat_add(a2184, a2185)?
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
                Ok(next) => { m2186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f481(v0: List<Adt13>) -> List<Bytes> {
    let m2189 = v0;
    match m2189.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2190 = f79(v1, f8(Str::lit("std")));
            match m2190 {
                None => {
                    f481(v2)
                }
                Some(v3) => {
                    List::cons(f47(v3, f8(Str::lit("authority"))), f481(v2))
                }
            }
        }
    }
}

pub fn f482(v0: List<Adt13>) -> List<Bytes> {
    let m2191 = v0;
    match m2191.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2192 = f78(f79(v1, f8(Str::lit("req"))));
            match m2192 {
                None => {
                    f482(v2)
                }
                Some(v3) => {
                    List::cons(v3, f482(v2))
                }
            }
        }
    }
}

pub fn f483(v0: Adt18) -> List<(Bytes, Adt18)> {
    let m2193 = v0;
    match m2193 {
        Adt18::C0 => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C1(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C2(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C3(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C4(_, _, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C5(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C6(_) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C7(r2194) => {
            let v4 = (*r2194).clone();
            f483(v4)
        }
        Adt18::C8 => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C9(_, _) => {
            List::<(Bytes, Adt18)>::nil()
        }
        Adt18::C10(v1, r2195, r2196) => {
            let v2 = (*r2195).clone();
            let v3 = (*r2196).clone();
            List::cons((v1, v2), f483(v3))
        }
    }
}

pub fn f484(p2201: List<(Bytes, Adt18)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt18)>> {
    let m2200 = p2201;
    {
        let mut m2200 = m2200;
        loop {
            let step = {
                let v0 = m2200;
                let m2197 = v0;
                match m2197.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(Ok(None::<(Bytes, Adt18)>))
                    }
                    Some((v3, v4)) => {
                        if f510(f511({
                            let (_, h2198) = v3.clone();
                            h2198
                        }, List::<Bytes>::nil())?, f486({
                            let (h2199, _) = v3.clone();
                            h2199
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt18)>, R<Option<(Bytes, Adt18)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2200 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(v0: List<(Bytes, Adt18)>, v1: Bytes) -> List<(Bytes, Adt18)> {
    let m2202 = v0;
    match m2202.uncons() {
        None => {
            List::<(Bytes, Adt18)>::nil()
        }
        Some((v2, v3)) => {
            if f28({
                let (h2203, _) = v2.clone();
                h2203
            }, v1.clone()) {
                f485(v3, v1)
            } else {
                List::cons(v2, f485(v3, v1))
            }
        }
    }
}

pub fn f486(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f328(v0)?;
    if f41(v1.clone(), f512()) {
        f513(v1)
    } else {
        Ok(v1)
    }
}

pub fn f487() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f488() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f489(v0: Bytes, v1: Adt18) -> R<Result<Adt28, Bytes>> {
    f514(v0, f515(v1))
}

pub fn f490(v0: Adt28) -> List<Adt25> {
    let m2204 = v0;
    match m2204 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f491(v0: Adt28) -> Bytes {
    let m2205 = v0;
    match m2205 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f492(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2206 = v0;
    match m2206.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f492(v3, false))
            } else {
                f492(v3, true)
            }
        }
    }
}

pub fn f493(v0: Fn18, p2210: u64, p2211: Adt27) -> R<Adt27> {
    let m2209 = (p2210, p2211);
    {
        let mut m2209 = m2209;
        loop {
            let step = {
                let (v1, v2) = m2209;
                let m2207 = v1;
                if m2207 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2207 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2208: Fn18 = v0.clone();
                        c2208.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2209 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: Adt27) -> R<Adt27> {
    f516(v0)
}

pub fn f495(v0: u32, v1: u32) -> u32 {
    let m2214 = {
        let a2212 = f517(v0);
        let a2213 = f517(v1);
        fixed_u64::checked_add(a2212, a2213)
    };
    match m2214 {
        None => {
            0u32
        }
        Some(v2) => {
            f518(v2)
        }
    }
}

pub fn f496(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2217 = {
        let a2215 = v3;
        let a2216 = 8u64;
        nat_add(a2215, a2216)?
    };
    let a2218 = v2;
    if nat_eq(a2217, a2218) {
        Ok(f519({
            let a2221 = {
                let a2219 = v1;
                let a2220 = 8u64;
                nat_mul(a2219, a2220)?
            };
            let a2222 = 4294967296u64;
            let a2223 = 0u64;
            nat_quot(a2221, a2222, a2223)
        }))
    } else {
        let a2226 = {
            let a2224 = v3;
            let a2225 = 4u64;
            nat_add(a2224, a2225)?
        };
        let a2227 = v2;
        if nat_eq(a2226, a2227) {
            Ok(f519({
                let a2230 = {
                    let a2228 = v1;
                    let a2229 = 8u64;
                    nat_mul(a2228, a2229)?
                };
                let a2231 = 4294967296u64;
                let a2232 = 0u64;
                nat_rem(a2230, a2231, a2232)
            }))
        } else {
            Ok(f520(f521(v0.clone(), v1, v3), f521(v0.clone(), v1, {
                let a2233 = v3;
                let a2234 = 1u64;
                nat_add(a2233, a2234)?
            }), f521(v0.clone(), v1, {
                let a2235 = v3;
                let a2236 = 2u64;
                nat_add(a2235, a2236)?
            }), f521(v0, v1, {
                let a2237 = v3;
                let a2238 = 3u64;
                nat_add(a2237, a2238)?
            })))
        }
    }
}

pub fn f497(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h2239, _) = v1.clone();
        h2239
    };
    let v3: Bytes = {
        let (h2240, _) = {
            let (_, h2241) = v1.clone();
            h2241
        };
        h2240
    };
    let a2244 = {
        let (_, h2242) = {
            let (_, h2243) = v1;
            h2243
        };
        h2242
    };
    let a2245 = 0u64;
    if nat_eq(a2244, a2245) {
        let v4: u64 = f347(v0.clone(), v2)?;
        let a2246 = v4;
        let a2247 = 34u64;
        if nat_eq(a2246, a2247) {
            Ok(Some(({
                let a2248 = v2;
                let a2249 = 1u64;
                nat_add(a2248, a2249)?
            }, (v3, 1u64))))
        } else {
            let a2250 = v4;
            let a2251 = 92u64;
            if nat_eq(a2250, a2251) {
                f522(v0, v2, v3)
            } else {
                let a2252 = v4;
                let a2253 = 32u64;
                if if nat_lt(a2252, a2253) {
                    true
                } else {
                    let a2254 = v4;
                    let a2255 = 256u64;
                    nat_eq(a2254, a2255)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2256 = v2;
                        let a2257 = 1u64;
                        nat_add(a2256, a2257)?
                    }, ({
                        let a2258 = v3;
                        let a2259 = f35(v0, v2, 1u64);
                        append_bytes(a2258, a2259)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f498(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f347(v0, v1)?;
    let a2260 = 47u64;
    let a2261 = v2;
    if if nat_lt(a2260, a2261) {
        let a2262 = v2;
        let a2263 = 58u64;
        nat_lt(a2262, a2263)
    } else {
        false
    } {
        Ok(Some({
            let a2264 = v1;
            let a2265 = 1u64;
            nat_add(a2264, a2265)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f499(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2266 = f347(v0, v1)?;
    let a2267 = 48u64;
    if nat_eq(a2266, a2267) {
        Ok(Some({
            let a2268 = v1;
            let a2269 = 1u64;
            nat_add(a2268, a2269)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f500(v0: Bytes) -> R<u64> {
    let (h2270, _) = f286(Fn3::F523(v0.clone()), {
        let a2271 = v0;
        length_bytes(a2271)
    }, 0u64)?;
    Ok(h2270)
}

pub fn f501(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2275 = {
        let a2273 = v0;
        let a2274 = {
            let (h2272, _) = v1;
            h2272
        };
        index_bytes(a2273, a2274)
    };
    match m2275 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2277 = {
                    let (h2276, _) = v1;
                    h2276
                };
                let a2278 = 1u64;
                nat_add(a2277, a2278)?
            }, {
                let a2282 = {
                    let a2280 = {
                        let (_, h2279) = v1;
                        h2279
                    };
                    let a2281 = 10u64;
                    nat_mul(a2280, a2281)?
                };
                let a2283 = f524(v2)?;
                nat_add(a2282, a2283)?
            })))
        }
    }
}

pub fn f502(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f525(v0, v1)
}

pub fn f503(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2284, _) = v2;
        h2284
    };
    let a2285 = v3;
    let a2286 = 0u64;
    if nat_eq(a2285, a2286) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2287 = v3;
            let a2288 = v1;
            let a2289 = 0u64;
            nat_quot(a2287, a2288, a2289)
        }, {
            let a2294 = f469(v0, {
                let a2290 = v3;
                let a2291 = v1;
                let a2292 = 0u64;
                nat_rem(a2290, a2291, a2292)
            });
            let a2295 = {
                let (_, h2293) = v2;
                h2293
            };
            append_bytes(a2294, a2295)
        }))
    }
}

pub fn f504(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2340 = {
            let a2316 = {
                let a2304 = {
                    let a2298 = {
                        let a2296 = v1;
                        let a2297 = 128u8;
                        fixed_u8::bit_and(a2296, a2297)
                    };
                    let a2299 = 128u8;
                    if equal(a2298, a2299) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2305 = {
                    let a2302 = {
                        let a2300 = v1;
                        let a2301 = 64u8;
                        fixed_u8::bit_and(a2300, a2301)
                    };
                    let a2303 = 64u8;
                    if equal(a2302, a2303) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2304, a2305)?
            };
            let a2317 = {
                let a2314 = {
                    let a2308 = {
                        let a2306 = v1;
                        let a2307 = 32u8;
                        fixed_u8::bit_and(a2306, a2307)
                    };
                    let a2309 = 32u8;
                    if equal(a2308, a2309) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2315 = {
                    let a2312 = {
                        let a2310 = v1;
                        let a2311 = 16u8;
                        fixed_u8::bit_and(a2310, a2311)
                    };
                    let a2313 = 16u8;
                    if equal(a2312, a2313) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2314, a2315)?
            };
            nat_add(a2316, a2317)?
        };
        let a2341 = {
            let a2338 = {
                let a2326 = {
                    let a2320 = {
                        let a2318 = v1;
                        let a2319 = 8u8;
                        fixed_u8::bit_and(a2318, a2319)
                    };
                    let a2321 = 8u8;
                    if equal(a2320, a2321) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2327 = {
                    let a2324 = {
                        let a2322 = v1;
                        let a2323 = 4u8;
                        fixed_u8::bit_and(a2322, a2323)
                    };
                    let a2325 = 4u8;
                    if equal(a2324, a2325) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2326, a2327)?
            };
            let a2339 = {
                let a2336 = {
                    let a2330 = {
                        let a2328 = v1;
                        let a2329 = 2u8;
                        fixed_u8::bit_and(a2328, a2329)
                    };
                    let a2331 = 2u8;
                    if equal(a2330, a2331) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2337 = {
                    let a2334 = {
                        let a2332 = v1;
                        let a2333 = 1u8;
                        fixed_u8::bit_and(a2332, a2333)
                    };
                    let a2335 = 1u8;
                    if equal(a2334, a2335) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2336, a2337)?
            };
            nat_add(a2338, a2339)?
        };
        nat_add(a2340, a2341)?
    };
    let a2348 = f469(f526(), {
        let a2342 = v2;
        let a2343 = 16u64;
        let a2344 = 0u64;
        nat_quot(a2342, a2343, a2344)
    });
    let a2349 = f469(f526(), {
        let a2345 = v2;
        let a2346 = 16u64;
        let a2347 = 0u64;
        nat_rem(a2345, a2346, a2347)
    });
    Ok(append_bytes(a2348, a2349))
}

pub fn f505(v0: Fn19, p2353: List<List<(Bytes, Bytes)>>, p2354: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2352 = (p2353, p2354);
    {
        let mut m2352 = m2352;
        loop {
            let step = {
                let (v1, v2) = m2352;
                let m2350 = v2;
                match m2350.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2351: Fn19 = v0.clone();
                            c2351.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2352 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f507(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2356 = {
        let a2355 = v0.clone();
        length_list(a2355)
    };
    let a2357 = 1u64;
    if nat_le(a2356, a2357) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f527(v0)?))
    }
}

pub fn f508(v0: Fn20, p2362: u64, p2363: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2361 = (p2362, p2363);
    {
        let mut m2361 = m2361;
        loop {
            let step = {
                let (v1, v2) = m2361;
                let m2358 = v1;
                if m2358 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2358 - 1;
                    let m2360 = {
                        let c2359: Fn20 = v0.clone();
                        c2359.apply(v2.clone())?
                    };
                    match m2360 {
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
                Ok(next) => { m2361 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f528(v0)
}

pub fn f510(p2366: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2365 = p2366;
    {
        let mut m2365 = m2365;
        loop {
            let step = {
                let v0 = m2365;
                let m2364 = v0;
                match m2364.uncons() {
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
                Ok(next) => { m2365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: Adt18, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2367 = v0;
    match m2367 {
        Adt18::C0 => {
            Ok(v1)
        }
        Adt18::C1(_, _) => {
            Ok(v1)
        }
        Adt18::C2(_) => {
            Ok(v1)
        }
        Adt18::C3(_, _) => {
            Ok(v1)
        }
        Adt18::C4(_, _, _) => {
            Ok(v1)
        }
        Adt18::C5(_) => {
            Ok(v1)
        }
        Adt18::C6(r2370) => {
            let v2 = (*r2370).clone();
            f511(v2, v1)
        }
        Adt18::C7(r2371) => {
            let v3 = (*r2371).clone();
            f511(v3, v1)
        }
        Adt18::C8 => {
            Ok(v1)
        }
        Adt18::C9(r2372, r2373) => {
            let v4 = (*r2372).clone();
            let v5 = (*r2373).clone();
            f511(v4, f511(v5, v1)?)
        }
        Adt18::C10(v6, r2374, r2375) => {
            let v7 = (*r2374).clone();
            let v8 = (*r2375).clone();
            let v9: List<Bytes> = f511(v7.clone(), f511(v8, v1)?)?;
            if f28(v6, f8(Str::lit("$ref"))) {
                let m2368 = f334(Some(v7));
                match m2368 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2369 = f529(v10)?;
                        match m2369 {
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

pub fn f512() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f513(v0: Bytes) -> R<Bytes> {
    let m2378 = {
        let a2376 = v0.clone();
        let a2377 = 0u64;
        index_bytes(a2376, a2377)
    };
    match m2378 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2384 = {
                let a2379 = f8(Str::lit("u"));
                let a2380 = f530(8u64, f62(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2379, a2380)
            };
            let a2385 = f35(v0.clone(), 1u64, {
                let a2382 = {
                    let a2381 = v0;
                    length_bytes(a2381)
                };
                let a2383 = 1u64;
                nat_sub(a2382, a2383)
            });
            Ok(append_bytes(a2384, a2385))
        }
    }
}

pub fn f514(v0: Bytes, v1: Adt18) -> R<Result<Adt28, Bytes>> {
    let m2386 = f242(v1.clone(), f8(Str::lit("enum")));
    match m2386 {
        None => {
            let m2388 = f242(v1.clone(), f8(Str::lit("oneOf")));
            match m2388 {
                None => {
                    if f535(v1.clone(), v0.clone())? {
                        let m2389 = f536(v0.clone(), f446(f242(v1.clone(), f8(Str::lit("properties")))), f246(f247(f242(v1, f8(Str::lit("required"))))), true)?;
                        match m2389 {
                            Ok(v7) => {
                                Ok(f533(f537(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f342(f538(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f539(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2390 = f446(f242(v1.clone(), f8(Str::lit("properties"))));
                        match m2390.uncons() {
                            None => {
                                if f540(v1.clone()) {
                                    Ok(f533(f541(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2393 = f542(v1, {
                                        let a2391 = v0.clone();
                                        let a2392 = f8(Str::lit("Value"));
                                        append_bytes(a2391, a2392)
                                    })?;
                                    match m2393 {
                                        Ok(v9) => {
                                            Ok(f533(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f491(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f490(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2394 = f536(v0.clone(), f446(f242(v1.clone(), f8(Str::lit("properties")))), f246(f247(f242(v1, f8(Str::lit("required"))))), false)?;
                                match m2394 {
                                    Ok(v13) => {
                                        Ok(f533(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2395 = f538(v13.clone());
                                            match m2395.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f342(f538(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f539(v13)))
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
                    f534(v0, f335(v5))
                }
            }
        }
        Some(v2) => {
            let m2387 = f531(f335(v2), List::<Bytes>::nil())?;
            match m2387 {
                Ok(v4) => {
                    Ok(f533(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f342(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f532(v3, v0))
                }
            }
        }
    }
}

pub fn f515(v0: Adt18) -> Adt18 {
    let v2: Adt18 = {
        let m2396 = f242(v0.clone(), f8(Str::lit("anyOf")));
        match m2396 {
            None => {
                v0
            }
            Some(v1) => {
                if f543(f335(v1.clone())) {
                    f544(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f545(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f546(f335(v1.clone()))
                    } {
                        v0
                    } else {
                        f547(f544(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt18 = {
        let m2397 = f242(v2.clone(), f8(Str::lit("oneOf")));
        match m2397 {
            None => {
                v2
            }
            Some(v3) => {
                if f543(f335(v3)) {
                    f544(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2398 = f242(v4.clone(), f8(Str::lit("allOf")));
    match m2398 {
        None => {
            v4
        }
        Some(v5) => {
            if f548(f335(v5)) {
                f544(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f516(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f549(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2399) => {
            h2399
        }
    });
    let v2: u32 = f495(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2400, _, _) => {
            h2400
        }
    }, f495(f550(match v0.clone() {
        Adt27::C0(_, _, _, _, h2401, _, _, _, _, _) => {
            h2401
        }
    }), f495(f551(match v0.clone() {
        Adt27::C0(_, _, _, _, h2402, _, _, _, _, _) => {
            h2402
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2403, _, _, _, _) => {
            h2403
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2404, _, _, _) => {
            h2404
        }
    }), f495(v1, {
        let m2407 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2406, _) => {
                h2406
            }
        };
        match m2407 {
            Adt26::C0(h2405, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2405
            }
        }
    }))));
    let v3: u32 = f495(f552(match v0.clone() {
        Adt27::C0(h2408, _, _, _, _, _, _, _, _, _) => {
            h2408
        }
    }), f553(match v0.clone() {
        Adt27::C0(h2409, _, _, _, _, _, _, _, _, _) => {
            h2409
        }
    }, match v0.clone() {
        Adt27::C0(_, h2410, _, _, _, _, _, _, _, _) => {
            h2410
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2411, _, _, _, _, _, _, _) => {
            h2411
        }
    }));
    Ok(Adt27::C0(f495(v2, v3), match v0.clone() {
        Adt27::C0(h2412, _, _, _, _, _, _, _, _, _) => {
            h2412
        }
    }, match v0.clone() {
        Adt27::C0(_, h2413, _, _, _, _, _, _, _, _) => {
            h2413
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2414, _, _, _, _, _, _, _) => {
            h2414
        }
    }, f495(match v0.clone() {
        Adt27::C0(_, _, _, h2415, _, _, _, _, _, _) => {
            h2415
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2416, _, _, _, _, _) => {
            h2416
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2417, _, _, _, _) => {
            h2417
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2418, _, _, _) => {
            h2418
        }
    }, f554(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2419, _) => {
            h2419
        }
    }), {
        let a2421 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2420) => {
                h2420
            }
        };
        let a2422 = 1u64;
        nat_add(a2421, a2422)?
    }))
}

pub fn f517(v0: u32) -> u64 {
    let m2424 = {
        let a2423 = v0;
        fixed_u64::convert(i128::from(a2423))
    };
    match m2424 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f518(v0: u64) -> u32 {
    let m2428 = {
        let a2427 = {
            let a2425 = v0;
            let a2426 = 4294967295u64;
            fixed_u64::bit_and(a2425, a2426)
        };
        fixed_u32::convert(i128::from(a2427))
    };
    match m2428 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f519(v0: u64) -> u32 {
    let (_, h2429) = {
        let (_, h2430) = f555(Fn21::F556, 32u64, (v0, (1u32, 0u32)));
        h2430
    };
    h2429
}

pub fn f520(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2435 = f557(v0, 24u32);
    let a2436 = {
        let a2433 = f557(v1, 16u32);
        let a2434 = {
            let a2431 = f557(v2, 8u32);
            let a2432 = v3;
            fixed_u32::bit_or(a2431, a2432)
        };
        fixed_u32::bit_or(a2433, a2434)
    };
    fixed_u32::bit_or(a2435, a2436)
}

pub fn f521(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2437 = v2;
    let a2438 = v1;
    if nat_lt(a2437, a2438) {
        let m2441 = {
            let a2439 = v0;
            let a2440 = v2;
            index_bytes(a2439, a2440)
        };
        match m2441 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2443 = {
                    let a2442 = v3;
                    fixed_u32::convert(i128::from(a2442))
                };
                match m2443 {
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
        let a2444 = v2;
        let a2445 = v1;
        if nat_eq(a2444, a2445) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f522(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f347(v0.clone(), {
        let a2446 = v1;
        let a2447 = 1u64;
        nat_add(a2446, a2447)?
    })?;
    let v4: Bytes = {
        let a2448 = v3;
        let a2449 = 117u64;
        if nat_eq(a2448, a2449) {
            f558(v0, v1)?
        } else {
            f559(v3)
        }
    };
    let a2450 = v4.clone();
    let a2451 = Bytes::lit(&[]);
    if equal(a2450, a2451) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2452 = v3;
            let a2453 = 117u64;
            if nat_eq(a2452, a2453) {
                let a2454 = v1;
                let a2455 = 6u64;
                nat_add(a2454, a2455)?
            } else {
                let a2456 = v1;
                let a2457 = 2u64;
                nat_add(a2456, a2457)?
            }
        }, ({
            let a2458 = v2;
            let a2459 = v4;
            append_bytes(a2458, a2459)
        }, 0u64))))
    }
}

pub fn f523(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2462 = {
        let a2460 = v0;
        let a2461 = v1;
        index_bytes(a2460, a2461)
    };
    match m2462 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f560(v2)? {
                Ok(Some({
                    let a2463 = v1;
                    let a2464 = 1u64;
                    nat_add(a2463, a2464)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f524(v0: u8) -> R<u64> {
    let a2511 = {
        let v1: u8 = v0;
        let a2509 = {
            let a2485 = {
                let a2473 = {
                    let a2467 = {
                        let a2465 = v1;
                        let a2466 = 128u8;
                        fixed_u8::bit_and(a2465, a2466)
                    };
                    let a2468 = 128u8;
                    if equal(a2467, a2468) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2474 = {
                    let a2471 = {
                        let a2469 = v1;
                        let a2470 = 64u8;
                        fixed_u8::bit_and(a2469, a2470)
                    };
                    let a2472 = 64u8;
                    if equal(a2471, a2472) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2473, a2474)?
            };
            let a2486 = {
                let a2483 = {
                    let a2477 = {
                        let a2475 = v1;
                        let a2476 = 32u8;
                        fixed_u8::bit_and(a2475, a2476)
                    };
                    let a2478 = 32u8;
                    if equal(a2477, a2478) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2484 = {
                    let a2481 = {
                        let a2479 = v1;
                        let a2480 = 16u8;
                        fixed_u8::bit_and(a2479, a2480)
                    };
                    let a2482 = 16u8;
                    if equal(a2481, a2482) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2483, a2484)?
            };
            nat_add(a2485, a2486)?
        };
        let a2510 = {
            let a2507 = {
                let a2495 = {
                    let a2489 = {
                        let a2487 = v1;
                        let a2488 = 8u8;
                        fixed_u8::bit_and(a2487, a2488)
                    };
                    let a2490 = 8u8;
                    if equal(a2489, a2490) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2496 = {
                    let a2493 = {
                        let a2491 = v1;
                        let a2492 = 4u8;
                        fixed_u8::bit_and(a2491, a2492)
                    };
                    let a2494 = 4u8;
                    if equal(a2493, a2494) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2495, a2496)?
            };
            let a2508 = {
                let a2505 = {
                    let a2499 = {
                        let a2497 = v1;
                        let a2498 = 2u8;
                        fixed_u8::bit_and(a2497, a2498)
                    };
                    let a2500 = 2u8;
                    if equal(a2499, a2500) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2506 = {
                    let a2503 = {
                        let a2501 = v1;
                        let a2502 = 1u8;
                        fixed_u8::bit_and(a2501, a2502)
                    };
                    let a2504 = 1u8;
                    if equal(a2503, a2504) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2505, a2506)?
            };
            nat_add(a2507, a2508)?
        };
        nat_add(a2509, a2510)?
    };
    let a2512 = 48u64;
    Ok(nat_sub(a2511, a2512))
}

pub fn f525(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2513 = v0.clone();
        length_bytes(a2513)
    };
    let a2514 = v1;
    let a2515 = v2;
    if if nat_lt(a2514, a2515) {
        let a2520 = f347(v0, {
            let a2518 = v2;
            let a2519 = {
                let a2516 = v1;
                let a2517 = 1u64;
                nat_add(a2516, a2517)?
            };
            nat_sub(a2518, a2519)
        })?;
        let a2521 = 48u64;
        nat_eq(a2520, a2521)
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

pub fn f526() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f527(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f472({
        let (_, h2524) = {
            let (h2525, _) = f561(Fn22::F562, {
                let a2527 = {
                    let a2526 = v0.clone();
                    length_list(a2526)
                };
                let a2528 = 1u64;
                nat_add(a2527, a2528)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2525
        };
        h2524
    }))
}

pub fn f528(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2529, _) = v0;
        h2529
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2530) = v0;
        h2530
    };
    let m2531 = v1;
    match m2531.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2532 = v4;
            match m2532.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f563(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f529(v0: Bytes) -> R<Option<Bytes>> {
    if f72(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f486(f35(v0.clone(), 8u64, {
            let a2534 = {
                let a2533 = v0;
                length_bytes(a2533)
            };
            let a2535 = 8u64;
            nat_sub(a2534, a2535)
        }))?))
    } else {
        if f72(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f486(f35(v0.clone(), 14u64, {
                let a2537 = {
                    let a2536 = v0;
                    length_bytes(a2536)
                };
                let a2538 = 14u64;
                nat_sub(a2537, a2538)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f530(p2555: u64, p2556: u64, p2557: u64, p2558: Bytes) -> Bytes {
    let m2554 = (p2555, (p2556, (p2557, p2558)));
    {
        let mut m2554 = m2554;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2554;
                let m2539 = v0;
                if m2539 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2539 - 1;
                    if f345({
                        let a2540 = v1;
                        let a2541 = 0u64;
                        nat_eq(a2540, a2541)
                    }, {
                        let a2542 = v2;
                        let a2543 = 0u64;
                        nat_eq(a2542, a2543)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2544 = v1;
                            let a2545 = 16u64;
                            let a2546 = 0u64;
                            nat_quot(a2544, a2545, a2546)
                        }, ({
                            let a2547 = v2;
                            let a2548 = 1u64;
                            nat_sub(a2547, a2548)
                        }, {
                            let a2552 = f564({
                                let a2549 = v1;
                                let a2550 = 16u64;
                                let a2551 = 0u64;
                                nat_rem(a2549, a2550, a2551)
                            });
                            let a2553 = v3;
                            append_bytes(a2552, a2553)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f531(v0: List<Adt18>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f565(f566(f246(v0.clone()))?, v0, v1)
}

pub fn f532(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f533(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f534(v0: Bytes, v1: List<Adt18>) -> R<Result<Adt28, Bytes>> {
    let m2559 = v1.clone();
    match m2559.uncons() {
        None => {
            Ok(f532(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2560 = f567(f446(f242(v2, f8(Str::lit("properties")))), v1.clone());
            match m2560.uncons() {
                None => {
                    let m2561 = f568(f566(f569(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2561 {
                        Ok(v5) => {
                            Ok(f533(f537(v0, f538(v5.clone())), f539(v5)))
                        }
                        Err(v4) => {
                            Ok(f532(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2562 = f570(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2562 {
                        Ok(v9) => {
                            Ok(f533(f537(v0, f538(v9.clone())), f539(v9)))
                        }
                        Err(v8) => {
                            Ok(f532(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f535(v0: Adt18, v1: Bytes) -> R<bool> {
    let m2563 = v0;
    match m2563 {
        Adt18::C0 => {
            Ok(false)
        }
        Adt18::C1(_, _) => {
            Ok(false)
        }
        Adt18::C2(_) => {
            Ok(false)
        }
        Adt18::C3(_, _) => {
            Ok(false)
        }
        Adt18::C4(_, _, _) => {
            Ok(false)
        }
        Adt18::C5(_) => {
            Ok(false)
        }
        Adt18::C6(r2564) => {
            let v2 = (*r2564).clone();
            f535(v2, v1)
        }
        Adt18::C7(r2565) => {
            let v3 = (*r2565).clone();
            f535(v3, v1)
        }
        Adt18::C8 => {
            Ok(false)
        }
        Adt18::C9(r2566, r2567) => {
            let v4 = (*r2566).clone();
            let v5 = (*r2567).clone();
            if f535(v4, v1.clone())? {
                Ok(true)
            } else {
                f535(v5, v1)
            }
        }
        Adt18::C10(v6, r2568, r2569) => {
            let v7 = (*r2568).clone();
            let v8 = (*r2569).clone();
            if if f28(v6, f8(Str::lit("$ref"))) {
                f571(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f535(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f535(v8, v1)
                }
            }
        }
    }
}

pub fn f536(v0: Bytes, v1: List<(Bytes, Adt18)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f572(f566(f573(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f537(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f342(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f538(v0: Adt29) -> List<Bytes> {
    let m2570 = v0;
    match m2570 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f539(v0: Adt29) -> List<Adt25> {
    let m2571 = v0;
    match m2571 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f540(v0: Adt18) -> bool {
    let m2572 = f446(f242(v0.clone(), f8(Str::lit("properties"))));
    match m2572.uncons() {
        None => {
            let m2573 = f574(v0);
            match m2573 {
                None => {
                    false
                }
                Some(v3) => {
                    f575(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f541(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f542(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2574 = f576(v0.clone());
    match m2574 {
        None => {
            if f28(f245(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2575 = f242(v0, f8(Str::lit("items")));
                match m2575 {
                    None => {
                        Ok(f577(f8(Str::lit("List")), f579()))
                    }
                    Some(v3) => {
                        Ok(f577(f8(Str::lit("List")), f578(v3, {
                            let a2576 = v1;
                            let a2577 = f8(Str::lit("Item"));
                            append_bytes(a2576, a2577)
                        })?))
                    }
                }
            } else {
                f578(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f577(f8(Str::lit("Option")), f578(v2, v1)?))
        }
    }
}

pub fn f543(p2580: List<Adt18>) -> bool {
    let m2579 = p2580;
    {
        let mut m2579 = m2579;
        loop {
            let step = {
                let v0 = m2579;
                let m2578 = v0;
                match m2578.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f580(f483(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt18>, bool>(v2)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544(v0: Adt18, v1: Bytes) -> Adt18 {
    let m2581 = v0;
    match m2581 {
        Adt18::C0 => {
            Adt18::C0
        }
        Adt18::C1(v6, r2582) => {
            let v7 = (*r2582).clone();
            Adt18::C1(v6, std::rc::Rc::new(v7))
        }
        Adt18::C2(v8) => {
            Adt18::C2(v8)
        }
        Adt18::C3(v12, v13) => {
            Adt18::C3(v12, v13)
        }
        Adt18::C4(v9, v10, v11) => {
            Adt18::C4(v9, v10, v11)
        }
        Adt18::C5(v14) => {
            Adt18::C5(v14)
        }
        Adt18::C6(r2583) => {
            let v15 = (*r2583).clone();
            Adt18::C6(std::rc::Rc::new(v15))
        }
        Adt18::C7(r2584) => {
            let v5 = (*r2584).clone();
            Adt18::C7(std::rc::Rc::new(f544(v5, v1)))
        }
        Adt18::C8 => {
            Adt18::C8
        }
        Adt18::C9(r2585, r2586) => {
            let v16 = (*r2585).clone();
            let v17 = (*r2586).clone();
            Adt18::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt18::C10(v2, r2587, r2588) => {
            let v3 = (*r2587).clone();
            let v4 = (*r2588).clone();
            if f28(v2.clone(), v1.clone()) {
                f544(v4, v1)
            } else {
                Adt18::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f544(v4, v1)))
            }
        }
    }
}

pub fn f545(v0: Adt18, v1: Bytes) -> bool {
    let m2589 = f242(v0, v1);
    match m2589 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f546(v0: List<Adt18>) -> bool {
    let m2590 = v0.clone();
    match m2590.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2591 = f567(f446(f242(v1, f8(Str::lit("properties")))), v0.clone());
            match m2591.uncons() {
                None => {
                    f582(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f581(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f547(v0: Adt18, v1: Bytes, v2: Adt18) -> Adt18 {
    let m2592 = v0;
    match m2592 {
        Adt18::C0 => {
            Adt18::C0
        }
        Adt18::C1(v7, r2593) => {
            let v8 = (*r2593).clone();
            Adt18::C1(v7, std::rc::Rc::new(v8))
        }
        Adt18::C2(v9) => {
            Adt18::C2(v9)
        }
        Adt18::C3(v13, v14) => {
            Adt18::C3(v13, v14)
        }
        Adt18::C4(v10, v11, v12) => {
            Adt18::C4(v10, v11, v12)
        }
        Adt18::C5(v15) => {
            Adt18::C5(v15)
        }
        Adt18::C6(r2594) => {
            let v16 = (*r2594).clone();
            Adt18::C6(std::rc::Rc::new(v16))
        }
        Adt18::C7(r2595) => {
            let v3 = (*r2595).clone();
            Adt18::C7(std::rc::Rc::new(Adt18::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt18::C8 => {
            Adt18::C8
        }
        Adt18::C9(r2596, r2597) => {
            let v17 = (*r2596).clone();
            let v18 = (*r2597).clone();
            Adt18::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt18::C10(v4, r2598, r2599) => {
            let v5 = (*r2598).clone();
            let v6 = (*r2599).clone();
            Adt18::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f548(p2602: List<Adt18>) -> bool {
    let m2601 = p2602;
    {
        let mut m2601 = m2601;
        loop {
            let step = {
                let v0 = m2601;
                let m2600 = v0;
                match m2600.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f583(v1) {
                            Ok::<List<Adt18>, bool>(v2)
                        } else {
                            Err::<List<Adt18>, bool>(false)
                        }
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

pub fn f549(v0: u64) -> u32 {
    let a2603 = v0;
    let a2604 = 32u64;
    if nat_lt(a2603, a2604) {
        let a2605 = v0;
        let a2606 = 16u64;
        if nat_lt(a2605, a2606) {
            let a2607 = v0;
            let a2608 = 8u64;
            if nat_lt(a2607, a2608) {
                let a2609 = v0;
                let a2610 = 4u64;
                if nat_lt(a2609, a2610) {
                    let a2611 = v0;
                    let a2612 = 2u64;
                    if nat_lt(a2611, a2612) {
                        let a2613 = v0;
                        let a2614 = 1u64;
                        if nat_lt(a2613, a2614) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2615 = v0;
                        let a2616 = 3u64;
                        if nat_lt(a2615, a2616) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2617 = v0;
                    let a2618 = 6u64;
                    if nat_lt(a2617, a2618) {
                        let a2619 = v0;
                        let a2620 = 5u64;
                        if nat_lt(a2619, a2620) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2621 = v0;
                        let a2622 = 7u64;
                        if nat_lt(a2621, a2622) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2623 = v0;
                let a2624 = 12u64;
                if nat_lt(a2623, a2624) {
                    let a2625 = v0;
                    let a2626 = 10u64;
                    if nat_lt(a2625, a2626) {
                        let a2627 = v0;
                        let a2628 = 9u64;
                        if nat_lt(a2627, a2628) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2629 = v0;
                        let a2630 = 11u64;
                        if nat_lt(a2629, a2630) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2631 = v0;
                    let a2632 = 14u64;
                    if nat_lt(a2631, a2632) {
                        let a2633 = v0;
                        let a2634 = 13u64;
                        if nat_lt(a2633, a2634) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2635 = v0;
                        let a2636 = 15u64;
                        if nat_lt(a2635, a2636) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2637 = v0;
            let a2638 = 24u64;
            if nat_lt(a2637, a2638) {
                let a2639 = v0;
                let a2640 = 20u64;
                if nat_lt(a2639, a2640) {
                    let a2641 = v0;
                    let a2642 = 18u64;
                    if nat_lt(a2641, a2642) {
                        let a2643 = v0;
                        let a2644 = 17u64;
                        if nat_lt(a2643, a2644) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2645 = v0;
                        let a2646 = 19u64;
                        if nat_lt(a2645, a2646) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2647 = v0;
                    let a2648 = 22u64;
                    if nat_lt(a2647, a2648) {
                        let a2649 = v0;
                        let a2650 = 21u64;
                        if nat_lt(a2649, a2650) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2651 = v0;
                        let a2652 = 23u64;
                        if nat_lt(a2651, a2652) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2653 = v0;
                let a2654 = 28u64;
                if nat_lt(a2653, a2654) {
                    let a2655 = v0;
                    let a2656 = 26u64;
                    if nat_lt(a2655, a2656) {
                        let a2657 = v0;
                        let a2658 = 25u64;
                        if nat_lt(a2657, a2658) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2659 = v0;
                        let a2660 = 27u64;
                        if nat_lt(a2659, a2660) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2661 = v0;
                    let a2662 = 30u64;
                    if nat_lt(a2661, a2662) {
                        let a2663 = v0;
                        let a2664 = 29u64;
                        if nat_lt(a2663, a2664) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2665 = v0;
                        let a2666 = 31u64;
                        if nat_lt(a2665, a2666) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2667 = v0;
        let a2668 = 48u64;
        if nat_lt(a2667, a2668) {
            let a2669 = v0;
            let a2670 = 40u64;
            if nat_lt(a2669, a2670) {
                let a2671 = v0;
                let a2672 = 36u64;
                if nat_lt(a2671, a2672) {
                    let a2673 = v0;
                    let a2674 = 34u64;
                    if nat_lt(a2673, a2674) {
                        let a2675 = v0;
                        let a2676 = 33u64;
                        if nat_lt(a2675, a2676) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2677 = v0;
                        let a2678 = 35u64;
                        if nat_lt(a2677, a2678) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2679 = v0;
                    let a2680 = 38u64;
                    if nat_lt(a2679, a2680) {
                        let a2681 = v0;
                        let a2682 = 37u64;
                        if nat_lt(a2681, a2682) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2683 = v0;
                        let a2684 = 39u64;
                        if nat_lt(a2683, a2684) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2685 = v0;
                let a2686 = 44u64;
                if nat_lt(a2685, a2686) {
                    let a2687 = v0;
                    let a2688 = 42u64;
                    if nat_lt(a2687, a2688) {
                        let a2689 = v0;
                        let a2690 = 41u64;
                        if nat_lt(a2689, a2690) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2691 = v0;
                        let a2692 = 43u64;
                        if nat_lt(a2691, a2692) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2693 = v0;
                    let a2694 = 46u64;
                    if nat_lt(a2693, a2694) {
                        let a2695 = v0;
                        let a2696 = 45u64;
                        if nat_lt(a2695, a2696) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2697 = v0;
                        let a2698 = 47u64;
                        if nat_lt(a2697, a2698) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2699 = v0;
            let a2700 = 56u64;
            if nat_lt(a2699, a2700) {
                let a2701 = v0;
                let a2702 = 52u64;
                if nat_lt(a2701, a2702) {
                    let a2703 = v0;
                    let a2704 = 50u64;
                    if nat_lt(a2703, a2704) {
                        let a2705 = v0;
                        let a2706 = 49u64;
                        if nat_lt(a2705, a2706) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2707 = v0;
                        let a2708 = 51u64;
                        if nat_lt(a2707, a2708) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2709 = v0;
                    let a2710 = 54u64;
                    if nat_lt(a2709, a2710) {
                        let a2711 = v0;
                        let a2712 = 53u64;
                        if nat_lt(a2711, a2712) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2713 = v0;
                        let a2714 = 55u64;
                        if nat_lt(a2713, a2714) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2715 = v0;
                let a2716 = 60u64;
                if nat_lt(a2715, a2716) {
                    let a2717 = v0;
                    let a2718 = 58u64;
                    if nat_lt(a2717, a2718) {
                        let a2719 = v0;
                        let a2720 = 57u64;
                        if nat_lt(a2719, a2720) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2721 = v0;
                        let a2722 = 59u64;
                        if nat_lt(a2721, a2722) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2723 = v0;
                    let a2724 = 62u64;
                    if nat_lt(a2723, a2724) {
                        let a2725 = v0;
                        let a2726 = 61u64;
                        if nat_lt(a2725, a2726) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2727 = v0;
                        let a2728 = 63u64;
                        if nat_lt(a2727, a2728) {
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

pub fn f550(v0: u32) -> u32 {
    let a2731 = {
        let a2729 = f584(v0, 6u32, 26u32);
        let a2730 = f584(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2729, a2730)
    };
    let a2732 = f584(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2731, a2732)
}

pub fn f551(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2738 = {
        let a2733 = v0;
        let a2734 = v1;
        fixed_u32::bit_and(a2733, a2734)
    };
    let a2739 = {
        let a2736 = {
            let a2735 = v0;
            fixed_u32::bit_not(a2735)
        };
        let a2737 = v2;
        fixed_u32::bit_and(a2736, a2737)
    };
    fixed_u32::bit_xor(a2738, a2739)
}

pub fn f552(v0: u32) -> u32 {
    let a2742 = {
        let a2740 = f584(v0, 2u32, 30u32);
        let a2741 = f584(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2740, a2741)
    };
    let a2743 = f584(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2742, a2743)
}

pub fn f553(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2752 = {
        let a2748 = {
            let a2744 = v0;
            let a2745 = v1;
            fixed_u32::bit_and(a2744, a2745)
        };
        let a2749 = {
            let a2746 = v0;
            let a2747 = v2;
            fixed_u32::bit_and(a2746, a2747)
        };
        fixed_u32::bit_xor(a2748, a2749)
    };
    let a2753 = {
        let a2750 = v1;
        let a2751 = v2;
        fixed_u32::bit_and(a2750, a2751)
    };
    fixed_u32::bit_xor(a2752, a2753)
}

pub fn f554(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h2754, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2754
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2755, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2755
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h2756, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2756
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h2757, _, _, _, _, _, _, _, _, _, _, _) => {
            h2757
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2758, _, _, _, _, _, _, _, _, _, _) => {
            h2758
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2759, _, _, _, _, _, _, _, _, _) => {
            h2759
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h2760, _, _, _, _, _, _, _, _) => {
            h2760
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h2761, _, _, _, _, _, _, _) => {
            h2761
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2762, _, _, _, _, _, _) => {
            h2762
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h2763, _, _, _, _, _) => {
            h2763
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h2764, _, _, _, _) => {
            h2764
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2765, _, _, _) => {
            h2765
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2766, _, _) => {
            h2766
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2767, _) => {
            h2767
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2768) => {
            h2768
        }
    }, f495(f585(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2769, _) => {
            h2769
        }
    }), f495(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2770, _, _, _, _, _, _) => {
            h2770
        }
    }, f495(f586(match v0.clone() {
        Adt26::C0(_, h2771, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2771
        }
    }), match v0 {
        Adt26::C0(h2772, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2772
        }
    }))))
}

pub fn f555(v0: Fn21, p2776: u64, p2777: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2775 = (p2776, p2777);
    {
        let mut m2775 = m2775;
        loop {
            let step = {
                let (v1, v2) = m2775;
                let m2773 = v1;
                if m2773 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2773 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2774: Fn21 = v0.clone();
                        c2774.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2778, _) = v0;
        h2778
    };
    let v2: u32 = {
        let (h2779, _) = {
            let (_, h2780) = v0;
            h2780
        };
        h2779
    };
    let v3: u32 = {
        let (_, h2781) = {
            let (_, h2782) = v0;
            h2782
        };
        h2781
    };
    ({
        let a2783 = v1;
        let a2784 = 2u64;
        let a2785 = 0u64;
        nat_quot(a2783, a2784, a2785)
    }, (f557(v2, 1u32), {
        let a2789 = {
            let a2786 = v1;
            let a2787 = 2u64;
            let a2788 = 0u64;
            nat_rem(a2786, a2787, a2788)
        };
        let a2790 = 1u64;
        if nat_eq(a2789, a2790) {
            let a2791 = v3;
            let a2792 = v2;
            fixed_u32::bit_or(a2791, a2792)
        } else {
            v3
        }
    }))
}

pub fn f557(v0: u32, v1: u32) -> u32 {
    let m2795 = {
        let a2793 = v0;
        let a2794 = v1;
        fixed_u32::shift_left(a2793, a2794)
    };
    match m2795 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f558(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2798 = f35(v0.clone(), {
        let a2796 = v1;
        let a2797 = 2u64;
        nat_add(a2796, a2797)?
    }, 2u64);
    let a2799 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2798, a2799) {
        let m2802 = f587(f35(v0, {
            let a2800 = v1;
            let a2801 = 4u64;
            nat_add(a2800, a2801)?
        }, 2u64))?;
        match m2802 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f347(v2.clone(), 0u64)?;
                let a2803 = v3;
                let a2804 = 32u64;
                if nat_lt(a2803, a2804) {
                    if f588(v3) {
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

pub fn f559(v0: u64) -> Bytes {
    let a2805 = v0;
    let a2806 = 34u64;
    if nat_eq(a2805, a2806) {
        Bytes::lit(&[0x22])
    } else {
        let a2807 = v0;
        let a2808 = 92u64;
        if nat_eq(a2807, a2808) {
            Bytes::lit(&[0x5c])
        } else {
            let a2809 = v0;
            let a2810 = 98u64;
            if nat_eq(a2809, a2810) {
                Bytes::lit(&[0x08])
            } else {
                let a2811 = v0;
                let a2812 = 102u64;
                if nat_eq(a2811, a2812) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2813 = v0;
                    let a2814 = 110u64;
                    if nat_eq(a2813, a2814) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2815 = v0;
                        let a2816 = 114u64;
                        if nat_eq(a2815, a2816) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2817 = v0;
                            let a2818 = 116u64;
                            if nat_eq(a2817, a2818) {
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

pub fn f560(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2863 = {
            let a2839 = {
                let a2827 = {
                    let a2821 = {
                        let a2819 = v1;
                        let a2820 = 128u8;
                        fixed_u8::bit_and(a2819, a2820)
                    };
                    let a2822 = 128u8;
                    if equal(a2821, a2822) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2828 = {
                    let a2825 = {
                        let a2823 = v1;
                        let a2824 = 64u8;
                        fixed_u8::bit_and(a2823, a2824)
                    };
                    let a2826 = 64u8;
                    if equal(a2825, a2826) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2827, a2828)?
            };
            let a2840 = {
                let a2837 = {
                    let a2831 = {
                        let a2829 = v1;
                        let a2830 = 32u8;
                        fixed_u8::bit_and(a2829, a2830)
                    };
                    let a2832 = 32u8;
                    if equal(a2831, a2832) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2838 = {
                    let a2835 = {
                        let a2833 = v1;
                        let a2834 = 16u8;
                        fixed_u8::bit_and(a2833, a2834)
                    };
                    let a2836 = 16u8;
                    if equal(a2835, a2836) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2837, a2838)?
            };
            nat_add(a2839, a2840)?
        };
        let a2864 = {
            let a2861 = {
                let a2849 = {
                    let a2843 = {
                        let a2841 = v1;
                        let a2842 = 8u8;
                        fixed_u8::bit_and(a2841, a2842)
                    };
                    let a2844 = 8u8;
                    if equal(a2843, a2844) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2850 = {
                    let a2847 = {
                        let a2845 = v1;
                        let a2846 = 4u8;
                        fixed_u8::bit_and(a2845, a2846)
                    };
                    let a2848 = 4u8;
                    if equal(a2847, a2848) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2849, a2850)?
            };
            let a2862 = {
                let a2859 = {
                    let a2853 = {
                        let a2851 = v1;
                        let a2852 = 2u8;
                        fixed_u8::bit_and(a2851, a2852)
                    };
                    let a2854 = 2u8;
                    if equal(a2853, a2854) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2860 = {
                    let a2857 = {
                        let a2855 = v1;
                        let a2856 = 1u8;
                        fixed_u8::bit_and(a2855, a2856)
                    };
                    let a2858 = 1u8;
                    if equal(a2857, a2858) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2859, a2860)?
            };
            nat_add(a2861, a2862)?
        };
        nat_add(a2863, a2864)?
    };
    let a2865 = 48u64;
    let a2866 = v2;
    if nat_le(a2865, a2866) {
        let a2867 = v2;
        let a2868 = 57u64;
        Ok(nat_le(a2867, a2868))
    } else {
        Ok(false)
    }
}

pub fn f561(v0: Fn22, p2873: u64, p2874: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2872 = (p2873, p2874);
    {
        let mut m2872 = m2872;
        loop {
            let step = {
                let (v1, v2) = m2872;
                let m2869 = v1;
                if m2869 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2869 - 1;
                    let m2871 = {
                        let c2870: Fn22 = v0.clone();
                        c2870.apply(v2.clone())?
                    };
                    match m2871 {
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
                Ok(next) => { m2872 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f562(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f589(v0)
}

pub fn f563(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2879 = {
            let a2877 = {
                let a2875 = v0.clone();
                length_list(a2875)
            };
            let a2878 = {
                let a2876 = v1.clone();
                length_list(a2876)
            };
            nat_add(a2877, a2878)?
        };
        let a2880 = 1u64;
        nat_add(a2879, a2880)?
    };
    Ok(f387({
        let (_, h2881) = {
            let (_, h2882) = {
                let (h2883, _) = f590(Fn23::F591, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2883
            };
            h2882
        };
        h2881
    }))
}

pub fn f564(v0: u64) -> Bytes {
    f35({
        let a2884 = Str::lit("0123456789ABCDEF");
        utf8_encode(a2884)
    }, v0, 1u64)
}

pub fn f565(v0: bool, v1: List<Adt18>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2885 = v1;
    match m2885.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2886 = f592(v0, v3.clone())?;
            match m2886 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f593(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f41(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2887 = f565(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2887 {
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

pub fn f566(p2891: List<Bytes>) -> R<bool> {
    let m2890 = p2891;
    {
        let mut m2890 = m2890;
        loop {
            let step = {
                let v0 = m2890;
                let m2888 = v0;
                match m2888.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2889 = f594(v1)?;
                        match m2889 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f41(v3, f595(false, v2.clone())?) {
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
                Ok(next) => { m2890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: List<(Bytes, Adt18)>, v1: List<Adt18>) -> List<Bytes> {
    let m2892 = v0;
    match m2892.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f596(v1.clone(), {
                let (h2893, _) = v2.clone();
                h2893
            }) {
                List::cons({
                    let (h2894, _) = v2;
                    h2894
                }, f567(v3, v1))
            } else {
                f567(v3, v1)
            }
        }
    }
}

pub fn f568(v0: bool, v1: Bytes, v2: List<Adt18>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m2895 = v2;
    match m2895.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m2896 = f597(v4.clone())?;
            match m2896 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m2897 = f598(v0, v6.clone())?;
                    match m2897 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f41(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2900 = f599(v4, {
                                    let a2898 = v1.clone();
                                    let a2899 = f328(v6)?;
                                    append_bytes(a2898, a2899)
                                })?;
                                match m2900 {
                                    Ok(v9) => {
                                        let m2901 = f568(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m2901 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f491(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f538(v11.clone())), f450(f490(v9), f539(v11)))))
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

pub fn f569(v0: List<Adt18>) -> R<List<Bytes>> {
    let m2902 = v0;
    match m2902.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m2903 = f597(v1)?;
            match m2903 {
                None => {
                    f569(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f569(v2)?))
                }
            }
        }
    }
}

pub fn f570(v0: Bytes, v1: List<Adt18>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f600(f566(f601(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f571(v0: Adt18, v1: Bytes) -> R<bool> {
    let m2904 = f334(Some(v0));
    match m2904 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2905 = f529(v2)?;
            match m2905 {
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

pub fn f572(v0: bool, v1: Bytes, v2: List<(Bytes, Adt18)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m2906 = v2;
    match m2906.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m2908 = f602(v0, {
                let (h2907, _) = v5.clone();
                h2907
            })?;
            match m2908 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h2909, _) = v5;
                        h2909
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f41(v7.clone(), f603(v0, f573(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2914 = f542({
                            let (_, h2910) = v5.clone();
                            h2910
                        }, {
                            let a2912 = v1.clone();
                            let a2913 = f328({
                                let (h2911, _) = v5.clone();
                                h2911
                            })?;
                            append_bytes(a2912, a2913)
                        })?;
                        match m2914 {
                            Ok(v9) => {
                                let v10: Bytes = if f41({
                                    let (h2915, _) = v5;
                                    h2915
                                }, v3.clone()) {
                                    f491(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f491(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m2916 = f572(v0, v1, v6, v3, v4)?;
                                match m2916 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f538(v12.clone())), f450(f490(v9), f539(v12)))))
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

pub fn f573(v0: List<(Bytes, Adt18)>) -> List<Bytes> {
    let m2917 = v0;
    match m2917.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2918, _) = v1;
                h2918
            }, f573(v2))
        }
    }
}

pub fn f574(v0: Adt18) -> Option<Adt18> {
    let m2919 = f242(v0.clone(), f8(Str::lit("additionalProperties")));
    match m2919 {
        None => {
            f604(v0)
        }
        Some(v1) => {
            if f575(v1.clone()) {
                let m2920 = f604(v0);
                match m2920 {
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

pub fn f575(v0: Adt18) -> bool {
    let m2921 = v0;
    match m2921 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_, _) => {
            false
        }
        Adt18::C2(v3) => {
            !v3
        }
        Adt18::C3(_, _) => {
            false
        }
        Adt18::C4(_, _, _) => {
            false
        }
        Adt18::C5(_) => {
            false
        }
        Adt18::C6(_) => {
            false
        }
        Adt18::C7(_) => {
            false
        }
        Adt18::C8 => {
            false
        }
        Adt18::C9(_, _) => {
            false
        }
        Adt18::C10(_, _, _) => {
            false
        }
    }
}

pub fn f576(v0: Adt18) -> Option<Adt18> {
    let m2922 = f247(f242(v0, f8(Str::lit("oneOf"))));
    match m2922.uncons() {
        None => {
            None::<Adt18>
        }
        Some((v1, v2)) => {
            let m2923 = v2;
            match m2923.uncons() {
                None => {
                    None::<Adt18>
                }
                Some((v3, v4)) => {
                    let m2924 = v4;
                    match m2924.uncons() {
                        None => {
                            if f605(v3.clone()) {
                                Some(v1)
                            } else {
                                if f605(v1) {
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

pub fn f577(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m2925 = v1;
    match m2925 {
        Ok(v3) => {
            f533(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f491(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f490(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f578(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2926 = f576(v0.clone());
    match m2926 {
        None => {
            if f28(f245(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2927 = f242(v0, f8(Str::lit("items")));
                match m2927 {
                    None => {
                        Ok(f577(f8(Str::lit("List")), f579()))
                    }
                    Some(v3) => {
                        Ok(f577(f8(Str::lit("List")), f599(v3, {
                            let a2928 = v1;
                            let a2929 = f8(Str::lit("Item"));
                            append_bytes(a2928, a2929)
                        })?))
                    }
                }
            } else {
                f599(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f577(f8(Str::lit("Option")), f599(v2, v1)?))
        }
    }
}

pub fn f579() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f487(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f580(p2933: List<(Bytes, Adt18)>, v1: List<Bytes>) -> bool {
    let m2932 = p2933;
    {
        let mut m2932 = m2932;
        loop {
            let step = {
                let v0 = m2932;
                let m2930 = v0;
                match m2930.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f41({
                            let (h2931, _) = v2;
                            h2931
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt18)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt18)>, bool>(false)
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

pub fn f581(p2937: List<Adt18>, v1: Bytes, p2938: List<Bytes>) -> bool {
    let m2936 = (p2937, p2938);
    {
        let mut m2936 = m2936;
        loop {
            let step = {
                let (v0, v2) = m2936;
                let m2934 = v0;
                match m2934.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m2935 = f606(v3, v1.clone());
                        match m2935 {
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
                Ok(next) => { m2936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(p2941: List<Adt18>, p2942: List<Bytes>) -> bool {
    let m2940 = (p2941, p2942);
    {
        let mut m2940 = m2940;
        loop {
            let step = {
                let (v0, v1) = m2940;
                let m2939 = v0;
                match m2939.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f245(v2, f8(Str::lit("type")));
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
                Ok(next) => { m2940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: Adt18) -> bool {
    let v2: Adt18 = {
        let m2943 = f242(v0.clone(), f8(Str::lit("anyOf")));
        match m2943 {
            None => {
                Adt18::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f580(f483(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f543(f335(v2))
    } else {
        false
    }
}

pub fn f584(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2944 = f457(v0, v1);
    let a2945 = f557(v0, v2);
    fixed_u32::bit_or(a2944, a2945)
}

pub fn f585(v0: u32) -> u32 {
    let a2948 = {
        let a2946 = f584(v0, 17u32, 15u32);
        let a2947 = f584(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2946, a2947)
    };
    let a2949 = f457(v0, 10u32);
    fixed_u32::bit_xor(a2948, a2949)
}

pub fn f586(v0: u32) -> u32 {
    let a2952 = {
        let a2950 = f584(v0, 7u32, 25u32);
        let a2951 = f584(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2950, a2951)
    };
    let a2953 = f457(v0, 3u32);
    fixed_u32::bit_xor(a2952, a2953)
}

pub fn f587(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2954 = v0.clone();
        length_bytes(a2954)
    };
    let a2958 = {
        let a2955 = v1;
        let a2956 = 2u64;
        let a2957 = 0u64;
        nat_rem(a2955, a2956, a2957)
    };
    let a2959 = 0u64;
    if nat_eq(a2958, a2959) {
        let (_, h2960) = {
            let (h2961, _) = f607(Fn24::F608(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2961
        };
        Ok(h2960)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f588(v0: u64) -> bool {
    let a2962 = v0;
    let a2963 = 8u64;
    if nat_eq(a2962, a2963) {
        true
    } else {
        let a2964 = v0;
        let a2965 = 9u64;
        if nat_eq(a2964, a2965) {
            true
        } else {
            let a2966 = v0;
            let a2967 = 10u64;
            if nat_eq(a2966, a2967) {
                true
            } else {
                let a2968 = v0;
                let a2969 = 12u64;
                if nat_eq(a2968, a2969) {
                    true
                } else {
                    let a2970 = v0;
                    let a2971 = 13u64;
                    nat_eq(a2970, a2971)
                }
            }
        }
    }
}

pub fn f589(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2972, _) = v0;
        h2972
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2973) = v0;
        h2973
    };
    let m2974 = v1;
    match m2974.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2975 = v4;
            match m2975.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f609(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f590(v0: Fn23, p2980: u64, p2981: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2979 = (p2980, p2981);
    {
        let mut m2979 = m2979;
        loop {
            let step = {
                let (v1, v2) = m2979;
                let m2976 = v1;
                if m2976 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2976 - 1;
                    let m2978 = {
                        let c2977: Fn23 = v0.clone();
                        c2977.apply(v2.clone())
                    };
                    match m2978 {
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
                Ok(next) => { m2979 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f610(v0)
}

pub fn f592(v0: bool, v1: Adt18) -> R<Option<Bytes>> {
    let m2982 = v1;
    match m2982 {
        Adt18::C0 => {
            Ok(None::<Bytes>)
        }
        Adt18::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt18::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C3(v7, v8) => {
            Ok(Some({
                let a2993 = if f345(v7, {
                    let a2991 = 0u64;
                    let a2992 = v8;
                    nat_lt(a2991, a2992)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2994 = f266(false, v8)?;
                append_bytes(a2993, a2994)
            }))
        }
        Adt18::C4(v3, v4, v5) => {
            let m2985 = f611({
                let a2983 = f8(Str::lit("n"));
                let a2984 = f403(false, v4, v5)?;
                append_bytes(a2983, a2984)
            })?;
            match m2985 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2989 = f8(Str::lit("nMinus"));
                        let a2990 = f35(v6.clone(), 1u64, {
                            let a2987 = {
                                let a2986 = v6;
                                length_bytes(a2986)
                            };
                            let a2988 = 1u64;
                            nat_sub(a2987, a2988)
                        });
                        append_bytes(a2989, a2990)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt18::C5(v2) => {
            f598(v0, v2)
        }
        Adt18::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt18::C8 => {
            Ok(None::<Bytes>)
        }
        Adt18::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt18::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f593(v0: Adt18) -> R<Bytes> {
    f333(v0)
}

pub fn f594(v0: Bytes) -> R<Option<Bytes>> {
    let m2995 = f612(v0)?;
    match m2995 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f613(v1)?))
        }
    }
}

pub fn f595(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2996 = v1;
    match m2996.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2997 = f598(v0, v2)?;
            match m2997 {
                None => {
                    f595(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f595(v0, v3)?))
                }
            }
        }
    }
}

pub fn f596(p3001: List<Adt18>, v1: Bytes) -> bool {
    let m3000 = p3001;
    {
        let mut m3000 = m3000;
        loop {
            let step = {
                let v0 = m3000;
                let m2998 = v0;
                match m2998.uncons() {
                    None => {
                        Err::<List<Adt18>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m2999 = f606(v2, v1.clone());
                        if match m2999 {
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
                Ok(next) => { m3000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f597(v0: Adt18) -> R<Option<Bytes>> {
    let m3002 = f334(f242(v0.clone(), f8(Str::lit("$ref"))));
    match m3002 {
        None => {
            Ok(f334(f242(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f529(v1)
        }
    }
}

pub fn f598(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f611(v1)
    } else {
        f594(v1)
    }
}

pub fn f599(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f614(f515(v0), v1)
}

pub fn f600(v0: bool, v1: Bytes, v2: List<Adt18>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3003 = v2;
    match m3003.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3004 = f606(v5.clone(), v3.clone());
            match m3004 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3005 = f598(v0, v7.clone())?;
                    match m3005 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f41(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3008 = f536({
                                    let a3006 = v1.clone();
                                    let a3007 = f328(v7)?;
                                    append_bytes(a3006, a3007)
                                }, f485(f446(f242(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f246(f247(f242(v5, f8(Str::lit("required"))))), true)?;
                                match m3008 {
                                    Ok(v10) => {
                                        let m3009 = f600(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3009 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f342(f538(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f538(v12.clone())), f450(f539(v10), f539(v12)))))
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

pub fn f601(v0: List<Adt18>, v1: Bytes) -> List<Bytes> {
    let m3010 = v0;
    match m3010.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3011 = f606(v2, v1.clone());
            match m3011 {
                None => {
                    f601(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f601(v3, v1))
                }
            }
        }
    }
}

pub fn f602(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3012 = f598(v0, v1)?;
    match m3012 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f28(v2.clone(), f8(Str::lit("mk"))) {
                f513(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f603(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3013 = v1;
    match m3013.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3014 = f602(v0, v2)?;
            match m3014 {
                None => {
                    f603(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f603(v0, v3)?))
                }
            }
        }
    }
}

pub fn f604(v0: Adt18) -> Option<Adt18> {
    let m3015 = f446(f242(v0, f8(Str::lit("patternProperties"))));
    match m3015.uncons() {
        None => {
            None::<Adt18>
        }
        Some((v1, v2)) => {
            let m3016 = v2;
            match m3016.uncons() {
                None => {
                    Some({
                        let (_, h3017) = v1;
                        h3017
                    })
                }
                Some((_, _)) => {
                    None::<Adt18>
                }
            }
        }
    }
}

pub fn f605(v0: Adt18) -> bool {
    f28(f245(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f606(v0: Adt18, v1: Bytes) -> Option<Bytes> {
    if f41(v1.clone(), f246(f247(f242(v0.clone(), f8(Str::lit("required")))))) {
        let m3018 = f242(v0, f8(Str::lit("properties")));
        match m3018 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3019 = f242(v2, v1);
                match m3019 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f334(f242(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f607(v0: Fn24, p3024: u64, p3025: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3023 = (p3024, p3025);
    {
        let mut m3023 = m3023;
        loop {
            let step = {
                let (v1, v2) = m3023;
                let m3020 = v1;
                if m3020 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3020 - 1;
                    let m3022 = {
                        let c3021: Fn24 = v0.clone();
                        c3021.apply(v2.clone())?
                    };
                    match m3022 {
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
                Ok(next) => { m3023 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f608(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f615(v0, v1)
}

pub fn f609(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3030 = {
            let a3028 = {
                let a3026 = v0.clone();
                length_list(a3026)
            };
            let a3029 = {
                let a3027 = v1.clone();
                length_list(a3027)
            };
            nat_add(a3028, a3029)?
        };
        let a3031 = 1u64;
        nat_add(a3030, a3031)?
    };
    Ok(f616({
        let (_, h3032) = {
            let (_, h3033) = {
                let (h3034, _) = f617(Fn25::F618, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3034
            };
            h3033
        };
        h3032
    }))
}

pub fn f610(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3035, _) = v0.clone();
        h3035
    };
    let v2: List<(Bytes, ())> = {
        let (h3036, _) = {
            let (_, h3037) = v0.clone();
            h3037
        };
        h3036
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3038) = {
            let (_, h3039) = v0;
            h3039
        };
        h3038
    };
    let m3040 = v1.clone();
    match m3040.uncons() {
        None => {
            let m3041 = v2;
            match m3041.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3042 = v2.clone();
            match m3042.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f619({
                        let (h3043, _) = v8.clone();
                        h3043
                    }, {
                        let (h3044, _) = v6.clone();
                        h3044
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

pub fn f611(v0: Bytes) -> R<Option<Bytes>> {
    let m3046 = f620({
        let a3045 = v0.clone();
        length_bytes(a3045)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3046 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3048 = {
                let a3047 = v1.clone();
                length_bytes(a3047)
            };
            let a3049 = 0u64;
            if nat_eq(a3048, a3049) {
                let a3051 = {
                    let a3050 = v0;
                    length_bytes(a3050)
                };
                let a3052 = 0u64;
                if nat_eq(a3051, a3052) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f613(v1)?))
            }
        }
    }
}

pub fn f612(v0: Bytes) -> R<Option<Bytes>> {
    if f621({
        let a3053 = v0.clone();
        length_bytes(a3053)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f622(v0)?;
        if f623(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3056 = {
                let a3054 = v1.clone();
                let a3055 = 0u64;
                index_bytes(a3054, a3055)
            };
            match m3056 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f444(f62(v2)?) {
                        Ok(Some({
                            let a3057 = f8(Str::lit("n"));
                            let a3058 = v1;
                            append_bytes(a3057, a3058)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f624({
            let a3059 = v0.clone();
            length_bytes(a3059)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f613(v0: Bytes) -> R<Bytes> {
    if f41(v0.clone(), f625()) {
        f513(v0)
    } else {
        Ok(v0)
    }
}

pub fn f614(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3060 = v0.clone();
    match m3060 {
        Adt18::C0 => {
            Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C1(_, _) => {
            Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C2(v4) => {
            if v4 {
                Ok(f579())
            } else {
                Ok(f532(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt18::C3(_, _) => {
            Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C4(_, _, _) => {
            Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C5(_) => {
            Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C6(_) => {
            Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C7(_) => {
            let m3061 = f626(f483(v0.clone()));
            match m3061 {
                None => {
                    let m3062 = f334(f242(v0.clone(), f8(Str::lit("$ref"))));
                    match m3062 {
                        None => {
                            let m3064 = f242(v0.clone(), f8(Str::lit("allOf")));
                            match m3064 {
                                None => {
                                    let m3069 = f242(v0.clone(), f8(Str::lit("enum")));
                                    match m3069 {
                                        None => {
                                            let m3070 = f242(v0.clone(), f8(Str::lit("const")));
                                            match m3070 {
                                                None => {
                                                    let m3072 = f242(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3072 {
                                                        None => {
                                                            let v32: Bytes = f245(v0.clone(), f8(Str::lit("type")));
                                                            if if f28(v32.clone(), Bytes::lit(&[])) {
                                                                !f627(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f579())
                                                            } else {
                                                                if f28(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f533(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f28(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f533(f628(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f28(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f533(f629(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f28(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f533(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f28(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3073 = f446(f242(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3073.uncons() {
                                                                                        None => {
                                                                                            if f540(v0.clone()) {
                                                                                                Ok(f533(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3074 = f574(v0);
                                                                                                match m3074 {
                                                                                                    None => {
                                                                                                        Ok(f577(f8(Str::lit("List")), f630(f579())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f577(f8(Str::lit("List")), f630(f631(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f533(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f532(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f533(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3071 = f334(Some(v29));
                                                    match m3071 {
                                                        None => {
                                                            Ok(f532(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f533(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f533(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3065 = f335(v21);
                                    match m3065.uncons() {
                                        None => {
                                            Ok(f532(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3066 = v23;
                                            match m3066.uncons() {
                                                None => {
                                                    let m3067 = f334(f242(v22, f8(Str::lit("$ref"))));
                                                    match m3067 {
                                                        None => {
                                                            Ok(f532(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3068 = f529(v26)?;
                                                            match m3068 {
                                                                None => {
                                                                    Ok(f532(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f533(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f532(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3063 = f529(v19)?;
                            match m3063 {
                                None => {
                                    Ok(f532(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f533(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f532(if f28(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt18::C8 => {
            Ok(f579())
        }
        Adt18::C9(_, _) => {
            Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt18::C10(_, _, _) => {
            Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f615(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3075, _) = v1;
        h3075
    };
    let m3077 = {
        let (_, h3076) = v1;
        h3076
    };
    match m3077 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3080 = {
                let a3078 = v0.clone();
                let a3079 = v2;
                index_bytes(a3078, a3079)
            };
            match m3080 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3085 = {
                        let a3083 = v0;
                        let a3084 = {
                            let a3081 = v2;
                            let a3082 = 1u64;
                            nat_add(a3081, a3082)?
                        };
                        index_bytes(a3083, a3084)
                    };
                    match m3085 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f632(v4)? {
                                f632(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3086 = v2;
                                    let a3087 = 2u64;
                                    nat_add(a3086, a3087)?
                                }, Some({
                                    let a3092 = v3;
                                    let a3093 = f469(f633(), {
                                        let a3090 = {
                                            let a3088 = f634(v4)?;
                                            let a3089 = 16u64;
                                            nat_mul(a3088, a3089)?
                                        };
                                        let a3091 = f634(v5)?;
                                        nat_add(a3090, a3091)?
                                    });
                                    append_bytes(a3092, a3093)
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

pub fn f616(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f355(Fn5::F635, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f617(v0: Fn25, p3098: u64, p3099: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3097 = (p3098, p3099);
    {
        let mut m3097 = m3097;
        loop {
            let step = {
                let (v1, v2) = m3097;
                let m3094 = v1;
                if m3094 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3094 - 1;
                    let m3096 = {
                        let c3095: Fn25 = v0.clone();
                        c3095.apply(v2.clone())
                    };
                    match m3096 {
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
                Ok(next) => { m3097 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f636(v0)
}

pub fn f619(v0: Bytes, v1: Bytes) -> bool {
    let a3104 = {
        let a3100 = v0;
        let a3101 = v1;
        compare_bytes(a3100, a3101)
    };
    let a3105 = {
        let a3102 = Bytes::lit(&[]);
        let a3103 = Bytes::lit(&[0x01]);
        compare_bytes(a3102, a3103)
    };
    equal(a3104, a3105)
}

pub fn f620(p3118: u64, v1: Bytes, p3119: u64, p3120: Bytes) -> R<Option<Bytes>> {
    let m3117 = (p3118, (p3119, p3120));
    {
        let mut m3117 = m3117;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3117;
                let m3106 = v0;
                if m3106 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3106 - 1;
                    let a3108 = {
                        let a3107 = v1.clone();
                        length_bytes(a3107)
                    };
                    let a3109 = v2;
                    if nat_le(a3108, a3109) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3110 = f637(v1.clone(), v2)?;
                        match m3110 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3112 = v2;
                                    let a3113 = {
                                        let (_, h3111) = v5;
                                        h3111
                                    };
                                    nat_add(a3112, a3113)?
                                }, {
                                    let a3115 = v3;
                                    let a3116 = f638(v1.clone(), v2, {
                                        let (h3114, _) = v5;
                                        h3114
                                    });
                                    append_bytes(a3115, a3116)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(p3130: u64, v1: Bytes, p3131: u64) -> R<bool> {
    let m3129 = (p3130, p3131);
    {
        let mut m3129 = m3129;
        loop {
            let step = {
                let (v0, v2) = m3129;
                let m3121 = v0;
                if m3121 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3121 - 1;
                    let m3124 = {
                        let a3122 = v1.clone();
                        let a3123 = v2;
                        index_bytes(a3122, a3123)
                    };
                    match m3124 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3125 = f62(v4)?;
                            let a3126 = 128u64;
                            if nat_lt(a3125, a3126) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3127 = v2;
                                    let a3128 = 1u64;
                                    nat_add(a3127, a3128)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f622(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f399(v0.clone(), 0u64, {
        let a3132 = v0;
        length_bytes(a3132)
    }, false, Bytes::lit(&[]))?;
    let m3135 = {
        let a3133 = v1.clone();
        let a3134 = 0u64;
        index_bytes(a3133, a3134)
    };
    match m3135 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f443(v3) {
                let a3142 = f35({
                    let a3136 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3136)
                }, {
                    let a3137 = v3;
                    let a3138 = 65u64;
                    nat_sub(a3137, a3138)
                }, 1u64);
                let a3143 = f35(v1.clone(), 1u64, {
                    let a3140 = {
                        let a3139 = v1;
                        length_bytes(a3139)
                    };
                    let a3141 = 1u64;
                    nat_sub(a3140, a3141)
                });
                Ok(append_bytes(a3142, a3143))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f623(v0: Bytes) -> R<bool> {
    let m3146 = {
        let a3144 = v0;
        let a3145 = 0u64;
        index_bytes(a3144, a3145)
    };
    match m3146 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f418(f442(f62(v1)?), f443(f62(v1)?)))
        }
    }
}

pub fn f624(p3159: u64, v1: Bytes, p3160: u64, p3161: Bytes) -> R<Option<Bytes>> {
    let m3158 = (p3159, (p3160, p3161));
    {
        let mut m3158 = m3158;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3158;
                let m3147 = v0;
                if m3147 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3147 - 1;
                    let a3149 = {
                        let a3148 = v1.clone();
                        length_bytes(a3148)
                    };
                    let a3150 = v2;
                    if nat_le(a3149, a3150) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3151 = f637(v1.clone(), v2)?;
                        match m3151 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3153 = v2;
                                    let a3154 = {
                                        let (_, h3152) = v5;
                                        h3152
                                    };
                                    nat_add(a3153, a3154)?
                                }, {
                                    let a3156 = v3;
                                    let a3157 = f639({
                                        let (h3155, _) = v5;
                                        h3155
                                    });
                                    append_bytes(a3156, a3157)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f625() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f626(p3166: List<(Bytes, Adt18)>) -> Option<Bytes> {
    let m3165 = p3166;
    {
        let mut m3165 = m3165;
        loop {
            let step = {
                let v0 = m3165;
                let m3162 = v0;
                match m3162.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt18)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f41({
                            let (h3163, _) = v1.clone();
                            h3163
                        }, f640()) {
                            Err::<List<(Bytes, Adt18)>, Option<Bytes>>(Some({
                                let (h3164, _) = v1;
                                h3164
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt18)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: Adt18) -> bool {
    if f545(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f545(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f545(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f545(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f545(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f545(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f628(v0: Adt18) -> Bytes {
    let m3167 = f641(f242(v0, f8(Str::lit("minimum"))));
    match m3167 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f629() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f630(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3168 = v0;
    match m3168 {
        Ok(v2) => {
            f533(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f491(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f490(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f631(v0: Adt18, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f642(16u64, v0, v1)
}

pub fn f632(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3213 = {
            let a3189 = {
                let a3177 = {
                    let a3171 = {
                        let a3169 = v1;
                        let a3170 = 128u8;
                        fixed_u8::bit_and(a3169, a3170)
                    };
                    let a3172 = 128u8;
                    if equal(a3171, a3172) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3178 = {
                    let a3175 = {
                        let a3173 = v1;
                        let a3174 = 64u8;
                        fixed_u8::bit_and(a3173, a3174)
                    };
                    let a3176 = 64u8;
                    if equal(a3175, a3176) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3177, a3178)?
            };
            let a3190 = {
                let a3187 = {
                    let a3181 = {
                        let a3179 = v1;
                        let a3180 = 32u8;
                        fixed_u8::bit_and(a3179, a3180)
                    };
                    let a3182 = 32u8;
                    if equal(a3181, a3182) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3188 = {
                    let a3185 = {
                        let a3183 = v1;
                        let a3184 = 16u8;
                        fixed_u8::bit_and(a3183, a3184)
                    };
                    let a3186 = 16u8;
                    if equal(a3185, a3186) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3187, a3188)?
            };
            nat_add(a3189, a3190)?
        };
        let a3214 = {
            let a3211 = {
                let a3199 = {
                    let a3193 = {
                        let a3191 = v1;
                        let a3192 = 8u8;
                        fixed_u8::bit_and(a3191, a3192)
                    };
                    let a3194 = 8u8;
                    if equal(a3193, a3194) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3200 = {
                    let a3197 = {
                        let a3195 = v1;
                        let a3196 = 4u8;
                        fixed_u8::bit_and(a3195, a3196)
                    };
                    let a3198 = 4u8;
                    if equal(a3197, a3198) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3199, a3200)?
            };
            let a3212 = {
                let a3209 = {
                    let a3203 = {
                        let a3201 = v1;
                        let a3202 = 2u8;
                        fixed_u8::bit_and(a3201, a3202)
                    };
                    let a3204 = 2u8;
                    if equal(a3203, a3204) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3210 = {
                    let a3207 = {
                        let a3205 = v1;
                        let a3206 = 1u8;
                        fixed_u8::bit_and(a3205, a3206)
                    };
                    let a3208 = 1u8;
                    if equal(a3207, a3208) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3209, a3210)?
            };
            nat_add(a3211, a3212)?
        };
        nat_add(a3213, a3214)?
    };
    let a3215 = 48u64;
    let a3216 = v2;
    if if nat_le(a3215, a3216) {
        let a3217 = v2;
        let a3218 = 57u64;
        nat_le(a3217, a3218)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3219 = 97u64;
        let a3220 = v2;
        if nat_le(a3219, a3220) {
            let a3221 = v2;
            let a3222 = 102u64;
            Ok(nat_le(a3221, a3222))
        } else {
            Ok(false)
        }
    }
}

pub fn f633() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f634(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3267 = {
            let a3243 = {
                let a3231 = {
                    let a3225 = {
                        let a3223 = v1;
                        let a3224 = 128u8;
                        fixed_u8::bit_and(a3223, a3224)
                    };
                    let a3226 = 128u8;
                    if equal(a3225, a3226) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3232 = {
                    let a3229 = {
                        let a3227 = v1;
                        let a3228 = 64u8;
                        fixed_u8::bit_and(a3227, a3228)
                    };
                    let a3230 = 64u8;
                    if equal(a3229, a3230) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3231, a3232)?
            };
            let a3244 = {
                let a3241 = {
                    let a3235 = {
                        let a3233 = v1;
                        let a3234 = 32u8;
                        fixed_u8::bit_and(a3233, a3234)
                    };
                    let a3236 = 32u8;
                    if equal(a3235, a3236) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3242 = {
                    let a3239 = {
                        let a3237 = v1;
                        let a3238 = 16u8;
                        fixed_u8::bit_and(a3237, a3238)
                    };
                    let a3240 = 16u8;
                    if equal(a3239, a3240) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3241, a3242)?
            };
            nat_add(a3243, a3244)?
        };
        let a3268 = {
            let a3265 = {
                let a3253 = {
                    let a3247 = {
                        let a3245 = v1;
                        let a3246 = 8u8;
                        fixed_u8::bit_and(a3245, a3246)
                    };
                    let a3248 = 8u8;
                    if equal(a3247, a3248) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3254 = {
                    let a3251 = {
                        let a3249 = v1;
                        let a3250 = 4u8;
                        fixed_u8::bit_and(a3249, a3250)
                    };
                    let a3252 = 4u8;
                    if equal(a3251, a3252) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3253, a3254)?
            };
            let a3266 = {
                let a3263 = {
                    let a3257 = {
                        let a3255 = v1;
                        let a3256 = 2u8;
                        fixed_u8::bit_and(a3255, a3256)
                    };
                    let a3258 = 2u8;
                    if equal(a3257, a3258) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3264 = {
                    let a3261 = {
                        let a3259 = v1;
                        let a3260 = 1u8;
                        fixed_u8::bit_and(a3259, a3260)
                    };
                    let a3262 = 1u8;
                    if equal(a3261, a3262) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3263, a3264)?
            };
            nat_add(a3265, a3266)?
        };
        nat_add(a3267, a3268)?
    };
    let a3269 = v2;
    let a3270 = 57u64;
    if nat_le(a3269, a3270) {
        let a3271 = v2;
        let a3272 = 48u64;
        Ok(nat_sub(a3271, a3272))
    } else {
        let a3273 = v2;
        let a3274 = 87u64;
        Ok(nat_sub(a3273, a3274))
    }
}

pub fn f635(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f636(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
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
                    if f619({
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

pub fn f637(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3287 = {
        let a3285 = v0.clone();
        let a3286 = v1;
        index_bytes(a3285, a3286)
    };
    match m3287 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            let a3288 = v3;
            let a3289 = 128u64;
            if nat_lt(a3288, a3289) {
                Ok(Some((v3, 1u64)))
            } else {
                if f345({
                    let a3290 = 194u64;
                    let a3291 = v3;
                    nat_le(a3290, a3291)
                }, {
                    let a3292 = v3;
                    let a3293 = 223u64;
                    nat_le(a3292, a3293)
                }) {
                    let m3296 = f643(v0, {
                        let a3294 = v1;
                        let a3295 = 1u64;
                        nat_add(a3294, a3295)?
                    })?;
                    match m3296 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f644({
                                let a3301 = {
                                    let a3299 = {
                                        let a3297 = v3;
                                        let a3298 = 192u64;
                                        nat_sub(a3297, a3298)
                                    };
                                    let a3300 = 64u64;
                                    nat_mul(a3299, a3300)?
                                };
                                let a3302 = v4;
                                nat_add(a3301, a3302)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f345({
                        let a3303 = 224u64;
                        let a3304 = v3;
                        nat_le(a3303, a3304)
                    }, {
                        let a3305 = v3;
                        let a3306 = 239u64;
                        nat_le(a3305, a3306)
                    }) {
                        let m3309 = f643(v0.clone(), {
                            let a3307 = v1;
                            let a3308 = 1u64;
                            nat_add(a3307, a3308)?
                        })?;
                        match m3309 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3312 = f643(v0, {
                                    let a3310 = v1;
                                    let a3311 = 2u64;
                                    nat_add(a3310, a3311)?
                                })?;
                                match m3312 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f644({
                                            let a3321 = {
                                                let a3315 = {
                                                    let a3313 = v3;
                                                    let a3314 = 224u64;
                                                    nat_sub(a3313, a3314)
                                                };
                                                let a3316 = 4096u64;
                                                nat_mul(a3315, a3316)?
                                            };
                                            let a3322 = {
                                                let a3319 = {
                                                    let a3317 = v5;
                                                    let a3318 = 64u64;
                                                    nat_mul(a3317, a3318)?
                                                };
                                                let a3320 = v6;
                                                nat_add(a3319, a3320)?
                                            };
                                            nat_add(a3321, a3322)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f345({
                            let a3323 = 240u64;
                            let a3324 = v3;
                            nat_le(a3323, a3324)
                        }, {
                            let a3325 = v3;
                            let a3326 = 244u64;
                            nat_le(a3325, a3326)
                        }) {
                            let m3329 = f643(v0.clone(), {
                                let a3327 = v1;
                                let a3328 = 1u64;
                                nat_add(a3327, a3328)?
                            })?;
                            match m3329 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3332 = f643(v0.clone(), {
                                        let a3330 = v1;
                                        let a3331 = 2u64;
                                        nat_add(a3330, a3331)?
                                    })?;
                                    match m3332 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3335 = f643(v0, {
                                                let a3333 = v1;
                                                let a3334 = 3u64;
                                                nat_add(a3333, a3334)?
                                            })?;
                                            match m3335 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f644({
                                                        let a3348 = {
                                                            let a3338 = {
                                                                let a3336 = v3;
                                                                let a3337 = 240u64;
                                                                nat_sub(a3336, a3337)
                                                            };
                                                            let a3339 = 262144u64;
                                                            nat_mul(a3338, a3339)?
                                                        };
                                                        let a3349 = {
                                                            let a3346 = {
                                                                let a3340 = v7;
                                                                let a3341 = 4096u64;
                                                                nat_mul(a3340, a3341)?
                                                            };
                                                            let a3347 = {
                                                                let a3344 = {
                                                                    let a3342 = v8;
                                                                    let a3343 = 64u64;
                                                                    nat_mul(a3342, a3343)?
                                                                };
                                                                let a3345 = v9;
                                                                nat_add(a3344, a3345)?
                                                            };
                                                            nat_add(a3346, a3347)?
                                                        };
                                                        nat_add(a3348, a3349)?
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

pub fn f638(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f418(f345(f418(f442(v2), f443(v2)), f345({
        let a3350 = v2;
        let a3351 = 117u64;
        !nat_eq(a3350, a3351)
    }, {
        let a3352 = v2;
        let a3353 = 85u64;
        !nat_eq(a3352, a3353)
    })), f345(f444(v2), {
        let a3354 = 0u64;
        let a3355 = v1;
        nat_lt(a3354, a3355)
    })) {
        f35(v0, v1, 1u64)
    } else {
        let a3356 = v2;
        let a3357 = 65536u64;
        if nat_lt(a3356, a3357) {
            let a3358 = f8(Str::lit("u"));
            let a3359 = f530(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3358, a3359)
        } else {
            let a3360 = f8(Str::lit("U"));
            let a3361 = f530(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3360, a3361)
        }
    }
}

pub fn f639(v0: u64) -> Bytes {
    let a3362 = f8(Str::lit("u"));
    let a3363 = f530(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3362, a3363)
}

pub fn f640() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f641(v0: Option<Adt18>) -> Option<u64> {
    let m3364 = v0;
    match m3364 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m3365 = v1;
            match m3365 {
                Adt18::C0 => {
                    None::<u64>
                }
                Adt18::C1(_, _) => {
                    None::<u64>
                }
                Adt18::C2(_) => {
                    None::<u64>
                }
                Adt18::C3(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt18::C4(_, _, _) => {
                    None::<u64>
                }
                Adt18::C5(_) => {
                    None::<u64>
                }
                Adt18::C6(_) => {
                    None::<u64>
                }
                Adt18::C7(_) => {
                    None::<u64>
                }
                Adt18::C8 => {
                    None::<u64>
                }
                Adt18::C9(_, _) => {
                    None::<u64>
                }
                Adt18::C10(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f642(v0: u64, v1: Adt18, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3366 = v0;
    if m3366 == 0 {
        Ok(f532(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3366 - 1;
        let m3367 = v1.clone();
        match m3367 {
            Adt18::C0 => {
                Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C1(_, _) => {
                Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C2(v6) => {
                if v6 {
                    Ok(f579())
                } else {
                    Ok(f532(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt18::C3(_, _) => {
                Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C4(_, _, _) => {
                Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C5(_) => {
                Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C6(_) => {
                Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C7(_) => {
                let m3368 = f334(f242(v1.clone(), f8(Str::lit("$ref"))));
                match m3368 {
                    None => {
                        if if f545(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f545(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f545(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f533(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f245(v1.clone(), f8(Str::lit("type")));
                            if if f28(v22.clone(), Bytes::lit(&[])) {
                                !f627(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f579())
                            } else {
                                if f28(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f533(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f28(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f533(f628(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f28(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f533(f629(), List::<Adt25>::nil()))
                                        } else {
                                            if f28(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f533(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f28(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3370 = f242(v1, f8(Str::lit("items")));
                                                    match m3370 {
                                                        None => {
                                                            Ok(f577(f8(Str::lit("List")), f579()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f577(f8(Str::lit("List")), f642(v3, v23, {
                                                                let a3371 = v2;
                                                                let a3372 = f8(Str::lit("Item"));
                                                                append_bytes(a3371, a3372)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f28(v22.clone(), f8(Str::lit("object"))) {
                                                        if f540(v1.clone()) {
                                                            Ok(f533(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3373 = f574(v1);
                                                            match m3373 {
                                                                None => {
                                                                    Ok(f577(f8(Str::lit("List")), f630(f579())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f577(f8(Str::lit("List")), f630(f642(v3, v24, {
                                                                        let a3374 = v2;
                                                                        let a3375 = f8(Str::lit("Value"));
                                                                        append_bytes(a3374, a3375)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f532(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3369 = f529(v20)?;
                        match m3369 {
                            None => {
                                Ok(f532(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f533(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt18::C8 => {
                Ok(f579())
            }
            Adt18::C9(_, _) => {
                Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt18::C10(_, _, _) => {
                Ok(f532(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f643(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3378 = {
        let a3376 = v0;
        let a3377 = v1;
        index_bytes(a3376, a3377)
    };
    match m3378 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f62(v2)?;
            if f345({
                let a3379 = 128u64;
                let a3380 = v3;
                nat_le(a3379, a3380)
            }, {
                let a3381 = v3;
                let a3382 = 191u64;
                nat_le(a3381, a3382)
            }) {
                Ok(Some({
                    let a3383 = v3;
                    let a3384 = 128u64;
                    nat_sub(a3383, a3384)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f644(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f345({
        let a3385 = v1;
        let a3386 = v0;
        nat_le(a3385, a3386)
    }, f345({
        let a3387 = v0;
        let a3388 = v2;
        nat_le(a3387, a3388)
    }, f418({
        let a3389 = v0;
        let a3390 = 55296u64;
        nat_lt(a3389, a3390)
    }, {
        let a3391 = 57343u64;
        let a3392 = v0;
        nat_lt(a3391, a3392)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
