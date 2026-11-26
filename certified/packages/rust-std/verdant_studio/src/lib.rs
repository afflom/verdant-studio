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
    C0(List<(Bytes, Option<Bytes>)>, List<(Bytes, List<Bytes>)>, List<Bytes>, List<(Bytes, Option<Adt2>)>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt2>),
    C5(std::rc::Rc<Adt2>),
    C6,
    C7(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C8(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
    F72(u8, Bytes),
    F183(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F72(k0, k1) => f72(*k0, k1.clone(), p0),
            Fn0::F183(k0) => f183(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F116,
    F238,
    F334,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F116 => f116(p0, p1),
            Fn1::F238 => f238(p0, p1),
            Fn1::F334 => f334(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F206(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F206(k0) => f206(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F302(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn3::F302(k0) => f302(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F304(Bytes),
    F305(Bytes),
    F321(Bytes, u64, Bytes, u64),
    F394(Bytes),
    F395(u64, Bytes),
    F401(Bytes),
    F459(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F304(k0) => f304(k0.clone(), p0),
            Fn4::F305(k0) => f305(k0.clone(), p0),
            Fn4::F321(k0, k1, k2, k3) => f321(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn4::F394(k0) => Ok(f394(k0.clone(), p0)),
            Fn4::F395(k0, k1) => f395(*k0, k1.clone(), p0),
            Fn4::F401(k0) => f401(k0.clone(), p0),
            Fn4::F459(k0) => f459(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F308(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn5::F308(k0) => f308(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F384(Bytes),
    F491(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn6::F384(k0) => f384(k0.clone(), p0),
            Fn6::F491(k0, k1) => Ok(f491(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F386,
    F633,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn7::F386 => f386(p0, p1),
            Fn7::F633 => f633(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F389,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn8::F389 => f389(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F417,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn9::F417 => f417(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F420,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn10::F420 => f420(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F422,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn11::F422 => f422(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F425,
}

impl Fn12 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn12::F425 => f425(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F446(Bytes, u64, u64),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn13::F446(k0, k1, k2) => f446(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F464,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn14::F464 => f464(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F467,
}

impl Fn15 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn15::F467 => f467(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F495,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F495 => f495(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F497,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F497 => f497(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F516,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F516 => f516(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F522,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F522 => f522(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F527,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F527 => f527(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F541(Bytes),
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn21::F541(k0) => f541(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F567,
}

impl Fn22 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn22::F567 => f567(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F570,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn23::F570 => f570(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F598,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn24::F598 => f598(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F621,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F621 => f621(p0),
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
    Adt1::C0(List::<(Bytes, Option<Bytes>)>::nil(), List::<(Bytes, List<Bytes>)>::nil(), List::<Bytes>::nil(), List::<(Bytes, Option<Adt2>)>::nil())
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
                Ok(List::cons(f36(v4)?, f23(v0, {
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
    Ok(Adt1::C0(List::cons((v1.clone(), v2.clone()), match v0.clone() {
        Adt1::C0(h153, _, _, _) => {
            h153
        }
    }), match v0.clone() {
        Adt1::C0(_, h154, _, _) => {
            h154
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h155, _) => {
            h155
        }
    }, if f37(v1.clone(), f8(Str::lit(".json"))) {
        List::cons((v1, f38({
            let m156 = v2;
            match m156 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v3) => {
                    v3
                }
            }
        })?), match v0 {
            Adt1::C0(_, _, _, h157) => {
                h157
            }
        })
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
    let v1: List<Bytes> = f39(v0.clone(), 47u8)?;
    let v2: Bytes = f40(v1);
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
        Ok(f41(v0.clone(), 0u64, {
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
    f33(f42(v0, v1)?)
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
                f43(v1)?
            }
            Adt12::C1(v2) => {
                f43(v2)?
            }
            Adt12::C2(v3, v4, _) => {
                f43(List::cons(v3, v4))?
            }
            Adt12::C3(v6, _) => {
                f43(f44(v6))?
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
            Ok(Adt12::C4(f45(v9)))
        }
    }
}

pub fn f34(v0: Adt1) -> R<Adt12> {
    if f46(f47(f48(), match v0.clone() {
        Adt1::C0(_, _, h185, _) => {
            h185
        }
    })) {
        Ok(f49(f48(), f50(f8(Str::lit("releases/latest")))))
    } else {
        let m186 = f51(v0.clone(), List::cons(f48(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil()))));
        match m186.uncons() {
            None => {
                let v4: Bytes = {
                    let m187 = f52(v0.clone(), f48())?;
                    match m187 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v3) => {
                            f53(v3, f8(Str::lit("tag_name")))
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
                    let v5: List<Bytes> = f54(f55(), f56(v4.clone()));
                    let a192 = {
                        let a191 = f57(v5.clone(), f58(f59(v0.clone(), f8(Str::lit("model/authorities.toml"))))?);
                        length_list(a191)
                    };
                    let a193 = 0u64;
                    if nat_eq(a192, a193) {
                        f60(v0, v4)
                    } else {
                        let m194 = f51(v0.clone(), f61(v5));
                        match m194.uncons() {
                            None => {
                                if f62(v0.clone(), f63(f64(Adt13::C1, f56(v4.clone())))) {
                                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting ingest: the verdant-engine ")), List::cons(v4, List::cons(f8(Str::lit(" bindings are staged, and advance ingests them\u{a}")), List::<Bytes>::nil()))))))
                                } else {
                                    f65(v0, v4)
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
            Adt12::C3(List::cons((f8(Str::lit(".github/workflows/track.yml")), f66()), v1), v2)
        }
        Adt12::C4(v8) => {
            Adt12::C4(v8)
        }
    }
}

pub fn f36(v0: u8) -> R<Str> {
    let v1: u64 = f67(v0)?;
    let m204 = {
        let a202 = f68({
            let a196 = v1;
            let a197 = 16u64;
            let a198 = 0u64;
            nat_quot(a196, a197, a198)
        });
        let a203 = {
            let a199 = v1;
            let a200 = 16u64;
            let a201 = 0u64;
            nat_rem(a199, a200, a201)
        };
        index_list(a202, a203)
    };
    match m204 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f37(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a205 = v0.clone();
        length_bytes(a205)
    };
    let v3: u64 = {
        let a206 = v1.clone();
        length_bytes(a206)
    };
    let a207 = v3;
    let a208 = v2;
    if nat_le(a207, a208) {
        let m214 = {
            let a211 = v0;
            let a212 = {
                let a209 = v2;
                let a210 = v3;
                nat_sub(a209, a210)
            };
            let a213 = v3;
            slice_bytes(a211, a212, a213)
        };
        match m214 {
            None => {
                false
            }
            Some(v4) => {
                let a215 = v4;
                let a216 = v1;
                equal(a215, a216)
            }
        }
    } else {
        false
    }
}

pub fn f38(v0: Bytes) -> R<Option<Adt2>> {
    let v1: Bytes = f69(v0)?;
    let m220 = f70(v1.clone(), 0u64, {
        let a218 = {
            let a217 = v1.clone();
            length_bytes(a217)
        };
        let a219 = 2u64;
        nat_add(a218, a219)?
    }, 0u64)?;
    match m220 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v2) => {
            let a223 = {
                let (_, h221) = v2;
                h221
            };
            let a224 = {
                let a222 = v1;
                length_bytes(a222)
            };
            if nat_eq(a223, a224) {
                Ok(Some({
                    let (h225, _) = v2;
                    h225
                }))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f39(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a226 = v0.clone();
        length_bytes(a226)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h227, _) = f71(Fn0::F72(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h227
    };
    let v4: u64 = {
        let (h228, _) = {
            let (_, h229) = v3.clone();
            h229
        };
        h228
    };
    Ok(f73(List::cons(f41(v0, v4, {
        let a230 = v2;
        let a231 = v4;
        nat_sub(a230, a231)
    }), {
        let (_, h232) = {
            let (_, h233) = v3;
            h233
        };
        h232
    })))
}

pub fn f40(p237: List<Bytes>) -> Bytes {
    let m236 = p237;
    {
        let mut m236 = m236;
        loop {
            let step = {
                let v0 = m236;
                let m234 = v0;
                match m234.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m235 = v2.clone();
                        match m235.uncons() {
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
                Ok(next) => { m236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m241 = {
        let a238 = v0;
        let a239 = v1;
        let a240 = v2;
        slice_bytes(a238, a239, a240)
    };
    match m241 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f42(v0: List<Bytes>, v1: Adt1) -> R<Adt12> {
    let m242 = v0;
    match m242.uncons() {
        None => {
            Ok(Adt12::C4(f74()))
        }
        Some((v2, v3)) => {
            if f32(v2.clone(), f8(Str::lit("agenda"))) {
                f75(v1)
            } else {
                if f32(v2.clone(), f8(Str::lit("build"))) {
                    Ok(f76(v1))
                } else {
                    if f32(v2.clone(), f8(Str::lit("check"))) {
                        f77(v1)
                    } else {
                        if f32(v2.clone(), f8(Str::lit("next"))) {
                            f78(v1, v3)
                        } else {
                            if f32(v2.clone(), f8(Str::lit("publish"))) {
                                f79(v1)
                            } else {
                                let m243 = v3;
                                match m243.uncons() {
                                    None => {
                                        Ok(Adt12::C4(f74()))
                                    }
                                    Some((v4, v5)) => {
                                        if f80(v2.clone(), f8(Str::lit("read-"))) {
                                            Ok(Adt12::C4(f8(Str::lit("read-webidl, read-xml, read-xsd and read-yaml run in verdant_readers, the command of the readers project\u{a}"))))
                                        } else {
                                            if f80(v2.clone(), f8(Str::lit("stage-"))) {
                                                f81(v2, v4, v5, v1)
                                            } else {
                                                f82(v2, v4, v5, v1)
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

pub fn f43(p246: List<Bytes>) -> R<Option<Bytes>> {
    let m245 = p246;
    {
        let mut m245 = m245;
        loop {
            let step = {
                let v0 = m245;
                let m244 = v0;
                match m244.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f83(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m247 = v0;
    match m247.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h248, _) = v1;
                h248
            }, f44(v2))
        }
    }
}

pub fn f45(v0: Bytes) -> Bytes {
    let a253 = {
        let a249 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a249)
    };
    let a254 = {
        let a251 = v0;
        let a252 = {
            let a250 = Str::lit(" is not\u{a}");
            utf8_encode(a250)
        };
        append_bytes(a251, a252)
    };
    append_bytes(a253, a254)
}

pub fn f46(v0: bool) -> bool {
    !v0
}

pub fn f47(v0: Bytes, p257: List<Bytes>) -> bool {
    let m256 = p257;
    {
        let mut m256 = m256;
        loop {
            let step = {
                let v1 = m256;
                let m255 = v1;
                match m255.uncons() {
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
                Ok(next) => { m256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48() -> Bytes {
    f8(Str::lit("target/engine/release.json"))
}

pub fn f49(v0: Bytes, v1: Bytes) -> Adt12 {
    Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v0.clone(), List::cons(v1, List::<Bytes>::nil()))))), v0)
}

pub fn f50(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://api.github.com/repos/afflom/verdant-engine/")), List::cons(v0, List::<Bytes>::nil())))
}

pub fn f51(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m258 = v1;
    match m258.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m260 = f84(match v0.clone() {
                Adt1::C0(h259, _, _, _) => {
                    h259
                }
            }, v2.clone());
            match m260 {
                None => {
                    List::cons(v2, f51(v0, v3))
                }
                Some(_) => {
                    f51(v0, v3)
                }
            }
        }
    }
}

pub fn f52(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    let m261 = f85(v0, v1)?;
    match m261 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            Ok(f86(v2))
        }
    }
}

pub fn f53(v0: Adt14, v1: Bytes) -> Bytes {
    let m262 = f87(f88(v0, v1));
    match m262 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f54(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m263 = v0;
    match m263.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f64(v2, v1.clone()), f54(v3, v1))
        }
    }
}

pub fn f55() -> List<Adt13> {
    List::cons(Adt13::C0, List::cons(Adt13::C1, List::cons(Adt13::C2, List::cons(Adt13::C3, List::<Adt13>::nil()))))
}

pub fn f56(v0: Bytes) -> Bytes {
    if f80(v0.clone(), f8(Str::lit("v"))) {
        f41(v0.clone(), 1u64, {
            let a265 = {
                let a264 = v0;
                length_bytes(a264)
            };
            let a266 = 1u64;
            nat_sub(a265, a266)
        })
    } else {
        v0
    }
}

pub fn f57(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m267 = v0;
    match m267.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f47(v2.clone(), v1.clone()) {
                f57(v3, v1)
            } else {
                List::cons(v2, f57(v3, v1))
            }
        }
    }
}

pub fn f58(v0: Bytes) -> R<List<Bytes>> {
    f89(f90(v0)?, f8(Str::lit("id = \"")))
}

pub fn f59(v0: Adt1, v1: Bytes) -> Bytes {
    let m269 = f84(match v0 {
        Adt1::C0(h268, _, _, _) => {
            h268
        }
    }, v1);
    match m269 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m270 = v2;
            match m270 {
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

pub fn f60(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let v2: List<Adt14> = f91(v0, f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f92(v2.clone(), f56(v1.clone()));
    if f93(f94(v2), f94(v3.clone())) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" is pinned, and every admission of its registers cites it\u{a}")), List::<Bytes>::nil()))))))
    } else {
        Ok(Adt12::C3(List::cons((f8(Str::lit("model/admissions.json")), f95(Adt14::C3(std::rc::Rc::new(f96(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
    }
}

pub fn f61(v0: List<Bytes>) -> List<Bytes> {
    let m271 = v0;
    match m271.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f63(v1), f61(v2))
        }
    }
}

pub fn f62(v0: Adt1, v1: Bytes) -> bool {
    let m273 = f84(match v0 {
        Adt1::C0(h272, _, _, _) => {
            h272
        }
    }, v1);
    match m273 {
        None => {
            false
        }
        Some(v2) => {
            let m274 = v2;
            match m274 {
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

pub fn f63(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f64(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f97(v0), List::cons(v1, List::<Bytes>::nil())))
}

pub fn f65(v0: Adt1, v1: Bytes) -> R<Adt12> {
    if f46(f47(f98(), match v0.clone() {
        Adt1::C0(_, _, h275, _) => {
            h275
        }
    })) {
        Ok(f49(f98(), f50(f7(List::cons(f8(Str::lit("git/ref/tags/")), List::cons(v1, List::<Bytes>::nil()))))))
    } else {
        let m276 = f51(v0.clone(), List::cons(f98(), List::<Bytes>::nil()));
        match m276.uncons() {
            None => {
                let v6: Adt14 = {
                    let m278 = f88({
                        let m277 = f52(v0.clone(), f98())?;
                        match m277 {
                            None => {
                                Adt14::C5
                            }
                            Some(v4) => {
                                v4
                            }
                        }
                    }, f8(Str::lit("object")));
                    match m278 {
                        None => {
                            Adt14::C5
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                };
                if f32(f53(v6.clone(), f8(Str::lit("type"))), f8(Str::lit("tag"))) {
                    if f46(f47(f99(), match v0.clone() {
                        Adt1::C0(_, _, h279, _) => {
                            h279
                        }
                    })) {
                        Ok(f49(f99(), f50(f7(List::cons(f8(Str::lit("git/tags/")), List::cons(f53(v6, f8(Str::lit("sha"))), List::<Bytes>::nil()))))))
                    } else {
                        let m280 = f51(v0.clone(), List::cons(f99(), List::<Bytes>::nil()));
                        match m280.uncons() {
                            None => {
                                f100(v0.clone(), v1, {
                                    let m282 = f88({
                                        let m281 = f52(v0, f99())?;
                                        match m281 {
                                            None => {
                                                Adt14::C5
                                            }
                                            Some(v9) => {
                                                v9
                                            }
                                        }
                                    }, f8(Str::lit("object")));
                                    match m282 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v10) => {
                                            f53(v10, f8(Str::lit("sha")))
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
                    f100(v0, v1, f53(v6, f8(Str::lit("sha"))))
                }
            }
            Some((v2, v3)) => {
                Ok(Adt12::C0(List::cons(v2, v3)))
            }
        }
    }
}

pub fn f66() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Engine.lex.tex. Do not edit.\u{a}# REQ-ENGINE-TRACKING: Verdant Studio tracks Verdant Engine releases. Every hour this runs\u{a}# operate stage-engine, which stages the latest release's four registers when they are new\u{a}# (advance then ingests them by sha256), and re-cites their admissions once they are ingested.\u{a}name: track\u{a}\u{a}on:\u{a}  schedule:\u{a}    - cron: '17 * * * *'\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}  actions: write\u{a}\u{a}jobs:\u{a}  track:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 5\u{a}    steps:\u{a}      - env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}        run: gh workflow run operate.yml --repo \"$GITHUB_REPOSITORY\" --ref main -f args=stage-engine\u{a}"))
}

pub fn f67(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a327 = {
        let a303 = {
            let a291 = {
                let a285 = {
                    let a283 = v1;
                    let a284 = 128u8;
                    fixed_u8::bit_and(a283, a284)
                };
                let a286 = 128u8;
                if equal(a285, a286) {
                    128u64
                } else {
                    0u64
                }
            };
            let a292 = {
                let a289 = {
                    let a287 = v1;
                    let a288 = 64u8;
                    fixed_u8::bit_and(a287, a288)
                };
                let a290 = 64u8;
                if equal(a289, a290) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a291, a292)?
        };
        let a304 = {
            let a301 = {
                let a295 = {
                    let a293 = v1;
                    let a294 = 32u8;
                    fixed_u8::bit_and(a293, a294)
                };
                let a296 = 32u8;
                if equal(a295, a296) {
                    32u64
                } else {
                    0u64
                }
            };
            let a302 = {
                let a299 = {
                    let a297 = v1;
                    let a298 = 16u8;
                    fixed_u8::bit_and(a297, a298)
                };
                let a300 = 16u8;
                if equal(a299, a300) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a301, a302)?
        };
        nat_add(a303, a304)?
    };
    let a328 = {
        let a325 = {
            let a313 = {
                let a307 = {
                    let a305 = v1;
                    let a306 = 8u8;
                    fixed_u8::bit_and(a305, a306)
                };
                let a308 = 8u8;
                if equal(a307, a308) {
                    8u64
                } else {
                    0u64
                }
            };
            let a314 = {
                let a311 = {
                    let a309 = v1;
                    let a310 = 4u8;
                    fixed_u8::bit_and(a309, a310)
                };
                let a312 = 4u8;
                if equal(a311, a312) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a313, a314)?
        };
        let a326 = {
            let a323 = {
                let a317 = {
                    let a315 = v1;
                    let a316 = 2u8;
                    fixed_u8::bit_and(a315, a316)
                };
                let a318 = 2u8;
                if equal(a317, a318) {
                    2u64
                } else {
                    0u64
                }
            };
            let a324 = {
                let a321 = {
                    let a319 = v1;
                    let a320 = 1u8;
                    fixed_u8::bit_and(a319, a320)
                };
                let a322 = 1u8;
                if equal(a321, a322) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a323, a324)?
        };
        nat_add(a325, a326)?
    };
    nat_add(a327, a328)
}

pub fn f68(v0: u64) -> List<Str> {
    let a329 = v0;
    let a330 = 0u64;
    if nat_eq(a329, a330) {
        f101()
    } else {
        let a331 = v0;
        let a332 = 1u64;
        if nat_eq(a331, a332) {
            f102()
        } else {
            let a333 = v0;
            let a334 = 2u64;
            if nat_eq(a333, a334) {
                f103()
            } else {
                let a335 = v0;
                let a336 = 3u64;
                if nat_eq(a335, a336) {
                    f104()
                } else {
                    let a337 = v0;
                    let a338 = 4u64;
                    if nat_eq(a337, a338) {
                        f105()
                    } else {
                        let a339 = v0;
                        let a340 = 5u64;
                        if nat_eq(a339, a340) {
                            f106()
                        } else {
                            let a341 = v0;
                            let a342 = 6u64;
                            if nat_eq(a341, a342) {
                                f107()
                            } else {
                                let a343 = v0;
                                let a344 = 7u64;
                                if nat_eq(a343, a344) {
                                    f108()
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

pub fn f69(v0: Bytes) -> R<Bytes> {
    f109(v0.clone(), 0u64, {
        let a345 = v0;
        length_bytes(a345)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f70(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m346 = v2;
    if m346 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m346 - 1;
        let v5: u64 = f110(v0.clone(), v1)?;
        let a347 = v3;
        let a348 = 1u64;
        if nat_eq(a347, a348) {
            let m349 = f70(v0.clone(), v1, v4, 0u64)?;
            match m349 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f110(v0.clone(), {
                        let (_, h350) = v6.clone();
                        h350
                    })?;
                    let a351 = v7;
                    let a352 = 44u64;
                    if nat_eq(a351, a352) {
                        let m356 = f70(v0, {
                            let a354 = {
                                let (_, h353) = v6;
                                h353
                            };
                            let a355 = 1u64;
                            nat_add(a354, a355)?
                        }, v4, 1u64)?;
                        match m356 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C7(std::rc::Rc::new({
                                    let (h357, _) = v6;
                                    h357
                                }), std::rc::Rc::new({
                                    let (h358, _) = v8;
                                    h358
                                })), {
                                    let (_, h359) = v8;
                                    h359
                                })))
                            }
                        }
                    } else {
                        let a360 = v7;
                        let a361 = 93u64;
                        if nat_eq(a360, a361) {
                            Ok(Some((Adt2::C7(std::rc::Rc::new({
                                let (h362, _) = v6;
                                h362
                            }), std::rc::Rc::new(Adt2::C6)), {
                                let a364 = {
                                    let (_, h363) = v6;
                                    h363
                                };
                                let a365 = 1u64;
                                nat_add(a364, a365)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a366 = v3;
            let a367 = 2u64;
            if nat_eq(a366, a367) {
                let m370 = f111(v0.clone(), {
                    let a368 = v1;
                    let a369 = 1u64;
                    nat_add(a368, a369)?
                })?;
                match m370 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f112({
                            let a371 = v5;
                            let a372 = 34u64;
                            nat_eq(a371, a372)
                        }, {
                            let a374 = f110(v0.clone(), {
                                let (h373, _) = v9.clone();
                                h373
                            })?;
                            let a375 = 58u64;
                            nat_eq(a374, a375)
                        }) {
                            let m379 = f70(v0.clone(), {
                                let a377 = {
                                    let (h376, _) = v9;
                                    h376
                                };
                                let a378 = 1u64;
                                nat_add(a377, a378)?
                            }, v4, 0u64)?;
                            match m379 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f110(v0.clone(), {
                                        let (_, h380) = v10.clone();
                                        h380
                                    })?;
                                    let a381 = v11;
                                    let a382 = 44u64;
                                    if nat_eq(a381, a382) {
                                        let m386 = f70(v0, {
                                            let a384 = {
                                                let (_, h383) = v10;
                                                h383
                                            };
                                            let a385 = 1u64;
                                            nat_add(a384, a385)?
                                        }, v4, 2u64)?;
                                        match m386 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C8({
                                                    let (_, h387) = v9;
                                                    h387
                                                }, std::rc::Rc::new({
                                                    let (h388, _) = v10;
                                                    h388
                                                }), std::rc::Rc::new({
                                                    let (h389, _) = v12;
                                                    h389
                                                })), {
                                                    let (_, h390) = v12;
                                                    h390
                                                })))
                                            }
                                        }
                                    } else {
                                        let a391 = v11;
                                        let a392 = 125u64;
                                        if nat_eq(a391, a392) {
                                            Ok(Some((Adt2::C8({
                                                let (_, h393) = v9;
                                                h393
                                            }, std::rc::Rc::new({
                                                let (h394, _) = v10;
                                                h394
                                            }), std::rc::Rc::new(Adt2::C6)), {
                                                let a396 = {
                                                    let (_, h395) = v10;
                                                    h395
                                                };
                                                let a397 = 1u64;
                                                nat_add(a396, a397)?
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
                let a398 = v5;
                let a399 = 34u64;
                if nat_eq(a398, a399) {
                    let m402 = f111(v0, {
                        let a400 = v1;
                        let a401 = 1u64;
                        nat_add(a400, a401)?
                    })?;
                    match m402 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C3({
                                let (_, h403) = v13;
                                h403
                            }), {
                                let (h404, _) = v13;
                                h404
                            })))
                        }
                    }
                } else {
                    if f32(f41(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt2::C0(true), {
                            let a405 = v1;
                            let a406 = 4u64;
                            nat_add(a405, a406)?
                        })))
                    } else {
                        if f32(f41(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt2::C0(false), {
                                let a407 = v1;
                                let a408 = 5u64;
                                nat_add(a407, a408)?
                            })))
                        } else {
                            let a409 = v5;
                            let a410 = 91u64;
                            if nat_eq(a409, a410) {
                                let a413 = f110(v0.clone(), {
                                    let a411 = v1;
                                    let a412 = 1u64;
                                    nat_add(a411, a412)?
                                })?;
                                let a414 = 93u64;
                                if nat_eq(a413, a414) {
                                    Ok(Some((Adt2::C4(std::rc::Rc::new(Adt2::C6)), {
                                        let a415 = v1;
                                        let a416 = 2u64;
                                        nat_add(a415, a416)?
                                    })))
                                } else {
                                    let m419 = f70(v0, {
                                        let a417 = v1;
                                        let a418 = 1u64;
                                        nat_add(a417, a418)?
                                    }, v4, 1u64)?;
                                    match m419 {
                                        None => {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt2::C4(std::rc::Rc::new({
                                                let (h420, _) = v14;
                                                h420
                                            })), {
                                                let (_, h421) = v14;
                                                h421
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a422 = v5;
                                let a423 = 123u64;
                                if nat_eq(a422, a423) {
                                    let a426 = f110(v0.clone(), {
                                        let a424 = v1;
                                        let a425 = 1u64;
                                        nat_add(a424, a425)?
                                    })?;
                                    let a427 = 125u64;
                                    if nat_eq(a426, a427) {
                                        Ok(Some((Adt2::C5(std::rc::Rc::new(Adt2::C6)), {
                                            let a428 = v1;
                                            let a429 = 2u64;
                                            nat_add(a428, a429)?
                                        })))
                                    } else {
                                        let m432 = f70(v0, {
                                            let a430 = v1;
                                            let a431 = 1u64;
                                            nat_add(a430, a431)?
                                        }, v4, 2u64)?;
                                        match m432 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt2::C5(std::rc::Rc::new({
                                                    let (h433, _) = v15;
                                                    h433
                                                })), {
                                                    let (_, h434) = v15;
                                                    h434
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m435 = f113(v0, v1)?;
                                    match m435 {
                                        None => {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h436) = v16;
                                                h436
                                            }, {
                                                let (h437, _) = v16;
                                                h437
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

pub fn f71(v0: Fn0, p442: u64, p443: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m441 = (p442, p443);
    {
        let mut m441 = m441;
        loop {
            let step = {
                let (v1, v2) = m441;
                let m438 = v1;
                if m438 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m438 - 1;
                    let m440 = {
                        let c439: Fn0 = v0.clone();
                        c439.apply(v2.clone())?
                    };
                    match m440 {
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
                Ok(next) => { m441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f114(v1, v0, v2)
}

pub fn f73(v0: List<Bytes>) -> List<Bytes> {
    f115(Fn1::F116, List::<Bytes>::nil(), v0)
}

pub fn f74() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f117(f118(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f75(v0: Adt1) -> R<Adt12> {
    let m444 = f119(v0.clone());
    match m444.uncons() {
        None => {
            let m445 = f120(f51(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f121(v0.clone())?);
            match m445.uncons() {
                None => {
                    let v5: Adt15 = f122(v0.clone())?;
                    let v6: List<Adt14> = f91(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(f120(f123(f124(match v5.clone() {
                        Adt15::C0(h446, _, _) => {
                            h446
                        }
                    }, f125(f91(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f126(v6.clone())), f127(f124(match v5.clone() {
                        Adt15::C0(h447, _, _) => {
                            h447
                        }
                    }, f125(f91(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f128(v5.clone(), v6, f129(v5, f130(v0.clone(), f131(v0)?)?))?)))))
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

pub fn f76(v0: Adt1) -> Adt12 {
    if f46(f47(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h448, _) => {
            h448
        }
    })) {
        Adt12::C2(f8(Str::lit("npx")), f132(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f46(f47(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h449, _) => {
                h449
            }
        })) {
            Adt12::C2(f8(Str::lit("npx")), f132(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f77(v0: Adt1) -> R<Adt12> {
    let m450 = f51(v0.clone(), f133());
    match m450.uncons() {
        None => {
            let m451 = f134(v0.clone(), f133())?;
            match m451.uncons() {
                None => {
                    let m454 = f135(match v0.clone() {
                        Adt1::C0(_, h452, _, _) => {
                            h452
                        }
                    }, f120(f136(), f137(match v0.clone() {
                        Adt1::C0(_, h453, _, _) => {
                            h453
                        }
                    }, false)));
                    match m454.uncons() {
                        None => {
                            let m456 = f51(v0.clone(), f137(match v0.clone() {
                                Adt1::C0(_, h455, _, _) => {
                                    h455
                                }
                            }, true));
                            match m456.uncons() {
                                None => {
                                    let m457 = f51(v0.clone(), f138());
                                    match m457.uncons() {
                                        None => {
                                            let m458 = f139(v0.clone())?;
                                            match m458 {
                                                None => {
                                                    let m460 = f140(v0.clone(), f137(match v0.clone() {
                                                        Adt1::C0(_, h459, _, _) => {
                                                            h459
                                                        }
                                                    }, true))?;
                                                    match m460 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f141(Adt17::C0(Adt16::C0(f58(f59(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(v0.clone())?, f143(f91(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f58(f59(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f143(f91(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f91(v0.clone(), f8(Str::lit("model/admissions.json")))?, f91(v0.clone(), f8(Str::lit("model/strata.json")))?, f144(f91(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f137(match v0 {
                                                                Adt1::C0(_, h461, _, _) => {
                                                                    h461
                                                                }
                                                            }, true)))?;
                                                            let (h462, _) = v13;
                                                            if h462 {
                                                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h463) = v13;
                                                                    h463
                                                                }))
                                                            } else {
                                                                Ok(Adt12::C4({
                                                                    let (_, h464) = v13;
                                                                    h464
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

pub fn f78(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f46(f145(match v0.clone() {
        Adt1::C0(_, h465, _, _) => {
            h465
        }
    }, f146())) {
        Ok(Adt12::C1(List::cons(f146(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f147(f148(f146(), f149(match v0.clone() {
            Adt1::C0(_, h466, _, _) => {
                h466
            }
        }, f146())))?;
        let m467 = f51(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m467.uncons() {
            None => {
                let m468 = f150(v0.clone(), f151(f152(f90(f59(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?, f153(v2, v1.clone()))?;
                match m468 {
                    None => {
                        f154(v0, v1)
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

pub fn f79(v0: Adt1) -> R<Adt12> {
    let m469 = f119(v0.clone());
    match m469.uncons() {
        None => {
            let m470 = f120(f51(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f155(), List::<Bytes>::nil())))), f121(v0.clone())?);
            match m470.uncons() {
                None => {
                    Ok(Adt12::C3(f156(f157(v0.clone())?, List::cons((f158(), f159()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f160()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f161()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f162()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f163()), List::cons((f8(Str::lit("docs/catalog.md")), f164(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f165(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f166(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f167()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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

pub fn f80(v0: Bytes, v1: Bytes) -> bool {
    let m475 = {
        let a472 = v0;
        let a473 = 0u64;
        let a474 = {
            let a471 = v1.clone();
            length_bytes(a471)
        };
        slice_bytes(a472, a473, a474)
    };
    match m475 {
        None => {
            false
        }
        Some(v2) => {
            let a476 = v2;
            let a477 = v1;
            equal(a476, a477)
        }
    }
}

pub fn f81(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f168(v3.clone(), v1.clone(), f169(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f168(v3.clone(), v1.clone(), f170(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f168(v3.clone(), v1.clone(), f169(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                    f168(v3.clone(), v1.clone(), f171(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                        f168(v3.clone(), v1.clone(), f172(v3, v1, false)?)
                    } else {
                        if f32(v0, f8(Str::lit("stage-matrix"))) {
                            f168(v3.clone(), v1.clone(), f172(v3, v1, true)?)
                        } else {
                            Ok(Adt12::C4(f74()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f82(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("admit"))) {
        f173(v3, v1)
    } else {
        if f32(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f174(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("ingest-authority"))) {
                f175(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                    f176(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                        f177(v3, v1)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("propose-encoding"))) {
                            let m478 = v2;
                            match m478.uncons() {
                                None => {
                                    Ok(Adt12::C4(f74()))
                                }
                                Some((v4, v5)) => {
                                    f178(v3, v1, v4, v5)
                                }
                            }
                        } else {
                            if f32(v0, f8(Str::lit("record-decision"))) {
                                f179(v3, v1)
                            } else {
                                Ok(Adt12::C4(f74()))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f83(v0: Bytes) -> R<bool> {
    f180(v0.clone(), 0u64, {
        let a479 = v0;
        length_bytes(a479)
    })
}

pub fn f84(p484: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m483 = p484;
    {
        let mut m483 = m483;
        loop {
            let step = {
                let v0 = m483;
                let m480 = v0;
                match m480.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h481, _) = v2;
                            h481
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h482) = v2;
                                h482
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m483 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m486 = f181(match v0.clone() {
        Adt1::C0(_, _, _, h485) => {
            h485
        }
    }, v1.clone());
    match m486 {
        None => {
            f38(f59(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f86(v0: Adt2) -> Option<Adt14> {
    let m487 = v0;
    match m487 {
        Adt2::C0(v1) => {
            Some(Adt14::C0(v1))
        }
        Adt2::C1(v2, v3) => {
            Some(Adt14::C1(v2, v3))
        }
        Adt2::C2(_, _, _) => {
            None::<Adt14>
        }
        Adt2::C3(v7) => {
            Some(Adt14::C2(v7))
        }
        Adt2::C4(r494) => {
            let v8 = (*r494).clone();
            let m488 = f86(v8);
            match m488 {
                None => {
                    None::<Adt14>
                }
                Some(v9) => {
                    Some(Adt14::C3(std::rc::Rc::new(v9)))
                }
            }
        }
        Adt2::C5(r495) => {
            let v10 = (*r495).clone();
            let m489 = f86(v10);
            match m489 {
                None => {
                    None::<Adt14>
                }
                Some(v11) => {
                    Some(Adt14::C4(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt2::C6 => {
            Some(Adt14::C5)
        }
        Adt2::C7(r496, r497) => {
            let v12 = (*r496).clone();
            let v13 = (*r497).clone();
            let m490 = f86(v12);
            match m490 {
                None => {
                    None::<Adt14>
                }
                Some(v14) => {
                    let m491 = f86(v13);
                    match m491 {
                        None => {
                            None::<Adt14>
                        }
                        Some(v15) => {
                            Some(Adt14::C6(std::rc::Rc::new(v14), std::rc::Rc::new(v15)))
                        }
                    }
                }
            }
        }
        Adt2::C8(v16, r498, r499) => {
            let v17 = (*r498).clone();
            let v18 = (*r499).clone();
            let m492 = f86(v17);
            match m492 {
                None => {
                    None::<Adt14>
                }
                Some(v19) => {
                    let m493 = f86(v18);
                    match m493 {
                        None => {
                            None::<Adt14>
                        }
                        Some(v20) => {
                            Some(Adt14::C7(v16, std::rc::Rc::new(v19), std::rc::Rc::new(v20)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f87(v0: Option<Adt14>) -> Option<Bytes> {
    let m500 = v0;
    match m500 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m501 = v1;
            match m501 {
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

pub fn f88(p507: Adt14, v1: Bytes) -> Option<Adt14> {
    let m506 = p507;
    {
        let mut m506 = m506;
        loop {
            let step = {
                let v0 = m506;
                let m502 = v0;
                match m502 {
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
                    Adt14::C4(r503) => {
                        let v5 = (*r503).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r504, r505) => {
                        let v3 = (*r504).clone();
                        let v4 = (*r505).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m508 = v0;
    match m508.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m509 = f182(v2, v1.clone())?;
            match m509 {
                None => {
                    f89(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f89(v3, v1)?))
                }
            }
        }
    }
}

pub fn f90(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a510 = v0.clone();
        length_bytes(a510)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h511, _) = f71(Fn0::F183(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h511
    };
    let v3: u64 = {
        let (h512, _) = {
            let (_, h513) = v2.clone();
            h513
        };
        h512
    };
    let v4: List<Bytes> = {
        let (_, h514) = {
            let (_, h515) = v2;
            h515
        };
        h514
    };
    let a516 = v3;
    let a517 = v1;
    if nat_lt(a516, a517) {
        Ok(f73(List::cons(f41(v0, v3, {
            let a518 = v1;
            let a519 = v3;
            nat_sub(a518, a519)
        }), v4)))
    } else {
        Ok(f73(v4))
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f184(f52(v0, v1)?))
}

pub fn f92(v0: List<Adt14>, v1: Bytes) -> List<Adt14> {
    let m520 = v0;
    match m520.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f185(v2, v1.clone()), f92(v3, v1))
        }
    }
}

pub fn f93(p525: List<Bytes>, p526: List<Bytes>) -> bool {
    let m524 = (p525, p526);
    {
        let mut m524 = m524;
        loop {
            let step = {
                let (v0, v1) = m524;
                let m521 = v0;
                match m521.uncons() {
                    None => {
                        let m522 = v1;
                        match m522.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m523 = v1;
                        match m523.uncons() {
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
                Ok(next) => { m524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: List<Adt14>) -> List<Bytes> {
    let m527 = v0;
    match m527.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m528 = f88(v1, f8(Str::lit("std")));
            match m528 {
                None => {
                    f94(v2)
                }
                Some(v3) => {
                    List::cons(f53(v3, f8(Str::lit("authority"))), f94(v2))
                }
            }
        }
    }
}

pub fn f95(v0: Adt14) -> R<Bytes> {
    let a529 = f186(v0)?;
    let a530 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a529, a530))
}

pub fn f96(v0: List<Adt14>) -> Adt14 {
    let m531 = v0;
    match m531.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f96(v2)))
        }
    }
}

pub fn f97(v0: Adt13) -> Bytes {
    let m532 = v0;
    match m532 {
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

pub fn f98() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f99() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f100(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt12> {
    let a534 = {
        let a533 = v2.clone();
        length_bytes(a533)
    };
    let a535 = 0u64;
    if nat_eq(a534, a535) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m536 = f187(v0.clone(), f55())?;
        match m536 {
            None => {
                if f46(f47(f190(), match v0.clone() {
                    Adt1::C0(_, _, h537, _) => {
                        h537
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f190()))
                } else {
                    let m538 = f51(v0.clone(), List::cons(f191(), f192(f55())?));
                    match m538.uncons() {
                        None => {
                            Ok(Adt12::C3(f193(v0.clone(), f55(), v1.clone(), v2.clone(), f194(f59(v0, f191()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f49(f188(v3.clone())?, f189(v3, v2)))
            }
        }
    }
}

pub fn f101() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f102() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f103() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f104() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f105() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f106() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f107() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f108() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f109(v0: Bytes, p574: u64, p575: u64, p576: bool, p577: bool, p578: u64, p579: Bytes) -> R<Bytes> {
    let m573 = (p574, (p575, (p576, (p577, (p578, p579)))));
    {
        let mut m573 = m573;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m573;
                let m539 = v2;
                if m539 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f195(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m539 - 1;
                    let m542 = {
                        let a540 = v0.clone();
                        let a541 = v1;
                        index_bytes(a540, a541)
                    };
                    match m542 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f195(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f67(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a543 = v1;
                                    let a544 = 1u64;
                                    nat_add(a543, a544)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a545 = v9;
                                    let a546 = 34u64;
                                    !nat_eq(a545, a546)
                                }, (if v4 {
                                    false
                                } else {
                                    let a547 = v9;
                                    let a548 = 92u64;
                                    nat_eq(a547, a548)
                                }, (v5, v6))))))
                            } else {
                                let a549 = v9;
                                let a550 = 32u64;
                                if if if if nat_eq(a549, a550) {
                                    true
                                } else {
                                    let a551 = v9;
                                    let a552 = 9u64;
                                    nat_eq(a551, a552)
                                } {
                                    true
                                } else {
                                    let a553 = v9;
                                    let a554 = 10u64;
                                    nat_eq(a553, a554)
                                } {
                                    true
                                } else {
                                    let a555 = v9;
                                    let a556 = 13u64;
                                    nat_eq(a555, a556)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a557 = v1;
                                        let a558 = 1u64;
                                        nat_add(a557, a558)?
                                    }, (v7, (false, (false, ({
                                        let a559 = v1;
                                        let a560 = 1u64;
                                        nat_add(a559, a560)?
                                    }, f195(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    let a561 = v9;
                                    let a562 = 110u64;
                                    if if nat_eq(a561, a562) {
                                        f32(f41(v0.clone(), v1, 4u64), f8(Str::lit("null")))
                                    } else {
                                        false
                                    } {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a563 = v1;
                                            let a564 = 4u64;
                                            nat_add(a563, a564)?
                                        }, (v7, (false, (false, ({
                                            let a565 = v1;
                                            let a566 = 4u64;
                                            nat_add(a565, a566)?
                                        }, {
                                            let a567 = f195(v0.clone(), v5, v1, v6);
                                            let a568 = f8(Str::lit("[]"));
                                            append_bytes(a567, a568)
                                        }))))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                            let a569 = v1;
                                            let a570 = 1u64;
                                            nat_add(a569, a570)?
                                        }, (v7, ({
                                            let a571 = v9;
                                            let a572 = 34u64;
                                            nat_eq(a571, a572)
                                        }, (false, (v5, v6))))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: Bytes, v1: u64) -> R<u64> {
    let m582 = {
        let a580 = v0;
        let a581 = v1;
        index_bytes(a580, a581)
    };
    match m582 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a627 = {
                let a603 = {
                    let a591 = {
                        let a585 = {
                            let a583 = v3;
                            let a584 = 128u8;
                            fixed_u8::bit_and(a583, a584)
                        };
                        let a586 = 128u8;
                        if equal(a585, a586) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a592 = {
                        let a589 = {
                            let a587 = v3;
                            let a588 = 64u8;
                            fixed_u8::bit_and(a587, a588)
                        };
                        let a590 = 64u8;
                        if equal(a589, a590) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a591, a592)?
                };
                let a604 = {
                    let a601 = {
                        let a595 = {
                            let a593 = v3;
                            let a594 = 32u8;
                            fixed_u8::bit_and(a593, a594)
                        };
                        let a596 = 32u8;
                        if equal(a595, a596) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a602 = {
                        let a599 = {
                            let a597 = v3;
                            let a598 = 16u8;
                            fixed_u8::bit_and(a597, a598)
                        };
                        let a600 = 16u8;
                        if equal(a599, a600) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a601, a602)?
                };
                nat_add(a603, a604)?
            };
            let a628 = {
                let a625 = {
                    let a613 = {
                        let a607 = {
                            let a605 = v3;
                            let a606 = 8u8;
                            fixed_u8::bit_and(a605, a606)
                        };
                        let a608 = 8u8;
                        if equal(a607, a608) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a614 = {
                        let a611 = {
                            let a609 = v3;
                            let a610 = 4u8;
                            fixed_u8::bit_and(a609, a610)
                        };
                        let a612 = 4u8;
                        if equal(a611, a612) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a613, a614)?
                };
                let a626 = {
                    let a623 = {
                        let a617 = {
                            let a615 = v3;
                            let a616 = 2u8;
                            fixed_u8::bit_and(a615, a616)
                        };
                        let a618 = 2u8;
                        if equal(a617, a618) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a624 = {
                        let a621 = {
                            let a619 = v3;
                            let a620 = 1u8;
                            fixed_u8::bit_and(a619, a620)
                        };
                        let a622 = 1u8;
                        if equal(a621, a622) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a623, a624)?
                };
                nat_add(a625, a626)?
            };
            nat_add(a627, a628)
        }
    }
}

pub fn f111(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f196(v0.clone(), v1, {
        let a630 = {
            let a629 = v0.clone();
            length_bytes(a629)
        };
        let a631 = 1u64;
        nat_add(a630, a631)?
    })?;
    let a632 = f110(v0.clone(), v2)?;
    let a633 = 34u64;
    if nat_eq(a632, a633) {
        Ok(Some(({
            let a634 = v2;
            let a635 = 1u64;
            nat_add(a634, a635)?
        }, f41(v0, v1, {
            let a636 = v2;
            let a637 = v1;
            nat_sub(a636, a637)
        }))))
    } else {
        f197(v0, v1)
    }
}

pub fn f112(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f113(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a638 = f110(v0.clone(), v1)?;
        let a639 = 45u64;
        nat_eq(a638, a639)
    };
    let v3: u64 = if v2 {
        let a640 = v1;
        let a641 = 1u64;
        nat_add(a640, a641)?
    } else {
        v1
    };
    let v4: u64 = f198(v0.clone(), v3)?;
    let v5: u64 = {
        let a642 = v4;
        let a643 = v3;
        nat_sub(a642, a643)
    };
    let v6: bool = {
        let a644 = f110(v0.clone(), v4)?;
        let a645 = 46u64;
        nat_eq(a644, a645)
    };
    let v7: u64 = if v6 {
        let a646 = v4;
        let a647 = 1u64;
        nat_add(a646, a647)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f198(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f110(v0.clone(), v8)?;
    let v10: bool = {
        let a648 = v9;
        let a649 = 101u64;
        if nat_eq(a648, a649) {
            true
        } else {
            let a650 = v9;
            let a651 = 69u64;
            nat_eq(a650, a651)
        }
    };
    let v11: u64 = f110(v0.clone(), {
        let a652 = v8;
        let a653 = 1u64;
        nat_add(a652, a653)?
    })?;
    let v12: bool = f112(v10, f199({
        let a654 = v11;
        let a655 = 43u64;
        nat_eq(a654, a655)
    }, {
        let a656 = v11;
        let a657 = 45u64;
        nat_eq(a656, a657)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a658 = v8;
            let a659 = 2u64;
            nat_add(a658, a659)?
        } else {
            let a660 = v8;
            let a661 = 1u64;
            nat_add(a660, a661)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f198(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f41(v0.clone(), v13, {
        let a662 = v14;
        let a663 = v13;
        nat_sub(a662, a663)
    });
    let v16: u64 = f200(v15.clone())?;
    let v17: Bytes = f41(v15.clone(), v16, {
        let a665 = {
            let a664 = v15;
            length_bytes(a664)
        };
        let a666 = v16;
        nat_sub(a665, a666)
    });
    let a667 = v5;
    let a668 = 0u64;
    if if nat_eq(a667, a668) {
        true
    } else {
        if f112({
            let a669 = 1u64;
            let a670 = v5;
            nat_lt(a669, a670)
        }, {
            let a671 = f110(v0.clone(), v3)?;
            let a672 = 48u64;
            nat_eq(a671, a672)
        }) {
            true
        } else {
            if f112(v6, {
                let a673 = v8;
                let a674 = v7;
                nat_eq(a673, a674)
            }) {
                true
            } else {
                f199(if v10 {
                    let a675 = v14;
                    let a676 = v13;
                    nat_eq(a675, a676)
                } else {
                    false
                }, {
                    let a678 = 18u64;
                    let a679 = {
                        let a677 = v17.clone();
                        length_bytes(a677)
                    };
                    nat_lt(a678, a679)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m683 = {
            let a681 = {
                let a680 = v17.clone();
                length_bytes(a680)
            };
            let a682 = 0u64;
            if nat_eq(a681, a682) {
                Some(0u64)
            } else {
                f201(v17)?
            }
        };
        match m683 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m688 = f202(f41(v0.clone(), v3, v5), f41(v0, v7, {
                    let a684 = v8;
                    let a685 = v7;
                    nat_sub(a684, a685)
                }), f112(v12, {
                    let a686 = v11;
                    let a687 = 45u64;
                    nat_eq(a686, a687)
                }), v18)?;
                match m688 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a691 = 19u64;
                        let a692 = {
                            let a690 = {
                                let (h689, _) = v19.clone();
                                h689
                            };
                            length_bytes(a690)
                        };
                        if nat_lt(a691, a692) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m694 = f201({
                                let (h693, _) = v19.clone();
                                h693
                            })?;
                            match m694 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f112(v2, {
                                        let a695 = 0u64;
                                        let a696 = v20;
                                        nat_lt(a695, a696)
                                    });
                                    if f203(v21, v20) {
                                        Ok(Some((v14, {
                                            let a698 = {
                                                let (_, h697) = v19.clone();
                                                h697
                                            };
                                            let a699 = 0u64;
                                            if nat_eq(a698, a699) {
                                                Adt2::C1(v21, v20)
                                            } else {
                                                Adt2::C2(v21, v20, {
                                                    let (_, h700) = v19;
                                                    h700
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

pub fn f114(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h701, _) = v2.clone();
        h701
    };
    let v4: u64 = {
        let (h702, _) = {
            let (_, h703) = v2.clone();
            h703
        };
        h702
    };
    let v5: List<Bytes> = {
        let (_, h704) = {
            let (_, h705) = v2;
            h705
        };
        h704
    };
    let m706 = f204(v0.clone(), v3);
    match m706 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a707 = v6;
            let a708 = v1;
            if equal(a707, a708) {
                Ok(Some(({
                    let a709 = v3;
                    let a710 = 1u64;
                    nat_add(a709, a710)?
                }, ({
                    let a711 = v3;
                    let a712 = 1u64;
                    nat_add(a711, a712)?
                }, List::cons(f41(v0, v4, {
                    let a713 = v3;
                    let a714 = v4;
                    nat_sub(a713, a714)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a715 = v3;
                    let a716 = 1u64;
                    nat_add(a715, a716)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f115(v0: Fn1, p720: List<Bytes>, p721: List<Bytes>) -> List<Bytes> {
    let m719 = (p720, p721);
    {
        let mut m719 = m719;
        loop {
            let step = {
                let (v1, v2) = m719;
                let m717 = v2;
                match m717.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c718: Fn1 = v0.clone();
                            c718.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m719 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f117(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m722 = f205(Fn2::F206(v1), None::<Bytes>, v0);
    match m722 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f118() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f119(v0: Adt1) -> List<Bytes> {
    if f145(match v0.clone() {
        Adt1::C0(_, h723, _, _) => {
            h723
        }
    }, f207()) {
        if f145(match v0 {
            Adt1::C0(_, h724, _, _) => {
                h724
            }
        }, f208()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f208(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f207(), List::<Bytes>::nil())
    }
}

pub fn f120(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m725 = v0;
    match m725.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f120(v3, v1))
        }
    }
}

pub fn f121(v0: Adt1) -> R<List<Bytes>> {
    Ok(f51(v0.clone(), List::cons(f209(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f131(v0)?)))))
}

pub fn f122(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f142(v0.clone())?, f58(f59(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f143(f91(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f123(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m726 = v0;
    match m726.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f47(v2.clone(), v1.clone()) {
                f123(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f123(v3, v1))
            }
        }
    }
}

pub fn f124(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m727 = v0;
    match m727.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f47(v2.clone(), v1.clone()) {
                f124(v3, v1)
            } else {
                List::cons(v2, f124(v3, v1))
            }
        }
    }
}

pub fn f125(v0: List<Adt14>) -> List<Bytes> {
    let m728 = v0;
    match m728.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f120(f144(f184(f88(v1, f8(Str::lit("defers"))))), f125(v2))
        }
    }
}

pub fn f126(v0: List<Adt14>) -> List<Bytes> {
    let m729 = v0;
    match m729.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m730 = f88(v1, f8(Str::lit("std")));
            match m730 {
                None => {
                    f126(v2)
                }
                Some(v3) => {
                    List::cons(f53(v3, f8(Str::lit("row"))), f126(v2))
                }
            }
        }
    }
}

pub fn f127(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m731 = v0;
    match m731.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f47(v2.clone(), v1.clone()) {
                f127(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f127(v3, v1))
            }
        }
    }
}

pub fn f128(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f156(f210(v2.clone()), f211(f212(v0, v1.clone(), v2)?));
    f213(f147(f214(v1))?, v3.clone(), {
        let a733 = {
            let a732 = v3;
            length_list(a732)
        };
        let a734 = 1u64;
        nat_add(a733, a734)?
    })
}

pub fn f129(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m735 = v1;
    match m735.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f215(v2.clone(), f8(Str::lit("subject"))) {
                f216(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f129(v0, v3))
            } else {
                f129(v0, v3)
            }
        }
    }
}

pub fn f130(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m736 = v1;
    match m736.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m737 = f217(f90(f59(v0.clone(), v2.clone()))?, f80(v2, f208()))?;
            match m737 {
                None => {
                    f130(v0, v3)
                }
                Some(v4) => {
                    Ok(f218(v4, f130(v0, v3)?))
                }
            }
        }
    }
}

pub fn f131(v0: Adt1) -> R<List<Bytes>> {
    Ok(f120(f147(f219(f207(), f149(match v0.clone() {
        Adt1::C0(_, h738, _, _) => {
            h738
        }
    }, f207())))?, f147(f219(f208(), f149(match v0 {
        Adt1::C0(_, h739, _, _) => {
            h739
        }
    }, f208())))?))
}

pub fn f132(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f133() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f209(), List::<Bytes>::nil())))))))
}

pub fn f134(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m740 = v1;
    match m740.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f62(v0.clone(), v2.clone()) {
                f37(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m741 = f52(v0.clone(), v2.clone())?;
                match m741 {
                    None => {
                        Ok(List::cons(v2, f134(v0, v3)?))
                    }
                    Some(_) => {
                        f134(v0, v3)
                    }
                }
            } else {
                f134(v0, v3)
            }
        }
    }
}

pub fn f135(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m742 = v1;
    match m742.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f145(v0.clone(), v2.clone()) {
                f135(v0, v3)
            } else {
                List::cons(v2, f135(v0, v3))
            }
        }
    }
}

pub fn f136() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f137(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m743 = v0;
    match m743.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f120(f220({
                let (h744, _) = v2;
                h744
            }, {
                let (_, h745) = v2;
                h745
            }, v1), f137(v3, v1))
        }
    }
}

pub fn f138() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f139(v0: Adt1) -> R<Option<Bytes>> {
    f221(f222(v0, f138())?)
}

pub fn f140(v0: Adt1, p749: List<Bytes>) -> R<Option<Bytes>> {
    let m748 = p749;
    {
        let mut m748 = m748;
        loop {
            let step = {
                let v1 = m748;
                let m746 = v1;
                match m746.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m747 = f223(f59(v0.clone(), v2.clone()), f224())?;
                        match m747 {
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
                Ok(next) => { m748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt17) -> R<(bool, Bytes)> {
    let v1: Adt16 = match v0.clone() {
        Adt17::C0(h750, _, _, _, _) => {
            h750
        }
    };
    let v2: Adt18 = f225(v1.clone())?;
    let v3: List<Bytes> = f226(v1.clone());
    let v4: List<Bytes> = f143(match v0.clone() {
        Adt17::C0(_, _, h751, _, _) => {
            h751
        }
    }, f8(Str::lit("path")));
    let m753 = f227(v2, List::<u64>::nil(), f228(v1.clone(), match v0.clone() {
        Adt17::C0(_, h752, _, _, _) => {
            h752
        }
    }, 0u64)?);
    match m753 {
        None => {
            if f230(f231(match v0.clone() {
                Adt17::C0(_, _, _, _, h755) => {
                    h755
                }
            }, f120(match v0.clone() {
                Adt17::C0(_, _, _, _, h756) => {
                    h756
                }
            }, v4.clone()))?, f231(v4.clone(), f120(match v0.clone() {
                Adt17::C0(_, _, _, _, h757) => {
                    h757
                }
            }, v4.clone()))?, f232(match v0.clone() {
                Adt17::C0(_, _, h758, _, _) => {
                    h758
                }
            })) {
                let m762 = f233(f231(match v0.clone() {
                    Adt17::C0(_, _, _, h759, _) => {
                        h759
                    }
                }, f120(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h760, _) => {
                        h760
                    }
                }))?, f231(v3.clone(), f120(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h761, _) => {
                        h761
                    }
                }))?);
                match m762 {
                    None => {
                        let m765 = f234(f143(match v0 {
                            Adt17::C0(_, h764, _, _, _) => {
                                h764
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m765 {
                            None => {
                                let m766 = f234(v4, List::<Bytes>::nil());
                                match m766 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f117(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f229(f120(v3, match v0 {
                            Adt17::C0(_, _, _, h763, _) => {
                                h763
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f229(match v1 {
                Adt16::C0(_, _, _, _, h754) => {
                    h754
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f142(v0: Adt1) -> R<List<Bytes>> {
    let m767 = f235(f59(v0, f209()))?;
    match m767.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f236(v2))
        }
    }
}

pub fn f143(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m768 = v0;
    match m768.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f53(v2, v1.clone()), f143(v3, v1))
        }
    }
}

pub fn f144(v0: List<Adt14>) -> List<Bytes> {
    let m769 = v0;
    match m769.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m770 = f87(Some(v1));
            match m770 {
                None => {
                    f144(v2)
                }
                Some(v3) => {
                    List::cons(v3, f144(v2))
                }
            }
        }
    }
}

pub fn f145(p774: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m773 = p774;
    {
        let mut m773 = m773;
        loop {
            let step = {
                let v0 = m773;
                let m771 = v0;
                match m771.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h772, _) = v2;
                            h772
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m773 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f147(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f237(f115(Fn1::F238, List::<Bytes>::nil(), f239(v0)?)))
}

pub fn f148(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m775 = v1;
    match m775.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f240(v0.clone(), v2), f148(v0, v3))
            } else {
                f148(v0, v3)
            }
        }
    }
}

pub fn f149(p780: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m779 = p780;
    {
        let mut m779 = m779;
        loop {
            let step = {
                let v0 = m779;
                let m776 = v0;
                match m776.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h777, _) = v2;
                            h777
                        }, v1.clone()) {
                            let (_, h778) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h778)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f150(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p783: List<Bytes>) -> R<Option<Bytes>> {
    let m782 = p783;
    {
        let mut m782 = m782;
        loop {
            let step = {
                let v2 = m782;
                let m781 = v2;
                match m781.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f241(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f151(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m784 = v0;
    match m784.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f242(v1.clone())?, v1), f151(v2)?))
        }
    }
}

pub fn f152(p787: List<Bytes>, p788: List<Bytes>, p789: List<List<Bytes>>) -> List<List<Bytes>> {
    let m786 = (p787, (p788, p789));
    {
        let mut m786 = m786;
        loop {
            let step = {
                let (v0, (v1, v2)) = m786;
                let m785 = v0;
                match m785.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f243(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f243(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f153(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m790 = v0;
    match m790.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f47(v2.clone(), v1.clone()) {
                f153(v3, v1)
            } else {
                List::cons(v2, f153(v3, v1))
            }
        }
    }
}

pub fn f154(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f46(f145(match v0.clone() {
        Adt1::C0(_, h791, _, _) => {
            h791
        }
    }, f244())) {
        Ok(Adt12::C1(List::cons(f244(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f153(f147(f148(f244(), f149(match v0.clone() {
            Adt1::C0(_, h792, _, _) => {
                h792
            }
        }, f244())))?, v1.clone());
        let m793 = f51(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m793.uncons() {
            None => {
                let v5: List<Bytes> = f143(f91(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m794 = f51(v0.clone(), f245(v0.clone(), v2.clone(), v5.clone())?);
                match m794.uncons() {
                    None => {
                        let m795 = f246(v0.clone(), v2, v5)?;
                        match m795 {
                            None => {
                                f247(v0, v1)
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

pub fn f155() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f156(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m796 = v0;
    match m796.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f156(v3, v1))
        }
    }
}

pub fn f157(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m797 = f91(v0.clone(), f155())?;
    match m797.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f248(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f158() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f159() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f160() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f249(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f161() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f249(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f162() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f163() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f164(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f122(v0.clone())?;
    let v2: List<Adt14> = f91(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f129(v1.clone(), f130(v0.clone(), f131(v0.clone())?)?);
    let v4: List<List<Bytes>> = f250(f235(f59(v0, f209()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f251(v4.clone(), f236(v4), f212(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f128(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f165(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f252(f122(v0.clone())?, f130(v0.clone(), f131(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f166(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f253(f91(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f254(f91(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f167() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f168(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m798 = f51(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m798.uncons() {
        None => {
            if f255(f90(f59(v0, v1))?)? {
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

pub fn f169(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m799 = f51(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m799.uncons() {
        None => {
            let v5: List<Bytes> = f90(f59(v0.clone(), v1.clone()))?;
            if f46(f37(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f256(v5)?
                } else {
                    f257(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f240(f258(), f259(v1.clone())?), f59(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f240(f258(), f259(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f260()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f170(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m800 = f51(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m800.uncons() {
        None => {
            let m801 = f261(f90(f59(v0.clone(), v1.clone()))?)?;
            match m801 {
                None => {
                    Ok(Adt12::C4(f262()))
                }
                Some(v4) => {
                    let m802 = f263(f90(f59(v0, v1))?)?;
                    match m802 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f117(f44(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f264(v5)))
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

pub fn f171(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m803 = f51(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m803.uncons() {
        None => {
            let m804 = f265(f90(f59(v0, v1))?)?;
            match m804 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f117(f44(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f172(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m805 = f51(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m805.uncons() {
        None => {
            let v5: Bytes = f240(if v2 {
                f208()
            } else {
                f207()
            }, f259(v1.clone())?);
            if f46(f37(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m806 = f217(f90(f59(v0.clone(), v1.clone()))?, v2)?;
                match m806 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a808 = {
                            let a807 = v6;
                            length_list(a807)
                        };
                        let a809 = 0u64;
                        if nat_eq(a808, a809) {
                            Ok(Adt12::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt12::C3(List::cons((v5.clone(), f59(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f173(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m810 = f51(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m810.uncons() {
        None => {
            let v4: List<Bytes> = f90(f59(v0.clone(), v1))?;
            let m811 = f266(v4.clone(), f8(Str::lit("requirement")))?;
            match m811 {
                None => {
                    Ok(Adt12::C4(f260()))
                }
                Some(v5) => {
                    let m812 = f266(v4, f8(Str::lit("admission")))?;
                    match m812 {
                        None => {
                            Ok(Adt12::C4(f260()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f91(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f91(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f120(f143(v5.clone(), f8(Str::lit("id"))), f143(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f147(f120(f143(v7.clone(), f8(Str::lit("id"))), f143(v8.clone(), f8(Str::lit("construct")))))?;
                            let m813 = f267(f126(v6.clone()), f125(f91(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m813 {
                                None => {
                                    let a815 = {
                                        let a814 = v9.clone();
                                        length_list(a814)
                                    };
                                    let a816 = 0u64;
                                    if nat_eq(a815, a816) {
                                        Ok(Adt12::C4(f260()))
                                    } else {
                                        if f46({
                                            let a819 = {
                                                let a817 = f147(f120(v9.clone(), v10.clone()))?;
                                                length_list(a817)
                                            };
                                            let a820 = {
                                                let a818 = f120(v9.clone(), v10);
                                                length_list(a818)
                                            };
                                            nat_eq(a819, a820)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f269(f8(Str::lit("model/requirements.json")), f218(v7, v5))?, List::cons(f269(f8(Str::lit("model/admissions.json")), f218(v8, v6))?, List::cons(f270(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f117(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f268(v11))
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

pub fn f174(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m821 = f51(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m821.uncons() {
        None => {
            let m822 = f85(v0.clone(), v1.clone())?;
            match m822 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m823 = f271(v4.clone(), f8(Str::lit("schema")));
                    match m823 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f272(v0, f273(v5.clone())?, v5, f274(v4.clone(), f8(Str::lit("name"))), f275(f276(f271(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f175(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m824 = f51(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m824.uncons() {
        None => {
            let m825 = f52(v0.clone(), v1.clone())?;
            match m825 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f46(f32(f53(v4.clone(), f8(Str::lit("schema"))), f277())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f53(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m826 = f88(v4.clone(), f8(Str::lit("source")));
                            match m826 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f87(f88(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f240(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f240(v9.clone(), f8(Str::lit("binding.json"))), f59(v0.clone(), v1));
                        let m827 = v8;
                        match m827 {
                            None => {
                                f278(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f46(f32(f53(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f278(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f240(v9, f259(f53(v7.clone(), f8(Str::lit("url"))))?);
                                    if f46(f47(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h828, _) => {
                                            h828
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f53(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m829 = f51(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m829.uncons() {
                                            None => {
                                                if f32(f279(f59(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f278(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f176(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m830 = f51(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m830.uncons() {
        None => {
            let v4: Bytes = f59(v0.clone(), v1);
            let v5: List<List<Bytes>> = f235(v4.clone())?;
            if f46(f93(f280(v5.clone()), f281())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f282(Str::lit("id"), v6.clone())?, List::cons(f282(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f282(Str::lit("artifact"), f209())?, List::cons(f282(Str::lit("checksum"), f279(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f209(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f283(f59(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f270(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f117(f236(f250(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f177(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m831 = f51(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m831.uncons() {
        None => {
            let m832 = f52(v0, v1)?;
            match m832 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f284(f184(Some(v4))) {
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

pub fn f178(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m833 = f51(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m833.uncons() {
        None => {
            let m834 = f85(v0.clone(), v1.clone())?;
            match m834 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f272(v0.clone(), f59(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f179(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m835 = f51(v0.clone(), List::cons(v1.clone(), List::cons(f155(), List::<Bytes>::nil())));
    match m835.uncons() {
        None => {
            let m836 = f285(f90(f59(v0.clone(), v1))?)?;
            match m836 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f91(v0, f155())?;
                    let v6: List<Bytes> = f143(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f147(f143(v5.clone(), f8(Str::lit("at"))))?;
                    let a838 = {
                        let a837 = v6.clone();
                        length_list(a837)
                    };
                    let a839 = 0u64;
                    if if nat_eq(a838, a839) {
                        true
                    } else {
                        f46({
                            let a842 = {
                                let a840 = f147(f120(v6.clone(), v7.clone()))?;
                                length_list(a840)
                            };
                            let a843 = {
                                let a841 = f120(v6.clone(), v7);
                                length_list(a841)
                            };
                            nat_eq(a842, a843)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f269(f155(), f218(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f117(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f180(v0: Bytes, p897: u64, p898: u64) -> R<bool> {
    let m896 = (p897, p898);
    {
        let mut m896 = m896;
        loop {
            let step = {
                let (v1, v2) = m896;
                let m844 = v2;
                if m844 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m844 - 1;
                    let m847 = {
                        let a845 = v0.clone();
                        let a846 = v1;
                        index_bytes(a845, a846)
                    };
                    match m847 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f286({
                                let v5: u8 = v4;
                                let a892 = {
                                    let a868 = {
                                        let a856 = {
                                            let a850 = {
                                                let a848 = v5;
                                                let a849 = 128u8;
                                                fixed_u8::bit_and(a848, a849)
                                            };
                                            let a851 = 128u8;
                                            if equal(a850, a851) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a857 = {
                                            let a854 = {
                                                let a852 = v5;
                                                let a853 = 64u8;
                                                fixed_u8::bit_and(a852, a853)
                                            };
                                            let a855 = 64u8;
                                            if equal(a854, a855) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a856, a857)?
                                    };
                                    let a869 = {
                                        let a866 = {
                                            let a860 = {
                                                let a858 = v5;
                                                let a859 = 32u8;
                                                fixed_u8::bit_and(a858, a859)
                                            };
                                            let a861 = 32u8;
                                            if equal(a860, a861) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a867 = {
                                            let a864 = {
                                                let a862 = v5;
                                                let a863 = 16u8;
                                                fixed_u8::bit_and(a862, a863)
                                            };
                                            let a865 = 16u8;
                                            if equal(a864, a865) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a866, a867)?
                                    };
                                    nat_add(a868, a869)?
                                };
                                let a893 = {
                                    let a890 = {
                                        let a878 = {
                                            let a872 = {
                                                let a870 = v5;
                                                let a871 = 8u8;
                                                fixed_u8::bit_and(a870, a871)
                                            };
                                            let a873 = 8u8;
                                            if equal(a872, a873) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a879 = {
                                            let a876 = {
                                                let a874 = v5;
                                                let a875 = 4u8;
                                                fixed_u8::bit_and(a874, a875)
                                            };
                                            let a877 = 4u8;
                                            if equal(a876, a877) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a878, a879)?
                                    };
                                    let a891 = {
                                        let a888 = {
                                            let a882 = {
                                                let a880 = v5;
                                                let a881 = 2u8;
                                                fixed_u8::bit_and(a880, a881)
                                            };
                                            let a883 = 2u8;
                                            if equal(a882, a883) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a889 = {
                                            let a886 = {
                                                let a884 = v5;
                                                let a885 = 1u8;
                                                fixed_u8::bit_and(a884, a885)
                                            };
                                            let a887 = 1u8;
                                            if equal(a886, a887) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a888, a889)?
                                    };
                                    nat_add(a890, a891)?
                                };
                                nat_add(a892, a893)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a894 = v1;
                                    let a895 = 1u64;
                                    nat_add(a894, a895)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(p903: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m902 = p903;
    {
        let mut m902 = m902;
        loop {
            let step = {
                let v0 = m902;
                let m899 = v0;
                match m899.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h900, _) = v2;
                            h900
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h901) = v2;
                                h901
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m902 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f80(v0.clone(), v1) {
        let m904 = f39(v0, 34u8)?;
        match m904.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m905 = v3;
                match m905.uncons() {
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

pub fn f183(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f287(v0, v1)
}

pub fn f184(v0: Option<Adt14>) -> List<Adt14> {
    let m906 = v0;
    match m906 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f288(v1)
        }
    }
}

pub fn f185(v0: Adt14, v1: Bytes) -> Adt14 {
    let m907 = v0;
    match m907 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r908) => {
            let v10 = (*r908).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r909) => {
            let v5 = (*r909).clone();
            Adt14::C4(std::rc::Rc::new(f185(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r910, r911) => {
            let v11 = (*r910).clone();
            let v12 = (*r911).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r912, r913) => {
            let v3 = (*r912).clone();
            let v4 = (*r913).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt14::C7(v2, std::rc::Rc::new(f289(v3, v1.clone())), std::rc::Rc::new(f185(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f185(v4, v1)))
            }
        }
    }
}

pub fn f186(v0: Adt14) -> R<Bytes> {
    let m914 = v0;
    match m914 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f290(v2, v3)
        }
        Adt14::C2(v4) => {
            f291(v4)
        }
        Adt14::C3(r915) => {
            let v5 = (*r915).clone();
            Ok(f292(f293(v5)?))
        }
        Adt14::C4(r916) => {
            let v6 = (*r916).clone();
            f294(f295(v6)?)
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

pub fn f187(v0: Adt1, p920: List<Adt13>) -> R<Option<Adt13>> {
    let m919 = p920;
    {
        let mut m919 = m919;
        loop {
            let step = {
                let v1 = m919;
                let m917 = v1;
                match m917.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f47(f188(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h918, _) => {
                                h918
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
                Ok(next) => { m919 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Adt13) -> R<Bytes> {
    Ok(f240(f296(), f259(f297(v0))?))
}

pub fn f189(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f297(v0), List::<Bytes>::nil())))))
}

pub fn f190() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f191() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f192(v0: List<Adt13>) -> R<List<Bytes>> {
    let m921 = v0;
    match m921.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f188(v1)?, f192(v2)?))
        }
    }
}

pub fn f193(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m922 = v1;
    match m922.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f63(f64(v5.clone(), f56(v2.clone()))), f298(v5.clone(), v2.clone(), v3.clone(), f279(f59(v0.clone(), f188(v5)?))?, v4.clone())?), f193(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f194(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f299(v0.clone())?;
    let v2: u64 = f300(v0.clone(), v1)?;
    Ok(f41(v0, v2, {
        let a923 = v1;
        let a924 = v2;
        nat_sub(a923, a924)
    }))
}

pub fn f195(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a927 = v3;
    let a928 = f41(v0, v1, {
        let a925 = v2;
        let a926 = v1;
        nat_sub(a925, a926)
    });
    append_bytes(a927, a928)
}

pub fn f196(v0: Bytes, p941: u64, p942: u64) -> R<u64> {
    let m940 = (p941, p942);
    {
        let mut m940 = m940;
        loop {
            let step = {
                let (v1, v2) = m940;
                let m929 = v2;
                if m929 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m929 - 1;
                    let v4: u64 = f110(v0.clone(), v1)?;
                    let a930 = v4;
                    let a931 = 34u64;
                    if nat_eq(a930, a931) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a932 = v4;
                        let a933 = 92u64;
                        if nat_eq(a932, a933) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a934 = v4;
                            let a935 = 32u64;
                            if nat_lt(a934, a935) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a936 = v4;
                                let a937 = 256u64;
                                if nat_eq(a936, a937) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a938 = v1;
                                        let a939 = 1u64;
                                        nat_add(a938, a939)?
                                    }, v3))
                                }
                            }
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

pub fn f197(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f301(Fn3::F302(v0.clone()), {
        let a944 = {
            let a943 = v0;
            length_bytes(a943)
        };
        let a945 = 1u64;
        nat_add(a944, a945)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a949 = {
        let (_, h946) = {
            let (_, h947) = {
                let (h948, _) = v2.clone();
                h948
            };
            h947
        };
        h946
    };
    let a950 = 1u64;
    if nat_eq(a949, a950) {
        Ok(Some(({
            let (h951, _) = {
                let (h952, _) = v2.clone();
                h952
            };
            h951
        }, {
            let (h953, _) = {
                let (_, h954) = {
                    let (h955, _) = v2;
                    h955
                };
                h954
            };
            h953
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f198(v0: Bytes, v1: u64) -> R<u64> {
    let (h956, _) = f303(Fn4::F304(v0.clone()), {
        let a958 = {
            let a957 = v0;
            length_bytes(a957)
        };
        let a959 = 1u64;
        nat_add(a958, a959)?
    }, v1)?;
    Ok(h956)
}

pub fn f199(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f200(v0: Bytes) -> R<u64> {
    let (h960, _) = f303(Fn4::F305(v0.clone()), {
        let a962 = {
            let a961 = v0;
            length_bytes(a961)
        };
        let a963 = 1u64;
        nat_add(a962, a963)?
    }, 0u64)?;
    Ok(h960)
}

pub fn f201(v0: Bytes) -> R<Option<u64>> {
    if f306(v0.clone())? {
        Ok(Some({
            let (_, h964) = {
                let (h965, _) = f307(Fn5::F308(v0.clone()), {
                    let a966 = v0;
                    length_bytes(a966)
                }, (0u64, 0u64))?;
                h965
            };
            h964
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f202(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f41(v1.clone(), 0u64, {
        let a968 = {
            let a967 = v1.clone();
            length_bytes(a967)
        };
        let a969 = f309(v1)?;
        nat_sub(a968, a969)
    });
    let v5: Bytes = {
        let a970 = v0;
        let a971 = v4.clone();
        append_bytes(a970, a971)
    };
    let v6: u64 = f200(v5.clone())?;
    let v7: Bytes = f41(v5.clone(), v6, {
        let a973 = {
            let a972 = v5;
            length_bytes(a972)
        };
        let a974 = v6;
        nat_sub(a973, a974)
    });
    let v8: u64 = {
        let a975 = v4;
        length_bytes(a975)
    };
    let v9: u64 = {
        let a976 = v7.clone();
        length_bytes(a976)
    };
    let a977 = v9;
    let a978 = 0u64;
    if nat_eq(a977, a978) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f199(v2, {
            let a979 = v3;
            let a980 = v8;
            nat_lt(a979, a980)
        }) {
            let v10: u64 = if v2 {
                let a981 = v3;
                let a982 = v8;
                nat_add(a981, a982)?
            } else {
                let a983 = v8;
                let a984 = v3;
                nat_sub(a983, a984)
            };
            let v11: u64 = f309(v7.clone())?;
            let a985 = v10;
            let a986 = v11;
            if nat_le(a985, a986) {
                Ok(Some((f41(v7, 0u64, {
                    let a987 = v9;
                    let a988 = v10;
                    nat_sub(a987, a988)
                }), 0u64)))
            } else {
                let a991 = {
                    let a989 = v10;
                    let a990 = v11;
                    nat_sub(a989, a990)
                };
                let a992 = 19u64;
                if nat_le(a991, a992) {
                    Ok(Some((f41(v7, 0u64, {
                        let a993 = v9;
                        let a994 = v11;
                        nat_sub(a993, a994)
                    }), {
                        let a995 = v10;
                        let a996 = v11;
                        nat_sub(a995, a996)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a997 = v3;
                let a998 = v8;
                nat_sub(a997, a998)
            };
            let a1001 = {
                let a999 = v9;
                let a1000 = v12;
                nat_add(a999, a1000)?
            };
            let a1002 = 19u64;
            if nat_le(a1001, a1002) {
                Ok(Some(({
                    let a1003 = v7;
                    let a1004 = f310(v12);
                    append_bytes(a1003, a1004)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f203(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1005 = 0u64;
        let a1006 = v1;
        if nat_lt(a1005, a1006) {
            let a1007 = v1;
            let a1008 = 9223372036854775808u64;
            nat_le(a1007, a1008)
        } else {
            false
        }
    } else {
        let a1009 = v1;
        let a1010 = 9223372036854775807u64;
        nat_le(a1009, a1010)
    }
}

pub fn f204(v0: Bytes, v1: u64) -> Option<u8> {
    let a1011 = v0;
    let a1012 = v1;
    index_bytes(a1011, a1012)
}

pub fn f205(v0: Fn2, p1016: Option<Bytes>, p1017: List<Bytes>) -> Option<Bytes> {
    let m1015 = (p1016, p1017);
    {
        let mut m1015 = m1015;
        loop {
            let step = {
                let (v1, v2) = m1015;
                let m1013 = v2;
                match m1013.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1014: Fn2 = v0.clone();
                            c1014.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1015 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1018 = v1;
    match m1018 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1021 = {
                    let a1019 = v3;
                    let a1020 = v0;
                    append_bytes(a1019, a1020)
                };
                let a1022 = v2;
                append_bytes(a1021, a1022)
            })
        }
    }
}

pub fn f207() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f208() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f209() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f210(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1023 = v0;
    match m1023.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f53(v1.clone(), f8(Str::lit("subject"))), f53(v1, f8(Str::lit("object")))), f210(v2))
        }
    }
}

pub fn f211(v0: List<Adt19>) -> List<(Bytes, Bytes)> {
    let m1024 = v0;
    match m1024.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt19::C0(h1025, _, _) => {
                    h1025
                }
            }, match v1 {
                Adt19::C0(_, _, h1026) => {
                    h1026
                }
            }), f211(v2))
        }
    }
}

pub fn f212(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt19>> {
    let v3: List<(Bytes, Bytes)> = f311(v1);
    let v4: List<(Bytes, Bytes)> = f312(v0, v2);
    f313(v3.clone(), f156(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f213(p1033: List<Bytes>, v1: List<(Bytes, Bytes)>, p1034: u64) -> R<List<Bytes>> {
    let m1032 = (p1033, p1034);
    {
        let mut m1032 = m1032;
        loop {
            let step = {
                let (v0, v2) = m1032;
                let m1027 = v2;
                if m1027 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1027 - 1;
                    let v4: List<Bytes> = f147(f120(v0.clone(), f314(v0.clone(), v1.clone())))?;
                    let a1030 = {
                        let a1028 = v4.clone();
                        length_list(a1028)
                    };
                    let a1031 = {
                        let a1029 = v0.clone();
                        length_list(a1029)
                    };
                    if nat_eq(a1030, a1031) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: List<Adt14>) -> List<Bytes> {
    let m1035 = v0;
    match m1035.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1036 = f88(v1, f8(Str::lit("std")));
            match m1036 {
                None => {
                    f214(v2)
                }
                Some(v3) => {
                    List::cons(f53(v3.clone(), f8(Str::lit("authority"))), if f215(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f53(v3, f8(Str::lit("row"))), f214(v2))
                    } else {
                        f214(v2)
                    })
                }
            }
        }
    }
}

pub fn f215(v0: Adt14, v1: Bytes) -> bool {
    f46({
        let a1038 = {
            let a1037 = f53(v0, v1);
            length_bytes(a1037)
        };
        let a1039 = 0u64;
        nat_eq(a1038, a1039)
    })
}

pub fn f216(v0: Adt15, v1: Adt14) -> bool {
    if f315(v0.clone(), v1.clone(), f53(v1.clone(), f8(Str::lit("subject")))) {
        f315(v0, v1.clone(), f53(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f217(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m1040 = v0;
    match m1040.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f194(v2)?;
            let a1042 = {
                let a1041 = v4.clone();
                length_bytes(a1041)
            };
            let a1043 = 0u64;
            if nat_eq(a1042, a1043) {
                f217(v3, v1)
            } else {
                let m1044 = f316(v4)?;
                match m1044 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f317(v5.clone())
                        } else {
                            f318(v5.clone())
                        } {
                            let m1045 = f217(v3, v1)?;
                            match m1045 {
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

pub fn f218(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m1046 = v0;
    match m1046.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f218(v3, v1))
        }
    }
}

pub fn f219(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1047 = v1;
    match m1047.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f37(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f240(v0.clone(), v2), f219(v0, v3))
            } else {
                f219(v0, v3)
            }
        }
    }
}

pub fn f220(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1048 = v1;
    match m1048.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f319(v3.clone())
            } else {
                f46(f319(v3.clone()))
            } {
                List::cons(f240(v0.clone(), v3), f220(v0, v4, v2))
            } else {
                f220(v0, v4, v2)
            }
        }
    }
}

pub fn f221(p1051: List<Bytes>) -> R<Option<Bytes>> {
    let m1050 = p1051;
    {
        let mut m1050 = m1050;
        loop {
            let step = {
                let v0 = m1050;
                let m1049 = v0;
                match m1049.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f320(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f194(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1052 = v1;
    match m1052.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f120(f90(f59(v0.clone(), v2))?, f222(v0, v3)?))
        }
    }
}

pub fn f223(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1053 = v0.clone();
        length_bytes(a1053)
    };
    let v3: u64 = {
        let a1054 = v1.clone();
        length_bytes(a1054)
    };
    let v4: u64 = {
        let (h1055, _) = f303(Fn4::F321(v1.clone(), v3, v0.clone(), v2), {
            let a1056 = v2;
            let a1057 = 1u64;
            nat_add(a1056, a1057)?
        }, 0u64)?;
        h1055
    };
    if f322(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f224() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f225(v0: Adt16) -> R<Adt18> {
    Ok(Adt18::C0(f323(match v0.clone() {
        Adt16::C0(h1058, _, _, _, _) => {
            h1058
        }
    }, 0u64)?, f323(match v0.clone() {
        Adt16::C0(_, h1059, _, _, _) => {
            h1059
        }
    }, 0u64)?, f323(match v0.clone() {
        Adt16::C0(_, _, h1060, _, _) => {
            h1060
        }
    }, 0u64)?, f323(match v0 {
        Adt16::C0(_, _, _, h1061, _) => {
            h1061
        }
    }, 0u64)?))
}

pub fn f226(v0: Adt16) -> List<Bytes> {
    f120(match v0.clone() {
        Adt16::C0(h1062, _, _, _, _) => {
            h1062
        }
    }, f120(match v0.clone() {
        Adt16::C0(_, _, h1063, _, _) => {
            h1063
        }
    }, f120(match v0.clone() {
        Adt16::C0(_, _, _, h1064, _) => {
            h1064
        }
    }, match v0 {
        Adt16::C0(_, _, _, _, h1065) => {
            h1065
        }
    })))
}

pub fn f227(v0: Adt18, p1071: List<u64>, p1072: List<Adt20>) -> Option<u64> {
    let m1070 = (p1071, p1072);
    {
        let mut m1070 = m1070;
        loop {
            let step = {
                let (v1, v2) = m1070;
                let m1066 = v2;
                match m1066.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f324(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1067) => {
                                h1067
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1068, _) => {
                                    h1068
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1069, _) => {
                                    h1069
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1070 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228(v0: Adt16, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m1073 = v1;
    match m1073.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f325(v0.clone(), v3)?), f228(v0, v4, {
                let a1074 = v2;
                let a1075 = 1u64;
                nat_add(a1074, a1075)?
            })?))
        }
    }
}

pub fn f229(p1079: List<Bytes>, p1080: u64) -> Bytes {
    let m1078 = (p1079, p1080);
    {
        let mut m1078 = m1078;
        loop {
            let step = {
                let (v0, v1) = m1078;
                let m1076 = v0;
                match m1076.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1077 = v1;
                        if m1077 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1077 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f326(v0.clone(), v1.clone()) {
        if f326(v1, v0) {
            f327(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f231(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1081 = v0;
    match m1081.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f328(v2, v1.clone())?, f231(v3, v1)?))
        }
    }
}

pub fn f232(v0: List<Adt14>) -> List<u64> {
    let m1082 = v0;
    match m1082.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f329(v1), f232(v2))
        }
    }
}

pub fn f233(p1085: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1084 = p1085;
    {
        let mut m1084 = m1084;
        loop {
            let step = {
                let v0 = m1084;
                let m1083 = v0;
                match m1083.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f330(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(p1088: List<Bytes>, p1089: List<Bytes>) -> Option<Bytes> {
    let m1087 = (p1088, p1089);
    {
        let mut m1087 = m1087;
        loop {
            let step = {
                let (v0, v1) = m1087;
                let m1086 = v0;
                match m1086.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f47(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1087 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt23 = f331(v0.clone(), 0u64, {
        let a1090 = v0;
        length_bytes(a1090)
    }, Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt23 = {
        let a1093 = {
            let a1092 = match v1.clone() {
                Adt23::C0(_, _, h1091, _, _) => {
                    h1091
                }
            };
            length_bytes(a1092)
        };
        let a1094 = 0u64;
        if if nat_eq(a1093, a1094) {
            let m1096 = match v1.clone() {
                Adt23::C0(_, _, _, h1095, _) => {
                    h1095
                }
            };
            match m1096.uncons() {
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
            f332(v1)
        }
    };
    Ok(f333(match v4 {
        Adt23::C0(_, _, _, _, h1097) => {
            h1097
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f236(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1098 = v0;
    match m1098.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1099 = v1;
            match m1099.uncons() {
                None => {
                    f236(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f236(v2))
                }
            }
        }
    }
}

pub fn f237(v0: List<Bytes>) -> List<Bytes> {
    f115(Fn1::F334, List::<Bytes>::nil(), v0)
}

pub fn f238(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f335(v0, v1)
}

pub fn f239(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f336(f337(f338(v0))?))
}

pub fn f240(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f241(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1100 = f52(v0, v2)?;
    match m1100 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f339(v1, f53(v3.clone(), f8(Str::lit("id"))))?;
            let m1101 = v4.clone();
            match m1101.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1103 = f87(f88({
                        let m1102 = f88(v3.clone(), f8(Str::lit("source")));
                        match m1102 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1103 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f46(f47(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f340(f341(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f242(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1104 = v0;
    match m1104.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f342(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1108 = f39(f41(v1.clone(), 6u64, {
                    let a1106 = {
                        let a1105 = v1;
                        length_bytes(a1105)
                    };
                    let a1107 = 6u64;
                    nat_sub(a1106, a1107)
                }), 34u8)?;
                match m1108.uncons() {
                    None => {
                        f242(v2)
                    }
                    Some((v3, v4)) => {
                        let m1109 = v4;
                        match m1109.uncons() {
                            None => {
                                f242(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f242(v2)?))
                            }
                        }
                    }
                }
            } else {
                f242(v2)
            }
        }
    }
}

pub fn f243(p1112: List<Bytes>, p1113: List<Bytes>) -> List<Bytes> {
    let m1111 = (p1112, p1113);
    {
        let mut m1111 = m1111;
        loop {
            let step = {
                let (v0, v1) = m1111;
                let m1110 = v0;
                match m1110.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f245(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1114 = v1;
    match m1114.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f343(v0.clone(), v3)?;
            if f47(f344(f274(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f345(v5)?, f245(v0, v4, v2)?))
            } else {
                f245(v0, v4, v2)
            }
        }
    }
}

pub fn f246(v0: Adt1, p1118: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1117 = p1118;
    {
        let mut m1117 = m1117;
        loop {
            let step = {
                let v1 = m1117;
                let m1115 = v1;
                match m1115.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f343(v0.clone(), v3.clone())?;
                        if if f47(f344(f274(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f273({
                                let m1116 = f271(v5.clone(), f8(Str::lit("schema")));
                                match m1116 {
                                    None => {
                                        Adt2::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f59(v0.clone(), f345(v5)?))
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
                Ok(next) => { m1117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f247(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f46(f145(match v0.clone() {
        Adt1::C0(_, h1119, _, _) => {
            h1119
        }
    }, f258())) {
        Ok(Adt12::C1(List::cons(f258(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f153(f147(f346(f149(match v0.clone() {
            Adt1::C0(_, h1120, _, _) => {
                h1120
            }
        }, f258())))?, v1);
        let m1121 = f51(v0.clone(), List::cons(f155(), List::cons(f209(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1121.uncons() {
            None => {
                let v5: (bool, Bytes) = f347(v0.clone(), v2)?;
                let (h1122, _) = v5;
                if h1122 {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h1123) = v5;
                        h1123
                    }))
                } else {
                    let m1124 = f119(v0.clone());
                    match m1124.uncons() {
                        None => {
                            let m1125 = f121(v0.clone())?;
                            match m1125.uncons() {
                                None => {
                                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h1126) = v5;
                                        h1126
                                    }, List::cons(f7(f147(f348(f122(v0.clone())?, f130(v0.clone(), f131(v0.clone())?)?))?), List::cons(f349(v0)?, List::<Bytes>::nil()))))))
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

pub fn f248(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f350(f91(v0, f155())?)?)))
}

pub fn f249() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f250(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1127 = v0;
    match m1127.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f251(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt19>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m1128 = v1;
    match m1128.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f120(f351(v5.clone(), v2.clone()), f352(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f353(v0.clone(), v5.clone()), List::cons(if f47(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1130 = {
                    let a1129 = v7.clone();
                    length_list(a1129)
                };
                let a1131 = 0u64;
                if nat_eq(a1130, a1131) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f117(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f251(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f252(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1132 = v1;
    match m1132.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f215(v2.clone(), f8(Str::lit("domain"))) {
                f354(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f53(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f53(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1133 = f88(v2.clone(), f8(Str::lit("role")));
                    match m1133 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f53(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f53(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f53(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f355(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1134 = f88(v2.clone(), f8(Str::lit("origin")));
                    match m1134 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f53(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1135 = f88(v2, f8(Str::lit("evidence")));
                    match m1135 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f53(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f252(v0, v3))
            } else {
                f252(v0, v3)
            }
        }
    }
}

pub fn f253(v0: List<Adt14>) -> List<Bytes> {
    let m1136 = v0;
    match m1136.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f53(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f53(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f53(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f253(v2))
        }
    }
}

pub fn f254(v0: List<Adt14>) -> List<Bytes> {
    let m1137 = v0;
    match m1137.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f53(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f356(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f254(v2))
        }
    }
}

pub fn f255(p1143: List<Bytes>) -> R<bool> {
    let m1142 = p1143;
    {
        let mut m1142 = m1142;
        loop {
            let step = {
                let v0 = m1142;
                let m1138 = v0;
                match m1138.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1140 = {
                            let a1139 = f194(v1)?;
                            length_bytes(a1139)
                        };
                        let a1141 = 0u64;
                        if nat_eq(a1140, a1141) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
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

pub fn f256(v0: List<Bytes>) -> R<bool> {
    if f357(f285(v0.clone())?) {
        Ok(f46({
            let a1145 = {
                let a1144 = f358(f285(v0)?);
                length_list(a1144)
            };
            let a1146 = 0u64;
            nat_eq(a1145, a1146)
        }))
    } else {
        Ok(false)
    }
}

pub fn f257(v0: List<Bytes>) -> R<bool> {
    if f357(f266(v0.clone(), f8(Str::lit("requirement")))?) {
        if f357(f266(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f46({
                let a1148 = {
                    let a1147 = f218(f358(f266(v0.clone(), f8(Str::lit("requirement")))?), f358(f266(v0, f8(Str::lit("admission")))?));
                    length_list(a1147)
                };
                let a1149 = 0u64;
                nat_eq(a1148, a1149)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f258() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f259(v0: Bytes) -> R<Bytes> {
    Ok(f40(f39(v0, 47u8)?))
}

pub fn f260() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f261(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1150 = v0;
    match m1150.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f194(v1)?;
            let a1152 = {
                let a1151 = v3.clone();
                length_bytes(a1151)
            };
            let a1153 = 0u64;
            if nat_eq(a1152, a1153) {
                f261(v2)
            } else {
                let m1154 = f316(v3.clone())?;
                match m1154 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f53(v4.clone(), f8(Str::lit("id")));
                        if f359(f53(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1155 = f261(v2)?;
                            match m1155 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f360(v5), {
                                        let a1156 = v3;
                                        let a1157 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1156, a1157)
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

pub fn f262() -> Bytes {
    let a1158 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1158)
}

pub fn f263(p1163: List<Bytes>) -> R<Option<Bytes>> {
    let m1162 = p1163;
    {
        let mut m1162 = m1162;
        loop {
            let step = {
                let v0 = m1162;
                let m1159 = v0;
                match m1159.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1160 = f316(f194(v1)?)?;
                        match m1160 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1161 = f88(v3.clone(), f8(Str::lit("source")));
                                match m1161 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f361(f53(v4.clone(), f8(Str::lit("redistribution"))), f53(v4, f8(Str::lit("url"))))? {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f53(v3, f8(Str::lit("id"))))))
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
                Ok(next) => { m1162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Bytes) -> Bytes {
    let a1168 = {
        let a1164 = Str::lit("binding ");
        utf8_encode(a1164)
    };
    let a1169 = {
        let a1166 = v0;
        let a1167 = {
            let a1165 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1165)
        };
        append_bytes(a1166, a1167)
    };
    append_bytes(a1168, a1169)
}

pub fn f265(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1170 = v0;
    match m1170.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f194(v1)?;
            let a1172 = {
                let a1171 = v3.clone();
                length_bytes(a1171)
            };
            let a1173 = 0u64;
            if nat_eq(a1172, a1173) {
                f265(v2)
            } else {
                let m1174 = f38(v3.clone())?;
                match m1174 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f362(v4.clone()) {
                            let m1175 = f265(v2)?;
                            match m1175 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f244(), List::cons(f8(Str::lit("/")), List::cons(f363(f274(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1176 = v3;
                                        let a1177 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1176, a1177)
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

pub fn f266(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m1178 = v0;
    match m1178.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f194(v2)?;
            let a1180 = {
                let a1179 = v4.clone();
                length_bytes(a1179)
            };
            let a1181 = 0u64;
            if nat_eq(a1180, a1181) {
                f266(v3, v1)
            } else {
                let m1182 = f316(v4)?;
                match m1182 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f364(v5.clone());
                        let a1184 = {
                            let a1183 = v6.clone();
                            length_bytes(a1183)
                        };
                        let a1185 = 0u64;
                        if nat_eq(a1184, a1185) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m1186 = f266(v3, v1.clone())?;
                            match m1186 {
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

pub fn f267(p1189: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1188 = p1189;
    {
        let mut m1188 = m1188;
        loop {
            let step = {
                let v0 = m1188;
                let m1187 = v0;
                match m1187.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f47(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
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

pub fn f268(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f269(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f95(Adt14::C3(std::rc::Rc::new(f96(v1))))?))
}

pub fn f270(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f95(Adt14::C3(std::rc::Rc::new(f365(f147(f120(v1, f144(f91(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f271(p1195: Adt2, v1: Bytes) -> Option<Adt2> {
    let m1194 = p1195;
    {
        let mut m1194 = m1194;
        loop {
            let step = {
                let v0 = m1194;
                let m1190 = v0;
                match m1190 {
                    Adt2::C0(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C1(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C2(_, _, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C3(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C4(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C5(r1191) => {
                        let v5 = (*r1191).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C6 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C8(v2, r1192, r1193) => {
                        let v3 = (*r1192).clone();
                        let v4 = (*r1193).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
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

pub fn f272(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1196 = f51(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1196.uncons() {
        None => {
            let m1198 = f267(f126(List::cons(Adt14::C4(std::rc::Rc::new({
                let m1197 = f366(v4.clone());
                match m1197 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f125(f91(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1198 {
                None => {
                    f367(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f268(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f273(v0: Adt2) -> R<Bytes> {
    let a1199 = f368(v0)?;
    let a1200 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1199, a1200))
}

pub fn f274(v0: Adt2, v1: Bytes) -> Bytes {
    let m1201 = f369(f271(v0, v1));
    match m1201 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f275(v0: List<Adt2>) -> List<Bytes> {
    let m1202 = v0;
    match m1202.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1203 = f369(Some(v1));
            match m1203 {
                None => {
                    f275(v2)
                }
                Some(v3) => {
                    List::cons(v3, f275(v2))
                }
            }
        }
    }
}

pub fn f276(v0: Option<Adt2>) -> List<Adt2> {
    let m1204 = v0;
    match m1204 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f370(v1)
        }
    }
}

pub fn f277() -> Bytes {
    f371()
}

pub fn f278(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f283(f59(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f372(v1, v4)?)?), List::cons(f270(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f279(v0: Bytes) -> R<Bytes> {
    Ok(f373(f374(v0)?))
}

pub fn f280(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1205 = v0;
    match m1205.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f281() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f282(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f340(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f283(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f375(f152(f90(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f376(f377(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f284(p1208: List<Adt14>) -> bool {
    let m1207 = p1208;
    {
        let mut m1207 = m1207;
        loop {
            let step = {
                let v0 = m1207;
                let m1206 = v0;
                match m1206.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f53(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f47(f8(Str::lit("studio")), f143(f378(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1207 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m1209 = v0;
    match m1209.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f194(v1)?;
            let a1211 = {
                let a1210 = v3.clone();
                length_bytes(a1210)
            };
            let a1212 = 0u64;
            if nat_eq(a1211, a1212) {
                f285(v2)
            } else {
                let m1213 = f316(v3)?;
                match m1213 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f215(v4.clone(), f8(Str::lit("agent"))) {
                            if f215(v4.clone(), f8(Str::lit("at"))) {
                                if f215(v4.clone(), f8(Str::lit("quote"))) {
                                    f379(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1214 = f285(v2)?;
                            match m1214 {
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

pub fn f286(v0: u64) -> bool {
    let a1215 = v0;
    let a1216 = 127u64;
    nat_le(a1215, a1216)
}

pub fn f287(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1217, _) = v1.clone();
        h1217
    };
    let v3: u64 = {
        let (h1218, _) = {
            let (_, h1219) = v1.clone();
            h1219
        };
        h1218
    };
    let v4: List<Bytes> = {
        let (_, h1220) = {
            let (_, h1221) = v1;
            h1221
        };
        h1220
    };
    let m1222 = f204(v0.clone(), v2);
    match m1222 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1223 = v5;
            let a1224 = 10u8;
            if equal(a1223, a1224) {
                Ok(Some(({
                    let a1225 = v2;
                    let a1226 = 1u64;
                    nat_add(a1225, a1226)?
                }, ({
                    let a1227 = v2;
                    let a1228 = 1u64;
                    nat_add(a1227, a1228)?
                }, List::cons(f380(f41(v0, v3, {
                    let a1229 = v2;
                    let a1230 = v3;
                    nat_sub(a1229, a1230)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1231 = v2;
                    let a1232 = 1u64;
                    nat_add(a1231, a1232)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f288(v0: Adt14) -> List<Adt14> {
    let m1233 = v0;
    match m1233 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1234) => {
            let v3 = (*r1234).clone();
            f288(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1235, r1236) => {
            let v1 = (*r1235).clone();
            let v2 = (*r1236).clone();
            List::cons(v1, f288(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f289(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1237 = v0;
    match m1237 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r1238) => {
            let v10 = (*r1238).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r1239) => {
            let v5 = (*r1239).clone();
            Adt14::C4(std::rc::Rc::new(f289(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1240, r1241) => {
            let v11 = (*r1240).clone();
            let v12 = (*r1241).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r1242, r1243) => {
            let v3 = (*r1242).clone();
            let v4 = (*r1243).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt14::C7(v2, std::rc::Rc::new(f381(v3, v1.clone())), std::rc::Rc::new(f289(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f289(v4, v1)))
            }
        }
    }
}

pub fn f290(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1244 = 0u64;
        let a1245 = v1;
        nat_lt(a1244, a1245)
    } else {
        false
    } {
        let a1246 = Bytes::lit(&[0x2d]);
        let a1247 = f382(v1)?;
        Ok(append_bytes(a1246, a1247))
    } else {
        f382(v1)
    }
}

pub fn f291(v0: Bytes) -> R<Bytes> {
    let a1253 = {
        let a1251 = Bytes::lit(&[0x22]);
        let a1252 = {
            let (_, h1248) = {
                let (h1249, _) = f383(Fn6::F384(v0.clone()), {
                    let a1250 = v0;
                    length_bytes(a1250)
                }, (0u64, Bytes::lit(&[])))?;
                h1249
            };
            h1248
        };
        append_bytes(a1251, a1252)
    };
    let a1254 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1253, a1254))
}

pub fn f292(v0: List<Bytes>) -> Bytes {
    let a1257 = {
        let a1255 = Bytes::lit(&[0x5b]);
        let a1256 = f117(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1255, a1256)
    };
    let a1258 = Bytes::lit(&[0x5d]);
    append_bytes(a1257, a1258)
}

pub fn f293(v0: Adt14) -> R<List<Bytes>> {
    let m1259 = v0;
    match m1259 {
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
        Adt14::C6(r1260, r1261) => {
            let v7 = (*r1260).clone();
            let v8 = (*r1261).clone();
            Ok(List::cons(f186(v7)?, f293(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f294(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f385(Fn7::F386, List::<(Bytes, Bytes)>::nil(), f387(v0)?);
    let v2: List<Bytes> = f388(Fn8::F389, List::<Bytes>::nil(), v1)?;
    let a1264 = {
        let a1262 = Bytes::lit(&[0x7b]);
        let a1263 = f117(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1262, a1263)
    };
    let a1265 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1264, a1265))
}

pub fn f295(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1266 = v0;
    match m1266 {
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
        Adt14::C7(v9, r1267, r1268) => {
            let v10 = (*r1267).clone();
            let v11 = (*r1268).clone();
            Ok(List::cons((v9, f186(v10)?), f295(v11)?))
        }
    }
}

pub fn f296() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f297(v0: Adt13) -> Bytes {
    let m1269 = v0;
    match m1269 {
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

pub fn f298(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f390(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f390(f8(Str::lit("id")), f64(v0.clone(), f56(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f390(f8(Str::lit("name")), f391(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f390(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f390(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f390(f8(Str::lit("statement")), f392(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f390(f8(Str::lit("media_type")), f393(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f390(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f390(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f390(f8(Str::lit("url")), f189(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f299(v0: Bytes) -> R<u64> {
    let (h1270, _) = f303(Fn4::F394(v0.clone()), {
        let a1271 = v0.clone();
        length_bytes(a1271)
    }, {
        let a1272 = v0;
        length_bytes(a1272)
    })?;
    Ok(h1270)
}

pub fn f300(v0: Bytes, v1: u64) -> R<u64> {
    let (h1273, _) = f303(Fn4::F395(v1, v0), v1, 0u64)?;
    Ok(h1273)
}

pub fn f301(v0: Fn3, p1278: u64, p1279: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1277 = (p1278, p1279);
    {
        let mut m1277 = m1277;
        loop {
            let step = {
                let (v1, v2) = m1277;
                let m1274 = v1;
                if m1274 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1274 - 1;
                    let m1276 = {
                        let c1275: Fn3 = v0.clone();
                        c1275.apply(v2.clone())?
                    };
                    match m1276 {
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
                Ok(next) => { m1277 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f396(v0, v1)
}

pub fn f303(v0: Fn4, p1284: u64, p1285: u64) -> R<(u64, bool)> {
    let m1283 = (p1284, p1285);
    {
        let mut m1283 = m1283;
        loop {
            let step = {
                let (v1, v2) = m1283;
                let m1280 = v1;
                if m1280 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1280 - 1;
                    let m1282 = {
                        let c1281: Fn4 = v0.clone();
                        c1281.apply(v2)?
                    };
                    match m1282 {
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
                Ok(next) => { m1283 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f397(v0, v1)
}

pub fn f305(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f398(v0, v1)
}

pub fn f306(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1286 = v0.clone();
        length_bytes(a1286)
    };
    let a1287 = 0u64;
    let a1288 = v1;
    if nat_lt(a1287, a1288) {
        let a1289 = f399(v0.clone())?;
        let a1290 = v1;
        if nat_eq(a1289, a1290) {
            let a1291 = 1u64;
            let a1292 = v1;
            if nat_lt(a1291, a1292) {
                let m1295 = {
                    let a1293 = v0;
                    let a1294 = 0u64;
                    index_bytes(a1293, a1294)
                };
                match m1295 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1296 = v2;
                        let a1297 = 48u8;
                        Ok(!equal(a1296, a1297))
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

pub fn f307(v0: Fn5, p1302: u64, p1303: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1301 = (p1302, p1303);
    {
        let mut m1301 = m1301;
        loop {
            let step = {
                let (v1, v2) = m1301;
                let m1298 = v1;
                if m1298 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1298 - 1;
                    let m1300 = {
                        let c1299: Fn5 = v0.clone();
                        c1299.apply(v2)?
                    };
                    match m1300 {
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
                Ok(next) => { m1301 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f308(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f400(v0, v1)
}

pub fn f309(v0: Bytes) -> R<u64> {
    let (h1304, _) = f303(Fn4::F401(v0.clone()), {
        let a1306 = {
            let a1305 = v0;
            length_bytes(a1305)
        };
        let a1307 = 1u64;
        nat_add(a1306, a1307)?
    }, 0u64)?;
    Ok(h1304)
}

pub fn f310(v0: u64) -> Bytes {
    let m1308 = v0;
    if m1308 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1308 - 1;
        let a1309 = f8(Str::lit("0"));
        let a1310 = f310(v1);
        append_bytes(a1309, a1310)
    }
}

pub fn f311(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1311 = v0;
    match m1311.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1312 = f88(v1, f8(Str::lit("std")));
            match m1312 {
                None => {
                    f311(v2)
                }
                Some(v3) => {
                    if f215(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f53(v3.clone(), f8(Str::lit("row"))), f53(v3, f8(Str::lit("authority")))), f311(v2))
                    } else {
                        f311(v2)
                    }
                }
            }
        }
    }
}

pub fn f312(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1313 = v1;
    match m1313.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f53(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f53(v2, f8(Str::lit("object")));
            if if f47(v4.clone(), match v0.clone() {
                Adt15::C0(h1314, _, _) => {
                    h1314
                }
            }) {
                f47(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1315, _) => {
                        h1315
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f312(v0, v3))
            } else {
                if if f47(v5.clone(), match v0.clone() {
                    Adt15::C0(h1316, _, _) => {
                        h1316
                    }
                }) {
                    f47(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1317, _) => {
                            h1317
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f312(v0, v3))
                } else {
                    f312(v0, v3)
                }
            }
        }
    }
}

pub fn f313(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt19>> {
    let m1318 = v0;
    match m1318.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v4, v5)) => {
            if if f402(v4.clone(), v2.clone()) {
                true
            } else {
                f402(v4.clone(), v3.clone())
            } {
                f313(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt19::C0({
                    let (h1319, _) = v4.clone();
                    h1319
                }, {
                    let a1322 = {
                        let a1321 = f147(f403({
                            let (h1320, _) = v4.clone();
                            h1320
                        }, v1.clone()))?;
                        length_list(a1321)
                    };
                    let a1323 = 1u64;
                    if nat_eq(a1322, a1323) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1324) = v4.clone();
                    h1324
                }), f313(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f314(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1325 = v1;
    match m1325.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f47({
                let (h1326, _) = v2.clone();
                h1326
            }, v0.clone()) {
                List::cons({
                    let (_, h1327) = v2;
                    h1327
                }, f314(v0, v3))
            } else {
                if f47({
                    let (_, h1328) = v2;
                    h1328
                }, v0.clone()) {
                    List::cons({
                        let (h1329, _) = v2;
                        h1329
                    }, f314(v0, v3))
                } else {
                    f314(v0, v3)
                }
            }
        }
    }
}

pub fn f315(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f47(v2.clone(), match v0.clone() {
        Adt15::C0(h1330, _, _) => {
            h1330
        }
    }) {
        true
    } else {
        if f47(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1331, _) => {
                h1331
            }
        }) {
            true
        } else {
            if f47(v2, match v0.clone() {
                Adt15::C0(_, _, h1332) => {
                    h1332
                }
            }) {
                true
            } else {
                if f215(v1.clone(), f8(Str::lit("scheme"))) {
                    f47(f53(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1333, _) => {
                            h1333
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f316(v0: Bytes) -> R<Option<Adt14>> {
    let m1334 = f38(v0)?;
    match m1334 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v1) => {
            Ok(f86(v1))
        }
    }
}

pub fn f317(v0: Adt14) -> bool {
    if f215(v0.clone(), f8(Str::lit("domain"))) {
        if f215(v0.clone(), f8(Str::lit("row"))) {
            if f47(f53(v0.clone(), f8(Str::lit("relation"))), f404()) {
                if f405(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f405(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f405(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f405(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f318(v0: Adt14) -> bool {
    if f215(v0.clone(), f8(Str::lit("subject"))) {
        if f215(v0.clone(), f8(Str::lit("object"))) {
            if f215(v0.clone(), f8(Str::lit("evidence"))) {
                f47(f53(v0, f8(Str::lit("relation"))), f406())
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

pub fn f319(v0: Bytes) -> bool {
    f37(v0, f8(Str::lit(".lex.tex")))
}

pub fn f320(v0: Bytes) -> R<bool> {
    let v1: Bytes = f194(v0)?;
    let v2: u64 = if f80(v1.clone(), f8(Str::lit("if ! "))) {
        5u64
    } else {
        if f80(v1.clone(), f8(Str::lit("elif ! "))) {
            7u64
        } else {
            0u64
        }
    };
    let v3: Bytes = f41(v1.clone(), v2, {
        let a1336 = {
            let a1335 = v1;
            length_bytes(a1335)
        };
        let a1337 = v2;
        nat_sub(a1336, a1337)
    });
    let v5: Bytes = if f80(v3.clone(), f8(Str::lit("("))) {
        let m1338 = f223(v3.clone(), f8(Str::lit(")")))?;
        match m1338 {
            None => {
                v3
            }
            Some(v4) => {
                f41(v3.clone(), {
                    let a1339 = v4;
                    let a1340 = 1u64;
                    nat_add(a1339, a1340)?
                }, {
                    let a1344 = {
                        let a1341 = v3;
                        length_bytes(a1341)
                    };
                    let a1345 = {
                        let a1342 = v4;
                        let a1343 = 1u64;
                        nat_add(a1342, a1343)?
                    };
                    nat_sub(a1344, a1345)
                })
            }
        }
    } else {
        v3
    };
    let a1346 = 0u64;
    let a1347 = v2;
    if nat_lt(a1346, a1347) {
        f407(v5)
    } else {
        Ok(false)
    }
}

pub fn f321(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f322(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1350 = {
            let a1348 = v4;
            let a1349 = v1;
            nat_add(a1348, a1349)?
        };
        let a1351 = v3;
        if nat_lt(a1350, a1351) {
            Ok(Some({
                let a1352 = v4;
                let a1353 = 1u64;
                nat_add(a1352, a1353)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f322(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1358 = {
        let a1355 = v0;
        let a1356 = v2;
        let a1357 = {
            let a1354 = v1.clone();
            length_bytes(a1354)
        };
        slice_bytes(a1355, a1356, a1357)
    };
    match m1358 {
        None => {
            false
        }
        Some(v3) => {
            let a1359 = v3;
            let a1360 = v1;
            equal(a1359, a1360)
        }
    }
}

pub fn f323(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1361 = v0;
    match m1361.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f323(v3, {
                let a1362 = v1;
                let a1363 = 1u64;
                nat_add(a1362, a1363)?
            })?))
        }
    }
}

pub fn f324(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1364 = v2;
    match m1364 {
        Adt21::C0(v3, v4) => {
            if f330(v3, match v0.clone() {
                Adt18::C0(h1365, _, _, _) => {
                    h1365
                }
            }) {
                f408(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f330(v5, match v0 {
                Adt18::C0(_, _, h1366, _) => {
                    h1366
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f326(v7, v1) {
                f330(v8, match v0 {
                    Adt18::C0(_, _, _, h1367) => {
                        h1367
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f325(v0: Adt16, v1: Adt14) -> R<Adt21> {
    let m1368 = f88(v1.clone(), f8(Str::lit("std")));
    match m1368 {
        None => {
            let m1373 = f87(f88(v1.clone(), f8(Str::lit("req"))));
            match m1373 {
                None => {
                    let m1375 = f88(v1, f8(Str::lit("der")));
                    match m1375 {
                        None => {
                            Ok(f409(v0))
                        }
                        Some(v5) => {
                            let m1376 = f410(f53(v5.clone(), f8(Str::lit("rule"))));
                            match m1376 {
                                None => {
                                    Ok(f409(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f231(f144(f184(f88(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt16::C0(_, _, _, _, h1377) => {
                                            h1377
                                        }
                                    })?, f328(f53(v5, f8(Str::lit("check"))), match v0 {
                                        Adt16::C0(_, _, _, h1378, _) => {
                                            h1378
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f328(v4, match v0 {
                        Adt16::C0(_, _, h1374, _, _) => {
                            h1374
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1369 = f87(f88(v2.clone(), f8(Str::lit("row"))));
            match m1369 {
                None => {
                    Ok(Adt21::C0(f328(f53(v2, f8(Str::lit("authority"))), match v0 {
                        Adt16::C0(h1370, _, _, _, _) => {
                            h1370
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f328(f53(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt16::C0(h1371, _, _, _, _) => {
                            h1371
                        }
                    })?, Some(f328(v3, match v0 {
                        Adt16::C0(_, h1372, _, _, _) => {
                            h1372
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f326(p1381: List<u64>, v1: List<u64>) -> bool {
    let m1380 = p1381;
    {
        let mut m1380 = m1380;
        loop {
            let step = {
                let v0 = m1380;
                let m1379 = v0;
                match m1379.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f330(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(p1386: List<u64>) -> bool {
    let m1385 = p1386;
    {
        let mut m1385 = m1385;
        loop {
            let step = {
                let v0 = m1385;
                let m1382 = v0;
                match m1382.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1383 = v1;
                        let a1384 = 7u64;
                        if nat_le(a1383, a1384) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1387 = f411(v0, v1.clone(), 0u64)?;
    match m1387 {
        None => {
            let a1388 = v1;
            Ok(length_list(a1388))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f329(v0: Adt14) -> u64 {
    let m1389 = f412(f88(v0, f8(Str::lit("stratum"))));
    match m1389 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f330(v0: u64, p1394: List<u64>) -> bool {
    let m1393 = p1394;
    {
        let mut m1393 = m1393;
        loop {
            let step = {
                let v1 = m1393;
                let m1390 = v1;
                match m1390.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1391 = v2;
                        let a1392 = v0;
                        if nat_eq(a1391, a1392) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Bytes, p1452: u64, p1453: u64, p1454: Adt23) -> R<Adt23> {
    let m1451 = (p1452, (p1453, p1454));
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1451;
                let m1395 = v2;
                if m1395 == 0 {
                    Err::<(u64, (u64, Adt23)), R<Adt23>>(Ok(v3))
                } else {
                    let v4 = m1395 - 1;
                    let v5: u64 = f413(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt23::C0(_, h1396, _, _, _) => {
                            h1396
                        }
                    } {
                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                            let a1397 = v1;
                            let a1398 = 1u64;
                            nat_add(a1397, a1398)?
                        }, (v4, Adt23::C0(true, false, match v3.clone() {
                            Adt23::C0(_, _, h1399, _, _) => {
                                h1399
                            }
                        }, match v3.clone() {
                            Adt23::C0(_, _, _, h1400, _) => {
                                h1400
                            }
                        }, match v3 {
                            Adt23::C0(_, _, _, _, h1401) => {
                                h1401
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt23::C0(h1402, _, _, _, _) => {
                                h1402
                            }
                        } {
                            let a1403 = v5;
                            let a1404 = 34u64;
                            if nat_eq(a1403, a1404) {
                                let a1407 = f413(v0.clone(), {
                                    let a1405 = v1;
                                    let a1406 = 1u64;
                                    nat_add(a1405, a1406)?
                                })?;
                                let a1408 = 34u64;
                                if nat_eq(a1407, a1408) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1409 = v1;
                                        let a1410 = 1u64;
                                        nat_add(a1409, a1410)?
                                    }, (v4, Adt23::C0(true, true, f414(match v3.clone() {
                                        Adt23::C0(_, _, h1411, _, _) => {
                                            h1411
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt23::C0(_, _, _, h1412, _) => {
                                            h1412
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1413) => {
                                            h1413
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1414 = v1;
                                        let a1415 = 1u64;
                                        nat_add(a1414, a1415)?
                                    }, (v4, Adt23::C0(false, false, match v3.clone() {
                                        Adt23::C0(_, _, h1416, _, _) => {
                                            h1416
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1417, _) => {
                                            h1417
                                        }
                                    }, match v3 {
                                        Adt23::C0(_, _, _, _, h1418) => {
                                            h1418
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1419 = v1;
                                    let a1420 = 1u64;
                                    nat_add(a1419, a1420)?
                                }, (v4, Adt23::C0(true, false, f414(match v3.clone() {
                                    Adt23::C0(_, _, h1421, _, _) => {
                                        h1421
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt23::C0(_, _, _, h1422, _) => {
                                        h1422
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1423) => {
                                        h1423
                                    }
                                }))))
                            }
                        } else {
                            let a1424 = v5;
                            let a1425 = 34u64;
                            if nat_eq(a1424, a1425) {
                                Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                    let a1426 = v1;
                                    let a1427 = 1u64;
                                    nat_add(a1426, a1427)?
                                }, (v4, Adt23::C0(true, false, match v3.clone() {
                                    Adt23::C0(_, _, h1428, _, _) => {
                                        h1428
                                    }
                                }, match v3.clone() {
                                    Adt23::C0(_, _, _, h1429, _) => {
                                        h1429
                                    }
                                }, match v3 {
                                    Adt23::C0(_, _, _, _, h1430) => {
                                        h1430
                                    }
                                }))))
                            } else {
                                let a1431 = v5;
                                let a1432 = 44u64;
                                if nat_eq(a1431, a1432) {
                                    Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                        let a1433 = v1;
                                        let a1434 = 1u64;
                                        nat_add(a1433, a1434)?
                                    }, (v4, Adt23::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt23::C0(_, _, h1435, _, _) => {
                                            h1435
                                        }
                                    }, match v3.clone() {
                                        Adt23::C0(_, _, _, h1436, _) => {
                                            h1436
                                        }
                                    }), match v3 {
                                        Adt23::C0(_, _, _, _, h1437) => {
                                            h1437
                                        }
                                    }))))
                                } else {
                                    let a1438 = v5;
                                    let a1439 = 13u64;
                                    if nat_eq(a1438, a1439) {
                                        Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                            let a1440 = v1;
                                            let a1441 = 1u64;
                                            nat_add(a1440, a1441)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1442 = v5;
                                        let a1443 = 10u64;
                                        if nat_eq(a1442, a1443) {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1444 = v1;
                                                let a1445 = 1u64;
                                                nat_add(a1444, a1445)?
                                            }, (v4, f332(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt23)), R<Adt23>>(({
                                                let a1446 = v1;
                                                let a1447 = 1u64;
                                                nat_add(a1446, a1447)?
                                            }, (v4, Adt23::C0(false, false, f414(match v3.clone() {
                                                Adt23::C0(_, _, h1448, _, _) => {
                                                    h1448
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt23::C0(_, _, _, h1449, _) => {
                                                    h1449
                                                }
                                            }, match v3 {
                                                Adt23::C0(_, _, _, _, h1450) => {
                                                    h1450
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
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: Adt23) -> Adt23 {
    Adt23::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f415(List::cons(match v0.clone() {
        Adt23::C0(_, _, h1455, _, _) => {
            h1455
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h1456, _) => {
            h1456
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt23::C0(_, _, _, _, h1457) => {
            h1457
        }
    }))
}

pub fn f333(p1460: List<List<Bytes>>, p1461: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1459 = (p1460, p1461);
    {
        let mut m1459 = m1459;
        loop {
            let step = {
                let (v0, v1) = m1459;
                let m1458 = v0;
                match m1458.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f334(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f335(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1462 = v0.clone();
    match m1462.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1463 = v2;
            let a1464 = v1.clone();
            if equal(a1463, a1464) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f336(v0: List<(Bytes, ())>) -> List<Bytes> {
    f237(f416(Fn9::F417, List::<Bytes>::nil(), v0))
}

pub fn f337(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f418(f419(Fn10::F420, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1469 = {
        let (h1465, _) = f421(Fn11::F422, {
            let a1467 = {
                let a1466 = v0;
                length_list(a1466)
            };
            let a1468 = 1u64;
            nat_add(a1467, a1468)?
        }, v1)?;
        h1465
    };
    match m1469.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f338(v0: List<Bytes>) -> List<(Bytes, ())> {
    f423(f424(Fn12::F425, List::<(Bytes, ())>::nil(), v0))
}

pub fn f339(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1470 = f39(v1, 34u8)?;
    match m1470.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1471 = v3;
            match m1471.uncons() {
                None => {
                    Ok(f426(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f340(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f427(v0.clone(), 0u64, {
        let a1472 = v0;
        length_bytes(a1472)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f341(v0: Adt14, v1: Bytes) -> Bytes {
    if f32(f428(f88(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f342(v0: Bytes, v1: Bytes, p1485: u64, p1486: u64) -> R<bool> {
    let m1484 = (p1485, p1486);
    {
        let mut m1484 = m1484;
        loop {
            let step = {
                let (v2, v3) = m1484;
                let m1473 = v3;
                if m1473 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1473 - 1;
                    let m1476 = {
                        let a1474 = v1.clone();
                        let a1475 = v2;
                        index_bytes(a1474, a1475)
                    };
                    match m1476 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1479 = {
                                let a1477 = v0.clone();
                                let a1478 = v2;
                                index_bytes(a1477, a1478)
                            };
                            match m1479 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1480 = v5;
                                    let a1481 = v6;
                                    if equal(a1480, a1481) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1482 = v2;
                                            let a1483 = 1u64;
                                            nat_add(a1482, a1483)?
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
                Ok(next) => { m1484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1487 = f85(v0, v1)?;
    match m1487 {
        None => {
            Ok(Adt2::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f344(v0: Bytes) -> R<Bytes> {
    let a1488 = f8(Str::lit("Catalog."));
    let a1489 = f363(v0)?;
    Ok(append_bytes(a1488, a1489))
}

pub fn f345(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f363(f274(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f346(v0: List<Bytes>) -> List<Bytes> {
    let m1490 = v0;
    match m1490.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f37(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f240(f258(), v1), f346(v2))
            } else {
                f346(v2)
            }
        }
    }
}

pub fn f347(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1491 = v1;
    match m1491.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f429(v0.clone(), v2)?;
            let (h1492, _) = v4.clone();
            if h1492 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f347(v0, v3)?;
                let (h1493, _) = v5.clone();
                if h1493 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1496 = {
                            let (_, h1494) = v4;
                            h1494
                        };
                        let a1497 = {
                            let (_, h1495) = v5;
                            h1495
                        };
                        append_bytes(a1496, a1497)
                    }))
                }
            }
        }
    }
}

pub fn f348(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1498 = v1;
    match m1498.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f215(v2.clone(), f8(Str::lit("subject"))) {
                if f216(v0.clone(), v2.clone()) {
                    f348(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f53(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f53(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f53(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f348(v0, v3))
                }
            } else {
                if f354(v0.clone(), v2.clone()) {
                    f348(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f53(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f355(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f348(v0, v3))
                }
            }
        }
    }
}

pub fn f349(v0: Adt1) -> R<Bytes> {
    let a1500 = {
        let a1499 = f123(f124(f236(f250(f235(f59(v0.clone(), f209()))?)), f125(f91(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f126(f91(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1499)
    };
    let a1501 = 0u64;
    if nat_eq(a1500, a1501) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f350(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1502 = v0;
    match m1502.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f117(f144(f184(f88(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f53(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f53(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f430(f90(f53(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f350(v2)?))
        }
    }
}

pub fn f351(v0: Bytes, v1: List<Adt19>) -> List<Bytes> {
    let m1503 = v1;
    match m1503.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt19::C0(h1504, _, _) => {
                    h1504
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt19::C0(_, h1505, _) => {
                        h1505
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt19::C0(_, _, h1506) => {
                        h1506
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f351(v0, v3))
            } else {
                f351(v0, v3)
            }
        }
    }
}

pub fn f352(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1507 = v1;
    match m1507.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f53(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f53(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f53(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f352(v0, v3))
            } else {
                if f32(f53(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f53(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f53(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f352(v0, v3))
                } else {
                    f352(v0, v3)
                }
            }
        }
    }
}

pub fn f353(p1512: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1511 = p1512;
    {
        let mut m1511 = m1511;
        loop {
            let step = {
                let v0 = m1511;
                let m1508 = v0;
                match m1508.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1509 = v2;
                        match m1509.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1510 = v5;
                                    match m1510.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f431(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: Adt15, v1: Adt14) -> bool {
    if f47(f53(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1513, _, _) => {
            h1513
        }
    }) {
        f47(f355(v1), match v0 {
            Adt15::C0(_, h1514, _) => {
                h1514
            }
        })
    } else {
        false
    }
}

pub fn f355(v0: Adt14) -> Bytes {
    let m1515 = f88(v0, f8(Str::lit("evidence")));
    match m1515 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f53(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f356(v0: Adt14) -> Bytes {
    if f215(v0.clone(), f8(Str::lit("req"))) {
        let a1516 = f8(Str::lit("req "));
        let a1517 = f53(v0, f8(Str::lit("req")));
        append_bytes(a1516, a1517)
    } else {
        let m1518 = f88(v0.clone(), f8(Str::lit("std")));
        match m1518 {
            None => {
                let m1521 = f88(v0, f8(Str::lit("der")));
                match m1521 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f53(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f117(f144(f184(f88(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f53(v1.clone(), f8(Str::lit("authority"))), List::cons(if f215(v1.clone(), f8(Str::lit("row"))) {
                    let a1519 = f8(Str::lit(" row "));
                    let a1520 = f53(v1, f8(Str::lit("row")));
                    append_bytes(a1519, a1520)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f357(v0: Option<List<Adt14>>) -> bool {
    let m1522 = v0;
    match m1522 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f358(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1523 = v0;
    match m1523 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f359(v0: Bytes, v1: Bytes) -> bool {
    let a1524 = v0;
    let a1525 = f371();
    if equal(a1524, a1525) {
        let a1529 = {
            let a1527 = {
                let a1526 = v1;
                length_bytes(a1526)
            };
            let a1528 = 0u64;
            nat_eq(a1527, a1528)
        };
        bool_not(a1529)
    } else {
        false
    }
}

pub fn f360(v0: Bytes) -> Bytes {
    f7(List::cons(f146(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f361(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1531 = v0;
    let a1532 = {
        let a1530 = Str::lit("redistributable");
        utf8_encode(a1530)
    };
    if equal(a1531, a1532) {
        f432(v1)
    } else {
        Ok(false)
    }
}

pub fn f362(v0: Adt2) -> bool {
    if f433(v0.clone(), f8(Str::lit("name"))) {
        if f434(v0.clone(), f8(Str::lit("schema"))) {
            let m1533 = f366(f275(f276(f271(v0, f8(Str::lit("citation"))))));
            match m1533 {
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

pub fn f363(v0: Bytes) -> R<Bytes> {
    f435(v0.clone(), 0u64, {
        let a1534 = v0;
        length_bytes(a1534)
    }, true, Bytes::lit(&[]))
}

pub fn f364(v0: Adt14) -> Bytes {
    if f215(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f215(v0.clone(), f8(Str::lit("id"))) {
            if f215(v0.clone(), f8(Str::lit("statement"))) {
                if f215(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f379(v0, f8(Str::lit("evidence")))
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

pub fn f365(v0: List<Bytes>) -> Adt14 {
    let m1535 = v0;
    match m1535.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f365(v2)))
        }
    }
}

pub fn f366(v0: List<Bytes>) -> Option<Adt14> {
    let m1536 = v0;
    match m1536.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1537 = v2;
                match m1537.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1538 = v2;
                    match m1538.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1539 = v6;
                                match m1539.uncons() {
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
                        let m1540 = v2;
                        match m1540.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1541 = v10;
                                match m1541.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f365(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f367(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1542 = f51(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1542.uncons() {
        None => {
            let m1543 = f366(v4);
            match m1543 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1544 = f8(Str::lit("Catalog."));
                        let a1545 = f363(v3.clone())?;
                        append_bytes(a1544, a1545)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f363(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1546 = f436(v2, v9.clone(), f363(v3.clone())?)?;
                    match m1546 {
                        Ok(v12) => {
                            Ok(Adt12::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f363(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f95(Adt14::C3(std::rc::Rc::new(f96(f437(f91(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f95(Adt14::C3(std::rc::Rc::new(f96(f437(f91(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1550 = f438(f59(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f41(v10.clone(), 4u64, {
                                    let a1548 = {
                                        let a1547 = v10;
                                        length_bytes(a1547)
                                    };
                                    let a1549 = 4u64;
                                    nat_sub(a1548, a1549)
                                }))?;
                                let a1551 = f8(Str::lit("\u{a}"));
                                append_bytes(a1550, a1551)
                            }), List::cons(f270(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f368(v0: Adt2) -> R<Bytes> {
    let m1552 = v0;
    match m1552 {
        Adt2::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt2::C1(v2, v3) => {
            f290(v2, v3)
        }
        Adt2::C2(v4, v5, v6) => {
            f439(v4, v5, v6)
        }
        Adt2::C3(v7) => {
            f440(v7)
        }
        Adt2::C4(r1553) => {
            let v8 = (*r1553).clone();
            Ok(f292(f441(v8)?))
        }
        Adt2::C5(r1554) => {
            let v9 = (*r1554).clone();
            f294(f442(v9)?)
        }
        Adt2::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f369(v0: Option<Adt2>) -> Option<Bytes> {
    let m1555 = v0;
    match m1555 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1556 = v1;
            match m1556 {
                Adt2::C0(_) => {
                    None::<Bytes>
                }
                Adt2::C1(_, _) => {
                    None::<Bytes>
                }
                Adt2::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt2::C3(v2) => {
                    Some(v2)
                }
                Adt2::C4(_) => {
                    None::<Bytes>
                }
                Adt2::C5(_) => {
                    None::<Bytes>
                }
                Adt2::C6 => {
                    None::<Bytes>
                }
                Adt2::C7(_, _) => {
                    None::<Bytes>
                }
                Adt2::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f370(v0: Adt2) -> List<Adt2> {
    let m1557 = v0;
    match m1557 {
        Adt2::C0(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C1(_, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C2(_, _, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C3(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C4(r1558) => {
            let v3 = (*r1558).clone();
            f370(v3)
        }
        Adt2::C5(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C6 => {
            List::<Adt2>::nil()
        }
        Adt2::C7(r1559, r1560) => {
            let v1 = (*r1559).clone();
            let v2 = (*r1560).clone();
            List::cons(v1, f370(v2))
        }
        Adt2::C8(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f371() -> Bytes {
    let a1561 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1561)
}

pub fn f372(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f88(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f282(Str::lit("id"), f53(v0.clone(), f8(Str::lit("id"))))?, List::cons(f282(Str::lit("name"), f53(v0.clone(), f8(Str::lit("name"))))?, List::cons(f282(Str::lit("issuer"), f53(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f282(Str::lit("edition"), f53(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f282(Str::lit("canonical_id"), f53(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f282(Str::lit("source_role"), f53(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f282(Str::lit("retrieval_date"), f53(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f282(Str::lit("url"), f428(v2.clone(), f8(Str::lit("url"))))?, List::cons(f282(Str::lit("media_type"), f428(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f282(Str::lit("license"), f428(v2.clone(), f8(Str::lit("license"))))?, List::cons(f282(Str::lit("redistribution"), f428(v2, f8(Str::lit("redistribution"))))?, List::cons(f282(Str::lit("statement"), f53(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1562 = v1;
        match m1562 {
            None => {
                f7(List::cons(f282(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f282(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f282(Str::lit("artifact"), {
                    let (h1563, _) = v3;
                    h1563
                })?, List::cons(f282(Str::lit("checksum"), {
                    let (_, h1564) = v3;
                    h1564
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f443(f144(f184(f88(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f373(v0: Adt24) -> Bytes {
    let a1585 = f444(match v0.clone() {
        Adt24::C0(h1565, _, _, _, _, _, _, _) => {
            h1565
        }
    });
    let a1586 = {
        let a1583 = f444(match v0.clone() {
            Adt24::C0(_, h1566, _, _, _, _, _, _) => {
                h1566
            }
        });
        let a1584 = {
            let a1581 = f444(match v0.clone() {
                Adt24::C0(_, _, h1567, _, _, _, _, _) => {
                    h1567
                }
            });
            let a1582 = {
                let a1579 = f444(match v0.clone() {
                    Adt24::C0(_, _, _, h1568, _, _, _, _) => {
                        h1568
                    }
                });
                let a1580 = {
                    let a1577 = f444(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1569, _, _, _) => {
                            h1569
                        }
                    });
                    let a1578 = {
                        let a1575 = f444(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1570, _, _) => {
                                h1570
                            }
                        });
                        let a1576 = {
                            let a1573 = f444(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1571, _) => {
                                    h1571
                                }
                            });
                            let a1574 = f444(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1572) => {
                                    h1572
                                }
                            });
                            append_bytes(a1573, a1574)
                        };
                        append_bytes(a1575, a1576)
                    };
                    append_bytes(a1577, a1578)
                };
                append_bytes(a1579, a1580)
            };
            append_bytes(a1581, a1582)
        };
        append_bytes(a1583, a1584)
    };
    append_bytes(a1585, a1586)
}

pub fn f374(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1587 = v0.clone();
        length_bytes(a1587)
    };
    let v2: u64 = {
        let a1593 = {
            let a1590 = {
                let a1588 = v1;
                let a1589 = 72u64;
                nat_add(a1588, a1589)?
            };
            let a1591 = 64u64;
            let a1592 = 0u64;
            nat_quot(a1590, a1591, a1592)
        };
        let a1594 = 64u64;
        nat_mul(a1593, a1594)?
    };
    let (_, h1595) = f445(Fn13::F446(v0, v1, v2), {
        let a1596 = v2;
        let a1597 = 64u64;
        let a1598 = 0u64;
        nat_quot(a1596, a1597, a1598)
    }, (0u64, f447()))?;
    Ok(h1595)
}

pub fn f375(p1601: List<List<Bytes>>, v1: Bytes, p1602: List<Bytes>) -> R<List<Bytes>> {
    let m1600 = (p1601, p1602);
    {
        let mut m1600 = m1600;
        loop {
            let step = {
                let (v0, v2) = m1600;
                let m1599 = v0;
                match m1599.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f448(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f377(f449(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Bytes) -> R<Bytes> {
    Ok(f41(v0.clone(), 0u64, f299(v0)?))
}

pub fn f377(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1603 = v0;
    match m1603.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1604 = v3.clone();
            match m1604.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1607 = v2;
                    let a1608 = {
                        let a1605 = v1.clone();
                        let a1606 = f377(v3, v1);
                        append_bytes(a1605, a1606)
                    };
                    append_bytes(a1607, a1608)
                }
            }
        }
    }
}

pub fn f378(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1609 = f88(v0, v1);
    match m1609 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f184(f88(v3, v2))
        }
    }
}

pub fn f379(v0: Adt14, v1: Bytes) -> bool {
    let m1610 = f88(v0, v1);
    match m1610 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f380(v0: Bytes) -> Bytes {
    if f37(v0.clone(), Bytes::lit(&[0x0d])) {
        f41(v0.clone(), 0u64, {
            let a1612 = {
                let a1611 = v0;
                length_bytes(a1611)
            };
            let a1613 = 1u64;
            nat_sub(a1612, a1613)
        })
    } else {
        v0
    }
}

pub fn f381(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1614 = v0;
    match m1614 {
        Adt14::C0(v3) => {
            Adt14::C0(v3)
        }
        Adt14::C1(v4, v5) => {
            Adt14::C1(v4, v5)
        }
        Adt14::C2(v2) => {
            Adt14::C2(f450(f55(), v2, v1))
        }
        Adt14::C3(r1615) => {
            let v6 = (*r1615).clone();
            Adt14::C3(std::rc::Rc::new(v6))
        }
        Adt14::C4(r1616) => {
            let v7 = (*r1616).clone();
            Adt14::C4(std::rc::Rc::new(v7))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1617, r1618) => {
            let v8 = (*r1617).clone();
            let v9 = (*r1618).clone();
            Adt14::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt14::C7(v10, r1619, r1620) => {
            let v11 = (*r1619).clone();
            let v12 = (*r1620).clone();
            Adt14::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f382(v0: u64) -> R<Bytes> {
    f451(f452(), 10u64, v0)
}

pub fn f383(v0: Fn6, p1625: u64, p1626: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1624 = (p1625, p1626);
    {
        let mut m1624 = m1624;
        loop {
            let step = {
                let (v1, v2) = m1624;
                let m1621 = v1;
                if m1621 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1621 - 1;
                    let m1623 = {
                        let c1622: Fn6 = v0.clone();
                        c1622.apply(v2.clone())?
                    };
                    match m1623 {
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
                Ok(next) => { m1624 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f453(v0, v1)
}

pub fn f385(v0: Fn7, p1630: List<(Bytes, Bytes)>, p1631: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1629 = (p1630, p1631);
    {
        let mut m1629 = m1629;
        loop {
            let step = {
                let (v1, v2) = m1629;
                let m1627 = v2;
                match m1627.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1628: Fn7 = v0.clone();
                            c1628.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1629 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f454(v0, v1)
}

pub fn f387(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f455(v0)
}

pub fn f388(v0: Fn8, p1635: List<Bytes>, p1636: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1634 = (p1635, p1636);
    {
        let mut m1634 = m1634;
        loop {
            let step = {
                let (v1, v2) = m1634;
                let m1632 = v2;
                match m1632.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1633: Fn8 = v0.clone();
                            c1633.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1634 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f389(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f456(v1)?, v0))
}

pub fn f390(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f291(v0)?, List::cons(f8(Str::lit(":")), List::cons(f291(v1)?, List::<Bytes>::nil())))))
}

pub fn f391(v0: Adt13) -> Bytes {
    let m1637 = v0;
    match m1637 {
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

pub fn f392(v0: Adt13) -> Bytes {
    let m1638 = v0;
    match m1638 {
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

pub fn f393(v0: Adt13) -> Bytes {
    let m1639 = v0;
    match m1639 {
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

pub fn f394(v0: Bytes, v1: u64) -> Option<u64> {
    let a1640 = 0u64;
    let a1641 = v1;
    if nat_lt(a1640, a1641) {
        if f457(v0, {
            let a1642 = v1;
            let a1643 = 1u64;
            nat_sub(a1642, a1643)
        }) {
            Some({
                let a1644 = v1;
                let a1645 = 1u64;
                nat_sub(a1644, a1645)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f395(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1646 = v2;
    let a1647 = v0;
    if nat_lt(a1646, a1647) {
        if f457(v1, v2) {
            Ok(Some({
                let a1648 = v2;
                let a1649 = 1u64;
                nat_add(a1648, a1649)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f396(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1650, _) = v1.clone();
        h1650
    };
    let v3: Bytes = {
        let (h1651, _) = {
            let (_, h1652) = v1.clone();
            h1652
        };
        h1651
    };
    let a1655 = {
        let (_, h1653) = {
            let (_, h1654) = v1;
            h1654
        };
        h1653
    };
    let a1656 = 0u64;
    if nat_eq(a1655, a1656) {
        let v4: u64 = f110(v0.clone(), v2)?;
        let a1657 = v4;
        let a1658 = 34u64;
        if nat_eq(a1657, a1658) {
            Ok(Some(({
                let a1659 = v2;
                let a1660 = 1u64;
                nat_add(a1659, a1660)?
            }, (v3, 1u64))))
        } else {
            let a1661 = v4;
            let a1662 = 92u64;
            if nat_eq(a1661, a1662) {
                f458(v0, v2, v3)
            } else {
                let a1663 = v4;
                let a1664 = 32u64;
                if if nat_lt(a1663, a1664) {
                    true
                } else {
                    let a1665 = v4;
                    let a1666 = 256u64;
                    nat_eq(a1665, a1666)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1667 = v2;
                        let a1668 = 1u64;
                        nat_add(a1667, a1668)?
                    }, ({
                        let a1669 = v3;
                        let a1670 = f41(v0, v2, 1u64);
                        append_bytes(a1669, a1670)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f397(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f110(v0, v1)?;
    let a1671 = 47u64;
    let a1672 = v2;
    if if nat_lt(a1671, a1672) {
        let a1673 = v2;
        let a1674 = 58u64;
        nat_lt(a1673, a1674)
    } else {
        false
    } {
        Ok(Some({
            let a1675 = v1;
            let a1676 = 1u64;
            nat_add(a1675, a1676)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f398(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1677 = f110(v0, v1)?;
    let a1678 = 48u64;
    if nat_eq(a1677, a1678) {
        Ok(Some({
            let a1679 = v1;
            let a1680 = 1u64;
            nat_add(a1679, a1680)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f399(v0: Bytes) -> R<u64> {
    let (h1681, _) = f303(Fn4::F459(v0.clone()), {
        let a1682 = v0;
        length_bytes(a1682)
    }, 0u64)?;
    Ok(h1681)
}

pub fn f400(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1686 = {
        let a1684 = v0;
        let a1685 = {
            let (h1683, _) = v1;
            h1683
        };
        index_bytes(a1684, a1685)
    };
    match m1686 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1688 = {
                    let (h1687, _) = v1;
                    h1687
                };
                let a1689 = 1u64;
                nat_add(a1688, a1689)?
            }, {
                let a1693 = {
                    let a1691 = {
                        let (_, h1690) = v1;
                        h1690
                    };
                    let a1692 = 10u64;
                    nat_mul(a1691, a1692)?
                };
                let a1694 = f460(v2)?;
                nat_add(a1693, a1694)?
            })))
        }
    }
}

pub fn f401(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f461(v0, v1)
}

pub fn f402(v0: (Bytes, Bytes), p1697: List<(Bytes, Bytes)>) -> bool {
    let m1696 = p1697;
    {
        let mut m1696 = m1696;
        loop {
            let step = {
                let v1 = m1696;
                let m1695 = v1;
                match m1695.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f462(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1696 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1698 = v1;
    match m1698.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1699, _) = v2;
                h1699
            }, v0.clone()) {
                List::cons({
                    let (_, h1700) = v2;
                    h1700
                }, f403(v0, v3))
            } else {
                f403(v0, v3)
            }
        }
    }
}

pub fn f404() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f405(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1701 = f88(v0, v1);
    match m1701 {
        None => {
            false
        }
        Some(v3) => {
            f215(v3, v2)
        }
    }
}

pub fn f406() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f407(v0: Bytes) -> R<bool> {
    let m1702 = f223(v0.clone(), f8(Str::lit("&&")))?;
    if match m1702 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m1703 = f223(v0, f8(Str::lit("||")))?;
        match m1703 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f408(v0: Adt18, v1: Option<u64>) -> bool {
    let m1704 = v1;
    match m1704 {
        None => {
            true
        }
        Some(v2) => {
            f330(v2, match v0 {
                Adt18::C0(_, h1705, _, _) => {
                    h1705
                }
            })
        }
    }
}

pub fn f409(v0: Adt16) -> Adt21 {
    Adt21::C1({
        let a1707 = match v0 {
            Adt16::C0(_, _, h1706, _, _) => {
                h1706
            }
        };
        length_list(a1707)
    })
}

pub fn f410(v0: Bytes) -> Option<Adt22> {
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

pub fn f411(v0: Bytes, p1712: List<Bytes>, p1713: u64) -> R<Option<u64>> {
    let m1711 = (p1712, p1713);
    {
        let mut m1711 = m1711;
        loop {
            let step = {
                let (v1, v2) = m1711;
                let m1708 = v1;
                match m1708.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1709 = v2;
                                let a1710 = 1u64;
                                nat_add(a1709, a1710)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1711 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: Option<Adt14>) -> Option<u64> {
    let m1714 = v0;
    match m1714 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1715 = v1;
            match m1715 {
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

pub fn f413(v0: Bytes, v1: u64) -> R<u64> {
    let m1718 = {
        let a1716 = v0;
        let a1717 = v1;
        index_bytes(a1716, a1717)
    };
    match m1718 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f67(v2)
        }
    }
}

pub fn f414(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1722 = {
        let a1719 = v1;
        let a1720 = v2;
        let a1721 = 1u64;
        slice_bytes(a1719, a1720, a1721)
    };
    match m1722 {
        None => {
            v0
        }
        Some(v3) => {
            let a1723 = v0;
            let a1724 = v3;
            append_bytes(a1723, a1724)
        }
    }
}

pub fn f415(p1727: List<Bytes>, p1728: List<Bytes>) -> List<Bytes> {
    let m1726 = (p1727, p1728);
    {
        let mut m1726 = m1726;
        loop {
            let step = {
                let (v0, v1) = m1726;
                let m1725 = v0;
                match m1725.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: Fn9, p1732: List<Bytes>, p1733: List<(Bytes, ())>) -> List<Bytes> {
    let m1731 = (p1732, p1733);
    {
        let mut m1731 = m1731;
        loop {
            let step = {
                let (v1, v2) = m1731;
                let m1729 = v2;
                match m1729.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1730: Fn9 = v0.clone();
                            c1730.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1734, _) = v1;
        h1734
    }, v0)
}

pub fn f418(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f463(Fn14::F464, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f419(v0: Fn10, p1738: List<List<(Bytes, ())>>, p1739: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1737 = (p1738, p1739);
    {
        let mut m1737 = m1737;
        loop {
            let step = {
                let (v1, v2) = m1737;
                let m1735 = v2;
                match m1735.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1736: Fn10 = v0.clone();
                            c1736.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f420(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f421(v0: Fn11, p1744: u64, p1745: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1743 = (p1744, p1745);
    {
        let mut m1743 = m1743;
        loop {
            let step = {
                let (v1, v2) = m1743;
                let m1740 = v1;
                if m1740 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1740 - 1;
                    let m1742 = {
                        let c1741: Fn11 = v0.clone();
                        c1741.apply(v2.clone())?
                    };
                    match m1742 {
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
                Ok(next) => { m1743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f422(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f465(v0)
}

pub fn f423(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f466(Fn15::F467, List::<(Bytes, ())>::nil(), v0)
}

pub fn f424(v0: Fn12, p1749: List<(Bytes, ())>, p1750: List<Bytes>) -> List<(Bytes, ())> {
    let m1748 = (p1749, p1750);
    {
        let mut m1748 = m1748;
        loop {
            let step = {
                let (v1, v2) = m1748;
                let m1746 = v2;
                match m1746.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1747: Fn12 = v0.clone();
                            c1747.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f426(p1755: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1754 = p1755;
    {
        let mut m1754 = m1754;
        loop {
            let step = {
                let v0 = m1754;
                let m1751 = v0;
                match m1751.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f47(v1.clone(), {
                            let (h1752, _) = v2;
                            h1752
                        }) {
                            let (_, h1753) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1753)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1754 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(v0: Bytes, p1781: u64, p1782: u64, p1783: Bytes) -> R<Bytes> {
    let m1780 = (p1781, (p1782, p1783));
    {
        let mut m1780 = m1780;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1780;
                let m1756 = v2;
                if m1756 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1756 - 1;
                    let m1759 = {
                        let a1757 = v0.clone();
                        let a1758 = v1;
                        index_bytes(a1757, a1758)
                    };
                    match m1759 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f67(v5)?;
                            if f199({
                                let a1760 = v6;
                                let a1761 = 34u64;
                                nat_eq(a1760, a1761)
                            }, {
                                let a1762 = v6;
                                let a1763 = 92u64;
                                nat_eq(a1762, a1763)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1764 = v1;
                                    let a1765 = 1u64;
                                    nat_add(a1764, a1765)?
                                }, (v4, {
                                    let a1768 = v3;
                                    let a1769 = {
                                        let a1766 = f8(Str::lit("\\"));
                                        let a1767 = f41(v0.clone(), v1, 1u64);
                                        append_bytes(a1766, a1767)
                                    };
                                    append_bytes(a1768, a1769)
                                })))
                            } else {
                                let a1770 = v6;
                                let a1771 = 10u64;
                                if nat_eq(a1770, a1771) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1772 = v1;
                                        let a1773 = 1u64;
                                        nat_add(a1772, a1773)?
                                    }, (v4, {
                                        let a1774 = v3;
                                        let a1775 = f8(Str::lit("\\n"));
                                        append_bytes(a1774, a1775)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1776 = v1;
                                        let a1777 = 1u64;
                                        nat_add(a1776, a1777)?
                                    }, (v4, {
                                        let a1778 = v3;
                                        let a1779 = f41(v0.clone(), v1, 1u64);
                                        append_bytes(a1778, a1779)
                                    })))
                                }
                            }
                        }
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

pub fn f428(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1784 = v0;
    match m1784 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f53(v2, v1)
        }
    }
}

pub fn f429(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f90(f59(v0.clone(), v1.clone()))?;
    let m1785 = f285(v2.clone())?;
    match m1785 {
        None => {
            f469(v0, v1, v2)
        }
        Some(v3) => {
            f468(v0, v1, v3)
        }
    }
}

pub fn f430(v0: List<Bytes>) -> List<Bytes> {
    let m1786 = v0;
    match m1786.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1788 = {
                    let a1787 = v1.clone();
                    length_bytes(a1787)
                };
                let a1789 = 0u64;
                if nat_eq(a1788, a1789) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f430(v2))
        }
    }
}

pub fn f431(v0: List<Bytes>) -> Bytes {
    let m1790 = v0;
    match m1790.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f432(v0: Bytes) -> R<bool> {
    let m1792 = {
        let a1791 = v0.clone();
        length_bytes(a1791)
    };
    if m1792 == 0 {
        Ok(true)
    } else {
        let v1 = m1792 - 1;
        let m1795 = {
            let a1793 = v0;
            let a1794 = v1;
            index_bytes(a1793, a1794)
        };
        match m1795 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f470({
                    let v3: u8 = v2;
                    let a1840 = {
                        let a1816 = {
                            let a1804 = {
                                let a1798 = {
                                    let a1796 = v3;
                                    let a1797 = 128u8;
                                    fixed_u8::bit_and(a1796, a1797)
                                };
                                let a1799 = 128u8;
                                if equal(a1798, a1799) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1805 = {
                                let a1802 = {
                                    let a1800 = v3;
                                    let a1801 = 64u8;
                                    fixed_u8::bit_and(a1800, a1801)
                                };
                                let a1803 = 64u8;
                                if equal(a1802, a1803) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1804, a1805)?
                        };
                        let a1817 = {
                            let a1814 = {
                                let a1808 = {
                                    let a1806 = v3;
                                    let a1807 = 32u8;
                                    fixed_u8::bit_and(a1806, a1807)
                                };
                                let a1809 = 32u8;
                                if equal(a1808, a1809) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1815 = {
                                let a1812 = {
                                    let a1810 = v3;
                                    let a1811 = 16u8;
                                    fixed_u8::bit_and(a1810, a1811)
                                };
                                let a1813 = 16u8;
                                if equal(a1812, a1813) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1814, a1815)?
                        };
                        nat_add(a1816, a1817)?
                    };
                    let a1841 = {
                        let a1838 = {
                            let a1826 = {
                                let a1820 = {
                                    let a1818 = v3;
                                    let a1819 = 8u8;
                                    fixed_u8::bit_and(a1818, a1819)
                                };
                                let a1821 = 8u8;
                                if equal(a1820, a1821) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1827 = {
                                let a1824 = {
                                    let a1822 = v3;
                                    let a1823 = 4u8;
                                    fixed_u8::bit_and(a1822, a1823)
                                };
                                let a1825 = 4u8;
                                if equal(a1824, a1825) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1826, a1827)?
                        };
                        let a1839 = {
                            let a1836 = {
                                let a1830 = {
                                    let a1828 = v3;
                                    let a1829 = 2u8;
                                    fixed_u8::bit_and(a1828, a1829)
                                };
                                let a1831 = 2u8;
                                if equal(a1830, a1831) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1837 = {
                                let a1834 = {
                                    let a1832 = v3;
                                    let a1833 = 1u8;
                                    fixed_u8::bit_and(a1832, a1833)
                                };
                                let a1835 = 1u8;
                                if equal(a1834, a1835) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1836, a1837)?
                        };
                        nat_add(a1838, a1839)?
                    };
                    nat_add(a1840, a1841)?
                }))
            }
        }
    }
}

pub fn f433(v0: Adt2, v1: Bytes) -> bool {
    let a1843 = {
        let a1842 = f274(v0, v1);
        length_bytes(a1842)
    };
    let a1844 = 0u64;
    !nat_eq(a1843, a1844)
}

pub fn f434(v0: Adt2, v1: Bytes) -> bool {
    let m1845 = f271(v0, v1);
    match m1845 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f435(v0: Bytes, p1866: u64, p1867: u64, p1868: bool, p1869: Bytes) -> R<Bytes> {
    let m1865 = (p1866, (p1867, (p1868, p1869)));
    {
        let mut m1865 = m1865;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1865;
                let m1846 = v2;
                if m1846 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1846 - 1;
                    let m1849 = {
                        let a1847 = v0.clone();
                        let a1848 = v1;
                        index_bytes(a1847, a1848)
                    };
                    match m1849 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f67(v6)?;
                            if f471(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1850 = v1;
                                    let a1851 = 1u64;
                                    nat_add(a1850, a1851)?
                                }, (v5, (true, v4))))
                            } else {
                                if f112(f472(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1852 = v1;
                                        let a1853 = 1u64;
                                        nat_add(a1852, a1853)?
                                    }, (v5, (false, {
                                        let a1857 = v4;
                                        let a1858 = f41({
                                            let a1854 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1854)
                                        }, {
                                            let a1855 = v7;
                                            let a1856 = 97u64;
                                            nat_sub(a1855, a1856)
                                        }, 1u64);
                                        append_bytes(a1857, a1858)
                                    }))))
                                } else {
                                    if f199(f472(v7), f199(f473(v7), f474(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1859 = v1;
                                            let a1860 = 1u64;
                                            nat_add(a1859, a1860)?
                                        }, (v5, (false, {
                                            let a1861 = v4;
                                            let a1862 = f41(v0.clone(), v1, 1u64);
                                            append_bytes(a1861, a1862)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1863 = v1;
                                            let a1864 = 1u64;
                                            nat_add(a1863, a1864)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f475(f476(f271(v0.clone(), f8(Str::lit("$defs")))), f476(f271(v0.clone(), f8(Str::lit("definitions")))));
    let m1870 = f477(4096u64, v3.clone(), List::<Bytes>::nil(), f478(v3)?)?;
    match m1870 {
        Ok(v5) => {
            let m1871 = f479(4096u64, f480(f481(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1871 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f377(f243(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f437(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1872 = v0;
    match m1872.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f32(f53(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f482(v5, v1, v2))
            } else {
                List::cons(v4, f437(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f438(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f377(f483(f90(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f439(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f382(v1)?;
    let v4: u64 = {
        let a1873 = v3.clone();
        length_bytes(a1873)
    };
    let v5: Bytes = {
        let a1874 = v2;
        let a1875 = v4;
        if nat_lt(a1874, a1875) {
            f7(List::cons(f41(v3.clone(), 0u64, {
                let a1876 = v4;
                let a1877 = v2;
                nat_sub(a1876, a1877)
            }), List::cons(f8(Str::lit(".")), List::cons(f41(v3, {
                let a1878 = v4;
                let a1879 = v2;
                nat_sub(a1878, a1879)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f310({
                let a1880 = v2;
                let a1881 = v4;
                nat_sub(a1880, a1881)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1882 = f8(Str::lit("-"));
        let a1883 = v5;
        Ok(append_bytes(a1882, a1883))
    } else {
        Ok(v5)
    }
}

pub fn f440(v0: Bytes) -> R<Bytes> {
    let a1888 = f196(v0.clone(), 0u64, {
        let a1885 = {
            let a1884 = v0.clone();
            length_bytes(a1884)
        };
        let a1886 = 1u64;
        nat_add(a1885, a1886)?
    })?;
    let a1889 = {
        let a1887 = v0.clone();
        length_bytes(a1887)
    };
    if nat_eq(a1888, a1889) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f291(v0)
    }
}

pub fn f441(v0: Adt2) -> R<List<Bytes>> {
    let m1890 = v0;
    match m1890 {
        Adt2::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(r1891, r1892) => {
            let v10 = (*r1891).clone();
            let v11 = (*r1892).clone();
            Ok(List::cons(f368(v10)?, f441(v11)?))
        }
        Adt2::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f442(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1893 = v0;
    match m1893 {
        Adt2::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C8(v12, r1894, r1895) => {
            let v13 = (*r1894).clone();
            let v14 = (*r1895).clone();
            Ok(List::cons((v12, f368(v13)?), f442(v14)?))
        }
    }
}

pub fn f443(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f377(f484(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f444(v0: u32) -> Bytes {
    let a1924 = f485({
        let a1896 = f486(v0, 28u32);
        let a1897 = 15u32;
        fixed_u32::bit_and(a1896, a1897)
    });
    let a1925 = {
        let a1922 = f485({
            let a1898 = f486(v0, 24u32);
            let a1899 = 15u32;
            fixed_u32::bit_and(a1898, a1899)
        });
        let a1923 = {
            let a1920 = f485({
                let a1900 = f486(v0, 20u32);
                let a1901 = 15u32;
                fixed_u32::bit_and(a1900, a1901)
            });
            let a1921 = {
                let a1918 = f485({
                    let a1902 = f486(v0, 16u32);
                    let a1903 = 15u32;
                    fixed_u32::bit_and(a1902, a1903)
                });
                let a1919 = {
                    let a1916 = f485({
                        let a1904 = f486(v0, 12u32);
                        let a1905 = 15u32;
                        fixed_u32::bit_and(a1904, a1905)
                    });
                    let a1917 = {
                        let a1914 = f485({
                            let a1906 = f486(v0, 8u32);
                            let a1907 = 15u32;
                            fixed_u32::bit_and(a1906, a1907)
                        });
                        let a1915 = {
                            let a1912 = f485({
                                let a1908 = f486(v0, 4u32);
                                let a1909 = 15u32;
                                fixed_u32::bit_and(a1908, a1909)
                            });
                            let a1913 = f485({
                                let a1910 = v0;
                                let a1911 = 15u32;
                                fixed_u32::bit_and(a1910, a1911)
                            });
                            append_bytes(a1912, a1913)
                        };
                        append_bytes(a1914, a1915)
                    };
                    append_bytes(a1916, a1917)
                };
                append_bytes(a1918, a1919)
            };
            append_bytes(a1920, a1921)
        };
        append_bytes(a1922, a1923)
    };
    append_bytes(a1924, a1925)
}

pub fn f445(v0: Fn13, p1929: u64, p1930: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1928 = (p1929, p1930);
    {
        let mut m1928 = m1928;
        loop {
            let step = {
                let (v1, v2) = m1928;
                let m1926 = v1;
                if m1926 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1926 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1927: Fn13 = v0.clone();
                        c1927.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1928 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1932 = {
            let (h1931, _) = v3.clone();
            h1931
        };
        let a1933 = 64u64;
        nat_add(a1932, a1933)?
    }, f487({
        let (_, h1934) = v3;
        h1934
    }, f488(v0, v1, v2, {
        let (h1935, _) = v3;
        h1935
    })?)?))
}

pub fn f447() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f448(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1936 = f39(v1, 34u8)?;
    match m1936.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1937 = v3;
            match m1937.uncons() {
                None => {
                    Ok(f489(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f449(v0: List<Bytes>) -> List<Bytes> {
    let m1938 = v0;
    match m1938.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f449(v2)
            } else {
                List::cons(v1, f449(v2))
            }
        }
    }
}

pub fn f450(p1941: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m1940 = p1941;
    {
        let mut m1940 = m1940;
        loop {
            let step = {
                let v0 = m1940;
                let m1939 = v0;
                match m1939.uncons() {
                    None => {
                        Err::<List<Adt13>, Bytes>(v1.clone())
                    }
                    Some((v3, v4)) => {
                        if f80(v1.clone(), f97(v3.clone())) {
                            Err::<List<Adt13>, Bytes>(f64(v3, v2.clone()))
                        } else {
                            Ok::<List<Adt13>, Bytes>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f451(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1942 = v2;
    let a1943 = 0u64;
    if nat_eq(a1942, a1943) {
        Ok(f490(v0, 0u64))
    } else {
        let (_, h1944) = {
            let (h1945, _) = f383(Fn6::F491(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1945
        };
        Ok(h1944)
    }
}

pub fn f452() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f453(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1949 = {
        let a1947 = v0.clone();
        let a1948 = {
            let (h1946, _) = v1.clone();
            h1946
        };
        index_bytes(a1947, a1948)
    };
    match m1949 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1951 = {
                    let (h1950, _) = v1.clone();
                    h1950
                };
                let a1952 = 1u64;
                nat_add(a1951, a1952)?
            }, {
                let a1955 = {
                    let (_, h1953) = v1;
                    h1953
                };
                let a1956 = f492(v0, {
                    let (h1954, _) = v1;
                    h1954
                }, v2)?;
                append_bytes(a1955, a1956)
            })))
        }
    }
}

pub fn f454(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1957 = v0.clone();
    match m1957.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1960 = {
                let (h1958, _) = v2;
                h1958
            };
            let a1961 = {
                let (h1959, _) = v1.clone();
                h1959
            };
            if equal(a1960, a1961) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f455(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f493(f494(Fn16::F495, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1966 = {
        let (h1962, _) = f496(Fn17::F497, {
            let a1964 = {
                let a1963 = v0;
                length_list(a1963)
            };
            let a1965 = 1u64;
            nat_add(a1964, a1965)?
        }, v1)?;
        h1962
    };
    match m1966.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f456(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1971 = {
        let a1968 = f291({
            let (h1967, _) = v0;
            h1967
        })?;
        let a1969 = Bytes::lit(&[0x3a]);
        append_bytes(a1968, a1969)
    };
    let a1972 = {
        let (_, h1970) = v0;
        h1970
    };
    Ok(append_bytes(a1971, a1972))
}

pub fn f457(v0: Bytes, v1: u64) -> bool {
    let m1973 = f204(v0, v1);
    match m1973 {
        None => {
            false
        }
        Some(v2) => {
            f498(v2)
        }
    }
}

pub fn f458(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f110(v0.clone(), {
        let a1974 = v1;
        let a1975 = 1u64;
        nat_add(a1974, a1975)?
    })?;
    let v4: Bytes = {
        let a1976 = v3;
        let a1977 = 117u64;
        if nat_eq(a1976, a1977) {
            f499(v0, v1)?
        } else {
            f500(v3)
        }
    };
    let a1978 = v4.clone();
    let a1979 = Bytes::lit(&[]);
    if equal(a1978, a1979) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1980 = v3;
            let a1981 = 117u64;
            if nat_eq(a1980, a1981) {
                let a1982 = v1;
                let a1983 = 6u64;
                nat_add(a1982, a1983)?
            } else {
                let a1984 = v1;
                let a1985 = 2u64;
                nat_add(a1984, a1985)?
            }
        }, ({
            let a1986 = v2;
            let a1987 = v4;
            append_bytes(a1986, a1987)
        }, 0u64))))
    }
}

pub fn f459(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1990 = {
        let a1988 = v0;
        let a1989 = v1;
        index_bytes(a1988, a1989)
    };
    match m1990 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f501(v2)? {
                Ok(Some({
                    let a1991 = v1;
                    let a1992 = 1u64;
                    nat_add(a1991, a1992)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f460(v0: u8) -> R<u64> {
    let a2039 = {
        let v1: u8 = v0;
        let a2037 = {
            let a2013 = {
                let a2001 = {
                    let a1995 = {
                        let a1993 = v1;
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
                        let a1997 = v1;
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
                        let a2003 = v1;
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
                        let a2007 = v1;
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
                        let a2015 = v1;
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
                        let a2019 = v1;
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
                        let a2025 = v1;
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
                        let a2029 = v1;
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
    };
    let a2040 = 48u64;
    Ok(nat_sub(a2039, a2040))
}

pub fn f461(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2041 = v0.clone();
        length_bytes(a2041)
    };
    let a2042 = v1;
    let a2043 = v2;
    if if nat_lt(a2042, a2043) {
        let a2048 = f110(v0, {
            let a2046 = v2;
            let a2047 = {
                let a2044 = v1;
                let a2045 = 1u64;
                nat_add(a2044, a2045)?
            };
            nat_sub(a2046, a2047)
        })?;
        let a2049 = 48u64;
        nat_eq(a2048, a2049)
    } else {
        false
    } {
        Ok(Some({
            let a2050 = v1;
            let a2051 = 1u64;
            nat_add(a2050, a2051)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f462(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f112(f32({
        let (h2052, _) = v0;
        h2052
    }, {
        let (h2053, _) = v1;
        h2053
    }), f32({
        let (_, h2054) = v0;
        h2054
    }, {
        let (_, h2055) = v1;
        h2055
    }))
}

pub fn f463(v0: Fn14, p2059: List<List<(Bytes, ())>>, p2060: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2058 = (p2059, p2060);
    {
        let mut m2058 = m2058;
        loop {
            let step = {
                let (v1, v2) = m2058;
                let m2056 = v2;
                match m2056.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2057: Fn14 = v0.clone();
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

pub fn f464(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f465(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2062 = {
        let a2061 = v0.clone();
        length_list(a2061)
    };
    let a2063 = 1u64;
    if nat_le(a2062, a2063) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f502(v0)?))
    }
}

pub fn f466(v0: Fn15, p2067: List<(Bytes, ())>, p2068: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2066 = (p2067, p2068);
    {
        let mut m2066 = m2066;
        loop {
            let step = {
                let (v1, v2) = m2066;
                let m2064 = v2;
                match m2064.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2065: Fn15 = v0.clone();
                            c2065.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f468(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f143(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f57(v3.clone(), f143(f91(v0, f155())?, f8(Str::lit("at"))));
    let a2070 = {
        let a2069 = v4.clone();
        length_list(a2069)
    };
    let a2071 = 0u64;
    if nat_eq(a2070, a2071) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2074 = {
            let a2072 = v4;
            length_list(a2072)
        };
        let a2075 = {
            let a2073 = v3;
            length_list(a2073)
        };
        if nat_eq(a2074, a2075) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f503(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f469(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f358(f266(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f358(f266(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f120(f143(v3.clone(), f8(Str::lit("id"))), f143(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f143(f91(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f57(v5.clone(), f120(v6.clone(), f143(f91(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2077 = {
        let a2076 = v7.clone();
        length_list(a2076)
    };
    let a2078 = 0u64;
    if nat_eq(a2077, a2078) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f46({
            let a2081 = {
                let a2079 = v7;
                length_list(a2079)
            };
            let a2082 = {
                let a2080 = v5;
                length_list(a2080)
            };
            nat_eq(a2081, a2082)
        }) {
            Ok((false, f503(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f147(f120(f57(f504(v4.clone()), f58(f59(v0, f8(Str::lit("model/authorities.toml"))))?), f57(f505(v4), f120(v6, f143(v3, f8(Str::lit("id")))))))?;
            let a2084 = {
                let a2083 = v8.clone();
                length_list(a2083)
            };
            let a2085 = 0u64;
            if nat_eq(a2084, a2085) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f503(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f117(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f470(v0: u64) -> bool {
    let a2086 = v0;
    let a2087 = 47u64;
    nat_eq(a2086, a2087)
}

pub fn f471(v0: u64) -> bool {
    f199({
        let a2088 = v0;
        let a2089 = 95u64;
        nat_eq(a2088, a2089)
    }, f199({
        let a2090 = v0;
        let a2091 = 45u64;
        nat_eq(a2090, a2091)
    }, f199({
        let a2092 = v0;
        let a2093 = 32u64;
        nat_eq(a2092, a2093)
    }, f199({
        let a2094 = v0;
        let a2095 = 46u64;
        nat_eq(a2094, a2095)
    }, {
        let a2096 = v0;
        let a2097 = 47u64;
        nat_eq(a2096, a2097)
    }))))
}

pub fn f472(v0: u64) -> bool {
    f112({
        let a2098 = 97u64;
        let a2099 = v0;
        nat_le(a2098, a2099)
    }, {
        let a2100 = v0;
        let a2101 = 122u64;
        nat_le(a2100, a2101)
    })
}

pub fn f473(v0: u64) -> bool {
    f112({
        let a2102 = 65u64;
        let a2103 = v0;
        nat_le(a2102, a2103)
    }, {
        let a2104 = v0;
        let a2105 = 90u64;
        nat_le(a2104, a2105)
    })
}

pub fn f474(v0: u64) -> bool {
    f112({
        let a2106 = 48u64;
        let a2107 = v0;
        nat_le(a2106, a2107)
    }, {
        let a2108 = v0;
        let a2109 = 57u64;
        nat_le(a2108, a2109)
    })
}

pub fn f475(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2110 = v0;
    match m2110.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f475(v3, v1))
        }
    }
}

pub fn f476(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2111 = v0;
    match m2111 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f506(v1)
        }
    }
}

pub fn f477(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2112 = v0;
    if m2112 == 0 {
        let m2113 = v1;
        match m2113.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2112 - 1;
        let m2114 = v1.clone();
        match m2114.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2115 = f507(v1.clone(), v2.clone(), v3.clone())?;
                match m2115 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f377(f478(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2118 = f477(v6, f508(v1, {
                            let (h2116, _) = v9.clone();
                            h2116
                        }), List::cons(f363({
                            let (h2117, _) = v9.clone();
                            h2117
                        })?, v2), v3)?;
                        match m2118 {
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

pub fn f478(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2119 = v0;
    match m2119.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f363({
                let (h2120, _) = v1;
                h2120
            })?, f478(v2)?))
        }
    }
}

pub fn f479(p2126: u64, p2127: List<Adt25>, p2128: List<Bytes>, p2129: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2125 = (p2126, (p2127, (p2128, p2129)));
    {
        let mut m2125 = m2125;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2125;
                let m2121 = v0;
                if m2121 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2121 - 1;
                    let m2122 = v1;
                    match m2122.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2123 = v5;
                            match m2123 {
                                Adt25::C0(v8, v9) => {
                                    if f32(v8.clone(), f509()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f47(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2124 = f511(v8.clone(), v9)?;
                                            match m2124 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f480(f512(v11.clone()), List::cons(Adt25::C1(f513(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f47(f509(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f509(), v2), List::cons(f510(), v3)))))
                                    }
                                }
                            }
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

pub fn f480(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2130 = v0;
    match m2130.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f480(v3, v1))
        }
    }
}

pub fn f481(v0: List<(Bytes, Adt2)>) -> R<List<Adt25>> {
    let m2131 = v0;
    match m2131.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f363({
                let (h2132, _) = v1;
                h2132
            })?, {
                let (_, h2133) = v1;
                h2133
            }), f481(v2)?))
        }
    }
}

pub fn f482(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2134 = v0;
    match m2134.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f32(f53(v3.clone(), v1.clone()), v2.clone()) {
                f482(v4, v1, v2)
            } else {
                List::cons(v3, f482(v4, v1, v2))
            }
        }
    }
}

pub fn f483(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2135 = v0;
    match m2135.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f80(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f377(f484(f147(List::cons(v1, f514(f39(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f483(v3, v1)?))
            }
        }
    }
}

pub fn f484(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2136 = v0;
    match m2136.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f340(v1)?, f484(v2)?))
        }
    }
}

pub fn f485(v0: u32) -> Bytes {
    let a2139 = {
        let a2137 = v0;
        let a2138 = 8u32;
        fixed_u32::bit_and(a2137, a2138)
    };
    let a2140 = 0u32;
    if equal(a2139, a2140) {
        let a2143 = {
            let a2141 = v0;
            let a2142 = 4u32;
            fixed_u32::bit_and(a2141, a2142)
        };
        let a2144 = 0u32;
        if equal(a2143, a2144) {
            let a2147 = {
                let a2145 = v0;
                let a2146 = 2u32;
                fixed_u32::bit_and(a2145, a2146)
            };
            let a2148 = 0u32;
            if equal(a2147, a2148) {
                let a2151 = {
                    let a2149 = v0;
                    let a2150 = 1u32;
                    fixed_u32::bit_and(a2149, a2150)
                };
                let a2152 = 0u32;
                if equal(a2151, a2152) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2155 = {
                    let a2153 = v0;
                    let a2154 = 1u32;
                    fixed_u32::bit_and(a2153, a2154)
                };
                let a2156 = 0u32;
                if equal(a2155, a2156) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2159 = {
                let a2157 = v0;
                let a2158 = 2u32;
                fixed_u32::bit_and(a2157, a2158)
            };
            let a2160 = 0u32;
            if equal(a2159, a2160) {
                let a2163 = {
                    let a2161 = v0;
                    let a2162 = 1u32;
                    fixed_u32::bit_and(a2161, a2162)
                };
                let a2164 = 0u32;
                if equal(a2163, a2164) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2167 = {
                    let a2165 = v0;
                    let a2166 = 1u32;
                    fixed_u32::bit_and(a2165, a2166)
                };
                let a2168 = 0u32;
                if equal(a2167, a2168) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2171 = {
            let a2169 = v0;
            let a2170 = 4u32;
            fixed_u32::bit_and(a2169, a2170)
        };
        let a2172 = 0u32;
        if equal(a2171, a2172) {
            let a2175 = {
                let a2173 = v0;
                let a2174 = 2u32;
                fixed_u32::bit_and(a2173, a2174)
            };
            let a2176 = 0u32;
            if equal(a2175, a2176) {
                let a2179 = {
                    let a2177 = v0;
                    let a2178 = 1u32;
                    fixed_u32::bit_and(a2177, a2178)
                };
                let a2180 = 0u32;
                if equal(a2179, a2180) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2183 = {
                    let a2181 = v0;
                    let a2182 = 1u32;
                    fixed_u32::bit_and(a2181, a2182)
                };
                let a2184 = 0u32;
                if equal(a2183, a2184) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2187 = {
                let a2185 = v0;
                let a2186 = 2u32;
                fixed_u32::bit_and(a2185, a2186)
            };
            let a2188 = 0u32;
            if equal(a2187, a2188) {
                let a2191 = {
                    let a2189 = v0;
                    let a2190 = 1u32;
                    fixed_u32::bit_and(a2189, a2190)
                };
                let a2192 = 0u32;
                if equal(a2191, a2192) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2195 = {
                    let a2193 = v0;
                    let a2194 = 1u32;
                    fixed_u32::bit_and(a2193, a2194)
                };
                let a2196 = 0u32;
                if equal(a2195, a2196) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f486(v0: u32, v1: u32) -> u32 {
    let m2199 = {
        let a2197 = v0;
        let a2198 = v1;
        fixed_u32::shift_right(a2197, a2198)
    };
    match m2199 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f487(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f515(Fn18::F516, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2200, _, _, _, _, _, _, _) => {
            h2200
        }
    }, match v0.clone() {
        Adt24::C0(_, h2201, _, _, _, _, _, _) => {
            h2201
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2202, _, _, _, _, _) => {
            h2202
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2203, _, _, _, _) => {
            h2203
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2204, _, _, _) => {
            h2204
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2205, _, _) => {
            h2205
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2206, _) => {
            h2206
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2207) => {
            h2207
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f517(match v0.clone() {
        Adt24::C0(h2208, _, _, _, _, _, _, _) => {
            h2208
        }
    }, match v2.clone() {
        Adt27::C0(h2209, _, _, _, _, _, _, _, _, _) => {
            h2209
        }
    }), f517(match v0.clone() {
        Adt24::C0(_, h2210, _, _, _, _, _, _) => {
            h2210
        }
    }, match v2.clone() {
        Adt27::C0(_, h2211, _, _, _, _, _, _, _, _) => {
            h2211
        }
    }), f517(match v0.clone() {
        Adt24::C0(_, _, h2212, _, _, _, _, _) => {
            h2212
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2213, _, _, _, _, _, _, _) => {
            h2213
        }
    }), f517(match v0.clone() {
        Adt24::C0(_, _, _, h2214, _, _, _, _) => {
            h2214
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2215, _, _, _, _, _, _) => {
            h2215
        }
    }), f517(match v0.clone() {
        Adt24::C0(_, _, _, _, h2216, _, _, _) => {
            h2216
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2217, _, _, _, _, _) => {
            h2217
        }
    }), f517(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2218, _, _) => {
            h2218
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2219, _, _, _, _) => {
            h2219
        }
    }), f517(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2220, _) => {
            h2220
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2221, _, _, _) => {
            h2221
        }
    }), f517(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2222) => {
            h2222
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2223, _, _) => {
            h2223
        }
    })))
}

pub fn f488(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f518(v0.clone(), v1, v2, v3)?, f518(v0.clone(), v1, v2, {
        let a2224 = v3;
        let a2225 = 4u64;
        nat_add(a2224, a2225)?
    })?, f518(v0.clone(), v1, v2, {
        let a2226 = v3;
        let a2227 = 8u64;
        nat_add(a2226, a2227)?
    })?, f518(v0.clone(), v1, v2, {
        let a2228 = v3;
        let a2229 = 12u64;
        nat_add(a2228, a2229)?
    })?, f518(v0.clone(), v1, v2, {
        let a2230 = v3;
        let a2231 = 16u64;
        nat_add(a2230, a2231)?
    })?, f518(v0.clone(), v1, v2, {
        let a2232 = v3;
        let a2233 = 20u64;
        nat_add(a2232, a2233)?
    })?, f518(v0.clone(), v1, v2, {
        let a2234 = v3;
        let a2235 = 24u64;
        nat_add(a2234, a2235)?
    })?, f518(v0.clone(), v1, v2, {
        let a2236 = v3;
        let a2237 = 28u64;
        nat_add(a2236, a2237)?
    })?, f518(v0.clone(), v1, v2, {
        let a2238 = v3;
        let a2239 = 32u64;
        nat_add(a2238, a2239)?
    })?, f518(v0.clone(), v1, v2, {
        let a2240 = v3;
        let a2241 = 36u64;
        nat_add(a2240, a2241)?
    })?, f518(v0.clone(), v1, v2, {
        let a2242 = v3;
        let a2243 = 40u64;
        nat_add(a2242, a2243)?
    })?, f518(v0.clone(), v1, v2, {
        let a2244 = v3;
        let a2245 = 44u64;
        nat_add(a2244, a2245)?
    })?, f518(v0.clone(), v1, v2, {
        let a2246 = v3;
        let a2247 = 48u64;
        nat_add(a2246, a2247)?
    })?, f518(v0.clone(), v1, v2, {
        let a2248 = v3;
        let a2249 = 52u64;
        nat_add(a2248, a2249)?
    })?, f518(v0.clone(), v1, v2, {
        let a2250 = v3;
        let a2251 = 56u64;
        nat_add(a2250, a2251)?
    })?, f518(v0, v1, v2, {
        let a2252 = v3;
        let a2253 = 60u64;
        nat_add(a2252, a2253)?
    })?))
}

pub fn f489(p2256: List<Bytes>, v1: Bytes) -> bool {
    let m2255 = p2256;
    {
        let mut m2255 = m2255;
        loop {
            let step = {
                let v0 = m2255;
                let m2254 = v0;
                match m2254.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f80(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: Bytes, v1: u64) -> Bytes {
    let m2260 = {
        let a2257 = v0;
        let a2258 = v1;
        let a2259 = 1u64;
        slice_bytes(a2257, a2258, a2259)
    };
    match m2260 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f491(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f519(v1, v0, v2)
}

pub fn f492(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2305 = {
            let a2281 = {
                let a2269 = {
                    let a2263 = {
                        let a2261 = v3;
                        let a2262 = 128u8;
                        fixed_u8::bit_and(a2261, a2262)
                    };
                    let a2264 = 128u8;
                    if equal(a2263, a2264) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2270 = {
                    let a2267 = {
                        let a2265 = v3;
                        let a2266 = 64u8;
                        fixed_u8::bit_and(a2265, a2266)
                    };
                    let a2268 = 64u8;
                    if equal(a2267, a2268) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2269, a2270)?
            };
            let a2282 = {
                let a2279 = {
                    let a2273 = {
                        let a2271 = v3;
                        let a2272 = 32u8;
                        fixed_u8::bit_and(a2271, a2272)
                    };
                    let a2274 = 32u8;
                    if equal(a2273, a2274) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2280 = {
                    let a2277 = {
                        let a2275 = v3;
                        let a2276 = 16u8;
                        fixed_u8::bit_and(a2275, a2276)
                    };
                    let a2278 = 16u8;
                    if equal(a2277, a2278) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2279, a2280)?
            };
            nat_add(a2281, a2282)?
        };
        let a2306 = {
            let a2303 = {
                let a2291 = {
                    let a2285 = {
                        let a2283 = v3;
                        let a2284 = 8u8;
                        fixed_u8::bit_and(a2283, a2284)
                    };
                    let a2286 = 8u8;
                    if equal(a2285, a2286) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2292 = {
                    let a2289 = {
                        let a2287 = v3;
                        let a2288 = 4u8;
                        fixed_u8::bit_and(a2287, a2288)
                    };
                    let a2290 = 4u8;
                    if equal(a2289, a2290) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2291, a2292)?
            };
            let a2304 = {
                let a2301 = {
                    let a2295 = {
                        let a2293 = v3;
                        let a2294 = 2u8;
                        fixed_u8::bit_and(a2293, a2294)
                    };
                    let a2296 = 2u8;
                    if equal(a2295, a2296) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2302 = {
                    let a2299 = {
                        let a2297 = v3;
                        let a2298 = 1u8;
                        fixed_u8::bit_and(a2297, a2298)
                    };
                    let a2300 = 1u8;
                    if equal(a2299, a2300) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2301, a2302)?
            };
            nat_add(a2303, a2304)?
        };
        nat_add(a2305, a2306)?
    };
    let a2307 = v4;
    let a2308 = 34u64;
    if nat_eq(a2307, a2308) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2309 = v4;
        let a2310 = 92u64;
        if nat_eq(a2309, a2310) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2311 = v4;
            let a2312 = 8u64;
            if nat_eq(a2311, a2312) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2313 = v4;
                let a2314 = 12u64;
                if nat_eq(a2313, a2314) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2315 = v4;
                    let a2316 = 10u64;
                    if nat_eq(a2315, a2316) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2317 = v4;
                        let a2318 = 13u64;
                        if nat_eq(a2317, a2318) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2319 = v4;
                            let a2320 = 9u64;
                            if nat_eq(a2319, a2320) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2321 = v4;
                                let a2322 = 32u64;
                                if nat_lt(a2321, a2322) {
                                    let a2323 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2324 = f520(v2)?;
                                    Ok(append_bytes(a2323, a2324))
                                } else {
                                    Ok(f41(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f493(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f521(Fn19::F522, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f494(v0: Fn16, p2328: List<List<(Bytes, Bytes)>>, p2329: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2327 = (p2328, p2329);
    {
        let mut m2327 = m2327;
        loop {
            let step = {
                let (v1, v2) = m2327;
                let m2325 = v2;
                match m2325.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2326: Fn16 = v0.clone();
                            c2326.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f496(v0: Fn17, p2334: u64, p2335: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2333 = (p2334, p2335);
    {
        let mut m2333 = m2333;
        loop {
            let step = {
                let (v1, v2) = m2333;
                let m2330 = v1;
                if m2330 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2330 - 1;
                    let m2332 = {
                        let c2331: Fn17 = v0.clone();
                        c2331.apply(v2.clone())?
                    };
                    match m2332 {
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
                Ok(next) => { m2333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f523(v0)
}

pub fn f498(v0: u8) -> bool {
    let a2336 = v0;
    let a2337 = 32u8;
    if if if equal(a2336, a2337) {
        true
    } else {
        let a2338 = v0;
        let a2339 = 9u8;
        equal(a2338, a2339)
    } {
        true
    } else {
        let a2340 = v0;
        let a2341 = 10u8;
        equal(a2340, a2341)
    } {
        true
    } else {
        let a2342 = v0;
        let a2343 = 12u8;
        if equal(a2342, a2343) {
            true
        } else {
            let a2344 = v0;
            let a2345 = 13u8;
            equal(a2344, a2345)
        }
    }
}

pub fn f499(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2348 = f41(v0.clone(), {
        let a2346 = v1;
        let a2347 = 2u64;
        nat_add(a2346, a2347)?
    }, 2u64);
    let a2349 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2348, a2349) {
        let m2352 = f524(f41(v0, {
            let a2350 = v1;
            let a2351 = 4u64;
            nat_add(a2350, a2351)?
        }, 2u64))?;
        match m2352 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f110(v2.clone(), 0u64)?;
                let a2353 = v3;
                let a2354 = 32u64;
                if nat_lt(a2353, a2354) {
                    if f525(v3) {
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

pub fn f500(v0: u64) -> Bytes {
    let a2355 = v0;
    let a2356 = 34u64;
    if nat_eq(a2355, a2356) {
        Bytes::lit(&[0x22])
    } else {
        let a2357 = v0;
        let a2358 = 92u64;
        if nat_eq(a2357, a2358) {
            Bytes::lit(&[0x5c])
        } else {
            let a2359 = v0;
            let a2360 = 98u64;
            if nat_eq(a2359, a2360) {
                Bytes::lit(&[0x08])
            } else {
                let a2361 = v0;
                let a2362 = 102u64;
                if nat_eq(a2361, a2362) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2363 = v0;
                    let a2364 = 110u64;
                    if nat_eq(a2363, a2364) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2365 = v0;
                        let a2366 = 114u64;
                        if nat_eq(a2365, a2366) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2367 = v0;
                            let a2368 = 116u64;
                            if nat_eq(a2367, a2368) {
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

pub fn f501(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2413 = {
            let a2389 = {
                let a2377 = {
                    let a2371 = {
                        let a2369 = v1;
                        let a2370 = 128u8;
                        fixed_u8::bit_and(a2369, a2370)
                    };
                    let a2372 = 128u8;
                    if equal(a2371, a2372) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2378 = {
                    let a2375 = {
                        let a2373 = v1;
                        let a2374 = 64u8;
                        fixed_u8::bit_and(a2373, a2374)
                    };
                    let a2376 = 64u8;
                    if equal(a2375, a2376) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2377, a2378)?
            };
            let a2390 = {
                let a2387 = {
                    let a2381 = {
                        let a2379 = v1;
                        let a2380 = 32u8;
                        fixed_u8::bit_and(a2379, a2380)
                    };
                    let a2382 = 32u8;
                    if equal(a2381, a2382) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2388 = {
                    let a2385 = {
                        let a2383 = v1;
                        let a2384 = 16u8;
                        fixed_u8::bit_and(a2383, a2384)
                    };
                    let a2386 = 16u8;
                    if equal(a2385, a2386) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2387, a2388)?
            };
            nat_add(a2389, a2390)?
        };
        let a2414 = {
            let a2411 = {
                let a2399 = {
                    let a2393 = {
                        let a2391 = v1;
                        let a2392 = 8u8;
                        fixed_u8::bit_and(a2391, a2392)
                    };
                    let a2394 = 8u8;
                    if equal(a2393, a2394) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2400 = {
                    let a2397 = {
                        let a2395 = v1;
                        let a2396 = 4u8;
                        fixed_u8::bit_and(a2395, a2396)
                    };
                    let a2398 = 4u8;
                    if equal(a2397, a2398) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2399, a2400)?
            };
            let a2412 = {
                let a2409 = {
                    let a2403 = {
                        let a2401 = v1;
                        let a2402 = 2u8;
                        fixed_u8::bit_and(a2401, a2402)
                    };
                    let a2404 = 2u8;
                    if equal(a2403, a2404) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2410 = {
                    let a2407 = {
                        let a2405 = v1;
                        let a2406 = 1u8;
                        fixed_u8::bit_and(a2405, a2406)
                    };
                    let a2408 = 1u8;
                    if equal(a2407, a2408) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2409, a2410)?
            };
            nat_add(a2411, a2412)?
        };
        nat_add(a2413, a2414)?
    };
    let a2415 = 48u64;
    let a2416 = v2;
    if nat_le(a2415, a2416) {
        let a2417 = v2;
        let a2418 = 57u64;
        Ok(nat_le(a2417, a2418))
    } else {
        Ok(false)
    }
}

pub fn f502(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f418({
        let (_, h2419) = {
            let (h2420, _) = f526(Fn20::F527, {
                let a2422 = {
                    let a2421 = v0.clone();
                    length_list(a2421)
                };
                let a2423 = 1u64;
                nat_add(a2422, a2423)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2420
        };
        h2419
    }))
}

pub fn f503(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f504(v0: List<Adt14>) -> List<Bytes> {
    let m2424 = v0;
    match m2424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2425 = f88(v1, f8(Str::lit("std")));
            match m2425 {
                None => {
                    f504(v2)
                }
                Some(v3) => {
                    List::cons(f53(v3, f8(Str::lit("authority"))), f504(v2))
                }
            }
        }
    }
}

pub fn f505(v0: List<Adt14>) -> List<Bytes> {
    let m2426 = v0;
    match m2426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2427 = f87(f88(v1, f8(Str::lit("req"))));
            match m2427 {
                None => {
                    f505(v2)
                }
                Some(v3) => {
                    List::cons(v3, f505(v2))
                }
            }
        }
    }
}

pub fn f506(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2428 = v0;
    match m2428 {
        Adt2::C0(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C1(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C2(_, _, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C3(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C4(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C5(r2429) => {
            let v4 = (*r2429).clone();
            f506(v4)
        }
        Adt2::C6 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C7(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C8(v1, r2430, r2431) => {
            let v2 = (*r2430).clone();
            let v3 = (*r2431).clone();
            List::cons((v1, v2), f506(v3))
        }
    }
}

pub fn f507(p2436: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2435 = p2436;
    {
        let mut m2435 = m2435;
        loop {
            let step = {
                let v0 = m2435;
                let m2432 = v0;
                match m2432.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f528(f529({
                            let (_, h2433) = v3.clone();
                            h2433
                        }, List::<Bytes>::nil())?, f363({
                            let (h2434, _) = v3.clone();
                            h2434
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2437 = v0;
    match m2437.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2438, _) = v2.clone();
                h2438
            }, v1.clone()) {
                f508(v3, v1)
            } else {
                List::cons(v2, f508(v3, v1))
            }
        }
    }
}

pub fn f509() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f510() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f511(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    f530(v0, f531(v1))
}

pub fn f512(v0: Adt28) -> List<Adt25> {
    let m2439 = v0;
    match m2439 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f513(v0: Adt28) -> Bytes {
    let m2440 = v0;
    match m2440 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f514(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2441 = v0;
    match m2441.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f514(v3, false))
            } else {
                f514(v3, true)
            }
        }
    }
}

pub fn f515(v0: Fn18, p2445: u64, p2446: Adt27) -> R<Adt27> {
    let m2444 = (p2445, p2446);
    {
        let mut m2444 = m2444;
        loop {
            let step = {
                let (v1, v2) = m2444;
                let m2442 = v1;
                if m2442 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2442 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2443: Fn18 = v0.clone();
                        c2443.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: Adt27) -> R<Adt27> {
    f532(v0)
}

pub fn f517(v0: u32, v1: u32) -> u32 {
    let m2449 = {
        let a2447 = f533(v0);
        let a2448 = f533(v1);
        fixed_u64::checked_add(a2447, a2448)
    };
    match m2449 {
        None => {
            0u32
        }
        Some(v2) => {
            f534(v2)
        }
    }
}

pub fn f518(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2452 = {
        let a2450 = v3;
        let a2451 = 8u64;
        nat_add(a2450, a2451)?
    };
    let a2453 = v2;
    if nat_eq(a2452, a2453) {
        Ok(f535({
            let a2456 = {
                let a2454 = v1;
                let a2455 = 8u64;
                nat_mul(a2454, a2455)?
            };
            let a2457 = 4294967296u64;
            let a2458 = 0u64;
            nat_quot(a2456, a2457, a2458)
        }))
    } else {
        let a2461 = {
            let a2459 = v3;
            let a2460 = 4u64;
            nat_add(a2459, a2460)?
        };
        let a2462 = v2;
        if nat_eq(a2461, a2462) {
            Ok(f535({
                let a2465 = {
                    let a2463 = v1;
                    let a2464 = 8u64;
                    nat_mul(a2463, a2464)?
                };
                let a2466 = 4294967296u64;
                let a2467 = 0u64;
                nat_rem(a2465, a2466, a2467)
            }))
        } else {
            Ok(f536(f537(v0.clone(), v1, v3), f537(v0.clone(), v1, {
                let a2468 = v3;
                let a2469 = 1u64;
                nat_add(a2468, a2469)?
            }), f537(v0.clone(), v1, {
                let a2470 = v3;
                let a2471 = 2u64;
                nat_add(a2470, a2471)?
            }), f537(v0, v1, {
                let a2472 = v3;
                let a2473 = 3u64;
                nat_add(a2472, a2473)?
            })))
        }
    }
}

pub fn f519(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2474, _) = v2;
        h2474
    };
    let a2475 = v3;
    let a2476 = 0u64;
    if nat_eq(a2475, a2476) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2477 = v3;
            let a2478 = v1;
            let a2479 = 0u64;
            nat_quot(a2477, a2478, a2479)
        }, {
            let a2484 = f490(v0, {
                let a2480 = v3;
                let a2481 = v1;
                let a2482 = 0u64;
                nat_rem(a2480, a2481, a2482)
            });
            let a2485 = {
                let (_, h2483) = v2;
                h2483
            };
            append_bytes(a2484, a2485)
        }))
    }
}

pub fn f520(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2530 = {
            let a2506 = {
                let a2494 = {
                    let a2488 = {
                        let a2486 = v1;
                        let a2487 = 128u8;
                        fixed_u8::bit_and(a2486, a2487)
                    };
                    let a2489 = 128u8;
                    if equal(a2488, a2489) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2495 = {
                    let a2492 = {
                        let a2490 = v1;
                        let a2491 = 64u8;
                        fixed_u8::bit_and(a2490, a2491)
                    };
                    let a2493 = 64u8;
                    if equal(a2492, a2493) {
                        64u64
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
                        let a2497 = 32u8;
                        fixed_u8::bit_and(a2496, a2497)
                    };
                    let a2499 = 32u8;
                    if equal(a2498, a2499) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2505 = {
                    let a2502 = {
                        let a2500 = v1;
                        let a2501 = 16u8;
                        fixed_u8::bit_and(a2500, a2501)
                    };
                    let a2503 = 16u8;
                    if equal(a2502, a2503) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2504, a2505)?
            };
            nat_add(a2506, a2507)?
        };
        let a2531 = {
            let a2528 = {
                let a2516 = {
                    let a2510 = {
                        let a2508 = v1;
                        let a2509 = 8u8;
                        fixed_u8::bit_and(a2508, a2509)
                    };
                    let a2511 = 8u8;
                    if equal(a2510, a2511) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2517 = {
                    let a2514 = {
                        let a2512 = v1;
                        let a2513 = 4u8;
                        fixed_u8::bit_and(a2512, a2513)
                    };
                    let a2515 = 4u8;
                    if equal(a2514, a2515) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2516, a2517)?
            };
            let a2529 = {
                let a2526 = {
                    let a2520 = {
                        let a2518 = v1;
                        let a2519 = 2u8;
                        fixed_u8::bit_and(a2518, a2519)
                    };
                    let a2521 = 2u8;
                    if equal(a2520, a2521) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2527 = {
                    let a2524 = {
                        let a2522 = v1;
                        let a2523 = 1u8;
                        fixed_u8::bit_and(a2522, a2523)
                    };
                    let a2525 = 1u8;
                    if equal(a2524, a2525) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2526, a2527)?
            };
            nat_add(a2528, a2529)?
        };
        nat_add(a2530, a2531)?
    };
    let a2538 = f490(f538(), {
        let a2532 = v2;
        let a2533 = 16u64;
        let a2534 = 0u64;
        nat_quot(a2532, a2533, a2534)
    });
    let a2539 = f490(f538(), {
        let a2535 = v2;
        let a2536 = 16u64;
        let a2537 = 0u64;
        nat_rem(a2535, a2536, a2537)
    });
    Ok(append_bytes(a2538, a2539))
}

pub fn f521(v0: Fn19, p2543: List<List<(Bytes, Bytes)>>, p2544: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2542 = (p2543, p2544);
    {
        let mut m2542 = m2542;
        loop {
            let step = {
                let (v1, v2) = m2542;
                let m2540 = v2;
                match m2540.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2541: Fn19 = v0.clone();
                            c2541.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f523(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2546 = {
        let a2545 = v0.clone();
        length_list(a2545)
    };
    let a2547 = 1u64;
    if nat_le(a2546, a2547) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f539(v0)?))
    }
}

pub fn f524(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2548 = v0.clone();
        length_bytes(a2548)
    };
    let a2552 = {
        let a2549 = v1;
        let a2550 = 2u64;
        let a2551 = 0u64;
        nat_rem(a2549, a2550, a2551)
    };
    let a2553 = 0u64;
    if nat_eq(a2552, a2553) {
        let (_, h2554) = {
            let (h2555, _) = f540(Fn21::F541(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2555
        };
        Ok(h2554)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f525(v0: u64) -> bool {
    let a2556 = v0;
    let a2557 = 8u64;
    if nat_eq(a2556, a2557) {
        true
    } else {
        let a2558 = v0;
        let a2559 = 9u64;
        if nat_eq(a2558, a2559) {
            true
        } else {
            let a2560 = v0;
            let a2561 = 10u64;
            if nat_eq(a2560, a2561) {
                true
            } else {
                let a2562 = v0;
                let a2563 = 12u64;
                if nat_eq(a2562, a2563) {
                    true
                } else {
                    let a2564 = v0;
                    let a2565 = 13u64;
                    nat_eq(a2564, a2565)
                }
            }
        }
    }
}

pub fn f526(v0: Fn20, p2570: u64, p2571: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2569 = (p2570, p2571);
    {
        let mut m2569 = m2569;
        loop {
            let step = {
                let (v1, v2) = m2569;
                let m2566 = v1;
                if m2566 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2566 - 1;
                    let m2568 = {
                        let c2567: Fn20 = v0.clone();
                        c2567.apply(v2.clone())?
                    };
                    match m2568 {
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
                Ok(next) => { m2569 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f542(v0)
}

pub fn f528(p2574: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2573 = p2574;
    {
        let mut m2573 = m2573;
        loop {
            let step = {
                let v0 = m2573;
                let m2572 = v0;
                match m2572.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f32(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f47(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f47(v4, v3.clone())
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
                Ok(next) => { m2573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2575 = v0;
    match m2575 {
        Adt2::C0(_) => {
            Ok(v1)
        }
        Adt2::C1(_, _) => {
            Ok(v1)
        }
        Adt2::C2(_, _, _) => {
            Ok(v1)
        }
        Adt2::C3(_) => {
            Ok(v1)
        }
        Adt2::C4(r2578) => {
            let v2 = (*r2578).clone();
            f529(v2, v1)
        }
        Adt2::C5(r2579) => {
            let v3 = (*r2579).clone();
            f529(v3, v1)
        }
        Adt2::C6 => {
            Ok(v1)
        }
        Adt2::C7(r2580, r2581) => {
            let v4 = (*r2580).clone();
            let v5 = (*r2581).clone();
            f529(v4, f529(v5, v1)?)
        }
        Adt2::C8(v6, r2582, r2583) => {
            let v7 = (*r2582).clone();
            let v8 = (*r2583).clone();
            let v9: List<Bytes> = f529(v7.clone(), f529(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2576 = f369(Some(v7));
                match m2576 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2577 = f543(v10)?;
                        match m2577 {
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

pub fn f530(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    let m2584 = f271(v1.clone(), f8(Str::lit("enum")));
    match m2584 {
        None => {
            let m2586 = f271(v1.clone(), f8(Str::lit("oneOf")));
            match m2586 {
                None => {
                    if f548(v1.clone(), v0.clone())? {
                        let m2587 = f549(v0.clone(), f476(f271(v1.clone(), f8(Str::lit("properties")))), f275(f276(f271(v1, f8(Str::lit("required"))))), true)?;
                        match m2587 {
                            Ok(v7) => {
                                Ok(f546(f550(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f377(f551(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f552(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2588 = f476(f271(v1.clone(), f8(Str::lit("properties"))));
                        match m2588.uncons() {
                            None => {
                                let m2591 = f553(v1, {
                                    let a2589 = v0.clone();
                                    let a2590 = f8(Str::lit("Value"));
                                    append_bytes(a2589, a2590)
                                })?;
                                match m2591 {
                                    Ok(v9) => {
                                        Ok(f546(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f513(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f512(v9)))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<Adt28, Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2592 = f549(v0.clone(), f476(f271(v1.clone(), f8(Str::lit("properties")))), f275(f276(f271(v1, f8(Str::lit("required"))))), false)?;
                                match m2592 {
                                    Ok(v13) => {
                                        Ok(f546(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2593 = f551(v13.clone());
                                            match m2593.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f377(f551(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f552(v13)))
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
                    f547(v0, f370(v5))
                }
            }
        }
        Some(v2) => {
            let m2585 = f544(f370(v2), List::<Bytes>::nil())?;
            match m2585 {
                Ok(v4) => {
                    Ok(f546(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f377(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f545(v3, v0))
                }
            }
        }
    }
}

pub fn f531(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2594 = f271(v0.clone(), f8(Str::lit("anyOf")));
        match m2594 {
            None => {
                v0
            }
            Some(v1) => {
                if f554(f370(v1.clone())) {
                    f555(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f556(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f557(f370(v1.clone()))
                    } {
                        v0
                    } else {
                        f558(f555(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2595 = f271(v2.clone(), f8(Str::lit("oneOf")));
        match m2595 {
            None => {
                v2
            }
            Some(v3) => {
                if f554(f370(v3)) {
                    f555(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2596 = f271(v4.clone(), f8(Str::lit("allOf")));
    match m2596 {
        None => {
            v4
        }
        Some(v5) => {
            if f559(f370(v5)) {
                f555(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f532(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f560(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2597) => {
            h2597
        }
    });
    let v2: u32 = f517(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2598, _, _) => {
            h2598
        }
    }, f517(f561(match v0.clone() {
        Adt27::C0(_, _, _, _, h2599, _, _, _, _, _) => {
            h2599
        }
    }), f517(f562(match v0.clone() {
        Adt27::C0(_, _, _, _, h2600, _, _, _, _, _) => {
            h2600
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2601, _, _, _, _) => {
            h2601
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2602, _, _, _) => {
            h2602
        }
    }), f517(v1, {
        let m2605 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2604, _) => {
                h2604
            }
        };
        match m2605 {
            Adt26::C0(h2603, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2603
            }
        }
    }))));
    let v3: u32 = f517(f563(match v0.clone() {
        Adt27::C0(h2606, _, _, _, _, _, _, _, _, _) => {
            h2606
        }
    }), f564(match v0.clone() {
        Adt27::C0(h2607, _, _, _, _, _, _, _, _, _) => {
            h2607
        }
    }, match v0.clone() {
        Adt27::C0(_, h2608, _, _, _, _, _, _, _, _) => {
            h2608
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2609, _, _, _, _, _, _, _) => {
            h2609
        }
    }));
    Ok(Adt27::C0(f517(v2, v3), match v0.clone() {
        Adt27::C0(h2610, _, _, _, _, _, _, _, _, _) => {
            h2610
        }
    }, match v0.clone() {
        Adt27::C0(_, h2611, _, _, _, _, _, _, _, _) => {
            h2611
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2612, _, _, _, _, _, _, _) => {
            h2612
        }
    }, f517(match v0.clone() {
        Adt27::C0(_, _, _, h2613, _, _, _, _, _, _) => {
            h2613
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2614, _, _, _, _, _) => {
            h2614
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2615, _, _, _, _) => {
            h2615
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2616, _, _, _) => {
            h2616
        }
    }, f565(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2617, _) => {
            h2617
        }
    }), {
        let a2619 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2618) => {
                h2618
            }
        };
        let a2620 = 1u64;
        nat_add(a2619, a2620)?
    }))
}

pub fn f533(v0: u32) -> u64 {
    let m2622 = {
        let a2621 = v0;
        fixed_u64::convert(i128::from(a2621))
    };
    match m2622 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f534(v0: u64) -> u32 {
    let m2626 = {
        let a2625 = {
            let a2623 = v0;
            let a2624 = 4294967295u64;
            fixed_u64::bit_and(a2623, a2624)
        };
        fixed_u32::convert(i128::from(a2625))
    };
    match m2626 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f535(v0: u64) -> u32 {
    let (_, h2627) = {
        let (_, h2628) = f566(Fn22::F567, 32u64, (v0, (1u32, 0u32)));
        h2628
    };
    h2627
}

pub fn f536(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2633 = f568(v0, 24u32);
    let a2634 = {
        let a2631 = f568(v1, 16u32);
        let a2632 = {
            let a2629 = f568(v2, 8u32);
            let a2630 = v3;
            fixed_u32::bit_or(a2629, a2630)
        };
        fixed_u32::bit_or(a2631, a2632)
    };
    fixed_u32::bit_or(a2633, a2634)
}

pub fn f537(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2635 = v2;
    let a2636 = v1;
    if nat_lt(a2635, a2636) {
        let m2639 = {
            let a2637 = v0;
            let a2638 = v2;
            index_bytes(a2637, a2638)
        };
        match m2639 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2641 = {
                    let a2640 = v3;
                    fixed_u32::convert(i128::from(a2640))
                };
                match m2641 {
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
        let a2642 = v2;
        let a2643 = v1;
        if nat_eq(a2642, a2643) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f538() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f539(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f493({
        let (_, h2644) = {
            let (h2645, _) = f569(Fn23::F570, {
                let a2647 = {
                    let a2646 = v0.clone();
                    length_list(a2646)
                };
                let a2648 = 1u64;
                nat_add(a2647, a2648)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2645
        };
        h2644
    }))
}

pub fn f540(v0: Fn21, p2653: u64, p2654: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2652 = (p2653, p2654);
    {
        let mut m2652 = m2652;
        loop {
            let step = {
                let (v1, v2) = m2652;
                let m2649 = v1;
                if m2649 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2649 - 1;
                    let m2651 = {
                        let c2650: Fn21 = v0.clone();
                        c2650.apply(v2.clone())?
                    };
                    match m2651 {
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
                Ok(next) => { m2652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f541(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f571(v0, v1)
}

pub fn f542(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2655, _) = v0;
        h2655
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2656) = v0;
        h2656
    };
    let m2657 = v1;
    match m2657.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2658 = v4;
            match m2658.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f572(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f543(v0: Bytes) -> R<Option<Bytes>> {
    if f80(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f363(f41(v0.clone(), 8u64, {
            let a2660 = {
                let a2659 = v0;
                length_bytes(a2659)
            };
            let a2661 = 8u64;
            nat_sub(a2660, a2661)
        }))?))
    } else {
        if f80(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f363(f41(v0.clone(), 14u64, {
                let a2663 = {
                    let a2662 = v0;
                    length_bytes(a2662)
                };
                let a2664 = 14u64;
                nat_sub(a2663, a2664)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f544(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f573(f574(f275(v0.clone()))?, v0, v1)
}

pub fn f545(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f546(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f547(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt28, Bytes>> {
    let m2665 = v1.clone();
    match m2665.uncons() {
        None => {
            Ok(f545(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2666 = f575(f476(f271(v2, f8(Str::lit("properties")))), v1.clone());
            match m2666.uncons() {
                None => {
                    let m2667 = f576(f574(f577(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2667 {
                        Ok(v5) => {
                            Ok(f546(f550(v0, f551(v5.clone())), f552(v5)))
                        }
                        Err(v4) => {
                            Ok(f545(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2668 = f578(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2668 {
                        Ok(v9) => {
                            Ok(f546(f550(v0, f551(v9.clone())), f552(v9)))
                        }
                        Err(v8) => {
                            Ok(f545(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f548(v0: Adt2, v1: Bytes) -> R<bool> {
    let m2669 = v0;
    match m2669 {
        Adt2::C0(_) => {
            Ok(false)
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(_, _, _) => {
            Ok(false)
        }
        Adt2::C3(_) => {
            Ok(false)
        }
        Adt2::C4(r2670) => {
            let v2 = (*r2670).clone();
            f548(v2, v1)
        }
        Adt2::C5(r2671) => {
            let v3 = (*r2671).clone();
            f548(v3, v1)
        }
        Adt2::C6 => {
            Ok(false)
        }
        Adt2::C7(r2672, r2673) => {
            let v4 = (*r2672).clone();
            let v5 = (*r2673).clone();
            if f548(v4, v1.clone())? {
                Ok(true)
            } else {
                f548(v5, v1)
            }
        }
        Adt2::C8(v6, r2674, r2675) => {
            let v7 = (*r2674).clone();
            let v8 = (*r2675).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f579(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f548(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f548(v8, v1)
                }
            }
        }
    }
}

pub fn f549(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f580(f574(f581(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f550(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f377(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f551(v0: Adt29) -> List<Bytes> {
    let m2676 = v0;
    match m2676 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f552(v0: Adt29) -> List<Adt25> {
    let m2677 = v0;
    match m2677 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f553(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2678 = f582(v0.clone());
    match m2678 {
        None => {
            if f32(f274(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2679 = f271(v0, f8(Str::lit("items")));
                match m2679 {
                    None => {
                        Ok(f583(f8(Str::lit("List")), f585()))
                    }
                    Some(v3) => {
                        Ok(f583(f8(Str::lit("List")), f584(v3, {
                            let a2680 = v1;
                            let a2681 = f8(Str::lit("Item"));
                            append_bytes(a2680, a2681)
                        })?))
                    }
                }
            } else {
                f584(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f583(f8(Str::lit("Option")), f584(v2, v1)?))
        }
    }
}

pub fn f554(p2684: List<Adt2>) -> bool {
    let m2683 = p2684;
    {
        let mut m2683 = m2683;
        loop {
            let step = {
                let v0 = m2683;
                let m2682 = v0;
                match m2682.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f586(f506(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(v0: Adt2, v1: Bytes) -> Adt2 {
    let m2685 = v0;
    match m2685 {
        Adt2::C0(v6) => {
            Adt2::C0(v6)
        }
        Adt2::C1(v10, v11) => {
            Adt2::C1(v10, v11)
        }
        Adt2::C2(v7, v8, v9) => {
            Adt2::C2(v7, v8, v9)
        }
        Adt2::C3(v12) => {
            Adt2::C3(v12)
        }
        Adt2::C4(r2686) => {
            let v13 = (*r2686).clone();
            Adt2::C4(std::rc::Rc::new(v13))
        }
        Adt2::C5(r2687) => {
            let v5 = (*r2687).clone();
            Adt2::C5(std::rc::Rc::new(f555(v5, v1)))
        }
        Adt2::C6 => {
            Adt2::C6
        }
        Adt2::C7(r2688, r2689) => {
            let v14 = (*r2688).clone();
            let v15 = (*r2689).clone();
            Adt2::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt2::C8(v2, r2690, r2691) => {
            let v3 = (*r2690).clone();
            let v4 = (*r2691).clone();
            if f32(v2.clone(), v1.clone()) {
                f555(v4, v1)
            } else {
                Adt2::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f555(v4, v1)))
            }
        }
    }
}

pub fn f556(v0: Adt2, v1: Bytes) -> bool {
    let m2692 = f271(v0, v1);
    match m2692 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f557(v0: List<Adt2>) -> bool {
    let m2693 = v0.clone();
    match m2693.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2694 = f575(f476(f271(v1, f8(Str::lit("properties")))), v0.clone());
            match m2694.uncons() {
                None => {
                    f588(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f587(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f558(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m2695 = v0;
    match m2695 {
        Adt2::C0(v7) => {
            Adt2::C0(v7)
        }
        Adt2::C1(v11, v12) => {
            Adt2::C1(v11, v12)
        }
        Adt2::C2(v8, v9, v10) => {
            Adt2::C2(v8, v9, v10)
        }
        Adt2::C3(v13) => {
            Adt2::C3(v13)
        }
        Adt2::C4(r2696) => {
            let v14 = (*r2696).clone();
            Adt2::C4(std::rc::Rc::new(v14))
        }
        Adt2::C5(r2697) => {
            let v3 = (*r2697).clone();
            Adt2::C5(std::rc::Rc::new(Adt2::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C6 => {
            Adt2::C6
        }
        Adt2::C7(r2698, r2699) => {
            let v15 = (*r2698).clone();
            let v16 = (*r2699).clone();
            Adt2::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C8(v4, r2700, r2701) => {
            let v5 = (*r2700).clone();
            let v6 = (*r2701).clone();
            Adt2::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f559(p2704: List<Adt2>) -> bool {
    let m2703 = p2704;
    {
        let mut m2703 = m2703;
        loop {
            let step = {
                let v0 = m2703;
                let m2702 = v0;
                match m2702.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f589(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
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

pub fn f560(v0: u64) -> u32 {
    let a2705 = v0;
    let a2706 = 32u64;
    if nat_lt(a2705, a2706) {
        let a2707 = v0;
        let a2708 = 16u64;
        if nat_lt(a2707, a2708) {
            let a2709 = v0;
            let a2710 = 8u64;
            if nat_lt(a2709, a2710) {
                let a2711 = v0;
                let a2712 = 4u64;
                if nat_lt(a2711, a2712) {
                    let a2713 = v0;
                    let a2714 = 2u64;
                    if nat_lt(a2713, a2714) {
                        let a2715 = v0;
                        let a2716 = 1u64;
                        if nat_lt(a2715, a2716) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2717 = v0;
                        let a2718 = 3u64;
                        if nat_lt(a2717, a2718) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2719 = v0;
                    let a2720 = 6u64;
                    if nat_lt(a2719, a2720) {
                        let a2721 = v0;
                        let a2722 = 5u64;
                        if nat_lt(a2721, a2722) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2723 = v0;
                        let a2724 = 7u64;
                        if nat_lt(a2723, a2724) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2725 = v0;
                let a2726 = 12u64;
                if nat_lt(a2725, a2726) {
                    let a2727 = v0;
                    let a2728 = 10u64;
                    if nat_lt(a2727, a2728) {
                        let a2729 = v0;
                        let a2730 = 9u64;
                        if nat_lt(a2729, a2730) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2731 = v0;
                        let a2732 = 11u64;
                        if nat_lt(a2731, a2732) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2733 = v0;
                    let a2734 = 14u64;
                    if nat_lt(a2733, a2734) {
                        let a2735 = v0;
                        let a2736 = 13u64;
                        if nat_lt(a2735, a2736) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2737 = v0;
                        let a2738 = 15u64;
                        if nat_lt(a2737, a2738) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2739 = v0;
            let a2740 = 24u64;
            if nat_lt(a2739, a2740) {
                let a2741 = v0;
                let a2742 = 20u64;
                if nat_lt(a2741, a2742) {
                    let a2743 = v0;
                    let a2744 = 18u64;
                    if nat_lt(a2743, a2744) {
                        let a2745 = v0;
                        let a2746 = 17u64;
                        if nat_lt(a2745, a2746) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2747 = v0;
                        let a2748 = 19u64;
                        if nat_lt(a2747, a2748) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2749 = v0;
                    let a2750 = 22u64;
                    if nat_lt(a2749, a2750) {
                        let a2751 = v0;
                        let a2752 = 21u64;
                        if nat_lt(a2751, a2752) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2753 = v0;
                        let a2754 = 23u64;
                        if nat_lt(a2753, a2754) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2755 = v0;
                let a2756 = 28u64;
                if nat_lt(a2755, a2756) {
                    let a2757 = v0;
                    let a2758 = 26u64;
                    if nat_lt(a2757, a2758) {
                        let a2759 = v0;
                        let a2760 = 25u64;
                        if nat_lt(a2759, a2760) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2761 = v0;
                        let a2762 = 27u64;
                        if nat_lt(a2761, a2762) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2763 = v0;
                    let a2764 = 30u64;
                    if nat_lt(a2763, a2764) {
                        let a2765 = v0;
                        let a2766 = 29u64;
                        if nat_lt(a2765, a2766) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2767 = v0;
                        let a2768 = 31u64;
                        if nat_lt(a2767, a2768) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2769 = v0;
        let a2770 = 48u64;
        if nat_lt(a2769, a2770) {
            let a2771 = v0;
            let a2772 = 40u64;
            if nat_lt(a2771, a2772) {
                let a2773 = v0;
                let a2774 = 36u64;
                if nat_lt(a2773, a2774) {
                    let a2775 = v0;
                    let a2776 = 34u64;
                    if nat_lt(a2775, a2776) {
                        let a2777 = v0;
                        let a2778 = 33u64;
                        if nat_lt(a2777, a2778) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2779 = v0;
                        let a2780 = 35u64;
                        if nat_lt(a2779, a2780) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2781 = v0;
                    let a2782 = 38u64;
                    if nat_lt(a2781, a2782) {
                        let a2783 = v0;
                        let a2784 = 37u64;
                        if nat_lt(a2783, a2784) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2785 = v0;
                        let a2786 = 39u64;
                        if nat_lt(a2785, a2786) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2787 = v0;
                let a2788 = 44u64;
                if nat_lt(a2787, a2788) {
                    let a2789 = v0;
                    let a2790 = 42u64;
                    if nat_lt(a2789, a2790) {
                        let a2791 = v0;
                        let a2792 = 41u64;
                        if nat_lt(a2791, a2792) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2793 = v0;
                        let a2794 = 43u64;
                        if nat_lt(a2793, a2794) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2795 = v0;
                    let a2796 = 46u64;
                    if nat_lt(a2795, a2796) {
                        let a2797 = v0;
                        let a2798 = 45u64;
                        if nat_lt(a2797, a2798) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2799 = v0;
                        let a2800 = 47u64;
                        if nat_lt(a2799, a2800) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2801 = v0;
            let a2802 = 56u64;
            if nat_lt(a2801, a2802) {
                let a2803 = v0;
                let a2804 = 52u64;
                if nat_lt(a2803, a2804) {
                    let a2805 = v0;
                    let a2806 = 50u64;
                    if nat_lt(a2805, a2806) {
                        let a2807 = v0;
                        let a2808 = 49u64;
                        if nat_lt(a2807, a2808) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2809 = v0;
                        let a2810 = 51u64;
                        if nat_lt(a2809, a2810) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2811 = v0;
                    let a2812 = 54u64;
                    if nat_lt(a2811, a2812) {
                        let a2813 = v0;
                        let a2814 = 53u64;
                        if nat_lt(a2813, a2814) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2815 = v0;
                        let a2816 = 55u64;
                        if nat_lt(a2815, a2816) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2817 = v0;
                let a2818 = 60u64;
                if nat_lt(a2817, a2818) {
                    let a2819 = v0;
                    let a2820 = 58u64;
                    if nat_lt(a2819, a2820) {
                        let a2821 = v0;
                        let a2822 = 57u64;
                        if nat_lt(a2821, a2822) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2823 = v0;
                        let a2824 = 59u64;
                        if nat_lt(a2823, a2824) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2825 = v0;
                    let a2826 = 62u64;
                    if nat_lt(a2825, a2826) {
                        let a2827 = v0;
                        let a2828 = 61u64;
                        if nat_lt(a2827, a2828) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2829 = v0;
                        let a2830 = 63u64;
                        if nat_lt(a2829, a2830) {
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

pub fn f561(v0: u32) -> u32 {
    let a2833 = {
        let a2831 = f590(v0, 6u32, 26u32);
        let a2832 = f590(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2831, a2832)
    };
    let a2834 = f590(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2833, a2834)
}

pub fn f562(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2840 = {
        let a2835 = v0;
        let a2836 = v1;
        fixed_u32::bit_and(a2835, a2836)
    };
    let a2841 = {
        let a2838 = {
            let a2837 = v0;
            fixed_u32::bit_not(a2837)
        };
        let a2839 = v2;
        fixed_u32::bit_and(a2838, a2839)
    };
    fixed_u32::bit_xor(a2840, a2841)
}

pub fn f563(v0: u32) -> u32 {
    let a2844 = {
        let a2842 = f590(v0, 2u32, 30u32);
        let a2843 = f590(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2842, a2843)
    };
    let a2845 = f590(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2844, a2845)
}

pub fn f564(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2854 = {
        let a2850 = {
            let a2846 = v0;
            let a2847 = v1;
            fixed_u32::bit_and(a2846, a2847)
        };
        let a2851 = {
            let a2848 = v0;
            let a2849 = v2;
            fixed_u32::bit_and(a2848, a2849)
        };
        fixed_u32::bit_xor(a2850, a2851)
    };
    let a2855 = {
        let a2852 = v1;
        let a2853 = v2;
        fixed_u32::bit_and(a2852, a2853)
    };
    fixed_u32::bit_xor(a2854, a2855)
}

pub fn f565(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h2856, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2856
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h2857, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2857
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h2858, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2858
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h2859, _, _, _, _, _, _, _, _, _, _, _) => {
            h2859
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h2860, _, _, _, _, _, _, _, _, _, _) => {
            h2860
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h2861, _, _, _, _, _, _, _, _, _) => {
            h2861
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h2862, _, _, _, _, _, _, _, _) => {
            h2862
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h2863, _, _, _, _, _, _, _) => {
            h2863
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2864, _, _, _, _, _, _) => {
            h2864
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h2865, _, _, _, _, _) => {
            h2865
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h2866, _, _, _, _) => {
            h2866
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2867, _, _, _) => {
            h2867
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2868, _, _) => {
            h2868
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2869, _) => {
            h2869
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2870) => {
            h2870
        }
    }, f517(f591(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2871, _) => {
            h2871
        }
    }), f517(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h2872, _, _, _, _, _, _) => {
            h2872
        }
    }, f517(f592(match v0.clone() {
        Adt26::C0(_, h2873, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2873
        }
    }), match v0 {
        Adt26::C0(h2874, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2874
        }
    }))))
}

pub fn f566(v0: Fn22, p2878: u64, p2879: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2877 = (p2878, p2879);
    {
        let mut m2877 = m2877;
        loop {
            let step = {
                let (v1, v2) = m2877;
                let m2875 = v1;
                if m2875 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2875 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2876: Fn22 = v0.clone();
                        c2876.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2877 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2880, _) = v0;
        h2880
    };
    let v2: u32 = {
        let (h2881, _) = {
            let (_, h2882) = v0;
            h2882
        };
        h2881
    };
    let v3: u32 = {
        let (_, h2883) = {
            let (_, h2884) = v0;
            h2884
        };
        h2883
    };
    ({
        let a2885 = v1;
        let a2886 = 2u64;
        let a2887 = 0u64;
        nat_quot(a2885, a2886, a2887)
    }, (f568(v2, 1u32), {
        let a2891 = {
            let a2888 = v1;
            let a2889 = 2u64;
            let a2890 = 0u64;
            nat_rem(a2888, a2889, a2890)
        };
        let a2892 = 1u64;
        if nat_eq(a2891, a2892) {
            let a2893 = v3;
            let a2894 = v2;
            fixed_u32::bit_or(a2893, a2894)
        } else {
            v3
        }
    }))
}

pub fn f568(v0: u32, v1: u32) -> u32 {
    let m2897 = {
        let a2895 = v0;
        let a2896 = v1;
        fixed_u32::shift_left(a2895, a2896)
    };
    match m2897 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f569(v0: Fn23, p2902: u64, p2903: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2901 = (p2902, p2903);
    {
        let mut m2901 = m2901;
        loop {
            let step = {
                let (v1, v2) = m2901;
                let m2898 = v1;
                if m2898 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2898 - 1;
                    let m2900 = {
                        let c2899: Fn23 = v0.clone();
                        c2899.apply(v2.clone())?
                    };
                    match m2900 {
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
                Ok(next) => { m2901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f593(v0)
}

pub fn f571(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2904, _) = v1;
        h2904
    };
    let m2906 = {
        let (_, h2905) = v1;
        h2905
    };
    match m2906 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2909 = {
                let a2907 = v0.clone();
                let a2908 = v2;
                index_bytes(a2907, a2908)
            };
            match m2909 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2914 = {
                        let a2912 = v0;
                        let a2913 = {
                            let a2910 = v2;
                            let a2911 = 1u64;
                            nat_add(a2910, a2911)?
                        };
                        index_bytes(a2912, a2913)
                    };
                    match m2914 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f594(v4)? {
                                f594(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2915 = v2;
                                    let a2916 = 2u64;
                                    nat_add(a2915, a2916)?
                                }, Some({
                                    let a2921 = v3;
                                    let a2922 = f490(f595(), {
                                        let a2919 = {
                                            let a2917 = f596(v4)?;
                                            let a2918 = 16u64;
                                            nat_mul(a2917, a2918)?
                                        };
                                        let a2920 = f596(v5)?;
                                        nat_add(a2919, a2920)?
                                    });
                                    append_bytes(a2921, a2922)
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

pub fn f572(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2927 = {
            let a2925 = {
                let a2923 = v0.clone();
                length_list(a2923)
            };
            let a2926 = {
                let a2924 = v1.clone();
                length_list(a2924)
            };
            nat_add(a2925, a2926)?
        };
        let a2928 = 1u64;
        nat_add(a2927, a2928)?
    };
    Ok(f423({
        let (_, h2929) = {
            let (_, h2930) = {
                let (h2931, _) = f597(Fn24::F598, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2931
            };
            h2930
        };
        h2929
    }))
}

pub fn f573(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2932 = v1;
    match m2932.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2933 = f599(v0, v3.clone())?;
            match m2933 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f600(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f47(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2934 = f573(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2934 {
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

pub fn f574(p2938: List<Bytes>) -> R<bool> {
    let m2937 = p2938;
    {
        let mut m2937 = m2937;
        loop {
            let step = {
                let v0 = m2937;
                let m2935 = v0;
                match m2935.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2936 = f601(v1)?;
                        match m2936 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f47(v3, f602(false, v2.clone())?) {
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
                Ok(next) => { m2937 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f575(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m2939 = v0;
    match m2939.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f603(v1.clone(), {
                let (h2940, _) = v2.clone();
                h2940
            }) {
                List::cons({
                    let (h2941, _) = v2;
                    h2941
                }, f575(v3, v1))
            } else {
                f575(v3, v1)
            }
        }
    }
}

pub fn f576(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m2942 = v2;
    match m2942.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m2943 = f604(v4.clone())?;
            match m2943 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m2944 = f605(v0, v6.clone())?;
                    match m2944 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f47(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2947 = f606(v4, {
                                    let a2945 = v1.clone();
                                    let a2946 = f363(v6)?;
                                    append_bytes(a2945, a2946)
                                })?;
                                match m2947 {
                                    Ok(v9) => {
                                        let m2948 = f576(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m2948 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f513(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f551(v11.clone())), f480(f512(v9), f552(v11)))))
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

pub fn f577(v0: List<Adt2>) -> R<List<Bytes>> {
    let m2949 = v0;
    match m2949.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m2950 = f604(v1)?;
            match m2950 {
                None => {
                    f577(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f577(v2)?))
                }
            }
        }
    }
}

pub fn f578(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f607(f574(f608(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f579(v0: Adt2, v1: Bytes) -> R<bool> {
    let m2951 = f369(Some(v0));
    match m2951 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2952 = f543(v2)?;
            match m2952 {
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

pub fn f580(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m2953 = v2;
    match m2953.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m2955 = f605(v0, {
                let (h2954, _) = v5.clone();
                h2954
            })?;
            match m2955 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h2956, _) = v5;
                        h2956
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f47(v7.clone(), f602(v0, f581(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2961 = f553({
                            let (_, h2957) = v5.clone();
                            h2957
                        }, {
                            let a2959 = v1.clone();
                            let a2960 = f363({
                                let (h2958, _) = v5.clone();
                                h2958
                            })?;
                            append_bytes(a2959, a2960)
                        })?;
                        match m2961 {
                            Ok(v9) => {
                                let v10: Bytes = if f47({
                                    let (h2962, _) = v5;
                                    h2962
                                }, v3.clone()) {
                                    f513(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f513(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m2963 = f580(v0, v1, v6, v3, v4)?;
                                match m2963 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f551(v12.clone())), f480(f512(v9), f552(v12)))))
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

pub fn f581(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m2964 = v0;
    match m2964.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2965, _) = v1;
                h2965
            }, f581(v2))
        }
    }
}

pub fn f582(v0: Adt2) -> Option<Adt2> {
    let m2966 = f276(f271(v0, f8(Str::lit("oneOf"))));
    match m2966.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m2967 = v2;
            match m2967.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m2968 = v4;
                    match m2968.uncons() {
                        None => {
                            if f609(v3.clone()) {
                                Some(v1)
                            } else {
                                if f609(v1) {
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

pub fn f583(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m2969 = v1;
    match m2969 {
        Ok(v3) => {
            f546(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f513(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f512(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f584(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m2970 = f582(v0.clone());
    match m2970 {
        None => {
            if f32(f274(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2971 = f271(v0, f8(Str::lit("items")));
                match m2971 {
                    None => {
                        Ok(f583(f8(Str::lit("List")), f585()))
                    }
                    Some(v3) => {
                        Ok(f583(f8(Str::lit("List")), f606(v3, {
                            let a2972 = v1;
                            let a2973 = f8(Str::lit("Item"));
                            append_bytes(a2972, a2973)
                        })?))
                    }
                }
            } else {
                f606(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f583(f8(Str::lit("Option")), f606(v2, v1)?))
        }
    }
}

pub fn f585() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f509(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f586(p2977: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m2976 = p2977;
    {
        let mut m2976 = m2976;
        loop {
            let step = {
                let v0 = m2976;
                let m2974 = v0;
                match m2974.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f47({
                            let (h2975, _) = v2;
                            h2975
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
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

pub fn f587(p2981: List<Adt2>, v1: Bytes, p2982: List<Bytes>) -> bool {
    let m2980 = (p2981, p2982);
    {
        let mut m2980 = m2980;
        loop {
            let step = {
                let (v0, v2) = m2980;
                let m2978 = v0;
                match m2978.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m2979 = f610(v3, v1.clone());
                        match m2979 {
                            None => {
                                Err::<(List<Adt2>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f47(v5.clone(), v2.clone()) {
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
                Ok(next) => { m2980 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(p2985: List<Adt2>, p2986: List<Bytes>) -> bool {
    let m2984 = (p2985, p2986);
    {
        let mut m2984 = m2984;
        loop {
            let step = {
                let (v0, v1) = m2984;
                let m2983 = v0;
                match m2983.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f274(v2, f8(Str::lit("type")));
                        if !f32(v4.clone(), Bytes::lit(&[])) {
                            if !f47(v4.clone(), v1.clone()) {
                                if !if if f32(v4.clone(), f8(Str::lit("integer"))) {
                                    f47(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f32(v4.clone(), f8(Str::lit("number"))) {
                                        f47(f8(Str::lit("integer")), v1.clone())
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
                Ok(next) => { m2984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m2987 = f271(v0.clone(), f8(Str::lit("anyOf")));
        match m2987 {
            None => {
                Adt2::C6
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f586(f506(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f554(f370(v2))
    } else {
        false
    }
}

pub fn f590(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2988 = f486(v0, v1);
    let a2989 = f568(v0, v2);
    fixed_u32::bit_or(a2988, a2989)
}

pub fn f591(v0: u32) -> u32 {
    let a2992 = {
        let a2990 = f590(v0, 17u32, 15u32);
        let a2991 = f590(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2990, a2991)
    };
    let a2993 = f486(v0, 10u32);
    fixed_u32::bit_xor(a2992, a2993)
}

pub fn f592(v0: u32) -> u32 {
    let a2996 = {
        let a2994 = f590(v0, 7u32, 25u32);
        let a2995 = f590(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2994, a2995)
    };
    let a2997 = f486(v0, 3u32);
    fixed_u32::bit_xor(a2996, a2997)
}

pub fn f593(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2998, _) = v0;
        h2998
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2999) = v0;
        h2999
    };
    let m3000 = v1;
    match m3000.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3001 = v4;
            match m3001.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f611(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f594(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3046 = {
            let a3022 = {
                let a3010 = {
                    let a3004 = {
                        let a3002 = v1;
                        let a3003 = 128u8;
                        fixed_u8::bit_and(a3002, a3003)
                    };
                    let a3005 = 128u8;
                    if equal(a3004, a3005) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3011 = {
                    let a3008 = {
                        let a3006 = v1;
                        let a3007 = 64u8;
                        fixed_u8::bit_and(a3006, a3007)
                    };
                    let a3009 = 64u8;
                    if equal(a3008, a3009) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3010, a3011)?
            };
            let a3023 = {
                let a3020 = {
                    let a3014 = {
                        let a3012 = v1;
                        let a3013 = 32u8;
                        fixed_u8::bit_and(a3012, a3013)
                    };
                    let a3015 = 32u8;
                    if equal(a3014, a3015) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3021 = {
                    let a3018 = {
                        let a3016 = v1;
                        let a3017 = 16u8;
                        fixed_u8::bit_and(a3016, a3017)
                    };
                    let a3019 = 16u8;
                    if equal(a3018, a3019) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3020, a3021)?
            };
            nat_add(a3022, a3023)?
        };
        let a3047 = {
            let a3044 = {
                let a3032 = {
                    let a3026 = {
                        let a3024 = v1;
                        let a3025 = 8u8;
                        fixed_u8::bit_and(a3024, a3025)
                    };
                    let a3027 = 8u8;
                    if equal(a3026, a3027) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3033 = {
                    let a3030 = {
                        let a3028 = v1;
                        let a3029 = 4u8;
                        fixed_u8::bit_and(a3028, a3029)
                    };
                    let a3031 = 4u8;
                    if equal(a3030, a3031) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3032, a3033)?
            };
            let a3045 = {
                let a3042 = {
                    let a3036 = {
                        let a3034 = v1;
                        let a3035 = 2u8;
                        fixed_u8::bit_and(a3034, a3035)
                    };
                    let a3037 = 2u8;
                    if equal(a3036, a3037) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3043 = {
                    let a3040 = {
                        let a3038 = v1;
                        let a3039 = 1u8;
                        fixed_u8::bit_and(a3038, a3039)
                    };
                    let a3041 = 1u8;
                    if equal(a3040, a3041) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3042, a3043)?
            };
            nat_add(a3044, a3045)?
        };
        nat_add(a3046, a3047)?
    };
    let a3048 = 48u64;
    let a3049 = v2;
    if if nat_le(a3048, a3049) {
        let a3050 = v2;
        let a3051 = 57u64;
        nat_le(a3050, a3051)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3052 = 97u64;
        let a3053 = v2;
        if nat_le(a3052, a3053) {
            let a3054 = v2;
            let a3055 = 102u64;
            Ok(nat_le(a3054, a3055))
        } else {
            Ok(false)
        }
    }
}

pub fn f595() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f596(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3100 = {
            let a3076 = {
                let a3064 = {
                    let a3058 = {
                        let a3056 = v1;
                        let a3057 = 128u8;
                        fixed_u8::bit_and(a3056, a3057)
                    };
                    let a3059 = 128u8;
                    if equal(a3058, a3059) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3065 = {
                    let a3062 = {
                        let a3060 = v1;
                        let a3061 = 64u8;
                        fixed_u8::bit_and(a3060, a3061)
                    };
                    let a3063 = 64u8;
                    if equal(a3062, a3063) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3064, a3065)?
            };
            let a3077 = {
                let a3074 = {
                    let a3068 = {
                        let a3066 = v1;
                        let a3067 = 32u8;
                        fixed_u8::bit_and(a3066, a3067)
                    };
                    let a3069 = 32u8;
                    if equal(a3068, a3069) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3075 = {
                    let a3072 = {
                        let a3070 = v1;
                        let a3071 = 16u8;
                        fixed_u8::bit_and(a3070, a3071)
                    };
                    let a3073 = 16u8;
                    if equal(a3072, a3073) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3074, a3075)?
            };
            nat_add(a3076, a3077)?
        };
        let a3101 = {
            let a3098 = {
                let a3086 = {
                    let a3080 = {
                        let a3078 = v1;
                        let a3079 = 8u8;
                        fixed_u8::bit_and(a3078, a3079)
                    };
                    let a3081 = 8u8;
                    if equal(a3080, a3081) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3087 = {
                    let a3084 = {
                        let a3082 = v1;
                        let a3083 = 4u8;
                        fixed_u8::bit_and(a3082, a3083)
                    };
                    let a3085 = 4u8;
                    if equal(a3084, a3085) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3086, a3087)?
            };
            let a3099 = {
                let a3096 = {
                    let a3090 = {
                        let a3088 = v1;
                        let a3089 = 2u8;
                        fixed_u8::bit_and(a3088, a3089)
                    };
                    let a3091 = 2u8;
                    if equal(a3090, a3091) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3097 = {
                    let a3094 = {
                        let a3092 = v1;
                        let a3093 = 1u8;
                        fixed_u8::bit_and(a3092, a3093)
                    };
                    let a3095 = 1u8;
                    if equal(a3094, a3095) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3096, a3097)?
            };
            nat_add(a3098, a3099)?
        };
        nat_add(a3100, a3101)?
    };
    let a3102 = v2;
    let a3103 = 57u64;
    if nat_le(a3102, a3103) {
        let a3104 = v2;
        let a3105 = 48u64;
        Ok(nat_sub(a3104, a3105))
    } else {
        let a3106 = v2;
        let a3107 = 87u64;
        Ok(nat_sub(a3106, a3107))
    }
}

pub fn f597(v0: Fn24, p3112: u64, p3113: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3111 = (p3112, p3113);
    {
        let mut m3111 = m3111;
        loop {
            let step = {
                let (v1, v2) = m3111;
                let m3108 = v1;
                if m3108 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3108 - 1;
                    let m3110 = {
                        let c3109: Fn24 = v0.clone();
                        c3109.apply(v2.clone())
                    };
                    match m3110 {
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
                Ok(next) => { m3111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f612(v0)
}

pub fn f599(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3114 = v1;
    match m3114 {
        Adt2::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(v7, v8) => {
            Ok(Some({
                let a3125 = if f112(v7, {
                    let a3123 = 0u64;
                    let a3124 = v8;
                    nat_lt(a3123, a3124)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3126 = f290(false, v8)?;
                append_bytes(a3125, a3126)
            }))
        }
        Adt2::C2(v3, v4, v5) => {
            let m3117 = f613({
                let a3115 = f8(Str::lit("n"));
                let a3116 = f439(false, v4, v5)?;
                append_bytes(a3115, a3116)
            })?;
            match m3117 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3121 = f8(Str::lit("nMinus"));
                        let a3122 = f41(v6.clone(), 1u64, {
                            let a3119 = {
                                let a3118 = v6;
                                length_bytes(a3118)
                            };
                            let a3120 = 1u64;
                            nat_sub(a3119, a3120)
                        });
                        append_bytes(a3121, a3122)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C3(v2) => {
            f605(v0, v2)
        }
        Adt2::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C6 => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f600(v0: Adt2) -> R<Bytes> {
    f368(v0)
}

pub fn f601(v0: Bytes) -> R<Option<Bytes>> {
    if f614({
        let a3127 = v0.clone();
        length_bytes(a3127)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f615(v0)?;
        if f616(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3130 = {
                let a3128 = v1.clone();
                let a3129 = 0u64;
                index_bytes(a3128, a3129)
            };
            match m3130 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f474(f67(v2)?) {
                        Ok(Some({
                            let a3131 = f8(Str::lit("n"));
                            let a3132 = v1;
                            append_bytes(a3131, a3132)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f617({
            let a3133 = v0.clone();
            length_bytes(a3133)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f602(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3134 = v1;
    match m3134.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3135 = f605(v0, v2)?;
            match m3135 {
                None => {
                    f602(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f602(v0, v3)?))
                }
            }
        }
    }
}

pub fn f603(p3139: List<Adt2>, v1: Bytes) -> bool {
    let m3138 = p3139;
    {
        let mut m3138 = m3138;
        loop {
            let step = {
                let v0 = m3138;
                let m3136 = v0;
                match m3136.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3137 = f610(v2, v1.clone());
                        if match m3137 {
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
                Ok(next) => { m3138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: Adt2) -> R<Option<Bytes>> {
    let m3140 = f369(f271(v0.clone(), f8(Str::lit("$ref"))));
    match m3140 {
        None => {
            Ok(f369(f271(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f543(v1)
        }
    }
}

pub fn f605(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f613(v1)
    } else {
        f601(v1)
    }
}

pub fn f606(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f618(f531(v0), v1)
}

pub fn f607(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3141 = v2;
    match m3141.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3142 = f610(v5.clone(), v3.clone());
            match m3142 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3143 = f605(v0, v7.clone())?;
                    match m3143 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f47(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3146 = f549({
                                    let a3144 = v1.clone();
                                    let a3145 = f363(v7)?;
                                    append_bytes(a3144, a3145)
                                }, f508(f476(f271(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f275(f276(f271(v5, f8(Str::lit("required"))))), true)?;
                                match m3146 {
                                    Ok(v10) => {
                                        let m3147 = f607(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3147 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f377(f551(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f551(v12.clone())), f480(f552(v10), f552(v12)))))
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

pub fn f608(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3148 = v0;
    match m3148.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3149 = f610(v2, v1.clone());
            match m3149 {
                None => {
                    f608(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f608(v3, v1))
                }
            }
        }
    }
}

pub fn f609(v0: Adt2) -> bool {
    f32(f274(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f610(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f47(v1.clone(), f275(f276(f271(v0.clone(), f8(Str::lit("required")))))) {
        let m3150 = f271(v0, f8(Str::lit("properties")));
        match m3150 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3151 = f271(v2, v1);
                match m3151 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f369(f271(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f611(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3156 = {
            let a3154 = {
                let a3152 = v0.clone();
                length_list(a3152)
            };
            let a3155 = {
                let a3153 = v1.clone();
                length_list(a3153)
            };
            nat_add(a3154, a3155)?
        };
        let a3157 = 1u64;
        nat_add(a3156, a3157)?
    };
    Ok(f619({
        let (_, h3158) = {
            let (_, h3159) = {
                let (h3160, _) = f620(Fn25::F621, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3160
            };
            h3159
        };
        h3158
    }))
}

pub fn f612(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3161, _) = v0.clone();
        h3161
    };
    let v2: List<(Bytes, ())> = {
        let (h3162, _) = {
            let (_, h3163) = v0.clone();
            h3163
        };
        h3162
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3164) = {
            let (_, h3165) = v0;
            h3165
        };
        h3164
    };
    let m3166 = v1.clone();
    match m3166.uncons() {
        None => {
            let m3167 = v2;
            match m3167.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3168 = v2.clone();
            match m3168.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f622({
                        let (h3169, _) = v8.clone();
                        h3169
                    }, {
                        let (h3170, _) = v6.clone();
                        h3170
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

pub fn f613(v0: Bytes) -> R<Option<Bytes>> {
    let m3172 = f623({
        let a3171 = v0.clone();
        length_bytes(a3171)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m3172 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3174 = {
                let a3173 = v1.clone();
                length_bytes(a3173)
            };
            let a3175 = 0u64;
            if nat_eq(a3174, a3175) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f614(p3185: u64, v1: Bytes, p3186: u64) -> R<bool> {
    let m3184 = (p3185, p3186);
    {
        let mut m3184 = m3184;
        loop {
            let step = {
                let (v0, v2) = m3184;
                let m3176 = v0;
                if m3176 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3176 - 1;
                    let m3179 = {
                        let a3177 = v1.clone();
                        let a3178 = v2;
                        index_bytes(a3177, a3178)
                    };
                    match m3179 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3180 = f67(v4)?;
                            let a3181 = 128u64;
                            if nat_lt(a3180, a3181) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3182 = v2;
                                    let a3183 = 1u64;
                                    nat_add(a3182, a3183)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f435(v0.clone(), 0u64, {
        let a3187 = v0;
        length_bytes(a3187)
    }, false, Bytes::lit(&[]))?;
    let m3190 = {
        let a3188 = v1.clone();
        let a3189 = 0u64;
        index_bytes(a3188, a3189)
    };
    match m3190 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f67(v2)?;
            if f473(v3) {
                let a3197 = f41({
                    let a3191 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3191)
                }, {
                    let a3192 = v3;
                    let a3193 = 65u64;
                    nat_sub(a3192, a3193)
                }, 1u64);
                let a3198 = f41(v1.clone(), 1u64, {
                    let a3195 = {
                        let a3194 = v1;
                        length_bytes(a3194)
                    };
                    let a3196 = 1u64;
                    nat_sub(a3195, a3196)
                });
                Ok(append_bytes(a3197, a3198))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f616(v0: Bytes) -> R<bool> {
    let m3201 = {
        let a3199 = v0;
        let a3200 = 0u64;
        index_bytes(a3199, a3200)
    };
    match m3201 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f199(f472(f67(v1)?), f473(f67(v1)?)))
        }
    }
}

pub fn f617(p3214: u64, v1: Bytes, p3215: u64, p3216: Bytes) -> R<Option<Bytes>> {
    let m3213 = (p3214, (p3215, p3216));
    {
        let mut m3213 = m3213;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3213;
                let m3202 = v0;
                if m3202 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3202 - 1;
                    let a3204 = {
                        let a3203 = v1.clone();
                        length_bytes(a3203)
                    };
                    let a3205 = v2;
                    if nat_le(a3204, a3205) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3206 = f624(v1.clone(), v2)?;
                        match m3206 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3208 = v2;
                                    let a3209 = {
                                        let (_, h3207) = v5;
                                        h3207
                                    };
                                    nat_add(a3208, a3209)?
                                }, {
                                    let a3211 = v3;
                                    let a3212 = f625({
                                        let (h3210, _) = v5;
                                        h3210
                                    });
                                    append_bytes(a3211, a3212)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3217 = v0.clone();
    match m3217 {
        Adt2::C0(v2) => {
            if v2 {
                Ok(f585())
            } else {
                Ok(f545(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C1(_, _) => {
            Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(_, _, _) => {
            Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C3(_) => {
            Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_) => {
            Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_) => {
            let m3218 = f626(f506(v0.clone()));
            match m3218 {
                None => {
                    let m3219 = f369(f271(v0.clone(), f8(Str::lit("$ref"))));
                    match m3219 {
                        None => {
                            let m3221 = f271(v0.clone(), f8(Str::lit("allOf")));
                            match m3221 {
                                None => {
                                    let m3226 = f271(v0.clone(), f8(Str::lit("enum")));
                                    match m3226 {
                                        None => {
                                            let m3227 = f271(v0.clone(), f8(Str::lit("const")));
                                            match m3227 {
                                                None => {
                                                    let m3229 = f271(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3229 {
                                                        None => {
                                                            let v30: Bytes = f274(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v30.clone(), Bytes::lit(&[])) {
                                                                !f627(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f585())
                                                            } else {
                                                                if f32(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f546(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f32(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f546(f628(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f32(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f546(f629(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f32(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f546(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f32(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m3230 = f476(f271(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3230.uncons() {
                                                                                        None => {
                                                                                            let m3231 = f630(v0);
                                                                                            match m3231 {
                                                                                                None => {
                                                                                                    Ok(f583(f8(Str::lit("List")), f631(f585())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f583(f8(Str::lit("List")), f631(f632(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f546(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f545(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f546(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m3228 = f369(Some(v27));
                                                    match m3228 {
                                                        None => {
                                                            Ok(f545(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f546(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f546(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m3222 = f370(v19);
                                    match m3222.uncons() {
                                        None => {
                                            Ok(f545(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m3223 = v21;
                                            match m3223.uncons() {
                                                None => {
                                                    let m3224 = f369(f271(v20, f8(Str::lit("$ref"))));
                                                    match m3224 {
                                                        None => {
                                                            Ok(f545(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m3225 = f543(v24)?;
                                                            match m3225 {
                                                                None => {
                                                                    Ok(f545(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f546(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f545(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m3220 = f543(v17)?;
                            match m3220 {
                                None => {
                                    Ok(f545(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f546(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f545(if f32(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C6 => {
            Ok(f585())
        }
        Adt2::C7(_, _) => {
            Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C8(_, _, _) => {
            Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f619(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f385(Fn7::F633, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f620(v0: Fn25, p3236: u64, p3237: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3235 = (p3236, p3237);
    {
        let mut m3235 = m3235;
        loop {
            let step = {
                let (v1, v2) = m3235;
                let m3232 = v1;
                if m3232 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3232 - 1;
                    let m3234 = {
                        let c3233: Fn25 = v0.clone();
                        c3233.apply(v2.clone())
                    };
                    match m3234 {
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
                Ok(next) => { m3235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f634(v0)
}

pub fn f622(v0: Bytes, v1: Bytes) -> bool {
    let a3242 = {
        let a3238 = v0;
        let a3239 = v1;
        compare_bytes(a3238, a3239)
    };
    let a3243 = {
        let a3240 = Bytes::lit(&[]);
        let a3241 = Bytes::lit(&[0x01]);
        compare_bytes(a3240, a3241)
    };
    equal(a3242, a3243)
}

pub fn f623(p3256: u64, v1: Bytes, p3257: u64, p3258: Bytes) -> R<Option<Bytes>> {
    let m3255 = (p3256, (p3257, p3258));
    {
        let mut m3255 = m3255;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3255;
                let m3244 = v0;
                if m3244 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3244 - 1;
                    let a3246 = {
                        let a3245 = v1.clone();
                        length_bytes(a3245)
                    };
                    let a3247 = v2;
                    if nat_le(a3246, a3247) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3248 = f624(v1.clone(), v2)?;
                        match m3248 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3250 = v2;
                                    let a3251 = {
                                        let (_, h3249) = v5;
                                        h3249
                                    };
                                    nat_add(a3250, a3251)?
                                }, {
                                    let a3253 = v3;
                                    let a3254 = f635(v1.clone(), v2, {
                                        let (h3252, _) = v5;
                                        h3252
                                    });
                                    append_bytes(a3253, a3254)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f624(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3261 = {
        let a3259 = v0.clone();
        let a3260 = v1;
        index_bytes(a3259, a3260)
    };
    match m3261 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f67(v2)?;
            let a3262 = v3;
            let a3263 = 128u64;
            if nat_lt(a3262, a3263) {
                Ok(Some((v3, 1u64)))
            } else {
                if f112({
                    let a3264 = 194u64;
                    let a3265 = v3;
                    nat_le(a3264, a3265)
                }, {
                    let a3266 = v3;
                    let a3267 = 223u64;
                    nat_le(a3266, a3267)
                }) {
                    let m3270 = f636(v0, {
                        let a3268 = v1;
                        let a3269 = 1u64;
                        nat_add(a3268, a3269)?
                    })?;
                    match m3270 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f637({
                                let a3275 = {
                                    let a3273 = {
                                        let a3271 = v3;
                                        let a3272 = 192u64;
                                        nat_sub(a3271, a3272)
                                    };
                                    let a3274 = 64u64;
                                    nat_mul(a3273, a3274)?
                                };
                                let a3276 = v4;
                                nat_add(a3275, a3276)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f112({
                        let a3277 = 224u64;
                        let a3278 = v3;
                        nat_le(a3277, a3278)
                    }, {
                        let a3279 = v3;
                        let a3280 = 239u64;
                        nat_le(a3279, a3280)
                    }) {
                        let m3283 = f636(v0.clone(), {
                            let a3281 = v1;
                            let a3282 = 1u64;
                            nat_add(a3281, a3282)?
                        })?;
                        match m3283 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3286 = f636(v0, {
                                    let a3284 = v1;
                                    let a3285 = 2u64;
                                    nat_add(a3284, a3285)?
                                })?;
                                match m3286 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f637({
                                            let a3295 = {
                                                let a3289 = {
                                                    let a3287 = v3;
                                                    let a3288 = 224u64;
                                                    nat_sub(a3287, a3288)
                                                };
                                                let a3290 = 4096u64;
                                                nat_mul(a3289, a3290)?
                                            };
                                            let a3296 = {
                                                let a3293 = {
                                                    let a3291 = v5;
                                                    let a3292 = 64u64;
                                                    nat_mul(a3291, a3292)?
                                                };
                                                let a3294 = v6;
                                                nat_add(a3293, a3294)?
                                            };
                                            nat_add(a3295, a3296)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f112({
                            let a3297 = 240u64;
                            let a3298 = v3;
                            nat_le(a3297, a3298)
                        }, {
                            let a3299 = v3;
                            let a3300 = 244u64;
                            nat_le(a3299, a3300)
                        }) {
                            let m3303 = f636(v0.clone(), {
                                let a3301 = v1;
                                let a3302 = 1u64;
                                nat_add(a3301, a3302)?
                            })?;
                            match m3303 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3306 = f636(v0.clone(), {
                                        let a3304 = v1;
                                        let a3305 = 2u64;
                                        nat_add(a3304, a3305)?
                                    })?;
                                    match m3306 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3309 = f636(v0, {
                                                let a3307 = v1;
                                                let a3308 = 3u64;
                                                nat_add(a3307, a3308)?
                                            })?;
                                            match m3309 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f637({
                                                        let a3322 = {
                                                            let a3312 = {
                                                                let a3310 = v3;
                                                                let a3311 = 240u64;
                                                                nat_sub(a3310, a3311)
                                                            };
                                                            let a3313 = 262144u64;
                                                            nat_mul(a3312, a3313)?
                                                        };
                                                        let a3323 = {
                                                            let a3320 = {
                                                                let a3314 = v7;
                                                                let a3315 = 4096u64;
                                                                nat_mul(a3314, a3315)?
                                                            };
                                                            let a3321 = {
                                                                let a3318 = {
                                                                    let a3316 = v8;
                                                                    let a3317 = 64u64;
                                                                    nat_mul(a3316, a3317)?
                                                                };
                                                                let a3319 = v9;
                                                                nat_add(a3318, a3319)?
                                                            };
                                                            nat_add(a3320, a3321)?
                                                        };
                                                        nat_add(a3322, a3323)?
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

pub fn f625(v0: u64) -> Bytes {
    let a3324 = f8(Str::lit("u"));
    let a3325 = f638(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3324, a3325)
}

pub fn f626(p3330: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3329 = p3330;
    {
        let mut m3329 = m3329;
        loop {
            let step = {
                let v0 = m3329;
                let m3326 = v0;
                match m3326.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f47({
                            let (h3327, _) = v1.clone();
                            h3327
                        }, f639()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3328, _) = v1;
                                h3328
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: Adt2) -> bool {
    if f556(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f556(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f556(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f556(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f556(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f556(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f628(v0: Adt2) -> Bytes {
    let m3331 = f640(f271(v0, f8(Str::lit("minimum"))));
    match m3331 {
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

pub fn f630(v0: Adt2) -> Option<Adt2> {
    let m3332 = f271(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3332 {
        None => {
            let m3333 = f476(f271(v0, f8(Str::lit("patternProperties"))));
            match m3333.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v2, v3)) => {
                    let m3334 = v3;
                    match m3334.uncons() {
                        None => {
                            Some({
                                let (_, h3335) = v2;
                                h3335
                            })
                        }
                        Some((_, _)) => {
                            None::<Adt2>
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

pub fn f631(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3336 = v0;
    match m3336 {
        Ok(v2) => {
            f546(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f513(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f512(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f632(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f641(16u64, v0, v1)
}

pub fn f633(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f634(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3337, _) = v0.clone();
        h3337
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3338, _) = {
            let (_, h3339) = v0.clone();
            h3339
        };
        h3338
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3340) = {
            let (_, h3341) = v0;
            h3341
        };
        h3340
    };
    let m3342 = v1.clone();
    match m3342.uncons() {
        None => {
            let m3343 = v2;
            match m3343.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3344 = v2.clone();
            match m3344.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f622({
                        let (h3345, _) = v8.clone();
                        h3345
                    }, {
                        let (h3346, _) = v6.clone();
                        h3346
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

pub fn f635(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f199(f112(f199(f472(v2), f473(v2)), f112({
        let a3347 = v2;
        let a3348 = 117u64;
        !nat_eq(a3347, a3348)
    }, {
        let a3349 = v2;
        let a3350 = 85u64;
        !nat_eq(a3349, a3350)
    })), f112(f474(v2), {
        let a3351 = 0u64;
        let a3352 = v1;
        nat_lt(a3351, a3352)
    })) {
        f41(v0, v1, 1u64)
    } else {
        let a3353 = v2;
        let a3354 = 65536u64;
        if nat_lt(a3353, a3354) {
            let a3355 = f8(Str::lit("u"));
            let a3356 = f638(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3355, a3356)
        } else {
            let a3357 = f8(Str::lit("U"));
            let a3358 = f638(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3357, a3358)
        }
    }
}

pub fn f636(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3361 = {
        let a3359 = v0;
        let a3360 = v1;
        index_bytes(a3359, a3360)
    };
    match m3361 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f67(v2)?;
            if f112({
                let a3362 = 128u64;
                let a3363 = v3;
                nat_le(a3362, a3363)
            }, {
                let a3364 = v3;
                let a3365 = 191u64;
                nat_le(a3364, a3365)
            }) {
                Ok(Some({
                    let a3366 = v3;
                    let a3367 = 128u64;
                    nat_sub(a3366, a3367)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f637(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f112({
        let a3368 = v1;
        let a3369 = v0;
        nat_le(a3368, a3369)
    }, f112({
        let a3370 = v0;
        let a3371 = v2;
        nat_le(a3370, a3371)
    }, f199({
        let a3372 = v0;
        let a3373 = 55296u64;
        nat_lt(a3372, a3373)
    }, {
        let a3374 = 57343u64;
        let a3375 = v0;
        nat_lt(a3374, a3375)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f638(p3392: u64, p3393: u64, p3394: u64, p3395: Bytes) -> Bytes {
    let m3391 = (p3392, (p3393, (p3394, p3395)));
    {
        let mut m3391 = m3391;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3391;
                let m3376 = v0;
                if m3376 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3376 - 1;
                    if f112({
                        let a3377 = v1;
                        let a3378 = 0u64;
                        nat_eq(a3377, a3378)
                    }, {
                        let a3379 = v2;
                        let a3380 = 0u64;
                        nat_eq(a3379, a3380)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3381 = v1;
                            let a3382 = 16u64;
                            let a3383 = 0u64;
                            nat_quot(a3381, a3382, a3383)
                        }, ({
                            let a3384 = v2;
                            let a3385 = 1u64;
                            nat_sub(a3384, a3385)
                        }, {
                            let a3389 = f642({
                                let a3386 = v1;
                                let a3387 = 16u64;
                                let a3388 = 0u64;
                                nat_rem(a3386, a3387, a3388)
                            });
                            let a3390 = v3;
                            append_bytes(a3389, a3390)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f639() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f640(v0: Option<Adt2>) -> Option<u64> {
    let m3396 = v0;
    match m3396 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m3397 = v1;
            match m3397 {
                Adt2::C0(_) => {
                    None::<u64>
                }
                Adt2::C1(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt2::C2(_, _, _) => {
                    None::<u64>
                }
                Adt2::C3(_) => {
                    None::<u64>
                }
                Adt2::C4(_) => {
                    None::<u64>
                }
                Adt2::C5(_) => {
                    None::<u64>
                }
                Adt2::C6 => {
                    None::<u64>
                }
                Adt2::C7(_, _) => {
                    None::<u64>
                }
                Adt2::C8(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f641(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3398 = v0;
    if m3398 == 0 {
        Ok(f545(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3398 - 1;
        let m3399 = v1.clone();
        match m3399 {
            Adt2::C0(v4) => {
                if v4 {
                    Ok(f585())
                } else {
                    Ok(f545(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C1(_, _) => {
                Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(_, _, _) => {
                Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C3(_) => {
                Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_) => {
                Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_) => {
                let m3400 = f369(f271(v1.clone(), f8(Str::lit("$ref"))));
                match m3400 {
                    None => {
                        if if f556(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f556(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f556(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f546(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f274(v1.clone(), f8(Str::lit("type")));
                            if if f32(v20.clone(), Bytes::lit(&[])) {
                                !f627(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f585())
                            } else {
                                if f32(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f546(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f32(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f546(f628(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f32(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f546(f629(), List::<Adt25>::nil()))
                                        } else {
                                            if f32(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f546(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f32(v20.clone(), f8(Str::lit("array"))) {
                                                    let m3402 = f271(v1, f8(Str::lit("items")));
                                                    match m3402 {
                                                        None => {
                                                            Ok(f583(f8(Str::lit("List")), f585()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f583(f8(Str::lit("List")), f641(v3, v21, {
                                                                let a3403 = v2;
                                                                let a3404 = f8(Str::lit("Item"));
                                                                append_bytes(a3403, a3404)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v20.clone(), f8(Str::lit("object"))) {
                                                        let m3405 = f630(v1);
                                                        match m3405 {
                                                            None => {
                                                                Ok(f583(f8(Str::lit("List")), f631(f585())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f583(f8(Str::lit("List")), f631(f641(v3, v22, {
                                                                    let a3406 = v2;
                                                                    let a3407 = f8(Str::lit("Value"));
                                                                    append_bytes(a3406, a3407)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f545(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
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
                        let m3401 = f543(v18)?;
                        match m3401 {
                            None => {
                                Ok(f545(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f546(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C6 => {
                Ok(f585())
            }
            Adt2::C7(_, _) => {
                Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C8(_, _, _) => {
                Ok(f545(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f642(v0: u64) -> Bytes {
    f41({
        let a3408 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3408)
    }, v0, 1u64)
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
